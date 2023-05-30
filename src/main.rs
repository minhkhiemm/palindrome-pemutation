fn main() {
    let a = "string";
    let b = "xtring";
    let mut is_a_bigger = false;
    let mut is_b_bigger = false;
    if a.len() != b.len() {
        if a.len() > b.len() {
            is_a_bigger = true;
            if a.len() - 1 != b.len() {
                println!("not palindrome permutation");
                std::process::exit(1);
            }
        }

        if a.len() < b.len() {
            is_b_bigger = true;
            if a.len() != b.len() - 1 {
                println!("not palindrome permutation");
                std::process::exit(1);
            }
        }
    }

    let a_array = to_array::<6>(a);
    let b_array = to_array::<6>(b);
    let mut i: usize = 0;
    let mut counter: usize = 0;
    if a.len() == b.len() && a_array[5] != b_array[5] {
        counter += 1;
    }

    while i < 5 {
        if is_a_bigger {
            if a_array[i] != b_array[i] && a_array[i + 1] != b_array[i] {
                println!("not palindrome permutation");
                std::process::exit(1);
            }
        }

        if is_b_bigger {
            if b_array[i] != a_array[i] && b_array[i + 1] != a_array[i] {
                println!("not palindrome permutation");
                std::process::exit(1);
            }
        }

        if a.len() == b.len() {
            if a_array[i] != b_array[i] {
                counter += 1;
            }
            if counter >= 2 {
                println!("not palindrome permutation");
                std::process::exit(1);
            }
        }
        i += 1;
    }
    println!("palindrome permutation");
}

fn to_array<const N: usize>(s: &str) -> [char; N] {
    let mut chars = s.chars();
    [(); N].map(|_| chars.next().unwrap())
}
