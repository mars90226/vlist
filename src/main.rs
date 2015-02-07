use std::mem;
use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;

struct VList<T: fmt::Display> {
  base: Option<Rc<RefCell<VSeg<T>>>>,
  offset: usize,
}

struct VSeg<T: fmt::Display> {
  next: Option<Rc<RefCell<VSeg<T>>>>,
  ele: Vec<Rc<RefCell<T>>>,
}

impl<T: fmt::Display> VSeg<T> {
  fn index(&self, i: usize) -> Option<Rc<RefCell<T>>> {
    let capacity = self.ele.capacity();
    if i < capacity {
      let index = capacity - 1 - i;
      Some(self.ele[index].clone())
    } else {
      match self.next {
        Some(ref seg) => {
          seg.borrow().index(i - capacity)
        },
        None => None,
      }
    }
  }

  fn each<F: FnMut(Rc<RefCell<T>>)>(&self, mut f: F) {
    for e in self.ele.iter().rev() {
      f(e.clone());
    }

    match self.next {
      Some(ref next) => {
        next.borrow().each(f);
      },
      None => (),
    }
  }
}

impl<T: fmt::Display> VList<T> {
  fn index(&self, i: usize) -> Option<Rc<RefCell<T>>> {
    match self.base {
      Some(ref seg) => {
        let index = i + seg.borrow().ele.capacity() - self.offset - 1;
        seg.borrow().index(index)
      },
      None => None,
    }
  }

  fn cons(&mut self, e: T) -> VList<T> {
    match self.base {
      Some(ref mut seg) if self.offset + 1 != seg.borrow().ele.capacity() => {
        seg.borrow_mut().ele.push(Rc::new(RefCell::new(e)));
        VList { base: Some(seg.clone()), offset: self.offset + 1 }
      },
      Some(..) => {
        let seg = self.base.as_ref().unwrap();
        let capacity = seg.borrow().ele.capacity();
        let mut ele = Vec::with_capacity(capacity * 2);
        ele.push(Rc::new(RefCell::new(e)));

        VList {
          base: Some(Rc::new(RefCell::new(VSeg { next: Some(seg.clone()), ele: ele }))),
          offset: 0
        }
      },
      None => VList {
        base: Some(Rc::new(RefCell::new(VSeg {
          next: None,
          ele: vec![Rc::new(RefCell::new(e))]
        }))),
        offset: 0
      },
    }
  }

  fn cdr(&mut self) -> Option<VList<T>> {
    match self.base {
      Some(ref mut seg) if seg.borrow().ele.len() != 0 => {
        Some(VList { base: Some(seg.clone()), offset: self.offset - 1 })
      },
      Some(ref mut seg) => {
        match seg.clone().borrow().next {
          Some(ref next_seg) => Some(VList { base: Some(next_seg.clone()), offset: 0 }),
          None =>  Some(VList { base: None, offset: 0 }),
        }
      },
      None => None,
    }
  }

  fn len(&self) -> usize {
    match self.base {
      Some(ref seg) => seg.borrow().ele.capacity() + self.offset - 1,
      None => 0,
    }
  }

  fn to_string(&self) -> String {
    match self.base {
      Some(ref seg) => {
        let mut result = "[".to_string();
        let sg = seg;

        for e in seg.borrow().ele[..self.offset + 1].iter().rev() {
          result.push_str(&format!(" {}", *e.borrow())[]);
        }

        match seg.borrow().next {
          Some(ref next_seg) => {
            next_seg.borrow().each(|&mut: e| {
              result.push_str(&format!(" {}", *e.borrow())[]);
            });
          },
          None => (),
        }

        result.push_str(" ]");
        result
      },
      None => "[]".to_string(),
    }
  }

  fn print_structure(&self) {
    match self.base {
      Some(ref seg) => {
        for e in seg.borrow().ele[..self.offset + 1].iter().rev() {
          print!(" {}", *e.borrow());
        }

        match seg.borrow().next {
          Some(ref next_seg) => {
            next_seg.borrow().each(|&mut: e| {
              print!(" {}", *e.borrow());
            });
          },
          None => (),
        }
      },
      None => (),
    }

    println!("")
  }
}

impl<T: fmt::Display> fmt::Display for VList<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.to_string())
  }
}

fn main() {
  let mut v: VList<i32> = VList { base: None, offset: 0 };
  println!("zero value for type. empty VList: {}", v);
  v.print_structure();

  for a in range(1, 7).rev() {
    v = v.cons(a);
  }
  println!("demonstrate cons. 6 elements added: {}", v);
  v.print_structure();

  v = v.cdr().unwrap();
  println!("demonstrate cdr. 1 elements removed: {}", v);
  v.print_structure();

  println!("demonstrate length. length = {}", v.len());
  println!("");

  println!("demonstrate element access. v[3] = {}", *v.index(3).unwrap().borrow());
  println!("");

  v = v.cdr().unwrap().cdr().unwrap();
  println!("show cdr releasing segment. 2 elements removed: {}", v);
  v.print_structure();
}
