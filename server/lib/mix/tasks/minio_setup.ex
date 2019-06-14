defmodule Mix.Tasks.Minio.Setup do
  @moduledoc """
  Creates the relevant S3 buckets in Minio to allow local/test development
  """
  use Mix.Task

  @shortdoc "Creates the required buckets in Minio"
  def run(_) do
    region = Application.get_env(:ex_aws, :s3)[:region]
    s3_bucket_name = Application.get_env(:buff_server, :s3_bucket_name)

    Mix.Task.run("app.start")
    Mix.shell().info("Creating bucket #{s3_bucket_name}...")

    case s3_bucket_name |> ExAws.S3.put_bucket(region) |> ExAws.request() do
      {:ok, _} -> :ok
      {:error, {:http_error, 409, _}} -> :ok
    end

    Mix.shell().info("Successfully created #{s3_bucket_name}!")
  end
end

# TODO(itay): Danni try to figure out why MIX_ENV=test mix minio.setup doesn't work with it
# defmodule Mix.Tasks.Minio.Setup do
#   use Mix.Task

#   @shortdoc "Creates the required buckets in Minio"
#   def run(_) do
#     region = Application.get_env(:ex_aws, :s3)[:region]
#     s3_bucket_name = Application.get_env(:buff_server, :s3_bucket_name)

#     Mix.Task.run("app.start")
#     Mix.shell.info("Creating bucket #{s3_bucket_name}...")

#     case ExAws.S3.put_bucket(s3_bucket_name, region) |> ExAws.request do
#       {:ok, _} -> :ok
#       {:error, {:http_error, 409, _}} -> :ok
#     end
#     Mix.shell.info("Successfully created #{s3_bucket_name}!")
#   end
# end
