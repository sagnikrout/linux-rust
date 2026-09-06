//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-max7301.c
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
// Copyright (C) 2006 Juergen Beisert, Pengutronix
// Copyright (C) 2008 Guennadi Liakhovetski, Pengutronix
// Copyright (C) 2009 Wolfram Sang, Pengutronix
//
// Check max730x.c for further details.
//

// A write to the MAX7301 means one message with one transfer
    static int max7301_spi_write(struct device *dev, unsigned int reg,
    unsigned int val)
    {
    struct spi_device *spi = to_spi_device(dev);
    let mut word: u16 = ((reg & 0x7F) << 8) | (val & 0xFF);
    return spi_write_then_read(spi, &word, sizeof(word), core::ptr::null_mut(), 0);
    }
// A read from the MAX7301 means two transfers; here, one message each
#[no_mangle]
unsafe extern "C" fn max7301_spi_read(dev: *mut device, reg: c_uint) -> c_int {
    static int max7301_spi_read(struct device *dev, unsigned int reg)
    {
    int ret;
    u16 word;
    struct spi_device *spi = to_spi_device(dev);
    word = 0x8000 | (reg << 8);
    ret = spi_write_then_read(spi, &word, sizeof(word), &word,
    sizeof(word));
    if (ret)
    return ret;
    return word & 0xff;
    }
#[no_mangle]
unsafe extern "C" fn max7301_probe(spi: *mut spi_device) -> c_int {
    static int max7301_probe(struct spi_device *spi)
    {
    struct max7301 *ts;
    int ret;
// bits_per_word cannot be configured in platform data
    spi.bits_per_word = 16;
    ret = spi_setup(spi);
    if (ret < 0)
    return ret;
    ts = devm_kzalloc(&spi.dev, sizeof(struct max7301), GFP_KERNEL);
    if (!ts)
    return -ENOMEM;
    ts.read = max7301_spi_read;
    ts.write = max7301_spi_write;
    ts.dev = &spi.dev;
    ret = __max730x_probe(ts);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn max7301_remove(spi: *mut spi_device) {
    static void max7301_remove(struct spi_device *spi)
    {
    __max730x_remove(&spi.dev);
    }
    static const struct spi_device_id max7301_id[] = {
    { "max7301", 0 },
    { }
    };
    MODULE_DEVICE_TABLE(spi, max7301_id);
    static struct spi_driver max7301_driver = {
    .driver = {
    .name = "max7301",
    },
    .probe = max7301_probe,
    .remove = max7301_remove,
    .id_table = max7301_id,
    };
#[no_mangle]
unsafe extern "C" fn max7301_init() -> int __init {
    static int __init max7301_init(void)
    {
    return spi_register_driver(&max7301_driver);
    }
// register after spi postcore initcall and before
// subsys initcalls that may rely on these GPIOs
//
    subsys_initcall(max7301_init);
#[no_mangle]
unsafe extern "C" fn max7301_exit() -> void __exit {
    static void __exit max7301_exit(void)
    {
    spi_unregister_driver(&max7301_driver);
    }
    module_exit(max7301_exit);
    MODULE_AUTHOR("Juergen Beisert, Wolfram Sang");
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("MAX7301 GPIO-Expander");
