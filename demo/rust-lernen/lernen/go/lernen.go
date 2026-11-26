// Lernprogramm: dieselbe Logik wie ../rust/src/main.rs, in Go.
//
// Starten: go run lernen.go
// Die Kommentare sagen, wo Rust anders denkt. Die Abschnittsnummern stimmen mit
// der Rust- und der Python-Fassung überein.
package main

import (
	"errors"
	"fmt"
	"slices"
	"strconv"
	"strings"
)

// ── 1. Structs: wie in Rust nur Daten, keine Vererbung ─────────────────────
// Großbuchstabe am Anfang = öffentlich (Rust: `pub`).
type Schueler struct {
	Name  string
	Noten []uint8 // Slice: wachsende Liste, wie Vec<u8>
}

// Einen Konstruktor gibt es auch in Go nicht, nur eine Namenskonvention.
func NeuerSchueler(name string) *Schueler {
	return &Schueler{Name: name}
}

// ── 3. Fehler: Go hat keine Enums mit Daten, also eigene Fehlertypen ───────
// Rust fasst beide als Varianten EINES enums zusammen, und match prüft, dass
// keine vergessen wurde. In Go kann jeder Typ mit Error()-Methode ein Fehler sein.
type UngueltigeNote struct{ Note int }

func (e UngueltigeNote) Error() string {
	return fmt.Sprintf("Note %d gibt es nicht (nur 1 bis 6)", e.Note)
}

type KeinText struct{ Text string }

func (e KeinText) Error() string {
	return fmt.Sprintf("'%s' ist keine Zahl", e.Text)
}

// ── 2. Methoden: wie Rusts impl stehen sie außerhalb des structs ───────────
// `s *Schueler` (Zeiger) entspricht ungefähr `&mut self`.
// Go gibt Fehler als zweiten Wert zurück. Rust: Result<(), NotenFehler>.
func (s *Schueler) NoteEintragen(note int) error {
	if note < 1 || note > 6 {
		return UngueltigeNote{note}
	}
	s.Noten = append(s.Noten, uint8(note))
	return nil // nil = kein Fehler
}

// Go hat kein Option. Üblich ist (wert, ok). Rust: Option<f64>.
// Den Unterschied merkst du beim Aufruf: in Go kannst du ok einfach ignorieren.
func (s *Schueler) Durchschnitt() (float64, bool) {
	if len(s.Noten) == 0 {
		return 0, false
	}
	summe := 0
	for _, n := range s.Noten {
		summe += int(n)
	}
	return float64(summe) / float64(len(s.Noten)), true
}

// ── 4. Interfaces: Go prüft sie strukturell, wie Pythons Protocol ──────────
// Rust verlangt ein ausdrückliches `impl Bewertung for Schueler`.
type Bewertung interface {
	Urteil() string
}

func (s *Schueler) Urteil() string {
	d, ok := s.Durchschnitt()
	switch {
	case !ok:
		return fmt.Sprintf("%s: noch keine Noten", s.Name)
	case d <= 2.0:
		return fmt.Sprintf("%s: sehr stark (%.2f)", s.Name, d)
	case d <= 4.0:
		return fmt.Sprintf("%s: solide (%.2f)", s.Name, d)
	default:
		return fmt.Sprintf("%s: Achtung (%.2f)", s.Name, d)
	}
}

// ── 5. Fehler weiterreichen: in Go von Hand mit `if err != nil` ────────────
// Rust schreibt dafür nur ein `?`.
func noteAusText(text string) (int, error) {
	zahl, err := strconv.Atoi(strings.TrimSpace(text))
	if err != nil {
		return 0, KeinText{text}
	}
	if zahl < 1 || zahl > 6 {
		return 0, UngueltigeNote{zahl}
	}
	return zahl, nil
}

// ── 6. Kein Ownership: Garbage Collector und geteilte Slices ───────────────
func ownershipDemo() {
	fmt.Println("\n== 6. Kein Ownership in Go ==")

	a := "Hallo"
	b := a // Strings sind unveränderlich, also kein Problem
	fmt.Printf("a = %s, b = %s  (beide gültig)\n", a, b)

	liste := []int{1, 2, 3}
	andere := liste // kopiert nur den Slice-Kopf, beide teilen sich den Speicher
	andere[0] = 99
	fmt.Printf("liste = %v  <- verändert, obwohl wir nur 'andere' angefasst haben!\n", liste)
	// Rust erlaubt nie zwei Namen, die gleichzeitig schreiben dürfen.

	kopie := slices.Clone(liste) // echte Kopie (Rust: .clone())
	kopie = append(kopie, 4)
	fmt.Printf("liste = %v, kopie = %v\n", liste, kopie)
	// Aufgeräumt wird später vom Garbage Collector (Rust: am Ende des Blocks).
}

// ── 7. Schleifen statt Iterator-Ketten, map statt HashMap ──────────────────
func iteratorDemo(klasse []*Schueler) {
	fmt.Println("\n== 7. Schleifen und maps ==")

	// Go hat kein filter/map. Rust: .iter().filter(...).map(...).collect()
	var gute []string
	for _, s := range klasse {
		if d, ok := s.Durchschnitt(); ok && d <= 2.5 {
			gute = append(gute, s.Name)
		}
	}
	fmt.Printf("gut (Schnitt <= 2.5): %v\n", gute)

	haeufigkeit := map[uint8]int{}
	for _, s := range klasse {
		for _, n := range s.Noten {
			haeufigkeit[n]++ // fehlender Schlüssel liefert 0, deshalb klappt ++ direkt
		}
	}
	noten := make([]uint8, 0, len(haeufigkeit))
	for n := range haeufigkeit {
		noten = append(noten, n)
	}
	slices.Sort(noten) // Maps haben in Go absichtlich keine feste Reihenfolge
	fmt.Print("Notenverteilung (Note, Anzahl): [")
	for i, n := range noten {
		if i > 0 {
			fmt.Print(", ")
		}
		fmt.Printf("(%d, %d)", n, haeufigkeit[n])
	}
	fmt.Println("]")
}

func main() {
	fmt.Println("== 1-4. Structs, Methoden, Fehler, Interfaces ==")
	anna := NeuerSchueler("Anna")
	ben := NeuerSchueler("Ben")
	cem := NeuerSchueler("Cem")

	for _, n := range []int{1, 2, 1} {
		if err := anna.NoteEintragen(n); err != nil {
			panic(err) // Rust: .expect(...)
		}
	}
	for _, n := range []int{3, 4, 5} {
		if err := ben.NoteEintragen(n); err != nil {
			panic(err)
		}
	}

	if err := ben.NoteEintragen(9); err != nil {
		fmt.Printf("Fehler abgefangen: %v\n", err)
	}

	var bewertbar []Bewertung = []Bewertung{anna, ben, cem}
	for _, b := range bewertbar {
		fmt.Println(b.Urteil())
	}
	fmt.Printf("Debug-Ausgabe: %+v\n", *anna)

	fmt.Println("\n== 5. Fehlerwerte ==")
	for _, eingabe := range []string{"2", " 5 ", "sieben", "0"} {
		n, err := noteAusText(eingabe)
		var kt KeinText
		switch {
		case err == nil:
			fmt.Printf("'%s' -> Note %d\n", eingabe, n)
		case errors.As(err, &kt): // nach dem konkreten Fehlertyp fragen
			fmt.Printf("'%s' -> Fehler vom Typ KeinText: %v\n", eingabe, err)
		default:
			fmt.Printf("'%s' -> Fehler: %v\n", eingabe, err)
		}
	}

	ownershipDemo()

	klasse := []*Schueler{anna, ben, cem} // Zeiger: kein Move, alle bleiben benutzbar
	iteratorDemo(klasse)
}
