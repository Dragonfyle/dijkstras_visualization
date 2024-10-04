use crate::dijkstras;
use crate::utils;
use crate::visualizer;
use crate::{NodeStatus, DEFAULT_COLOR, GRID_SIZE};
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::{Html, NodeRef};
use crate::grid_actions::{GridAction, TouchSquare, GridState};
use crate::utils::MouseAction;

pub struct GridNode {
    node: VNode,
    pub node_ref: NodeRef,
}

impl GridNode {
    fn build(id: usize) -> Self {
        let node_ref = NodeRef::default();
        GridNode {
            node: html! {
                <div class={DEFAULT_COLOR} id={id.to_string()} ref={node_ref.clone()}></div>
            },
            node_ref,
        }
    }
}

#[function_component]
pub fn Board() -> Html {
    let nodes = use_state(|| create_nodes());
    let node_satus_map = use_mut_ref(|| vec![NodeStatus::On; GRID_SIZE]);
    let current_start_node_id = use_mut_ref(|| Option::<usize>::None);
    let current_end_node_id = use_mut_ref(|| Option::<usize>::None);
    let is_first_render = use_mut_ref(|| true);
    let is_running = use_mut_ref(|| false);

    {
        let nodes = Rc::clone(&nodes);
        let current_start_node_id = Rc::clone(&current_start_node_id);
        let current_end_node_id = Rc::clone(&current_end_node_id);
        let node_status_map = Rc::clone(&node_satus_map);

        use_effect(move || {
            let is_not_first_render = !*is_first_render.borrow();
            if is_not_first_render {
                return;
            }

            *is_first_render.borrow_mut() = false;

            utils::set_start_node(&nodes, &node_status_map, 0, current_start_node_id);
            utils::set_end_node(
                &nodes,
                &node_status_map,
                GRID_SIZE - 1,
                current_end_node_id,
            );
        });
    }

    let handle_drag_start = {
        Callback::from(|event: DragEvent| {
            event.prevent_default();
        })
    };

    let handle_click = {
        let nodes = Rc::clone(&nodes);
        let node_status_map = Rc::clone(&node_satus_map);
        let current_start_node_id = Rc::clone(&current_start_node_id);
        let current_end_node_id = Rc::clone(&current_end_node_id);

        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            let grid_state = GridState::new(
                Rc::clone(&nodes),
                Rc::clone(&node_status_map),
                Rc::clone(&current_start_node_id),
                Rc::clone(&current_end_node_id),
            );
            let grid_action = GridAction::new(&event, MouseAction::Click, grid_state);

            if let Some(grid_action) = grid_action {
                grid_action.trigger_node();
            }

        })
    };

    let handle_mouse_over = {
        let nodes = Rc::clone(&nodes);
        let node_status_map = Rc::clone(&node_satus_map);
        let current_start_node_id = Rc::clone(&current_start_node_id);
        let current_end_node_id = Rc::clone(&current_end_node_id);

        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            let grid_state = GridState::new(
                Rc::clone(&nodes),
                Rc::clone(&node_status_map),
                Rc::clone(&current_start_node_id),
                Rc::clone(&current_end_node_id),
            );
            let grid_action = GridAction::new(&event, MouseAction::Move, grid_state);

            if let Some(grid_action) = grid_action {
                grid_action.trigger_node();
            }
        })
    };

    let handle_context_menu = {
        let nodes = Rc::clone(&nodes);
        let current_end_node_id = Rc::clone(&current_end_node_id);
        let current_start_node_id = Rc::clone(&current_start_node_id);
        let node_status_map = Rc::clone(&node_satus_map);

        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            let grid_state = GridState::new(
                Rc::clone(&nodes),
                Rc::clone(&node_status_map),
                Rc::clone(&current_start_node_id),
                Rc::clone(&current_end_node_id),
            );
            let grid_action = GridAction::new(&event, MouseAction::Click, grid_state);

            if let Some(grid_action) = grid_action {
                grid_action.trigger_node();
            }
        })
    };

    fn create_nodes() -> Rc<RefCell<Vec<GridNode>>> {
        let grid_as_vec = Rc::new(RefCell::new(Vec::with_capacity(GRID_SIZE)));

        (0..GRID_SIZE).for_each(|i| {
            let grid_node = GridNode::build(i);
            grid_as_vec.borrow_mut().push(grid_node);
        });

        grid_as_vec
    }

    fn clear_traversed_nodes(
        nodes: &Rc<RefCell<Vec<GridNode>>>,
        node_status_map: &Rc<RefCell<Vec<NodeStatus>>>,
    ) {
        (0..GRID_SIZE).for_each(|i| {
            if let Some(node_ref) = nodes
                .borrow()
                .get(i)
                .unwrap()
                .node_ref
                .cast::<HtmlElement>()
            {
                let node_status_map_entry = &mut node_status_map.borrow_mut()[i];
                if *node_status_map_entry == NodeStatus::Visited
                    || *node_status_map_entry == NodeStatus::Path
                {
                    utils::set_square_color(&node_ref, NodeStatus::On);
                    utils::set_node_status(node_status_map_entry, NodeStatus::On);
                }
            }
        })
    }

    fn clear_board(
        nodes: &Rc<RefCell<Vec<GridNode>>>,
        node_status_map: &Rc<RefCell<Vec<NodeStatus>>>,
        current_start_node_id: Rc<RefCell<Option<usize>>>,
        current_end_node_id: Rc<RefCell<Option<usize>>>,
    ) {
        let node_status_map2 = Rc::clone(&node_status_map);
        (0..GRID_SIZE).for_each(|i| {
            let node_status_map = Rc::clone(node_status_map);
            node_status_map.borrow_mut()[i] = NodeStatus::On;
            if let Some(node_ref) = nodes
                .borrow()
                .get(i)
                .unwrap()
                .node_ref
                .cast::<HtmlElement>()
            {
                utils::set_node_on(node_ref, &mut node_status_map.borrow_mut()[i]);
            }
        });

        *current_start_node_id.borrow_mut() = None;
        *current_end_node_id.borrow_mut() = None;
        utils::set_start_node(&nodes, &node_status_map2, 0, current_start_node_id);
        utils::set_end_node(
            &nodes,
            &node_status_map2,
            GRID_SIZE - 1,
            current_end_node_id,
        );
    }


    let handle_clear_board = {
        let node_status_map = Rc::clone(&node_satus_map);
        let nodes = Rc::clone(&nodes);
        let current_start_node_id = Rc::clone(&current_start_node_id);
        let current_end_node_id = Rc::clone(&current_end_node_id);
        let is_running = Rc::clone(&is_running);

        Callback::from(move |_| {
            if *is_running.borrow() {
                return;
            }

            let current_start_node_id = Rc::clone(&current_start_node_id);
            let current_end_node_id = Rc::clone(&current_end_node_id);
            clear_board(
                &nodes,
                &node_status_map,
                current_start_node_id,
                current_end_node_id,
            );
        })
    };

    let handle_clear_traversed_nodes = {
        let node_status_map = Rc::clone(&node_satus_map);
        let nodes = Rc::clone(&nodes);
        let is_running = Rc::clone(&is_running);

        Callback::from(move |_| {
            if *is_running.borrow() {
                return;
            }

            clear_traversed_nodes(&nodes, &node_status_map);
        })
    };

    let handle_create_adjacency_list = {
        let node_status_map = Rc::clone(&node_satus_map);
        let current_start_node_id = Rc::clone(&current_start_node_id);
        let current_end_node_id = Rc::clone(&current_end_node_id);
        let nodes = Rc::clone(&nodes);
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

            clear_traversed_nodes(&nodes, &node_status_map);
            let traversed_nodes = dijkstras::get_traversed_nodes(
                node_status_map.borrow().clone(),
                current_start_node_id.borrow().unwrap(),
                current_end_node_id.borrow().unwrap(),
            );

            visualizer::visualize(
                Rc::clone(&nodes),
                Rc::clone(&node_status_map),
                traversed_nodes.visited_ordered,
                traversed_nodes.path,
                end_of_visualization_callback,
            );
        })
    };

    html! {
        <>
            <div class="flex  gap-20 justify-center items-center bg-gray-900 h-screen w-screen">
                <div class="flex flex-col gap-6">
                    <div
                       class="grid gap-0.25 grid-cols-50 grid-rows-50 bg-gray-900 h-800 w-800 border-2 border-teal-700 "
                       onclick={handle_click}
                       oncontextmenu={handle_context_menu}
                       onmouseover={handle_mouse_over}
                       ondragstart={handle_drag_start}
                       >
                       {nodes.borrow().iter().map(|GridNode {node, ..}| node.clone()).collect::<Html>()}
                    </div>

                    <div class="flex items-center flex-col gap-2 text-zinc-500">
                        <p>{"Left Click: "}<span class="pl-4">{"set "}</span><span class="text-green-500">{"start "}</span><span>{" node"}</span></p>
                        <p>{"Right Click: "}<span class="pl-4">{"set "}</span><span class="text-red-700">{"end"}</span><span>{" node"}</span></p>
                        <p>{"Ctrl + Click: "}<span class="pl-4">{"deactivate "}</span><span>{"node"}</span></p>
                        <p>{"Shift + Click: "}<span class="pl-4">{"activate "}</span><span>{"node"}</span></p>
                    </div>
                </div>
                    <div class="flex flex-col gap-6">
                       <button class="text-white border-2 border-green-600 p-2 rounded-md hover:bg-green-600 hover:text-black" onclick={handle_create_adjacency_list}>{"Find shortest Path"}</button>
                       <button class="text-white border-2 border-emerald-900 p-2 rounded-md hover:bg-emerald-900 hover:text-black" onclick={handle_clear_traversed_nodes}>{"Clear Path"}</button>
                       <button class="text-white border-2 border-red-800 p-2 rounded-md hover:bg-red-800 hover:text-black" onclick={handle_clear_board}>{"Reset board"}</button>
                    </div>
            </div>
        </>
    }
}