defmodule BuffServerGrpc.RegistryService.Server do
  @moduledoc """
  Implementation of GRPC Registry service
  """
  use GRPC.Server, service: BuffServerGrpc.RegistryService.Service

  def publish(%{artifact: artifact}, _stream) do
    package_attributes = get_package_attributes_from_artifact(artifact)
    attrs = package_attributes |> Map.merge(%{artifact_binary: artifact})
    {:ok, package} = BuffServer.Packages.create_package(attrs)
    BuffServerGrpc.PublishResponse.new(result: true)
  end

  defp get_package_attributes_from_artifact(artifact) do
    {:ok, package_file} = :erl_tar.extract({:binary, artifact}, [:memory, :compressed, {:files, ['buff.toml']}])
    {_, buff_toml} = List.first(package_file)
    {:ok, package} = Toml.decode(buff_toml, keys: :atoms)
    package[:package]
  end
end
