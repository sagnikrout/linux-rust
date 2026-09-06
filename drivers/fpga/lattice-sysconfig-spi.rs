//! Automatically rewritten from C to Rust
//! Source: drivers/fpga/lattice-sysconfig-spi.c
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
// Lattice FPGA programming over slave SPI sysCONFIG interface.
//

    let mut ecp5_spi_max_speed_hz: static u32 = 60000000;
    static int sysconfig_spi_cmd_transfer(struct sysconfig_priv *priv,
    const void *tx_buf, size_t tx_len,
    void *rx_buf, size_t rx_len)
    {
    struct spi_device *spi = to_spi_device(priv.dev);
    return spi_write_then_read(spi, tx_buf, tx_len, rx_buf, rx_len);
    }
#[no_mangle]
unsafe extern "C" fn sysconfig_spi_bitstream_burst_init(priv: *mut sysconfig_priv) -> c_int {
    static int sysconfig_spi_bitstream_burst_init(struct sysconfig_priv *priv)
    {
    const u8 lsc_bitstream_burst[] = SYSCONFIG_LSC_BITSTREAM_BURST;
    struct spi_device *spi = to_spi_device(priv.dev);
    let mut xfer: spi_transfer = {};
    struct spi_message msg;
    size_t buf_len;
    void *buf;
    int ret;
    buf_len = sizeof(lsc_bitstream_burst);
    buf = kmemdup(lsc_bitstream_burst, buf_len, GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
    xfer.len = buf_len;
    xfer.tx_buf = buf;
    xfer.cs_change = 1;
    spi_message_init_with_transfers(&msg, &xfer, 1);
//
// Lock SPI bus for exclusive usage until FPGA programming is done.
// SPI bus will be released in sysconfig_spi_bitstream_burst_complete().
//
    spi_bus_lock(spi.controller);
    ret = spi_sync_locked(spi, &msg);
    if (ret)
    spi_bus_unlock(spi.controller);
    kfree(buf);
    return ret;
    }
    static int sysconfig_spi_bitstream_burst_write(struct sysconfig_priv *priv,
    const char *buf, size_t len)
    {
    struct spi_device *spi = to_spi_device(priv.dev);
    struct spi_transfer xfer = {
    .tx_buf = buf,
    .len = len,
    .cs_change = 1,
    };
    struct spi_message msg;
    spi_message_init_with_transfers(&msg, &xfer, 1);
    return spi_sync_locked(spi, &msg);
    }
#[no_mangle]
unsafe extern "C" fn sysconfig_spi_bitstream_burst_complete(priv: *mut sysconfig_priv) -> c_int {
    static int sysconfig_spi_bitstream_burst_complete(struct sysconfig_priv *priv)
    {
    struct spi_device *spi = to_spi_device(priv.dev);
// Bitstream burst write is done, release SPI bus
    spi_bus_unlock(spi.controller);
// Toggle CS to finish bitstream write
    return spi_write(spi, core::ptr::null_mut(), 0);
    }
#[no_mangle]
unsafe extern "C" fn sysconfig_spi_probe(spi: *mut spi_device) -> c_int {
    static int sysconfig_spi_probe(struct spi_device *spi)
    {
    struct device *dev = &spi.dev;
    struct sysconfig_priv *priv;
    const u32 *spi_max_speed;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    spi_max_speed = spi_get_device_match_data(spi);
    if (!spi_max_speed)
    return -EINVAL;
    if (spi.max_speed_hz > *spi_max_speed) {
    dev_err(dev, "SPI speed %u is too high, maximum speed is %u\n",
    spi.max_speed_hz, *spi_max_speed);
    return -EINVAL;
    }
    priv.dev = dev;
    priv.command_transfer = sysconfig_spi_cmd_transfer;
    priv.bitstream_burst_write_init = sysconfig_spi_bitstream_burst_init;
    priv.bitstream_burst_write = sysconfig_spi_bitstream_burst_write;
    priv.bitstream_burst_write_complete = sysconfig_spi_bitstream_burst_complete;
    return sysconfig_probe(priv);
    }
    static const struct spi_device_id sysconfig_spi_ids[] = {
    {
    .name = "sysconfig-ecp5",
    .driver_data = (kernel_ulong_t)&ecp5_spi_max_speed_hz,
    },
    {}
    };
    MODULE_DEVICE_TABLE(spi, sysconfig_spi_ids);
    static const struct of_device_id sysconfig_of_ids[] = {
    {
    .compatible = "lattice,sysconfig-ecp5",
    .data = &ecp5_spi_max_speed_hz,
    },
    {}
    };
    MODULE_DEVICE_TABLE(of, sysconfig_of_ids);
    static struct spi_driver lattice_sysconfig_driver = {
    .probe = sysconfig_spi_probe,
    .id_table = sysconfig_spi_ids,
    .driver = {
    .name = "lattice_sysconfig_spi_fpga_mgr",
    .of_match_table = sysconfig_of_ids,
    },
    };
    module_spi_driver(lattice_sysconfig_driver);
    MODULE_DESCRIPTION("Lattice sysCONFIG Slave SPI FPGA Manager");
    MODULE_LICENSE("GPL");
