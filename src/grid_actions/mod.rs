use crate::board::GridNode;
use crate::utils::{self, ModifierKey};
use crate::NodeStatus;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use yew::prelude::*;
use utils::{Button, ButtonWithModifierKey, MouseAction};

pub trait TouchSquare {
    fn trigger_node(&self);
}

pub struct GridState {
    nodes: Rc<RefCell<Vec<GridNode>>>,
    node_status_map: Rc<RefCell<Vec<NodeStatus>>>,
    current_start_node_id: Rc<RefCell<Option<usize>>>,
    current_end_node_id: Rc<RefCell<Option<usize>>>,
}

impl GridState {
    pub fn new(nodes: Rc<RefCell<Vec<GridNode>>>, node_status_map: Rc<RefCell<Vec<NodeStatus>>>, current_start_node_id: Rc<RefCell<Option<usize>>>, current_end_node_id: Rc<RefCell<Option<usize>>>) -> GridState {
        GridState {
            nodes,
            node_status_map,
            current_start_node_id,
            current_end_node_id,
        }
    }
}

pub struct GridAction {
    html_element: HtmlElement,
    node_id: usize,
    grid_state: GridState,
    pub button_with_modifier: ButtonWithModifierKey,
}

impl GridAction {
   pub fn new(event: &MouseEvent, mouse_action: MouseAction, grid_state: GridState) -> Option<Self> {
        let html_element = event.target().and_then(|t| t.dyn_into::<HtmlElement>().ok())?;
         
        if html_element.id().is_empty() {
            return None;
        }

        let button = match mouse_action {
            MouseAction::Click => utils::get_click_button(&event),
            MouseAction::Move => utils::get_move_button(&event),
        };
        let modifier_key = utils::get_modifier_key(&event);
        let node_id = html_element.id().parse::<usize>().ok()?;

        let button_with_modifier = match button {
            Button::Left => ButtonWithModifierKey::Left(modifier_key),
            Button::Right => ButtonWithModifierKey::Right(),
            Button::Other => ButtonWithModifierKey::Other(),
        };

        Some(Self {
            html_element,
            node_id,
            grid_state,
            button_with_modifier,
        })
    }
}

impl TouchSquare for GridAction {
    fn trigger_node(&self) {
        let mut node_status_map = self.grid_state.node_status_map.borrow_mut();
        let grid_status_map_entry = &mut node_status_map[self.node_id];
        if !utils::is_node_toggleable(&grid_status_map_entry) {
            return;
        }
        let current_start_node_id = Rc::clone(&self.grid_state.current_start_node_id);
        let current_end_node_id = Rc::clone(&self.grid_state.current_end_node_id);


        match self.button_with_modifier {
            ButtonWithModifierKey::Left(ModifierKey::Ctrl) => {
                utils::set_node_off(self.html_element.clone(), grid_status_map_entry);
            }
            ButtonWithModifierKey::Left(ModifierKey::Shift) => {
                utils::set_node_on(self.html_element.clone(), grid_status_map_entry);
            }
            ButtonWithModifierKey::Left(ModifierKey::None) => {
                drop(node_status_map);
                utils::set_start_node(&self.grid_state.nodes, &self.grid_state.node_status_map, self.node_id, current_start_node_id);
            }
            ButtonWithModifierKey::Right() => {
                drop(node_status_map);
                utils::set_end_node(&self.grid_state.nodes, &self.grid_state.node_status_map, self.node_id, current_end_node_id);
            }
            ButtonWithModifierKey::Other() => {
                return;
            }
        }
    }
}