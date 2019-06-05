use Mix.Config

config :buff_server,
  password_hasher: buffServer.Argon2Mock

# Configure your database
config :buff_server, buffServer.Repo,
  username: "postgres",
  password: "postgres",
  database: "buff_server_test",
  hostname: "localhost",
  pool: Ecto.Adapters.SQL.Sandbox

# We don't run a server during test. If one is required,
# you can enable the server option below.
config :buff_server, buffServerWeb.Endpoint,
  http: [port: 4002],
  server: false

# Print only warnings and errors during test
config :logger, level: :warn
