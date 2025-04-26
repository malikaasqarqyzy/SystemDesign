FROM flink:1.18.1-java11

WORKDIR /app
COPY FlinkIoTExample.java /app/

RUN apt-get update && \
    apt-get install -y maven

COPY pom.xml /app/
RUN mvn clean package

CMD ["flink", "run", "target/flink-iot-example-1.0-SNAPSHOT.jar"] 