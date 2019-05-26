defmodule PuffServerWeb.PageController do
  use PuffServerWeb, :controller

  def index(conn, _params) do
    render(conn, "index.html")
  end
end
