defmodule BuffServer.ComeoninStub do
  @moduledoc false
  @behaviour Comeonin

  def add_hash(password, _opts \\ []) do
    %{password_hash: "#{password}123", password: nil}
  end

  def check_pass(user_struct, password, _opts \\ []) do
    if user_struct.password_hash == "#{password}123" do
      {:ok, user_struct}
    else
      {:error, "Incorrect password or email"}
    end
  end

  def no_user_verify(_opts) do
    false
  end
end
