//! Automatically rewritten from C to Rust
//! Source: drivers/base/regmap/regmap-spi.c
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
// Register map access API - SPI support
//
// Copyright 2011 Wolfson Microelectronics plc
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regmap_async_spi {
    pub core: regmap_async,
    pub m: spi_message,
    pub t: [spi_transfer; 2],
}

#[no_mangle]
unsafe extern "C" fn regmap_spi_complete(data: *mut c_void) {
    static void regmap_spi_complete(void *data)
    {
    struct regmap_async_spi *async = data;
    regmap_async_complete_cb(&async.core, async.m.status);
    }
#[no_mangle]
unsafe extern "C" fn regmap_spi_write(context: *mut c_void, data: *const c_void, count: usize) -> c_int {
    static int regmap_spi_write(void *context, const void *data, size_t count)
    {
    struct device *dev = context;
    struct spi_device *spi = to_spi_device(dev);
    return spi_write(spi, data, count);
    }
    static int regmap_spi_gather_write(void *context,
    const void *reg, size_t reg_len,
    const void *val, size_t val_len)
    {
    struct device *dev = context;
    struct spi_device *spi = to_spi_device(dev);
    struct spi_message m;
    struct spi_transfer t[2] = { { .tx_buf = reg, .len = reg_len, },
    { .tx_buf = val, .len = val_len, }, };
    spi_message_init(&m);
    spi_message_add_tail(&t[0], &m);
    spi_message_add_tail(&t[1], &m);
    return spi_sync(spi, &m);
    }
    static int regmap_spi_async_write(void *context,
    const void *reg, size_t reg_len,
    const void *val, size_t val_len,
    struct regmap_async *a)
    {
    struct regmap_async_spi *async = container_of(a,
    struct regmap_async_spi,
    core);
    struct device *dev = context;
    struct spi_device *spi = to_spi_device(dev);
    async.t[0].tx_buf = reg;
    async.t[0].len = reg_len;
    async.t[1].tx_buf = val;
    async.t[1].len = val_len;
    spi_message_init(&async.m);
    spi_message_add_tail(&async.t[0], &async.m);
    if (val)
    spi_message_add_tail(&async.t[1], &async.m);
    async.m.complete = regmap_spi_complete;
    async.m.context = async;
    return spi_async(spi, &async.m);
    }
    static struct regmap_async *regmap_spi_async_alloc(void)
    {
    struct regmap_async_spi *async_spi;
    async_spi = kzalloc_obj(*async_spi);
    if (!async_spi)
    return core::ptr::null_mut();
    return &async_spi.core;
    }
    static int regmap_spi_read(void *context,
    const void *reg, size_t reg_size,
    void *val, size_t val_size)
    {
    struct device *dev = context;
    struct spi_device *spi = to_spi_device(dev);
    return spi_write_then_read(spi, reg, reg_size, val, val_size);
    }
    static const struct regmap_bus regmap_spi = {
    .write = regmap_spi_write,
    .gather_write = regmap_spi_gather_write,
    .async_write = regmap_spi_async_write,
    .async_alloc = regmap_spi_async_alloc,
    .read = regmap_spi_read,
    .read_flag_mask = 0x80,
    .reg_format_endian_default = REGMAP_ENDIAN_BIG,
    .val_format_endian_default = REGMAP_ENDIAN_BIG,
    };
    static const struct regmap_bus *regmap_get_spi_bus(struct spi_device *spi,
    const struct regmap_config *config)
    {
    let mut max_size: usize = spi_max_transfer_size(spi);
    size_t max_msg_size, reg_reserve_size;
    struct regmap_bus *bus;
    if (max_size != SIZE_MAX) {
    bus = kmemdup(&regmap_spi, sizeof(*bus), GFP_KERNEL);
    if (!bus)
    return ERR_PTR(-ENOMEM);
    max_msg_size = spi_max_message_size(spi);
    reg_reserve_size = (config.reg_bits + config.pad_bits) / BITS_PER_BYTE;
    if (max_size + reg_reserve_size > max_msg_size)
    max_size -= reg_reserve_size;
    bus.free_on_exit = true;
    bus.max_raw_read = max_size;
    bus.max_raw_write = max_size;
    return bus;
    }
    return &regmap_spi;
    }
    struct regmap *__regmap_init_spi(struct spi_device *spi,
    const struct regmap_config *config,
    struct lock_class_key *lock_key,
    const char *lock_name)
    {
    const struct regmap_bus *bus = regmap_get_spi_bus(spi, config);
    if (IS_ERR(bus))
    return ERR_CAST(bus);
    return __regmap_init(&spi.dev, bus, &spi.dev, config, lock_key, lock_name);
    }
    EXPORT_SYMBOL_GPL(__regmap_init_spi);
    struct regmap *__devm_regmap_init_spi(struct spi_device *spi,
    const struct regmap_config *config,
    struct lock_class_key *lock_key,
    const char *lock_name)
    {
    const struct regmap_bus *bus = regmap_get_spi_bus(spi, config);
    if (IS_ERR(bus))
    return ERR_CAST(bus);
    return __devm_regmap_init(&spi.dev, bus, &spi.dev, config, lock_key, lock_name);
    }
    EXPORT_SYMBOL_GPL(__devm_regmap_init_spi);
    MODULE_DESCRIPTION("regmap SPI Module");
    MODULE_LICENSE("GPL");
