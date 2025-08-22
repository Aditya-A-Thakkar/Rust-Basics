fn main() {
    let mut x: i32 = 22;
    let mut v: Vec<&i32> = vec![];
    let r: &mut Vec<&i32> = &mut v;
    let p: &i32 = &x;
    r.push(p);
    // r.push1(p);
    // push2(r, p);
    // let mut smth = MyStruct { my_num: &1 };
    // empty(&mut smth, p);
    x += 1;
    take(v);
    // take(smth);
}

fn take<T>(v: T) {

}

trait Push1<T> {
    fn push1(&self, data: T);
}

impl<T> Push1<T> for Vec<T> {
    fn push1(&self, data: T) {
        // May or may not push data
        // self.push(data);
        println!("Do nothing!");
    }
}

fn push2<T>(v: &mut Vec<T>, data: T) {
    // May or may not push data
    // v.push(data);
    println!("Do nothing!");
}

fn empty<'a>(some_struct: &'a mut MyStruct<'a>, y: &'a i32) {
    // some_struct.my_num = y;
}

struct MyStruct<'a> {
    my_num: &'a i32,
}
