fn main() {
    let s = String::from("arepretestsstrongforthisproblem");

    if s.len() == 1 {
        println!("{}", s);
        return;
    }

    let mut output = String::new();
    let mut mini = s.chars().last().unwrap();
    for c in s.chars().rev() {
        if c >= mini {
            output.push(c);
            mini = c;
        }
    }

    output = output.chars().rev().collect();

    // Print the output
    println!("{}", output);
}
