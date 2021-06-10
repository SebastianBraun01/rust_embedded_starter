# Variablen

Variablen in Rust werden anders als in C mit dem `let` Schlüsselwort deklariert. Obwohl auch in Rust jede Variable einen bestimmten Datentyp hat, muss dieser nicht zwangsläufig angegeben werden, wenn dieser aus dem Kontext klar wird.
Eine explizite Angabe eines Datentyps erfolgt nach dem `let` Schlüsselwort und einem Doppelpunkt.

```rust
let info = true; // Typ: bool
let i: i32 = 1;  // Typ: i32 (int)
```

Einen allgemeinen Ganzzahl-Datentyp wie `int` gibt es in Rust nicht. Stattdessen muss ähnlich wie bei Typen aus dem `stdint.h`-Header immer angegeben werden, welche Größe in Bits der Datentyp belegt und ob er vorzeichenbehaftet ist oder nicht. Auch Fließkomma-Datentypen werden mit ihrer Speichergröße angegeben.

```rust
let counter: u16; // entspricht einem uint16_t
let average: f64; // entspricht einem double
```

Anders als bei C sind Variablen in Rust standardmäßig nicht veränderlich, sondern verhalten sich eher wie mit `const` markierte Variablen in C. Veränderbare (engl. mutable, kurz mut) Variablen müssen explizit mit `mut` markiert werden. Außerdem müssen alle Variablen initialisiert werden, bevor sie ausgelesen werden. Die Initialisierung muss hierbei nicht unbedingt bei der Deklaration erfolgen, vielmehr prüft der Rust-Compiler alle Verzweigungen im Programmfluss auf uninitialisierte Zugriffe.

```rust
let i = 1;
i = 2;         // Error: i ist nicht veränderbar
let mut j = 1;
j = 2;         // Erlaubt, da j veränderlich
```

# Kontrollstrukturen

## If-Abfragen

Wie C erlaubt auch Rust, Code mit `if`-Abfragen zu verzweigen. Weitere Bedingungen können mit `else if` hinzugefügt werden und im Fall, dass keine Bedingung zutrifft, kann ein `else`-Block ausgeführt werden. In Rust müssen allerdings Bedingungen immer den den Datentyp `bool` zurückgeben, so dass z. B. eine Ganzzahl mit Wert 0 nicht implizit als `false` gewertet wird.
Typumwandlungen werden in Rust generell niemals implizit durchgeführt. Um einen Variable vom Typ `u32` mit einer Variable vom Typ `i32` zu vergleichen muss also einer der Datentypen in den anderen umgewandelt werden. 

Außerdem sind Klammern um Bedingungen optional und werden in der Regel nicht verwendet. Zudem können fast alle Kontrollstrukturen - wie der Auswahloperator in C - Werte zurückgeben.

```rust
let a = 10;

let text = if a % 2 == 0 {
	"even"
} else {
	"odd"
};
```

## loop-Schleifen

Dauerschleifen können in Rust einfach mit dem Schlüsselwort `loop` definiert werden. Diese können nur durch mit dem `break` Schlüsselwort wieder verlassen werden, mit dem auch gleichzeitig Werte zurückzugeben werden können.

```rust
let mut i = 1;
let value = loop {
	if i > 32 {
		break i - 1; // Wert mit dem value initialisiert wird
	}
	i = i * 2;
};
```

# Funktionen