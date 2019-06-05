defmodule BuffServer.Repo.Migrations.CreatePackages do
  use Ecto.Migration

  def change do
    create table(:packages, primary_key: false) do
      add(:id, :binary_id, primary_key: true)
      add(:name, :string, null: false)
      add(:description, :string)
      add(:keywords, {:array, :string}, default: fragment("ARRAY['']"))
      add(:homepage, :string)
      add(:repository_url, :string)
      add(:owner_user_id, references(:users, on_delete: :restrict, type: :binary_id))

      timestamps()
    end

    create(unique_index(:packages, [:name]))
    create(index(:packages, [:owner_user_id]))
  end
end
