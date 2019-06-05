defmodule PuffServer.AuthenticationTest do
  use PuffServer.DataCase, async: true

  alias PuffServer.Accounts
  alias PuffServer.Authentication

  @test_user params_for(:user)
  describe "authentication" do
    def user_fixture(attrs \\ %{}) do
      {:ok, user} =
        attrs
        |> Enum.into(@test_user)
        |> Accounts.create_user()

      user
    end

    test "authenticate/2 throws if username doesnt exists" do
      assert_raise Ecto.NoResultsError, fn ->
        Authentication.authenticate!(
          "some_non_existing_username",
          "does it really matter?"
        )
      end
    end

    test "authenticate/2 throws when given incorrect password " do
      user = user_fixture()

      assert_raise MatchError, fn ->
        Authentication.authenticate!(
          user.username,
          @test_user.password <> "extra"
        )
      end
    end

    test "authenticate/2 returns token with everything is ok" do
      user = user_fixture()

      {:ok, token} =
        Authentication.authenticate!(
          user.username,
          @test_user.password
        )

      user_id = user.id

      assert {:ok, %{"user_id" => ^user_id}} =
               PuffServer.Authentication.Token.verify_and_validate(token)
    end
  end
end
