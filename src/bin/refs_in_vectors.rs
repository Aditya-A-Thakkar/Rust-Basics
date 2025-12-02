fn main() {
    let mut x: i32 = 22;
    let mut v: Vec<&i32> = vec![];
    let p: &i32 = &x;
    //v.push(p);
    //v.push1(p);
    // push2(v, p);
    // push2 is almost the same as push1, except push1 is a method and this is not. We can call push1 or push2 here.
    //x += 1;
    //take_ref(&v);
    // There is an error above if we uncomment  'x += 1' and 'take_ref(&v)'. The reason being, the compiler thinks
    // the call to push1 might leak the reference p into the vector v (based on the rule described below). The compiler also
    // thinks the vector's contents could be looked up inside take_ref. So, the compiler thinks the (immutable) reference
    // to x is live inside the vector while x is being mutated. This is the cause of the error.

    // take_ownership(v);
    // we put the call above just to take away all permissions from v, thus marking v and its contents
    // as inaccessible. If we move 'x += 1' to this point, there will be no error, because v is no longer live,
    // and hence the immutable reference to x inside v is not live.

    // Here is (our guess) regarding the general principle. If we pass two structs s1 and s2 to a function (or references to two structs s1 and s2)
    // to a function foo(), conservatively the compiler may assume that any reference inside either struct (in a direct or transitive field)
    // will henceforth live as long as the other struct is accessible. This is to account for  the possibility that foo could copy references found inside
    // one struct to the fields of other struct. The compiler does not inspect the code of the functionn.
    // However, it is not really as strict as the blanket rule above. There are various optimizations, based on the types of 
    // fields and their (im)mutability, etc., which the compiler uses to refine its view of which references 
    // could actually get copied. 
    // TODO: Aditya, show some examples where the compiler thinks a reference is leaked from one struct to the other,
    // and some other examples where some optimizations kicked in. Use structs in this examples (not library types like Vec).

    
   

    let mut smth = MyStruct { my_num: "hello" };
    let r = &mut smth;
    //empty(& mut smth/*,  p*/);
    // TODO: Aditya, see why it does not compile if we uncomment the call above
    take_ownership(smth);
}

fn take_ref(w: &Vec<&i32>) {

}

fn take_ownership<T>(v: T) {

}

trait Push1<T> {
    fn push1(&mut self, data: T);
}

impl<T> Push1<T> for Vec<T> {
    fn push1(&mut self, data: T) {
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

fn empty<'a>(some_struct: &'a mut MyStruct<'a>/* , y: &'a i32*/) {
}

struct MyStruct<'a> {
    my_num: &'a str,
}
