# Rust Padronanza — Trait, Generics, Error Handling

Repo di esercizi progressivi in stile Rustlings/TDD: **i test sono già scritti, il tuo lavoro è farli passare.** Ogni modulo in `src/` è un problema autonomo con la traccia completa nel doc comment in cima al file.

## Come lavorare

```bash
# Compila tutto (deve compilare da subito, i test falliranno per i todo!())
cargo build --tests

# Lavora su un problema alla volta
cargo test ex01

# Quando tutti i moduli sono verdi
cargo test
```

## I 10 problemi

| Modulo | Tema | Concetti chiave |
|--------|------|-----------------|
| `ex01_notifiche` | Sistema di notifiche | trait, metodi di default, override |
| `ex02_statistiche` | Statistiche generiche | funzioni generiche, bound multipli, `where` |
| `ex03_codici` | Validatore codici prodotto | `Result`, enum errore, dominio tipizzato |
| `ex04_tariffe` | Tariffe di spedizione | trait+generics, `impl Trait`, dispatch statico |
| `ex05_registro` | Registro con capacità | struct generica, impl condizionali |
| `ex06_movimenti` | Import movimenti bancari | `?`, `From`, catene di conversione |
| `ex07_pipeline` | Pipeline a plugin | trait objects, object safety, `clona_box` |
| `ex08_sorgenti` | Sorgenti dati | associated types vs generics sul trait |
| `ex09_prenotazioni` | Servizio prenotazioni | `source()`, errori a strati, DI via trait |
| `ex10_motore_regole` | Motore di regole | blanket impl, `Infallible`, `try_fold` |

Ordine consigliato: numerico. Ritmo: 1–3 in 2-3 giorni, 4–6 in 3-4 giorni, 7–9 uno ogni due giorni, il 10 in un weekend.

## Regole

1. **Vietato modificare i test.** Puoi aggiungerne di tuoi (consigliato).
2. **Vietato `unwrap()`/`expect()`** fuori dai test.
3. **Ogni `clone()` va giustificato**: se esiste un'alternativa con riferimenti, usala.
4. **Le DOMANDE DI PADRONANZA in cima a ogni file vanno risposte per iscritto** (in un commento in fondo al file) prima di considerare chiuso il problema. Se compila ma non sai rispondere, non hai finito.
5. Alcuni file contengono righe commentate del tipo `// r.elenco(); // <- NON deve compilare`: decommentale, leggi l'errore del compilatore per intero, e spiega perché nel tuo commento di risposta.

## Metodo consigliato per ogni problema

1. Leggi la traccia nel doc comment, poi leggi i **test**: sono la specifica esecutiva.
2. Scrivi su carta i tipi e il flusso prima di toccare l'editor.
3. Implementa un `todo!()` alla volta, lanciando i test a ogni passo.
4. A verde: rispondi alle domande di padronanza sperimentando nel codice (non a memoria).
5. Revisione finale: cerca `clone()` superflui, `String` dove bastava `&str`, `match` dove bastava un combinatore.

## Note

- Il progetto compila da subito con `cargo build --tests`: i corpi sono `todo!()`, quindi i test falliscono a runtime, non a compile time. Fa eccezione qualche punto dove un tuo errore di design può rompere la compilazione: è voluto, l'errore del compilatore è parte dell'esercizio.
- Zero dipendenze esterne: tutto con la standard library. Quando avrai finito, rifai gli errori dei problemi 6 e 9 con `thiserror` e confronta il codice generato con il tuo.
# rust_trait
