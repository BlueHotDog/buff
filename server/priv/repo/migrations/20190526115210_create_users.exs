defmodule BuffServer.Repo.Migrations.CreateUsers do
  use Ecto.Migration

  def change do
    create table(:users, primary_key: false) do
      add(:id, :binary_id, primary_key: true)
      add(:full_name, :string, null: false)
      add(:public_email, :string, null: false)
      add(:email, :string, null: false)
      add(:password_hash, :string, null: false)
      add(:is_public_email_verified, :boolean, default: false, null: false)
      add(:is_email_verified, :boolean, default: false, null: false)

      timestamps()
    end

    create(unique_index(:users, [:email]))
  end
end
