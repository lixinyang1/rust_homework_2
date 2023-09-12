

use std::cell::RefCell;

pub struct MyLIFO<T> {
    stack: RefCell<Vec<T>>
}

impl<T> MyLIFO<T>{
    pub fn new() -> MyLIFO<T> {
        MyLIFO{ stack: RefCell::new(Vec::new()) }
    }
    pub fn push(&self, x: T) -> () {
        self.stack.borrow_mut().push(x);
    }
    /* pop
        弹出栈顶元素。（通过 Option 返回）
        如果栈为空，返回 None。
    */
    pub fn pop(&self) -> Option<T> {
        self.stack.borrow_mut().pop()
    }
}
fn main() {
    let stack = MyLIFO::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.pop(), Some(2));

    stack.push(4);
    assert_eq!(stack.pop(), Some(4));
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), None);
}