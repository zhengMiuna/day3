//test for macro

#[macro_use]
extern crate std;

use std::collections::HashMap;
use std::cell::RefCell;
use std::ops::Deref;

macro_rules! hash_map {
    ($($key: expr => $val: expr),*) => {
        {
            let mut map=HashMap::new();
            $(
                map.insert($key,$val);
            )*
            map
        }
    };
}

//test for rc

struct  MyRc<T> {
    data: *mut T,
    count: *mut usize,
}

impl<T> MyRc<T> {

    fn new(data: T) -> Self {
        let data= Box::new(data);
        let count=Box::new(1);
        Self {
            data: Box::into_raw(data),
            count: Box::into_raw(count),
        }
    }

    fn clone(&self) -> Self {
        unsafe{
            *self.count += 1;
        }
        Self{..*self}
    }

    fn strong_count(&self) -> usize {
        unsafe { *self.count }
    }
}

impl<T> Deref for MyRc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.data }
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        unsafe { *self.count -= 1; }
    }
}

//test for simple stack

#[derive(Debug)]
struct SimpleStack<T> {
    stack: RefCell<Vec<T>>,
}

impl<T> SimpleStack<T> {
    fn new() -> SimpleStack<T> {
        SimpleStack { stack: RefCell::new(Vec::new()) }
    }

    fn push(&self, value: T) {
        self.stack.borrow_mut().push(value);
    }

    fn pop(&self) -> Option<T> {
        self.stack.borrow_mut().pop()
    }
}



fn main() {
    //test for macro

    println!("test1 macro:");
    let map = hash_map!{
        "one" => 1,
        "two" => 2,
        "three" => 3
    };

    println!("{:?}",map);

    //test for rc

    println!("test2 rc:");
    let five = MyRc::new(5);
    {
        let fivel=five.clone();
        println!("five: {}",*fivel);
        println!("before count: {}",MyRc::strong_count(&fivel));
    }
    println!("after count: {}",MyRc::strong_count(&five));

    //test for simple stack

    println!("test3 simple stack:");
    let stack = SimpleStack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("Poped value: {:?}",stack.pop());
    println!("Poped value: {:?}",stack.pop());

    stack.push(4);

    println!("Poped value: {:?}",stack.pop());
    println!("Poped value: {:?}",stack.pop());
    println!("Poped value: {:?}",stack.pop());

}
