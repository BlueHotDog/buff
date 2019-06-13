defmodule BuffServer.Repo.Migrations.AddBucketToPackages do
  use Ecto.Migration

  def change do
    alter table(:packages) do
      add(:s3_bucket_name, :string)
      add(:s3_bucket_path, :string)
    end
  end
end
