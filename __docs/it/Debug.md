[< HOME](../../README.it.md)
# 🚀 Debug con UART

Durante lo sviluppo di applicazioni embedded è spesso necessario visualizzare i
valori letti dai sensori o monitorare lo stato dell'applicazione in esecuzione.

Questa guida descrive una semplice soluzione basata sulla comunicazione UART per effettuare
il debug di firmware sviluppati per ATmega328P in Rust.
---

## Requisiti

### Installazione software sul PC di sviluppo

Installare `picocom`, un terminale seriale leggero utilizzato per leggere i messaggi inviati dalla UART 
del micro-controllore.
```bash
sudo apt install picocom
```

---

### Visualizzazione dei messaggi seriali

Per connettersi alla porta seriale dell'Arduino ed iniziare a visualizzare i messaggi di debug:
```bash
picocom -b 115200 /dev/ttyACM0
```

Dove:
- 115200 è il baud rate configurato nel firmware
- /dev/ttyACM0 è la porta seriale esposta dall'Arduino sul sistema Linux.

Una volta stabilita la connessione, tutti i dati inviati dalla UART saranno visualizzati nel terminale.

---

### Terminare Picom

Per terminare la sessione di monitoraggio:
```
Ctrl + A
Ctrl + X
```
> Nota: i due comandi devono essere eseguiti in sequenza e non contemporaneamente.
---

### Invio Messaggio UART

Per semplificare le operazioni di debug è disponibile una libreria UART minimale nel percorso: `./library/uart`
Prima di utilizzarla è necessario inizializzare la periferica USART.

L'esempio seguente configura:
- Clock MCU: 16 MHz
- Baud rate: 115200
- Formato frame: 8N1 (8 bit dati, nessuna parità, 1 bit di stop)

```rust
// 16 MHz - 115200 baud
dp.USART0
    .ubrr0()
    .write(|w| unsafe { w.bits(8) });

// Abilitazione trasmissione (TX)
dp.USART0.ucsr0b().write(|w| {
    w.txen0().set_bit()
});

// Configurazione 8N1:
// - 8 bit dati
// - Nessuna parità
// - 1 bit di stop
dp.USART0.ucsr0c().write(|w| unsafe {
    w.bits(0b0000_0110)
});

let uart = Uart::new(dp.USART0);
```

Una volta inizializzata la periferica, è possibile inviare un messaggio di debug:
`uart.write_str(b"Hi, we're connected!\r\n");`