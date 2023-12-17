fn find_sqrt(n: i32) -> i32 {
    let mut l = 1;
    let mut r = n;
    
    while l <= r {
        let mid = (l + r) / 2;
        let val = mid * mid;
        
        if val <= n {
            l = mid + 1;
        } else {
            r = mid - 1;
        }
    }
    
    r
}

fn main() {
    println!("{}", find_sqrt(36));
}
