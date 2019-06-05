defmodule PuffServer.Factory do
  @moduledoc false
  use ExMachina.Ecto, repo: PuffServer.Repo

  def user_factory(attrs) do
    password = Map.get(attrs, :password, "some password")

    %PuffServer.Accounts.User{
      password: password,
      full_name: Faker.Name.name(),
      private_email: Faker.Internet.email(),
      public_email: Faker.Internet.email(),
      username: Faker.Internet.user_name()
    }
  end

  def set_password(user, password) do
    user |> PuffServer.Accounts.User.changeset(%{"password" => password})
  end
end
