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

    // Here is (our guess) regarding the general principle. If we pass two structs s1 and s2 to a function (or references to two structs s1 and s2)
    // to a function foo(), conservatively the compiler may assume that any reference inside either struct (in a direct or transitive field)
    // will henceforth live as long as the other struct is accessible. This is to account for  the possibility that foo could copy references found inside
    // one struct to the fields of other struct. The compiler does not inspect the code of the functionn.
    // However, it is not really as strict as the blanket rule above. There are various optimizations, based on the types of 
    // fields and their (im)mutability, etc., which the compiler uses to refine its view of which references 
    // could actually get copied. 
    // TODO: Aditya, show some examples where the compiler thinks a reference is leaked from one struct to the other,
    // and some other examples where some optimizations kicked in. Use structs in this examples (not library types like Vec).

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
