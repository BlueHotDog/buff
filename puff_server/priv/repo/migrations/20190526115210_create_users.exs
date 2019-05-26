defmodule PuffServer.Repo.Migrations.CreateUsers do
  use Ecto.Migration

  def change do
    create table(:users, primary_key: false) do
      add(:id, :binary_id, primary_key: true)
      add(:full_name, :string)
      add(:public_email, :string)
      add(:private_email, :string)
      add(:username, :string)
      add(:encrypted_password, :string)
      add(:is_public_email_verified, :boolean, default: false)
      add(:is_private_email_verified, :boolean, default: false)

      timestamps()
    end

    create(unique_index(:users, [:username]))
  end
end
