.PHONY: all start deploy restart

start:
	@echo "Starting dfx in the background..."
	@dfx start --background

deploy: start
	@echo "Deploying dfx project..."
	@dfx deploy

restart: deploy
	@echo "Stopping dfx..."
	@dfx stop
	@echo "Restarting dfx..."
	@dfx start

all: start deploy restart
