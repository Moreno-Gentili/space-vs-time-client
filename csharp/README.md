# Space VS Time - Client C#

Questo è il client C# per partecipare al workshop "Space VS Time".

## Preparazione
Si tratta di un'applicazione console .NET 8. Verifica che la compilazione vada a buon fine sul tuo sistema lanciando il comando:

```
dotnet build
```

Puoi anche provare ad eseguirla con:
```
dotnet run
```

Vedrai apparire l'errore `Error connecting to SpaceTimeDB` che per ora è normale.

## Partecipazione al workshop
Ti verrà chiesto di scrivere codice unicamente nel file [Program.cs](./Program.cs). Non dovrai fare alcuna modifica ai file che si trovano nella directory `ModuleBindings` che sono stati autogenerati da SpaceTimeDB e rappresentano il client che astrae la comunicazione con il server. Neanche il file [Helpers.cs](./Helpers.cs) dovrà essere modificato: contiene solo codice che aiuterà durante il workshop.