.PHONY: app server console sh clean

app:
	docker-compose up --detach --build app

setup: app
	docker-compose exec app /wait-for-postgres.sh postgres mix ecto.setup

server: setup
	docker-compose exec app mix phx.server

grpc: setup
	docker-compose exec app mix grpc

console: setup
	docker-compose exec app iex -S mix

sh: setup
	docker-compose exec app bash

clean:
	docker-compose down
