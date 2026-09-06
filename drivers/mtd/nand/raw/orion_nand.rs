//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/nand/raw/orion_nand.c
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


//
// NAND support for Marvell Orion SoC platforms
//
// Tzachi Perelstein <tzachi@marvell.com>
//
// This file is licensed under  the terms of the GNU General Public
// License version 2. This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orion_nand_info {
    pub controller: nand_controller,
    pub chip: nand_chip,
    pub clk: *mut clk,
}

    static void orion_nand_cmd_ctrl(struct nand_chip *nc, int cmd,
    unsigned int ctrl)
    {
    struct orion_nand_data *board = nand_get_controller_data(nc);
    u32 offs;
    if (cmd == NAND_CMD_NONE)
    return;
    if (ctrl & NAND_CLE)
    offs = (1 << board.cle);
#[no_mangle]
pub unsafe extern "C" fn if(NAND_ALE: ctrl &) -> else {
    else if (ctrl & NAND_ALE)
    offs = (1 << board.ale);
    else
    return;
    if (nc.options & NAND_BUSWIDTH_16)
    offs <<= 1;
    writeb(cmd, nc.legacy.IO_ADDR_W + offs);
    }
#[no_mangle]
unsafe extern "C" fn orion_nand_read_buf(chip: *mut nand_chip, buf: *mut u8, len: c_int) {
    static void orion_nand_read_buf(struct nand_chip *chip, uint8_t *buf, int len)
    {
    void __iomem *io_base = chip.legacy.IO_ADDR_R;

    uint64_t *buf64;

    let mut i: c_int = 0;
    while (len && (unsigned long)buf & 7) {
// buf++ = readb(io_base);
    len--;
    }

    buf64 = (uint64_t *)buf;
    while (i < len/8) {
//
// Since GCC has no proper constraint (PR 43518)
// force x variable to r2/r3 registers as ldrd instruction
// requires first register to be even.
//
    register uint64_t x asm ("r2");
    asm volatile ("ldrd\t%0, [%1]" : "=&r" (x) : "r" (io_base));
    buf64[i++] = x;
    }
    i *= 8;

    readsl(io_base, buf, len/4);
    i = len / 4 * 4;

    while (i < len)
    buf[i++] = readb(io_base);
    }
#[no_mangle]
unsafe extern "C" fn orion_nand_attach_chip(chip: *mut nand_chip) -> c_int {
    static int orion_nand_attach_chip(struct nand_chip *chip)
    {
    if (chip.ecc.engine_type == NAND_ECC_ENGINE_TYPE_SOFT &&
    chip.ecc.algo == NAND_ECC_ALGO_UNKNOWN)
    chip.ecc.algo = NAND_ECC_ALGO_HAMMING;
    return 0;
    }
    static const struct nand_controller_ops orion_nand_ops = {
    .attach_chip = orion_nand_attach_chip,
    };
#[no_mangle]
unsafe extern "C" fn orion_nand_probe(pdev: *mut platform_device) -> int __init {
    static int __init orion_nand_probe(struct platform_device *pdev)
    {
    struct orion_nand_info *info;
    struct mtd_info *mtd;
    struct nand_chip *nc;
    struct orion_nand_data *board;
    void __iomem *io_base;
    let mut ret: c_int = 0;
    let mut val: u32 = 0;
    info = devm_kzalloc(&pdev.dev,
    sizeof(struct orion_nand_info),
    GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    nc = &info.chip;
    mtd = nand_to_mtd(nc);
    nand_controller_init(&info.controller);
    info.controller.ops = &orion_nand_ops;
    nc.controller = &info.controller;
    io_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(io_base))
    return PTR_ERR(io_base);
    if (pdev.dev.of_node) {
    board = devm_kzalloc(&pdev.dev, sizeof(struct orion_nand_data),
    GFP_KERNEL);
    if (!board)
    return -ENOMEM;
    if (!of_property_read_u32(pdev.dev.of_node, "cle", &val))
    board.cle = (u8)val;
    else
    board.cle = 0;
    if (!of_property_read_u32(pdev.dev.of_node, "ale", &val))
    board.ale = (u8)val;
    else
    board.ale = 1;
    if (!of_property_read_u32(pdev.dev.of_node,
    "bank-width", &val))
    board.width = (u8)val * 8;
    else
    board.width = 8;
    if (!of_property_read_u32(pdev.dev.of_node,
    "chip-delay", &val))
    board.chip_delay = (u8)val;
    } else {
    board = dev_get_platdata(&pdev.dev);
    }
    mtd.dev.parent = &pdev.dev;
    nand_set_controller_data(nc, board);
    nand_set_flash_node(nc, pdev.dev.of_node);
    nc.legacy.IO_ADDR_R = nc.legacy.IO_ADDR_W = io_base;
    nc.legacy.cmd_ctrl = orion_nand_cmd_ctrl;
    nc.legacy.read_buf = orion_nand_read_buf;
    if (board.chip_delay)
    nc.legacy.chip_delay = board.chip_delay;
    WARN(board.width > 16,
    "%d bit bus width out of range",
    board.width);
    if (board.width == 16)
    nc.options |= NAND_BUSWIDTH_16;
    platform_set_drvdata(pdev, info);
// Not all platforms can gate the clock, so it is optional.
    info.clk = devm_clk_get_optional_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(info.clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(info.clk),
    "failed to get and enable clock!\n");
//
// This driver assumes that the default ECC engine should be TYPE_SOFT.
// Set ->engine_type before registering the NAND devices in order to
// provide a driver specific default value.
//
    nc.ecc.engine_type = NAND_ECC_ENGINE_TYPE_SOFT;
    ret = nand_scan(nc, 1);
    if (ret)
    return ret;
    mtd.name = "orion_nand";
    ret = mtd_device_register(mtd, board.parts, board.nr_parts);
    if (ret)
    nand_cleanup(nc);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn orion_nand_remove(pdev: *mut platform_device) {
    static void orion_nand_remove(struct platform_device *pdev)
    {
    struct orion_nand_info *info = platform_get_drvdata(pdev);
    struct nand_chip *chip = &info.chip;
    int ret;
    ret = mtd_device_unregister(nand_to_mtd(chip));
    WARN_ON(ret);
    nand_cleanup(chip);
    }

    static const struct of_device_id orion_nand_of_match_table[] = {
    { .compatible = "marvell,orion-nand", },
    {},
    };
    MODULE_DEVICE_TABLE(of, orion_nand_of_match_table);

    static struct platform_driver orion_nand_driver = {
    .remove		= orion_nand_remove,
    .driver		= {
    .name	= "orion_nand",
    .of_match_table = of_match_ptr(orion_nand_of_match_table),
    },
    };
    module_platform_driver_probe(orion_nand_driver, orion_nand_probe);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Tzachi Perelstein");
    MODULE_DESCRIPTION("NAND glue for Orion platforms");
    MODULE_ALIAS("platform:orion_nand");
