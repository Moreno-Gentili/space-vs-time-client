# Space VS Time - Client Rust

Questa è un'applicazione console Rust per la partecipazione al workshop "Space VS Time".

## Preparazione
Compila il progetto con:

```
cargo build
```

Se la compilazione va in errore, è probabile che l'errore che trovi in output ti dica cosa manca. Se sei su Linux (anche WSL), probabilmente dovrai installare i pacchetti `openssl`,  `pkg-config` e `libssl-dev`.

Poi verifica che l'applicazione riesca ad avviarsi con il comando:

```
cargo run
```

Vedrai apparire l'errore `Error connecting to SpaceTimeDB` ma questo per il momento è normale.

## Partecipazione al workshop
Ti verrà chiesto di scrivere codice unicamente nel file [src/main.rs](./src/main.rs). Non dovrai fare alcuna modifica ai file che si trovano nella directory `src/module_bindings` che sono stati autogenerati da SpaceTimeDB e rappresentano il client che astrae la comunicazione con il server. Nenche il file [src/helpers.rs](./src/helpers.rs) dovrà essere modificato: contiene solo codice che aiuterà durante il workshop.
