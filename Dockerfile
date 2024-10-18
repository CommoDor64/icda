FROM ubuntu:20.04

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        build-essential \
        jq \
        nodejs \
        npm \
        golang \
        curl \
	git \
        ca-certificates \
        make \
        libunwind8 && \
    rm -rf /var/lib/apt/lists/*

# Install Rust non-interactively
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

# Add wasm32 target for Rust without prompts
RUN rustup target add wasm32-unknown-unknown

# Set DFX version to avoid interactive prompts
ENV DFX_VERSION=latest

# Install DFX non-interactively
ENV DFX_VERSION=0.23.0
RUN DFXVM_INIT_YES=true sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
RUN . $HOME/.local/share/dfx/env
ENV PATH="/root/.local/share/dfx/bin:$PATH"

ENV PATH="/root/bin:${PATH}"

# Install Foundry non-interactively
RUN curl -L https://foundry.paradigm.xyz | bash -s -- --skip
RUN /root/.foundry/bin/foundryup

# Set the working directory
WORKDIR /icda

# Copy project files
COPY . /icda

# Install npm dependencies without interaction
RUN npm install --yes

# Expose DFX ports (default is 8000)
EXPOSE 8000

# Set environment variables for DFX and Cargo
ENV DFX_ENV=/root/.local/share/dfx/env
ENV CARGO_ENV=/root/.cargo/env

# Copy the entrypoint script into the image
COPY entrypoint.sh /usr/local/bin/entrypoint.sh

# Make the entrypoint script executable
RUN chmod +x /usr/local/bin/entrypoint.sh

# Set the command to run the entrypoint script
CMD ["/usr/local/bin/entrypoint.sh"]
