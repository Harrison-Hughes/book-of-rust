fn largest_by_copy<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_by_clone<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();

    for item in list {
        if item > &largest {
            largest = item.clone();
        }
    }
    largest
}

fn largest_by_type_reference<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_by_copy(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_by_clone(&char_list);
    println!("The largest char is {}", result);

    let result = largest_by_type_reference(&char_list);
    println!("The largest char is {}", result);
}
