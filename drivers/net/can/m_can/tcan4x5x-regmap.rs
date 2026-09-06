//! Automatically rewritten from C to Rust
//! Source: drivers/net/can/m_can/tcan4x5x-regmap.c
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
// tcan4x5x - Texas Instruments TCAN4x5x Family CAN controller driver
//
// Copyright (c) 2020 Pengutronix,
// Marc Kleine-Budde <kernel@pengutronix.de>
// Copyright (c) 2018-2019 Texas Instruments Incorporated
// http://www.ti.com

pub const TCAN4X5X_MAX_REGISTER: c_uint = 0x87fc;
    static int tcan4x5x_regmap_gather_write(void *context,
    const void *reg, size_t reg_len,
    const void *val, size_t val_len)
    {
    struct spi_device *spi = context;
    struct tcan4x5x_priv *priv = spi_get_drvdata(spi);
    struct tcan4x5x_map_buf *buf_tx = &priv.map_buf_tx;
    struct spi_transfer xfer[] = {
    {
    .tx_buf = buf_tx,
    .len = sizeof(buf_tx.cmd) + val_len,
    },
    };
    memcpy(&buf_tx.cmd, reg, sizeof(buf_tx.cmd.cmd) +
    sizeof(buf_tx.cmd.addr));
    tcan4x5x_spi_cmd_set_len(&buf_tx.cmd, val_len);
    memcpy(buf_tx.data, val, val_len);
    return spi_sync_transfer(spi, xfer, ARRAY_SIZE(xfer));
    }
#[no_mangle]
unsafe extern "C" fn tcan4x5x_regmap_write(context: *mut c_void, data: *const c_void, count: usize) -> c_int {
    static int tcan4x5x_regmap_write(void *context, const void *data, size_t count)
    {
    return tcan4x5x_regmap_gather_write(context, data, sizeof(__be32),
    data + sizeof(__be32),
    count - sizeof(__be32));
    }
    static int tcan4x5x_regmap_read(void *context,
    const void *reg_buf, size_t reg_len,
    void *val_buf, size_t val_len)
    {
    struct spi_device *spi = context;
    struct tcan4x5x_priv *priv = spi_get_drvdata(spi);
    struct tcan4x5x_map_buf *buf_rx = &priv.map_buf_rx;
    struct tcan4x5x_map_buf *buf_tx = &priv.map_buf_tx;
    struct spi_transfer xfer[2] = {
    {
    .tx_buf = buf_tx,
    }
    };
    struct spi_message msg;
    int err;
    spi_message_init(&msg);
    spi_message_add_tail(&xfer[0], &msg);
    memcpy(&buf_tx.cmd, reg_buf, sizeof(buf_tx.cmd.cmd) +
    sizeof(buf_tx.cmd.addr));
    tcan4x5x_spi_cmd_set_len(&buf_tx.cmd, val_len);
    if (spi.controller.flags & SPI_CONTROLLER_HALF_DUPLEX) {
    xfer[0].len = sizeof(buf_tx.cmd);
    xfer[1].rx_buf = val_buf;
    xfer[1].len = val_len;
    spi_message_add_tail(&xfer[1], &msg);
    } else {
    xfer[0].rx_buf = buf_rx;
    xfer[0].len = sizeof(buf_tx.cmd) + val_len;
    if (TCAN4X5X_SANITIZE_SPI)
    memset(buf_tx.data, 0x0, val_len);
    }
    err = spi_sync(spi, &msg);
    if (err)
    return err;
    if (!(spi.controller.flags & SPI_CONTROLLER_HALF_DUPLEX))
    memcpy(val_buf, buf_rx.data, val_len);
    return 0;
    }
    static const struct regmap_range tcan4x5x_reg_table_wr_range[] = {
// Device ID and SPI Registers
    regmap_reg_range(0x000c, 0x0010),
// Device configuration registers and Interrupt Flags
    regmap_reg_range(0x0800, 0x080c),
    regmap_reg_range(0x0820, 0x0820),
    regmap_reg_range(0x0830, 0x0830),
// M_CAN
    regmap_reg_range(0x100c, 0x102c),
    regmap_reg_range(0x1048, 0x1048),
    regmap_reg_range(0x1050, 0x105c),
    regmap_reg_range(0x1080, 0x1088),
    regmap_reg_range(0x1090, 0x1090),
    regmap_reg_range(0x1098, 0x10a0),
    regmap_reg_range(0x10a8, 0x10b0),
    regmap_reg_range(0x10b8, 0x10c0),
    regmap_reg_range(0x10c8, 0x10c8),
    regmap_reg_range(0x10d0, 0x10d4),
    regmap_reg_range(0x10e0, 0x10e4),
    regmap_reg_range(0x10f0, 0x10f0),
    regmap_reg_range(0x10f8, 0x10f8),
// MRAM
    regmap_reg_range(0x8000, 0x87fc),
    };
    static const struct regmap_range tcan4x5x_reg_table_rd_range[] = {
    regmap_reg_range(0x0000, 0x0010),	/* Device ID and SPI Registers */
    regmap_reg_range(0x0800, 0x0830),	/* Device configuration registers and Interrupt Flags*/
    regmap_reg_range(0x1000, 0x10fc),	/* M_CAN */
    regmap_reg_range(0x8000, 0x87fc),	/* MRAM */
    };
    static const struct regmap_access_table tcan4x5x_reg_table_wr = {
    .yes_ranges = tcan4x5x_reg_table_wr_range,
    .n_yes_ranges = ARRAY_SIZE(tcan4x5x_reg_table_wr_range),
    };
    static const struct regmap_access_table tcan4x5x_reg_table_rd = {
    .yes_ranges = tcan4x5x_reg_table_rd_range,
    .n_yes_ranges = ARRAY_SIZE(tcan4x5x_reg_table_rd_range),
    };
    static const struct regmap_config tcan4x5x_regmap = {
    .reg_bits = 24,
    .reg_stride = 4,
    .pad_bits = 8,
    .val_bits = 32,
    .wr_table = &tcan4x5x_reg_table_wr,
    .rd_table = &tcan4x5x_reg_table_rd,
    .max_register = TCAN4X5X_MAX_REGISTER,
    .cache_type = REGCACHE_NONE,
    .read_flag_mask = ( unsigned long)
    cpu_to_be32(TCAN4X5X_SPI_INSTRUCTION_READ),
    .write_flag_mask = ( unsigned long)
    cpu_to_be32(TCAN4X5X_SPI_INSTRUCTION_WRITE),
    };
    static const struct regmap_bus tcan4x5x_bus = {
    .write = tcan4x5x_regmap_write,
    .gather_write = tcan4x5x_regmap_gather_write,
    .read = tcan4x5x_regmap_read,
    .reg_format_endian_default = REGMAP_ENDIAN_BIG,
    .val_format_endian_default = REGMAP_ENDIAN_BIG,
    .max_raw_read = 256,
    .max_raw_write = 256,
    };
#[no_mangle]
pub unsafe extern "C" fn tcan4x5x_regmap_init(priv: *mut tcan4x5x_priv) -> c_int {
    int tcan4x5x_regmap_init(struct tcan4x5x_priv *priv)
    {
    priv.regmap = devm_regmap_init(&priv.spi.dev, &tcan4x5x_bus,
    priv.spi, &tcan4x5x_regmap);
    return PTR_ERR_OR_ZERO(priv.regmap);
    }
