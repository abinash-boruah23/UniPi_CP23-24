fn main() {
    let a = vec![15, 37, 1, 32, 135, 2];
    let mut curr = a[a.len() - 1];

    for &element in a.iter().rev() {
        if element >= curr {
            print!("{} ", element);
            curr = element;
        }
    }
}
