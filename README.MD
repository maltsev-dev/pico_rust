# 🧪 RP2040 Pico W Lab

**Experiments, Prototypes & Notes with the Raspberry Pi Pico W**

## 📌 Overview

This repository contains my projects, experiments, and notes using the **Raspberry Pi Pico W**, built on the **RP2040** microcontroller with integrated Wi-Fi.  
It serves as a playground for testing ideas, learning embedded concepts, and building small prototypes.

**Goals:**

* Explore RP2040 features with Pico SDK / Arduino / MicroPython / CircuitPython
* Build Wi-Fi-enabled IoT devices
* Interface with peripherals (GPIO, I2C, SPI, UART, ADC, PWM)
* Experiment with power management, timers, interrupts
* Integrate with sensors and modules (temperature, motion, light, etc.)

---

## ⚙️ Hardware Used

* Raspberry Pi Pico W
* Breadboard & jumper wires
* Sensors: DHT11, MPU6050, BH1750, etc.
* Modules: OLED, WS2812 (Neopixels), relay, MOSFET driver
* Power: USB / battery / external supply 

---

## 🧩 Projects & Experiments

| Project Name            | Description                         | Status         |
| ----------------------- | ----------------------------------- | -------------- |
| `blinky_led_by_button`  | External LED blink controlled by button           | ✅ Done         |
| `led_bar`               | Make led flow on 10 LED bar graph   | ✅ Done |
| `led_lamp`              | LED ON and LED OFF with button         | ✅ Done      |
| `led_analog`            | Experiments with PWM                | 🔄 In Progress |
| `led_cycle`             | Experiments with Freenove 8 RGB LED Module | ⏳ Planned      |

*More to come.*

---

## 🔧 Tools & Tech Stack

* Languages:  Rust
* IDEs: VSCode
* Libraries: `embedded_hal`, `rp2040_hal`, `panic_halt`, `hal`
* Protocols: GPIO, MQTT, UDP, TCP
* Flash storage & OTA update experiments (planned)

---

## 📁 Repo Structure

```
/bin/
  └── blinky_external_led/
  └── blinky_led_by_button/
  └── led_analog/
  └── led_bar/
  └── led_lamp/
  └── ...
/docs/
  └── wiring_schematics/
  └── setup_notes/
README.md
```

---

## 📎 Notes

This is an open lab space – not a polished library or framework. Things may be messy, experimental, or half-finished.
If you're tinkering with the Pico W too, feel free to fork, comment, or share ideas.


```cargo run --bin file_name```
