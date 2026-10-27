[< HOME](../../README.it.md)
# 🔧 Creazione di un nuovo progetto con AVR-HAL

Questa guida spiega come creare un nuovo progetto in **Rust** per **Arduino Uno R3 (ATMega328P)**, utilizzando l'eccellente ecosistema **avr-hal** mantenuto da Rahix.

L'utilizzo del template `avr-hal` fornisce un ambiente di sviluppo già configurato, consentendoti di concentrarti sullo sviluppo del firmware anziché sulla configurazione del progetto.

---

## Cos'è AVR-HAL?

L'**Hardware Abstraction Layer (HAL)** offre un'API di livello superiore che semplifica l'accesso alle periferiche del microcontrollore, rispetto all'interazione diretta con un PAC (Peripheral Access Crate).

Vantaggi dell'utilizzo di un HAL:

- Configurazione delle periferiche più semplice
- Astrazioni più sicure
- Sviluppo più rapido
- Ottimo per imparare a programmare in Rust in ambito embedded

---

## Installazione degli strumenti necessari

Installa gli strumenti utilizzati per generare i progetti e caricare il firmware:

```bash
cargo install cargo-generate
cargo install ravedude
```

Verifica l'installazione:

```bash
cargo generate --version
ravedude --version
```

---

## Creazione di un nuovo progetto

Genera un nuovo progetto partendo dal template ufficiale di AVR-HAL:

```bash
cargo generate --git https://github.com/Rahix/avr-hal-template.git
```

Durante la creazione del progetto:

1. Inserisci il nome del progetto.
2. Seleziona **arduino-uno** come scheda di destinazione (target board).
3. Spostati nella directory appena creata.

```bash
cd <nome-progetto>
```

4. Compila il progetto.

```bash
cargo build
```

---

## Struttura del progetto

Il progetto generato include già:

- Configurazione della scheda
- Supporto per il target AVR
- Utility per il caricamento del firmware
- Esempio di firmware
- Dipendenze per lo sviluppo

Puoi iniziare subito a modificare il codice sorgente. ---

## Compilazione e caricamento del firmware

Una volta creato il progetto, segui le istruzioni contenute in:

- 📖 [Compilazione e caricamento del firmware](./CompileAndInstall.md)

---

## Risorse ufficiali

- AVR-HAL: https://github.com/Rahix/avr-hal
- Template AVR-HAL: https://github.com/Rahix/avr-hal-template
