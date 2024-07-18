# Start with a base image that has Go installed
FROM golang:1.17 AS builder

# Set the current working directory inside the container
WORKDIR /app

# Copy the Go module files to download dependencies
COPY go.mod go.sum ./

# Download Go modules specified in go.mod and go.sum
RUN go mod download

# Copy the rest of the application source code
COPY . .

# Build the Go application
RUN go build -o goservices .

# Use a minimal base image to reduce the final image size
FROM debian:buster-slim

# Set the current working directory inside the container
WORKDIR /app

# Copy the built executable from the builder stage
COPY --from=builder /app/backend_engine .

# Expose the port that the application listens on
EXPOSE 8080

# Command to run the executable
CMD ["./backend_engine"]
