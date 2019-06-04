defmodule PuffServer.Repo do
  use Ecto.Repo,
    otp_app: :puff_server,
    adapter: Ecto.Adapters.Postgres
end
