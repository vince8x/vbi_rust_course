use std::io;

fn main() {
    let mut haystack_vec: Vec<i32> = Vec::new();
    let mut needle_vec: Vec<i32> = Vec::new();
    let mut haystack_input = String::new();
    let mut needle_input = String::new();

    // Input the items of haystack from the keyboard
    println!("Enter the haystack array (separated by space):");
    io::stdin().read_line(&mut haystack_input).unwrap();
    for item in haystack_input.trim().split_whitespace() {
        match item.parse::<i32>() {
            Ok(n) => haystack_vec.push(n),
            Err(e) => panic!("Invalid input. Input must be a valid number. {}", e),
        }
    }

    // Input the items of the needle from the keyboard
    println!("Enter the needle array (separated by space):");
    io::stdin().read_line(&mut needle_input).unwrap();
    for item in needle_input.trim().split_whitespace() {
        match item.parse::<i32>() {
            Ok(n) => needle_vec.push(n),
            Err(e) => panic!("Invalid input. Input must be a valid number. {}", e),
        }
    }

    // Find the needle in the
    let result = find_needle_in_haystack(&needle_vec, &haystack_vec);

    println!("Result: {}", result);
}

fn find_needle_in_haystack(needle: &Vec<i32>, haystack: &Vec<i32>) -> bool {
    if needle.len() == 0 {
        return true;
    }

    if haystack.len() == 0 {
        return false;
    }

    let mut i = 0;
    let mut j = 0;
    while i < needle.len() {
        while j < haystack.len() {
            if needle[i] == haystack[j] {
                // There is an item not match
                j += 1;
                break;
            }
            j += 1;
        }
        i += 1;

        // End of the haystack but not end of needle
        // Still have items in the needle
        if j == haystack.len() && i <= needle.len() {
            return false;
        }
    }

    return true;
}

#[test]
fn test_empty_needle() {
    let needle = Vec::<i32>::new();
    let haystack: Vec<i32> = vec![6, 8, 10];

    assert_eq!(find_needle_in_haystack(&needle, &haystack), true);
}

#[test]
fn test_needle_is_found() {
    let needle: Vec<i32> = vec![6, 8];
    let haystack: Vec<i32> = vec![6, 8, 10];

    assert_eq!(find_needle_in_haystack(&needle, &haystack), true);
}

#[test]
fn test_needle_is_not_found_1() {
    let needle: Vec<i32> = vec![6, 9];
    let haystack: Vec<i32> = vec![6, 8, 10];

    assert_eq!(find_needle_in_haystack(&needle, &haystack), false)
}

#[test]
fn test_needle_is_not_found_2() {
    let needle: Vec<i32> = vec![6, 8, 10, 11];
    let haystack: Vec<i32> = vec![6, 8, 10];

    assert_eq!(find_needle_in_haystack(&needle, &haystack), false)
}
