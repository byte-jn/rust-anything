//! Speedtest: vier gleiche Aufgaben in Rust, Python und Go.
//! Starten: `cargo run --release` (ohne --release ist Rust bis zu 30x langsamer!)
//! Jede Aufgabe gibt eine Prüfsumme aus. Die muss in allen drei Sprachen gleich sein.

use std::collections::HashMap;
use std::fmt::Debug;
use std::hint::black_box;
use std::time::Instant;

// Gleiche Größen in allen drei Sprachen
const FIB_N: u32 = 32;
const SIEB_GRENZE: usize = 10_000_000;
const SORT_N: usize = 2_000_000;
const MAP_N: usize = 5_000_000;

/// Einfacher Zufallsgenerator (LCG), damit alle Sprachen dieselben Zahlen bekommen.
struct Lcg(u64);

impl Lcg {
    fn next(&mut self) -> u64 {
        // wrapping_*: Überlauf ist hier gewollt. Ein normales `*` würde im
        // Debug-Build bei Überlauf abbrechen, Rust ist da streng.
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.0 >> 33
    }
}

/// 1. Rekursion: misst Funktionsaufrufe (fib(32) braucht rund 7 Millionen Aufrufe).
fn fib(n: u32) -> u64 {
    if n < 2 {
        n as u64
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

/// 2. Sieb des Eratosthenes: misst Schleifen und Speicherzugriffe.
fn sieb(grenze: usize) -> usize {
    let mut ist_prim = vec![true; grenze];
    ist_prim[0] = false;
    ist_prim[1] = false;
    let mut i = 2;
    while i * i < grenze {
        if ist_prim[i] {
            let mut j = i * i;
            while j < grenze {
                ist_prim[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    ist_prim.iter().filter(|&&p| p).count()
}

/// 3. Zufallszahlen erzeugen und sortieren.
fn sortieren(n: usize) -> (u64, u64, u64) {
    let mut rng = Lcg(42);
    let mut v: Vec<u64> = (0..n).map(|_| rng.next()).collect();
    v.sort_unstable();
    (v[0], v[n / 2], v[n - 1])
}

/// 4. HashMap: zählen, wie oft jeder von 100.000 Schlüsseln vorkommt.
fn zaehlen(n: usize) -> (usize, u32) {
    let mut rng = Lcg(7);
    let mut anzahl: HashMap<u64, u32> = HashMap::new();
    for _ in 0..n {
        *anzahl.entry(rng.next() % 100_000).or_insert(0) += 1;
    }
    let groesste = *anzahl.values().max().expect("Map ist nicht leer");
    (anzahl.len(), groesste)
}

/// Generische Messfunktion: nimmt jede Closure, deren Ergebnis man ausgeben kann.
fn messen<T: Debug>(name: &str, aufgabe: impl FnOnce() -> T) -> f64 {
    let start = Instant::now();
    let ergebnis = aufgabe();
    let ms = start.elapsed().as_secs_f64() * 1000.0;
    println!("{name:<22} {ms:>10.1} ms   Prüfsumme: {ergebnis:?}");
    ms
}

fn main() {
    println!("Rust (rustc, --release)");
    // black_box verhindert, dass der Compiler das Ergebnis schon beim Übersetzen ausrechnet.
    let gesamt = messen("1. fib(32) rekursiv", || fib(black_box(FIB_N)))
        + messen("2. Primzahlsieb 10 Mio", || sieb(black_box(SIEB_GRENZE)))
        + messen("3. 2 Mio sortieren", || sortieren(black_box(SORT_N)))
        + messen("4. HashMap 5 Mio", || zaehlen(black_box(MAP_N)));
    println!("{:<22} {gesamt:>10.1} ms", "GESAMT");
}
