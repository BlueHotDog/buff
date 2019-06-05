defmodule buffServer.ComeoninStub do
  @moduledoc false
  @behaviour Comeonin

  def add_hash(password, opts) do
    %{password_hash: "password_hash", password: nil}
  end

  def check_pass(user_struct, _password, _opts) do
    {:ok, user_struct}
  end

  def no_user_verify(_opts) do
    false
  end
end
