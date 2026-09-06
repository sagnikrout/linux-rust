//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/s5c73m3/s5c73m3-spi.c
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
// Samsung LSI S5C73M3 8M pixel camera driver
//
// Copyright (C) 2012, Samsung Electronics, Co., Ltd.
// Sylwester Nawrocki <s.nawrocki@samsung.com>
// Andrzej Hajda <a.hajda@samsung.com>
//

    static const struct of_device_id s5c73m3_spi_ids[] = {
    { .compatible = "samsung,s5c73m3" },
    { }
    };
    MODULE_DEVICE_TABLE(of, s5c73m3_spi_ids);
    enum spi_direction {
    SPI_DIR_RX,
    SPI_DIR_TX
    };
    static int spi_xmit(struct spi_device *spi_dev, void *addr, const int len,
    enum spi_direction dir)
    {
    struct spi_message msg;
    int r;
    struct spi_transfer xfer = {
    .len	= len,
    };
    if (dir == SPI_DIR_TX)
    xfer.tx_buf = addr;
    else
    xfer.rx_buf = addr;
    if (spi_dev == core::ptr::null_mut()) {
    pr_err("SPI device is uninitialized\n");
    return -ENODEV;
    }
    spi_message_init(&msg);
    spi_message_add_tail(&xfer, &msg);
    r = spi_sync(spi_dev, &msg);
    if (r < 0)
    dev_err(&spi_dev.dev, "%s spi_sync failed %d\n", __func__, r);
    return r;
    }
    int s5c73m3_spi_write(struct s5c73m3 *state, const void *addr,
    const unsigned int len, const unsigned int tx_size)
    {
    struct spi_device *spi_dev = state.spi_dev;
    let mut count: u32 = len / tx_size;
    let mut extra: u32 = len % tx_size;
    unsigned int i, j = 0;
    u8 padding[32];
    let mut r: c_int = 0;
    memset(padding, 0, sizeof(padding));
    for (i = 0; i < count; i++) {
    r = spi_xmit(spi_dev, (void *)addr + j, tx_size, SPI_DIR_TX);
    if (r < 0)
    return r;
    j += tx_size;
    }
    if (extra > 0) {
    r = spi_xmit(spi_dev, (void *)addr + j, extra, SPI_DIR_TX);
    if (r < 0)
    return r;
    }
    return spi_xmit(spi_dev, padding, sizeof(padding), SPI_DIR_TX);
    }
    int s5c73m3_spi_read(struct s5c73m3 *state, void *addr,
    const unsigned int len, const unsigned int tx_size)
    {
    struct spi_device *spi_dev = state.spi_dev;
    let mut count: u32 = len / tx_size;
    let mut extra: u32 = len % tx_size;
    unsigned int i, j = 0;
    let mut r: c_int = 0;
    for (i = 0; i < count; i++) {
    r = spi_xmit(spi_dev, addr + j, tx_size, SPI_DIR_RX);
    if (r < 0)
    return r;
    j += tx_size;
    }
    if (extra > 0)
    return spi_xmit(spi_dev, addr + j, extra, SPI_DIR_RX);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn s5c73m3_spi_probe(spi: *mut spi_device) -> c_int {
    static int s5c73m3_spi_probe(struct spi_device *spi)
    {
    int r;
    struct s5c73m3 *state = container_of(spi.dev.driver, struct s5c73m3,
    spidrv.driver);
    spi.bits_per_word = 32;
    r = spi_setup(spi);
    if (r < 0) {
    dev_err(&spi.dev, "spi_setup() failed\n");
    return r;
    }
    mutex_lock(&state.lock);
    state.spi_dev = spi;
    mutex_unlock(&state.lock);
    v4l2_info(&state.sensor_sd, "S5C73M3 SPI probed successfully\n");
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn s5c73m3_register_spi_driver(state: *mut s5c73m3) -> c_int {
    int s5c73m3_register_spi_driver(struct s5c73m3 *state)
    {
    struct spi_driver *spidrv = &state.spidrv;
    spidrv.probe = s5c73m3_spi_probe;
    spidrv.driver.name = S5C73M3_SPI_DRV_NAME;
    spidrv.driver.of_match_table = s5c73m3_spi_ids;
    return spi_register_driver(spidrv);
    }
#[no_mangle]
pub unsafe extern "C" fn s5c73m3_unregister_spi_driver(state: *mut s5c73m3) {
    void s5c73m3_unregister_spi_driver(struct s5c73m3 *state)
    {
    spi_unregister_driver(&state.spidrv);
    }
