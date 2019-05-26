defmodule PuffServer.Accounts.User do
  use Ecto.Schema
  import Ecto.Changeset
  # alias Comeonin

  @primary_key {:id, :binary_id, autogenerate: true}
  @foreign_key_type :binary_id
  schema "users" do
    field(:full_name, :string)
    field(:private_email, :string)
    field(:public_email, :string)
    field(:username, :string)
    field(:is_public_email_verified, :boolean, default: false)
    field(:is_private_email_verified, :boolean, default: false)
    field(:encrypted_password, :string)

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
    |> validate_length(:username, min: 4)
    |> validate_format(:username, ~r/^[a-z][a-z_0-9]+[a-z0-9]$/i)
    |> unique_constraint(:username)
    |> update_change(:encrypted_password, &put_pass_hash/1)
    |> update_change(:username, &String.downcase/1)
  end

  @doc false
  defp put_pass_hash(%Ecto.Changeset{valid?: true, changes: %{password: password}} = changeset) do
    change(changeset, Comeonin.add_hash(password))
  end

  @doc false
  defp put_pass_hash(changeset), do: changeset

  defp downcase_username(changeset) do
    update_change(changeset, :username, &String.downcase/1)
  end
end
