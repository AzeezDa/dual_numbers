#[allow(dead_code)]
mod dual_numbers;

pub use self::{dual_numbers::*, util::*};

#[allow(dead_code)]
#[allow(unused_macros)]
#[allow(unused_imports)]
mod util {
    use super::Dual;
    pub type Dual32 = Dual<f32>;
    pub type Dual64 = Dual<f64>;

    macro_rules! dual {
        ($a:expr, $b:expr) => {
            Dual {a: $a, b: $b}
        };
    }
    pub (crate) use dual;
}
