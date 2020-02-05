use std::cell::RefCell;
use std::rc::{Rc, Weak};

type RefBox<T> = RefCell<Box<T>>;

trait Control {
    fn id(&self) -> &Option<String>;
    fn set_id(&mut self, id: Option<String>);

    fn parent(&self) -> &Option<Weak<RefBox<dyn Control>>>;
    fn set_parent(&mut self, parent: Option<Weak<RefBox<dyn Control>>>);

    fn children(&self) -> &Vec<Rc<RefBox<dyn Control>>>;
    fn children_mut(&mut self) -> &mut Vec<Rc<RefBox<dyn Control>>>;
}

struct ControlData {
    id_: Option<String>,
    parent_: Option<Weak<RefBox<dyn Control>>>,
    children_: Vec<Rc<RefBox<dyn Control>>>,
}

impl ControlData {
    fn new() -> Self {
        ControlData {
            id_: None,
            parent_: None,
            children_: Vec::new()
        }
    }
}

impl Control for ControlData {
    fn id(&self) -> &Option<String> {
        &self.id_
    }

    fn set_id(&mut self, id: Option<String>) {
        self.id_ = id
    }

    fn parent(&self) -> &Option<Weak<RefBox<dyn Control>>> {
        &self.parent_
    }

    fn set_parent(&mut self, parent: Option<Weak<RefBox<dyn Control>>>) {
        self.parent_ = parent
    }

    fn children(&self) -> &Vec<Rc<RefBox<dyn Control>>> {
        &self.children_
    }

    fn children_mut(&mut self) -> &mut Vec<Rc<RefBox<dyn Control>>> {
        &mut self.children_
    }
}

trait HasText {
    fn text(&self) -> &str;
    fn set_text(&mut self, text: String);
}

struct TextData {
    text_: String,
}

impl TextData {
    fn new() -> Self {
        TextData { text_: "".into() }
    }

    fn text(&self) -> &str {
        &self.text_
    }

    fn set_text(&mut self, text: String) {
        self.text_ = text;
    }
}

trait Button: Control + HasText {}

struct FtkButton {
    ctrl_: ControlData,
    text_: TextData,
}

impl FtkButton {
    fn new() -> Rc<RefBox<Self>> {
        Rc::new(RefCell::new(Box::new(FtkButton {
            ctrl_: ControlData::new(),
            text_: TextData::new()
        })))
    }
}

impl Control for FtkButton {
    fn id(&self) -> &Option<String> {
        &self.ctrl_.id()
    }

    fn set_id(&mut self, id: Option<String>) {
        self.ctrl_.set_id(id)
    }

    fn parent(&self) -> &Option<Weak<RefBox<dyn Control>>> {
        &self.ctrl_.parent()
    }

    fn set_parent(&mut self, parent: Option<Weak<RefBox<dyn Control>>>) {
        self.ctrl_.set_parent(parent)
    }

    fn children(&self) -> &Vec<Rc<RefBox<dyn Control>>> {
        self.ctrl_.children()
    }

    fn children_mut(&mut self) -> &mut Vec<Rc<RefBox<dyn Control>>> {
        self.ctrl_.children_mut()
    }
}

impl HasText for FtkButton {
    fn text(&self) -> &str { self.text_.text() }
    fn set_text(&mut self, text: String) { self.text_.set_text(text); }
}

impl Button for FtkButton {}

fn main() {
    println!("Hello, world!");

    let btn = FtkButton::new();
    btn.borrow_mut().set_text("Click me!".into());
    println!("{}", btn.borrow().text());

    let btn2 = FtkButton::new();

    // TODO: this doesn't work
    btn.borrow_mut().set_parent(Some(Rc::downgrade(&btn2)));
}
