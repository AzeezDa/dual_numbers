use super::*;
use std::{f32, f64};

const LN2_32: f32 = 0.6931472;
const LN10_32: f32 = 2.3025851;
const LN2_64: f64 = 0.6931471805599453;
const LN10_64: f64 = 2.302585092994046;

macro_rules! functions {
    ($a:ty, $ln2:ident, $ln10:ident) => {
        #[allow(dead_code)]
        impl Dual<$a> {
            // f(a+bɛ) = f(a) + b*f'(a)ɛ
        
            // |a+bɛ| = |a| + b*sgn(a)ɛ
            pub fn abs(&self) -> Self {
                Self {
                    a: self.a.abs(),
                    b: self.b * self.a.signum(),
                }
            }
        
            // arccos(a+bɛ) = arccos(a) - b/sqrt(1-a^2) ɛ
            pub fn acos(&self) -> Self {
                Self {
                    a: self.a.acos(),
                    b: -self.b / (1.0 - self.a * self.a).sqrt(),
                }
            }
        
            // arccosh(a+bɛ) = acosh(a) + b/sqrt(a^2 - 1) ɛ
            pub fn acosh(&self) -> Self {
                Self {
                    a: self.a.acosh(),
                    b: self.b / (self.a * self.a - 1.0).sqrt(),
                }
            }
        
            // arcsin(a + bɛ) = arcsin(a) + b/sqrt(1 - a^2) ɛ
            pub fn asin(&self) -> Self {
                Self {
                    a: self.a.asin(),
                    b: self.b / (1.0 - self.a * self.a).sqrt()
                }
            }
        
            // arcsinh(a + bɛ) = arcsinh(a) + b/sqrt(a^2 + 1) ɛ
            pub fn asinh(&self) -> Self {
                Self {
                    a: self.a.asinh(),
                    b: self.b / (1.0 + self.a * self.a).sqrt()
                }
            }
        
            // arctan(a + bɛ) = arctan(a) + b/(a^2 + 1) ɛ
            pub fn atan(&self) -> Self {
                Self {
                    a: self.a.atan(),
                    b: self.b / (1.0 + self.a * self.a)
                }
            }
        
            // arctanh(a + bɛ) = arctanh(a) + b/(1-a^2) ɛ
            pub fn atanh(&self) -> Self {
                Self {
                    a: self.a.atanh(),
                    b: self.b / (1.0 - self.a * self.a)
                }
            }
        
            // (a + bɛ)^(1/3) = a^(1/3) + b(a^(-2/3))/3 ɛ
            pub fn cbrt(&self) -> Self {
                Self {
                    a: self.a.cbrt(),
                    b: self.b * self.a.powf(-2.0/3.0) / 3.0
                }
            }
        
            // ⌈a + bɛ⌉ = ⌈a⌉ + 0ɛ
            pub fn ceil(&self) -> Self {
                Self {
                    a: self.a.ceil(),
                    b: 0.0
                }
            }
        
            // cos(a + bɛ) = cos(a) - b*sin(a)ɛ
            pub fn cos(&self) -> Self {
                Self {
                    a: self.a.cos(),
                    b: -self.b * self.a.sin()
                }
            }
        
            // cosh(a + bɛ) = cosh(a) + b*sinh(a)ɛ
            pub fn cosh(&self) -> Self {
                Self {
                    a: self.a.cosh(),
                    b: self.b * self.a.sinh()
                }
            }
        
            // exp(a + bɛ) = exp(a) + b*exp(a)ɛ
            pub fn exp(&self) -> Self {
                let ex = self.a.exp();
                Self {
                    a: ex,
                    b: self.b * ex
                }
            }
        
            // 2^(a + bɛ) = 2^a + b*2^a ɛ
            pub fn exp2(&self) -> Self {
                let ex = self.a.exp2();
        
                Self {
                    a: ex,
                    b: self.b * $ln2 * ex
                }
            }
        
            // exp(a + bɛ) - 1 = exp(a) - 1 + b*exp(a)ɛ
            pub fn exp_m1(&self) -> Self {
                let ex = self.a.exp_m1();
                Self {
                    a: ex,
                    b: self.b * (ex + 1.0)
                }
            }
        
            // ⌊a + bɛ⌋ = ⌊a⌋ + 0ɛ
            pub fn floor(&self) -> Self {
                Self {
                    a: self.a.floor(),
                    b: 0.0
                }
            }
        
            // (a + bɛ) - ⌊a + bɛ⌋ = a - ⌊a⌋ + bɛ
            pub fn frac(&self) -> Self {
                Self {
                    a: self.a.fract(),
                    b: self.b
                }
            }
        
            // ln(a + bɛ) = ln(a) + b/a ɛ
            pub fn ln(&self) -> Self {
                Self {
                    a: self.a.ln(),
                    b: self.b / self.a
                }
            }
        
            // ln(a+1 + bɛ) = ln(a + 1) + b/(a+1) ɛ
            pub fn ln_1p(&self) -> Self {
                Self {
                    a: self.a.ln_1p(),
                    b: self.b / (self.a + 1.0)
                }
            }
        
            // log_c(a + bɛ) = log_c(a) + b/(a*ln(c)) ɛ
            pub fn log(&self, base: $a) -> Self {
                Self {
                    a: self.a.log(base),
                    b: self.b / (self.a * base.ln())
                }
            }
        
            // log_2(a + bɛ) = log_2(a) + b/(a*ln(2)) ɛ
            pub fn log2(&self) -> Self {
                Self {
                    a: self.a.log2(),
                    b: self.b / (self.a * $ln2)
                }
            }
        
            // log_10(a + bɛ) = log_10(a) + b/(a*ln(10)) ɛ
            pub fn log10(&self) -> Self {
                Self {
                    a: self.a.log10(),
                    b: self.b / (self.a * $ln10)
                }
            }
        
            // (a + bɛ)^n = a^n + b*n*a^(n-1)ɛ
            pub fn powf(&self, n: $a) -> Self {
                Self {
                    a: self.a.powf(n),
                    b: self.b * n * self.a.powf(n - 1.0)
                }
            }
        
            // (a + bɛ)^n = a^n + b*n*a^(n-1)ɛ
            pub fn powi(&self, n: i32) -> Self {
                Self {
                    a: self.a.powi(n),
                    b: self.b * n as $a * self.a.powi(n - 1)
                }
            }
        
            // 1/(a + bɛ) = 1/a - b/(a^2)
            pub fn recip(&self) -> Self {
                Self {
                    a: self.a.recip(),
                    b: -self.b / (self.a * self.a)
                }
            }
        
            // [a + bɛ] = [a] + 0ɛ, [] is rounding
            pub fn round(&self) -> Self {
                Self {
                    a: self.a.round(),
                    b: 0.0
                }
            }
        
            // sgn(a + bɛ) = sgn(a) + 0ɛ
            pub fn signum(&self) -> Self {
                Self {
                    a: self.a.signum(),
                    b: 0.0
                }
            }
        
            // sin(a + bɛ) = sin(a) + b*cos(a)ɛ
            pub fn sin(&self) -> Self {
                Self {
                    a: self.a.sin(),
                    b: self.b * self.a.sin()
                }
            }
        
            // sinh(a + bɛ) = sinh(a) + b*cosh(a)ɛ
            pub fn sinh(&self) -> Self {
                Self {
                    a: self.a.sinh(),
                    b: self.b * self.a.cosh()
                }
            }
        
            // (a + bɛ)^(1/2) = a^(1/2) + b/(2*a^(1/2)) ɛ
            pub fn sqrt(&self) -> Self {
                let sq = self.a.sqrt();
                Self {
                    a: sq,
                    b: self.b / (2.0 * sq)
                }
            }
        
            // tan(a + bɛ) = tan(a) + b(1 + tan(a)^2)ɛ
            pub fn tan(&self) -> Self {
                let ta = self.a.tan();
                Self {
                    a: ta,
                    b: self.b * (1.0 + ta * ta)
                }
            }
        
            // tanh(a + bɛ) = tanh(a) + b(1 - tanh(a)^2)ɛ
            pub fn tanh(&self) -> Self {
                let ta = self.a.tanh();
                Self {
                    a: ta, 
                    b: self.b * (1.0 - ta * ta)
                }
            }
        
            // trunc(a + bɛ) = trunc(a) + 0ɛ
            pub fn trunc(&self) -> Self {
                Self {
                    a: self.a.trunc(),
                    b: 0.0
                }
            }
        }
    };
}

functions!(f32, LN2_32, LN10_32);
functions!(f64, LN2_64, LN10_64);