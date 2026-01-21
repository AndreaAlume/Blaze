# Blaze

![logo](Blaze.png)

**Blaze** è un nuovo protocollo di comunicazione **modulare** basato sul modello TCP che si adegua agli standard, non ufficiali, ma de facto utilizzati nei protocolli odierni.

### Obiettivi 

Il progetto è nato come progetto studentesco per comprendere meglio le dinamiche di funzionamento dei moderni protocolli di comunicazioni e le potenziali vulnerabilità nella trasmissione di dati.

L'idea alla base non è dunque quella di sostituire protocolli noti e già perfettamente funzionanti, ma di **proporsi come possibile alternativa** per sviluppo di progetti personali, comunicazione tra dispositivi e così via.

I requisiti non funzionali tenuti a mente durante lo sviluppo del progetto saranno:

- performance
- sicurezza
- integrità
- affidabilità
- **modularità**

Per chiarire bene cosa stiamo definendo con il termine *modularità* del protocollo intendiamo un sistema dinamico che in base al tipo di dato che vogliamo trasferirre il protocollo decide autonomamente la modalità migliore per rispettare tutti i requisiti elencati sopra. 

Banale e ovvio, ma come già detto il progetto non è un *game changer*, ma ha un fine didattico  e personale.

## 1 Fattori in gioco

Questa sezione consisterà in un *"recap generico"* per comprendere quali sono i fattori che comprendono le basi della comunicazione seguendo una struttura scalare andando sempre più nel dettaglio per essere a conoscenza di quello che si sta sviluppando e come lo si sta sviluppando.

### 1.1 TCP

TCP è un canale per la comunicazione bidirezionale che possiamo definire tramite un esempio come un "tubo" stabile e flessibile che garantisce che data una certa quantità di dati in entrata (dal mittente) vi sia la stessa quantità in uscita (al ricevente).

**L'unica criticità** di questo sistema è che non garantisce un chunking specifico e prevedibile per ogni dato in arrivo. 

### 1.2 Socket

Con socket intendiamo la "porta d'accesso" che permette al mittente di comunicare col destinatario. 

L'unico vincolo che abbiamo in questa procedura è mantenere sempre aperta questa porta da entrambi le parti.

### 1.3 Frame

Il frame è la struttura logica che stabilisce le regole per definire l'insieme di dati che arrivano come devono essere interpretati. E' fondamentale in questa parte definire regole feree da rispettare per garantire l'affidabilità della comunicazione
