defmodule BuffServer.Factory do
  @moduledoc false
  use ExMachina.Ecto, repo: BuffServer.Repo

  def user_factory(attrs) do
    password = Map.get(attrs, :password, "some password")

    %BuffServer.Accounts.User{
      password: password,
      full_name: Faker.Name.name(),
      private_email: Faker.Internet.email(),
      public_email: Faker.Internet.email(),
      username: Faker.Internet.user_name()
    }
  end

  def set_password(user, password) do
    user |> BuffServer.Accounts.User.changeset(%{"password" => password})
  end
end
