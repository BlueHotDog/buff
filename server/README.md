# BuffServer

To start your Phoenix server:

  * Install dependencies with `mix deps.get`
  * Create and migrate your database with `mix ecto.setup`
  * Install Node.js dependencies with `cd assets && npm install`
  * Start Phoenix endpoint with `mix phx.server`

Now you can visit [`localhost:4000`](http://localhost:4000) from your browser.

Ready to run in production? Please [check our deployment guides](https://hexdocs.pm/phoenix/deployment.html).

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

