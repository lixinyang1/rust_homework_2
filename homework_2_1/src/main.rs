
pub struct tem<T: std::ops::Add<Output = T> + Clone>{
    vector:Vec<T>,
}
impl<T:std::ops::Add<Output = T> + Clone> tem<T>{
    pub fn new(v:&Vec<T>) -> Self{
        tem { vector: v.clone()}
    }
    pub fn push(&mut self, item:T){
        self.vector.push(item);
    }
    pub fn pop(&mut self){
        self.vector.pop();        
    }
    pub fn clear(&mut self){
        self.vector.clear();        
    }
    pub fn sum(&self)-> Option<T>{
        if self.vector.len() == 0 {
            return None;
        }
        let mut v_iter: std::slice::Iter<'_, T> = self.vector.iter();
        let mut res: T = v_iter.next().unwrap().clone();
        loop {
            match v_iter.next() {
                None => {break;}
                Some(value) => { res = res + value.clone(); }
            }
        }
        Some(res)
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { x: (self.x + other.x), y: (self.y + other.y) }
    }
}

fn main() {
    let mut vec: Vec<Point> = Vec::new();
    for i in 0..5 {
        vec.push(Point { x: i, y: 10 - i });
    }
    let mut buf: tem<Point> = tem::new(&vec);
    // 测试sum方法
    match buf.sum() {
        Some(p) => {
            println!("({}, {})", p.x, p.y);
        }
        None => {
            println!("The buffer is empty");
        }
    }
    buf.push(Point { x: 4, y: 5 });
    // 增加一个元素后再次测试sum方法
    match buf.sum() {
        Some(p) => {
            println!("({}, {})", p.x, p.y);
        }
        None => {
            println!("The buffer is empty");
        }
    }
    // 当buf是空时测试sum方法
    buf.clear();

    match buf.sum() {
        Some(p) => {
            println!("({}, {})", p.x, p.y);
        }
        None => {
            println!("The buffer is empty");
        }
    }

}
