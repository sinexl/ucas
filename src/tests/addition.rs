use crate::addition;
use crate::ast::associative::associative_list::AssociativeList;
use crate::ast::node::Node;
use crate::tests::helper::run;
use crate::util::macros::{number, variable};
use crate::vars;
use crate::{multiplication, Addition};

#[test]
fn x_plus_x() {
    let x = vars!(x);
    assert_eq!(run(addition!(x.clone(), x.clone())), "(2 * x)")
}

#[test]
fn x_minus_x() {
    let x = variable("x");
    assert_eq!(
        run(addition!(
            multiplication!(Node::Float(-1.0), x.clone()),
            x.clone()
        )),
        "0"
    )
}

#[test]
fn flattening() {
    let (a, b, c, d, e, f, g, h, i, j) = vars!(a, b, c, d, e, f, g, h, i, j);

    // (a + (b * (h * (i + (j + (a + b))))) + (c + d + (e + (f + g))
    // = a + (b * (h * (i + j + a + b))) + c + d + e + f + g
    assert_eq!(
        addition!(
            addition!(
                a.clone(),
                multiplication!(
                    b.clone(),
                    multiplication!(h, addition!(i, addition!(j, addition!(a, b))))
                )
            ),
            addition!(c, d, addition!(e, addition!(f, g)))
        )
        .to_string(),
        "(a + c + d + e + f + g + (b * h * (a + b + i + j)))"
    )
}

#[test]
fn x_plus_nx() {
    let (x, y) = vars!(x, y);
    // 2x + 1 + x + 2 + y + 6y
    assert_eq!(
        run(addition! {
            multiplication! { number(2.0), x.clone() },
            number(1.0),
            x.clone(),
            number(2.0),

            y.clone(),
            multiplication! { number(6.0), y.clone() },
        }),
        "(3 + (3 * x) + (7 * y))"
    );
}
#[test]
fn constant_prop() {
    let l = |x| number(x as f32);
    let x = variable("a");
    assert_eq!(
        run(addition!(
            l(1),
            x.clone(),
            addition!(x.clone(), l(6)),
            l(2),
            addition!(l(3), addition!(l(4), l(5)))
        )),
        "(21 + (2 * a))"
    )
}

#[test]
fn identity_clear() {
    assert_eq!(Addition::from_items([]), Node::Float(0.0));

    let x = Node::Float(39.0);
    assert_eq!(addition!(x.clone()), x);
}
