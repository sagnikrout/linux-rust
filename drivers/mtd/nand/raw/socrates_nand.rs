//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/nand/raw/socrates_nand.c
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
// Copyright © 2008 Ilya Yanok, Emcraft Systems
//

pub const FPGA_NAND_DATA_SHIFT: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct socrates_nand_host {
    pub controller: nand_controller,
    pub nand_chip: nand_chip,
    pub io_base: *mut void __iomem,
    pub dev: *mut device,
}

//
// socrates_nand_write_buf -  write buffer to chip
// @this:	NAND chip object
// @buf:	data buffer
// @len:	number of bytes to write
//
    static void socrates_nand_write_buf(struct nand_chip *this, const uint8_t *buf,
    int len)
    {
    int i;
    struct socrates_nand_host *host = nand_get_controller_data(this);
    for (i = 0; i < len; i++) {
    out_be32(host.io_base, FPGA_NAND_ENABLE |
    FPGA_NAND_CMD_WRITE |
    (buf[i] << FPGA_NAND_DATA_SHIFT));
    }
    }
//
// socrates_nand_read_buf -  read chip data into buffer
// @this:	NAND chip object
// @buf:	buffer to store date
// @len:	number of bytes to read
//
    static void socrates_nand_read_buf(struct nand_chip *this, uint8_t *buf,
    int len)
    {
    int i;
    struct socrates_nand_host *host = nand_get_controller_data(this);
    uint32_t val;
    val = FPGA_NAND_ENABLE | FPGA_NAND_CMD_READ;
    out_be32(host.io_base, val);
    for (i = 0; i < len; i++) {
    buf[i] = (in_be32(host.io_base) >>
    FPGA_NAND_DATA_SHIFT) & 0xff;
    }
    }
//
// socrates_nand_read_byte -  read one byte from the chip
// @mtd:	MTD device structure
//
#[no_mangle]
unsafe extern "C" fn socrates_nand_read_byte(this: *mut nand_chip) -> u8 {
    static uint8_t socrates_nand_read_byte(struct nand_chip *this)
    {
    uint8_t byte;
    socrates_nand_read_buf(this, &byte, sizeof(byte));
    return byte;
    }
//
// Hardware specific access to control-lines
//
    static void socrates_nand_cmd_ctrl(struct nand_chip *nand_chip, int cmd,
    unsigned int ctrl)
    {
    struct socrates_nand_host *host = nand_get_controller_data(nand_chip);
    uint32_t val;
    if (cmd == NAND_CMD_NONE)
    return;
    if (ctrl & NAND_CLE)
    val = FPGA_NAND_CMD_COMMAND;
    else
    val = FPGA_NAND_CMD_ADDR;
    if (ctrl & NAND_NCE)
    val |= FPGA_NAND_ENABLE;
    val |= (cmd & 0xff) << FPGA_NAND_DATA_SHIFT;
    out_be32(host.io_base, val);
    }
//
// Read the Device Ready pin.
//
#[no_mangle]
unsafe extern "C" fn socrates_nand_device_ready(nand_chip: *mut nand_chip) -> c_int {
    static int socrates_nand_device_ready(struct nand_chip *nand_chip)
    {
    struct socrates_nand_host *host = nand_get_controller_data(nand_chip);
    if (in_be32(host.io_base) & FPGA_NAND_BUSY)
    return 0; /* busy */
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn socrates_attach_chip(chip: *mut nand_chip) -> c_int {
    static int socrates_attach_chip(struct nand_chip *chip)
    {
    if (chip.ecc.engine_type == NAND_ECC_ENGINE_TYPE_SOFT &&
    chip.ecc.algo == NAND_ECC_ALGO_UNKNOWN)
    chip.ecc.algo = NAND_ECC_ALGO_HAMMING;
    return 0;
    }
    static const struct nand_controller_ops socrates_ops = {
    .attach_chip = socrates_attach_chip,
    };
//
// Probe for the NAND device.
//
#[no_mangle]
unsafe extern "C" fn socrates_nand_probe(ofdev: *mut platform_device) -> c_int {
    static int socrates_nand_probe(struct platform_device *ofdev)
    {
    struct socrates_nand_host *host;
    struct mtd_info *mtd;
    struct nand_chip *nand_chip;
    int res;
// Allocate memory for the device structure (and zero it)
    host = devm_kzalloc(&ofdev.dev, sizeof(*host), GFP_KERNEL);
    if (!host)
    return -ENOMEM;
    host.io_base = of_iomap(ofdev.dev.of_node, 0);
    if (host.io_base == core::ptr::null_mut()) {
    dev_err(&ofdev.dev, "ioremap failed\n");
    return -EIO;
    }
    nand_chip = &host.nand_chip;
    mtd = nand_to_mtd(nand_chip);
    host.dev = &ofdev.dev;
    nand_controller_init(&host.controller);
    host.controller.ops = &socrates_ops;
    nand_chip.controller = &host.controller;
// link the private data structures
    nand_set_controller_data(nand_chip, host);
    nand_set_flash_node(nand_chip, ofdev.dev.of_node);
    mtd.name = "socrates_nand";
    mtd.dev.parent = &ofdev.dev;
    nand_chip.legacy.cmd_ctrl = socrates_nand_cmd_ctrl;
    nand_chip.legacy.read_byte = socrates_nand_read_byte;
    nand_chip.legacy.write_buf = socrates_nand_write_buf;
    nand_chip.legacy.read_buf = socrates_nand_read_buf;
    nand_chip.legacy.dev_ready = socrates_nand_device_ready;
// TODO: I have no idea what real delay is.
    nand_chip.legacy.chip_delay = 20;	/* 20us command delay time */
//
// This driver assumes that the default ECC engine should be TYPE_SOFT.
// Set ->engine_type before registering the NAND devices in order to
// provide a driver specific default value.
//
    nand_chip.ecc.engine_type = NAND_ECC_ENGINE_TYPE_SOFT;
    dev_set_drvdata(&ofdev.dev, host);
    res = nand_scan(nand_chip, 1);
    if (res)
    goto out;
    res = mtd_device_register(mtd, core::ptr::null_mut(), 0);
    if (!res)
    return res;
    nand_cleanup(nand_chip);
    out:
    iounmap(host.io_base);
    return res;
    }
//
// Remove a NAND device.
//
#[no_mangle]
unsafe extern "C" fn socrates_nand_remove(ofdev: *mut platform_device) {
    static void socrates_nand_remove(struct platform_device *ofdev)
    {
    struct socrates_nand_host *host = dev_get_drvdata(&ofdev.dev);
    struct nand_chip *chip = &host.nand_chip;
    int ret;
    ret = mtd_device_unregister(nand_to_mtd(chip));
    WARN_ON(ret);
    nand_cleanup(chip);
    iounmap(host.io_base);
    }
    static const struct of_device_id socrates_nand_match[] =
    {
    {
    .compatible   = "abb,socrates-nand",
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, socrates_nand_match);
    static struct platform_driver socrates_nand_driver = {
    .driver = {
    .name = "socrates_nand",
    .of_match_table = socrates_nand_match,
    },
    .probe		= socrates_nand_probe,
    .remove		= socrates_nand_remove,
    };
    module_platform_driver(socrates_nand_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Ilya Yanok");
    MODULE_DESCRIPTION("NAND driver for Socrates board");
