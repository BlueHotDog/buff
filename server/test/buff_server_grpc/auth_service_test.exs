defmodule BuffServerGrpc.AuthServiceTest do
  use BuffServer.DataCase

  alias BuffServer.Authentication.Token, as: AuthToken
  alias BuffServerGrpc.AuthService.Server, as: AuthServer

  describe "AuthService" do
    setup :setup_user_fixture

    test "should return an error for incorrect username" do
      assert_raise(GRPC.RPCError, fn ->
        AuthServer.login(%{username: "a", password: "c"}, nil)
      end)
    end

    test "should return an error for incorrect password", %{
      user: user,
      user_params: user_params
    } do
      assert_raise(GRPC.RPCError, fn ->
        AuthServer.login(
          %{username: user.username, password: user_params.password <> "a"},
          nil
        )
      end)
    end

    test "should return a valid token for correct credentials", %{
      user: user,
      user_params: user_params
    } do
      %BuffServerGrpc.LoginResponse{token: token} =
        AuthServer.login(
          %{username: user_params.username, password: user_params.password},
          nil
        )

      user_id = user.id
      assert {:ok, %{"user_id" => ^user_id}} = AuthToken.verify_and_validate(token)
    end
  end
end
