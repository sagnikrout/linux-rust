//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-dac124s085.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright 2008
// Guennadi Liakhovetski, DENX Software Engineering, <lg@denx.de>
//
// LED driver for the DAC124S085 SPI DAC
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dac124s085_led {
    pub ldev: led_classdev,
    pub spi: *mut spi_device,
    pub id: c_int,
    pub name: [c_char; sizeof("dac124s085-3")],
    pub mutex: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dac124s085 {
    pub leds: [dac124s085_led; 4],
}

    static int dac124s085_set_brightness(struct led_classdev *ldev,
    enum led_brightness brightness)
    {
    struct dac124s085_led *led = container_of(ldev, struct dac124s085_led,
    ldev);
    __le16 word;
    int ret;
    mutex_lock(&led.mutex);
    word = cpu_to_le16(((led.id) << 14) | REG_WRITE_UPDATE |
    (brightness & 0xfff));
    ret = spi_write(led.spi, (const u8 *)&word, sizeof(word));
    mutex_unlock(&led.mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dac124s085_probe(spi: *mut spi_device) -> c_int {
    static int dac124s085_probe(struct spi_device *spi)
    {
    struct dac124s085	*dac;
    struct dac124s085_led	*led;
    int i, ret;
    dac = devm_kzalloc(&spi.dev, sizeof(*dac), GFP_KERNEL);
    if (!dac)
    return -ENOMEM;
    spi.bits_per_word = 16;
    for (i = 0; i < ARRAY_SIZE(dac.leds); i++) {
    led		= dac.leds + i;
    led.id		= i;
    led.spi	= spi;
    snprintf(led.name, sizeof(led.name), "dac124s085-%d", i);
    mutex_init(&led.mutex);
    led.ldev.name = led.name;
    led.ldev.brightness = LED_OFF;
    led.ldev.max_brightness = 0xfff;
    led.ldev.brightness_set_blocking = dac124s085_set_brightness;
    ret = led_classdev_register(&spi.dev, &led.ldev);
    if (ret < 0)
    goto eledcr;
    }
    spi_set_drvdata(spi, dac);
    return 0;
    eledcr:
    while (i--)
    led_classdev_unregister(&dac.leds[i].ldev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dac124s085_remove(spi: *mut spi_device) {
    static void dac124s085_remove(struct spi_device *spi)
    {
    struct dac124s085	*dac = spi_get_drvdata(spi);
    int i;
    for (i = 0; i < ARRAY_SIZE(dac.leds); i++)
    led_classdev_unregister(&dac.leds[i].ldev);
    }
    static struct spi_driver dac124s085_driver = {
    .probe		= dac124s085_probe,
    .remove		= dac124s085_remove,
    .driver = {
    .name	= "dac124s085",
    },
    };
    module_spi_driver(dac124s085_driver);
    MODULE_AUTHOR("Guennadi Liakhovetski <lg@denx.de>");
    MODULE_DESCRIPTION("DAC124S085 LED driver");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("spi:dac124s085");
