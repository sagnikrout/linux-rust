//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/devices/mchp23k256.c
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
// mchp23k256.c
//
// Driver for Microchip 23k256 SPI RAM chips
//
// Copyright © 2016 Andrew Lunn <andrew@lunn.ch>
//

pub const MAX_CMD_SIZE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mchp23_caps {
    pub addr_width: u8,
    pub size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mchp23k256_flash {
    pub spi: *mut spi_device,
    pub lock: mutex,
    pub mtd: mtd_info,
    pub caps: *const mchp23_caps,
}

pub const MCHP23K256_CMD_WRITE_STATUS: c_uint = 0x01;
pub const MCHP23K256_CMD_WRITE: c_uint = 0x02;
pub const MCHP23K256_CMD_READ: c_uint = 0x03;

    static void mchp23k256_addr2cmd(struct mchp23k256_flash *flash,
    unsigned int addr, u8 *cmd)
    {
    int i;
//
// Address is sent in big endian (MSB first) and we skip
// the first entry of the cmd array which contains the cmd
// opcode.
//
    for (i = flash.caps.addr_width; i > 0; i--, addr >>= 8)
    cmd[i] = addr;
    }
#[no_mangle]
unsafe extern "C" fn mchp23k256_cmdsz(flash: *mut mchp23k256_flash) -> c_int {
    static int mchp23k256_cmdsz(struct mchp23k256_flash *flash)
    {
    return 1 + flash.caps.addr_width;
    }
    static int mchp23k256_write(struct mtd_info *mtd, loff_t to, size_t len,
    size_t *retlen, const unsigned char *buf)
    {
    struct mchp23k256_flash *flash = to_mchp23k256_flash(mtd);
    struct spi_transfer transfer[2] = {};
    struct spi_message message;
    unsigned char command[MAX_CMD_SIZE];
    int ret, cmd_len;
    spi_message_init(&message);
    cmd_len = mchp23k256_cmdsz(flash);
    command[0] = MCHP23K256_CMD_WRITE;
    mchp23k256_addr2cmd(flash, to, command);
    transfer[0].tx_buf = command;
    transfer[0].len = cmd_len;
    spi_message_add_tail(&transfer[0], &message);
    transfer[1].tx_buf = buf;
    transfer[1].len = len;
    spi_message_add_tail(&transfer[1], &message);
    mutex_lock(&flash.lock);
    ret = spi_sync(flash.spi, &message);
    mutex_unlock(&flash.lock);
    if (ret)
    return ret;
    if (retlen && message.actual_length > cmd_len)
// retlen += message.actual_length - cmd_len;
    return 0;
    }
    static int mchp23k256_read(struct mtd_info *mtd, loff_t from, size_t len,
    size_t *retlen, unsigned char *buf)
    {
    struct mchp23k256_flash *flash = to_mchp23k256_flash(mtd);
    struct spi_transfer transfer[2] = {};
    struct spi_message message;
    unsigned char command[MAX_CMD_SIZE];
    int ret, cmd_len;
    spi_message_init(&message);
    cmd_len = mchp23k256_cmdsz(flash);
    memset(&transfer, 0, sizeof(transfer));
    command[0] = MCHP23K256_CMD_READ;
    mchp23k256_addr2cmd(flash, from, command);
    transfer[0].tx_buf = command;
    transfer[0].len = cmd_len;
    spi_message_add_tail(&transfer[0], &message);
    transfer[1].rx_buf = buf;
    transfer[1].len = len;
    spi_message_add_tail(&transfer[1], &message);
    mutex_lock(&flash.lock);
    ret = spi_sync(flash.spi, &message);
    mutex_unlock(&flash.lock);
    if (ret)
    return ret;
    if (retlen && message.actual_length > cmd_len)
// retlen += message.actual_length - cmd_len;
    return 0;
    }
//
// Set the device into sequential mode. This allows read/writes to the
// entire SRAM in a single operation
//
#[no_mangle]
unsafe extern "C" fn mchp23k256_set_mode(spi: *mut spi_device) -> c_int {
    static int mchp23k256_set_mode(struct spi_device *spi)
    {
    let mut transfer: spi_transfer = {};
    struct spi_message message;
    unsigned char command[2];
    spi_message_init(&message);
    command[0] = MCHP23K256_CMD_WRITE_STATUS;
    command[1] = MCHP23K256_MODE_SEQ;
    transfer.tx_buf = command;
    transfer.len = sizeof(command);
    spi_message_add_tail(&transfer, &message);
    return spi_sync(spi, &message);
    }
    static const struct mchp23_caps mchp23k256_caps = {
    .size = SZ_32K,
    .addr_width = 2,
    };
    static const struct mchp23_caps mchp23lcv1024_caps = {
    .size = SZ_128K,
    .addr_width = 3,
    };
#[no_mangle]
unsafe extern "C" fn mchp23k256_probe(spi: *mut spi_device) -> c_int {
    static int mchp23k256_probe(struct spi_device *spi)
    {
    struct mchp23k256_flash *flash;
    struct flash_platform_data *data;
    int err;
    flash = devm_kzalloc(&spi.dev, sizeof(*flash), GFP_KERNEL);
    if (!flash)
    return -ENOMEM;
    flash.spi = spi;
    mutex_init(&flash.lock);
    spi_set_drvdata(spi, flash);
    err = mchp23k256_set_mode(spi);
    if (err)
    return err;
    data = dev_get_platdata(&spi.dev);
    flash.caps = spi_get_device_match_data(spi);
    if (!flash.caps)
    flash.caps = &mchp23k256_caps;
    mtd_set_of_node(&flash.mtd, spi.dev.of_node);
    flash.mtd.dev.parent	= &spi.dev;
    flash.mtd.type		= MTD_RAM;
    flash.mtd.flags	= MTD_CAP_RAM;
    flash.mtd.writesize	= 1;
    flash.mtd.size		= flash.caps.size;
    flash.mtd._read	= mchp23k256_read;
    flash.mtd._write	= mchp23k256_write;
    err = mtd_device_register(&flash.mtd, data ? data.parts : core::ptr::null_mut(),
    data ? data.nr_parts : 0);
    if (err)
    return err;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mchp23k256_remove(spi: *mut spi_device) {
    static void mchp23k256_remove(struct spi_device *spi)
    {
    struct mchp23k256_flash *flash = spi_get_drvdata(spi);
    WARN_ON(mtd_device_unregister(&flash.mtd));
    }
    static const struct of_device_id mchp23k256_of_table[] = {
    {
    .compatible = "microchip,mchp23k256",
    .data = &mchp23k256_caps,
    },
    {
    .compatible = "microchip,mchp23lcv1024",
    .data = &mchp23lcv1024_caps,
    },
    {}
    };
    MODULE_DEVICE_TABLE(of, mchp23k256_of_table);
    static const struct spi_device_id mchp23k256_spi_ids[] = {
    {
    .name = "mchp23k256",
    .driver_data = (kernel_ulong_t)&mchp23k256_caps,
    },
    {
    .name = "mchp23lcv1024",
    .driver_data = (kernel_ulong_t)&mchp23lcv1024_caps,
    },
    {}
    };
    MODULE_DEVICE_TABLE(spi, mchp23k256_spi_ids);
    static struct spi_driver mchp23k256_driver = {
    .driver = {
    .name	= "mchp23k256",
    .of_match_table = mchp23k256_of_table,
    },
    .probe		= mchp23k256_probe,
    .remove		= mchp23k256_remove,
    .id_table	= mchp23k256_spi_ids,
    };
    module_spi_driver(mchp23k256_driver);
    MODULE_DESCRIPTION("MTD SPI driver for MCHP23K256 RAM chips");
    MODULE_AUTHOR("Andrew Lunn <andre@lunn.ch>");
    MODULE_LICENSE("GPL v2");
