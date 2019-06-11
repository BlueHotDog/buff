defmodule BuffServer.Authentication do
  @moduledoc """
  Handles Authentication in Buff - Authentication is currently done via email/password or JWT token.
  This Context delegates a lot of its work to the Accounts context and to Joken Token
  """

  alias BuffServer.Accounts
  alias BuffServer.Authentication.Token

  @password_hasher Application.get_env(:buff_server, :password_hasher)

  @doc """
    Tries to authenticate a given email using the provided passowrd
    Returns tuple {:ok, token} if successful, throws otherwise
  """
  @spec authenticate!(String.t(), String.t()) :: {:ok, String.t()}
  def authenticate!(email, password) do
    user = Accounts.get_by_email!(email)
    {:ok, user} = @password_hasher.check_pass(user, password, [])

    {:ok, token, _claims} =
      Token.generate_and_sign(%{"user_id" => user.id, "email" => user.email})

    {:ok, token}
  end
end
