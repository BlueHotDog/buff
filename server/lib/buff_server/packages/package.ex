defmodule BuffServer.Packages.Package do
  @moduledoc """
  Table to hold metadata about the packages
  """
  use Ecto.Schema
  import Ecto.Changeset

  @primary_key {:id, :binary_id, autogenerate: true}
  @foreign_key_type :binary_id
  @required_fields [
    :name,
    :description,
    :keywords,
    :homepage,
    :repository_url,
    :s3_bucket_name,
    :s3_bucket_path
  ]
  schema "packages" do
    field(:description, :string)
    field(:homepage, :string)
    field(:keywords, {:array, :string})
    field(:name, :string)
    field(:repository_url, :string)
    field(:owner_user_id, :binary_id)
    field(:s3_bucket_name, :string)
    field(:s3_bucket_path, :string)

    timestamps()
  end

  @doc false
  def changeset(package, attrs) do
    package
    |> cast(attrs, @required_fields)
    |> validate_required(@required_fields)
    |> unique_constraint(:name)
    |> validate_url(:repository_url)
  end

  @doc false
  defp validate_url(changeset, field, _opts \\ []) do
    validate_change(changeset, field, fn _, url ->
      uri = URI.parse(url)

      case uri.scheme != nil && uri.host =~ "." do
        true -> []
        false -> [{field, "invalid url"}]
      end
    end)
  end
end
