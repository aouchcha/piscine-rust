use find_factorial::*;

fn main() {
    println!("The factorial of 0 = {}", factorial(0));
    println!("The factorial of 1 = {}", factorial(1));
    println!("The factorial of 5 = {}", factorial(5));
    println!("The factorial of 10 = {}", factorial(10));
    println!("The factorial of 19 = {}", factorial(19));
}

#[cfg(test)]
mod tests {
    use find_factorial::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(0));
        assert_eq!(1, factorial(1));
        assert_eq!(120, factorial(5));
        assert_eq!(40320, factorial(8));
        assert_eq!(3628800, factorial(10));
        assert_eq!(87178291200, factorial(14));
        assert_eq!(6402373705728000, factorial(18));
        assert_eq!(121645100408832000, factorial(19));
        assert_eq!(2432902008176640000, factorial(20));
    }
}