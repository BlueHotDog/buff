defmodule BuffServer.Packages.Package do
  @moduledoc """
  Table to hold metadata about the packages
  """
  use Ecto.Schema
  import Ecto.Changeset

  @primary_key {:id, :binary_id, autogenerate: true}
  @foreign_key_type :binary_id
  @required_fields [:name, :description, :keywords, :homepage, :repository_url, :s3_bucket_name, :s3_bucket_path]
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
  def changeset(package, attrs, omit_fields \\ []) do
    required_fields = @required_fields -- omit_fields
    package
    |> cast(attrs, required_fields)
    |> validate_required(required_fields)
    |> unique_constraint(:name)
    |> validate_url(:repository_url)
  end

  @doc false
  defp validate_url(changeset, field, _opts \\ []) do
    validate_host = fn host ->
      case :inet.gethostbyname(Kernel.to_charlist(host)) do
        {:ok, _} -> :ok
        {:error, _} -> {:err, "invalid host"}
      end
    end

    validate_change(changeset, field, fn _, value ->
      result =
        case URI.parse(value) do
          %URI{scheme: nil} ->
            {:err, "is missing a scheme (e.g. https)"}

          %URI{host: nil} ->
            {:err, "is missing a host"}

          %URI{host: host} ->
            validate_host.(host)
        end

      result
      |> case do
        {:err, msg} -> [{field, msg}]
        :ok -> []
      end
    end)
  end
end
