fn main() {
    const FIRST_NUM: u32 = 402238;
    const LAST_NUM: u32 = 864247;

    for number in FIRST_NUM..=LAST_NUM {
        let first_digit = number / 100000;
        let second_digit = number / 10000 % 10;
        let third_digit = number / 1000 % 10;
        let fourth_digit = number / 100 % 10;
        let fifth_digit = number / 10 % 10;
        let sixth_digit = number % 10;
        
    }


}
