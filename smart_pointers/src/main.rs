use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
#[allow(dead_code)]
enum BoxedList {
    Cons(i32, Box<BoxedList>),
    Nil,
}

#[derive(Debug)]
#[allow(dead_code)]
enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

#[derive(Debug)]
#[allow(dead_code)]
enum RefCellList {
    Cons(Rc<RefCell<i32>>, Rc<RefCellList>),
    Nil
}

#[derive(Debug)]
#[allow(dead_code)]
enum MutTailList {
    Cons(i32, RefCell<Rc<MutTailList>>),
    Nil,
}

impl MutTailList {
    fn tail(&self) -> Option<&RefCell<Rc<MutTailList>>> {
        use MutTailList::{Cons, Nil};
        match self {
            Cons(_, item) => Some(item),
            Nil => None
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

use BoxedList::{Cons, Nil};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

struct DerefMutExample<T> {
    value: T,
}

impl<T> Deref for DerefMutExample<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for DerefMutExample<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T> Drop for DerefMutExample<T> {
    fn drop(&mut self) {
        println!("Dropping DerefMutExample");
    }
}

fn main() {
    let b = Box::new(5);
    let a = *b;
    println!("b = {b}, a = {a}");

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("list: {list:?}");

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let mut z = DerefMutExample { value: 1 };
    println!("DerefMutExample created");
    *z = 2;
    assert_eq!(2, *z);
    drop(z);
    println!("Still in main after dropping DerefMutExample");
    {
        use RcList::{Cons, Nil};
        let j =
            Rc::new(Cons(5,
                Rc::new(Cons(10,
                    Rc::new(Nil)
                )
            ),
        ));
        println!("ref count after creating j: {}", Rc::strong_count(&j));
        let k = Cons(3, Rc::clone(&j));
        println!("ref count after creating k: {}", Rc::strong_count(&j));
        {
            let l = Cons(4, Rc::clone(&j));
            println!("ref count after creating l: {}", Rc::strong_count(&j));
            println!("j: {j:?}, k: {k:?}, l: {l:?}");
        }
        println!("ref count after dropping l: {}", Rc::strong_count(&j));
    }
    
    {
        use RefCellList::{Cons, Nil};
        let ref_cell_val = Rc::new(RefCell::new(5));

        let q = Rc::new(Cons(
            Rc::clone(&ref_cell_val),
            Rc::new(Nil)
        ));
        let r = Cons(
            Rc::new(RefCell::new(3)),
            Rc::clone(&q)
        );
        let s = Cons(
            Rc::new(RefCell::new(4)),
            Rc::clone(&q)
        );
        println!("RefCellList before modification: q = {q:?}, r = {r:?}, s = {s:?}");
        *ref_cell_val.borrow_mut() += 10;
        println!("RefCellList after modification: q = {q:?}, r = {r:?}, s = {s:?}");
    }

    {
        use MutTailList::{Cons, Nil};
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());

        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));

        // Uncomment the next line to see that we have a cycle;
        // it will overflow the stack.
        // println!("a next item = {:?}", a.tail());
    }

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![])
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!(
        "leaf (just created) strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)])
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch (inner scope) strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );

        println!(
            "leaf (inner scope) strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );

    }

    println!("leaf (after branch destruction) parent = {:?}", leaf.parent.borrow().upgrade());

    println!(
        "leaf (after branch destruction) strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
    
}
