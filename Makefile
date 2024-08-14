term:
	gnome-terminal -- bash -c "docker exec -it my_postgres psql -U qzan -d rust_sqlx; exec bash"
run:
	docker-compose up -d
	cargo run
all:
	docker-compose up -d
	gnome-terminal -- bash -c "docker exec -it my_postgres psql -U qzan -d rust_sqlx; exec bash"
	cargo run
