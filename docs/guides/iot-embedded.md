# Iot Embedded

**Version**: 0.13.0 | **Syntax**: v3.2 | **Updated**: 2025-12-28

---

## IoT & Embedded Development Tutorial

Build IoT applications and embedded systems with GUL.

## 🔌 Hardware Support

GUL supports:

- **Microcontrollers**: ESP32, STM32, Raspberry Pi Pico
- **Single-Board Computers**: Raspberry Pi, BeagleBone
- **Sensors**: Temperature, humidity, motion, light
- **Actuators**: Motors, LEDs, servos, relays

## 🚀 Getting Started

### ESP32 Blink LED

```gul
@imp std.embedded.gpio
@imp std.time

let LED_PIN = 2

mn:
    # Configure GPIO
    let led = gpio.Pin(LED_PIN, mode=gpio.OUTPUT)

    # Blink loop
    while true:
        led.high()
        time.sleep_ms(1000)
        led.low()
        time.sleep_ms(1000)
```

### Reading Sensors

```gul
@imp std.embedded.sensors

# Temperature sensor (DHT22)
let dht = sensors.DHT22(pin=4)

mn:
    while true:
        let temp = dht.read_temperature()
        let humidity = dht.read_humidity()

        print(f"Temperature: {temp}°C")
        print(f"Humidity: {humidity}%")

        time.sleep(2)
```

## 🌐 IoT Communication

### WiFi Connection

```gul
@imp std.embedded.wifi

let wifi_client = wifi.WiFi()

mn:
    # Connect to network
    wifi_client.connect(
        ssid=env("WIFI_SSID"),
        password=env("WIFI_PASSWORD")
    )

    if wifi_client.is_connected():
        print(f"Connected! IP: {wifi_client.ip_address()}")
```

### MQTT Messaging

```gul
@imp std.embedded.mqtt

let mqtt_client = mqtt.Client(
    broker="mqtt.example.com",
    port=1883,
    client_id="my-device"
)

@mqtt_client.on_connect
@fn on_connect():
    print("Connected to MQTT broker")
    mqtt_client.subscribe("sensors/temperature")

@mqtt_client.on_message
@fn on_message(topic, payload):
    print(f"{topic}: {payload}")

mn:
    mqtt_client.connect()

    # Publish sensor data
    while true:
        let temp = read_temperature()
        mqtt_client.publish("sensors/temp", temp)
        time.sleep(60)
```

### HTTP API Integration

```gul
@imp std.http
@imp std.embedded.sensors

mn:
    # Read sensor
    let dht = sensors.DHT22(pin=4)
    let temp = dht.read_temperature()

    # Send to cloud
    let response = http.post("https://api.example.com/data", json=@dict{
        device_id: "esp32-001",
        temperature: temp,
        timestamp: time.now()
    })

    print(f"Sent: {response.status_code}")
```

## 🎯 Complete IoT Project

```gul
@imp std.embedded.gpio
@imp std.embedded.sensors
@imp std.embedded.wifi
@imp std.embedded.mqtt

# Configuration
let LED_PIN = 2
let DHT_PIN = 4
let MQTT_BROKER = "mqtt.example.com"

mn:
    # Setup hardware
    let led = gpio.Pin(LED_PIN, mode=gpio.OUTPUT)
    let dht_sensor = sensors.DHT22(pin=DHT_PIN)

    # Connect WiFi
    let wifi_conn = wifi.WiFi()
    wifi_conn.connect(env("WIFI_SSID"), env("WIFI_PASSWORD"))

    #Connect MQTT
    let mqtt_conn = mqtt.Client(broker=MQTT_BROKER)
    mqtt_conn.connect()

    # Main loop
    while true:
        # Read sensors
        let temp = dht_sensor.read_temperature()
        let humidity = dht_sensor.read_humidity()

        # Publish data
        mqtt_conn.publish("sensors/temperature", temp)
        mqtt_conn.publish("sensors/humidity", humidity)

        # Blink LED to indicate activity
        led.high()
        time.sleep_ms(100)
        led.low()

        # Wait before next reading
        time.sleep(60)
```

---

**Last Updated**: 2025-12-28  
**Version: 0.13.0  
**License\*\*: MIT
