# Flink IoT Temperature Monitoring Example

This project demonstrates a simple IoT temperature monitoring system using Apache Flink.

## Prerequisites

You only need to have Docker and Docker Compose installed on your system:
- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/install/)

## Running the Application

1. Clone this repository:
```bash
# If using HTTPS (recommended)
git clone https://github.com/username/repository-name.git

# OR if using SSH
git clone git@github.com:username/repository-name.git

# Then change to the project directory
cd repository-name
```

2. Start the application using Docker Compose:
```bash
docker-compose up --build
```

This will:
- Start a Flink JobManager
- Start a Flink TaskManager
- Build and run the temperature monitoring application

3. View the Flink Dashboard:
- Open your browser and go to http://localhost:8081
- You can see your running job and its metrics

## Project Structure

- `FlinkIoTExample.java` - Main application code
- `pom.xml` - Maven dependencies and build configuration
- `Dockerfile` - Container configuration for the application
- `docker-compose.yml` - Multi-container Docker configuration

## How it Works

The application:
1. Generates sample temperature readings from two room sensors
2. Processes the readings to detect if temperatures are outside the safe range (15°C - 30°C)
3. Outputs warnings for any temperature readings outside this range

## Output

You can view the output in the Docker logs. The application will show messages like:
- Normal temperature readings
- Warnings for high temperatures (above 30°C)
- Warnings for low temperatures (below 15°C)

## Stopping the Application

To stop the application:
```bash
docker-compose down
``` 