// Speedtest: vier gleiche Aufgaben in Rust, Python und Go.
//
// Starten: go run speedtest.go   (oder: go build speedtest.go && ./speedtest)
// Jede Aufgabe gibt eine Prüfsumme aus. Die muss in allen drei Sprachen gleich sein.
package main

import (
	"fmt"
	"runtime"
	"slices"
	"time"
)

const (
	fibN       = 32
	siebGrenze = 10_000_000
	sortN      = 2_000_000
	mapN       = 5_000_000
)

// Derselbe Zufallsgenerator wie in Rust und Python.
// uint64 läuft in Go einfach über, so wie Rusts wrapping_mul.
type lcg struct{ x uint64 }

func (r *lcg) next() uint64 {
	r.x = r.x*6364136223846793005 + 1442695040888963407
	return r.x >> 33
}

// 1. Rekursion: misst Funktionsaufrufe.
func fib(n uint32) uint64 {
	if n < 2 {
		return uint64(n)
	}
	return fib(n-1) + fib(n-2)
}

// 2. Sieb des Eratosthenes: misst Schleifen und Speicherzugriffe.
func sieb(grenze int) int {
	istPrim := make([]bool, grenze)
	for i := 2; i < grenze; i++ {
		istPrim[i] = true
	}
	for i := 2; i*i < grenze; i++ {
		if istPrim[i] {
			for j := i * i; j < grenze; j += i {
				istPrim[j] = false
			}
		}
	}
	anzahl := 0
	for _, p := range istPrim {
		if p {
			anzahl++
		}
	}
	return anzahl
}

// 3. Zufallszahlen erzeugen und sortieren.
func sortieren(n int) [3]uint64 {
	rng := lcg{42}
	v := make([]uint64, n)
	for i := range v {
		v[i] = rng.next()
	}
	slices.Sort(v)
	return [3]uint64{v[0], v[n/2], v[n-1]}
}

// 4. map: zählen, wie oft jeder von 100.000 Schlüsseln vorkommt.
func zaehlen(n int) [2]int {
	rng := lcg{7}
	anzahl := map[uint64]int{}
	for i := 0; i < n; i++ {
		anzahl[rng.next()%100_000]++
	}
	groesste := 0
	for _, c := range anzahl {
		groesste = max(groesste, c)
	}
	return [2]int{len(anzahl), groesste}
}

func messen(name string, aufgabe func() any) float64 {
	start := time.Now()
	ergebnis := aufgabe()
	ms := float64(time.Since(start).Microseconds()) / 1000
	fmt.Printf("%-22s %10.1f ms   Prüfsumme: %v\n", name, ms, ergebnis)
	return ms
}

func main() {
	fmt.Println("Go", runtime.Version())
	gesamt := messen("1. fib(32) rekursiv", func() any { return fib(fibN) }) +
		messen("2. Primzahlsieb 10 Mio", func() any { return sieb(siebGrenze) }) +
		messen("3. 2 Mio sortieren", func() any { return sortieren(sortN) }) +
		messen("4. HashMap 5 Mio", func() any { return zaehlen(mapN) })
	fmt.Printf("%-22s %10.1f ms\n", "GESAMT", gesamt)
}
