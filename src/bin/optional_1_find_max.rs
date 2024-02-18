// A Generic function to find max.number out of a list of any type

fn find_max<T: PartialOrd>(list: &[T]) -> Option<&T> {
    if list.is_empty() {
        return None;
    }
    
    let mut max = &list[0];

    for item in list.iter() {
        if item > max {
            max = item;
        }
    }
    
    Some(max)

}

fn main() {

    let l1 = [1,5,8];
    let max = find_max(&l1).unwrap().to_owned();
    println!("Max value in l1: {}", max);

    let l2: [_; 0] = [];
    let max2 = find_max::<Option<&u32>>(&l2);
    println!("Max value in l2: {:?}", max2);

}
