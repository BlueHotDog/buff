defmodule BuffServerGrpc.IntegrationCase do
  @moduledoc """
  This module defines the test case to be used by
  tests that require setting up a GRPC server/client.

  Such tests rely on `GRPC.Server` and `GRPC.Stub` to
  start the GRPC Server/Endpoint to manage communications.
  """

  use ExUnit.CaseTemplate

  using do
    quote do
      import BuffServerGrpc.IntegrationCase,
        only: [
          run_server: 2,
          run_server: 3,
          run_endpoint: 2,
          run_endpoint: 3,
          get_client: 2,
          get_client: 3
        ]
    end
  end

  def run_server(servers, func, port \\ 0) do
    {:ok, _pid, port} = GRPC.Server.start(servers, port)

    try do
      func.(port)
    after
      :ok = GRPC.Server.stop(servers)
    end
  end

  def get_client(server, func, port \\ 0) do
    run_server(
      server,
      fn port ->
        # Seems like i can't use default options since the GRPC elixir lib passes some weird arguments to GUN.
        opts = [
          adapter_opts: %{retry: 10, retry_timeout: 1_000}
        ]

        {:ok, channel} = GRPC.Stub.connect("localhost:#{port}", opts)

        try do
          func.(channel)
        after
          {:ok, _channel} = GRPC.Stub.disconnect(channel)
        end
      end,
      port
    )
  end

  def run_endpoint(endpoint, func, port \\ 0) do
    {:ok, _pid, port} = GRPC.Server.start_endpoint(endpoint, port)

    try do
      func.(port)
    after
      :ok = GRPC.Server.stop_endpoint(endpoint, [])
    end
  end
end
