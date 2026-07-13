fn sieve(n: usize) -> Vec<usize> {
    if n <= 1 {
        return vec![];
    }

    let mut marked = vec![false; n+1];
    marked[0] = true;
    marked[1] = true;
    marked[2] = true;
    for p in 2..n {
        for i in (2*p..n).filter(|&n| n % p == 0) {
            marked[i] = true;
        }
    }
    marked.iter()
          .enumerate()
          .filter_map(|(i, &m)| if m { None } else { Some(i) })
          .collect()
}

fn is_prime(n: usize) -> bool {
    n != 0 && n != 1 && (2..).take_while(|i| i*i <= n).all(|i| n % i != 0)
}

fn main() {
    println!("Sieve 3: {:?} {:?}", sieve(3), sieve(3).into_iter().all(is_prime));
    println!("Sieve 4: {:?} {:?}", sieve(4), sieve(4).into_iter().all(is_prime));
    println!("Sieve 5: {:?} {:?}", sieve(5), sieve(5).into_iter().all(is_prime));
    println!("Sieve 6: {:?} {:?}", sieve(6), sieve(6).into_iter().all(is_prime));
    println!("Sieve 8: {:?} {:?}", sieve(8), sieve(8).into_iter().all(is_prime));
}
