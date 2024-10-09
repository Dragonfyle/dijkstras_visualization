use std::rc::Rc;
use std::cell::RefCell;

use yew::virtual_dom::VNode;
use yew::NodeRef;

use crate::NodeStatus;

pub type Nodes = Rc<RefCell<Vec<GridNode>>>;
pub type CurrentStartNode = Rc<RefCell<Option<usize>>>;
pub type CurrentEndNode = Rc<RefCell<Option<usize>>>;

pub struct GridNode {
    pub node: VNode,
    pub node_ref: NodeRef,
    pub node_status: NodeStatus,
}

pub enum BoardStatus {
    Empty,
    NotVisualized,
    Visualizing,
    Visualized,
}

pub const CLICK: &str = "click";
pub const CONTEXT_MENU: &str = "contextmenu";
pub const MOUSE_OVER: &str = "mouseover";
