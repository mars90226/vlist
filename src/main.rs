use std::mem;
use std::fmt;

struct VList<T: fmt::Display> {
  base: Option<Box<VSeg<T>>>
}

struct VSeg<T: fmt::Display> {
  next: Option<Box<VSeg<T>>>,
  ele: Vec<T>,
}

impl<T: fmt::Display> VList<T> {
  fn index(&self, mut i: usize) -> Option<&T> {
    match self.base {
      Some(ref seg) => i += seg.ele.capacity() - seg.ele.len(),
      None => return None,
    }

    let mut sg = &self.base;
    loop {
      match *sg {
        Some(ref seg) => {
          if i < seg.ele.capacity() {
            let index = seg.ele.capacity() - 1 - i;
            return Some(&seg.ele[index]);
          }
          i -= seg.ele.capacity();
          sg = &seg.next;
        },
        None => return None,
      }
    }
  }

  fn cons(&mut self, e: T) -> &VList<T> {
    match self.base {
      Some(ref mut seg) => {
        if seg.ele.len() == seg.ele.capacity() {
          let len = seg.ele.capacity() * 2;
          let mut ele = Vec::with_capacity(len);

          ele.push(e);
          let old = mem::replace(seg, Box::new(VSeg { next: None, ele: Vec::new() }));
          **seg = VSeg { next: Some(old), ele: ele };
        } else {
          seg.ele.push(e);
        }
      },
      None => {
        self.base = Some(Box::new(VSeg {
          next: None,
          ele: vec![e]
        }));
      },
    }
    self
  }

  fn cdr(&mut self) -> Option<&mut VList<T>> {
    match self.base {
      Some(ref mut seg) if seg.ele.len() != 0 => {
        seg.ele.pop();
      },
      Some(..) => {
        let old = mem::replace(&mut self.base, None);
        self.base = old.unwrap().next;
      },
      None => return None,
    }
    Some(&mut *self)
  }

  fn len(&self) -> usize {
    match self.base {
      Some(ref seg) => seg.ele.capacity() + seg.ele.len() - 1,
      None => 0,
    }
  }

  fn to_string(&self) -> String {
    match self.base {
      Some(ref seg) => {
        let mut result = "[".to_string();
        let mut sg = seg;
        let mut sl = &seg.ele[];
        loop {
          for e in sl.iter().rev() {
            result.push_str(&format!(" {}", e)[]);
          }

          match sg.next {
            Some(ref next) => {
              sg = next;
              sl = sg.ele.as_slice();
            },
            None => break,
          }
        }

        result.push_str(" ]");
        result
      },
      None => "[]".to_string(),
    }
  }

  fn print_structure(&self) {
    let mut sg = &self.base;
    loop {
      match *sg {
        Some(ref seg) => {
          for e in seg.ele.iter().rev() {
            print!(" {}", e);
          }
          sg = &seg.next;
        },
        None => break,
      }
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
  let mut v: VList<i32> = VList { base: None };
  println!("zero value for type. empty VList: {}", v);
  v.print_structure();

  for a in range(1, 7).rev() {
    v.cons(a);
  }
  println!("demonstrate cons. 6 elements added: {}", v);
  v.print_structure();

  v.cdr();
  println!("demonstrate cdr. 1 elements removed: {}", v);
  v.print_structure();

  println!("demonstrate length. length = {}", v.len());
  println!("");

  println!("demonstrate element access. v[3] = {}", v.index(3).unwrap());
  println!("");

  v.cdr().unwrap().cdr();
  println!("show cdr releasing segment. 2 elements removed: {}", v);
  v.print_structure();
}
