//! Automatically rewritten from C to Rust
//! Source: drivers/siox/siox-bus-gpio.c
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
//
// Copyright (C) 2015-2017 Pengutronix, Uwe Kleine-König <kernel@pengutronix.de>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct siox_gpio_ddata {
    pub din: *mut gpio_desc,
    pub dout: *mut gpio_desc,
    pub dclk: *mut gpio_desc,
    pub dld: *mut gpio_desc,
}

    let mut siox_clkhigh_ns: static unsigned int = 1000;
    static unsigned int siox_loadhigh_ns;
    static unsigned int siox_bytegap_ns;
    static int siox_gpio_pushpull(struct siox_master *smaster,
    size_t setbuf_len, const u8 setbuf[],
    size_t getbuf_len, u8 getbuf[])
    {
    struct siox_gpio_ddata *ddata = siox_master_get_devdata(smaster);
    size_t i;
    let mut cycles: usize = max(setbuf_len, getbuf_len);
// reset data and clock
    gpiod_set_value_cansleep(ddata.dout, 0);
    gpiod_set_value_cansleep(ddata.dclk, 0);
    gpiod_set_value_cansleep(ddata.dld, 1);
    ndelay(siox_loadhigh_ns);
    gpiod_set_value_cansleep(ddata.dld, 0);
    for (i = 0; i < cycles; ++i) {
    let mut set: u8 = 0, get = 0;
    size_t j;
    if (i >= cycles - setbuf_len)
    set = setbuf[i - (cycles - setbuf_len)];
    for (j = 0; j < 8; ++j) {
    get <<= 1;
    if (gpiod_get_value_cansleep(ddata.din))
    get |= 1;
// DOUT is logically inverted
    gpiod_set_value_cansleep(ddata.dout, !(set & 0x80));
    set <<= 1;
    gpiod_set_value_cansleep(ddata.dclk, 1);
    ndelay(siox_clkhigh_ns);
    gpiod_set_value_cansleep(ddata.dclk, 0);
    }
    if (i < getbuf_len)
    getbuf[i] = get;
    ndelay(siox_bytegap_ns);
    }
    gpiod_set_value_cansleep(ddata.dld, 1);
    ndelay(siox_loadhigh_ns);
    gpiod_set_value_cansleep(ddata.dld, 0);
//
// Resetting dout isn't necessary protocol wise, but it makes the
// signals more pretty because the dout level is deterministic between
// cycles. Note that this only affects dout between the master and the
// first siox device. dout for the later devices depend on the output of
// the previous siox device.
//
    gpiod_set_value_cansleep(ddata.dout, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn siox_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int siox_gpio_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct siox_gpio_ddata *ddata;
    int ret;
    struct siox_master *smaster;
    smaster = devm_siox_master_alloc(dev, sizeof(*ddata));
    if (!smaster)
    return -ENOMEM;
    platform_set_drvdata(pdev, smaster);
    ddata = siox_master_get_devdata(smaster);
    ddata.din = devm_gpiod_get(dev, "din", GPIOD_IN);
    if (IS_ERR(ddata.din))
    return dev_err_probe(dev, PTR_ERR(ddata.din),
    "Failed to get din GPIO\n");
    ddata.dout = devm_gpiod_get(dev, "dout", GPIOD_OUT_LOW);
    if (IS_ERR(ddata.dout))
    return dev_err_probe(dev, PTR_ERR(ddata.dout),
    "Failed to get dout GPIO\n");
    ddata.dclk = devm_gpiod_get(dev, "dclk", GPIOD_OUT_LOW);
    if (IS_ERR(ddata.dclk))
    return dev_err_probe(dev, PTR_ERR(ddata.dclk),
    "Failed to get dclk GPIO\n");
    ddata.dld = devm_gpiod_get(dev, "dld", GPIOD_OUT_LOW);
    if (IS_ERR(ddata.dld))
    return dev_err_probe(dev, PTR_ERR(ddata.dld),
    "Failed to get dld GPIO\n");
    smaster.pushpull = siox_gpio_pushpull;
// XXX: determine automatically like spi does
    smaster.busno = 0;
    ret = devm_siox_master_register(dev, smaster);
    if (ret)
    return dev_err_probe(dev, ret,
    "Failed to register siox master\n");
    return 0;
    }
    static const struct of_device_id siox_gpio_dt_ids[] = {
    { .compatible = "eckelmann,siox-gpio", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, siox_gpio_dt_ids);
    static struct platform_driver siox_gpio_driver = {
    .probe = siox_gpio_probe,
    .driver = {
    .name = DRIVER_NAME,
    .of_match_table = siox_gpio_dt_ids,
    },
    };
    module_platform_driver(siox_gpio_driver);
    MODULE_AUTHOR("Uwe Kleine-Koenig <u.kleine-koenig@pengutronix.de>");
    MODULE_DESCRIPTION("SIOX GPIO bus driver");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:" DRIVER_NAME);
