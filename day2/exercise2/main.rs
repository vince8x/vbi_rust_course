//Exercise 2
// Mục đích: giải quyết vấn đề ownership và borrowing ko dùng clone()
// Các bạn có thể sửa thêm logic để đúng với mục đichs bài này là liệt kê các số nguyên tố
fn main() {
    let mut count: u32 = 1;
    let mut num: u64 = 1;
    let mut primes: Vec<u64> = Vec::new();
    primes.push(2);

    while count < 10 {
        num += 2;
        if is_prime(num) {
            count += 1;
            primes.push(num);
        }
    }
    println!("{:?}", primes);
}

fn is_prime(num: u64) -> bool {
    for i in 2..num {
        if num > i && num % i == 0 {
            return false;
        }
    }

    true
}
