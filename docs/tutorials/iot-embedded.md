# IoT & Embedded Development Tutorial

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
import std.embedded.gpio
import std.time

const LED_PIN = 2

main:
    # Configure GPIO
    led = gpio.Pin(LED_PIN, mode=gpio.OUTPUT)

    # Blink loop
    while True:
        led.high()
        time.sleep_ms(1000)
        led.low()
        time.sleep_ms(1000)
```

### Reading Sensors

```gul
import std.embedded.sensors

# Temperature sensor (DHT22)
dht = sensors.DHT22(pin=4)

main:
    while True:
        temp = dht.read_temperature()
        humidity = dht.read_humidity()

        print(f"Temperature: {temp}°C")
        print(f"Humidity: {humidity}%")

        time.sleep(2)
```

## 🌐 IoT Communication

### WiFi Connection

```gul
import std.embedded.wifi

wifi = wifi.WiFi()

main:
    # Connect to network
    wifi.connect(
        ssid=env("WIFI_SSID"),
        password=env("WIFI_PASSWORD")
    )

    if wifi.is_connected():
        print(f"Connected! IP: {wifi.ip_address()}")
```

### MQTT Messaging

```gul
import std.embedded.mqtt

mqtt = mqtt.Client(
    broker="mqtt.example.com",
    port=1883,
    client_id="my-device"
)

@mqtt.on_connect
fn on_connect():
    print("Connected to MQTT broker")
    mqtt.subscribe("sensors/temperature")

@mqtt.on_message
fn on_message(topic, payload):
    print(f"{topic}: {payload}")

main:
    mqtt.connect()

    # Publish sensor data
    while True:
        temp = read_temperature()
        mqtt.publish("sensors/temp", temp)
        time.sleep(60)
```

### HTTP API Integration

```gul
import std.http
import std.embedded.sensors

main:
    # Read sensor
    dht = sensors.DHT22(pin=4)
    temp = dht.read_temperature()

    # Send to cloud
    response = http.post("https://api.example.com/data", json={
        "device_id": "esp32-001",
        "temperature": temp,
        "timestamp": time.now()
    })

    print(f"Sent: {response.status_code}")
```

## 🎯 Complete IoT Project

```gul
import std.embedded.gpio
import std.embedded.sensors
import std.embedded.wifi
import std.embedded.mqtt

# Configuration
LED_PIN = 2
DHT_PIN = 4
MQTT_BROKER = "mqtt.example.com"

main:
    # Setup hardware
    led = gpio.Pin(LED_PIN, mode=gpio.OUTPUT)
    dht = sensors.DHT22(pin=DHT_PIN)

    # Connect WiFi
    wifi = wifi.WiFi()
    wifi.connect(env("WIFI_SSID"), env("WIFI_PASSWORD"))

    #Connect MQTT
    mqtt = mqtt.Client(broker=MQTT_BROKER)
    mqtt.connect()

    # Main loop
    while True:
        # Read sensors
        temp = dht.read_temperature()
        humidity = dht.read_humidity()

        # Publish data
        mqtt.publish("sensors/temperature", temp)
        mqtt.publish("sensors/humidity", humidity)

        # Blink LED to indicate activity
        led.high()
        time.sleep_ms(100)
        led.low()

        # Wait before next reading
        time.sleep(60)
```

---

**Last Updated**: 2025-12-10  
**Version**: 1.0.0  
**License**: MIT
