
pub fn compareString(x: &str, y: &str) -> bool {
    let x_chars: Vec<char> = x.chars().collect();
    let y_chars: Vec<char> = y.chars().collect();

    let mut x_i: usize = 0;
    let mut y_i: usize = 0;

    while (x_i < x_chars.len()) && (y_i < y_chars.len()) {
        if x_chars[x_i] > y_chars[y_i] {
            return true;
        }
        if x_chars[x_i] < y_chars[y_i] {
            return false;
        }
        x_i += 1;
        y_i += 1;
    }
    if y_i == y_chars.len() && x_i < x_chars.len() {
        return true;
    }
    false
}
fn main() {
    if compareString("1234", "abcd")==true {
        println!("true");
    } else {
        println!("false");
    }
    if compareString("abcd", "abc")==true {
        println!("true");
    } else {
        println!("false");
    }
}

