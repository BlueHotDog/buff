defmodule BuffServer.Packages.Package do
  use Ecto.Schema
  import Ecto.Changeset

  @primary_key {:id, :binary_id, autogenerate: true}
  @foreign_key_type :binary_id
  schema "packages" do
    field(:description, :string)
    field(:homepage, :string)
    field(:keywords, {:array, :string})
    field(:name, :string)
    field(:repository_url, :string)
    field(:owner_user_id, :binary_id)

    timestamps()
  end

  @doc false
  def changeset(package, attrs) do
    package
    |> cast(attrs, [:name, :description, :keywords, :homepage, :repository_url])
    |> validate_required([:name, :description, :keywords, :homepage, :repository_url])
    |> unique_constraint(:name)
    |> validate_url(:repository_url)
  end

  @doc false
  defp validate_url(changeset, field, _opts \\ []) do
    validate_change(changeset, field, fn _, value ->
      case URI.parse(value) do
        %URI{scheme: nil} ->
          {:err, "is missing a scheme (e.g. https)"}

        %URI{host: nil} ->
          {:err, "is missing a host"}

        %URI{host: host} ->
          case :inet.gethostbyname(Kernel.to_charlist(host)) do
            {:ok, _} -> :ok
            {:error, _} -> {:err, "invalid host"}
          end
      end
      |> case do
        {:err, msg} -> [{field, msg}]
        :ok -> []
      end
    end)
  end
end
