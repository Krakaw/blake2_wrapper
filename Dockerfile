# Use the latest Rust image for building the Python module
FROM rust:latest as builder

# Install Python and required tools
RUN apt-get update && \
    apt-get install -y python3 python3-pip python3-venv && \
    pip3 install maturin --break-system-packages

# Create a new directory for the project
WORKDIR /app

# Copy all files from current directory into /app in the container
COPY . .

# Build the Python library using maturin
RUN maturin build --release --manylinux=off
RUN mkdir -p /wheels \
    && mv /app/target/wheels/*.whl /wheels/ \
    &&  pip install /wheels/*.whl --break-system-packages

# Test that the module is correctly installed
CMD ["python3", "-c", "import blake2_wrapper; print('Blake2 wrapper module installed successfully.')"]
