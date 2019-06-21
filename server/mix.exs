defmodule BuffServer.MixProject do
  use Mix.Project

  def project do
    [
      app: :buff_server,
      version: "0.1.0",
      elixir: "~> 1.8",
      elixirc_paths: elixirc_paths(Mix.env()),
      compilers: [:phoenix, :gettext] ++ Mix.compilers(),
      start_permanent: Mix.env() == :prod,
      aliases: aliases(),
      deps: deps(),
      test_coverage: [tool: ExCoveralls],
      dialyzer: [plt_add_apps: [:mix]],
      preferred_cli_env: [
        coveralls: :test,
        "coveralls.detail": :test,
        "coveralls.post": :test,
        "coveralls.html": :test,
        "coveralls.json": :test
      ]
    ]
  end

  # Configuration for the OTP application.
  #
  # Type `mix help compile.app` for more information.
  def application do
    [
      mod: {BuffServer.Application, []},
      extra_applications: [:logger, :runtime_tools]
    ]
  end

  # Specifies which paths to compile per environment.
  defp elixirc_paths(:test), do: ["lib", "test/support"]
  defp elixirc_paths(_), do: ["lib"]

  # Specifies your project dependencies.
  #
  # Type `mix help deps` for examples and options.
  defp deps do
    [
      {:phoenix, "~> 1.4.6"},
      {:phoenix_pubsub, "~> 1.1"},
      {:phoenix_ecto, "~> 4.0"},
      {:ecto_sql, "~> 3.0"},
      {:postgrex, ">= 0.0.0"},
      {:phoenix_html, "~> 2.11"},
      {:gettext, "~> 0.11"},
      {:jason, "~> 1.0"},
      {:plug_cowboy, "~> 2.0"},
      {:argon2_elixir, "~> 2.0"},
      {:comeonin, "~> 5.1"},
      {:joken, "~> 2.0"},
      {:ex_aws_s3, "~> 2.0"},
      {:toml, "~> 0.5.2"},
      # required by ex_aws_s3
      {:ex_aws, "~> 2.0"},
      # required by ex_aws_s3
      {:hackney, "~> 1.9"},
      {:protobuf, "~> 0.5.3"},
      # Only for files generated from Google's protos.
      # Can be ignored if you don't use Google's protos.
      {:google_protos, "~> 0.1"},
      # Overrides for GRPC to work..
      # more info here https://github.com/elixir-grpc/grpc/issues/100
      {:grpc, github: "tony612/grpc-elixir"},
      {:cowboy, "~> 2.5", [env: :prod, hex: "cowboy", repo: "hexpm", optional: false, override: true]},
      {:cowlib, "~> 2.7.3", [env: :prod, hex: "cowlib", repo: "hexpm", optional: false, override: true]},
      {:gun, "~> 1.3", override: true},
      # Test/Dev stuff
      {:mix_test_watch, "~> 0.8", only: :dev, runtime: false},
      {:dialyxir, "~> 1.0.0-rc.6", only: [:dev, :test], runtime: false},
      {:phoenix_live_reload, "~> 1.2", only: :dev},
      {:ex_unit_notifier, "~> 0.1", only: :test},
      {:credo, "~> 1.0.0", only: [:dev, :test], runtime: false},
      {:apex, "~>1.2.1", only: [:dev, :test]},
      {:mox, "~> 0.5", only: :test},
      {:faker, "~> 0.12", only: :test},
      {:ex_machina, "~> 2.3", only: :test},
      {:excoveralls, "~> 0.10", only: :test},
      {:junit_formatter, "~> 3.0", only: :test},
      {:inch_ex, only: [:dev, :test]}
    ]
  end

  # Aliases are shortcuts or tasks specific to the current project.
  # For example, to create, migrate and run the seeds file at once:
  #
  #     $ mix ecto.setup
  #
  # See the documentation for `Mix` for more info on aliases.
  defp aliases do
    [
      "ecto.setup": ["ecto.create", "ecto.migrate", "run priv/repo/seeds.exs"],
      "ecto.reset": ["ecto.drop", "ecto.setup"],
      test: ["ecto.create --quiet", "ecto.migrate", "minio.setup", "test"]
    ]
  end
end
