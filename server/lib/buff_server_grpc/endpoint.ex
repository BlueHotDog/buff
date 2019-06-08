defmodule BuffServerGrpc.Endpoint do
  use GRPC.Endpoint

  intercept(GRPC.Logger.Server)
  run(BuffServerGrpc.AuthService.Server)
end
