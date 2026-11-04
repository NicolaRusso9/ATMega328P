# 🦀 Progetti Rust per ATMega328P

 🇮🇹 Italiano | [🇬🇧 English](./README.md)

> Raccolta di progetti in Rust 🦀 per Arduino Uno R3 (ATMega328P) 📟

---

## Panoramica

Questo repository contiene diversi progetti in **Rust** pensati per la piattaforma **Arduino Uno R3 (ATMega328P)**.
> ⚠️ **Nota:** Questo progetto è un percorso personale di studio. Sto imparando le dinamiche dell'embedded in Rust passo dopo passo e ritengo utile condividere ciò che imparo. Critiche **costruttive** e **suggerimenti per migliorare** sono estremamente graditi!

L'obiettivo è offrire un ambiente organizzato per imparare e sperimentare lo sviluppo di sistemi embedded in Rust, utilizzando diversi livelli di astrazione:

- **HAL (Hardware Abstraction Layer)**: per uno sviluppo più semplice e portabile.
- **PAC (Peripheral Access Crate)**: per l'accesso diretto alle periferiche.
- **Bare Metal**: programmazione a basso livello per un controllo totale sui registri hardware, senza strati di astrazione.

L'esempio **Blink LED** introduce i tre diversi approcci disponibili per lo sviluppo bare-metal. Una volta coperti questi concetti, non verrà più utilizzato lo sviluppo con HAL in quanto personalmente ritengo che nasconda fin troppo il funzionamento dell'hardware non rendendo chiaro come in realtà funziona.

---

## 🚀 Per iniziare

Prima di compilare o eseguire un progetto, assicurati che il tuo ambiente di sviluppo sia configurato correttamente.

### Documentazione

- 📋 [Requisiti di sistema](./__docs/it/Required.md)
- 🔧 [Creare un nuovo progetto con HAL](./__docs/it/HAL-Project.md)
- ⚙️ [Creare un nuovo progetto con PAC](./__docs/it/PAC-Project.md)

---

## 📦 Lavorare con i progetti

Puoi:

- Eseguire i progetti già esistenti.
- Aggiungere un nuovo progetto creando un nuovo crate Rust all'interno del repository.
- Sperimentare diversi approcci di sviluppo (HAL, PAC o Bare Metal). ---

## 🔨 Compilazione e caricamento (Build & Flash)

Per semplificare il processo di compilazione e caricamento del firmware, il repository include il seguente script di supporto:

```bash
./build_and_install.sh
```

Se hai bisogno di modificare la scheda di destinazione, la configurazione dell'MCU o le impostazioni del compilatore, puoi aggiornare il file:

```text
atmega328p.json
```

---

## 📚 Compilazione e caricamento manuali

Se preferisci non utilizzare lo script automatizzato, segui le istruzioni fornite in:

- 🚀 [Guida alla compilazione e all'installazione](./__docs/it/CompileAndInstall.md)

---

## 📂 Struttura del repository

```text
ATMega328P/
├── __docs/
│   ├── en/
│   └── it/
├── project_1/
├── project_2/
├── ....
├── build_and_install.sh
├── atmega328p.json
└── README.md
```
