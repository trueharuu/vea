use crate::{ imaginary::i, math_fns::gamma };

pub fn one() -> f64 {
    1.0
}

pub fn two() -> f64 {
    2.0
}

pub fn one_half() -> f64 {
    0.5
}

pub fn pi() -> f64 {
    3.14159_26535_89793_23846
}

pub fn tau() -> f64 {
    6.28318_53071_79586_47692
}

pub fn sqrt_2() -> f64 {
    1.41421_35623_73095_04880
}

pub fn sqrt_3() -> f64 {
    1.73205_08075_68877_29352
}

pub fn sqrt_5() -> f64 {
    2.23606_79774_99789_69640
}

pub fn phi() -> f64 {
    (one() + sqrt_5()) / two()
}

pub fn silver_ratio() -> f64 {
    sqrt_2() + one()
}

pub fn zero() -> f64 {
    0.0
}

pub fn negative_one() -> f64 {
    -1.0
}

pub fn cbrt_2() -> f64 {
    1.44224_95703_07408_38232
}

pub fn twelve_rt_2() -> f64 {
    1.05946_30943_59295_26456
}

pub fn supergolden_ratio() -> f64 {
    (1.0 +
        ((29.0 + 3.0 * (93.0f64).cbrt()) / 2.0).cbrt() +
        ((29.0 - 3.0 * (93.0f64).cbrt()) / 2.0).cbrt()) /
        3.0
}

pub fn imaginary_unit() -> i {
    i(1.0)
}

pub fn connective_constant_for_hexagonal_lattice() -> f64 {
    (2.0 + sqrt_2()).sqrt()
}

pub fn kepler_bouwkamp(precision: usize) -> f64 {
    (3..precision).map(|x| (pi() / (x as f64)).cos()).product()
}

pub fn wallis() -> f64 {
    ((45.0 - (1929.0f64).sqrt()) / 18.0).cbrt() + ((45.0 + (1929.0f64).sqrt()) / 18.0).cbrt()
}

pub fn e(precision: usize) -> f64 {
    1.0 + (0..precision).map(|x| 1.0 / ((1..=x).product::<usize>() as f64)).sum::<f64>()
}

pub fn ln_2(precision: usize) -> f64 {
    (1..precision).map(|x| (-1.0f64).powi((x as i32) + 1) / (x as f64)).sum()
}

pub fn lemniscate() -> f64 {
    (1.0 / 4.0) * (2.0 / pi()).sqrt() * gamma(1.0 / 4.0).powf(2.0)
}

pub fn eulers_constant(n: usize) -> f64 {
    -(n as f64).ln() + (1..n).map(|x| 1f64 / (x as f64)).sum::<f64>()
}

pub fn erdos_borwein(n: usize) -> f64 {
    (1..n).map(|x| 1.0 / ((2f64).powi(x as i32) - 1.0)).sum()
}



/// positive root of -6+3x-6x^{2}+12x^{3}-4x^{4}+7x^{5}-7x^{6}+x^{7}+5x^{9}-2x^{10}-4x^{11}-12x^{12}+2x^{13}+7x^{14}+12x^{15}-7x^{16}-10x^{17}-4x^{18}+3x^{19}+9x^{20}-7x^{21}-8x^{23}+14x^{24}-3x^{25}+9x^{26}+2x^{27}-3x^{28}-10x^{29}-2x^{30}-6x^{31}+x^{32}+10x^{33}-3x^{34}+x^{35}+7x^{36}-7x^{37}+7x^{38}-12x^{39}-5x^{40}+8x^{41}+6x^{42}+10x^{43}-8x^{44}-8x^{45}-7x^{46}-3x^{47}+9x^{48}+x^{49}+6x^{50}+6x^{51}-2x^{52}-3x^{53}-10x^{54}-2x^{54}+3x^{56}+5x^{57}+2x^{58}-x^{59}-x^{60}-x^{61}-x^{62}-x^{63}+x^{64}+2x^{65}+2x^{66}-x^{67}-2x^{68}-x^{69}+x^{71}=0
pub fn conway() -> f64 {
    1.30357_72690_34296_39125
}