defmodule buffServer.Accounts.User do
  use Ecto.Schema
  import Ecto.Changeset

  @password_hasher Application.get_env(:buff_server, :password_hasher)

  @primary_key {:id, :binary_id, autogenerate: true}
  @foreign_key_type :binary_id
  schema "users" do
    field(:full_name, :string)
    field(:private_email, :string)
    field(:public_email, :string)
    field(:username, :string)
    field(:is_public_email_verified, :boolean, default: false)
    field(:is_private_email_verified, :boolean, default: false)
    # TODO: figure out how :load_in_query works, seems like a proper use here
    field(:password_hash, :string)

    # Virtual fields - We dont want to save them to the DB
    field(:password, :string, virtual: true)
    field(:password_confirmation, :string, virtual: true)

    timestamps()
  end

  @doc false
  def changeset(user, attrs) do
    user
    |> cast(attrs, [
      :full_name,
      :public_email,
      :private_email,
      :username,
      :password,
      :password_confirmation
    ])
    |> validate_required([
      :full_name,
      :public_email,
      :private_email,
      :username,
      :password,
      :is_public_email_verified,
      :is_private_email_verified
    ])
    |> validate_length(:password, min: 8)
    |> validate_confirmation(:password)
    |> validate_format(:public_email, ~r/@/)
    |> validate_format(:private_email, ~r/@/)
    |> validate_length(:username, min: 4, max: 20)
    |> validate_format(:username, ~r/^[a-z][a-z_0-9]+[a-z0-9]$/i)
    |> unique_constraint(:username)
    |> put_pass_hash
    |> update_change(:username, &String.downcase/1)
  end

  defp put_pass_hash(%Ecto.Changeset{valid?: true, changes: %{password: password}} = changeset) do
    change(changeset, @password_hasher.add_hash(password, b: 4))
  end

  defp put_pass_hash(changeset), do: changeset
end
