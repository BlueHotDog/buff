defmodule BuffServer.AccountsTest do
  use BuffServer.DataCase, async: false
  import Mox

  alias BuffServer.Accounts
  setup :set_mox_from_context

  describe "users" do
    setup :setup_user_fixture
    setup :verify_on_exit!

    alias BuffServer.Accounts.User

    @update_attrs params_for(:user, %{
                    password: "some updated encrypted_password",
                    full_name: "some updated full_name",
                    private_email: "ohMy@email.com",
                    public_email: "wow@gmail.com",
                    username: "oh_diff_username"
                  })

    @invalid_attrs %{
      encrypted_password: nil,
      full_name: nil,
      private_email: nil,
      public_email: nil,
      username: nil
    }

    test "list_users/0 returns all users", %{user_params: user_params} do
      list_users = Accounts.list_users()
      assert length(list_users) == 1
      user = hd(list_users)
      refute user.password_hash == nil
      assert user.username == user_params.username
    end

    test "get_user!/1 returns the user with given id", %{
      user: user
    } do
      assert Accounts.get_user!(user.id) == user
    end

    test "create_user/1 with valid data creates a user", %{user: user, user_params: user_params} do
      refute user.password_hash == nil
      assert user.full_name == user_params.full_name
      assert user.private_email == user_params.private_email
      assert user.public_email == user_params.public_email
      assert user.username == user_params.username
    end

    test "create_user/1 with invalid data returns error changeset" do
      assert {:error, %Ecto.Changeset{}} = Accounts.create_user(@invalid_attrs)
    end

    test "update_user/2 with valid data updates the user", %{
      user: user
    } do
      assert {:ok, %User{} = updated_user} = Accounts.update_user(user, @update_attrs)
      refute updated_user.password_hash == nil
      assert updated_user.full_name == @update_attrs.full_name
      assert updated_user.private_email == @update_attrs.private_email
      assert updated_user.public_email == @update_attrs.public_email
      assert updated_user.username == user.username
    end

    test "update_user/2 with invalid data returns error changeset", %{
      user: user
    } do
      assert {:error, %Ecto.Changeset{}} = Accounts.update_user(user, @invalid_attrs)
      assert user == Accounts.get_user!(user.id)
    end

    test "delete_user/1 deletes the user", %{
      user: user
    } do
      assert {:ok, %User{}} = Accounts.delete_user(user)
      assert_raise Ecto.NoResultsError, fn -> Accounts.get_user!(user.id) end
    end

    test "change_user/1 returns a user changeset", %{
      user: user
    } do
      assert %Ecto.Changeset{} = Accounts.change_user(user)
    end
  end

end
