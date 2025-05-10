using SpacetimeDB;
using SpacetimeDB.Types;

// TODO: Cambia l'indirizzo IP qui
await SpaceVsTime.Connect("127.0.0.1").Then(Init);

static void Init(DbConnection conn)
{
    // TODO: Scrivi codice qui per controllare il tuo personaggio
    // conn.Reducers...
}