defmodule BuffServer.PackagesTest do
  use BuffServer.DataCase, async: true
  alias BuffServer.Packages
  import Mox

  describe "packages" do
    @s3_bucket_name Application.get_env(:buff_server, :s3_bucket_name)
    @dummy_artifact File.read("test/buff_server/fixtures/dummy_artifact.gz") |> elem(1)

    @default_attrs %{
      description: Faker.Lorem.Shakespeare.hamlet(),
      homepage: Faker.Internet.url(),
      keywords: Enum.reduce(1..10, [], fn _x, acc -> acc ++ [Faker.Company.En.buzzword()] end),
      name: Faker.Company.En.buzzword(),
      repository_url: Faker.Internet.url(),
      artifact_binary: @dummy_artifact
    }

    @invalid_attrs %{
      description: nil,
      homepage: nil,
      keywords: nil,
      name: nil,
      repository_url: nil,
      artifact_binary: @dummy_artifact
    }

    def package_fixture(attrs) do
      ExAws.Request.HttpMock
      |> stub(:request, fn _method, _url, _body, _headers, _opts ->
        {:ok, %{status_code: 200}}
      end)

      attrs |> Packages.create_package()
    end

    test "create_package/2 with no binary artifact fails" do
      assert_raise FunctionClauseError, fn ->
        %{@default_attrs | artifact_binary: nil} |> Packages.create_package()
      end
    end

    test "create_package/2 with valid data creates a package" do
      {:ok, package} = package_fixture(@default_attrs)
      assert package.name == @default_attrs.name
      assert package.description == @default_attrs.description
      assert package.homepage == @default_attrs.homepage
      assert package.keywords == @default_attrs.keywords
      assert package.repository_url == @default_attrs.repository_url
      assert package.s3_bucket_name == @s3_bucket_name
      assert package.s3_bucket_path == Packages.s3_bucket_path(@default_attrs)
    end

    test "list_packages/0 returns all packages" do
      {:ok, package} = package_fixture(@default_attrs)
      assert Packages.list_packages() == [package]
    end

    test "get_package!/1 returns the package with given id" do
      {:ok, package} = package_fixture(@default_attrs)
      assert Packages.get_package!(package.id) == package
    end

    test "create_package/1 with invalid data returns error changeset" do
      {:error, :package, changeset, _} = package_fixture(@invalid_attrs)
      assert changeset.valid? == false
    end

    test "create_package/1 doesnt create when failing to create artifact on s3" do
      ExAws.Request.HttpMock
      |> expect(:request, fn _method, _url, _body, _headers, _opts ->
        {:ok, %{status_code: 404}}
      end)

      assert {:error, :s3_artifact, _, _} = @default_attrs |> Packages.create_package()
      assert Packages.list_packages() == []
    end

    test "delete_package/1 deletes the package" do
      {:ok, package} = package_fixture(@default_attrs)

      ExAws.Request.HttpMock
      |> expect(:request, fn _method, _url, _body, _headers, _opts ->
        {:ok, %{status_code: 200}}
      end)

      assert {:ok, _} = Packages.delete_package(package)
      assert_raise Ecto.NoResultsError, fn -> Packages.get_package!(package.id) end
    end

    test "delete_package/1 fails if s3 delete fails" do
      {:ok, package} = package_fixture(@default_attrs)

      ExAws.Request.HttpMock
      |> expect(:request, fn _method, _url, _body, _headers, _opts ->
        {:ok, %{status_code: 404}}
      end)

      assert {:error, :s3_artifact, _, _} = Packages.delete_package(package)
      assert Packages.get_package!(package.id) == package
    end
  end
end
