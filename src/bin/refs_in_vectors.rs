fn main() {
    let mut x: i32 = 22;
    let mut v: Vec<&i32> = vec![];
    let p: &i32 = &x;
    v.push(p);
    // x += 1;
    // If you uncomment the line above, it would be an error, because
    // the immutable reference to x is stored inside the vector v, and the vector
    // is still accessible here.
    take_ownership(v);
    // we put the call above just to take away all permissions from v, thus marking v and its contents
    // as inaccessible.

    // v.push1(p);
    // push2(v, p);
    // let mut smth = MyStruct { my_num: &1 };
    // empty(&mut smth, p);
    // take(smth);
}

fn take_ownership<T>(v: T) {

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
