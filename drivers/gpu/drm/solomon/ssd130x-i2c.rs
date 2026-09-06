//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/solomon/ssd130x-i2c.c
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
// DRM driver for Solomon SSD13xx OLED displays (I2C bus)
//
// Copyright 2022 Red Hat Inc.
// Author: Javier Martinez Canillas <javierm@redhat.com>
//
// Based on drivers/video/fbdev/ssd1307fb.c
// Copyright 2012 Free Electrons
//

    static const struct regmap_config ssd130x_i2c_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    };
#[no_mangle]
unsafe extern "C" fn ssd130x_i2c_probe(client: *mut i2c_client) -> c_int {
    static int ssd130x_i2c_probe(struct i2c_client *client)
    {
    struct ssd130x_device *ssd130x;
    struct regmap *regmap;
    regmap = devm_regmap_init_i2c(client, &ssd130x_i2c_regmap_config);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    ssd130x = ssd130x_probe(&client.dev, regmap);
    if (IS_ERR(ssd130x))
    return PTR_ERR(ssd130x);
    i2c_set_clientdata(client, ssd130x);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ssd130x_i2c_remove(client: *mut i2c_client) {
    static void ssd130x_i2c_remove(struct i2c_client *client)
    {
    struct ssd130x_device *ssd130x = i2c_get_clientdata(client);
    ssd130x_remove(ssd130x);
    }
#[no_mangle]
unsafe extern "C" fn ssd130x_i2c_shutdown(client: *mut i2c_client) {
    static void ssd130x_i2c_shutdown(struct i2c_client *client)
    {
    struct ssd130x_device *ssd130x = i2c_get_clientdata(client);
    ssd130x_shutdown(ssd130x);
    }
    static const struct of_device_id ssd130x_of_match[] = {
// ssd130x family
    {
    .compatible = "sinowealth,sh1106",
    .data = &ssd130x_variants[SH1106_ID],
    },
    {
    .compatible = "solomon,ssd1305",
    .data = &ssd130x_variants[SSD1305_ID],
    },
    {
    .compatible = "solomon,ssd1306",
    .data = &ssd130x_variants[SSD1306_ID],
    },
    {
    .compatible = "solomon,ssd1307",
    .data = &ssd130x_variants[SSD1307_ID],
    },
    {
    .compatible = "solomon,ssd1309",
    .data = &ssd130x_variants[SSD1309_ID],
    },
// Deprecated but kept for backward compatibility
    {
    .compatible = "solomon,ssd1305fb-i2c",
    .data = &ssd130x_variants[SSD1305_ID],
    },
    {
    .compatible = "solomon,ssd1306fb-i2c",
    .data = &ssd130x_variants[SSD1306_ID],
    },
    {
    .compatible = "solomon,ssd1307fb-i2c",
    .data = &ssd130x_variants[SSD1307_ID],
    },
    {
    .compatible = "solomon,ssd1309fb-i2c",
    .data = &ssd130x_variants[SSD1309_ID],
    },
// ssd132x family
    {
    .compatible = "solomon,ssd1322",
    .data = &ssd130x_variants[SSD1322_ID],
    },
    {
    .compatible = "solomon,ssd1325",
    .data = &ssd130x_variants[SSD1325_ID],
    },
    {
    .compatible = "solomon,ssd1327",
    .data = &ssd130x_variants[SSD1327_ID],
    },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, ssd130x_of_match);
    static struct i2c_driver ssd130x_i2c_driver = {
    .driver = {
    .name = DRIVER_NAME,
    .of_match_table = ssd130x_of_match,
    },
    .probe = ssd130x_i2c_probe,
    .remove = ssd130x_i2c_remove,
    .shutdown = ssd130x_i2c_shutdown,
    };
    module_i2c_driver(ssd130x_i2c_driver);
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_AUTHOR("Javier Martinez Canillas <javierm@redhat.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("DRM_SSD130X");
