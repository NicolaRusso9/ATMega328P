[< HOME](../../README.it.md)
# 📋 Requisiti di sistema

Questa pagina descrive tutte le dipendenze necessarie e i passaggi di configurazione richiesti per compilare, caricare (flash) ed eseguire il debug di applicazioni Rust per **Arduino Uno R3 (ATMega328P)**.

---

# 1. Installazione della toolchain AVR

Aggiorna l'indice dei pacchetti e installa gli strumenti di sviluppo AVR necessari:

```bash
sudo apt update

sudo apt install \
gcc-avr \
avr-libc \
binutils-avr \
avrdude \
make \
pkg-config \
libudev-dev
```

---

# 2. Verifica dell'installazione

Assicurati che la toolchain sia stata installata correttamente:

```bash
avr-gcc --version
avr-objcopy --version
avrdude -?
```

Se tutti i comandi restituiscono le informazioni sulla versione, la toolchain AVR è pronta all'uso.

---

# 3. Installazione della toolchain Rust "nightly"

Lo sviluppo per AVR richiede attualmente la toolchain Rust "nightly".

Installala con il comando:

```bash
rustup toolchain install nightly
```

---

## Opzione A: Utilizzo di "nightly" a livello globale

Imposta "nightly" come toolchain Rust predefinita per il tuo sistema:

```bash
rustup default nightly
```

---

## Opzione B: Utilizzo di "nightly" solo per questo repository

Spostati nella directory del tuo progetto e configura un override locale:

```bash
rustup override set nightly
```

Questo approccio è consigliato se lavori anche a progetti che utilizzano la toolchain "stable".

---

# 4. Installazione del componente "rust-src"

Installa il codice sorgente di Rust per la toolchain attiva:

```bash
rustup component add rust-src
```

Questo componente è necessario per compilare le librerie standard di Rust per target AVR personalizzati.

---

# 5. Verifica del rilevamento della scheda

Collega l'Arduino Uno e verifica che Linux rilevi la scheda.

## Controllo dei dispositivi seriali

```bash
ls /dev/ttyACM*
```

Output previsto:

```text
/dev/ttyACM0
```

A seconda dell'hardware in uso, il numero del dispositivo potrebbe variare. ---

## Alternativa: Verifica tramite `lsusb`

```bash
lsusb
```

Esempio di output:

```text
Bus 001 Device 001: ID 1d6b:0002 Linux Foundation 2.0 root hub
Bus 001 Device 006: ID 04d9:a088 Holtek Semiconductor, Inc. USB Gaming Mouse
Bus 001 Device 008: ID 0c45:7603 Microdia USB Keyboard
Bus 001 Device 009: ID 0b05:1996 ASUSTek Computer, Inc. USB Audio
Bus 001 Device 010: ID 2341:0043 Arduino SA Uno R3 (CDC ACM)
Bus 002 Device 001: ID 1d6b:0003 Linux Foundation 3.0 root hub
```

Se la voce relativa ad Arduino è visibile, significa che la scheda è stata rilevata correttamente.

---

# 6. Abilitazione dell'accesso alla porta seriale

La maggior parte delle distribuzioni Linux limita l'accesso ai dispositivi seriali agli utenti non privilegiati.

Aggiungi il tuo account utente al gruppo `dialout`:

```bash
sudo usermod -aG dialout $USER
```

> ⚠️ Dopo aver eseguito questo comando, è necessario disconnettersi e riconnettersi (oppure riavviare il sistema) affinché la modifica abbia effetto.

---

## Spiegazione del comando

| Opzione   | Descrizione                                                                                  |
|-----------|----------------------------------------------------------------------------------------------|
| `sudo`    | Esegue il comando con privilegi amministrativi                                               |
| `usermod` | Modifica un account utente                                                                   |
| `-aG`     | Aggiunge l'utente a un gruppo supplementare senza rimuoverlo dai gruppi a cui appartiene già |
| `dialout` | Gruppo Linux che consente l'accesso ai dispositivi seriali                                   |
| `$USER`   | Utente attualmente connesso al sistema                                                       | ---

# ✅ Lista di controllo per la verifica

Prima di procedere, assicurati che:

- [ ] La toolchain AVR sia installata
- [ ] `avr-gcc` sia disponibile
- [ ] `avr-objcopy` sia disponibile
- [ ] `avrdude` sia disponibile
- [ ] La versione "nightly" di Rust sia installata
- [ ] Il componente `rust-src` sia installato
- [ ] L'Arduino Uno venga rilevato come `/dev/ttyACM*`
- [ ] Il tuo utente appartenga al gruppo `dialout`

Una volta soddisfatti tutti i requisiti, sarai pronto per compilare e caricare il firmware sull'ATMega328P.