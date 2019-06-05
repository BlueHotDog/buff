defmodule BuffServerWeb.PageController do
  use BuffServerWeb, :controller

  def index(conn, _params) do
    render(conn, "index.html")
  end
end
