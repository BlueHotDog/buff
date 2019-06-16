defmodule BuffServerGrpc.AuthServiceTest do
  use BuffServerGrpc.IntegrationCase
  use BuffServer.DataCase

  alias BuffServer.Authentication.Token, as: AuthToken
  alias BuffServerGrpc.AuthService
  alias BuffServerGrpc.AuthService.Server, as: AuthServer

  describe "AuthService" do
    setup :setup_user_fixture

    test "should return an error for incorrect email" do
      get_client(AuthServer, fn channel ->
        login_req = BuffServerGrpc.LoginRequest.new(email: "a", password: "b")

        assert {:error, %Elixir.GRPC.RPCError{message: "Incorrect credentials", status: 16}} ==
                 AuthService.Stub.login(channel, login_req)
      end)
    end

    test "should return an error for incorrect password", %{
      user: user,
      user_params: user_params
    } do
      login_req = BuffServerGrpc.LoginRequest.new(email: user.email, password: user_params.password <> "a")

      get_client(AuthServer, fn channel ->
        assert {:error, %Elixir.GRPC.RPCError{message: "Incorrect credentials", status: 16}} ==
                 AuthService.Stub.login(channel, login_req)
      end)
    end

    test "should return a valid token for correct credentials", %{
      user: user,
      user_params: user_params
    } do
      login_req = BuffServerGrpc.LoginRequest.new(email: user_params.email, password: user_params.password)

      get_client(AuthServer, fn channel ->
        assert {:ok, %BuffServerGrpc.LoginResponse{token: token}} = AuthService.Stub.login(channel, login_req)

        user_id = user.id
        assert {:ok, %{"user_id" => ^user_id}} = AuthToken.verify_and_validate(token)
      end)
    end
  end
end
