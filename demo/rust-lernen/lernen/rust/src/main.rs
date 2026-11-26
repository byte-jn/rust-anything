//! Lernprogramm: die wichtigsten Rust-Konzepte an einem kleinen Notenbuch.
//! Starten: `cargo run` in diesem Ordner.
//! Dieselbe Logik steht in ../python/lernen.py und ../go/lernen.go.
//! Vergleich die drei Abschnitt für Abschnitt (die Nummern stimmen überein).

use std::collections::HashMap;
use std::fmt;

// ── 1. Structs: nur Daten, keine Vererbung ─────────────────────────────────
// `#[derive(Debug)]` lässt den Compiler eine Debug-Ausgabe ({:?}) erzeugen.
#[derive(Debug)]
struct Schueler {
    name: String,   // String: gehört dem Struct, liegt auf dem Heap
    noten: Vec<u8>, // u8: Zahl 0..=255 ohne Vorzeichen
}

// ── 2. impl: Methoden hängen am Typ, getrennt von den Daten ────────────────
impl Schueler {
    // Einen Konstruktor gibt es nicht. `new` ist nur eine Namenskonvention.
    // `Self` steht für den eigenen Typ (hier: Schueler).
    fn new(name: &str) -> Self {
        Schueler {
            name: name.to_string(),
            noten: Vec::new(),
        }
    }

    // `&mut self`: die Methode darf das Objekt verändern.
    // Rückgabe `Result`: an der Signatur sieht man, dass es schiefgehen kann.
    fn note_eintragen(&mut self, note: u8) -> Result<(), NotenFehler> {
        if !(1..=6).contains(&note) {
            return Err(NotenFehler::Ungueltig(note));
        }
        self.noten.push(note);
        Ok(()) // `()` heißt "nichts", wie void
    }

    // `&self`: nur lesen. `Option` statt null: entweder Some(wert) oder None.
    fn durchschnitt(&self) -> Option<f64> {
        if self.noten.is_empty() {
            return None;
        }
        // Iterator-Kette: jede Note nach u32 umwandeln, dann summieren.
        let summe: u32 = self.noten.iter().map(|&n| n as u32).sum();
        Some(summe as f64 / self.noten.len() as f64) // letzte Zeile ohne ; = Rückgabewert
    }
}

// ── 3. Enums mit Daten: jede Variante trägt ihre eigenen Felder ────────────
#[derive(Debug)]
enum NotenFehler {
    Ungueltig(u8),    // trägt die falsche Zahl
    KeinText(String), // trägt den Text, der keine Zahl war
}

// ── 4. Traits: gemeinsames Verhalten, wie Interfaces in Java ───────────────
// Display ist ein Trait der Standardbibliothek: damit klappt `{}` in println!.
impl fmt::Display for NotenFehler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // match ist erschöpfend: fehlt eine Variante, kompiliert es nicht.
        match self {
            NotenFehler::Ungueltig(n) => write!(f, "Note {n} gibt es nicht (nur 1 bis 6)"),
            NotenFehler::KeinText(t) => write!(f, "'{t}' ist keine Zahl"),
        }
    }
}

// Ein eigener Trait. Jeder Typ kann ihn implementieren, auch nachträglich.
trait Bewertung {
    fn urteil(&self) -> String;
}

impl Bewertung for Schueler {
    fn urteil(&self) -> String {
        // match mit Bedingungen ("guards") nach dem `if`
        match self.durchschnitt() {
            None => format!("{}: noch keine Noten", self.name),
            Some(d) if d <= 2.0 => format!("{}: sehr stark ({d:.2})", self.name),
            Some(d) if d <= 4.0 => format!("{}: solide ({d:.2})", self.name),
            Some(d) => format!("{}: Achtung ({d:.2})", self.name),
        }
    }
}

// ── 5. Fehler weiterreichen mit `?` ────────────────────────────────────────
// `?` heißt: ist es ein Err, gib ihn sofort an den Aufrufer zurück.
// Das ist wie eine Exception, die weitergeworfen wird, nur sichtbar im Code.
fn note_aus_text(text: &str) -> Result<u8, NotenFehler> {
    let zahl: u8 = text
        .trim()
        .parse()
        .map_err(|_| NotenFehler::KeinText(text.to_string()))?;
    if !(1..=6).contains(&zahl) {
        return Err(NotenFehler::Ungueltig(zahl));
    }
    Ok(zahl)
}

// ── 6. Ownership und Borrowing: das, was Rust einzigartig macht ────────────
fn ownership_demo() {
    println!("\n== 6. Ownership und Borrowing ==");

    let a = String::from("Hallo");
    let b = a; // MOVE: der Besitz wandert zu b, a ist ab jetzt ungültig
    // println!("{a}"); // <- Kommentar entfernen: "borrow of moved value: `a`"
    println!("b = {b}");

    let c = b.clone(); // echte Kopie. Kostet Speicher, deshalb musst du es hinschreiben.
    println!("b = {b}, c = {c}");

    let laenge = zaehle(&c); // & = ausleihen. c gehört danach immer noch uns.
    println!("'{c}' hat {laenge} Bytes");

    let mut liste = vec![1, 2, 3];
    anhaengen(&mut liste); // &mut = genau EINE schreibende Leihe zur selben Zeit
    println!("liste = {liste:?}");

    let x = 5;
    let y = x; // Zahlen sind `Copy`: werden kopiert, beide bleiben gültig
    println!("x = {x}, y = {y}");
} // <- hier endet der Gültigkeitsbereich: b, c, liste werden automatisch freigegeben

fn zaehle(s: &str) -> usize {
    s.len()
}

fn anhaengen(v: &mut Vec<i32>) {
    v.push(4);
}

// ── 7. Iteratoren, Closures, HashMap ───────────────────────────────────────
// `&[Schueler]` ist ein "Slice": eine geliehene Ansicht auf eine Liste.
fn iterator_demo(klasse: &[Schueler]) {
    println!("\n== 7. Iteratoren, Closures, HashMap ==");

    // filter/map/collect: wie List-Comprehensions in Python oder Streams in Java.
    // `|s| ...` ist eine Closure (Lambda).
    let gute: Vec<&str> = klasse
        .iter()
        .filter(|s| s.durchschnitt().is_some_and(|d| d <= 2.5))
        .map(|s| s.name.as_str())
        .collect();
    println!("gut (Schnitt <= 2.5): {gute:?}");

    let mut haeufigkeit: HashMap<u8, u32> = HashMap::new();
    for s in klasse {
        for &n in &s.noten {
            // entry: Eintrag holen oder mit 0 anlegen, dann erhöhen
            *haeufigkeit.entry(n).or_insert(0) += 1;
        }
    }
    let mut sortiert: Vec<(u8, u32)> = haeufigkeit.into_iter().collect();
    sortiert.sort();
    println!("Notenverteilung (Note, Anzahl): {sortiert:?}");
}

fn main() {
    println!("== 1-4. Structs, Methoden, Enums, Traits ==");
    let mut anna = Schueler::new("Anna"); // mut, weil wir Noten eintragen
    let mut ben = Schueler::new("Ben");
    let cem = Schueler::new("Cem"); // bleibt ohne Noten, braucht kein mut

    for n in [1, 2, 1] {
        anna.note_eintragen(n).expect("gültige Note"); // expect: bei Err abbrechen
    }
    for n in [3, 4, 5] {
        ben.note_eintragen(n).expect("gültige Note");
    }

    // `if let`: nur den einen Fall behandeln, der uns interessiert
    if let Err(e) = ben.note_eintragen(9) {
        println!("Fehler abgefangen: {e}");
    }

    println!("{}", anna.urteil());
    println!("{}", ben.urteil());
    println!("{}", cem.urteil());
    println!("Debug-Ausgabe: {anna:?}");

    println!("\n== 5. Result und ? ==");
    for eingabe in ["2", " 5 ", "sieben", "0"] {
        match note_aus_text(eingabe) {
            Ok(n) => println!("'{eingabe}' -> Note {n}"),
            Err(e) => println!("'{eingabe}' -> Fehler: {e}"),
        }
    }

    ownership_demo();

    // anna, ben und cem WANDERN in den Vec. Danach kann man sie nicht mehr direkt benutzen.
    let klasse = vec![anna, ben, cem];
    iterator_demo(&klasse); // ausleihen: klasse gehört weiter main
}
