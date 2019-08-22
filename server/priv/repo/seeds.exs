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

BuffServer.Repo.insert!(%Ecto.Changeset{
  action: nil,
  changes: %{
    email: "test@test.com",
    full_name: "Test Test",
    # password is password
    password_hash: "$argon2id$v=19$m=131072,t=8,p=4$C9cbxIUGINuaSKdW8T/fNA$6I7e3BScnTku88kN73lZHWJHbXYemApHYJ9ZSsgrfN0",
    public_email: "test@test.com"
  },
  errors: [],
  data: %BuffServer.Accounts.User{},
  valid?: true
})
