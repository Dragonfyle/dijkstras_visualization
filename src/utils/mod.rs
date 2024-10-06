use crate::NodeStatus;
use crate::{DEFAULT_COLOR, END_COLOR, OFF_COLOR, PATH_COLOR, START_COLOR, VISITED_COLOR};
use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use crate::board::GridNode;
use web_sys::HtmlElement;
use yew::MouseEvent;

pub enum Button {
    Left,
    Right,
    Other,
}

pub enum ModifierKey {
    Ctrl,
    Shift,
    None,
}

pub enum MouseAction {
    Click,
    Move,
}

pub enum ButtonWithModifierKey {
    Left(ModifierKey),
    Right(),
    Other(),
}

pub fn get_click_button(event: &MouseEvent) -> Button {
    match event.button() {
        0 => Button::Left,
        2 => Button::Right,
        _ => Button::Other,
    }
}

pub fn get_move_button(event: &MouseEvent) -> Button {
    match event.buttons() {
        1 => Button::Left,
        2 => Button::Right,
        _ => Button::Other,
    }
}

pub fn set_square_color(node: &HtmlElement, node_status: NodeStatus) {
    match node_status {
        NodeStatus::On => node.set_class_name(DEFAULT_COLOR),
        NodeStatus::Off => node.set_class_name(OFF_COLOR),
        NodeStatus::Start => node.set_class_name(START_COLOR),
        NodeStatus::End => node.set_class_name(END_COLOR),
        NodeStatus::Path => node.set_class_name(PATH_COLOR),
        NodeStatus::Visited => node.set_class_name(VISITED_COLOR),
    }
}

pub fn set_node_status(node_status_map_entry: &mut NodeStatus, new_status: NodeStatus) {
    *node_status_map_entry = new_status;
}

pub fn drop_first_and_last<T>(vec: &mut Vec<T>) {
    vec.remove(0);
    vec.pop();
}

pub fn set_node_on(node_ref: HtmlElement, node_status_map_entry: &mut NodeStatus) {
    set_square_color(&node_ref, NodeStatus::On);
    set_node_status(node_status_map_entry, NodeStatus::On);
}

pub fn set_node_off(node_ref: HtmlElement, node_status_map_entry: &mut NodeStatus) {
    set_square_color(&node_ref, NodeStatus::Off);
    set_node_status(node_status_map_entry, NodeStatus::Off);
}

pub fn is_node_toggleable(node_status_map_entry: &NodeStatus) -> bool {
    *node_status_map_entry == NodeStatus::On || *node_status_map_entry == NodeStatus::Off
}

pub fn get_modifier_key(event: &MouseEvent) -> ModifierKey {
    match event.ctrl_key() {
        true => ModifierKey::Ctrl,
        false => match event.shift_key() {
            true => ModifierKey::Shift,
            false => ModifierKey::None,
        },
    }
}

pub fn set_start_node(
    mut nodes: RefMut<Vec<GridNode>>,
    new_start_id: usize,
    current_start_node_id: Rc<RefCell<Option<usize>>>,
) {
    if let Some(id) = current_start_node_id.borrow().as_ref() {
        if let Some(previous_start_node) = nodes.get(*id) {
            if let Some(previous_start_node) =
                previous_start_node.node_ref.cast::<HtmlElement>()
            {
                set_square_color(&previous_start_node, NodeStatus::On);
                set_node_status(
                    &mut nodes[*id].node_status,
                    NodeStatus::On,
                );
            }
        }
    }

    if let Some(new_start_node) = nodes.get(new_start_id).unwrap().node_ref.cast::<HtmlElement>()
    {
        set_square_color(&new_start_node, NodeStatus::Start);
        set_node_status(
            &mut nodes[new_start_id].node_status,
            NodeStatus::Start,
        );
        current_start_node_id.borrow_mut().replace(new_start_id);
    }
}

pub fn set_end_node(
    mut nodes: RefMut<Vec<GridNode>>,
    new_end_id: usize,
    current_end_node_id: Rc<RefCell<Option<usize>>>,
) {
    if let Some(id) = current_end_node_id.borrow().as_ref() {
        if let Some(previous_end_node) = nodes.get(*id) {
            if let Some(previous_end_node) = previous_end_node.node_ref.cast::<HtmlElement>()
            {
                set_square_color(&previous_end_node, NodeStatus::On);
                set_node_status(
                    &mut nodes[*id].node_status,
                    NodeStatus::On,
                );
            }
        }
    }

    if let Some(new_end_node) = nodes.get(new_end_id).unwrap().node_ref.cast::<HtmlElement>()
    {
        set_square_color(&new_end_node, NodeStatus::End);
        set_node_status(
            &mut nodes[new_end_id].node_status,
            NodeStatus::End,
        );
        current_end_node_id.borrow_mut().replace(new_end_id);
    }
}