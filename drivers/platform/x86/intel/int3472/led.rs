//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/intel/int3472/led.c
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0
// Author: Hans de Goede <hdegoede@redhat.com>

#[no_mangle]
unsafe extern "C" fn int3472_led_set(led_cdev: *mut led_classdev, brightness: enum led_brightness) -> c_int {
    static int int3472_led_set(struct led_classdev *led_cdev, enum led_brightness brightness)
    {
    struct int3472_led *led = container_of(led_cdev, struct int3472_led, classdev);
    gpiod_set_value_cansleep(led.gpio, brightness);
    return 0;
    }
    int skl_int3472_register_led(struct int3472_discrete_device *int3472, struct gpio_desc *gpio,
    const char *con_id)
    {
    struct int3472_led *led;
    char *p;
    int ret;
    if (int3472.n_leds >= INT3472_MAX_LEDS)
    return -ENOSPC;
    led = &int3472.leds[int3472.n_leds];
    led.gpio = gpio;
// Generate the name, replacing the ':' in the ACPI devname with '_'
    snprintf(led.name, sizeof(led.name),
    "%s::%s_led", acpi_dev_name(int3472.sensor), con_id);
    p = strchr(led.name, ':');
    if (p)
// p = '_';
    led.classdev.name = led.name;
    led.classdev.max_brightness = 1;
    led.classdev.brightness_set_blocking = int3472_led_set;
    ret = led_classdev_register(int3472.dev, &led.classdev);
    if (ret)
    return ret;
    led.lookup.provider = led.name;
    led.lookup.dev_id = int3472.sensor_name;
    led.lookup.con_id = con_id;
    led_add_lookup(&led.lookup);
    int3472.n_leds++;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn skl_int3472_unregister_leds(int3472: *mut int3472_discrete_device) {
    void skl_int3472_unregister_leds(struct int3472_discrete_device *int3472)
    {
    for (unsigned int i = 0; i < int3472.n_leds; i++) {
    struct int3472_led *led = &int3472.leds[i];
    led_remove_lookup(&led.lookup);
    led_classdev_unregister(&led.classdev);
    gpiod_put(led.gpio);
    }
    }
