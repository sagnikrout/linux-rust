//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/solomon/ssd130x-spi.c
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
// DRM driver for Solomon SSD13xx OLED displays (SPI bus)
//
// Copyright 2022 Red Hat Inc.
// Authors: Javier Martinez Canillas <javierm@redhat.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssd130x_spi_transport {
    pub spi: *mut spi_device,
    pub dc: *mut gpio_desc,
}

//
// The regmap bus .write handler, it is just a wrapper around spi_write()
// but toggling the Data/Command control pin (D/C#). Since for 4-wire SPI
// a D/C# pin is used, in contrast with I2C where a control byte is sent,
// prior to every data byte, that contains a bit with the D/C# value.
//
// These control bytes are considered registers by the ssd130x core driver
// and can be used by the ssd130x SPI driver to determine if the data sent
// is for a command register or for the Graphic Display Data RAM (GDDRAM).
//
#[no_mangle]
unsafe extern "C" fn ssd130x_spi_write(context: *mut c_void, data: *const c_void, count: usize) -> c_int {
    static int ssd130x_spi_write(void *context, const void *data, size_t count)
    {
    struct ssd130x_spi_transport *t = context;
    struct spi_device *spi = t.spi;
    const u8 *reg = data;
    if (*reg == SSD13XX_COMMAND)
    gpiod_set_value_cansleep(t.dc, 0);
    if (*reg == SSD13XX_DATA)
    gpiod_set_value_cansleep(t.dc, 1);
// Remove control byte since is not used in a 4-wire SPI interface
    return spi_write(spi, reg + 1, count - 1);
    }
// The ssd130x driver does not read registers but regmap expects a .read
    static int ssd130x_spi_read(void *context, const void *reg, size_t reg_size,
    void *val, size_t val_size)
    {
    return -EOPNOTSUPP;
    }
    static const struct regmap_config ssd130x_spi_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .write = ssd130x_spi_write,
    .read = ssd130x_spi_read,
    .can_multi_write = true,
    };
#[no_mangle]
unsafe extern "C" fn ssd130x_spi_probe(spi: *mut spi_device) -> c_int {
    static int ssd130x_spi_probe(struct spi_device *spi)
    {
    struct ssd130x_spi_transport *t;
    struct ssd130x_device *ssd130x;
    struct regmap *regmap;
    struct gpio_desc *dc;
    struct device *dev = &spi.dev;
    dc = devm_gpiod_get(dev, "dc", GPIOD_OUT_LOW);
    if (IS_ERR(dc))
    return dev_err_probe(dev, PTR_ERR(dc),
    "Failed to get dc gpio\n");
    t = devm_kzalloc(dev, sizeof(*t), GFP_KERNEL);
    if (!t)
    return -ENOMEM;
    t.spi = spi;
    t.dc = dc;
    regmap = devm_regmap_init(dev, core::ptr::null_mut(), t, &ssd130x_spi_regmap_config);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    ssd130x = ssd130x_probe(dev, regmap);
    if (IS_ERR(ssd130x))
    return PTR_ERR(ssd130x);
    spi_set_drvdata(spi, ssd130x);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ssd130x_spi_remove(spi: *mut spi_device) {
    static void ssd130x_spi_remove(struct spi_device *spi)
    {
    struct ssd130x_device *ssd130x = spi_get_drvdata(spi);
    ssd130x_remove(ssd130x);
    }
#[no_mangle]
unsafe extern "C" fn ssd130x_spi_shutdown(spi: *mut spi_device) {
    static void ssd130x_spi_shutdown(struct spi_device *spi)
    {
    struct ssd130x_device *ssd130x = spi_get_drvdata(spi);
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
// ssd133x family
    {
    .compatible = "solomon,ssd1331",
    .data = &ssd130x_variants[SSD1331_ID],
    },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, ssd130x_of_match);
//
// The SPI core always reports a MODALIAS uevent of the form "spi:<dev>", even
// if the device was registered via OF. This means that the module will not be
// auto loaded, unless it contains an alias that matches the MODALIAS reported.
//
// To workaround this issue, add a SPI device ID table. Even when this should
// not be needed for this driver to match the registered SPI devices.
//
    static const struct spi_device_id ssd130x_spi_id[] = {
// ssd130x family
    { "sh1106",  SH1106_ID },
    { "ssd1305", SSD1305_ID },
    { "ssd1306", SSD1306_ID },
    { "ssd1307", SSD1307_ID },
    { "ssd1309", SSD1309_ID },
// ssd132x family
    { "ssd1322", SSD1322_ID },
    { "ssd1325", SSD1325_ID },
    { "ssd1327", SSD1327_ID },
// ssd133x family
    { "ssd1331", SSD1331_ID },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(spi, ssd130x_spi_id);
    static struct spi_driver ssd130x_spi_driver = {
    .driver = {
    .name = DRIVER_NAME,
    .of_match_table = ssd130x_of_match,
    },
    .id_table = ssd130x_spi_id,
    .probe = ssd130x_spi_probe,
    .remove = ssd130x_spi_remove,
    .shutdown = ssd130x_spi_shutdown,
    };
    module_spi_driver(ssd130x_spi_driver);
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_AUTHOR("Javier Martinez Canillas <javierm@redhat.com>");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("DRM_SSD130X");
