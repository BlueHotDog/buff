# TODO: maybe there needs to be another scope of Registry
defmodule BuffServer.Packages do
  @s3_bucket_name Application.get_env(:buff_server, :s3_bucket_name)


  @moduledoc """
  The Packages context.
  """

  import Ecto.Query, warn: false
  alias BuffServer.Repo

  alias BuffServer.Packages.Package

  @doc """
  Returns the list of packages.

  ## Examples

      iex> list_packages()
      [%Package{}, ...]

  """
  def list_packages do
    Repo.all(Package)
  end

  @doc """
  Gets a single package.

  Raises `Ecto.NoResultsError` if the Package does not exist.

  ## Examples

      iex> get_package!(123)
      %Package{}

      iex> get_package!(456)
      ** (Ecto.NoResultsError)

  """
  def get_package!(id) do
    package = Repo.get!(Package, id)
    package
  end

  @doc """
  Creates a package.

  ## Examples

      iex> create_package(%{field: value})
      {:ok, %Package{}}

      iex> create_package(%{field: bad_value})
      {:error, %Ecto.Changeset{}}

  """
  def create_package!(%{artifact_binary: artifact_binary} = attrs) when is_binary(artifact_binary) do
    changeset = %Package{}
    |> Package.changeset(attrs, [:s3_bucket_name, :s3_bucket_path])

    bucket_path = "/#{attrs.name}/artifact"
    %{status_code: 200} = ExAws.S3.put_object(@s3_bucket_name, bucket_path, artifact_binary) |> ExAws.request!
    attrs = Map.merge(attrs, %{s3_bucket_name: @s3_bucket_name, s3_bucket_path: bucket_path})
    changeset
    |> Package.changeset(attrs)
    |> Repo.insert()
  end

  @doc """
  Updates a package.

  ## Examples

      iex> update_package(package, %{field: new_value})
      {:ok, %Package{}}

      iex> update_package(package, %{field: bad_value})
      {:error, %Ecto.Changeset{}}

  """
  # def update_package(%Package{} = package, attrs) do
  #   package
  #   |> Package.changeset(attrs)
  #   |> Repo.update()
  # end

  @doc """
  Deletes a Package.

  ## Examples

      iex> delete_package(package)
      {:ok, %Package{}}

      iex> delete_package(package)
      {:error, %Ecto.Changeset{}}

  """
  def delete_package!(%Package{} = package) do
    {:ok, %BuffServer.Packages.Package{}} = Repo.delete(package)
    %{status_code: 200} = ExAws.S3.delete_object(package.s3_bucket_name, package.s3_bucket_path) |> ExAws.request!
  end

  @doc """
  Returns an `%Ecto.Changeset{}` for tracking package changes.

  ## Examples

      iex> change_package(package)
      %Ecto.Changeset{source: %Package{}}

  """
  # def change_package(%Package{} = package) do
  #   Package.changeset(package, %{})
  # end
end
