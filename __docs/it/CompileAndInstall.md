[< HOME](../../README.it.md)
# 🚀 Compilazione e caricamento del firmware

Questa pagina spiega come compilare e caricare il firmware su una scheda **Arduino Uno R3 (ATMega328P)**.

---

## Compilazione del progetto

### Compilazione standard (Release)

Utilizza la procedura di compilazione standard di Rust:

```bash
cargo build --release
```

---

### Compilazione tramite target AVR generico

Esempio di utilizzo del target AVR integrato:

```bash
cargo build -Z build-std=core --target avr-none
```

---

### Compilazione tramite specifica di target personalizzata

Questo repository include una definizione di target personalizzata per l'ATMega328P.

```bash
cargo build --target atmega328p.json -Zjson-target-spec
```

> **Consigliato:** questa è la configurazione di compilazione preferita per questo repository.

---

## Generazione del file HEX

Dopo aver compilato il progetto, converti l'eseguibile generato in un file Intel HEX:

```bash
avr-objcopy \
-O ihex \
target/<target>/debug/blink \
firmware.hex
```

Sostituisci:

- `<target>` con la tua directory di destinazione (target).
- `blink` con il nome del tuo eseguibile.

---

## Caricamento del firmware (Flashing)

Carica il file HEX generato sulla scheda utilizzando `avrdude`:

```bash
avrdude \
-p m328p \
-c arduino \
-P /dev/ttyACM0 \
-b 115200 \
-U flash:w:firmware.hex:i
```

### Parametri

| Parametro                   | Descrizione                            |
|-----------------------------|----------------------------------------|
| `-p m328p`                  | MCU di destinazione (ATMega328P)       |
| `-c arduino`                | Programmatore per bootloader Arduino   |
| `-P /dev/ttyACM0`           | Dispositivo seriale                    |
| `-b 115200`                 | Baud rate (velocità di trasmissione)   |
| `-U flash:w:firmware.hex:i` | Scrive il file HEX nella memoria flash |

---

## Caricamento completato con successo

Se la procedura di caricamento (flashing) termina correttamente, dovresti visualizzare il seguente messaggio:

```text
avrdude done. Thank you.
```

---

## Risoluzione dei problemi

### Permesso negato (Permission Denied)

Assicurati che il tuo utente abbia i permessi di accesso al dispositivo seriale:

```bash
sudo usermod -aG dialout $USER
```

Effettua il logout e rientra nel sistema affinché la modifica abbia effetto.

### Porta non trovata

Verifica quali dispositivi seriali sono disponibili:

```bash
ls /dev/ttyACM*
```

oppure

```bash
ls /dev/ttyUSB*
```

### Scheda non rilevata

Controlla che:

- Il cavo USB supporti il ​​trasferimento dati.
- Sia selezionata la porta seriale corretta.
- Il bootloader di Arduino sia presente sul microcontrollore (MCU).

---

## Script di automazione

Questo repository include anche uno script di automazione che esegue automaticamente le fasi di compilazione e caricamento:

```bash
./build_and_install.sh
```

È il metodo più semplice per compilare e installare il firmware durante lo sviluppo.