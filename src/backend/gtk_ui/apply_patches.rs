use crate::Patch;
use gtk::{prelude::*, ApplicationWindow, Button, Container, ContainerExt, Widget};
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    rc::Rc,
};

pub fn apply_patches<MSG>(root_node: &Rc<ApplicationWindow>, patches: &Vec<Patch<MSG>>)
where
    MSG: Debug,
{
    let container: &Container = root_node.upcast_ref();
    let contents = container.get_children();
    //TODO: this doesn't really need the application window,
    //just the gtk::Box will do just fine
    // TODO add special case of TextView
    let gbox: &Container = contents[0]
        .downcast_ref()
        .expect("must have the first component");
    let nodes_to_patch = find_nodes(gbox, patches);
    println!("nodes to patch: {:#?}", nodes_to_patch);

    for patch in patches {
        let patch_node_idx = patch.node_idx();
        println!("patching for {}", patch_node_idx);
        let widget = nodes_to_patch
            .get(&patch_node_idx)
            .expect("must have a node to patch");
        println!("patching this widget: {:?}", widget);
        if let Some(button) = widget.downcast_ref::<Button>() {
            println!("this is a button");
            println!("with a label of: {:?}", button.get_label());
            match patch {
                Patch::AddAttributes(_node_idx, attrs) => {
                    for att in attrs {
                        println!("att: {:?}", att);
                        if att.name == "value" {
                            if let Some(value) = att.get_value() {
                                println!("value: {:?}", value);
                                button.set_label(&value.to_string());
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

fn find_nodes<MSG>(root_node: &Container, patches: &[Patch<MSG>]) -> HashMap<usize, Widget> {
    let mut nodes_to_find = HashSet::new();
    let mut cur_node_idx = 0;

    for patch in patches {
        nodes_to_find.insert(patch.node_idx());
    }
    println!("nodes to find: {:#?}", nodes_to_find);
    find_nodes_recursive(root_node, &mut cur_node_idx, &nodes_to_find)
}

fn find_nodes_recursive(
    root_node: &Container,
    cur_node_idx: &mut usize,
    nodes_to_find: &HashSet<usize>,
) -> HashMap<usize, Widget> {
    let mut nodes_to_patch: HashMap<usize, Widget> = HashMap::new();
    println!("cur_node_idx: {}", cur_node_idx);

    if nodes_to_find.get(&cur_node_idx).is_some() {
        let root_widget: Widget = root_node.clone().upcast();
        println!(" --- >> found here: {}", cur_node_idx);
        if let Some(btn) = root_widget.downcast_ref::<Button>() {
            println!("with a label of: {:?}", btn.get_label());
        }
        nodes_to_patch.insert(*cur_node_idx, root_widget);
    }

    let children = root_node.get_children();
    let child_node_count = children.len();

    *cur_node_idx += 1;

    for i in 0..child_node_count {
        let child_node = children[i].clone();
        if let Some(container) = child_node.downcast_ref::<Container>() {
            let child_nodes_to_patch = find_nodes_recursive(container, cur_node_idx, nodes_to_find);
            nodes_to_patch.extend(child_nodes_to_patch);
        }
    }
    nodes_to_patch
}