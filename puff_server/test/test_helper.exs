ExUnit.configure(formatters: [ExUnit.CLIFormatter, ExUnitNotifier])

Mox.defmock(Argon2, for: Comeonin)

ExUnit.start()
Ecto.Adapters.SQL.Sandbox.mode(PuffServer.Repo, :manual)
