defmodule BuffServerGrpc.AuthService.Server do
  @moduledoc """
  Implementation of GRPC Authentication service, in charge of logging in/out users
  """
  use GRPC.Server, service: BuffServerGrpc.AuthService.Service

  @login_error GRPC.RPCError.exception(
                 GRPC.Status.unauthenticated(),
                 "Incorrect credentials"
               )
  @spec login(%{password: String.t(), username: String.t()}, any) :: any
  def login(%{username: username, password: password}, _stream) do
    {:ok, token} = BuffServer.Authentication.authenticate!(username, password)
    BuffServerGrpc.LoginResponse.new(token: token)
  rescue
    Ecto.NoResultsError ->
      reraise @login_error, __STACKTRACE__

    MatchError ->
      reraise @login_error, __STACKTRACE__
  end
end
