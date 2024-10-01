use crate::dijkstras;
use crate::utils::{self, ModifierKey};
use crate::visualizer;
use crate::{SquareStatus, DEFAULT_COLOR, GRID_SIZE};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::{Html, NodeRef};

const RIGHT_BUTTON: u16 = 2;
const LEFT_BUTTON: u16 = 1;
const NO_BUTTON: u16 = 0;

pub struct SquareNode {
    node: VNode,
    pub node_ref: NodeRef,
}

impl SquareNode {
    fn build(id: usize) -> Self {
        let node_ref = NodeRef::default();
        SquareNode {
            node: html! {
                <div class={DEFAULT_COLOR} id={id.to_string()} ref={node_ref.clone()}></div>
            },
            node_ref,
        }
    }
}

#[function_component]
pub fn Board() -> Html {
    let squares = use_state(|| create_squares());
    let square_status_map = use_mut_ref(|| vec![SquareStatus::On; GRID_SIZE]);
    let current_start_square_id = use_mut_ref(|| Option::<usize>::None);
    let current_end_square_id = use_mut_ref(|| Option::<usize>::None);
    let is_first_render = use_mut_ref(|| true);
    let is_running = use_mut_ref(|| false);

    {
        let squares = Rc::clone(&squares);
        let current_start_square_id = Rc::clone(&current_start_square_id);
        let current_end_square_id = Rc::clone(&current_end_square_id);
        let square_status_map = Rc::clone(&square_status_map);

        use_effect(move || {
            let is_not_first_render = !*is_first_render.borrow();
            if is_not_first_render {
                return;
            }

            *is_first_render.borrow_mut() = false;

            set_start_square(&squares, &square_status_map, 0, current_start_square_id);
            set_end_square(
                &squares,
                &square_status_map,
                GRID_SIZE - 1,
                current_end_square_id,
            );
        });
    }

    let handle_drag_start = {
        Callback::from(|event: DragEvent| {
            event.prevent_default();
        })
    };

    let handle_click = {
        let squares = Rc::clone(&squares);
        let square_status_map = Rc::clone(&square_status_map);
        let current_start_square_id = Rc::clone(&current_start_square_id);

        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            let modifier_key = utils::get_modifier_key(&event);

            if let Some(target) = event
                .target()
                .and_then(|t| t.dyn_into::<HtmlElement>().ok())
            {
                if target.id().is_empty() {
                    return;
                }

                let target_id = target
                    .id()
                    .parse::<usize>()
                    .expect("The id of the element has to be a number");

                let mut square_status_map_borrow = square_status_map.borrow_mut();
                let square_status_map_entry = &mut square_status_map_borrow[target_id];

                if utils::is_square_toggleable(square_status_map_entry) {
                    return;
                }

                if let Some(node_ref) = squares
                    .borrow()
                    .get(target_id)
                    .unwrap()
                    .node_ref
                    .cast::<HtmlElement>()
                {
                    match modifier_key {
                        ModifierKey::Ctrl => {
                            utils::set_square_off(node_ref, square_status_map_entry)
                        }
                        ModifierKey::Shift => {
                            utils::set_square_on(node_ref, square_status_map_entry)
                        }
                        ModifierKey::None => {
                            drop(square_status_map_borrow);
                            set_start_square(
                                &squares,
                                &square_status_map,
                                target_id,
                                Rc::clone(&current_start_square_id),
                            );
                        }
                    };
                }
            }
        })
    };

    let handle_mouse_over = {
        let squares = Rc::clone(&squares);
        let square_status_map = Rc::clone(&square_status_map);
        let current_start_square_id = Rc::clone(&current_start_square_id);
        let current_end_square_id = Rc::clone(&current_end_square_id);
        let square_status_map2 = Rc::clone(&square_status_map);

        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            let mouse_button = event.buttons();
            if mouse_button == NO_BUTTON {
                return;
            }

            let modifier_key = utils::get_modifier_key(&event);
            if let Some(target) = event
                .target()
                .and_then(|t| t.dyn_into::<HtmlElement>().ok())
            {
                if target.id().is_empty() {
                    return;
                }

                let target_id = target
                    .id()
                    .parse::<usize>()
                    .expect("The id of the element has to be a number");
                let mut square_status_map_borrow = square_status_map.borrow_mut();
                let square_status_map_entry = &mut square_status_map_borrow[target_id];

                if utils::is_square_toggleable(square_status_map_entry) {
                    return;
                }

                if let Some(node_ref) = squares
                    .borrow()
                    .get(target_id)
                    .unwrap()
                    .node_ref
                    .cast::<HtmlElement>()
                {
                    match modifier_key {
                        ModifierKey::Ctrl => {
                            utils::set_square_off(node_ref, square_status_map_entry)
                        }
                        ModifierKey::Shift => {
                            utils::set_square_on(node_ref, square_status_map_entry)
                        }
                        ModifierKey::None => {
                            drop(square_status_map_borrow);
                            match mouse_button {
                                LEFT_BUTTON => set_start_square(
                                    &squares,
                                    &square_status_map2,
                                    target_id,
                                    Rc::clone(&current_start_square_id),
                                ),
                                RIGHT_BUTTON => set_end_square(
                                    &squares,
                                    &square_status_map2,
                                    target_id,
                                    Rc::clone(&current_end_square_id),
                                ),
                                _ => return,
                            }
                        }
                    };
                }
            }
        })
    };

    let handle_context_menu = {
        let squares = Rc::clone(&squares);
        let current_end_square_id = Rc::clone(&current_end_square_id);
        let square_status_map = Rc::clone(&square_status_map);

        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            if event.button() as u16 != RIGHT_BUTTON {
                return;
            }

            if let Some(target) = event
                .target()
                .and_then(|t| t.dyn_into::<HtmlElement>().ok())
            {
                let current_end_square_id = Rc::clone(&current_end_square_id);
                set_end_square(
                    &squares,
                    &square_status_map,
                    target.id().parse::<usize>().unwrap(),
                    current_end_square_id,
                );
            }
        })
    };

    fn create_squares() -> Rc<RefCell<Vec<SquareNode>>> {
        let grid_as_vec = Rc::new(RefCell::new(Vec::with_capacity(GRID_SIZE)));

        (0..GRID_SIZE).for_each(|i| {
            let square_node = SquareNode::build(i);
            grid_as_vec.borrow_mut().push(square_node);
        });

        grid_as_vec
    }

    fn clear_traversed_nodes(
        squares: &Rc<RefCell<Vec<SquareNode>>>,
        square_status_map: &Rc<RefCell<Vec<SquareStatus>>>,
    ) {
        (0..GRID_SIZE).for_each(|i| {
            if let Some(node_ref) = squares
                .borrow()
                .get(i)
                .unwrap()
                .node_ref
                .cast::<HtmlElement>()
            {
                let square_status_map_entry = &mut square_status_map.borrow_mut()[i];
                if *square_status_map_entry == SquareStatus::Visited
                    || *square_status_map_entry == SquareStatus::Path
                {
                    utils::set_square_color(&node_ref, SquareStatus::On);
                    utils::set_square_status(square_status_map_entry, SquareStatus::On);
                }
            }
        })
    }

    fn clear_board(
        squares: &Rc<RefCell<Vec<SquareNode>>>,
        square_status_map: &Rc<RefCell<Vec<SquareStatus>>>,
        current_start_square_id: Rc<RefCell<Option<usize>>>,
        current_end_square_id: Rc<RefCell<Option<usize>>>,
    ) {
        let square_status_map2 = Rc::clone(&square_status_map);
        (0..GRID_SIZE).for_each(|i| {
            let square_status_map = Rc::clone(square_status_map);
            square_status_map.borrow_mut()[i] = SquareStatus::On;
            if let Some(node_ref) = squares
                .borrow()
                .get(i)
                .unwrap()
                .node_ref
                .cast::<HtmlElement>()
            {
                utils::set_square_on(node_ref, &mut square_status_map.borrow_mut()[i]);
            }
        });

        *current_start_square_id.borrow_mut() = None;
        *current_end_square_id.borrow_mut() = None;
        set_start_square(squares, &square_status_map2, 0, current_start_square_id);
        set_end_square(
            squares,
            &square_status_map2,
            GRID_SIZE - 1,
            current_end_square_id,
        );
    }

    fn set_start_square(
        squares: &Rc<RefCell<Vec<SquareNode>>>,
        square_status_map: &Rc<RefCell<Vec<SquareStatus>>>,
        new_start_id: usize,
        current_start_square_id: Rc<RefCell<Option<usize>>>,
    ) {
        if let Some(id) = current_start_square_id.borrow().as_ref() {
            let squares_borrow = squares.borrow();
            if let Some(previous_start_square) = squares_borrow.get(*id) {
                if let Some(previous_start_node) =
                    previous_start_square.node_ref.cast::<HtmlElement>()
                {
                    utils::set_square_color(&previous_start_node, SquareStatus::On);
                    utils::set_square_status(
                        &mut square_status_map.borrow_mut()[*id],
                        SquareStatus::On,
                    );
                }
            }
        }

        if let Some(new_start_node) = squares
            .borrow()
            .get(new_start_id)
            .unwrap()
            .node_ref
            .cast::<HtmlElement>()
        {
            utils::set_square_color(&new_start_node, SquareStatus::Start);
            utils::set_square_status(
                &mut square_status_map.borrow_mut()[new_start_id],
                SquareStatus::Start,
            );
            current_start_square_id.borrow_mut().replace(new_start_id);
        }
    }

    fn set_end_square(
        squares: &Rc<RefCell<Vec<SquareNode>>>,
        square_status_map: &Rc<RefCell<Vec<SquareStatus>>>,
        new_end_id: usize,
        current_end_square_id: Rc<RefCell<Option<usize>>>,
    ) {
        if let Some(id) = current_end_square_id.borrow().as_ref() {
            let squares_borrow = squares.borrow();
            if let Some(previous_end_square) = squares_borrow.get(*id) {
                if let Some(previous_end_node) = previous_end_square.node_ref.cast::<HtmlElement>()
                {
                    utils::set_square_color(&previous_end_node, SquareStatus::On);
                    utils::set_square_status(
                        &mut square_status_map.borrow_mut()[*id],
                        SquareStatus::On,
                    );
                }
            }
        }

        if let Some(new_end_node) = squares
            .borrow()
            .get(new_end_id)
            .unwrap()
            .node_ref
            .cast::<HtmlElement>()
        {
            utils::set_square_color(&new_end_node, SquareStatus::End);
            utils::set_square_status(
                &mut square_status_map.borrow_mut()[new_end_id],
                SquareStatus::End,
            );
            current_end_square_id.borrow_mut().replace(new_end_id);
        }
    }

    let handle_clear_board = {
        let square_status_map = Rc::clone(&square_status_map);
        let squares = Rc::clone(&squares);
        let current_start_square_id = Rc::clone(&current_start_square_id);
        let current_end_square_id = Rc::clone(&current_end_square_id);
        let is_running = Rc::clone(&is_running);

        Callback::from(move |_| {
            if *is_running.borrow() {
                return;
            }

            let current_start_square_id = Rc::clone(&current_start_square_id);
            let current_end_square_id = Rc::clone(&current_end_square_id);
            clear_board(
                &squares,
                &square_status_map,
                current_start_square_id,
                current_end_square_id,
            );
        })
    };

    let handle_clear_traversed_nodes = {
        let square_status_map = Rc::clone(&square_status_map);
        let squares = Rc::clone(&squares);
        let is_running = Rc::clone(&is_running);

        Callback::from(move |_| {
            if *is_running.borrow() {
                return;
            }

            clear_traversed_nodes(&squares, &square_status_map);
        })
    };

    let handle_create_adjacency_list = {
        let square_status_map = Rc::clone(&square_status_map);
        let current_start_square_id = Rc::clone(&current_start_square_id);
        let current_end_square_id = Rc::clone(&current_end_square_id);
        let squares = Rc::clone(&squares);
        let is_running = Rc::clone(&is_running);

        Callback::from(move |_: MouseEvent| {
            if *is_running.borrow() {
                return;
            }
            *is_running.borrow_mut() = true;

            let is_running = Rc::clone(&is_running);
            let end_of_visualization_callback = move || {
                *is_running.borrow_mut() = false;
            };

            clear_traversed_nodes(&squares, &square_status_map);
            let traversed_nodes = dijkstras::get_traversed_nodes(
                square_status_map.borrow().clone(),
                current_start_square_id.borrow().unwrap(),
                current_end_square_id.borrow().unwrap(),
            );

            visualizer::visualize(
                Rc::clone(&squares),
                Rc::clone(&square_status_map),
                traversed_nodes.visited_ordered,
                traversed_nodes.path,
                end_of_visualization_callback,
            );
        })
    };

    html! {
        <>
            <div class="flex gap-20 justify-center items-center bg-gray-900 h-screen w-screen">
                 <div
                    class="grid gap-0.25 grid-cols-50 grid-rows-50 bg-gray-900 h-800 w-800 border-2 border-teal-700 "
                    onclick={handle_click}
                    oncontextmenu={handle_context_menu}
                    onmouseover={handle_mouse_over}
                    ondragstart={handle_drag_start}
                    >
                    {squares.borrow().iter().map(|SquareNode {node, ..}| node.clone()).collect::<Html>()}
                 </div>

                 <button class="text-white" onclick={handle_create_adjacency_list}>{"Find Path"}</button>
                 <button class="text-white" onclick={handle_clear_traversed_nodes}>{"Clear Path"}</button>
                 <button class="text-white" onclick={handle_clear_board}>{"Reset board"}</button>
            </div>
        </>
    }
}
