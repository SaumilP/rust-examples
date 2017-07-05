fn find_max<'a, T: Ord>(lst: &'a Vec<T>) -> Option<&'a T> {
    let mut max = None;

    for i in lst.iter() {
        max = match max {
            None => Some(i),
            Some(ref m) if i > *m => Some(i),
            _ => max
        }
    }
    max
}

#[test]
fn test_find_max1() {
    let v = vec!(0i32, 1,4,5,6,7,3, 9);
    let nine = 9i32;
    assert_eq!(Some(&nine), find_max(&v));
}

#[cfg(not(test))]
fn main() {
    let int_v = vec!(5i32, 2, 0, 8, 40, 1, 39, 23, 49, 20, 87, 92, 999 ,7 ,20);
    println!("find_max1 -> {:?}", find_max(&int_v));
}