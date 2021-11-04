fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
} 

// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }
 
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);

    println!("The largest number is {}", result);

    let number_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&number_list);

    println!("The largest char is {}", result);

    // let integer = Point { x: 5, y: 10 };
    // let float = Point { x: 1.0, y: 4.0 };

    // println!("{:?} {:?}", integer.x, float.y);

    // let p = Point { x: 5, y: 10 };

    // println!("p.x = {}", p.x());

    let string1 = String::from("long string is long");
    
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest  string is {}", result);
    }
}

