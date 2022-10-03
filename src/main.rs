mod with_arc;
mod with_box;
mod with_generics;
mod with_rc;
use std::thread;
use with_arc::with_arc;
use with_box::with_box;
use with_generics::with_generics;
use with_rc::with_rc;

fn main() {
  with_generics();
  with_box();
  with_rc();

  let handler = thread::spawn(with_arc);
  handler.join().unwrap();
}
