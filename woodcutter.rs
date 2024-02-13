fn main() {
    let n: usize = 5;
    let v: Vec<(i32, i32)> = vec![(1, 2), (2, 1), (5, 10), (10, 9), (19, 1)];
    
    if n == 1 {
        println!("1");
        return;
    }
    
    let mut ans = 2;
    let mut dist = v[0].0;
    
    for i in 1..n-1 {
        if v[i].0 - v[i].1 > dist {
            dist = v[i].0;
            ans += 1;
        } else if v[i].0 + v[i].1 < v[i+1].0 {
            dist = v[i].0 + v[i].1;
            ans += 1;
        } else {
            dist = v[i].0;
        }
    }
    
    println!("{}", ans);
}
