# BuffServer

## To start your Phoenix server:

  * Install dependencies with `mix deps.get`
  * Create and migrate your database with `mix ecto.setup`
  * Install Node.js dependencies with `cd assets && npm install`
  * Start Phoenix endpoint with `mix phx.server`

Now you can visit [`localhost:4000`](http://localhost:4000) from your browser.

Ready to run in production? Please [check our deployment guides](https://hexdocs.pm/phoenix/deployment.html).

## To start your GRPC server:

  * Follow steps to run Phoenix
  * Start GRPC server with `mix grpc.server`

Now you can access your GRPC server at `localhost:50051`


## Regenerate Elixir GRPC stubs:

* Make sure you've [protobuf](https://github.com/protocolbuffers/protobuf) installed
* Make sure you've the Elixir protobuf plugin installed: `mix escript.install hex protobuf`
* From this folder run: `protoc --elixir_out=plugins=grpc:./lib/buff_server_grpc/ -I .. ../protobuffers/*.proto`

## Learn more

  * Official website: http://www.phoenixframework.org/
  * Guides: https://hexdocs.pm/phoenix/overview.html
  * Docs: https://hexdocs.pm/phoenix
  * Mailing list: http://groups.google.com/group/phoenix-talk
  * Source: https://github.com/phoenixframework/phoenix



## Stuff

- .buffignore
- Orgs
- Teams
- Accounts
- Package
  - Access
  - Name - x
  - Description - x
  - Keywords - x
  - Homepage - x
  - Bugs
  - License?
  - Contributors
    - Name
    - Email
    - URL
  - Repository - x
    - Type
    - URL
    - Directory - For mono-repo support
  - Dependencies
    - name:path
  - isPrivate

## Minio - S3 in development

* Start with `docker-compose up -d` in `./server`
* Example: `ExAws.S3.put_object("buff-packages-development", "/1", "123") |> ExAws.request!`

