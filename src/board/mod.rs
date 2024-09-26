use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use web_sys::{Element, EventTarget, HtmlElement};
use gloo_console::log;
use yew::{Component, Html, NodeRef, Context};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

const GRID_SIZE: usize = 2500;
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

struct AdjacencyEntry {
    neighbors: HashMap<usize, Vec<usize>>,
}

#[function_component]
pub fn Board() -> Html {
    let squares = use_state(|| create_squares());
    let square_status_map = use_mut_ref(|| vec![0; GRID_SIZE]);
    let start_square_id = use_state(|| None::<usize>);
    let end_square_id = use_state(|| None::<usize>);
    let is_first_render = use_mut_ref(|| true);

    {
        let squares = Rc::clone(&squares);
        let start_square_id = start_square_id.clone();
        let end_square_id = end_square_id.clone();

       use_effect(move || {
           let is_not_first_render = !*is_first_render.borrow();
           if is_not_first_render {
               return;
           }
       
           *is_first_render.borrow_mut() = false;
       
           set_start_square(&squares, 0, &start_square_id);
           set_end_square(&squares, GRID_SIZE - 1, &end_square_id);
       });
   
    }

    // fn get_neighbors(square_id: usize, squares: &Rc<RefCell<Vec<SquareNode>>>) -> HashMap<usize, Vec<usize>> {
    // }
        
    // fn create_adjacency_list(squares: &Rc<RefCell<Vec<SquareNode>>>) -> Vec<AdjacencyEntry> {
    //     let squares_borrow = squares.borrow();
    //     squares_borrow.iter().enumerate().map(|(i, square)| {
    //         let mut neighbors = get_neighbors();;
    //     });
    // }



    let handle_click = {
        let start_square_id = start_square_id.clone();
        let squares = squares.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            let mod_key = event.ctrl_key();
            let target: Option<HtmlElement> = event.target().and_then(|t| t.dyn_into::<HtmlElement>().ok());

            if target.as_ref().unwrap().id() == "" {
                return;
            }


            match mod_key {
                false => {
                    set_start_square(&squares, target.as_ref().unwrap().id().parse::<usize>().unwrap(), &start_square_id);
                }
                true => {
                    target.as_ref().map(|t| t.set_class_name(OFF_COLOR));
                }
            }
        })
    };

    let handle_mouse_over = {
        let start_square_id = start_square_id.clone();
        let end_square_id = end_square_id.clone();
        let squares = squares.clone();
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
                toggle_square(target.id().parse::<usize>().unwrap(), &squares, Rc::clone(&square_status_map));
                return;
            }

            let target_id = target.id().parse::<usize>().unwrap();

            match mouse_button as i16 {
                LEFT_BUTTON => set_start_square(&squares, target_id, &start_square_id),
                RIGHT_BUTTON => set_end_square(&squares, target_id, &end_square_id),
                _ => return,
            }
        })
    };
    
    let handle_context_menu = {
        let end_square_id = end_square_id.clone();
        let squares = squares.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            if event.button() != RIGHT_BUTTON {
                return;
            }

            let target: Option<HtmlElement> = event.target()
                .and_then(|t| t.dyn_into::<HtmlElement>().ok());

            let Some(target) = target else { return };

            set_end_square(&squares, target.id().parse::<usize>().unwrap(), &end_square_id);
        })
    };

    fn set_square_on(id: usize, squares: &Rc<RefCell<Vec<SquareNode>>>, square_status_map: Rc<RefCell<Vec<i32>>>) {
        let squares_borrow = squares.borrow();
        let square = squares_borrow.get(id).unwrap();
        let node = square.node_ref.cast::<HtmlElement>().unwrap();
        node.set_class_name(DEFAULT_COLOR);
        square_status_map.borrow_mut()[id] = 0;
    }

    fn set_square_off(id: usize, squares: &Rc<RefCell<Vec<SquareNode>>>, square_status_map: Rc<RefCell<Vec<i32>>>) {
        let squares_borrow = squares.borrow();
        let square = squares_borrow.get(id).unwrap();
        let node = square.node_ref.cast::<HtmlElement>().unwrap();
        node.set_class_name(OFF_COLOR);
        square_status_map.borrow_mut()[id] = 1;
    }

    fn toggle_square(id: usize, squares: &Rc<RefCell<Vec<SquareNode>>>, square_status_map: Rc<RefCell<Vec<i32>>>) {
        let squares_borrow = squares.borrow();
        let square = squares_borrow.get(id).unwrap();
        let node = square.node_ref.cast::<HtmlElement>().unwrap();
        let current_class = node.class_name();

        log!(square_status_map.borrow()[id]);

        match current_class.as_str() {
            OFF_COLOR => set_square_on(id, squares, square_status_map),
            _ => set_square_off(id, squares, square_status_map),
        };
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

   fn set_start_square(squares: &Rc<RefCell<Vec<SquareNode>>>, new_start_id: usize, id_state: &UseStateHandle<Option<usize>>) {
       if let Some(id) = id_state.as_ref() {
                let squares_borrow = squares.borrow();
                let previous_start_square = squares_borrow.get(*id).unwrap();
                let previous_start_node = previous_start_square.node_ref.cast::<HtmlElement>().unwrap();
                previous_start_node.set_class_name(DEFAULT_COLOR);
       }

       let new_start_node = squares.borrow().get(new_start_id).unwrap().node_ref.cast::<HtmlElement>().unwrap();
       new_start_node.set_class_name(START_COLOR);
       id_state.set(Some(new_start_id));
   }


   fn set_end_square(squares: &Rc<RefCell<Vec<SquareNode>>>, new_end_id: usize, id_state: &UseStateHandle<Option<usize>>) {
    if let Some(id) = id_state.as_ref() {
            let squares_borrow = squares.borrow();
            let previous_end_square = squares_borrow.get(*id).unwrap();
            let previous_end_node = previous_end_square.node_ref.cast::<HtmlElement>().unwrap();
            previous_end_node.set_class_name(DEFAULT_COLOR);
    }

    let new_end_node = squares.borrow().get(new_end_id).unwrap().node_ref.cast::<HtmlElement>().unwrap();
    new_end_node.set_class_name(END_COLOR);
    id_state.set(Some(new_end_id));
   }

   let handle_drag_start = {
       Callback::from(move |event: DragEvent| {
           event.prevent_default();
       })
   };


   
    html! {
        <>
            <div class="flex justify-center items-center bg-gray-900 h-screen w-screen">
                 <div 
                    class="grid gap-0.25 grid-cols-50 grid-rows-50 bg-gray-900 h-800 w-800"
                    onclick={handle_click}
                    oncontextmenu={handle_context_menu}
                    onmouseover={handle_mouse_over}
                    ondragstart={handle_drag_start}
                    >
                    {(*squares).borrow().iter().map(|square_node| square_node.node.clone()).collect::<Html>()}
                 </div>
            </div>
        </>
    }
}