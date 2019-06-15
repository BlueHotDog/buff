defmodule BuffServerGrpc.LoginRequest do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          email: String.t(),
          password: String.t()
        }
  defstruct [:email, :password]

  field :email, 1, type: :string
  field :password, 2, type: :string
end

defmodule BuffServerGrpc.LoginResponse do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          token: String.t()
        }
  defstruct [:token]

  field(:token, 1, type: :string)
end

defmodule BuffServerGrpc.AuthService.Service do
  @moduledoc false
  use GRPC.Service, name: "buff_server_grpc.AuthService"

  rpc(:Login, BuffServerGrpc.LoginRequest, BuffServerGrpc.LoginResponse)
end

defmodule BuffServerGrpc.AuthService.Stub do
  @moduledoc false
  use GRPC.Stub, service: BuffServerGrpc.AuthService.Service
end
