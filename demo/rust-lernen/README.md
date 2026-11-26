# Rust lernen: Rust, Python und Go im Vergleich

## lernen/

Dasselbe kleine Notenbuch in drei Sprachen. Die Abschnittsnummern 1–7 stimmen überein,
also am besten nebeneinander öffnen:

| # | Thema | Rust | Python | Go |
|---|---|---|---|---|
| 1 | Datentyp | `struct` | `@dataclass` | `struct` |
| 2 | Methoden | `impl`-Block | in der Klasse | `func (s *T)` |
| 3 | Fehlerarten | `enum` mit Daten | Exception-Klassen | Typen mit `Error()` |
| 4 | Gemeinsames Verhalten | `trait` | `Protocol` | `interface` |
| 5 | Fehler weiterreichen | `?` | `raise` | `if err != nil` |
| 6 | Speicher | Ownership/Borrowing | GC, geteilte Objekte | GC, geteilte Slices |
| 7 | Listen verarbeiten | Iterator-Ketten | Comprehensions | Schleifen |

```powershell
cd lernen\rust; cargo run
python lernen\python\lernen.py
cd lernen\go; go run lernen.go
```

## speedtest/

Vier gleiche Aufgaben, gleiche Zufallszahlen, gleiche Prüfsummen:

```powershell
.\speedtest.ps1
```

Gemessen am 26.11.2026 auf einem AMD Ryzen 7 5700X (8 Kerne), zwei Läufe, die Zahlen
lagen jeweils innerhalb weniger Prozent. Hier steht der zweite Lauf:

| Aufgabe | Rust 1.100 (release) | Go 1.27.4 | Python 3.14.7 |
|---|---:|---:|---:|
| fib(32) rekursiv | 4.0 ms | 7.8 ms | 264.8 ms |
| Primzahlsieb 10 Mio | 10.4 ms | 16.9 ms | 889.7 ms |
| 2 Mio sortieren | 27.5 ms | 122.4 ms | 998.4 ms |
| HashMap 5 Mio | 72.7 ms | 89.3 ms | 1434.9 ms |
| **Gesamt** | **114.7 ms** | **236.3 ms** | **3587.7 ms** |

Rust läuft hier mit der GNU-Toolchain (`stable-x86_64-pc-windows-gnu`), weil keine
Visual-Studio-Build-Tools installiert sind.
