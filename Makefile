SHELL := /bin/bash

.PHONY: all deploy start

DFX_ENV = $(HOME)/.local/share/dfx/env
CARGO_ENV = $(HOME)/.cargo/env

start: 
	@echo "Starting dfx in the background..."
	@source $(DFX_ENV) && source $(CARGO_ENV) && dfx start --background --clean

deploy:
	@echo "Deploying dfx project..."
	@source $(DFX_ENV) && source $(CARGO_ENV) && dfx deploy

all: start deploy
