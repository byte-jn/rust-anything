"""Lernprogramm: dieselbe Logik wie ../rust/src/main.rs, in Python.

Starten: python lernen.py
Die Kommentare sagen, wo Rust anders denkt. Die Abschnittsnummern stimmen mit
der Rust- und der Go-Fassung überein.
"""

from __future__ import annotations

from collections import Counter
from dataclasses import dataclass, field
from typing import Protocol


# ── 3. Fehler: Python nutzt Exception-Klassen ────────────────────────────────
# Rust: enum NotenFehler { Ungueltig(u8), KeinText(String) }, ein Typ mit zwei
# Varianten, die der Compiler bei `match` alle sehen will.
class UngueltigeNote(Exception):
    def __init__(self, note: int):
        super().__init__(f"Note {note} gibt es nicht (nur 1 bis 6)")


class KeinText(Exception):
    def __init__(self, text: str):
        super().__init__(f"'{text}' ist keine Zahl")


# ── 1. Klasse statt struct ───────────────────────────────────────────────────
@dataclass
class Schueler:
    name: str
    noten: list[int] = field(default_factory=list)

    # ── 2. Methoden stehen direkt in der Klasse (Rust: eigener impl-Block) ──
    def note_eintragen(self, note: int) -> None:
        # Python wirft eine Exception. Rust gibt Result<(), NotenFehler> zurück:
        # dort sieht man schon an der Signatur, dass es schiefgehen kann.
        if not 1 <= note <= 6:
            raise UngueltigeNote(note)
        self.noten.append(note)

    # Rust: Option<f64>. Python: float | None. Aber niemand zwingt dich,
    # None zu prüfen. In Rust kommst du ohne Prüfung nicht an den Wert.
    def durchschnitt(self) -> float | None:
        if not self.noten:
            return None
        return sum(self.noten) / len(self.noten)

    # ── 4. "Trait"-Methode. Python prüft das per Duck-Typing zur Laufzeit. ──
    def urteil(self) -> str:
        match self.durchschnitt():
            case None:
                return f"{self.name}: noch keine Noten"
            case d if d <= 2.0:
                return f"{self.name}: sehr stark ({d:.2f})"
            case d if d <= 4.0:
                return f"{self.name}: solide ({d:.2f})"
            case d:
                return f"{self.name}: Achtung ({d:.2f})"


# Protocol ist das, was in Python einem Trait am nächsten kommt:
# jede Klasse mit einer Methode urteil() passt, ohne es anzumelden.
class Bewertung(Protocol):
    def urteil(self) -> str: ...


# ── 5. Fehler weiterreichen ──────────────────────────────────────────────────
# In Python fliegt die Exception einfach nach oben. Rust: `?` macht dasselbe,
# aber sichtbar, und nur wenn die Funktion selbst Result zurückgibt.
def note_aus_text(text: str) -> int:
    try:
        zahl = int(text.strip())
    except ValueError:
        raise KeinText(text) from None
    if not 1 <= zahl <= 6:
        raise UngueltigeNote(zahl)
    return zahl


# ── 6. Kein Ownership: Namen sind nur Zettel an Objekten ─────────────────────
def ownership_demo() -> None:
    print("\n== 6. Kein Ownership in Python ==")

    a = "Hallo"
    b = a  # kein Move: a und b zeigen auf DASSELBE Objekt
    print(f"a = {a}, b = {b}  (beide gültig)")

    liste = [1, 2, 3]
    andere = liste  # zweiter Name, gleiche Liste
    andere.append(4)
    print(f"liste = {liste}  <- verändert, obwohl wir nur 'andere' angefasst haben!")
    # Rust lässt das nicht zu: zwei Namen, die gleichzeitig schreiben dürfen,
    # gibt es dort nicht. Genau dieser Fehler fällt also schon beim Kompilieren auf.

    kopie = liste.copy()  # echte Kopie (Rust: .clone())
    kopie.append(5)
    print(f"liste = {liste}, kopie = {kopie}")
    # Aufgeräumt wird später vom Garbage Collector (Rust: am Ende des Blocks).


# ── 7. Comprehensions, Lambdas, dict/Counter ─────────────────────────────────
def iterator_demo(klasse: list[Schueler]) -> None:
    print("\n== 7. Comprehensions, Lambdas, Counter ==")

    # Rust: .iter().filter(...).map(...).collect()
    gute = [
        s.name
        for s in klasse
        if (d := s.durchschnitt()) is not None and d <= 2.5
    ]
    print(f"gut (Schnitt <= 2.5): {gute}")

    # Rust: HashMap mit entry(n).or_insert(0) += 1
    haeufigkeit = Counter(n for s in klasse for n in s.noten)
    print(f"Notenverteilung (Note, Anzahl): {sorted(haeufigkeit.items())}")


def main() -> None:
    print("== 1-4. Klassen, Methoden, Fehler, Protocols ==")
    anna = Schueler("Anna")  # Python kennt kein mut: alles ist veränderbar
    ben = Schueler("Ben")
    cem = Schueler("Cem")

    for n in [1, 2, 1]:
        anna.note_eintragen(n)
    for n in [3, 4, 5]:
        ben.note_eintragen(n)

    try:
        ben.note_eintragen(9)
    except UngueltigeNote as e:
        print(f"Fehler abgefangen: {e}")

    for s in (anna, ben, cem):
        print(s.urteil())
    print(f"Debug-Ausgabe: {anna!r}")

    print("\n== 5. Exceptions ==")
    for eingabe in ["2", " 5 ", "sieben", "0"]:
        try:
            print(f"'{eingabe}' -> Note {note_aus_text(eingabe)}")
        except (KeinText, UngueltigeNote) as e:
            print(f"'{eingabe}' -> Fehler: {e}")

    ownership_demo()

    klasse = [anna, ben, cem]  # kein Move: anna & Co. bleiben benutzbar
    iterator_demo(klasse)


if __name__ == "__main__":
    main()
