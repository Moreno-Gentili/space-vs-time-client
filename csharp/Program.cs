using SpacetimeDB.Types;

// TODO: Cambia l'indirizzo IP qui
await SpaceVsTime.Connect("127.0.0.1").Then(Init);

static void Init(DbConnection conn)
{
    // TODO: Scrivi codice qui per unirti alla partita
    // conn.Reducers...
}




// TODO: Questa servirà dopo...
/*
static void OnUpdate(EventContext ctx, Movable previous, Movable current)
{
    if (current.Name == "...")
    {
        switch (current.State)
        {
            case EntityState.ReadyToThrow:
                ctx.Reducers.ThrowTo(-current.Position.X, current.Position.Y);
                break;

            case EntityState.ReadyToMove:
                var firstUsableBall = ctx.Db.Movables.Iter().FirstOrDefault(m =>
                    m.Kind == EntityKind.Ball &&
                    Math.Sign(m.Position.X) == Math.Sign(current.Position.X));
            
                if (firstUsableBall is not null)
                    ctx.Reducers.MoveTo(firstUsableBall.Position.X, firstUsableBall.Position.Y);

                break;
        }
    }
}
*/