# SPDX-License-Identifier: Apache-2.0

cmake_minimum_required(VERSION 3.20.0)
find_package(Zephyr REQUIRED HINTS $ENV{ZEPHYR_BASE})
project(fw)

<<<<<<< HEAD
target_sources(app PRIVATE src/Bluetooth.c src/Control.c src/Heartbeat.c src/Subject.c src/Ui.c src/main.c)
=======
target_sources(app PRIVATE src/main.c)
>>>>>>> 4bad30e (Updated framework for nrf project)
