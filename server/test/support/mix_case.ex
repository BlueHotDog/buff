defmodule BuffServer.MixCase do
  @moduledoc """
  This module defines the setup for tests requiring
  access to Mix tasks
  """

  use ExUnit.CaseTemplate

  setup_all do
    Mix.shell(Mix.Shell.Process)

    on_exit(fn ->
      Mix.shell(Mix.Shell.IO)
    end)

    :ok
  end
end
