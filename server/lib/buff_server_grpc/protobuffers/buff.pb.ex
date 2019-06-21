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

  field :token, 1, type: :string
end

defmodule BuffServerGrpc.Package do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          name: String.t(),
          description: String.t(),
          homepage: String.t(),
          repository_url: String.t(),
          keywords: [String.t()]
        }
  defstruct [:name, :description, :homepage, :repository_url, :keywords]

  field :name, 1, type: :string
  field :description, 2, type: :string
  field :homepage, 3, type: :string
  field :repository_url, 4, type: :string
  field :keywords, 5, repeated: true, type: :string
end

defmodule BuffServerGrpc.PublishRequest do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          artifact: binary
        }
  defstruct [:artifact]

  field :artifact, 1, type: :bytes
end

defmodule BuffServerGrpc.PublishResponse do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          result: boolean
        }
  defstruct [:result]

  field :result, 1, type: :bool
end

defmodule BuffServerGrpc.AuthService.Service do
  @moduledoc false
  use GRPC.Service, name: "buff_server_grpc.AuthService"

  rpc :Login, BuffServerGrpc.LoginRequest, BuffServerGrpc.LoginResponse
end

defmodule BuffServerGrpc.AuthService.Stub do
  @moduledoc false
  use GRPC.Stub, service: BuffServerGrpc.AuthService.Service
end

defmodule BuffServerGrpc.RegistryService.Service do
  @moduledoc false
  use GRPC.Service, name: "buff_server_grpc.RegistryService"

  rpc :Publish, BuffServerGrpc.PublishRequest, BuffServerGrpc.PublishResponse
end

defmodule BuffServerGrpc.RegistryService.Stub do
  @moduledoc false
  use GRPC.Stub, service: BuffServerGrpc.RegistryService.Service
end
