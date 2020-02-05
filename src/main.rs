use std::cell::RefCell;
use std::rc::{Rc, Weak};
use ambassador::delegatable_trait;
use ambassador::Delegate;
use std::fmt::{Debug, Formatter, Error};

type RefBox<T> = RefCell<Box<T>>;

struct Point2D {
    x: f64,
    y: f64
}
type Size2D = Point2D;

#[delegatable_trait]
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

#[delegatable_trait]
trait HasLayout {
    fn position(&self) -> &Point2D;
    fn set_position(&mut self, p: Point2D);

    fn size(&self) -> &Size2D;
    fn set_size(&mut self, sz: Size2D);

    fn border(&self) -> f64;
    fn set_border(&mut self, width: f64);

    fn margin(&self) -> f64;
    fn set_margin(&mut self, width: f64);

    fn padding(&self) -> f64;
    fn set_padding(&mut self, width: f64);
}

struct LayoutData {
    pos: Point2D,
    size: Size2D,
    border: f64,
    margin: f64,
    padding: f64
}

impl LayoutData {
    fn new() -> LayoutData {
        LayoutData {
            pos: Point2D { x: 0.0, y: 0.0 },
            size: Point2D { x: 0.0, y: 0.0 },
            border: 0.0,
            margin: 0.0,
            padding: 0.0
        }
    }
}

impl Debug for LayoutData {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let marg = format!("{}", self.margin);
        let pad = format!("{}", self.padding);
        let border = format!("{}", self.border);
        let sz = format!("{}x{}", self.size.x, self.size.y);
        let pos = format!("{}x{}", self.pos.x, self.pos.y);
        let first_line = format!("{} | {} [    {}    ] {} | {}\n",
            marg, pad, sz, pad, marg);

        f.write_str(first_line.as_ref())?;

        let second_line = format!("  {} [   @{}    ] {}  \n",
            border, pos, border);

        f.write_str(second_line.as_ref())?;

        Ok(())
    }
}

impl HasLayout for LayoutData {
    fn position(&self) -> &Point2D {
        &self.pos
    }

    fn set_position(&mut self, p: Point2D) {
        self.pos = p;
    }

    fn size(&self) -> &Point2D {
        &self.size
    }

    fn set_size(&mut self, sz: Point2D) {
        self.size = sz;
    }

    fn border(&self) -> f64 {
        self.border
    }

    fn set_border(&mut self, width: f64) {
        self.border = width
    }

    fn margin(&self) -> f64 {
        self.margin
    }

    fn set_margin(&mut self, width: f64) {
        self.margin = width
    }

    fn padding(&self) -> f64 {
        self.padding
    }

    fn set_padding(&mut self, width: f64) {
        self.padding = width
    }
}

trait Renderable {
    fn render(&self);
}

#[delegatable_trait]
trait HasText {
    fn text(&self) -> &str;
    fn set_text(&mut self, text: String);
}

struct TextData {
    text: String,
}

impl TextData {
    fn new() -> Self {
        TextData { text: "".into() }
    }

    fn text(&self) -> &str {
        &self.text
    }

    fn set_text(&mut self, text: String) {
        self.text = text;
    }
}

impl Renderable for TextData {
    fn render(&self) {
        unimplemented!()
    }
}

trait Button: Control + HasText + Renderable {}

#[derive(Delegate)]
#[delegate(Control, target="ctrl")]
#[delegate(HasText, target="text")]
#[delegate(HasLayout, target="layout")]
struct FtkButton {
    ctrl: ControlData,
    text: TextData,
    layout: LayoutData,
}

impl FtkButton {
    fn new() -> Rc<RefBox<Self>> {
        Rc::new(RefCell::new(Box::new(FtkButton {
            ctrl: ControlData::new(),
            text: TextData::new(),
            layout: LayoutData::new()
        })))
    }
}

impl Renderable for FtkButton {
    fn render(&self) {
        println!("I'm a button and I say: \"{}\"",
            self.text());
    }
}

impl Button for FtkButton {}

fn main() {
    println!("Hello, world!");

    let btn = FtkButton::new();

    {
        let mut tmp = btn.borrow_mut();
        tmp.set_text("Click me!".into());

        tmp.set_position(Point2D { x: 30.0, y: 30.0 });
        tmp.set_size(Size2D { x: 100.0, y: 50.0 });
        tmp.set_padding(8.0);
        tmp.set_border(1.0);
    }

    //println!("{}", btn.borrow().text());
    btn.borrow().render();

    println!("btn layout:\n{:?}", btn.borrow().layout);

    //let btn2 = FtkButton::new();
    // TODO: this doesn't work
    //btn.borrow_mut().set_parent(Some(Rc::downgrade(&btn2)));
}
