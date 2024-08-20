#!/bin/bash

# Default values
DEFAULT_THREADS=5
DEFAULT_CONNECTIONS=100
DEFAULT_DURATION="60s"

# Display help message
function show_help {
    echo "Usage: $0 {rust|python|go} [number_of_threads] [number_of_connections] [duration]"
    echo
    echo "Arguments:"
    echo "  rust, python, go       The language/environment to run the server."
    echo "  number_of_threads       Optional. Number of threads for wrk (default: $DEFAULT_THREADS)."
    echo "  number_of_connections   Optional. Number of connections for wrk (default: $DEFAULT_CONNECTIONS)."
    echo "  duration                Optional. Duration for wrk (default: $DEFAULT_DURATION)."
    echo
    echo "Examples:"
    echo "  $0 rust 10 200 120s     Run Rust server with 10 threads, 200 connections, for 120 seconds."
    echo "  $0 python               Run Python server with default values."
    echo "  $0 go 8 150             Run Go server with 8 threads and 150 connections for 60 seconds."
    exit 0
}

# Check for help option
if [[ "$1" == "--help" ]] || [[ "$1" == "-h" ]]; then
    show_help
fi

# Check if the correct number of arguments is provided
if [ $# -lt 1 ]; then
    show_help
fi

# Get the input arguments
LANGUAGE=$1
NUM_THREADS=${2:-$DEFAULT_THREADS}
NUM_CONNECTIONS=${3:-$DEFAULT_CONNECTIONS}
DURATION=${4:-$DEFAULT_DURATION}

# Validate number of threads and connections
if ! [[ "$NUM_THREADS" =~ ^[0-9]+$ ]] || ! [[ "$NUM_CONNECTIONS" =~ ^[0-9]+$ ]]; then
    echo "Error: Number of threads and connections must be positive integers."
    show_help
fi

# Validate duration format
if ! [[ "$DURATION" =~ ^[0-9]+[sm]$ ]]; then
    echo "Error: Duration must be in the format of a number followed by 's' (seconds) or 'm' (minutes)."
    show_help
fi

# Run the command based on the input
case $LANGUAGE in
    rust)
        echo "Running Rust command..."
        # Replace with the actual command you want to run for Rust
        cargo run --release &
        RUST_PID=$!
        sleep 5
        wrk -t"$NUM_THREADS" -c"$NUM_CONNECTIONS" -d"$DURATION" http://localhost:8080
        kill $RUST_PID
        ;;
    python)
        echo "Running Python command..."

        CONDA_SH="/opt/homebrew/anaconda3/etc/profile.d/conda.sh"

        # Initialize Conda
        source "$CONDA_SH"

        # Name of your Conda environment
        ENV_NAME="base"

        # Activate the Conda environment
        conda activate "$ENV_NAME"

        # Replace with the actual command you want to run for Python
        gunicorn --workers=$(sysctl -n hw.ncpu) --worker-class=gevent main:app --bind 0.0.0.0:8080 &

        # Store the PID of the Python server
        PYTHON_PID=$!

        sleep 5

        wrk -t"$NUM_THREADS" -c"$NUM_CONNECTIONS" -d"$DURATION" http://localhost:8080

        kill $PYTHON_PID
        ;;
    go)
        echo "Running Go command..."
        # Replace with the actual command you want to run for Go
        go run main.go &
        GO_PID=$!
        sleep 5
        #wrk -t"$NUM_THREADS" -c"$NUM_CONNECTIONS" -d"$DURATION" http://localhost:8080
        kill $GO_PID
        ;;
    *)
        echo "Unknown language: $LANGUAGE"
        show_help
        ;;
esac
