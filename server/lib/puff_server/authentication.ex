defmodule BuffServer.Authentication do
  @moduledoc """
  Handles Authentication in Buff - Authentication is currently done via username/password or JWT token.
  This Context delegates a lot of its work to the Accounts context and to Joken Token
  """

  alias BuffServer.Accounts
  alias BuffServer.Authentication.Token

  @password_hasher Application.get_env(:buff_server, :password_hasher)

  @doc """
    Tries to authenticate a given username using the provided passowrd
    Returns tuple {:ok, token} if successful, throws otherwise
  """
  @spec authenticate!(String.t(), String.t()) :: {:ok, String.t()}
  def authenticate!(username, password) do
    user = Accounts.get_by_username!(username)
    {:ok, user} = @password_hasher.check_pass(user, password, [])

    {:ok, token, _claims} =
      Token.generate_and_sign(%{"user_id" => user.id, "username" => user.username})

    {:ok, token}
  end
end