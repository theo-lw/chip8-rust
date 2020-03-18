mod chip8;
mod config;
mod instructions;
mod variables;

use chip8::{
    display::*, keyboard::SDLKeyboard, memory::Memory, registers::Registers, stack::Stack,
    timers::Timers, State,
};
use clap::App;
use config::Config;
use instructions::Instruction;
use sdl2;
use sdl2::{pixels::Color, rect::Rect};
use std::{fs, thread, time::Duration};

fn main() {
    // Read command line arguments
    let matches = App::new("chip8 emulator")
        .author("Theodore Wang")
        .version("v0.0.1")
        .about("A chip8 emulator")
        .args_from_usage(
            "<PROGRAM>          'Set the file containing the chip8 program'
            -c, --config=[FILE] 'Use a custom config.json file'
            -d, --debug         'Print debugging information'
            -s, --step          'Step through instructions one by one (press the key mapped to one to quit)'",
        )
        .get_matches();

    // Read config
    let config: Config = if let Some(config_file) = matches.value_of("config") {
        Config::from_file(config_file)
    } else {
        Default::default()
    };
    let active_color: Color = config.get_active_color();
    let inactive_color: Color = config.get_inactive_color();
    let pixel_size: u32 = config.pixel_size;

    // Read program
    let program: Vec<u8> =
        fs::read(matches.value_of("PROGRAM").unwrap()).expect("Could not read the chip8 program!");

    // Set up SDL
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(
            "chip8 emulator",
            Display::WIDTH as u32 * config.pixel_size,
            Display::HEIGHT as u32 * config.pixel_size,
        )
        .position_centered()
        .build()
        .unwrap();
    let event_pump = sdl_context.event_pump().unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let sdl_display_observer = move |event: DisplayEvent| match event {
        DisplayEvent::XOR(x, y, active) => {
            if active == 1 {
                canvas.set_draw_color(active_color);
            } else {
                canvas.set_draw_color(inactive_color);
            }
            canvas
                .fill_rect(Rect::new(
                    (x as u32 * pixel_size) as i32,
                    (y as u32 * pixel_size) as i32,
                    pixel_size,
                    pixel_size,
                ))
                .unwrap_or_else(|e| eprintln!("Error while XOR-ing onto canvas: {}", e));
        }
        DisplayEvent::CLEAR => {
            canvas.set_draw_color(inactive_color);
            canvas.clear();
        }
        DisplayEvent::PRESENT => {
            canvas.present();
        }
    };

    // Initialize state
    let mut state: State = State {
        memory: Memory::new(&program),
        program_counter: chip8::memory::PROGRAM_START,
        stack: Stack::new(),
        registers: Registers::new(),
        timers: Timers::new(),
        keyboard: Box::new(SDLKeyboard::new(event_pump, config.get_keyboard())),
        display: Display::new(vec![Box::new(sdl_display_observer)]),
    };

    // Run emulator
    'running: while state.program_counter + 1 < chip8::memory::MAX_SIZE {
        if state.keyboard.is_quit() {
            break;
        }
        
        for _ in 0..config.ticks_per_frame {
            let pc: usize = state.program_counter;
            if matches.is_present("debug") {
                println!("PC: {:?}", state.program_counter);
                println!("{:?}", state.stack);
                println!("{:?}", state.registers);
                println!("{:?}", state.timers);
                println!("Display:");
                for row in state.display.pixels.iter() {
                    println!("{:?}", &row[..]);
                }
            }
            let bytes: (u8, u8) = (state.memory.ram[pc], state.memory.ram[pc + 1]);
            let instruction: Box<dyn Instruction> = instructions::parse(bytes).unwrap();
            if matches.is_present("debug") {
                println!(
                    "Instruction: {:X} {:X} ({:?})",
                    bytes.0, bytes.1, instruction
                );
                println!("======================================================================");
            }
            instruction.execute(&mut state);
            state.program_counter += 2;
            if matches.is_present("step") && state.keyboard.wait_for_key_press() == 1 {
                break 'running;
            }
        }
        state.display.present();
        state.timers.decrement_timers();
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 40));
    }
}

/*

CHIP-8 emulator architecture

The overall idea is simple, read instructions and execute them.
There are many types of instructions and the effect of an instruction depends on the state of the emulator.
I.e, the effects can depend on the contents of the registers.

This calls for an Instruction trait with a method `evaluate` that takes in the state of the emulator

Instructions are called on variables. You can read variables or you can write variables.

This calls for two traits, Read<T> and Write<T>.
Read<T> should take in State and return T.
Write<T> should take in T and mutate State.

*/
