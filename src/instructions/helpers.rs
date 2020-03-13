/// Helper macro to implement traits for types with overflowing operations
#[macro_export]
macro_rules! overflow_op {
    ($trait_name:ident, $method:ident, $t:ty) => {
        impl $trait_name for $t {
            #[inline]
            fn $method(&self, v: &$t) -> ($t, bool) {
                <$t>::$method(*self, *v)
            }
        }
    };
}
