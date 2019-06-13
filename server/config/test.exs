use Mix.Config

config :grpc, start_server: false

config :buff_server,
  s3_bucket_name: "buff-packages-test"

config :buff_server,
  password_hasher: BuffServer.Argon2Mock

config :ex_aws,
  region: "local",
  access_key_id: "minio",
  secret_access_key: "minio123",
  http_client: ExAws.Request.HttpMock

config :ex_aws, :s3,
  host: "127.0.0.1",
  region: "local",
  scheme: "http://",
  port: 9001,
  http_client: ExAws.Request.HttpMock

# Configure your database
config :buff_server, BuffServer.Repo,
  username: "postgres",
  password: "postgres",
  database: "buff_server_test",
  hostname: "localhost",
  pool: Ecto.Adapters.SQL.Sandbox

config :joken,
  default_signer: "testtest"

# We don't run a server during test. If one is required,
# you can enable the server option below.
config :buff_server, BuffServerWeb.Endpoint,
  http: [port: 4002],
  server: false

# Print only warnings and errors during test
config :logger, level: :warn
