//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-nic78bx.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2016 National Instruments Corp.
//

pub const NIC78BX_USER1_LED_MASK: c_uint = 0x3;

pub const NIC78BX_USER2_LED_MASK: c_uint = 0xC;

pub const NIC78BX_LOCK_REG_OFFSET: c_int = 1;
pub const NIC78BX_LOCK_VALUE: c_uint = 0xA5;
pub const NIC78BX_UNLOCK_VALUE: c_uint = 0x5A;
pub const NIC78BX_USER_LED_IO_SIZE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nic78bx_led_data {
    pub io_base: u16,
    pub lock: spinlock_t,
    pub pdev: *mut platform_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nic78bx_led {
    pub bit: u8,
    pub mask: u8,
    pub data: *mut nic78bx_led_data,
    pub cdev: led_classdev,
}

    static inline struct nic78bx_led *to_nic78bx_led(struct led_classdev *cdev)
    {
    return container_of(cdev, struct nic78bx_led, cdev);
    }
    static void nic78bx_brightness_set(struct led_classdev *cdev,
    enum led_brightness brightness)
    {
    struct nic78bx_led *nled = to_nic78bx_led(cdev);
    unsigned long flags;
    u8 value;
    spin_lock_irqsave(&nled.data.lock, flags);
    value = inb(nled.data.io_base);
    if (brightness) {
    value &= ~nled.mask;
    value |= nled.bit;
    } else {
    value &= ~nled.bit;
    }
    outb(value, nled.data.io_base);
    spin_unlock_irqrestore(&nled.data.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn nic78bx_brightness_get(cdev: *mut led_classdev) -> enum led_brightness {
    static enum led_brightness nic78bx_brightness_get(struct led_classdev *cdev)
    {
    struct nic78bx_led *nled = to_nic78bx_led(cdev);
    unsigned long flags;
    u8 value;
    spin_lock_irqsave(&nled.data.lock, flags);
    value = inb(nled.data.io_base);
    spin_unlock_irqrestore(&nled.data.lock, flags);
    return (value & nled.bit) ? 1 : LED_OFF;
    }
    static struct nic78bx_led nic78bx_leds[] = {
    {
    .bit = NIC78BX_USER1_GREEN_LED,
    .mask = NIC78BX_USER1_LED_MASK,
    .cdev = {
    .name = "nilrt:green:user1",
    .max_brightness = 1,
    .brightness_set = nic78bx_brightness_set,
    .brightness_get = nic78bx_brightness_get,
    }
    },
    {
    .bit = NIC78BX_USER1_YELLOW_LED,
    .mask = NIC78BX_USER1_LED_MASK,
    .cdev = {
    .name = "nilrt:yellow:user1",
    .max_brightness = 1,
    .brightness_set = nic78bx_brightness_set,
    .brightness_get = nic78bx_brightness_get,
    }
    },
    {
    .bit = NIC78BX_USER2_GREEN_LED,
    .mask = NIC78BX_USER2_LED_MASK,
    .cdev = {
    .name = "nilrt:green:user2",
    .max_brightness = 1,
    .brightness_set = nic78bx_brightness_set,
    .brightness_get = nic78bx_brightness_get,
    }
    },
    {
    .bit = NIC78BX_USER2_YELLOW_LED,
    .mask = NIC78BX_USER2_LED_MASK,
    .cdev = {
    .name = "nilrt:yellow:user2",
    .max_brightness = 1,
    .brightness_set = nic78bx_brightness_set,
    .brightness_get = nic78bx_brightness_get,
    }
    }
    };
#[no_mangle]
unsafe extern "C" fn lock_led_reg_action(data: *mut c_void) {
    static void lock_led_reg_action(void *data)
    {
    struct nic78bx_led_data *led_data = data;
// Lock LED register
    outb(NIC78BX_LOCK_VALUE,
    led_data.io_base + NIC78BX_LOCK_REG_OFFSET);
    }
#[no_mangle]
unsafe extern "C" fn nic78bx_probe(pdev: *mut platform_device) -> c_int {
    static int nic78bx_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct nic78bx_led_data *led_data;
    struct resource *io_rc;
    int ret, i;
    led_data = devm_kzalloc(dev, sizeof(*led_data), GFP_KERNEL);
    if (!led_data)
    return -ENOMEM;
    led_data.pdev = pdev;
    platform_set_drvdata(pdev, led_data);
    io_rc = platform_get_resource(pdev, IORESOURCE_IO, 0);
    if (!io_rc) {
    dev_err(dev, "missing IO resources\n");
    return -EINVAL;
    }
    if (resource_size(io_rc) < NIC78BX_USER_LED_IO_SIZE) {
    dev_err(dev, "IO region too small\n");
    return -EINVAL;
    }
    if (!devm_request_region(dev, io_rc.start, resource_size(io_rc),
    KBUILD_MODNAME)) {
    dev_err(dev, "failed to get IO region\n");
    return -EBUSY;
    }
    led_data.io_base = io_rc.start;
    spin_lock_init(&led_data.lock);
    ret = devm_add_action(dev, lock_led_reg_action, led_data);
    if (ret)
    return ret;
    for (i = 0; i < ARRAY_SIZE(nic78bx_leds); i++) {
    nic78bx_leds[i].data = led_data;
    ret = devm_led_classdev_register(dev, &nic78bx_leds[i].cdev);
    if (ret)
    return ret;
    }
// Unlock LED register
    outb(NIC78BX_UNLOCK_VALUE,
    led_data.io_base + NIC78BX_LOCK_REG_OFFSET);
    return ret;
    }
    static const struct acpi_device_id led_device_ids[] = {
    { "NIC78B3" },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, led_device_ids);
    static struct platform_driver led_driver = {
    .probe = nic78bx_probe,
    .driver = {
    .name = KBUILD_MODNAME,
    .acpi_match_table = led_device_ids,
    },
    };
    module_platform_driver(led_driver);
    MODULE_DESCRIPTION("National Instruments PXI User LEDs driver");
    MODULE_AUTHOR("Hui Chun Ong <hui.chun.ong@ni.com>");
    MODULE_LICENSE("GPL");
