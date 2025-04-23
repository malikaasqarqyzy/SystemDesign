import org.apache.flink.streaming.api.environment.StreamExecutionEnvironment;
import org.apache.flink.streaming.api.functions.ProcessFunction;
import org.apache.flink.streaming.api.datastream.DataStream;
import org.apache.flink.util.Collector;
import java.io.Serializable;

// Data class to represent sensor readings
public class FlinkIoTExample {
    // Sensor data class
    public static class SensorData implements Serializable {
        private String deviceId;
        private double value;
        private long timestamp;

        public SensorData(String deviceId, double value, long timestamp) {
            this.deviceId = deviceId;
            this.value = value;
            this.timestamp = timestamp;
        }

        public String getDeviceId() { return deviceId; }
        public double getValue() { return value; }
        public long getTimestamp() { return timestamp; }


        public String toString() {
            return "SensorData{deviceId='" + deviceId + "', value=" + value + ", timestamp=" + timestamp + "}";
        }
    }

    // Alert class for anomaly notifications
    public static class Alert implements Serializable {
        private String deviceId;
        private double anomalyValue;
        private long timestamp;

        public Alert(String deviceId, double anomalyValue, long timestamp) {
            this.deviceId = deviceId;
            this.anomalyValue = anomalyValue;
            this.timestamp = timestamp;
        }

  
        public String toString() {
            return "Alert{deviceId='" + deviceId + "', anomalyValue=" + anomalyValue + ", timestamp=" + timestamp + "}";
        }
    }

    // Simple anomaly detector that flags values above a threshold
    public static class SimpleAnomalyDetector extends ProcessFunction<SensorData, Alert> {
        private static final double THRESHOLD = 50.0; // Threshold for anomaly detection

        public void processElement(SensorData sensor, Context context, Collector<Alert> collector) {
            if (sensor.getValue() > THRESHOLD) {
                collector.collect(new Alert(
                    sensor.getDeviceId(),
                    sensor.getValue(),
                    sensor.getTimestamp()
                ));
            }
        }
    }

    public static void main(String[] args) throws Exception {
        // Set up the streaming execution environment
        final StreamExecutionEnvironment env = StreamExecutionEnvironment.getExecutionEnvironment();

        // Generate test data
        DataStream<SensorData> sensorData = env.fromElements(
            new SensorData("sensor1", 25.0, System.currentTimeMillis()),
            new SensorData("sensor2", 30.0, System.currentTimeMillis()),
            new SensorData("sensor1", 75.0, System.currentTimeMillis()), // This will trigger an alert
            new SensorData("sensor2", 28.0, System.currentTimeMillis()),
            new SensorData("sensor1", 40.0, System.currentTimeMillis())
        );

        // Process the sensor data and detect anomalies
        DataStream<Alert> alerts = sensorData
            .keyBy(SensorData::getDeviceId)
            .process(new SimpleAnomalyDetector());

        // Print the alerts
        alerts.print();

        // Execute the job
        env.execute("IoT Anomaly Detection Example");
    }
}