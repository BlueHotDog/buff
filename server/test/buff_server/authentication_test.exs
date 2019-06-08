defmodule BuffServer.AuthenticationTest do
  use BuffServer.DataCase, async: true

  alias BuffServer.Authentication
  alias BuffServer.Authentication.Token, as: AuthToken

  describe "authentication" do
    setup :setup_user_fixture

    test "authenticate/2 throws if username doesnt exists" do
      assert_raise Ecto.NoResultsError, fn ->
        Authentication.authenticate!(
          "some_non_existing_username",
          "does it really matter?"
        )
      end
    end

    test "authenticate/2 throws when given incorrect password ", %{
      user: user,
      user_params: user_params
    } do
      assert_raise MatchError, fn ->
        Authentication.authenticate!(
          user.username,
          user_params.password <> "extra"
        )
      end
    end

    test "authenticate/2 returns token with everything is ok", %{
      user: user,
      user_params: user_params
    } do
      {:ok, token} =
        Authentication.authenticate!(
          user.username,
          user_params.password
        )

      user_id = user.id

      assert {:ok, %{"user_id" => ^user_id}} = AuthToken.verify_and_validate(token)
    end
  end
end
