clean:
	docker rm -f icda || true

build:
	docker build -t icda .

run: clean build
	docker run -d --name icda --network=host icda

deploy:
	docker exec -it icda bash -c "dfx deploy icda_backend"

all: run deploy

