use crate::consts::pi;

pub(crate) fn gcd(a: f64, b: f64) -> f64 {
    if b != 0.0 { gcd(b, (a % b).floor()) } else { a }
}

pub(crate) fn gamma(mut n: f64) -> f64 {
    let g = 15usize;

    // magic.
    let p = [
        0.99999999999980993, 676.5203681218851, -1259.1392167224028, 771.32342877765313,
        -176.61502916214059, 12.507343278686905, -0.13857109526572012, 9.9843695780195716e-6,
        1.5056327351493116e-7,
    ];

    if n < 0.5 {
        return pi() / (pi() * n).sin() / gamma(1.0 - n);
    } else {
        n -= 1.0;
        let mut x = p[0];
        for i in 1..g {
            x += p[i] / (n + (i as f64));
        }

        let t = n + (g as f64) + 0.5;
        return (2.0 * pi()).sqrt() * t.powf(n + 0.5) * (-t).exp() * x;
    }
}