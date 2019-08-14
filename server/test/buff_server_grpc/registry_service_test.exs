defmodule BuffServerGrpc.BuffRegistryTest do
  use BuffServerGrpc.IntegrationCase
  use BuffServer.DataCase

  alias BuffServerGrpc.PublishResponse
  alias BuffServerGrpc.RegistryService
  alias BuffServerGrpc.RegistryService.Server, as: RegistryServer

  describe "RegistryService" do
    setup :setup_user_fixture

    @dummy_artifact File.read("test/buff_server_grpc/fixtures/dummy.tar.gz") |> elem(1)

    test "should return ok for a valid PublishRequest" do
      publish_req = BuffServerGrpc.PublishRequest.new(artifact: @dummy_artifact)
      ExAws.Request.HttpMock
      |> stub(:request, fn _method, _url, _body, _headers, _opts ->
        {:ok, %{status_code: 200}}
      end)
      get_client(RegistryServer, fn channel ->
        assert {:ok, %PublishResponse{result: true}} == RegistryService.Stub.publish(channel, publish_req)
      end)
    end

    test "should return error for an existing artifact" do
      publish_req = BuffServerGrpc.PublishRequest.new(artifact: @dummy_artifact)
      ExAws.Request.HttpMock
      |> stub(:request, fn _method, _url, _body, _headers, _opts ->
        {:ok, %{status_code: 200}}
      end)
      get_client(RegistryServer, fn channel ->
        RegistryService.Stub.publish(channel, publish_req)
        assert {:error, %GRPC.RPCError{message: "Internal Server Error", status: 2}} == RegistryService.Stub.publish(channel, publish_req)
      end)
    end

  end
end
