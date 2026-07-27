use crate::tests::helper::run;
use crate::util::macros::literal;
use crate::{addition, multiplication, vars};

#[test]
fn distributive() {
    // Some cases with constats just in case,
    let (a, b) = vars!(a, b);
    assert_eq!(
        run(multiplication!(literal(2.0), addition!(a, b))),
        "((2 * a) + (2 * b))"
    );

    let (a, b, c) = vars!(a, b, c);
    assert_eq!(
        run(multiplication!(addition!(a, b), literal(3.0), c)),
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
