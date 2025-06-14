// Iterators
// All the iterators implement the Iterator trait and the next method

fn main() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    println!("{:?}", v1_iter.next().unwrap());
    println!("{:?}", v1_iter.next().unwrap());
    println!("{:?}", v1_iter.next().unwrap());
    println!("{:?}", v1_iter.next().is_none());

    let v2_iter = v1.iter();
    let total :i32 = v2_iter.sum(); // Iterator consumed here, it cannot be used any further
    println!("{:?}", total);

    // while there are methods that consume an iterator, there are methods like map and filter, which produce one
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }

    let shoes = vec![
        Shoe {
            size: 7,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 9,
            style: String::from("sandal"),
        },
        Shoe {
            size: 7,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 7);

    println!("{:?}", in_my_size);
}