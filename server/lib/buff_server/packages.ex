defmodule BuffServer.Packages do
  @moduledoc """
  The Packages Context.
  A Package holds the actual artifact with the Protobuf files and also meta-data to help manage
  the pacakge itself.
  """

  @s3_bucket_name Application.get_env(:buff_server, :s3_bucket_name)

  import Ecto.Query, warn: false

  alias BuffServer.Packages.Package
  alias BuffServer.Repo
  alias Ecto.Multi

  @doc """
  Returns the list of packages.
  """
  def list_packages do
    Repo.all(Package)
  end

  @doc """
  Gets a single package.

  Raises `Ecto.NoResultsError` if the Package does not exist.
  """
  def get_package!(id), do: Repo.get!(Package, id)

  @doc """
  Creates the package and uploads to S3, this is run inside a transaction so if anything fails it'll rollback.
  """
  def create_package(%{artifact_binary: artifact_binary} = attrs)
      when is_binary(artifact_binary) do
    bucket_path = s3_bucket_path(attrs)
    attrs = Map.merge(attrs, %{s3_bucket_name: @s3_bucket_name, s3_bucket_path: bucket_path})
    changeset = Package.changeset(%Package{}, attrs)

    transaction_res =
      Multi.new()
      |> Multi.insert(:package, changeset)
      |> Multi.run(:s3_artifact, fn _repo, changes ->
        put_result =
          @s3_bucket_name
          |> ExAws.S3.put_object(bucket_path, artifact_binary)
          |> ExAws.request()

        with {:ok, %{status_code: 200}} <- put_result do
          {:ok, changes}
        else
          err -> {:error, err}
        end
      end)
      |> Repo.transaction()

    with {:ok, %{package: package}} <- transaction_res do
      {:ok, package}
    else
      err -> err
    end
  end

  @doc """
  Generates the s3 bucket path to be used for this package.
  """
  def s3_bucket_path(attrs) do
    "/#{attrs.name}/artifact"
  end

  @doc """
  Deletes a Package.
  """
  def delete_package(%Package{} = package) do
    Multi.new()
    |> Multi.delete(:package, package)
    |> Multi.run(:s3_artifact, fn _repo, changes ->
      delete_res =
        package.s3_bucket_name
        |> ExAws.S3.delete_object(package.s3_bucket_path)
        |> ExAws.request()

      with {:ok, %{status_code: 200}} <- delete_res do
        {:ok, nil}
      else
        _ -> {:error, changes}
      end
    end)
    |> Repo.transaction()
  end
end
