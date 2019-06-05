defmodule PuffServer.Authentication.Token do
  @moduledoc """
  Implementing custom JWT token using Joken.
  """
  use Joken.Config

  @hostname Application.get_env(:puff_server, PuffServerWeb.Endpoint) |> get_in([:url, :host])
  # 7 days
  @expiration 60 * 60 * 24 * 7

  @impl true
  def token_config do
    default_claims(default_exp: @expiration)
    |> add_claim("iss", fn -> "PuffServer" end, &(&1 == "PuffServer"))
    |> add_claim("aud", fn -> @hostname end, &(&1 == @hostname))
  end
end
