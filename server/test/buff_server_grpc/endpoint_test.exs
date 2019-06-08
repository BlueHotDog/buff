defmodule BuffServerGrpc.EndpointTest do
  use ExUnit.Case

  describe "endpoint" do
    test "should start correctly" do
      assert {:ok, _pid, 50_051} = GRPC.Server.start_endpoint(BuffServerGrpc.Endpoint, 50_051)
    end
  end
end
