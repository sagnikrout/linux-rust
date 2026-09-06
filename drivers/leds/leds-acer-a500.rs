//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-acer-a500.c
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


// SPDX-License-Identifier: GPL-2.0+

    enum {
    REG_RESET_LEDS = 0x40,
    REG_POWER_LED_ON = 0x42,
    REG_CHARGE_LED_ON = 0x43,
    REG_ANDROID_LEDS_OFF = 0x5a,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a500_led {
    pub cdev: led_classdev,
    pub enable_seq: *const reg_sequence,
    pub other: *mut a500_led,
    pub rmap: *mut regmap,
}

    static const struct reg_sequence a500_ec_leds_reset_seq[] = {
    REG_SEQ(REG_RESET_LEDS, 0x0, A500_EC_LED_DELAY_USEC),
    REG_SEQ(REG_ANDROID_LEDS_OFF, 0x0, A500_EC_LED_DELAY_USEC),
    };
    static const struct reg_sequence a500_ec_white_led_enable_seq[] = {
    REG_SEQ(REG_POWER_LED_ON, 0x0, A500_EC_LED_DELAY_USEC),
    };
    static const struct reg_sequence a500_ec_orange_led_enable_seq[] = {
    REG_SEQ(REG_CHARGE_LED_ON, 0x0, A500_EC_LED_DELAY_USEC),
    };
    static int a500_ec_led_brightness_set(struct led_classdev *led_cdev,
    enum led_brightness value)
    {
    struct a500_led *led = container_of(led_cdev, struct a500_led, cdev);
    struct reg_sequence control_seq[2];
    let mut num_regs: c_uint = 1;
    if (value) {
    control_seq[0] = led.enable_seq[0];
    } else {
//
// There is no separate controls which can disable LEDs
// individually, there is only RESET_LEDS command that turns
// off both LEDs.
//
// RESET_LEDS turns off both LEDs, thus restore other LED if
// it's turned ON.
//
    if (led.other.cdev.brightness)
    num_regs = 2;
    control_seq[0] = a500_ec_leds_reset_seq[0];
    control_seq[1] = led.other.enable_seq[0];
    }
    return regmap_multi_reg_write(led.rmap, control_seq, num_regs);
    }
#[no_mangle]
unsafe extern "C" fn a500_ec_leds_probe(pdev: *mut platform_device) -> c_int {
    static int a500_ec_leds_probe(struct platform_device *pdev)
    {
    struct a500_led *white_led, *orange_led;
    struct regmap *rmap;
    int err;
    rmap = dev_get_regmap(pdev.dev.parent, "KB930");
    if (!rmap)
    return -EINVAL;
// reset and turn off LEDs
    regmap_multi_reg_write(rmap, a500_ec_leds_reset_seq, 2);
    white_led = devm_kzalloc(&pdev.dev, sizeof(*white_led), GFP_KERNEL);
    if (!white_led)
    return -ENOMEM;
    white_led.cdev.name = "power:white";
    white_led.cdev.brightness_set_blocking = a500_ec_led_brightness_set;
    white_led.cdev.flags = LED_CORE_SUSPENDRESUME;
    white_led.cdev.max_brightness = 1;
    white_led.enable_seq = a500_ec_white_led_enable_seq;
    white_led.rmap = rmap;
    orange_led = devm_kzalloc(&pdev.dev, sizeof(*orange_led), GFP_KERNEL);
    if (!orange_led)
    return -ENOMEM;
    orange_led.cdev.name = "power:orange";
    orange_led.cdev.brightness_set_blocking = a500_ec_led_brightness_set;
    orange_led.cdev.flags = LED_CORE_SUSPENDRESUME;
    orange_led.cdev.max_brightness = 1;
    orange_led.enable_seq = a500_ec_orange_led_enable_seq;
    orange_led.rmap = rmap;
    white_led.other = orange_led;
    orange_led.other = white_led;
    err = devm_led_classdev_register(&pdev.dev, &white_led.cdev);
    if (err) {
    dev_err(&pdev.dev, "failed to register white LED\n");
    return err;
    }
    err = devm_led_classdev_register(&pdev.dev, &orange_led.cdev);
    if (err) {
    dev_err(&pdev.dev, "failed to register orange LED\n");
    return err;
    }
    return 0;
    }
    static struct platform_driver a500_ec_leds_driver = {
    .driver = {
    .name = "acer-a500-iconia-leds",
    },
    .probe = a500_ec_leds_probe,
    };
    module_platform_driver(a500_ec_leds_driver);
    MODULE_DESCRIPTION("LED driver for Acer Iconia Tab A500 Power Button");
    MODULE_AUTHOR("Dmitry Osipenko <digetx@gmail.com>");
    MODULE_ALIAS("platform:acer-a500-iconia-leds");
    MODULE_LICENSE("GPL");
