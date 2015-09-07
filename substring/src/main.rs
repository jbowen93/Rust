fn main() {
    let main = "aaaaaaaaaaaaab";
    let sub = "aab";
    
    let main_bytes = main.as_bytes();
    let sub_bytes = sub.as_bytes();

    let max_len = main_bytes.len() - sub_bytes.len();
    
    for i in 0..(max_len + 1) {
        let mut found = true;
        for j in 0..sub_bytes.len() {
            if main_bytes[i+j] != sub_bytes[j] {
                found = false;
                break;
            }
        }
        if found {
            println!("Found Substring!");
            break;
        }
        if i == max_len + 1 && found == false {
            println!("No Substring!");
        }
    }
}
