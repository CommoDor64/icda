.PHONY: all deploy start env-setup

DFX_ENV = $(HOME)/.local/share/dfx/env
CARGO_ENV = $(HOME)/.cargo/env

# Helper target to source environment files
env-setup:
	@source $(DFX_ENV) && source $(CARGO_ENV)

start: env-setup
	@echo "Starting dfx in the background..."
	@dfx start --background --clean

deploy: env-setup
	@echo "Deploying dfx project..."
	@dfx deploy

all: start deploy
