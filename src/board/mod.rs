use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use web_sys::HtmlElement;
use gloo_console::log;
use yew::{ Html, NodeRef};
use std::rc::Rc;
use std::cell::RefCell;
use crate::adjacency_list;
use crate::{GRID_SIZE, SquareStatus};
use crate::dijkstras;





const RIGHT_BUTTON: i16 = 2;
const LEFT_BUTTON: i16 = 1;
const DEFAULT_COLOR: &str = "bg-gray-700";
const START_COLOR: &str = "bg-green-700";
const END_COLOR: &str = "bg-red-700";
const OFF_COLOR: &str = "bg-gray-900";

struct SquareNode {
    node: VNode,
    node_ref: NodeRef,
}

#[function_component]
pub fn Board() -> Html {
    let squares = use_state(|| create_squares());
    let square_status_map = use_mut_ref(|| vec![SquareStatus::Off; GRID_SIZE]);
    let current_start_square_id = use_mut_ref(|| None::<usize>);
    let current_end_square_id = use_mut_ref(|| None::<usize>);
    let is_first_render = use_mut_ref(|| true);

    {
        let squares = Rc::clone(&squares);
        let current_start_square_id = Rc::clone(&current_start_square_id);
        let current_end_square_id = Rc::clone(&current_end_square_id);

       use_effect(move || {
           let is_not_first_render = !*is_first_render.borrow();
           if is_not_first_render {
               return;
           }
       
           *is_first_render.borrow_mut() = false;
       
           set_start_square(&squares, 0, current_start_square_id);
           set_end_square(&squares, GRID_SIZE - 1, current_end_square_id);
       });
   
    }

   let handle_drag_start = {
       Callback::from(move |event: DragEvent| {
           event.prevent_default();
       })
   };

    let handle_click = {
        let squares = Rc::clone(&squares);
        let square_status_map = Rc::clone(&square_status_map);
        let current_start_square_id = Rc::clone(&current_start_square_id);

        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            let target: Option<HtmlElement> = event.target().and_then(|t| t.dyn_into::<HtmlElement>().ok());

            if target.as_ref().unwrap().id().is_empty() {
                return;
            }

            if event.ctrl_key() {
                set_square_off(target.as_ref().unwrap().id().parse::<usize>().unwrap(), &squares, Rc::clone(&square_status_map));
                return;
            } else if event.shift_key() {
                set_square_on(target.as_ref().unwrap().id().parse::<usize>().unwrap(), &squares, Rc::clone(&square_status_map));
                return;
            }

            let current_start_square_id = Rc::clone(&current_start_square_id);
            set_start_square(&squares, target.as_ref().unwrap().id().parse::<usize>().unwrap(), current_start_square_id);
        })
    };

    let handle_mouse_over = {
        let squares = Rc::clone(&squares);
        let square_status_map = Rc::clone(&square_status_map);
        let current_start_square_id = Rc::clone(&current_start_square_id);
        let current_end_square_id = Rc::clone(&current_end_square_id);

        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            let mouse_button = event.buttons();
            if mouse_button == 0 {
                return;
            }

            let current_target = event.target()
                .and_then(|t| t.dyn_into::<HtmlElement>().ok())
                .filter(|t| !t.id().is_empty());

            let Some(target) = current_target else { return };

            if event.ctrl_key() {
                set_square_off(target.id().parse::<usize>().unwrap(), &squares, Rc::clone(&square_status_map));
                return;
            } else if event.shift_key() {
                set_square_on(target.id().parse::<usize>().unwrap(), &squares, Rc::clone(&square_status_map));
                return;
            }

            let target_id = target.id().parse::<usize>().unwrap();

            let current_end_square_id = Rc::clone(&current_end_square_id);
            let current_start_square_id = Rc::clone(&current_start_square_id);
            match mouse_button as i16 {
                LEFT_BUTTON => set_start_square(&squares, target_id, current_start_square_id),
                RIGHT_BUTTON => set_end_square(&squares, target_id, current_end_square_id),
                _ => return,
            }
        })
    };
    
    let handle_context_menu = {
        let squares = Rc::clone(&squares);
        let current_end_square_id = Rc::clone(&current_end_square_id);

        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            if event.button() != RIGHT_BUTTON {
                return;
            }

            let target: Option<HtmlElement> = event.target()
                .and_then(|t| t.dyn_into::<HtmlElement>().ok());

            let Some(target) = target else { return };

            let current_end_square_id = Rc::clone(&current_end_square_id);
            set_end_square(&squares, target.id().parse::<usize>().unwrap(), current_end_square_id);
        })
    };

    fn set_square_on(id: usize, squares: &Rc<RefCell<Vec<SquareNode>>>, square_status_map: Rc<RefCell<Vec<SquareStatus>>>) -> Rc<RefCell<Vec<SquareStatus>>> {
        let squares_borrow = squares.borrow();
        let square = squares_borrow.get(id).unwrap();
        let node = square.node_ref.cast::<HtmlElement>().unwrap();
        node.set_class_name(DEFAULT_COLOR);
        square_status_map.borrow_mut()[id] = SquareStatus::On;

        square_status_map
    }

    fn set_square_off(id: usize, squares: &Rc<RefCell<Vec<SquareNode>>>, square_status_map: Rc<RefCell<Vec<SquareStatus>>>) -> Rc<RefCell<Vec<SquareStatus>>> {
        let squares_borrow = squares.borrow();
        let square = squares_borrow.get(id).unwrap();
        let node = square.node_ref.cast::<HtmlElement>().unwrap();
        node.set_class_name(OFF_COLOR);
        square_status_map.borrow_mut()[id] = SquareStatus::Off;

        square_status_map
    }

   fn create_squares() -> Rc<RefCell<Vec<SquareNode>>> {
    let grid_as_vec = Rc::new(RefCell::new(Vec::new()));

    for i in 0..GRID_SIZE {
        let node_ref = NodeRef::default();
        let square = html! {
            <div ref={node_ref.clone()} class={DEFAULT_COLOR} id={i.to_string()}></div>
        };

        grid_as_vec.borrow_mut().push(SquareNode {
            node: square,
            node_ref,
        });
    }

    grid_as_vec
   }

   fn clear_board(
       squares: &Rc<RefCell<Vec<SquareNode>>>,
       square_status_map: &Rc<RefCell<Vec<SquareStatus>>>,
       current_start_square_id: Rc<RefCell<Option<usize>>>,
       current_end_square_id: Rc<RefCell<Option<usize>>>
   ) {
       for i in 0..GRID_SIZE {
           let square_status_map = Rc::clone(square_status_map);
           let square_status_map = set_square_on(i, squares, square_status_map);
           square_status_map.borrow_mut()[i] = SquareStatus::On;
       }

       *current_start_square_id.borrow_mut() = None;
       *current_end_square_id.borrow_mut() = None;

       set_start_square(squares, 0, current_start_square_id);
       set_end_square(squares, GRID_SIZE - 1, current_end_square_id);
   }

   fn set_start_square(squares: &Rc<RefCell<Vec<SquareNode>>>, new_start_id: usize, current_start_square_id: Rc<RefCell<Option<usize>>>) {
    if let Some(id) = current_start_square_id.borrow().as_ref() {
             let squares_borrow = squares.borrow();
             let previous_start_square = squares_borrow.get(*id).unwrap();
             let previous_start_node = previous_start_square.node_ref.cast::<HtmlElement>().unwrap();
             previous_start_node.set_class_name(DEFAULT_COLOR);
    }

    let new_start_node = squares.borrow().get(new_start_id).unwrap().node_ref.cast::<HtmlElement>().unwrap();
    new_start_node.set_class_name(START_COLOR);
    current_start_square_id.borrow_mut().replace(new_start_id);
   }


   fn set_end_square(squares: &Rc<RefCell<Vec<SquareNode>>>, new_end_id: usize, current_end_square_id: Rc<RefCell<Option<usize>>>) {
    if let Some(id) = current_end_square_id.borrow().as_ref() {
            let squares_borrow = squares.borrow();
            let previous_end_square = squares_borrow.get(*id).unwrap();
            let previous_end_node = previous_end_square.node_ref.cast::<HtmlElement>().unwrap();
            previous_end_node.set_class_name(DEFAULT_COLOR);
    }

    let new_end_node = squares.borrow().get(new_end_id).unwrap().node_ref.cast::<HtmlElement>().unwrap();
    new_end_node.set_class_name(END_COLOR);
    current_end_square_id.borrow_mut().replace(new_end_id);
   }

   let handle_clear_board = {
    let square_status_map = Rc::clone(&square_status_map);
    let squares = Rc::clone(&squares);
    let current_start_square_id = Rc::clone(&current_start_square_id);
    let current_end_square_id = Rc::clone(&current_end_square_id);
    
    Callback::from(move |_| {
        let current_start_square_id = Rc::clone(&current_start_square_id);
        let current_end_square_id = Rc::clone(&current_end_square_id);
        clear_board(&squares, &square_status_map, current_start_square_id, current_end_square_id);
    })
   };


    let handle_create_adjacency_list = {
        let square_status_map = Rc::clone(&square_status_map);
        let current_start_square_id = Rc::clone(&current_start_square_id);
        let current_end_square_id = Rc::clone(&current_end_square_id);

        Callback::from(move |_: MouseEvent| {
            let adjacency_list = adjacency_list::create_adjacency_list(&square_status_map.borrow());
            let traversed_nodes = dijkstras::get_traversed_nodes(&adjacency_list, current_start_square_id.borrow().unwrap(), current_end_square_id.borrow().unwrap(), GRID_SIZE);
            log!(format!("{:?}", traversed_nodes.visited_ordered));
        })
    };

    html! {
        <>
            <div class="flex gap-20 justify-center items-center bg-gray-900 h-screen w-screen">
                 <div 
                    class="grid gap-0.25 grid-cols-50 grid-rows-50 bg-gray-900 h-800 w-800"
                    onclick={handle_click}
                    oncontextmenu={handle_context_menu}
                    onmouseover={handle_mouse_over}
                    ondragstart={handle_drag_start}
                    >
                    {squares.borrow().iter().map(|SquareNode {node, ..}| node.clone()).collect::<Html>()}
                 </div>

                 <button class="text-white" onclick={handle_create_adjacency_list}>{"Create Adjacency List"}</button>
                 <button class="text-white" onclick={handle_clear_board}>{"Clear Board"}</button>
            </div>
        </>
    }
}