defmodule BuffServer.Application do
  # See https://hexdocs.pm/elixir/Application.html
  # for more information on OTP Applications
  @moduledoc false

  use Application

  def start(_type, _args) do
    # List all child processes to be supervised
    children = [
      # Start the Ecto repository
      BuffServer.Repo,
      # Start the endpoint when the application starts
      BuffServerWeb.Endpoint,
      # Starts a worker by calling: BuffServer.Worker.start_link(arg)
      # {BuffServer.Worker, arg},
      # TODO: move port to config arg
      {GRPC.Server.Supervisor, {BuffServerGrpc.Endpoint, 50_051}}
    ]

    # See https://hexdocs.pm/elixir/Supervisor.html
    # for other strategies and supported options
    opts = [strategy: :one_for_one, name: BuffServer.Supervisor]
    Supervisor.start_link(children, opts)
  end

  # Tell Phoenix to update the endpoint configuration
  # whenever the application is updated.
  # coveralls-ignore-start
  def config_change(changed, _new, removed) do
    BuffServerWeb.Endpoint.config_change(changed, removed)
    :ok
  end

  # coveralls-ignore-stop
end
