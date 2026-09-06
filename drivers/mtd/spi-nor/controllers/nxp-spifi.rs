//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/spi-nor/controllers/nxp-spifi.c
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
// SPI NOR driver for NXP SPI Flash Interface (SPIFI)
//
// Copyright (C) 2015 Joachim Eastwood <manabian@gmail.com>
//
// Based on Freescale QuadSPI driver:
// Copyright (C) 2013 Freescale Semiconductor, Inc.
//

// NXP SPIFI registers, bits and macros
pub const SPIFI_CTRL: c_uint = 0x000;

pub const SPIFI_CMD: c_uint = 0x004;

pub const SPIFI_ADDR: c_uint = 0x008;
pub const SPIFI_IDATA: c_uint = 0x00c;
pub const SPIFI_CLIMIT: c_uint = 0x010;
pub const SPIFI_DATA: c_uint = 0x014;
pub const SPIFI_MCMD: c_uint = 0x018;
pub const SPIFI_STAT: c_uint = 0x01c;

pub const SPI_NOR_MAX_ID_LEN: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxp_spifi {
    pub dev: *mut device,
    pub clk_spifi: *mut clk,
    pub clk_reg: *mut clk,
    pub io_base: *mut void __iomem,
    pub flash_base: *mut void __iomem,
    pub nor: spi_nor,
    pub memory_mode: bool,
    pub mcmd: u32,
}

#[no_mangle]
unsafe extern "C" fn nxp_spifi_wait_for_cmd(spifi: *mut nxp_spifi) -> c_int {
    static int nxp_spifi_wait_for_cmd(struct nxp_spifi *spifi)
    {
    u8 stat;
    int ret;
    ret = readb_poll_timeout(spifi.io_base + SPIFI_STAT, stat,
    !(stat & SPIFI_STAT_CMD), 10, 30);
    if (ret)
    dev_warn(spifi.dev, "command timed out\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn nxp_spifi_reset(spifi: *mut nxp_spifi) -> c_int {
    static int nxp_spifi_reset(struct nxp_spifi *spifi)
    {
    u8 stat;
    int ret;
    writel(SPIFI_STAT_RESET, spifi.io_base + SPIFI_STAT);
    ret = readb_poll_timeout(spifi.io_base + SPIFI_STAT, stat,
    !(stat & SPIFI_STAT_RESET), 10, 30);
    if (ret)
    dev_warn(spifi.dev, "state reset timed out\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn nxp_spifi_set_memory_mode_off(spifi: *mut nxp_spifi) -> c_int {
    static int nxp_spifi_set_memory_mode_off(struct nxp_spifi *spifi)
    {
    int ret;
    if (!spifi.memory_mode)
    return 0;
    ret = nxp_spifi_reset(spifi);
    if (ret)
    dev_err(spifi.dev, "unable to enter command mode\n");
    else
    spifi.memory_mode = false;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn nxp_spifi_set_memory_mode_on(spifi: *mut nxp_spifi) -> c_int {
    static int nxp_spifi_set_memory_mode_on(struct nxp_spifi *spifi)
    {
    u8 stat;
    int ret;
    if (spifi.memory_mode)
    return 0;
    writel(spifi.mcmd, spifi.io_base + SPIFI_MCMD);
    ret = readb_poll_timeout(spifi.io_base + SPIFI_STAT, stat,
    stat & SPIFI_STAT_MCINIT, 10, 30);
    if (ret)
    dev_err(spifi.dev, "unable to enter memory mode\n");
    else
    spifi.memory_mode = true;
    return ret;
    }
    static int nxp_spifi_read_reg(struct spi_nor *nor, u8 opcode, u8 *buf,
    size_t len)
    {
    struct nxp_spifi *spifi = nor.priv;
    u32 cmd;
    int ret;
    ret = nxp_spifi_set_memory_mode_off(spifi);
    if (ret)
    return ret;
    cmd = SPIFI_CMD_DATALEN(len) |
    SPIFI_CMD_OPCODE(opcode) |
    SPIFI_CMD_FIELDFORM_ALL_SERIAL |
    SPIFI_CMD_FRAMEFORM_OPCODE_ONLY;
    writel(cmd, spifi.io_base + SPIFI_CMD);
    while (len--)
// buf++ = readb(spifi->io_base + SPIFI_DATA);
    return nxp_spifi_wait_for_cmd(spifi);
    }
    static int nxp_spifi_write_reg(struct spi_nor *nor, u8 opcode, const u8 *buf,
    size_t len)
    {
    struct nxp_spifi *spifi = nor.priv;
    u32 cmd;
    int ret;
    ret = nxp_spifi_set_memory_mode_off(spifi);
    if (ret)
    return ret;
    cmd = SPIFI_CMD_DOUT |
    SPIFI_CMD_DATALEN(len) |
    SPIFI_CMD_OPCODE(opcode) |
    SPIFI_CMD_FIELDFORM_ALL_SERIAL |
    SPIFI_CMD_FRAMEFORM_OPCODE_ONLY;
    writel(cmd, spifi.io_base + SPIFI_CMD);
    while (len--)
    writeb(*buf++, spifi.io_base + SPIFI_DATA);
    return nxp_spifi_wait_for_cmd(spifi);
    }
    static ssize_t nxp_spifi_read(struct spi_nor *nor, loff_t from, size_t len,
    u_char *buf)
    {
    struct nxp_spifi *spifi = nor.priv;
    int ret;
    ret = nxp_spifi_set_memory_mode_on(spifi);
    if (ret)
    return ret;
    memcpy_fromio(buf, spifi.flash_base + from, len);
    return len;
    }
    static ssize_t nxp_spifi_write(struct spi_nor *nor, loff_t to, size_t len,
    const u_char *buf)
    {
    struct nxp_spifi *spifi = nor.priv;
    u32 cmd;
    int ret;
    size_t i;
    ret = nxp_spifi_set_memory_mode_off(spifi);
    if (ret)
    return ret;
    writel(to, spifi.io_base + SPIFI_ADDR);
    cmd = SPIFI_CMD_DOUT |
    SPIFI_CMD_DATALEN(len) |
    SPIFI_CMD_FIELDFORM_ALL_SERIAL |
    SPIFI_CMD_OPCODE(nor.program_opcode) |
    SPIFI_CMD_FRAMEFORM(spifi.nor.addr_nbytes + 1);
    writel(cmd, spifi.io_base + SPIFI_CMD);
    for (i = 0; i < len; i++)
    writeb(buf[i], spifi.io_base + SPIFI_DATA);
    ret = nxp_spifi_wait_for_cmd(spifi);
    if (ret)
    return ret;
    return len;
    }
#[no_mangle]
unsafe extern "C" fn nxp_spifi_erase(nor: *mut spi_nor, offs: loff_t) -> c_int {
    static int nxp_spifi_erase(struct spi_nor *nor, loff_t offs)
    {
    struct nxp_spifi *spifi = nor.priv;
    u32 cmd;
    int ret;
    ret = nxp_spifi_set_memory_mode_off(spifi);
    if (ret)
    return ret;
    writel(offs, spifi.io_base + SPIFI_ADDR);
    cmd = SPIFI_CMD_FIELDFORM_ALL_SERIAL |
    SPIFI_CMD_OPCODE(nor.erase_opcode) |
    SPIFI_CMD_FRAMEFORM(spifi.nor.addr_nbytes + 1);
    writel(cmd, spifi.io_base + SPIFI_CMD);
    return nxp_spifi_wait_for_cmd(spifi);
    }
#[no_mangle]
unsafe extern "C" fn nxp_spifi_setup_memory_cmd(spifi: *mut nxp_spifi) -> c_int {
    static int nxp_spifi_setup_memory_cmd(struct nxp_spifi *spifi)
    {
    switch (spifi.nor.read_proto) {
    case SNOR_PROTO_1_1_1:
    spifi.mcmd = SPIFI_CMD_FIELDFORM_ALL_SERIAL;
    break;
    case SNOR_PROTO_1_1_2:
    case SNOR_PROTO_1_1_4:
    spifi.mcmd = SPIFI_CMD_FIELDFORM_QUAD_DUAL_DATA;
    break;
    default:
    dev_err(spifi.dev, "unsupported SPI read mode\n");
    return -EINVAL;
    }
// Memory mode supports address length between 1 and 4
    if (spifi.nor.addr_nbytes < 1 || spifi.nor.addr_nbytes > 4)
    return -EINVAL;
    spifi.mcmd |= SPIFI_CMD_OPCODE(spifi.nor.read_opcode) |
    SPIFI_CMD_INTLEN(spifi.nor.read_dummy / 8) |
    SPIFI_CMD_FRAMEFORM(spifi.nor.addr_nbytes + 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nxp_spifi_dummy_id_read(nor: *mut spi_nor) {
    static void nxp_spifi_dummy_id_read(struct spi_nor *nor)
    {
    u8 id[SPI_NOR_MAX_ID_LEN];
    nor.controller_ops.read_reg(nor, SPINOR_OP_RDID, id,
    SPI_NOR_MAX_ID_LEN);
    }
    static const struct spi_nor_controller_ops nxp_spifi_controller_ops = {
    .read_reg  = nxp_spifi_read_reg,
    .write_reg = nxp_spifi_write_reg,
    .read  = nxp_spifi_read,
    .write = nxp_spifi_write,
    .erase = nxp_spifi_erase,
    };
    static int nxp_spifi_setup_flash(struct nxp_spifi *spifi,
    struct device_node *np)
    {
    struct spi_nor_hwcaps hwcaps = {
    .mask = SNOR_HWCAPS_READ |
    SNOR_HWCAPS_READ_FAST |
    SNOR_HWCAPS_PP,
    };
    u32 ctrl, property;
    let mut mode: u16 = 0;
    int ret;
    if (!of_property_read_u32(np, "spi-rx-bus-width", &property)) {
    switch (property) {
    case 1:
    break;
    case 2:
    mode |= SPI_RX_DUAL;
    break;
    case 4:
    mode |= SPI_RX_QUAD;
    break;
    default:
    dev_err(spifi.dev, "unsupported rx-bus-width\n");
    return -EINVAL;
    }
    }
    if (of_property_read_bool(np, "spi-cpha"))
    mode |= SPI_CPHA;
    if (of_property_read_bool(np, "spi-cpol"))
    mode |= SPI_CPOL;
// Setup control register defaults
    ctrl = SPIFI_CTRL_TIMEOUT(1000) |
    SPIFI_CTRL_CSHIGH(15) |
    SPIFI_CTRL_FBCLK;
    if (mode & SPI_RX_DUAL) {
    ctrl |= SPIFI_CTRL_DUAL;
    hwcaps.mask |= SNOR_HWCAPS_READ_1_1_2;
    } else if (mode & SPI_RX_QUAD) {
    ctrl &= ~SPIFI_CTRL_DUAL;
    hwcaps.mask |= SNOR_HWCAPS_READ_1_1_4;
    } else {
    ctrl |= SPIFI_CTRL_DUAL;
    }
    switch (mode & SPI_MODE_X_MASK) {
    case SPI_MODE_0:
    ctrl &= ~SPIFI_CTRL_MODE3;
    break;
    case SPI_MODE_3:
    ctrl |= SPIFI_CTRL_MODE3;
    break;
    default:
    dev_err(spifi.dev, "only mode 0 and 3 supported\n");
    return -EINVAL;
    }
    writel(ctrl, spifi.io_base + SPIFI_CTRL);
    spifi.nor.dev   = spifi.dev;
    spi_nor_set_flash_node(&spifi.nor, np);
    spifi.nor.priv  = spifi;
    spifi.nor.controller_ops = &nxp_spifi_controller_ops;
//
// The first read on a hard reset isn't reliable so do a
// dummy read of the id before calling spi_nor_scan().
// The reason for this problem is unknown.
//
// The official NXP spifilib uses more or less the same
// workaround that is applied here by reading the device
// id multiple times.
//
    nxp_spifi_dummy_id_read(&spifi.nor);
    ret = spi_nor_scan(&spifi.nor, core::ptr::null_mut(), &hwcaps);
    if (ret) {
    dev_err(spifi.dev, "device scan failed\n");
    return ret;
    }
    ret = nxp_spifi_setup_memory_cmd(spifi);
    if (ret) {
    dev_err(spifi.dev, "memory command setup failed\n");
    return ret;
    }
    ret = mtd_device_register(&spifi.nor.mtd, core::ptr::null_mut(), 0);
    if (ret) {
    dev_err(spifi.dev, "mtd device parse failed\n");
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nxp_spifi_probe(pdev: *mut platform_device) -> c_int {
    static int nxp_spifi_probe(struct platform_device *pdev)
    {
    struct device_node *flash_np;
    struct nxp_spifi *spifi;
    int ret;
    spifi = devm_kzalloc(&pdev.dev, sizeof(*spifi), GFP_KERNEL);
    if (!spifi)
    return -ENOMEM;
    spifi.io_base = devm_platform_ioremap_resource_byname(pdev, "spifi");
    if (IS_ERR(spifi.io_base))
    return PTR_ERR(spifi.io_base);
    spifi.flash_base = devm_platform_ioremap_resource_byname(pdev, "flash");
    if (IS_ERR(spifi.flash_base))
    return PTR_ERR(spifi.flash_base);
    spifi.clk_spifi = devm_clk_get_enabled(&pdev.dev, "spifi");
    if (IS_ERR(spifi.clk_spifi)) {
    dev_err(&pdev.dev, "spifi clock not found or unable to enable\n");
    return PTR_ERR(spifi.clk_spifi);
    }
    spifi.clk_reg = devm_clk_get_enabled(&pdev.dev, "reg");
    if (IS_ERR(spifi.clk_reg)) {
    dev_err(&pdev.dev, "reg clock not found or unable to enable\n");
    return PTR_ERR(spifi.clk_reg);
    }
    spifi.dev = &pdev.dev;
    platform_set_drvdata(pdev, spifi);
// Initialize and reset device
    nxp_spifi_reset(spifi);
    writel(0, spifi.io_base + SPIFI_IDATA);
    writel(0, spifi.io_base + SPIFI_MCMD);
    nxp_spifi_reset(spifi);
    flash_np = of_get_next_available_child(pdev.dev.of_node, core::ptr::null_mut());
    if (!flash_np) {
    dev_err(&pdev.dev, "no SPI flash device to configure\n");
    return -ENODEV;
    }
    ret = nxp_spifi_setup_flash(spifi, flash_np);
    of_node_put(flash_np);
    if (ret) {
    dev_err(&pdev.dev, "unable to setup flash chip\n");
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nxp_spifi_remove(pdev: *mut platform_device) {
    static void nxp_spifi_remove(struct platform_device *pdev)
    {
    struct nxp_spifi *spifi = platform_get_drvdata(pdev);
    mtd_device_unregister(&spifi.nor.mtd);
    }
    static const struct of_device_id nxp_spifi_match[] = {
    {.compatible = "nxp,lpc1773-spifi"},
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, nxp_spifi_match);
    static struct platform_driver nxp_spifi_driver = {
    .probe	= nxp_spifi_probe,
    .remove = nxp_spifi_remove,
    .driver	= {
    .name = "nxp-spifi",
    .of_match_table = nxp_spifi_match,
    },
    };
    module_platform_driver(nxp_spifi_driver);
    MODULE_DESCRIPTION("NXP SPI Flash Interface driver");
    MODULE_AUTHOR("Joachim Eastwood <manabian@gmail.com>");
    MODULE_LICENSE("GPL v2");
