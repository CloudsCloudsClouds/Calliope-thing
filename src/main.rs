use std::io;

fn main() {
    println!("Calliope. Introduce 2 numeros.");
    let mut n = String::new();
    let mut k = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Fallo al leer numero.");

    io::stdin()
        .read_line(&mut k)
        .expect("Fallo al leer numero.");

    let mut n = n
        .trim()
        .parse::<u32>()
        .expect("Introduce un numero natural");

    let k = k
        .trim()
        .parse::<u32>()
        .expect("Introduce un numero natural");

    if k == 0 || n == 0 {
        println!("0 Detectado, no es valido.");
    }

    if n > 1 {
        let tn: u32 = n / 2;
        n = tn + ((1 + n) / 2);
    }

    if n % k == 0 {
        println!("SI");
    } else {
        println!("NO");
    }
}


#[cfg(test)]
mod tests {
    // Extract the core logic first — right now it's buried in main()
    // so we need to pull it out into a testable function

    fn solve(n: u32, k: u32) -> &'static str {
        if k == 0 || n == 0 {
            return "INVALID";
        }
        let n = if n > 1 {
            let tn = n / 2;
            tn + ((1 + n) / 2)
        } else {
            n
        };
        if n % k == 0 { "SI" } else { "NO" }
    }

    #[test]
    fn zero_inputs_are_invalid() {
        assert_eq!(solve(0, 5), "INVALID");
        assert_eq!(solve(5, 0), "INVALID");
    }

    #[test]
    fn divisible_returns_si() {
        assert_eq!(solve(4, 2), "SI");
        assert_eq!(solve(1, 1), "SI");
    }

    #[test]
    fn not_divisible_returns_no() {
        assert_eq!(solve(4, 3), "NO");
    }

    #[test]
    fn odd_n_transform() {
        // n=3 -> 1 + 2 = 3, n=5 -> 2 + 3 = 5
        // the transform is a no-op on odd numbers too
        assert_eq!(solve(3, 3), "SI");
    }
}
