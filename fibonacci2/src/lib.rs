pub fn fibonacci(n: u32) -> u32 {
    if n == 0 || n == 1 {return n}
    let mut first = 0;
    let mut second = 1;
    let mut counter = 1;
    while counter < n {
        let temp = second;
        second += first; 
        first = temp;
        counter += 1;
    }
    second
}