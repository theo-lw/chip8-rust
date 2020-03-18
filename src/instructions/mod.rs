mod add;
mod and;
mod call;
mod cls;
mod drw;
mod helpers;
mod jp;
mod ld;
mod or;
mod ret;
mod rnd;
mod se;
mod shl;
mod shr;
mod sknp;
mod skp;
mod sne;
mod sub;
mod sys;
mod xor;

use super::chip8::State;
use crate::variables::{
    bcd::BCD, byte::B8, delay_timer::DT, font::F, iregister::I, key::K, memory_at::AT, nibble::B4,
    range::RANGE, sound_timer::ST, tribble::B12, vregister::V,
};
use add::{ADD, ADDF};
use and::AND;
use call::CALL;
use cls::CLS;
use drw::DRW;
use jp::JP;
use ld::LD;
use or::OR;
use ret::RET;
use rnd::RND;
use se::SE;
use shl::SHL;
use shr::SHR;
use sknp::SKNP;
use skp::SKP;
use sne::SNE;
use std::fmt::Debug;
use sub::{SUB, SUBN};
use sys::SYS;
use xor::XOR;

/// A trait for instructions.
///
/// Defines one method, `execute(&self, &mut State)` because
/// instructions should be able to be executed in the context of a State struct
pub trait Instruction: Debug {
    fn execute(&self, state: &mut State);
}

/// Error for when an instruction can't be parsed
#[derive(Debug, Clone)]
pub struct InstructionError(String);

pub fn parse(instruction: (u8, u8)) -> Result<Box<dyn Instruction>, InstructionError> {
    let (first, second): (B4, B4) = B4::from_u8(instruction.0);
    let (third, fourth): (B4, B4) = B4::from_u8(instruction.1);
    match (first, second, third, fourth) {
        (B4(0x0), B4(0x0), B4(0xE), B4(0x0)) => Ok(Box::new(CLS)),
        (B4(0x0), B4(0x0), B4(0xE), B4(0xE)) => Ok(Box::new(RET)),
        (B4(0x0), _, _, _) => Ok(Box::new(SYS(B12(second, third, fourth)))),
        (B4(0x1), _, _, _) => Ok(Box::new(JP(B4(0), B12(second, third, fourth)))),
        (B4(0x2), _, _, _) => Ok(Box::new(CALL(B12(second, third, fourth)))),
        (B4(0x3), _, _, _) => Ok(Box::new(SE(V(second), B8(third, fourth)))),
        (B4(0x4), _, _, _) => Ok(Box::new(SNE(V(second), B8(third, fourth)))),
        (B4(0x5), _, _, B4(0x0)) => Ok(Box::new(SE(V(second), V(third)))),
        (B4(0x6), _, _, _) => Ok(Box::new(LD::new(V(second), B8(third, fourth)))),
        (B4(0x7), _, _, _) => Ok(Box::new(ADD::new(V(second), B8(third, fourth)))),
        (B4(0x8), _, _, B4(0x0)) => Ok(Box::new(LD::new(V(second), V(third)))),
        (B4(0x8), _, _, B4(0x1)) => Ok(Box::new(OR(V(second), V(third)))),
        (B4(0x8), _, _, B4(0x2)) => Ok(Box::new(AND(V(second), V(third)))),
        (B4(0x8), _, _, B4(0x3)) => Ok(Box::new(XOR(V(second), V(third)))),
        (B4(0x8), _, _, B4(0x4)) => Ok(Box::new(ADDF::new(V(second), V(third)))),
        (B4(0x8), _, _, B4(0x5)) => Ok(Box::new(SUB(V(second), V(third)))),
        (B4(0x8), _, _, B4(0x6)) => Ok(Box::new(SHR(V(second)))),
        (B4(0x8), _, _, B4(0x7)) => Ok(Box::new(SUBN(V(second), V(third)))),
        (B4(0x8), _, _, B4(0xE)) => Ok(Box::new(SHL(V(second)))),
        (B4(0x9), _, _, B4(0x0)) => Ok(Box::new(SNE(V(second), V(third)))),
        (B4(0xA), _, _, _) => Ok(Box::new(LD::new(I, B12(second, third, fourth)))),
        (B4(0xB), _, _, _) => Ok(Box::new(JP(V(B4(0)), B12(second, third, fourth)))),
        (B4(0xC), _, _, _) => Ok(Box::new(RND(V(second), B8(third, fourth)))),
        (B4(0xD), _, _, _) => Ok(Box::new(DRW(V(second), V(third), fourth))),
        (B4(0xE), _, B4(0x9), B4(0xE)) => Ok(Box::new(SKP(V(second)))),
        (B4(0xE), _, B4(0xA), B4(0x1)) => Ok(Box::new(SKNP(V(second)))),
        (B4(0xF), _, B4(0x0), B4(0x7)) => Ok(Box::new(LD::new(V(second), DT))),
        (B4(0xF), _, B4(0x0), B4(0xA)) => Ok(Box::new(LD::new(V(second), K))),
        (B4(0xF), _, B4(0x1), B4(0x5)) => Ok(Box::new(LD::new(DT, V(second)))),
        (B4(0xF), _, B4(0x1), B4(0x8)) => Ok(Box::new(LD::new(ST, V(second)))),
        (B4(0xF), _, B4(0x1), B4(0xE)) => Ok(Box::new(ADD::new(I, V(second)))),
        (B4(0xF), _, B4(0x2), B4(0x9)) => Ok(Box::new(LD::new(I, F(V(second))))),
        (B4(0xF), _, B4(0x3), B4(0x3)) => {
            let arg = RANGE(vec![AT(I, 0), AT(I, 1), AT(I, 2)]);
            Ok(Box::new(LD::new(arg, BCD(V(second)))))
        }
        (B4(0xF), B4(x), B4(0x5), B4(0x5)) => {
            let memory_at = RANGE(
                (0usize..=x.into())
                    .map(|y| AT(I, y))
                    .collect::<Vec<AT<I>>>(),
            );
            let registers = RANGE((0u8..=x).map(|y| V(B4(y))).collect::<Vec<V<B4>>>());
            Ok(Box::new(LD::new(memory_at, registers)))
        }
        (B4(0xF), B4(x), B4(0x6), B4(0x5)) => {
            let memory_at = RANGE(
                (0usize..=x.into())
                    .map(|y| AT(I, y))
                    .collect::<Vec<AT<I>>>(),
            );
            let registers = RANGE((0u8..=x).map(|y| V(B4(y))).collect::<Vec<V<B4>>>());
            Ok(Box::new(LD::new(registers, memory_at)))
        }
        _ => Err(InstructionError(format!(
            "Could not parse instruction: {:?}",
            instruction
        ))),
    }
}
