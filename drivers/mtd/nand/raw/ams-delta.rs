//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/nand/raw/ams-delta.c
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
// Copyright (C) 2006 Jonathan McDowell <noodles@earth.li>
//
// Derived from drivers/mtd/nand/toto.c (removed in v2.6.28)
// Copyright (c) 2003 Texas Instruments
// Copyright (c) 2002 Thomas Gleixner <tgxl@linutronix.de>
//
// Converted to platform driver by Janusz Krzysztofik <jkrzyszt@tis.icnet.pl>
// Partially stolen from plat_nand.c
//
// Overview:
// This is a device driver for the NAND flash device found on the
// Amstrad E3 (Delta).
//

//
// MTD structure for E3 (Delta)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_nand {
    pub base: nand_controller,
    pub nand_chip: nand_chip,
    pub gpiod_rdy: *mut gpio_desc,
    pub gpiod_nce: *mut gpio_desc,
    pub gpiod_nre: *mut gpio_desc,
    pub gpiod_nwp: *mut gpio_desc,
    pub gpiod_nwe: *mut gpio_desc,
    pub gpiod_ale: *mut gpio_desc,
    pub gpiod_cle: *mut gpio_desc,
    pub data_gpiods: *mut gpio_descs,
    pub data_in: bool,
    pub tRP: c_uint,
    pub tWP: c_uint,
    pub this): *mut *mut u8 (io_read)(struct gpio_nand,
    pub byte): *mut *mut *mut void (io_write)(struct gpio_nand this, u8,
}

#[no_mangle]
unsafe extern "C" fn gpio_nand_write_commit(priv: *mut gpio_nand) {
    static void gpio_nand_write_commit(struct gpio_nand *priv)
    {
    gpiod_set_value(priv.gpiod_nwe, 1);
    ndelay(priv.tWP);
    gpiod_set_value(priv.gpiod_nwe, 0);
    }
#[no_mangle]
unsafe extern "C" fn gpio_nand_io_write(priv: *mut gpio_nand, byte: u8) {
    static void gpio_nand_io_write(struct gpio_nand *priv, u8 byte)
    {
    struct gpio_descs *data_gpiods = priv.data_gpiods;
    DECLARE_BITMAP(values, BITS_PER_TYPE(byte)) = { byte, };
    gpiod_set_raw_array_value(data_gpiods.ndescs, data_gpiods.desc,
    data_gpiods.info, values);
    gpio_nand_write_commit(priv);
    }
#[no_mangle]
unsafe extern "C" fn gpio_nand_dir_output(priv: *mut gpio_nand, byte: u8) {
    static void gpio_nand_dir_output(struct gpio_nand *priv, u8 byte)
    {
    struct gpio_descs *data_gpiods = priv.data_gpiods;
    DECLARE_BITMAP(values, BITS_PER_TYPE(byte)) = { byte, };
    int i;
    for (i = 0; i < data_gpiods.ndescs; i++)
    gpiod_direction_output_raw(data_gpiods.desc[i],
    test_bit(i, values));
    gpio_nand_write_commit(priv);
    priv.data_in = false;
    }
#[no_mangle]
unsafe extern "C" fn gpio_nand_io_read(priv: *mut gpio_nand) -> u8 {
    static u8 gpio_nand_io_read(struct gpio_nand *priv)
    {
    u8 res;
    struct gpio_descs *data_gpiods = priv.data_gpiods;
    DECLARE_BITMAP(values, BITS_PER_TYPE(res)) = { 0, };
    gpiod_set_value(priv.gpiod_nre, 1);
    ndelay(priv.tRP);
    gpiod_get_raw_array_value(data_gpiods.ndescs, data_gpiods.desc,
    data_gpiods.info, values);
    gpiod_set_value(priv.gpiod_nre, 0);
    res = values[0];
    return res;
    }
#[no_mangle]
unsafe extern "C" fn gpio_nand_dir_input(priv: *mut gpio_nand) {
    static void gpio_nand_dir_input(struct gpio_nand *priv)
    {
    struct gpio_descs *data_gpiods = priv.data_gpiods;
    int i;
    for (i = 0; i < data_gpiods.ndescs; i++)
    gpiod_direction_input(data_gpiods.desc[i]);
    priv.data_in = true;
    }
#[no_mangle]
unsafe extern "C" fn gpio_nand_write_buf(priv: *mut gpio_nand, buf: *const u8, len: c_int) {
    static void gpio_nand_write_buf(struct gpio_nand *priv, const u8 *buf, int len)
    {
    let mut i: c_int = 0;
    if (len > 0 && priv.data_in)
    gpio_nand_dir_output(priv, buf[i++]);
    while (i < len)
    priv.io_write(priv, buf[i++]);
    }
#[no_mangle]
unsafe extern "C" fn gpio_nand_read_buf(priv: *mut gpio_nand, buf: *mut u8, len: c_int) {
    static void gpio_nand_read_buf(struct gpio_nand *priv, u8 *buf, int len)
    {
    int i;
    if (priv.data_gpiods && !priv.data_in)
    gpio_nand_dir_input(priv);
    for (i = 0; i < len; i++)
    buf[i] = priv.io_read(priv);
    }
#[no_mangle]
unsafe extern "C" fn gpio_nand_ctrl_cs(priv: *mut gpio_nand, assert: bool) {
    static void gpio_nand_ctrl_cs(struct gpio_nand *priv, bool assert)
    {
    gpiod_set_value(priv.gpiod_nce, assert);
    }
    static int gpio_nand_exec_op(struct nand_chip *this,
    const struct nand_operation *op, bool check_only)
    {
    struct gpio_nand *priv = nand_get_controller_data(this);
    const struct nand_op_instr *instr;
    let mut ret: c_int = 0;
    if (check_only)
    return 0;
    gpio_nand_ctrl_cs(priv, 1);
    for (instr = op.instrs; instr < op.instrs + op.ninstrs; instr++) {
    switch (instr.type) {
    case NAND_OP_CMD_INSTR:
    gpiod_set_value(priv.gpiod_cle, 1);
    gpio_nand_write_buf(priv, &instr.ctx.cmd.opcode, 1);
    gpiod_set_value(priv.gpiod_cle, 0);
    break;
    case NAND_OP_ADDR_INSTR:
    gpiod_set_value(priv.gpiod_ale, 1);
    gpio_nand_write_buf(priv, instr.ctx.addr.addrs,
    instr.ctx.addr.naddrs);
    gpiod_set_value(priv.gpiod_ale, 0);
    break;
    case NAND_OP_DATA_IN_INSTR:
    gpio_nand_read_buf(priv, instr.ctx.data.buf.in,
    instr.ctx.data.len);
    break;
    case NAND_OP_DATA_OUT_INSTR:
    gpio_nand_write_buf(priv, instr.ctx.data.buf.out,
    instr.ctx.data.len);
    break;
    case NAND_OP_WAITRDY_INSTR:
    ret = priv.gpiod_rdy ?
    nand_gpio_waitrdy(this, priv.gpiod_rdy,
    instr.ctx.waitrdy.timeout_ms) :
    nand_soft_waitrdy(this,
    instr.ctx.waitrdy.timeout_ms);
    break;
    }
    if (ret)
    break;
    }
    gpio_nand_ctrl_cs(priv, 0);
    return ret;
    }
    static int gpio_nand_setup_interface(struct nand_chip *this, int csline,
    const struct nand_interface_config *cf)
    {
    struct gpio_nand *priv = nand_get_controller_data(this);
    const struct nand_sdr_timings *sdr = nand_get_sdr_timings(cf);
    struct device *dev = &nand_to_mtd(this).dev;
    if (IS_ERR(sdr))
    return PTR_ERR(sdr);
    if (csline == NAND_DATA_IFACE_CHECK_ONLY)
    return 0;
    if (priv.gpiod_nre) {
    priv.tRP = DIV_ROUND_UP(sdr.tRP_min, 1000);
    dev_dbg(dev, "using %u ns read pulse width\n", priv.tRP);
    }
    priv.tWP = DIV_ROUND_UP(sdr.tWP_min, 1000);
    dev_dbg(dev, "using %u ns write pulse width\n", priv.tWP);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gpio_nand_attach_chip(chip: *mut nand_chip) -> c_int {
    static int gpio_nand_attach_chip(struct nand_chip *chip)
    {
    if (chip.ecc.engine_type == NAND_ECC_ENGINE_TYPE_SOFT &&
    chip.ecc.algo == NAND_ECC_ALGO_UNKNOWN)
    chip.ecc.algo = NAND_ECC_ALGO_HAMMING;
    return 0;
    }
    static const struct nand_controller_ops gpio_nand_ops = {
    .exec_op = gpio_nand_exec_op,
    .attach_chip = gpio_nand_attach_chip,
    .setup_interface = gpio_nand_setup_interface,
    };
//
// Main initialization routine
//
#[no_mangle]
unsafe extern "C" fn gpio_nand_probe(pdev: *mut platform_device) -> c_int {
    static int gpio_nand_probe(struct platform_device *pdev)
    {
    struct gpio_nand_platdata *pdata = dev_get_platdata(&pdev.dev);
    const struct mtd_partition *partitions = core::ptr::null_mut();
    let mut num_partitions: c_int = 0;
    struct gpio_nand *priv;
    struct nand_chip *this;
    struct mtd_info *mtd;
    int (*probe)(struct platform_device *pdev, struct gpio_nand *priv);
    let mut err: c_int = 0;
    if (pdata) {
    partitions = pdata.parts;
    num_partitions = pdata.num_parts;
    }
// Allocate memory for MTD device structure and private data
    priv = devm_kzalloc(&pdev.dev, sizeof(struct gpio_nand),
    GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    this = &priv.nand_chip;
    mtd = nand_to_mtd(this);
    mtd.dev.parent = &pdev.dev;
    nand_set_controller_data(this, priv);
    nand_set_flash_node(this, pdev.dev.of_node);
    priv.gpiod_rdy = devm_gpiod_get_optional(&pdev.dev, "rdy", GPIOD_IN);
    if (IS_ERR(priv.gpiod_rdy)) {
    err = PTR_ERR(priv.gpiod_rdy);
    dev_warn(&pdev.dev, "RDY GPIO request failed (%d)\n", err);
    return err;
    }
    platform_set_drvdata(pdev, priv);
// Set chip enabled but write protected
    priv.gpiod_nwp = devm_gpiod_get_optional(&pdev.dev, "nwp",
    GPIOD_OUT_HIGH);
    if (IS_ERR(priv.gpiod_nwp)) {
    err = PTR_ERR(priv.gpiod_nwp);
    dev_err(&pdev.dev, "NWP GPIO request failed (%d)\n", err);
    return err;
    }
    priv.gpiod_nce = devm_gpiod_get_optional(&pdev.dev, "nce",
    GPIOD_OUT_LOW);
    if (IS_ERR(priv.gpiod_nce)) {
    err = PTR_ERR(priv.gpiod_nce);
    dev_err(&pdev.dev, "NCE GPIO request failed (%d)\n", err);
    return err;
    }
    priv.gpiod_nre = devm_gpiod_get_optional(&pdev.dev, "nre",
    GPIOD_OUT_LOW);
    if (IS_ERR(priv.gpiod_nre)) {
    err = PTR_ERR(priv.gpiod_nre);
    dev_err(&pdev.dev, "NRE GPIO request failed (%d)\n", err);
    return err;
    }
    priv.gpiod_nwe = devm_gpiod_get_optional(&pdev.dev, "nwe",
    GPIOD_OUT_LOW);
    if (IS_ERR(priv.gpiod_nwe)) {
    err = PTR_ERR(priv.gpiod_nwe);
    dev_err(&pdev.dev, "NWE GPIO request failed (%d)\n", err);
    return err;
    }
    priv.gpiod_ale = devm_gpiod_get(&pdev.dev, "ale", GPIOD_OUT_LOW);
    if (IS_ERR(priv.gpiod_ale)) {
    err = PTR_ERR(priv.gpiod_ale);
    dev_err(&pdev.dev, "ALE GPIO request failed (%d)\n", err);
    return err;
    }
    priv.gpiod_cle = devm_gpiod_get(&pdev.dev, "cle", GPIOD_OUT_LOW);
    if (IS_ERR(priv.gpiod_cle)) {
    err = PTR_ERR(priv.gpiod_cle);
    dev_err(&pdev.dev, "CLE GPIO request failed (%d)\n", err);
    return err;
    }
// Request array of data pins, initialize them as input
    priv.data_gpiods = devm_gpiod_get_array_optional(&pdev.dev, "data",
    GPIOD_IN);
    if (IS_ERR(priv.data_gpiods)) {
    err = PTR_ERR(priv.data_gpiods);
    dev_err(&pdev.dev, "data GPIO request failed: %d\n", err);
    return err;
    }
    if (priv.data_gpiods) {
    if (!priv.gpiod_nwe) {
    dev_err(&pdev.dev,
    "mandatory NWE pin not provided by platform\n");
    return -ENODEV;
    }
    priv.io_read = gpio_nand_io_read;
    priv.io_write = gpio_nand_io_write;
    priv.data_in = true;
    }
    if (pdev.id_entry)
    probe = (void *) pdev.id_entry.driver_data;
    else
    probe = of_device_get_match_data(&pdev.dev);
    if (probe)
    err = probe(pdev, priv);
    if (err)
    return err;
    if (!priv.io_read || !priv.io_write) {
    dev_err(&pdev.dev, "incomplete device configuration\n");
    return -ENODEV;
    }
// Initialize the NAND controller object embedded in gpio_nand.
    priv.base.ops = &gpio_nand_ops;
    nand_controller_init(&priv.base);
    this.controller = &priv.base;
//
// FIXME: We should release write protection only after nand_scan() to
// be on the safe side but we can't do that until we have a generic way
// to assert/deassert WP from the core.  Even if the core shouldn't
// write things in the nand_scan() path, it should have control on this
// pin just in case we ever need to disable write protection during
// chip detection/initialization.
//
// Release write protection
    gpiod_set_value(priv.gpiod_nwp, 0);
//
// This driver assumes that the default ECC engine should be TYPE_SOFT.
// Set ->engine_type before registering the NAND devices in order to
// provide a driver specific default value.
//
    this.ecc.engine_type = NAND_ECC_ENGINE_TYPE_SOFT;
// Scan to find existence of the device
    err = nand_scan(this, 1);
    if (err)
    return err;
// Register the partitions
    err = mtd_device_register(mtd, partitions, num_partitions);
    if (err)
    goto err_nand_cleanup;
    return 0;
    err_nand_cleanup:
    nand_cleanup(this);
    return err;
    }
//
// Clean up routine
//
#[no_mangle]
unsafe extern "C" fn gpio_nand_remove(pdev: *mut platform_device) {
    static void gpio_nand_remove(struct platform_device *pdev)
    {
    struct gpio_nand *priv = platform_get_drvdata(pdev);
    struct mtd_info *mtd = nand_to_mtd(&priv.nand_chip);
    int ret;
// Apply write protection
    gpiod_set_value(priv.gpiod_nwp, 1);
// Unregister device
    ret = mtd_device_unregister(mtd);
    WARN_ON(ret);
    nand_cleanup(mtd_to_nand(mtd));
    }

    static const struct of_device_id gpio_nand_of_id_table[] = {
    {
// sentinel
    },
    };
    MODULE_DEVICE_TABLE(of, gpio_nand_of_id_table);

    static const struct platform_device_id gpio_nand_plat_id_table[] = {
    {
    .name	= "ams-delta-nand",
    }, {
// sentinel
    },
    };
    MODULE_DEVICE_TABLE(platform, gpio_nand_plat_id_table);
    static struct platform_driver gpio_nand_driver = {
    .probe		= gpio_nand_probe,
    .remove		= gpio_nand_remove,
    .id_table	= gpio_nand_plat_id_table,
    .driver		= {
    .name	= "ams-delta-nand",
    .of_match_table = of_match_ptr(gpio_nand_of_id_table),
    },
    };
    module_platform_driver(gpio_nand_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Jonathan McDowell <noodles@earth.li>");
    MODULE_DESCRIPTION("Glue layer for NAND flash on Amstrad E3 (Delta)");
