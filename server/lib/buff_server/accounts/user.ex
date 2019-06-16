defmodule BuffServer.Accounts.User do
  @moduledoc """
  Represents a user in the system, a user is essentially a real person using the system
  """
  use Ecto.Schema
  import Ecto.Changeset

  @password_hasher Application.get_env(:buff_server, :password_hasher)

  @primary_key {:id, :binary_id, autogenerate: true}
  @foreign_key_type :binary_id
  schema "users" do
    field(:full_name, :string)
    field(:public_email, :string)
    field(:email, :string)
    field(:is_public_email_verified, :boolean, default: false)
    field(:is_email_verified, :boolean, default: false)
    # TODO(danni): figure out how :load_in_query works, seems like a proper use here
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
      :email,
      :password,
      :password_confirmation
    ])
    |> validate_required([
      :full_name,
      :public_email,
      :is_public_email_verified,
      :email,
      :is_email_verified,
      :password
    ])
    |> validate_length(:password, min: 8)
    |> validate_confirmation(:password)
    |> validate_format(:public_email, ~r/@/)
    |> validate_format(:email, ~r/@/)
    |> unique_constraint(:email)
    |> put_pass_hash
    |> update_change(:email, &String.downcase/1)
  end

  defp put_pass_hash(%Ecto.Changeset{valid?: true, changes: %{password: password}} = changeset) do
    change(changeset, @password_hasher.add_hash(password, []))
  end

  defp put_pass_hash(changeset), do: changeset
end
