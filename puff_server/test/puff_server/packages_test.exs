defmodule PuffServer.PackagesTest do
  use PuffServer.DataCase, async: true

  alias PuffServer.Packages

  describe "packages" do
    alias PuffServer.Packages.Package

    @valid_attrs %{
      description: "some description",
      homepage: "some homepage",
      keywords: [],
      name: "some name",
      repository_url: "http://aaa.com"
    }
    @update_attrs %{
      description: "some updated description",
      homepage: "some updated homepage",
      keywords: [],
      name: "some updated name",
      repository_url: "http://bbb.com"
    }
    @invalid_attrs %{
      description: nil,
      homepage: nil,
      keywords: nil,
      name: nil,
      repository_url: nil
    }

    def package_fixture(attrs \\ %{}) do
      {:ok, package} =
        attrs
        |> Enum.into(@valid_attrs)
        |> Packages.create_package()

      package
    end

    test "list_packages/0 returns all packages" do
      package = package_fixture()
      assert Packages.list_packages() == [package]
    end

    test "get_package!/1 returns the package with given id" do
      package = package_fixture()
      assert Packages.get_package!(package.id) == package
    end

    test "create_package/1 with valid data creates a package" do
      assert {:ok, %Package{} = package} = Packages.create_package(@valid_attrs)
      assert package.description == "some description"
      assert package.homepage == "some homepage"
      assert package.keywords == []
      assert package.name == "some name"
      assert package.repository_url == "http://aaa.com"
    end

    test "create_package/1 with invalid data returns error changeset" do
      assert {:error, %Ecto.Changeset{}} = Packages.create_package(@invalid_attrs)
    end

    test "update_package/2 with valid data updates the package" do
      package = package_fixture()
      assert {:ok, %Package{} = package} = Packages.update_package(package, @update_attrs)
      assert package.description == "some updated description"
      assert package.homepage == "some updated homepage"
      assert package.keywords == []
      assert package.name == "some updated name"
      assert package.repository_url == "http://bbb.com"
    end

    test "update_package/2 with invalid data returns error changeset" do
      package = package_fixture()
      assert {:error, %Ecto.Changeset{}} = Packages.update_package(package, @invalid_attrs)
      assert package == Packages.get_package!(package.id)
    end

    test "delete_package/1 deletes the package" do
      package = package_fixture()
      assert {:ok, %Package{}} = Packages.delete_package(package)
      assert_raise Ecto.NoResultsError, fn -> Packages.get_package!(package.id) end
    end

    test "change_package/1 returns a package changeset" do
      package = package_fixture()
      assert %Ecto.Changeset{} = Packages.change_package(package)
    end
  end
end
