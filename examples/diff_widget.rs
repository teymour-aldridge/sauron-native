use sauron_native::{
    widget::{attribute::*, *},
    AttribKey, Node, *,
};
use sauron_vdom::diff_with_key;

fn main() {
    let old: Node<()> = column(
        vec![],
        vec![
            row(
                vec![],
                vec![
                    button(vec![label("column1 element1")]),
                    button(vec![label("column1 element2")]),
                    button(vec![label("column1 element3")]),
                    button(vec![label("column1 element4")]),
                    button(vec![label("column1 element5")]),
                    button(vec![label("column1 element6")]),
                ],
            ),
            row(
                vec![],
                vec![
                    button(vec![label("column2")]),
                    button(vec![label("c2 element2")]),
                ],
            ),
            button(vec![label("Hello")]),
            text("I'm a text kid!"),
            text(
                "Hello, will this be a paragrapah\n
                    The quick brown fox jumps over the lazy\n
                    dog. Lorem ipsun\n
                    The shadows of mordor\n
                     ",
            ),
        ],
    );

    let new: Node<()> = column(
        vec![],
        vec![
            row(
                vec![],
                vec![
                    button(vec![label("Changed column1 element1")]),
                    button(vec![label("column1 element2")]),
                    button(vec![label("column1 element3")]),
                    button(vec![label("column1 element4")]),
                    button(vec![label("Changed column1 element5")]),
                    button(vec![label("Changed column1 element6")]),
                ],
            ),
            row(
                vec![],
                vec![
                    button(vec![label("column2-changed")]),
                    button(vec![label("c2 element2")]),
                ],
            ),
            button(vec![label("Hello")]),
            text("I'm a text kid!"),
            text(
                "Hello, will this be a paragrapah\n
                    The quick brown fox jumps over the lazy\n
                    dog. Lorem ipsun\n
                    The shadows of mordor\n
                     ",
            ),
        ],
    );

    let changed = diff_with_key(&old, &new, &AttribKey::Key);
    println!("changed: {:#?}", changed);
}
