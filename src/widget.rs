use crate::{Attribute, Node};
use sauron_vdom::builder::element;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub enum Widget {
    Vbox,
    Hbox,
    Button,
    Text(String),
    Block(String),
    TextBox(String),
}

pub fn widget<MSG>(
    widget: Widget,
    attrs: Vec<Attribute<MSG>>,
    children: Vec<Node<MSG>>,
) -> Node<MSG> {
    element(widget, attrs, children)
}

pub fn vbox<MSG>(attrs: Vec<Attribute<MSG>>, children: Vec<Node<MSG>>) -> Node<MSG> {
    widget(Widget::Vbox, attrs, children)
}

pub fn hbox<MSG>(attrs: Vec<Attribute<MSG>>, children: Vec<Node<MSG>>) -> Node<MSG> {
    widget(Widget::Hbox, attrs, children)
}

pub fn button<MSG>(attrs: Vec<Attribute<MSG>>) -> Node<MSG> {
    widget(Widget::Button, attrs, vec![])
}

pub fn text<MSG>(txt: &str) -> Node<MSG> {
    widget(Widget::Text(txt.to_string()), vec![], vec![])
}

pub fn textbox<MSG>(txt: &str) -> Node<MSG> {
    widget(Widget::TextBox(txt.to_string()), vec![], vec![])
}

pub fn block<MSG>(title: &str) -> Node<MSG> {
    widget(Widget::Block(title.to_string()), vec![], vec![])
}
