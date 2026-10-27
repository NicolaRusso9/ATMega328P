[< HOME](../../README.it.md)
# Creazione di un nuovo progetto con PAC

Questa guida descrive la configurazione minima necessaria per sviluppare firmware in Rust per l'**ATMega328P** utilizzando direttamente il **Peripheral Access Crate (PAC)**.

A differenza di un progetto basato su HAL, lavorare con un PAC offre un accesso diretto alle periferiche e ai registri del microcontrollore, rendendolo lo strumento ideale per comprendere il funzionamento interno dell'hardware.

---

## Cos'è un PAC?

Un **Peripheral Access Crate (PAC)** è un crate Rust di basso livello, generato automaticamente a partire dal datasheet del microcontrollore.

L'utilizzo di un PAC consente di:

- Accedere direttamente ai registri hardware.
- Configurare le periferiche senza strati di astrazione.
- Scrivere firmware leggero.
- Comprendere meglio l'architettura del microcontrollore (MCU).

Se preferisci una configurazione delle periferiche più semplice e API di livello superiore, valuta l'utilizzo di un approccio basato su HAL.

---

## Prerequisiti

Assicurati di aver completato la configurazione di base del sistema:

- 📋 [Requisiti](Required.md)

---

## Creazione di un nuovo progetto

Crea un progetto Rust di tipo "binary" (eseguibile):

```bash
cargo new blink
cd blink
```

---

## Aggiunta delle dipendenze necessarie

Modifica il file `Cargo.toml`:

```toml
[dependencies]
avr-device = "0.9"
panic-halt = "1.0"

[profile.release]
panic = "abort"
lto = true
opt-level = "s"
```

### Panoramica delle dipendenze

| Dipendenza   | Scopo                                                    |
|--------------|----------------------------------------------------------|
| `avr-device` | PAC (Peripheral Access Crate) per microcontrollori AVR   |
| `panic-halt` | Interrompe l'esecuzione del programma in caso di "panic" |

---

## Verifica del supporto per il target AVR

Verifica che Rust riconosca i target AVR:

```bash
rustc --print target-list | grep avr
```

Esempio di output:

```text
avr-none
```

---

## Stack software

Un ambiente minimale basato su PAC è composto da:

- Rust Nightly
- rust-src
- avr-device (PAC)
- panic-halt
- avr-gcc
- avr-objcopy
- avrdude

---

## Strumenti non necessari

I seguenti strumenti sono comunemente utilizzati con **avr-hal**, ma non sono richiesti quando si sviluppa direttamente tramite un PAC:

```text
cargo-generate
ravedude
```

È comunque possibile installarli se si utilizzano progetti basati su HAL in altri contesti.

---

## PAC vs HAL

| Caratteristica                                   | PAC | HAL         |
|--------------------------------------------------|-----|-------------|
| Accesso diretto ai registri                      | ✅  | ❌          |
| Controllo a basso livello                        | ✅  | ⚠️ Parziale |
| Facilità d'uso                                   | ❌  | ✅          |
| Apprendimento del funzionamento interno dell'MCU | ✅  | ⚠️          |
| Sviluppo più rapido                              | ❌  | ✅          |

---

## Struttura del progetto consigliata

```text
blink/
├── src/
│   └── main.rs
├── Cargo.toml
├── atmega328p.json
└── .cargo/
└── config.toml
```

---

## Compilazione e caricamento (flash) del firmware

Una volta creato il progetto, segui le istruzioni contenute in:

- 📖 [Compila & Installa](CompileAndInstall.md)
