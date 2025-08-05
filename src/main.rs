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
