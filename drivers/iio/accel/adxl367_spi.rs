//! Automatically rewritten from C to Rust
//! Source: drivers/iio/accel/adxl367_spi.c
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
//
// Copyright (C) 2021 Analog Devices, Inc.
// Author: Cosmin Tanislav <cosmin.tanislav@analog.com>
//

pub const ADXL367_SPI_WRITE_COMMAND: c_uint = 0x0A;
pub const ADXL367_SPI_READ_COMMAND: c_uint = 0x0B;
pub const ADXL367_SPI_FIFO_COMMAND: c_uint = 0x0D;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adxl367_spi_state {
    pub spi: *mut spi_device,
    pub reg_write_msg: spi_message,
    pub reg_write_xfer: [spi_transfer; 2],
    pub reg_read_msg: spi_message,
    pub reg_read_xfer: [spi_transfer; 2],
    pub fifo_msg: spi_message,
    pub fifo_xfer: [spi_transfer; 2],
//
// DMA (thus cache coherency maintenance) may require the
// transfer buffers live in their own cache lines.
//
    pub __aligned(IIO_DMA_MINALIGN): u8 reg_write_tx_buf[1],
    pub reg_read_tx_buf: [u8; 2],
    pub fifo_tx_buf: [u8; 1],
}

    static int adxl367_read_fifo(void *context, __be16 *fifo_buf,
    unsigned int fifo_entries)
    {
    struct adxl367_spi_state *st = context;
    st.fifo_xfer[1].rx_buf = fifo_buf;
    st.fifo_xfer[1].len = fifo_entries * sizeof(*fifo_buf);
    return spi_sync(st.spi, &st.fifo_msg);
    }
    static int adxl367_read(void *context, const void *reg_buf, size_t reg_size,
    void *val_buf, size_t val_size)
    {
    struct adxl367_spi_state *st = context;
    let mut reg: u8 = ((const u8 *)reg_buf)[0];
    st.reg_read_tx_buf[1] = reg;
    st.reg_read_xfer[1].rx_buf = val_buf;
    st.reg_read_xfer[1].len = val_size;
    return spi_sync(st.spi, &st.reg_read_msg);
    }
#[no_mangle]
unsafe extern "C" fn adxl367_write(context: *mut c_void, val_buf: *const c_void, val_size: usize) -> c_int {
    static int adxl367_write(void *context, const void *val_buf, size_t val_size)
    {
    struct adxl367_spi_state *st = context;
    st.reg_write_xfer[1].tx_buf = val_buf;
    st.reg_write_xfer[1].len = val_size;
    return spi_sync(st.spi, &st.reg_write_msg);
    }
    static const struct regmap_bus adxl367_spi_regmap_bus = {
    .read = adxl367_read,
    .write = adxl367_write,
    };
    static const struct regmap_config adxl367_spi_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    };
    static const struct adxl367_ops adxl367_spi_ops = {
    .read_fifo = adxl367_read_fifo,
    };
#[no_mangle]
unsafe extern "C" fn adxl367_spi_probe(spi: *mut spi_device) -> c_int {
    static int adxl367_spi_probe(struct spi_device *spi)
    {
    struct adxl367_spi_state *st;
    struct regmap *regmap;
    st = devm_kzalloc(&spi.dev, sizeof(*st), GFP_KERNEL);
    if (!st)
    return -ENOMEM;
    st.spi = spi;
//
// Xfer:   [XFR1] [           XFR2           ]
// Master:  0x0A   ADDR DATA0 DATA1 ... DATAN
// Slave:   ....   ..........................
//
    st.reg_write_tx_buf[0] = ADXL367_SPI_WRITE_COMMAND;
    st.reg_write_xfer[0].tx_buf = st.reg_write_tx_buf;
    st.reg_write_xfer[0].len = sizeof(st.reg_write_tx_buf);
    spi_message_init_with_transfers(&st.reg_write_msg,
    st.reg_write_xfer, 2);
//
// Xfer:   [   XFR1  ] [         XFR2        ]
// Master:  0x0B ADDR   .....................
// Slave:   .........   DATA0 DATA1 ... DATAN
//
    st.reg_read_tx_buf[0] = ADXL367_SPI_READ_COMMAND;
    st.reg_read_xfer[0].tx_buf = st.reg_read_tx_buf;
    st.reg_read_xfer[0].len = sizeof(st.reg_read_tx_buf);
    spi_message_init_with_transfers(&st.reg_read_msg,
    st.reg_read_xfer, 2);
//
// Xfer:   [XFR1] [         XFR2        ]
// Master:  0x0D   .....................
// Slave:   ....   DATA0 DATA1 ... DATAN
//
    st.fifo_tx_buf[0] = ADXL367_SPI_FIFO_COMMAND;
    st.fifo_xfer[0].tx_buf = st.fifo_tx_buf;
    st.fifo_xfer[0].len = sizeof(st.fifo_tx_buf);
    spi_message_init_with_transfers(&st.fifo_msg, st.fifo_xfer, 2);
    regmap = devm_regmap_init(&spi.dev, &adxl367_spi_regmap_bus, st,
    &adxl367_spi_regmap_config);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    return adxl367_probe(&spi.dev, &adxl367_spi_ops, st, regmap, spi.irq);
    }
    static const struct spi_device_id adxl367_spi_id[] = {
    { .name = "adxl367" },
    { }
    };
    MODULE_DEVICE_TABLE(spi, adxl367_spi_id);
    static const struct of_device_id adxl367_of_match[] = {
    { .compatible = "adi,adxl367" },
    { }
    };
    MODULE_DEVICE_TABLE(of, adxl367_of_match);
    static struct spi_driver adxl367_spi_driver = {
    .driver = {
    .name = "adxl367_spi",
    .of_match_table = adxl367_of_match,
    },
    .probe = adxl367_spi_probe,
    .id_table = adxl367_spi_id,
    };
    module_spi_driver(adxl367_spi_driver);
    MODULE_IMPORT_NS("IIO_ADXL367");
    MODULE_AUTHOR("Cosmin Tanislav <cosmin.tanislav@analog.com>");
    MODULE_DESCRIPTION("Analog Devices ADXL367 3-axis accelerometer SPI driver");
    MODULE_LICENSE("GPL");
