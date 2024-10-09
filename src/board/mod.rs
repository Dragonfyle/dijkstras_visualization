mod grid_actions;
mod board_utils;

use std::cell::RefCell;
use std::rc::Rc;

#[allow(unused_imports)]
use gloo::console::log;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew::{Html, NodeRef};

use crate::dijkstras::DijkstrasTraversal;
use crate::utils;
use crate::visualizer;
use crate::{NodeStatus, DEFAULT_COLOR, GRID_SIZE};
use crate::utils::MouseAction;

use grid_actions::{GridAction, GridState, TouchSquare};
pub use board_utils::{GridNode, Nodes, CurrentStartNode, CurrentEndNode, BoardStatus, CLICK, CONTEXT_MENU, MOUSE_OVER};

impl GridNode {
    pub fn build(id: usize) -> Self {
        let node_ref = NodeRef::default();
        GridNode {
            node: html! {
                <div class={DEFAULT_COLOR} id={id.to_string()} ref={node_ref.clone()}></div>
            },
            node_ref,
            node_status: NodeStatus::On,
        }
    }
}

#[function_component]
pub fn Board() -> Html {
    let nodes = use_state(|| create_nodes());
    let current_start_node_id = use_mut_ref(|| Option::<usize>::None);
    let current_end_node_id = use_mut_ref(|| Option::<usize>::None);
    let is_first_render = use_mut_ref(|| true);
    let board_status = use_mut_ref(|| BoardStatus::NotVisualized);

    {
        let nodes = Rc::clone(&nodes);
        let current_start_node_id = Rc::clone(&current_start_node_id);
        let current_end_node_id = Rc::clone(&current_end_node_id);

        use_effect(move || {
            let is_not_first_render = !*is_first_render.borrow();
            if is_not_first_render {
                return;
            }

            *is_first_render.borrow_mut() = false;

            utils::set_start_node(nodes.borrow_mut(), 0, current_start_node_id);
            utils::set_end_node(
                nodes.borrow_mut(),
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

    let handle_mouse_action = {
        let nodes = Rc::clone(&nodes);
        let current_start_node_id = Rc::clone(&current_start_node_id);
        let current_end_node_id = Rc::clone(&current_end_node_id);
        let board_status = Rc::clone(&board_status);

        Callback::from(move |event: MouseEvent| {
            event.prevent_default();

            if let BoardStatus::Visualizing | BoardStatus::Visualized = *board_status.borrow() {
                return;
            }

            let mouse_action = match event.type_().as_str() {
                CLICK | CONTEXT_MENU => MouseAction::Click,
                MOUSE_OVER => MouseAction::Move,
                _ => return,
            };

            
            let grid_state = GridState::new(
                Rc::clone(&nodes),
                Rc::clone(&current_start_node_id),
                Rc::clone(&current_end_node_id),
            );
            let grid_action = GridAction::new(&event, mouse_action, grid_state);

            if let Some(grid_action) = grid_action {
                grid_action.trigger_node();

                let mut board_status_borrow = board_status.borrow_mut();
                if let BoardStatus::Empty = *board_status_borrow {
                    *board_status_borrow = BoardStatus::NotVisualized;
                }
            }

        })
    };

    fn create_nodes() -> Nodes {
        let grid_as_vec = Rc::new(RefCell::new(Vec::with_capacity(GRID_SIZE)));

        (0..GRID_SIZE).for_each(|i| {
            let grid_node = GridNode::build(i);
            grid_as_vec.borrow_mut().push(grid_node);
        });

        grid_as_vec
    }

    fn clear_traversed_nodes(nodes: &Nodes) {
        let mut nodes_borrow = nodes.borrow_mut();
        (0..GRID_SIZE).for_each(|i| {
            if let Some(node_ref) = nodes_borrow
                .get(i)
                .unwrap()
                .node_ref
                .cast::<HtmlElement>()
            {
                let node_status = &mut nodes_borrow[i].node_status;
                if *node_status == NodeStatus::Visited
                    || *node_status == NodeStatus::Path
                {
                    utils::set_square_color(&node_ref, NodeStatus::On);
                    utils::set_node_status(node_status, NodeStatus::On);
                }
            }
        })

    }

    fn reset_board(
        nodes: &Nodes,
        current_start_node_id: CurrentStartNode,
        current_end_node_id: CurrentEndNode,
    ) {
        let mut nodes_borrow = nodes.borrow_mut();
        (0..GRID_SIZE).for_each(|i| {
            nodes_borrow[i].node_status = NodeStatus::On;
            if let Some(node_ref) = nodes_borrow
                .get(i)
                .unwrap()
                .node_ref
                .cast::<HtmlElement>()
            {
                utils::set_node_on(node_ref, &mut nodes_borrow[i].node_status);
            }
        });

        *current_start_node_id.borrow_mut() = None;
        *current_end_node_id.borrow_mut() = None;
        utils::set_start_node(nodes_borrow, 0, current_start_node_id);
        utils::set_end_node(
            nodes.borrow_mut(),
            GRID_SIZE - 1,
            current_end_node_id,
        );
    }


    let handle_reset_board = {
        let nodes = Rc::clone(&nodes);
        let current_start_node_id = Rc::clone(&current_start_node_id);
        let current_end_node_id = Rc::clone(&current_end_node_id);
        let board_status = Rc::clone(&board_status);

        Callback::from(move |_| {
            let mut board_status_borrow = board_status.borrow_mut();
            if let BoardStatus::Visualizing | BoardStatus::Empty = *board_status_borrow {
                return;
            }

            let current_start_node_id = Rc::clone(&current_start_node_id);
            let current_end_node_id = Rc::clone(&current_end_node_id);
            reset_board(
                &nodes,
                current_start_node_id,
                current_end_node_id,
            );

            *board_status_borrow = BoardStatus::Empty;
        })

    };

    let handle_clear_traversed_nodes = {
        let nodes = Rc::clone(&nodes);
        let board_status = Rc::clone(&board_status);

        Callback::from(move |_| {
            let mut board_status_borrow = board_status.borrow_mut();
            if let BoardStatus::Visualizing | BoardStatus::NotVisualized | BoardStatus::Empty = *board_status_borrow {
                return;
            }

            clear_traversed_nodes(&nodes);

            *board_status_borrow = BoardStatus::NotVisualized;
        })
    };

    let handle_create_adjacency_list = {
        let current_start_node_id = Rc::clone(&current_start_node_id);
        let current_end_node_id = Rc::clone(&current_end_node_id);
        let nodes = Rc::clone(&nodes);
        let board_status = Rc::clone(&board_status);

        Callback::from(move |_| {
            if let BoardStatus::Visualizing | BoardStatus::Visualized = *board_status.borrow() {
                return;
            }

            *board_status.borrow_mut() = BoardStatus::Visualizing;

            let dijkstras_traversal = DijkstrasTraversal::run(
                &nodes.borrow(),
                current_start_node_id.borrow().unwrap(),
                current_end_node_id.borrow().unwrap(),
            );

            let board_status = Rc::clone(&board_status);
            let end_of_visualization_callback = move || {
                *board_status.borrow_mut() = BoardStatus::Visualized;
            };
            visualizer::visualize(
                Rc::clone(&nodes),
                dijkstras_traversal.get_traversed_nodes(),
                dijkstras_traversal.get_path(),
                end_of_visualization_callback,
            );
        })
    };

    html! {
        <>
            <div class="flex gap-20 justify-center items-center bg-gray-900 h-screen w-screen">
                <div
                   class="grid gap-0.25 grid-cols-50 grid-rows-50 bg-gray-900 h-400 w-400 lg:h-600 lg:w-600 2xl:h-800 2xl:w-800 border-2 border-teal-700 "
                   onclick={&handle_mouse_action}
                   oncontextmenu={&handle_mouse_action}
                   onmouseover={&handle_mouse_action}
                   ondragstart={&handle_drag_start}
                   >
                   {nodes.borrow().iter().map(|GridNode {node, ..}| node.clone()).collect::<Html>()}
                </div>

                <div class="flex flex-col gap-20">
                    <div class="flex flex-col gap-6">   
                        <button class="text-white border-2 border-green-600 p-2 rounded-md hover:bg-green-600 hover:text-black" onclick={handle_create_adjacency_list}>{"Find shortest Path"}</button>
                        <button class="text-white border-2 border-emerald-900 p-2 rounded-md hover:bg-emerald-900 hover:text-black" onclick={handle_clear_traversed_nodes}>{"Clear Path"}</button>
                        <button class="text-white border-2 border-red-800 p-2 rounded-md hover:bg-red-800 hover:text-black" onclick={handle_reset_board}>{"Reset board"}</button>
                    </div>

                    <div class="flex items-center flex-col gap-2 text-zinc-500">
                        <p>{"Left Click: "}<span class="pl-4">{"set "}</span><span class="text-green-500">{"start "}</span><span>{" node"}</span></p>
                        <p>{"Right Click: "}<span class="pl-4">{"set "}</span><span class="text-red-700">{"end"}</span><span>{" node"}</span></p>
                        <p>{"Ctrl + Click: "}<span class="pl-4">{"deactivate "}</span><span>{"node"}</span></p>
                        <p>{"Shift + Click: "}<span class="pl-4">{"activate "}</span><span>{"node"}</span></p>
                    </div>
                </div>
            </div>
        </>
    }
}