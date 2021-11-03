// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }



struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
 
fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];

    // let result = largest(&number_list);

    // println!("The largest number is {}", result);

    // let number_list = vec!['y', 'm', 'a', 'q'];

    // let result = largest(&number_list);

    // println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("{:?} {:?}", integer.x, float.y);

    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
