fn main() {
    let ans = is_even(32);
    println!("{}", ans);
}
// function which returns true if the number is even
// and this is rust function
fn is_even(num: i32) -> bool {
    if num % 2==0 {
        return true;
    } else {
        return false;
    }
}