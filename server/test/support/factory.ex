defmodule BuffServer.Factory do
  @moduledoc false
  use ExMachina.Ecto, repo: BuffServer.Repo

  alias BuffServer.Accounts
  alias BuffServer.Accounts.User

  def user_factory(attrs) do
    password = Map.get(attrs, :password, "some password")

    %BuffServer.Accounts.User{
      password: password,
      full_name: Faker.Name.name(),
      email: Faker.Internet.email(),
      public_email: Faker.Internet.email()
    }
  end

  def set_password(user, password) do
    user |> User.changeset(%{"password" => password})
  end

  @doc """
  Helper setup function that creates a user in the DB, also returns the original user
  """
  def setup_user_fixture(_context, attrs \\ %{}) do
    user_params = params_for(:user)

    {:ok, user} =
      attrs
      |> Enum.into(user_params)
      |> Accounts.create_user()

    [user: user, user_params: user_params]
  end
end
