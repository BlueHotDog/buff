defmodule buffServer.Repo do
  use Ecto.Repo,
    otp_app: :buff_server,
    adapter: Ecto.Adapters.Postgres
end
