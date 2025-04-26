import org.apache.flink.streaming.api.environment.StreamExecutionEnvironment;
import org.apache.flink.streaming.api.functions.ProcessFunction;
import org.apache.flink.streaming.api.datastream.DataStream;
import org.apache.flink.util.Collector;
import java.io.Serializable;

public class FlinkIoTExample {
    // Temperature reading data class
    public static class TemperatureReading implements Serializable {
        private String sensorId;
        private double temperature;
        private long timestamp;

        public TemperatureReading(String sensorId, double temperature, long timestamp) {
            this.sensorId = sensorId;
            this.temperature = temperature;
            this.timestamp = timestamp;
        }

        public String getSensorId() { return sensorId; }
        public double getTemperature() { return temperature; }
        public long getTimestamp() { return timestamp; }

        @Override
        public String toString() {
            return String.format("Sensor %s: %.1f°C at %d", sensorId, temperature, timestamp);
        }
    }

    // Temperature monitor to check if temperature is within safe range
    public static class TemperatureMonitor extends ProcessFunction<TemperatureReading, String> {
        private static final double HIGH_TEMP_THRESHOLD = 30.0;
        private static final double LOW_TEMP_THRESHOLD = 15.0;

        @Override
        public void processElement(TemperatureReading reading, Context context, Collector<String> collector) {
            if (reading.getTemperature() > HIGH_TEMP_THRESHOLD) {
                collector.collect(String.format("WARNING: High temperature detected for sensor %s: %.1f°C", 
                    reading.getSensorId(), reading.getTemperature()));
            } else if (reading.getTemperature() < LOW_TEMP_THRESHOLD) {
                collector.collect(String.format("WARNING: Low temperature detected for sensor %s: %.1f°C", 
                    reading.getSensorId(), reading.getTemperature()));
            } else {
                collector.collect(String.format("Normal temperature for sensor %s: %.1f°C", 
                    reading.getSensorId(), reading.getTemperature()));
            }
        }
    }

    public static void main(String[] args) throws Exception {
        // Set up the streaming execution environment
        final StreamExecutionEnvironment env = StreamExecutionEnvironment.getExecutionEnvironment();

        // Generate sample temperature readings
        DataStream<TemperatureReading> temperatureReadings = env.fromElements(
            new TemperatureReading("room-1", 22.5, System.currentTimeMillis()),
            new TemperatureReading("room-2", 31.0, System.currentTimeMillis()),
            new TemperatureReading("room-1", 14.0, System.currentTimeMillis()),
            new TemperatureReading("room-2", 25.5, System.currentTimeMillis()),
            new TemperatureReading("room-1", 23.0, System.currentTimeMillis())
        );

        // Process the temperature readings
        DataStream<String> monitoringResults = temperatureReadings
            .keyBy(TemperatureReading::getSensorId)
            .process(new TemperatureMonitor());

        // Print the results
        monitoringResults.print();

        // Execute the Flink job
        env.execute("Temperature Monitoring System");
    }
}