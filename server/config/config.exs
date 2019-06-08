# This file is responsible for configuring your application
# and its dependencies with the aid of the Mix.Config module.
#
# This configuration file is loaded before any dependency and
# is restricted to this project.

# General application configuration
use Mix.Config

config :ex_aws,
  json_codec: Jason

# This prevents the GRPC server being automatically started when you start the app,
# the reason we dont want it is since if this happens, you'll not be able to run ```iex -S mix``` since
# it'll also try to start the GRPC server and fail on port being already used(duh)
config :grpc, start_server: false

config :buff_server,
  ecto_repos: [BuffServer.Repo],
  generators: [binary_id: true],
  password_hasher: Argon2

# Configures the endpoint
config :buff_server, BuffServerWeb.Endpoint,
  url: [host: "localhost"],
  secret_key_base: "wXUw9SIoWJdr5qja1muwdTp0f9m/op9COZtL/hoqqQH4WCFDc01CVAT6Y7Tq2c1T",
  render_errors: [view: BuffServerWeb.ErrorView, accepts: ~w(html json)],
  pubsub: [name: BuffServer.PubSub, adapter: Phoenix.PubSub.PG2]

# Configures Elixir's Logger
config :logger, :console,
  format: "$time $metadata[$level] $message\n",
  metadata: [:request_id]

# Use Jason for JSON parsing in Phoenix
config :phoenix, :json_library, Jason

if Mix.env() == :dev do
  config :mix_test_watch,
    tasks: [
      "test --stale",
      "credo"
    ]
end

# Import environment specific config. This must remain at the bottom
# of this file so it overrides the configuration defined above.
import_config "#{Mix.env()}.exs"
