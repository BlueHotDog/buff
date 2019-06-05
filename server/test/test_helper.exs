ExUnit.configure(formatters: [ExUnit.CLIFormatter, ExUnitNotifier])

{:ok, _} = Application.ensure_all_started(:ex_machina)
ExUnit.start()
Ecto.Adapters.SQL.Sandbox.mode(BuffServer.Repo, :manual)
Faker.start()
