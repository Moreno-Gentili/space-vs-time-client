using SpacetimeDB;
using SpacetimeDB.Types;

await SpaceVsTime.Connect("127.0.0.1").Then(Init);

static void Init(DbConnection conn)
{
    // TODO: Scrivi codice qui
    // conn.Reducers...
}