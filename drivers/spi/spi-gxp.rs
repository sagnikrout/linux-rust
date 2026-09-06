//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-gxp.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) 2022 Hewlett-Packard Development Company, L.P.

pub const GXP_SPI0_MAX_CHIPSELECT: c_int = 2;
pub const GXP_SPI_SLEEP_TIME: c_int = 1;

pub const MANUAL_MODE: c_int = 0;
pub const DIRECT_MODE: c_int = 1;
pub const SPILDAT_LEN: c_int = 256;
pub const OFFSET_SPIMCFG: c_uint = 0x0;
pub const OFFSET_SPIMCTRL: c_uint = 0x4;
pub const OFFSET_SPICMD: c_uint = 0x5;
pub const OFFSET_SPIDCNT: c_uint = 0x6;
pub const OFFSET_SPIADDR: c_uint = 0x8;
pub const OFFSET_SPIINTSTS: c_uint = 0xc;
pub const SPIMCTRL_START: c_uint = 0x01;
pub const SPIMCTRL_BUSY: c_uint = 0x02;
pub const SPIMCTRL_DIR: c_uint = 0x08;
    struct gxp_spi;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gxp_spi_chip {
    pub spifi: *mut gxp_spi,
    pub cs: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gxp_spi_data {
    pub max_cs: u32,
    pub mode_bits: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gxp_spi {
    pub data: *const gxp_spi_data,
    pub reg_base: *mut void __iomem,
    pub dat_base: *mut void __iomem,
    pub dir_base: *mut void __iomem,
    pub dev: *mut device,
    pub chips: [gxp_spi_chip; GXP_SPI0_MAX_CHIPSELECT],
}

#[no_mangle]
unsafe extern "C" fn gxp_spi_set_mode(spifi: *mut gxp_spi, mode: c_int) {
    static void gxp_spi_set_mode(struct gxp_spi *spifi, int mode)
    {
    u8 value;
    void __iomem *reg_base = spifi.reg_base;
    value = readb(reg_base + OFFSET_SPIMCTRL);
    if (mode == MANUAL_MODE) {
    writeb(0x55, reg_base + OFFSET_SPICMD);
    writeb(0xaa, reg_base + OFFSET_SPICMD);
    value &= ~0x30;
    } else {
    value |= 0x30;
    }
    writeb(value, reg_base + OFFSET_SPIMCTRL);
    }
#[no_mangle]
unsafe extern "C" fn gxp_spi_read_reg(chip: *mut gxp_spi_chip, op: *const spi_mem_op) -> c_int {
    static int gxp_spi_read_reg(struct gxp_spi_chip *chip, const struct spi_mem_op *op)
    {
    int ret;
    struct gxp_spi *spifi = chip.spifi;
    void __iomem *reg_base = spifi.reg_base;
    u32 value;
    value = readl(reg_base + OFFSET_SPIMCFG);
    value &= ~(1 << 24);
    value |= (chip.cs << 24);
    value &= ~(0x07 << 16);
    value &= ~(0x1f << 19);
    writel(value, reg_base + OFFSET_SPIMCFG);
    writel(0, reg_base + OFFSET_SPIADDR);
    writeb(op.cmd.opcode, reg_base + OFFSET_SPICMD);
    writew(op.data.nbytes, reg_base + OFFSET_SPIDCNT);
    value = readb(reg_base + OFFSET_SPIMCTRL);
    value &= ~SPIMCTRL_DIR;
    value |= SPIMCTRL_START;
    writeb(value, reg_base + OFFSET_SPIMCTRL);
    ret = readb_poll_timeout(reg_base + OFFSET_SPIMCTRL, value,
    !(value & SPIMCTRL_BUSY),
    GXP_SPI_SLEEP_TIME, GXP_SPI_TIMEOUT);
    if (ret) {
    dev_warn(spifi.dev, "read reg busy time out\n");
    return ret;
    }
    memcpy_fromio(op.data.buf.in, spifi.dat_base, op.data.nbytes);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn gxp_spi_write_reg(chip: *mut gxp_spi_chip, op: *const spi_mem_op) -> c_int {
    static int gxp_spi_write_reg(struct gxp_spi_chip *chip, const struct spi_mem_op *op)
    {
    int ret;
    struct gxp_spi *spifi = chip.spifi;
    void __iomem *reg_base = spifi.reg_base;
    u32 value;
    value = readl(reg_base + OFFSET_SPIMCFG);
    value &= ~(1 << 24);
    value |= (chip.cs << 24);
    value &= ~(0x07 << 16);
    value &= ~(0x1f << 19);
    writel(value, reg_base + OFFSET_SPIMCFG);
    writel(0, reg_base + OFFSET_SPIADDR);
    writeb(op.cmd.opcode, reg_base + OFFSET_SPICMD);
    memcpy_toio(spifi.dat_base, op.data.buf.in, op.data.nbytes);
    writew(op.data.nbytes, reg_base + OFFSET_SPIDCNT);
    value = readb(reg_base + OFFSET_SPIMCTRL);
    value |= SPIMCTRL_DIR;
    value |= SPIMCTRL_START;
    writeb(value, reg_base + OFFSET_SPIMCTRL);
    ret = readb_poll_timeout(reg_base + OFFSET_SPIMCTRL, value,
    !(value & SPIMCTRL_BUSY),
    GXP_SPI_SLEEP_TIME, GXP_SPI_TIMEOUT);
    if (ret)
    dev_warn(spifi.dev, "write reg busy time out\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn gxp_spi_read(chip: *mut gxp_spi_chip, op: *const spi_mem_op) -> isize {
    static ssize_t gxp_spi_read(struct gxp_spi_chip *chip, const struct spi_mem_op *op)
    {
    struct gxp_spi *spifi = chip.spifi;
    let mut offset: u32 = op.addr.val;
    if (chip.cs == 0)
    offset += 0x4000000;
    memcpy_fromio(op.data.buf.in, spifi.dir_base + offset, op.data.nbytes);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gxp_spi_write(chip: *mut gxp_spi_chip, op: *const spi_mem_op) -> isize {
    static ssize_t gxp_spi_write(struct gxp_spi_chip *chip, const struct spi_mem_op *op)
    {
    struct gxp_spi *spifi = chip.spifi;
    void __iomem *reg_base = spifi.reg_base;
    u32 write_len;
    u32 value;
    int ret;
    write_len = op.data.nbytes;
    if (write_len > SPILDAT_LEN)
    write_len = SPILDAT_LEN;
    value = readl(reg_base + OFFSET_SPIMCFG);
    value &= ~(1 << 24);
    value |= (chip.cs << 24);
    value &= ~(0x07 << 16);
    value |= (op.addr.nbytes << 16);
    value &= ~(0x1f << 19);
    writel(value, reg_base + OFFSET_SPIMCFG);
    writel(op.addr.val, reg_base + OFFSET_SPIADDR);
    writeb(op.cmd.opcode, reg_base + OFFSET_SPICMD);
    writew(write_len, reg_base + OFFSET_SPIDCNT);
    memcpy_toio(spifi.dat_base, op.data.buf.in, write_len);
    value = readb(reg_base + OFFSET_SPIMCTRL);
    value |= SPIMCTRL_DIR;
    value |= SPIMCTRL_START;
    writeb(value, reg_base + OFFSET_SPIMCTRL);
    ret = readb_poll_timeout(reg_base + OFFSET_SPIMCTRL, value,
    !(value & SPIMCTRL_BUSY),
    GXP_SPI_SLEEP_TIME, GXP_SPI_TIMEOUT);
    if (ret) {
    dev_warn(spifi.dev, "write busy time out\n");
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn do_gxp_exec_mem_op(mem: *mut spi_mem, op: *const spi_mem_op) -> c_int {
    static int do_gxp_exec_mem_op(struct spi_mem *mem, const struct spi_mem_op *op)
    {
    struct gxp_spi *spifi = spi_controller_get_devdata(mem.spi.controller);
    struct gxp_spi_chip *chip = &spifi.chips[spi_get_chipselect(mem.spi, 0)];
    int ret;
    if (op.data.dir == SPI_MEM_DATA_IN) {
    if (!op.addr.nbytes)
    ret = gxp_spi_read_reg(chip, op);
    else
    ret = gxp_spi_read(chip, op);
    } else {
    if (!op.addr.nbytes)
    ret = gxp_spi_write_reg(chip, op);
    else
    ret = gxp_spi_write(chip, op);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn gxp_exec_mem_op(mem: *mut spi_mem, op: *const spi_mem_op) -> c_int {
    static int gxp_exec_mem_op(struct spi_mem *mem, const struct spi_mem_op *op)
    {
    int ret;
    ret = do_gxp_exec_mem_op(mem, op);
    if (ret)
    dev_err(&mem.spi.dev, "operation failed: %d", ret);
    return ret;
    }
    static const struct spi_controller_mem_ops gxp_spi_mem_ops = {
    .exec_op = gxp_exec_mem_op,
    };
#[no_mangle]
unsafe extern "C" fn gxp_spi_setup(spi: *mut spi_device) -> c_int {
    static int gxp_spi_setup(struct spi_device *spi)
    {
    struct gxp_spi *spifi = spi_controller_get_devdata(spi.controller);
    let mut cs: c_uint = spi_get_chipselect(spi, 0);
    struct gxp_spi_chip *chip = &spifi.chips[cs];
    chip.spifi = spifi;
    chip.cs = cs;
    gxp_spi_set_mode(spifi, MANUAL_MODE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gxp_spifi_probe(pdev: *mut platform_device) -> c_int {
    static int gxp_spifi_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    const struct gxp_spi_data *data;
    struct spi_controller *ctlr;
    struct gxp_spi *spifi;
    int ret;
    data = of_device_get_match_data(&pdev.dev);
    ctlr = devm_spi_alloc_host(dev, sizeof(*spifi));
    if (!ctlr)
    return -ENOMEM;
    spifi = spi_controller_get_devdata(ctlr);
    platform_set_drvdata(pdev, spifi);
    spifi.data = data;
    spifi.dev = dev;
    spifi.reg_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(spifi.reg_base))
    return PTR_ERR(spifi.reg_base);
    spifi.dat_base = devm_platform_ioremap_resource(pdev, 1);
    if (IS_ERR(spifi.dat_base))
    return PTR_ERR(spifi.dat_base);
    spifi.dir_base = devm_platform_ioremap_resource(pdev, 2);
    if (IS_ERR(spifi.dir_base))
    return PTR_ERR(spifi.dir_base);
    ctlr.mode_bits = data.mode_bits;
    ctlr.bus_num = pdev.id;
    ctlr.mem_ops = &gxp_spi_mem_ops;
    ctlr.setup = gxp_spi_setup;
    ctlr.num_chipselect = data.max_cs;
    ret = devm_spi_register_controller(dev, ctlr);
    if (ret) {
    return dev_err_probe(&pdev.dev, ret,
    "failed to register spi controller\n");
    }
    return 0;
    }
    static const struct gxp_spi_data gxp_spifi_data = {
    .max_cs	= 2,
    .mode_bits = 0,
    };
    static const struct of_device_id gxp_spifi_match[] = {
    {.compatible = "hpe,gxp-spifi", .data = &gxp_spifi_data },
    { /* null */ }
    };
    MODULE_DEVICE_TABLE(of, gxp_spifi_match);
    static struct platform_driver gxp_spifi_driver = {
    .probe = gxp_spifi_probe,
    .driver = {
    .name = "gxp-spifi",
    .of_match_table = gxp_spifi_match,
    },
    };
    module_platform_driver(gxp_spifi_driver);
    MODULE_DESCRIPTION("HPE GXP SPI Flash Interface driver");
    MODULE_AUTHOR("Nick Hawkins <nick.hawkins@hpe.com>");
    MODULE_LICENSE("GPL");
