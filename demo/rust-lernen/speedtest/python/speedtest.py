"""Speedtest: vier gleiche Aufgaben in Rust, Python und Go.

Starten: python speedtest.py
Jede Aufgabe gibt eine Prüfsumme aus. Die muss in allen drei Sprachen gleich sein.
Absichtlich in reinem Python geschrieben (Schleifen statt numpy), damit man
die Sprache selbst misst und nicht eine C-Bibliothek.
"""

import time
from collections.abc import Callable

FIB_N = 32
SIEB_GRENZE = 10_000_000
SORT_N = 2_000_000
MAP_N = 5_000_000

MASKE = (1 << 64) - 1  # Python-Zahlen laufen nie über, also schneiden wir selbst auf 64 Bit ab


class Lcg:
    """Derselbe Zufallsgenerator wie in Rust und Go."""

    def __init__(self, seed: int):
        self.x = seed

    def next(self) -> int:
        self.x = (self.x * 6364136223846793005 + 1442695040888963407) & MASKE
        return self.x >> 33


def fib(n: int) -> int:
    """1. Rekursion: misst Funktionsaufrufe."""
    return n if n < 2 else fib(n - 1) + fib(n - 2)


def sieb(grenze: int) -> int:
    """2. Sieb des Eratosthenes: misst Schleifen und Speicherzugriffe."""
    ist_prim = bytearray([1]) * grenze
    ist_prim[0] = ist_prim[1] = 0
    i = 2
    while i * i < grenze:
        if ist_prim[i]:
            j = i * i
            while j < grenze:
                ist_prim[j] = 0
                j += i
            # Pythonisch (und ~50x schneller, weil es in C läuft) wäre:
            # ist_prim[i*i::i] = bytes(len(range(i*i, grenze, i)))
        i += 1
    return sum(ist_prim)


def sortieren(n: int) -> tuple[int, int, int]:
    """3. Zufallszahlen erzeugen und sortieren.

    Achtung: list.sort() ist in C geschrieben. Langsam ist hier vor allem das
    Erzeugen der Zahlen in der Python-Schleife.
    """
    rng = Lcg(42)
    v = [rng.next() for _ in range(n)]
    v.sort()
    return (v[0], v[n // 2], v[n - 1])


def zaehlen(n: int) -> tuple[int, int]:
    """4. dict: zählen, wie oft jeder von 100.000 Schlüsseln vorkommt."""
    rng = Lcg(7)
    anzahl: dict[int, int] = {}
    for _ in range(n):
        k = rng.next() % 100_000
        anzahl[k] = anzahl.get(k, 0) + 1
    return (len(anzahl), max(anzahl.values()))


def messen(name: str, aufgabe: Callable[[], object]) -> float:
    start = time.perf_counter()
    ergebnis = aufgabe()
    ms = (time.perf_counter() - start) * 1000
    print(f"{name:<22} {ms:>10.1f} ms   Prüfsumme: {ergebnis}")
    return ms


def main() -> None:
    import sys

    print(f"Python {sys.version.split()[0]}")
    gesamt = (
        messen("1. fib(32) rekursiv", lambda: fib(FIB_N))
        + messen("2. Primzahlsieb 10 Mio", lambda: sieb(SIEB_GRENZE))
        + messen("3. 2 Mio sortieren", lambda: sortieren(SORT_N))
        + messen("4. HashMap 5 Mio", lambda: zaehlen(MAP_N))
    )
    print(f"{'GESAMT':<22} {gesamt:>10.1f} ms")


if __name__ == "__main__":
    main()
