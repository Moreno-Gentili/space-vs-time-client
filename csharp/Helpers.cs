using System.Runtime.InteropServices;
using SpacetimeDB;
using SpacetimeDB.Types;
using System.IO;

public static class TaskExtensions
{
    public static async Task Then<T>(this Task<T> task, Action<T> continuation)
    {
        try
        {
            continuation(await task.ConfigureAwait(false));
        }
        catch
        {
        }
    }
}

public static class Vector3DExtensions
{
    public static double Distance(this Vector3D pos1, Vector3D pos2)
    {
        float dx = pos1.X - pos2.X;
        float dy = pos1.Y - pos2.Y;
        float dz = pos1.Z - pos2.Z;
        return Math.Sqrt(dx * dx + dy * dy + dz * dz);
    }
}

public static class SpaceVsTime
{
    private const string tokenPath = "./token.txt";
    private static Identity myIdentity = default;

    public static Task<DbConnection> Connect(string hostName)
    {
        CancellationTokenSource tokenSource = new();
        HandleCancellation(tokenSource);

        TaskCompletionSource<DbConnection> tcs = new();
        DbConnection conn = GetDbConnection(hostName, tokenSource, tcs);
        
        StartPollingConnection(tokenSource, conn);
        SubscribeReducers(conn);

        return tcs.Task;
    }

    private static void StartPollingConnection(CancellationTokenSource tokenSource, DbConnection conn)
    {
        Task.Run(() => PollConnection(conn, tokenSource.Token));
    }

    private static DbConnection GetDbConnection(string hostName, CancellationTokenSource tokenSource, TaskCompletionSource<DbConnection> tcs)
    {
        return DbConnection.Builder()
                    .WithUri($"http://{hostName}")
                    .WithModuleName("space-vs-time")
                    .WithToken(GetTokenIfAny())
                    .OnConnect((conn, identity, token) => OnConnected(conn, identity, token, tcs))
                    .OnConnectError(exc => OnConnectError(exc, tcs, tokenSource))
                    .OnDisconnect((conn, exc) => OnDisconnected(tokenSource))
                    .Build();
    }

    private static void HandleCancellation(CancellationTokenSource tokenSource)
    {
        Console.CancelKeyPress += (sender, e) =>
        {
            e.Cancel = true;
            tokenSource.Cancel();
        };
    }

    private static void SubscribeReducers(DbConnection conn)
    {
        conn.Reducers.OnJoin += OnJoin;
        conn.Reducers.OnMoveTo += OnMoveTo;
        conn.Reducers.OnThrowTo += OnThrowTo;
        conn.Reducers.OnSay += OnSay;
    }

    private static void OnJoin(ReducerEventContext ctx, string name) =>
        OnReducerResult(ctx, $"You joined as {name}", "Join failed");

    private static void OnThrowTo(ReducerEventContext ctx, float x, float y) =>
        OnReducerResult(ctx, $"You started throwing a ball to {x},{y}", "ThrowTo failed");

    private static void OnMoveTo(ReducerEventContext ctx, float x, float y) =>
        OnReducerResult(ctx, $"You started moving to {x},{y}", "MoveTo failed");

    private static void OnSay(ReducerEventContext ctx, string text) =>
        OnReducerResult(ctx, $"You say: ${text}", "Say failed");

    private static void OnReducerResult(ReducerEventContext ctx, string successMessage, string failureMessage) {
        if(ctx.Event.CallerIdentity == myIdentity) {
            switch (ctx.Event.Status) {
                case Status.Committed:
                    Console.WriteLine($"✅ {successMessage}");
                    break;
                case Status.Failed:
                    Console.WriteLine($"❌ {failureMessage}: {ctx.Event.Status}");
                    break;
            }
        }
    }

    private static void OnConnectError(Exception exc, TaskCompletionSource<DbConnection> tcs, CancellationTokenSource tokenSource)
    {
        Console.Error.WriteLine("❌ Error connecting to SpaceTimeDB");
        tokenSource.Cancel();
        tcs.SetException(exc);
    }

    private static void OnConnected(DbConnection conn, Identity identity, string token, TaskCompletionSource<DbConnection> tcs)
    {
        myIdentity = identity;
        SaveToken(token);
        Console.WriteLine($"✅ Connected to SpaceTimeDB with identity: {identity}");
        tcs.SetResult(conn);
    }

    private static void OnDisconnected(CancellationTokenSource tokenSource)
    {
        tokenSource.Cancel();
        Console.WriteLine("Disconnected");
    }

    private static async Task PollConnection(DbConnection conn, CancellationToken token)
    {
        using PeriodicTimer timer = new(TimeSpan.FromMilliseconds(1000.0 / 60));
        try
        {
            while (await timer.WaitForNextTickAsync(token))
            {
                conn.FrameTick();
            }
        }
        catch (OperationCanceledException)
        {
        }

        try
        {
            conn.Disconnect();
            Console.WriteLine("Exiting...");
        }
        catch
        {
        }

        Environment.Exit(0);
    }

    private static string? GetTokenIfAny()
    {
        if (File.Exists(tokenPath)) {
            return File.ReadAllText(tokenPath);
        } else {
            return null;
        }
    }

    private static void SaveToken(string token)
    {
        File.WriteAllText(tokenPath, token);
    }
}