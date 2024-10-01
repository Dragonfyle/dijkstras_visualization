use crate::SquareStatus;
use crate::{DEFAULT_COLOR, END_COLOR, OFF_COLOR, PATH_COLOR, START_COLOR, VISITED_COLOR};
use web_sys::HtmlElement;
use yew::MouseEvent;

pub enum ModifierKey {
    Ctrl,
    Shift,
    None,
}

pub fn set_square_color(node: &HtmlElement, square_status: SquareStatus) {
    match square_status {
        SquareStatus::On => node.set_class_name(DEFAULT_COLOR),
        SquareStatus::Off => node.set_class_name(OFF_COLOR),
        SquareStatus::Start => node.set_class_name(START_COLOR),
        SquareStatus::End => node.set_class_name(END_COLOR),
        SquareStatus::Path => node.set_class_name(PATH_COLOR),
        SquareStatus::Visited => node.set_class_name(VISITED_COLOR),
    }
}

pub fn set_square_status(square_status_map_entry: &mut SquareStatus, new_status: SquareStatus) {
    *square_status_map_entry = new_status;
}

pub fn drop_first_and_last<T>(vec: &mut Vec<T>) {
    vec.remove(0);
    vec.pop();
}

pub fn set_square_on(node_ref: HtmlElement, square_status_map_entry: &mut SquareStatus) {
    set_square_color(&node_ref, SquareStatus::On);
    set_square_status(square_status_map_entry, SquareStatus::On);
}

pub fn set_square_off(node_ref: HtmlElement, square_status_map_entry: &mut SquareStatus) {
    set_square_color(&node_ref, SquareStatus::Off);
    set_square_status(square_status_map_entry, SquareStatus::Off);
}

pub fn is_square_toggleable(square_status_map_entry: &SquareStatus) -> bool {
    *square_status_map_entry != SquareStatus::On && *square_status_map_entry != SquareStatus::Off
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
