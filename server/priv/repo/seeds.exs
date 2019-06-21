# Script for populating the database. You can run it as:
#
#     mix run priv/repo/seeds.exs
#
# Inside the script, you can read and write to any of your
# repositories directly:
#
#     BuffServer.Repo.insert!(%BuffServer.SomeSchema{})
#
# We recommend using the bang functions (`insert!`, `update!`
# and so on) as they will fail if something goes wrong.

%BuffServer.Accounts.User{}
|> BuffServer.Accounts.User.changeset(%{
  password: "password",
  full_name: "Test Test",
  email: "test@test.com",
  is_email_verified: true,
  public_email: "test@test.com",
  is_public_email_verified: true
})
|> BuffServer.Repo.insert()
