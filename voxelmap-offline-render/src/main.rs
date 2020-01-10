
//use mc_render::mcdata::{resource, model, util};
use mc_render::render::model::Element;
use mc_render::render::model::TransformedModel;

use std::rc::{Rc};

fn main() {
    println!("Hello, world!");

    let m = std::mem::size_of::<Element<u128>>();
    let n = std::mem::size_of::<Option<Element<u128>>>();
    let p = std::mem::size_of::<TransformedModel<u32>>();
    println!("{} {} {}", m, n, p);
}

use std::time::Instant;
struct Timer(Instant);

impl Timer {

    fn start() -> Self {
        Timer(Instant::now())
    }

    fn stop(&self) {
        let time = Instant::now() - self.0;
        println!("use {} us", time.as_micros());
    }
}

use std::ptr::NonNull;

pub struct ListNode<T> {
    value: T,
    next: Option<NonNull<ListNode<T>>>
}