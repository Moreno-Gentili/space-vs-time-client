# Space VS Time - Client TypeScript

Questa è un'applicazione console per NodeJS >= 20, scritta in TypeScript, per la partecipazione al workshop "Space VS Time".

## Preparazione
Ripristina i pacchetti con il comando:

```
pnpm i
```

Poi verifica che l'applicazione riesca ad avviarsi con il comando:

```
pnpm start
```

Vedrai apparire l'errore `Error connecting to SpaceTimeDB` ma questo per il momento è normale.

## Partecipazione al workshop
Ti verrà chiesto di scrivere codice unicamente nel file [src/app.ts](./src/app.ts). Non dovrai fare alcuna modifica ai file che si trovano nella directory `src/module_bindings` che sono stati autogenerati da SpaceTimeDB e rappresentano il client che astrae la comunicazione con il server. Nenche il file [src/helpers.ts](./src/helpers.ts) dovrà essere modificato: contiene solo codice che aiuterà durante il workshop.