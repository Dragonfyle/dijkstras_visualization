use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use web_sys::{EventTarget, HtmlElement};
use gloo::utils::document;
use gloo_console::log;

const GRID_SIZE: usize = 2500;
const RIGHT_BUTTON: i16 = 2;

#[function_component]
pub fn Board() -> Html {
    let squares = use_state(|| create_squares());
    let start_square = use_state(|| None::<HtmlElement>);
    let end_square = use_state(|| None::<HtmlElement>);

    let handle_click = {
        Callback::from(move |event: MouseEvent| {
            let current_target: Option<EventTarget> = event.target();
            let current_target = current_target
               .and_then(|t| t.dyn_into::<HtmlElement>().ok());

            if let Some(el) = start_square.as_ref() {
                el.set_class_name("bg-gray-700");
            }
            current_target.as_ref().map(|t| t.set_class_name("bg-green-700"));
            start_square.set(current_target);
        })
    };

    let handle_context_menu = {
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            if event.button() != RIGHT_BUTTON {
                return;
            }

            let current_target: Option<EventTarget> = event.target();
            let current_target = current_target
               .and_then(|t| t.dyn_into::<HtmlElement>().ok());

            if let Some(el) = end_square.as_ref() {
                el.set_class_name("bg-gray-700");
            }
            current_target.as_ref().map(|t| t.set_class_name("bg-red-700"));
            end_square.set(current_target);
        })
    };

   fn create_squares() -> Vec<Html> {
    let mut grid_as_vec = Vec::new();
    for i in 0..GRID_SIZE {
        grid_as_vec.push(
            html! {
                 <div class="bg-gray-700" id={i.to_string()}></div>
            });
    }

    grid_as_vec
   }

   fn set_start_square() {
    let first_square = document().get_element_by_id("0").unwrap();
    first_square.set_class_name("bg-green-700");
   }

   let set_end_square = {
       let end_square = end_square.clone();
       move || {
           let last_square = document().get_element_by_id(&(GRID_SIZE - 1).to_string()).unwrap();
           last_square.set_class_name("bg-red-700");
           end_square.set(Some(last_square.dyn_into().unwrap()));
       }
   };

   use_effect(move || {
    set_start_square();
    set_end_square();
   },);

   
    html! {
        <>
            <div class="flex justify-center items-center bg-gray-900 h-screen w-screen">
                 <div 
                    class="grid gap-0.25 grid-cols-50 grid-rows-50 bg-gray-900 h-800 w-800"
                    onclick={handle_click}
                    oncontextmenu={handle_context_menu}>
                    {(*squares).clone()}
                 </div>
            </div>
        </>
    }
}