trait Pipe<T> where T: Copy {
    fn push(t: T);
    fn pop() -> T;
}

fn main() {
    println!("Hello, world!");
}
