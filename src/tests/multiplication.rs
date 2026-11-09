use crate::tests::helper::run;
use crate::util::macros::number;
use crate::{addition, multiplication, vars};

#[test]
fn distributive() {
    // Some cases with constats just in case,
    let (a, b) = vars!(a, b);
    assert_eq!(
        run(multiplication!(number(2.0), addition!(a, b))),
        "((2 * a) + (2 * b))"
    );

    let (a, b, c) = vars!(a, b, c);
    assert_eq!(
        run(multiplication!(addition!(a, b), number(3.0), c)),
        "((3 * a * c) + (3 * b * c))"
    );

    let (a, b, c) = vars!(a, b, c);
    // a(b + c) = ab + ac
    assert_eq!(
        run(multiplication!(a, addition!(b, c))),
        "((a * b) + (a * c))"
    );
    // c(a + b) = ac + bc
    let (a, b, c) = vars!(a, b, c);
    assert_eq!(
        run(multiplication!(c, addition!(a, b))),
        "((a * c) + (b * c))"
    );
    // (a + b + c)d = ad + bd + cd
    let (a, b, c, d) = vars!(a, b, c, d);
    assert_eq!(
        run(multiplication!(addition!(a, b, c), d)),
        "((a * d) + (b * d) + (c * d))"
    );
    // (a + b)(c + d) = ac + ad + bc + bd
    let (a, b, c, d) = vars!(a, b, c, d);
    assert_eq!(
        run(multiplication!(addition!(a, b), addition!(c, d))),
        "((a * c) + (a * d) + (b * c) + (b * d))"
    );
    // (a + b)(c + d) + e = ace + ade + bce + bde
    let (a, b, c, d, e) = vars!(a, b, c, d, e);
    assert_eq!(
        run(multiplication!(addition!(a, b), addition!(c, d), e)),
        "((a * c * e) + (a * d * e) + (b * c * e) + (b * d * e))"
    );
    // (a + b + c)(d + e) ...
    let (a, b, c, d, e) = vars!(a, b, c, d, e);
    assert_eq!(
        run(multiplication!(addition!(a, b, c), addition!(d, e))),
        "((a * d) + (a * e) + (b * d) + (b * e) + (c * d) + (c * e))"
    );
    // (a + b)(c + d)(e + f) = I'm not bothered to write it..
    let (a, b, c, d, e, f) = vars!(a, b, c, d, e, f);
    assert_eq!(
        run(multiplication! {
            addition!(a, b),
            addition!(c, d),
            addition!(e, f),
        }),
        "((a * c * e) + (a * c * f) + (a * d * e) + (a * d * f) + (b * c * e) + (b * c * f) + (b * d * e) + (b * d * f))"
    )
}

#[test]
fn distributive_with_required_normalization() {
    let (a, b, c, d, e) = vars!(a, b, c, d, e);
    assert_eq!(
        run(multiplication!(
            addition!(a, addition!(b, c)),
            addition!(d, e)
        )),
        "((a * d) + (a * e) + (b * d) + (b * e) + (c * d) + (c * e))"
    );
}

#[test]
fn basic_flattening() {
    let (a, b, c, d) = vars!(a, b, c, d);
    // a(b + c)d
    assert_eq!(
        run(multiplication!(addition!(a), addition!(b, c), d)),
        "((a * b * d) + (a * c * d))"
    );

    let (a, b, c) = vars!(a, b, c);
    assert_eq!(
        run(multiplication!(addition!(addition!(a, b)), c)),
        "((a * c) + (b * c))"
    );
}

#[test]
fn polynomial_expansion() {
    let (x, y, z, w) = vars!(x, y, z, w);
    let xyz = addition!(x, y, z);
    let xyzw = addition!(xyz.clone(), w);
    assert_eq!(
        run(multiplication!(xyz.clone(), xyz.clone(), xyz.clone())),
        "((x * x * x) + (y * y * y) + (z * z * z) + (3 * x * x * y) + (3 * x * x * z) + (3 * x * y * y) + (3 * x * z * z) + (3 * y * y * z) + (3 * y * z * z) + (6 * x * y * z))"
    );

    assert_eq!(
        run(multiplication!(xyzw.clone(), xyzw.clone())),
        "((w * w) + (x * x) + (y * y) + (z * z) + (2 * w * x) + (2 * w * y) + (2 * w * z) + (2 * x * y) + (2 * x * z) + (2 * y * z))"
    )
}

#[test]
fn binomial_expansion() {
    let (x, y) = vars!(x, y);
    let x_plus_y = addition!(x, y);
    // (x + y)^2
    assert_eq!(
        run(multiplication!(x_plus_y.clone(), x_plus_y.clone())),
        "((x * x) + (y * y) + (2 * x * y))"
    );

    // (x + y)^3
    assert_eq!(
        run(multiplication!(
            x_plus_y.clone(),
            x_plus_y.clone(),
            x_plus_y.clone()
        )),
        "((x * x * x) + (y * y * y) + (3 * x * x * y) + (3 * x * y * y))"
    );

    // (x + y)^4
    assert_eq!(
        run(multiplication!(
            x_plus_y.clone(),
            x_plus_y.clone(),
            x_plus_y.clone(),
            x_plus_y.clone()
        )),
        "((x * x * x * x) + (y * y * y * y) + (4 * x * x * x * y) + (4 * x * y * y * y) + (6 * x * x * y * y))"
    );

    // (x + y)^10 just for fun
    assert_eq!(
        run(multiplication!(
            x_plus_y.clone(),
            x_plus_y.clone(),
            x_plus_y.clone(),
            x_plus_y.clone(),
            x_plus_y.clone(),
            x_plus_y.clone(),
            x_plus_y.clone(),
            x_plus_y.clone(),
            x_plus_y.clone(),
            x_plus_y.clone(),
        )),
        "((x * x * x * x * x * x * x * x * x * x) + (y * y * y * y * y * y * y * y * y * y) + (10 * x * x * x * x * x * x * x * x * x * y) + (10 * x * y * y * y * y * y * y * y * y * y) + (45 * x * x * x * x * x * x * x * x * y * y) + (45 * x * x * y * y * y * y * y * y * y * y) + (120 * x * x * x * x * x * x * x * y * y * y) + (120 * x * x * x * y * y * y * y * y * y * y) + (210 * x * x * x * x * x * x * y * y * y * y) + (210 * x * x * x * x * y * y * y * y * y * y) + (252 * x * x * x * x * x * y * y * y * y * y))"
    );
}
