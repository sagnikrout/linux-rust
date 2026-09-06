//! Automatically rewritten from C to Rust
//! Source: drivers/iio/pressure/mpl115_spi.c
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
// Freescale MPL115A1 pressure/temperature sensor
//
// Copyright (c) 2016 Akinobu Mita <akinobu.mita@gmail.com>
//
// Datasheet: http://www.nxp.com/files/sensors/doc/data_sheet/MPL115A1.pdf
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpl115_spi_buf {
    pub tx: [u8; 4],
    pub rx: [u8; 4],
}

#[no_mangle]
unsafe extern "C" fn mpl115_spi_init(dev: *mut device) -> c_int {
    static int mpl115_spi_init(struct device *dev)
    {
    struct spi_device *spi = to_spi_device(dev);
    struct mpl115_spi_buf *buf;
    buf = devm_kzalloc(dev, sizeof(*buf), GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
    spi_set_drvdata(spi, buf);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mpl115_spi_read(dev: *mut device, address: u8) -> c_int {
    static int mpl115_spi_read(struct device *dev, u8 address)
    {
    struct spi_device *spi = to_spi_device(dev);
    struct mpl115_spi_buf *buf = spi_get_drvdata(spi);
    struct spi_transfer xfer = {
    .tx_buf = buf.tx,
    .rx_buf = buf.rx,
    .len = 4,
    };
    int ret;
    buf.tx[0] = MPL115_SPI_READ(address);
    buf.tx[2] = MPL115_SPI_READ(address + 1);
    ret = spi_sync_transfer(spi, &xfer, 1);
    if (ret)
    return ret;
    return (buf.rx[1] << 8) | buf.rx[3];
    }
#[no_mangle]
unsafe extern "C" fn mpl115_spi_write(dev: *mut device, address: u8, value: u8) -> c_int {
    static int mpl115_spi_write(struct device *dev, u8 address, u8 value)
    {
    struct spi_device *spi = to_spi_device(dev);
    struct mpl115_spi_buf *buf = spi_get_drvdata(spi);
    struct spi_transfer xfer = {
    .tx_buf = buf.tx,
    .len = 2,
    };
    buf.tx[0] = MPL115_SPI_WRITE(address);
    buf.tx[1] = value;
    return spi_sync_transfer(spi, &xfer, 1);
    }
    static const struct mpl115_ops mpl115_spi_ops = {
    .init = mpl115_spi_init,
    .read = mpl115_spi_read,
    .write = mpl115_spi_write,
    };
#[no_mangle]
unsafe extern "C" fn mpl115_spi_probe(spi: *mut spi_device) -> c_int {
    static int mpl115_spi_probe(struct spi_device *spi)
    {
    const struct spi_device_id *id = spi_get_device_id(spi);
    return mpl115_probe(&spi.dev, id.name, &mpl115_spi_ops);
    }
    static const struct spi_device_id mpl115_spi_ids[] = {
    { .name = "mpl115" },
    { }
    };
    MODULE_DEVICE_TABLE(spi, mpl115_spi_ids);
    static struct spi_driver mpl115_spi_driver = {
    .driver = {
    .name   = "mpl115",
    .pm = pm_ptr(&mpl115_dev_pm_ops),
    },
    .probe = mpl115_spi_probe,
    .id_table = mpl115_spi_ids,
    };
    module_spi_driver(mpl115_spi_driver);
    MODULE_AUTHOR("Akinobu Mita <akinobu.mita@gmail.com>");
    MODULE_DESCRIPTION("Freescale MPL115A1 pressure/temperature driver");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_MPL115");
