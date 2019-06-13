defmodule BuffServer.PackagesTest do

  @s3_bucket_name Application.get_env(:buff_server, :s3_bucket_name)
  @dummy_artifact File.read("test/buff_server/fixtures/dummy_artifact.gz")
  @dummy_artifact2 File.read("test/buff_server/fixtures/dummy_artifact2.gz")
  use BuffServer.DataCase, async: true
  import Mox

  alias BuffServer.Packages

  describe "packages" do
    setup :verify_on_exit!

    def artifact_fixture(file) do
      {:ok, artifact_binary} = file
      artifact_binary
    end

    def default_attrs() do
      %{
        description: "some description",
        homepage: "some homepage",
        keywords: [],
        name: "some-name",
        repository_url: "http://aaa.com",
        artifact_binary: artifact_fixture(@dummy_artifact)
      }
    end

    def attrs(attrs \\ %{}) do
      attrs
      |> Enum.into(default_attrs())
    end

    @invalid_attrs %{
      description: nil,
      homepage: nil,
      keywords: nil,
      name: nil,
      repository_url: nil
    }

    def package_fixture(attrs) do
      ExAws.Request.HttpMock
      |> expect(:request, fn _method, _url, _body, _headers, _opts -> {:ok, %{status_code: 200}} end)

      attrs
      |> Packages.create_package!()
    end

    test "create_package/2 with no binary artifact fails" do
      assert_raise FunctionClauseError, fn ->
        attrs(%{artifact_binary: nil})
        |> Packages.create_package!()
      end
    end

    test "create_package/2 with valid data creates a package" do
      {:ok, package} = package_fixture(attrs())
      assert package.name == attrs().name
      assert package.description == attrs().description
      assert package.homepage == attrs().homepage
      assert package.keywords == attrs().keywords
      assert package.repository_url == attrs().repository_url
      assert package.s3_bucket_name == @s3_bucket_name
      assert package.s3_bucket_path == "/#{attrs().name}/artifact"
    end

    test "list_packages/0 returns all packages" do
      {:ok, package} = package_fixture(attrs())
      assert Packages.list_packages() == [package]
    end

    test "get_package!/1 returns the package with given id" do
      {:ok, package} = package_fixture(attrs())
      assert Packages.get_package!(package.id) == package
    end

    test "create_package/1 with invalid data returns error changeset" do
      {:error, changeset} = package_fixture(attrs(@invalid_attrs))
      assert changeset.valid? == false
    end

    test "delete_package/1 deletes the package" do
      {:ok, package} = package_fixture(attrs())
      ExAws.Request.HttpMock
      |> expect(:request, fn _method, _url, _body, _headers, _opts -> {:ok, %{status_code: 200}} end)
      Packages.delete_package!(package)
      assert_raise Ecto.NoResultsError, fn -> Packages.get_package!(package.id) end
    end

    # test "change_package/1 returns a package changeset" do
    #   {:ok, package} = package_fixture(attrs())
    #   assert %Ecto.Changeset{} = Packages.change_package(package)
    # end
  end
end
