.PHONY: app setup test.watch server console sh clean

app:
	docker-compose up --detach app

setup: app
	docker-compose exec app /wait-for-postgres.sh postgres mix ecto.setup

test.watch: app
	docker-compose exec app mix test.watch

server: setup
	docker-compose exec app mix phx.server

console: setup
	docker-compose exec app iex -S mix

sh: setup
	docker-compose exec app bash

clean:
	docker-compose down
