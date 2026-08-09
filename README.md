# 🦀 Rust Fundamentals — Guida Completa e Note di Studio

Benvenuto nel repository **Rust Fundamentals**! Questo repository rappresenta un percorso pratico e teorico per l'apprendimento delle basi del linguaggio di programmazione **Rust**, strutturato in base ai capitoli della guida ufficiale [*The Rust Programming Language* (The Rust Book)](https://doc.rust-lang.org/book/).

Questo README è stato progettato come un **manuale di ripasso completo**: leggendo questa guida potrai ripassare tutta la teoria, le regole del compilatore, la memoria e la sintassi prima di esplorare direttamente il codice nelle singole cartelle.

---

## 📂 Struttura del Repository e Indice dei Moduli

Tutti i collegamenti ai file del repository sono relativi per facilitare la navigazione sia da GitHub che in ambiente locale.

| Modulo / Cartella | Sorgente Principale | Descrizione Sintetica | Rif. Book |
| :--- | :--- | :--- | :--- |
| **01. Hello World** | [`./hello_world/main.rs`](./hello_world/main.rs) | Il primo programma Rust compilato direttamente con `rustc`. | Cap. 1.1 |
| **02. Hello Cargo** | [`./hello_cargo/src/main.rs`](./hello_cargo/src/main.rs) | Struttura dei progetti Cargo, comandi `build`, `run`, `check`. | Cap. 1.3 |
| **03. Guessing Game** | [`./guessing_game/src/main.rs`](./guessing_game/src/main.rs) | Input/Output, crate esterne (`rand`), parsing e pattern matching `match`. | Cap. 2 |
| **04. Variables & Mutability** | [`./variables/src/main.rs`](./variables/src/main.rs) | Immutabilità, `mut`, costanti `const` e il concetto di **Shadowing**. | Cap. 3.1 |
| **05. Data Types** | [`./data_types/src/main.rs`](./data_types/src/main.rs) | Tipi scalari (integers, float, bool, char) e composti (tuple, array). | Cap. 3.2 |
| **06. Functions** | [`./functions/src/main.rs`](./functions/src/main.rs) | Funzioni, argomenti, return ed **Expressions vs Statements**. | Cap. 3.3 |
| **07. Control Flow** | [`./control_flow/src/main.rs`](./control_flow/src/main.rs) | Condizionali `if`, cicli `loop` (con `break` con valore ed etichette `'label`), `while`, `for`. | Cap. 3.5 |
| **08. Exercises: Temperature** | [`./exercises/temperature-f-c/src/main.rs`](./exercises/temperature-f-c/src/main.rs) | Convertitore di temperatura $°C \leftrightarrow °F$ a menu interattivo. | Cap. 3 (Es.) |
| **09. Exercises: Fibonacci** | [`./exercises/fibonacci_series/src/main.rs`](./exercises/fibonacci_series/src/main.rs) | Calcolo dell'n-esimo numero della sequenza di Fibonacci. | Cap. 3 (Es.) |
| **10. Ownership** | [`./ownership/src/main.rs`](./ownership/src/main.rs) | **Ownership**, gestione memoria Stack/Heap, `String`, Move semantics e Clone. | Cap. 4.1 |
| **11. References & Borrowing** | [`./references_borrowing/src/main.rs`](./references_borrowing/src/main.rs) | Prestito dati: `&T` (immutabili), `&mut T` (mutabili), prevenzione Data Race e Dangling References. | Cap. 4.2 |
| **12. Slices** | [`./slice/src/main.rs`](./slice/src/main.rs) | Le Slice (`&str`, `&[T]`): viste trasversali contigue senza possesso. | Cap. 4.3 |
| **13. Structs** | [`./structs/src/main.rs`](./structs/src/main.rs) | Tipi di dato personalizzati: Struct con campi, Tuple Structs, Unit-like Structs, Update Syntax. | Cap. 5.1 |
| **14. Rectangles** | [`./rectangles/src/main.rs`](./rectangles/src/main.rs) | Rifattorizzazione con struct, `#[derive(Debug)]`, formattazione `{:#?}` e macro `dbg!`. | Cap. 5.2 |
| **15. Methods** | [`./methods/src/main.rs`](./methods/src/main.rs) | Sintassi dei metodi `impl`, il parametro `&self` e le **Funzioni Associate** (es. costruttori). | Cap. 5.3 |
| **16. Enums (WIP)** | [`./enums/src/main.rs`](./enums/src/main.rs) | **(Work In Progress)** Enumerazioni e associazione dati ai varianti *(Commit `2f481176ddb9aa51b9ac088f19f56cadd6ef1acd`)*. | Cap. 6 |
| **17. Cuore ASCII** | [`./cuore/src/main.rs`](./cuore/src/main.rs) | Algoritmo di grafica console in ASCII Art mediante cicli `for` nidificati. | Extra |

---

## 📖 Trattazione Teorica Dettagliata per il Ripasso

---

### 1. Primo Impatto: `hello_world` e `hello_cargo`
**Cartelle**: [`./hello_world/`](./hello_world/main.rs) | [`./hello_cargo/`](./hello_cargo/src/main.rs)

#### Concetti Teorici:
- **Compilazione diretta (`rustc`)**: In Rust è possibile compilare un file singolo con `rustc main.rs`. Viene generato un eseguibile binario nativo (senza virtual machine né runtime interpretato).
- **Cargo**: È il build system e package manager ufficiale di Rust. Gestisce dipendenze, compilazione, testing e documentazione.
- **File di configurazione `Cargo.toml`**: Utilizza la sintassi TOML per definire il pacchetto (`[package]`) e le dipendenze (`[dependencies]`).
- **File di lock `Cargo.lock`**: Mantiene l'esatta versione deterministica delle dipendenze per garantire build riproducibili.

#### Comandi Cargo fondamentali:
```bash
cargo new <nome_progetto>  # Crea una nuova struttura di progetto con Cargo
cargo build                # Compila il progetto e crea l'eseguibile in target/debug/
cargo run                  # Compila ed esegue il progetto in un solo passaggio
cargo check                # Verifica la correttezza del codice SENZA produrre il binario (molto veloce)
cargo build --release      # Compila con ottimizzazioni per la produzione (target/release/)
```

---

### 2. Progetto Pratico: `guessing_game`
**Cartella**: [`./guessing_game/`](./guessing_game/src/main.rs)

#### Concetti Teorici:
1. **Punto d'ingresso e macro `println!`**: Il punto di partenza di ogni programma eseguibile è la funzione `fn main()`. Le funzioni che terminano con `!` (come `println!`) sono **macro**, non funzioni ordinarie.
2. **Libreria Standard `std::io`**: Fornisce strumenti per l'I/O. `io::stdin().read_line(&mut guess)` legge l'input da tastiera e lo salva in una stringa mutabile passata per riferimento mutabile (`&mut`).
3. **Gestione dei Risultati (`Result` enum)**: `read_line` restituisce un tipo `std::io::Result` che può essere `Ok` o `Err`. Chiamare `.expect("Messaggio")` interrompe il programma (*panic*) se il risultato è un errore.
4. **Crate esterne**: Nel file `Cargo.toml` viene aggiunta la dipendenza `rand = "0.8.5"`. Viene usata la funzione `rand::thread_rng().gen_range(1..=100)` per generare un numero casuale nell'intervallo compreso $[1, 100]$.
5. **Parsing del tipo e Shadowing**:
   ```rust
   let guess: u32 = match guess.trim().parse() {
       Ok(num) => num,
       Err(_) => continue,
   };
   ```
   Con `trim()` si rimuovono gli spazi e l'a-capo (`\n`). Con `parse()` la stringa viene convertita in `u32`. Utilizzando `match` sul tipo `Result` restituito da `parse()`, se l'utente inserisce un valore non valido il ciclo prosegue con `continue` senza far crasciare il gioco.
6. **Pattern Matching con `match` e `Ordering`**:
   `guess.cmp(&secret_number)` restituisce l'enum `std::cmp::Ordering` (`Less`, `Greater`, `Equal`). Il costrutto `match` valuta tutti i possibili casi ed esegue il blocco corrispondente.

---

### 3. Variabili e Mutabilità: `variables`
**Cartella**: [`./variables/`](./variables/src/main.rs)

#### Concetti Teorici:
- **Immutabilità di Default**: In Rust, le variabili create con `let` **non possono essere modificate**. Questo previene mutazioni accidentali dello stato.
  ```rust
  let x = 5;
  // x = 6; // ERRORE DI COMPILAZIONE! Impossibile riassegnare a variabile immutabile.
  ```
- **Mutabilità Esplicita (`mut`)**: Per consentire la modifica di un valore occorre usare `mut`:
  ```rust
  let mut mutable_variable = 10;
  mutable_variable = 15; // Valido!
  ```
- **Costanti (`const`)**:
  - Si dichiarano con `const` (es. `const MAX_IQ: u32 = 200;`).
  - **Devono** sempre avere un tipo di dato esplicitato.
  - Possono essere impostate solo su espressioni valutabili a tempo di compilazione (non su risultati di chiamate a runtime).
  - Sono valide per tutta la durata del programma in qualsiasi scope.
- **Shadowing (Offuscamento)**:
  Rust permette di dichiarare una nuova variabile con lo stesso nome di una variabile precedente nello stesso o in un nuovo scope:
  ```rust
  let x = 5;
  let x = x + 1; // x diventa 6 (nuova variabile che nasconde la precedente)
  {
      let x = x * 2; // x diventa 12 in questo blocco interno
      println!("X interno: {x}");
  }
  println!("X esterno: {x}"); // x torna a essere 6!
  ```
  *Vantaggio dello Shadowing rispetto a `mut`*: È possibile cambiare anche il **tipo** della variabile mantenendo lo stesso nome (es. `let spaces = "   "; let spaces = spaces.len();`).

---

### 4. Tipi di Dati: `data_types`
**Cartella**: [`./data_types/`](./data_types/src/main.rs)

Rust è un linguaggio a **tipizzazione statica forte**. Ogni valore ha un tipo ben definito a tempo di compilazione.

#### A. Tipi Scalari (Un singolo valore):
1. **Interi (Integers)**:
   - Con segno (*signed*, `i8`, `i16`, `i32`, `i64`, `i128`, `isize`). Possono essere positivi o negativi.
   - Senza segno (*unsigned*, `u8`, `u16`, `u32`, `u64`, `u128`, `usize`). Solo positivi.
   - `isize` e `usize` dipendono dall'architettura del computer (32 o 64 bit) e sono usati principalmente per indicizzare le collezioni.
   - *Default*: `i32`.
   - *Sintassi letterali*: Si può usare l'underscore per la leggibilità (es. `4_000_000`), prefissi esadecimali `0x`, ottali `0o`, binari `0b`, e byte `b'A'`.
2. **Floating-Point (Numeri con virgola)**:
   - `f32` (singola precisione) e `f64` (doppia precisione, *default*).
3. **Booleani**:
   - `bool` con valori `true` o `false` (dimensione: 1 byte).
4. **Caratteri (`char`)**:
   - Rappresentano un carattere Unicode a **4 byte** (es. `let c = 'z';`, `let z: char = 'ℤ';`). Racchiusi tra apici singoli `'`.

#### B. Tipi Composti (Raggruppano più valori):
1. **Tuple**:
   - Raggruppano valori di **tipi diversi** in un singolo tipo a lunghezza fissa.
   ```rust
   let private_info: (&'static str, i32, &'static str) = ("Alessia", 2008, "Roma");
   // Destrutturazione (Pattern Matching):
   let (name, year, city) = private_info;
   // Accesso diretto mediante indice numerico:
   let name = private_info.0;
   let year = private_info.1;
   ```
   - **Unit (`()`)**: La tupla vuota `()` rappresenta un valore vuoto o un tipo di ritorno nullo.
2. **Array**:
   - Raggruppano valori dello **stesso tipo** a lunghezza fissa salvati nello **Stack**.
   ```rust
   let marks = [7, 4, 9, 6, 5];
   let keke: [f64; 3] = [58.54, 98.23, 57.73]; // Tipo [f64; 3] -> 3 elementi f64
   let ekek = [10; 5]; // Crea un array di 5 elementi tutti uguali a 10: [10, 10, 10, 10, 10]
   ```
   - L'accesso agli elementi avviene con le quadre (`ekek[0]`). Se si tenta di accedere a un indice fuori dai limiti, Rust genera un *panic* a runtime impedendo accessi illegali alla memoria (buffer overflow).

---

### 5. Funzioni ed Espressioni: `functions`
**Cartella**: [`./functions/`](./functions/src/main.rs)

#### Concetti Teorici:
- **Dichiarazione**: Si usa la sintassi `fn nome_funzione(parametro: Tipo) -> TipoRitorno`.
- **Convenzione**: Nomi in *snake_case* (lettere minuscole e underscore).
- **Statements (Istruzioni) vs Expressions (Espressioni)**:
  - **Statements**: Istruzioni che compiono un'azione ma **non restituiscono alcun valore**. Terminano con il punto e virgola `;`.
    - Esempio: `let x = 6;` è uno statement. Non si può fare `let x = (let y = 6);` in Rust.
  - **Expressions**: Blocchi di codice che **valutano e restituiscono un valore**. Non terminano con il punto e virgola.
    ```rust
    let x = {
        let y = 0;
        y + 1 * 5 // NIENTE punto e virgola! Questa è un'espressione che restituisce 5.
    };
    ```
    *Nota*: Se si aggiunge un punto e virgola a `y + 1 * 5;`, essa diventa uno statement e restituisce il tipo unit `()`.

---

### 6. Controllo del Flusso: `control_flow`
**Cartella**: [`./control_flow/`](./control_flow/src/main.rs)

#### Concetti Teorici:
1. **Diramazione `if / else if / else`**:
   - La condizione in un `if` deve essere **strettamente booleana**. Rust non effettua conversione implicita da interi a booleani (a differenza di C o JavaScript).
   - **`if` come espressione**: Poiché `if` è un'espressione, può essere assegnato direttamente a una variabile:
     ```rust
     let mut y = if x > 0 { 100 } else { -100 };
     ```
     *Regola*: I rami del `if` e dell'`else` devono restituire lo stesso tipo di dato!
2. **Ciclo `loop` e ritorno di valori**:
   - `loop` esegue un blocco di codice all'infinito finché non incontra `break`.
   - Si può restituire un valore da un `loop` passandolo dopo `break`:
     ```rust
     let tax = loop {
         y -= 1;
         if y < 50 {
             break y * 5 + y * 4; // Il valore calcolato viene restituito a 'tax'
         }
     };
     ```
3. **Etichette dei Cicli (`'label`)**:
   - In presenza di cicli annidati, è possibile assegnare un'etichetta al ciclo esterno per interromperlo direttamente dall'interno:
     ```rust
     'out: loop {
         loop {
             if counter == 3 {
                 break 'out; // Interrompe il ciclo esterno identificato dall'etichetta 'out
             }
         }
     }
     ```
4. **Ciclo `while`**: Esegue il codice finché una condizione rimane vera.
5. **Ciclo `for`**: Utilizzato per iterare in modo sicuro e performante su collezioni o range di numeri:
   ```rust
   let marks = [8, 6, 4, 9, 7];
   for mark in marks { println!("{mark}"); }

   // Range da 1 a 10 compreso, al contrario:
   for n in (1..=10).rev() { println!("{n}"); }
   ```

---

### 7. Esercizi Pratici: `exercises`
**Cartella**: [`./exercises/`](./exercises/)

- [**`temperature-f-c`**](./exercises/temperature-f-c/src/main.rs): Utilizza un ciclo `loop` con `match` su stringa per creare un menu interattivo da riga di comando. Applica la formula di conversione tra $°C$ e $°F$ incapsulando il parsing dell'input all'interno di funzioni dedicate (`from_c_to_f` e `from_f_to_c`).
- [**`fibonacci_series`**](./exercises/fibonacci_series/src/main.rs): Calcola l'n-esimo numero della sequenza di Fibonacci ($F_n = F_{n-1} + F_{n-2}$) sfruttando variabili mutabili per tracciare lo stato corrente (`a` e `b`) e un ciclo `for` per eseguire gli scambi.

---

### 8. Il Cuore di Rust: Ownership (`ownership`)
**Cartella**: [`./ownership/`](./ownership/src/main.rs)

#### Concetti Teorici:
L'**Ownership** è il sistema gestito direttamente dal compilatore che garantisce la sicurezza della memoria senza bisogno di un Garbage Collector.

#### Le 3 Regole dell'Ownership:
1. **Ogni valore in Rust ha un proprietario (una variabile).**
2. **Può esserci solo un proprietario alla volta.**
3. **Quando il proprietario va fuori dal suo scope (`}`), il valore viene eliminato (*dropped*).**

#### Stack vs Heap:
- **Stack**: Memoria veloce con ordine LIFO (Last In, First Out). Contiene dati di dimensione fissa e nota a tempo di compilazione (es. `i32`, `bool`, `char`, array fissa).
- **Heap**: Memoria per dati di dimensione dinamica o incerta (es. `String`). Richiede un'allocazione mediante puntatore.

#### Trasferimento della Proprietà (**Move Semantics**):
```rust
let s1 = String::from("hello");
let s2 = s1; // MOVE! La proprietà dei dati nell'heap passa a s2.
// println!("{s1}"); // ERRORE DI COMPILAZIONE! s1 non è più valida.
```
*Spiegazione*: Invece di fare una copia profonda (*deep copy*) o di creare due puntatori allo stesso indirizzo (che causerebbe un errore di *double free* all'uscita dello scope), Rust rende invalida la prima variabile (`s1`). Questo passaggio prende il nome di **Move**.

#### Clonazione dei Dati (**Clone**):
Se si desidera copiare sia il puntatore che i dati contenuti nell'heap, occorre chiamare esplicitamente `.clone()`:
```rust
let s2 = String::from("ciao");
let s3 = s2.clone(); // Crea una copia indipendente nell'Heap
println!("{s2} {s3}"); // Valido!
```

#### Ownership e Funzioni:
- Passare un valore a una funzione equivale ad assegnarlo a una variabile: trasferisce l'ownership (**Move**).
- Restituire un valore da una funzione trasferisce l'ownership al chiamante.
```rust
fn lose_ownership(s: String) { println!("{s}"); } // s viene deallocata alla fine della funzione

let secret = String::from("segreto");
lose_ownership(secret);
// secret non è più usabile qui!
```

---

### 9. Prestito e Riferimenti: `references_borrowing`
**Cartella**: [`./references_borrowing/`](./references_borrowing/src/main.rs)

#### Concetti Teorici:
Passare l'ownership ad ogni funzione e restituirla continuamente è sgradevole. Per ovviare a questo problema, Rust introduce i **Riferimenti (`&`)** e il meccanismo del **Borrowing (Prestito)**.

#### Tipi di Riferimenti:
1. **Riferimento Immutabile (`&T`)**: Permette di leggere i dati senza modificarli e senza prenderne la proprietà.
   ```rust
   fn calculate_length(s: &String) -> usize {
       s.len() // s viene solo preso in prestito
   }
   ```
2. **Riferimento Mutabile (`&mut T`)**: Permette di modificare i dati presi in prestito.
   ```rust
   fn change(s: &mut String) {
       s.push_str(" ogni giorno");
   }
   ```

#### Le Regole Fondamentali del Borrowing:
1. **Puoi avere un qualsiasi numero di riferimenti immutabili (`&T`) contemporaneamente.**
2. **Puoi avere SOLO UN riferimento mutabile (`&mut T`) alla volta per un determinato dato in un determinato scope.**
3. **NON puoi combinare riferimenti mutabili e immutabili nello stesso scope se i riferimenti immutabili sono ancora in uso.**

```rust
let mut s1 = String::from("test");

// ERRORE: due riferimenti mutabili contemporanei!
// let sr1 = &mut s1;
// let sr2 = &mut s1;

// CORRETTO: gli scope dei riferimenti immutabili terminano dopo il loro ultimo utilizzo (NLL - Non-Lexical Lifetimes)
let sr1 = &s1;
let sr2 = &s1;
println!("{sr1} {sr2}"); // Ultimo uso di sr1 e sr2!

let sr3 = &mut s1; // Ora è possibile creare un riferimento mutabile!
println!("{sr3}");
```

#### Prevenzione delle Dangling References (Puntatori Penzolanti):
Il compilatore di Rust garantisce che un riferimento non vivrà mai più a lungo dei dati a cui punta.
```rust
// ERRORE DI COMPILAZIONE:
/*
fn dangle() -> &String {
    let s = String::from("hello");
    &s // s viene deallocata qui, &s punterebbe a memoria liberata!
}
*/

// SOLUZIONE: Restituire direttamente la String trasferendone la proprietà
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
```

---

### 10. Le Slice: `slice`
**Cartella**: [`./slice/`](./slice/src/main.rs)

#### Concetti Teorici:
Una **Slice** è un riferimento a una parte contigua di una collezione (es. una porzione di una stringa o di un array) senza averne la proprietà.

#### Slices di Stringa (`&str`):
Puntano a una sequenza di byte all'interno di una `String`:
```rust
let s = String::from("Hello World");
let hello = &s[0..5];  // Slice dai byte 0 a 4 ("Hello")
let world = &s[6..11]; // Slice dai byte 6 a 10 ("World")
```

#### Sintassi dei Range:
- `[0..2]` equivale a `[..2]`
- `[3..len]` equivale a `[3..]`
- `[0..len]` equivale a `[..]` (tutta la stringa)

#### String Literals e Parametri idiomatici:
Le stringhe letterali (es. `let s = "Ciao";`) sono esse stesse delle `&str`.
In Rust è una buona pratica definire le funzioni accettando `&str` invece di `&String`, in quanto `&str` accetta sia stringhe letterali che slice di `String` o riferimenti a `String` (tramite *Deref Coercion*):
```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
```

---

### 11. Le Struct e Debugging: `structs` & `rectangles`
**Cartelle**: [`./structs/`](./structs/src/main.rs) | [`./rectangles/`](./rectangles/src/main.rs)

#### Concetti Teorici:
Le **Struct** sono tipi di dati personalizzati che consentono di raggruppare valori di tipo diverso legati da un significato logico.

#### Sintassi delle Struct:
```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

#### Altre tipologie di Struct:
- **Tuple Structs**: Struct senza nomi di campo espliciti, usate per creare tipi distinti:
  ```rust
  struct Color(i32, i32, i32);
  struct Point(i32, i32, i32);
  ```
- **Unit-Like Structs**: Struct senza alcun campo, utili per implementare comportamenti/trait senza stato:
  ```rust
  struct AlwaysEqual;
  ```

#### Field Init Shorthand & Struct Update Syntax:
- **Field Init Shorthand**: Se il nome del parametro della funzione coincide con il campo della struct, si può omettere il valore (`username` invece di `username: username`).
- **Struct Update Syntax (`..user`)**: Per creare un nuovo oggetto riutilizzando i campi di un altro oggetto:
  ```rust
  let user4 = User {
      email: String::from("nuova@email.it"),
      ..user // Copia/Sposta i restanti campi da 'user'
  };
  ```
  *Attenzione*: Se i campi copiati non implementano `Copy` (come `username: String`), l'ownership di quel campo viene trasferita (**Move**) e l'oggetto originario (`user`) non potrà più essere usato per intero!

#### Debugging di Structs con `derive(Debug)` e `dbg!`:
Aggiungendo `#[derive(Debug)]` sopra la definizione di una struct, Rust genera automaticamente le capacità di formattazione per il debug:
- `println!("{:?}", rect);` -> Stampa su una sola linea.
- `println!("{:#?}", rect);` -> Stampa formattata con rientri (pretty-print).
- `dbg!(&rect);` -> Macro che stampa il file, la riga e il valore dell'espressione sullo standard error (`stderr`), restituendo la proprietà dell'oggetto.

---

### 12. Metodi e Funzioni Associate: `methods`
**Cartella**: [`./methods/`](./methods/src/main.rs)

#### Concetti Teorici:
I **Metodi** sono definiti all'interno di un blocco `impl` (implementazione) per un tipo di dato specifico.

#### Sintassi del parametro `self`:
Il primo parametro di un metodo è sempre una variante di `self`:
- `&self`: Prende l'istanza in prestito immutabile (accesso in lettura). É la forma più comune.
- `&mut self`: Prende l'istanza in prestito mutabile (per modificarne i campi).
- `self`: Prende la proprietà dell'istanza (raro, usato quando il metodo deve consumare/trasformare l'oggetto).

```rust
impl Rectangle {
    // Metodo: calcola l'area del rettangolo
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Metodo con parametri aggiuntivi: verifica se contiene un altro rettangolo
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Funzione Associata (Costruttore): non accetta self
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
```

#### Funzioni Associate:
Le funzioni all'interno di un blocco `impl` che **non hanno `self` come primo parametro** sono chiamate **funzioni associate**. Si invocano con la sintassi con i doppi punti `::` (es. `Rectangle::square(10)` o `String::from("hello")`).

---

### 13. Enumerazioni (Enums): `enums` *(WIP)*
**Cartella**: [`./enums/`](./enums/src/main.rs)

#### Concetti Teorici:
Gli **Enum** (Enumerazioni) permettono di definire un tipo specificando un insieme di varianti possibili.

#### Vantaggio principale degli Enum in Rust:
A differenza di altri linguaggi, in Rust le varianti di un enum possono **contenere direttamente dei dati**:

```rust
// Enum basico:
enum IpAddrKind {
    V4,
    V6,
}

// Enum con dati associati direttamente nelle varianti:
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
}
```

#### Stato Attuale della Cartella:
⚠️ **Work in Progress**: Il modulo relativo agli enum è attualmente in fase di completamento *(riferimento al commit `2f481176ddb9aa51b9ac088f19f56cadd6ef1acd`)*. Le sezioni su `Option<T>`, `match` esaustivo e `if let` verranno integrate con i successivi aggiornamenti.

---

### 14. Algoritmo Grafico ASCII Art: `cuore`
**Cartella**: [`./cuore/`](./cuore/src/main.rs)

#### Concetti Teorici:
Un'applicazione pratica dei costrutti di controllo del flusso (`for`, `if / else if / else`) e degli operatori logici.
L'algoritmo calcola le coordinate cartesiane $(x, y)$ su una matrice di $35 \times 51$ caratteri per tracciare i contorni superiori ed inferiori di una figura a cuore, stampando il carattere `*` sui bordi e uno spazio vuoto all'interno.

---

## 🛠️ Guida Rapida alla Compilazione ed Esecuzione

Dalla radice del workspace puoi eseguire un qualsiasi modulo utilizzando Cargo:

```bash
# Esecuzione diretta di un modulo specificando la cartella
cd guessing_game
cargo run

# Ritorno alla root
cd ..

# Esecuzione diretta dalla root specificando il Cargo.toml di destinazione
cargo run --manifest-path variables/Cargo.toml
cargo run --manifest-path exercises/temperature-f-c/Cargo.toml
cargo run --manifest-path ownership/Cargo.toml
cargo run --manifest-path methods/Cargo.toml
cargo run --manifest-path cuore/Cargo.toml
```

Per il progetto `hello_world` (che non usa Cargo):
```bash
cd hello_world
rustc main.rs
./main
```

---

## 🔗 Risorse di Approfondimento

- 📖 **The Rust Programming Language (The Book)**: [doc.rust-lang.org/book/](https://doc.rust-lang.org/book/)
- ⚡ **Rust by Example**: [doc.rust-lang.org/rust-by-example/](https://doc.rust-lang.org/rust-by-example/)
- 📦 **Documentazione Standard Library**: [doc.rust-lang.org/std/](https://doc.rust-lang.org/std/)
