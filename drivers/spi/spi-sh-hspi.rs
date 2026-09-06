//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-sh-hspi.c
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
// SuperH HSPI bus driver
//
// Copyright (C) 2011  Kuninori Morimoto
//
// Based on spi-sh.c:
// Based on pxa2xx_spi.c:
// Copyright (C) 2011 Renesas Solutions Corp.
// Copyright (C) 2005 Stephen Street / StreetFire Sound Labs
//

pub const SPCR: c_uint = 0x00;
pub const SPSR: c_uint = 0x04;
pub const SPSCR: c_uint = 0x08;
pub const SPTBR: c_uint = 0x0C;
pub const SPRBR: c_uint = 0x10;
pub const SPCR2: c_uint = 0x14;
// SPSR

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hspi_priv {
    pub addr: *mut void __iomem,
    pub ctlr: *mut spi_controller,
    pub dev: *mut device,
    pub clk: *mut clk,
}

//
// basic function
//
#[no_mangle]
unsafe extern "C" fn hspi_write(hspi: *mut hspi_priv, reg: c_int, val: u32) {
    static void hspi_write(struct hspi_priv *hspi, int reg, u32 val)
    {
    iowrite32(val, hspi.addr + reg);
    }
#[no_mangle]
unsafe extern "C" fn hspi_read(hspi: *mut hspi_priv, reg: c_int) -> u32 {
    static u32 hspi_read(struct hspi_priv *hspi, int reg)
    {
    return ioread32(hspi.addr + reg);
    }
#[no_mangle]
unsafe extern "C" fn hspi_bit_set(hspi: *mut hspi_priv, reg: c_int, mask: u32, set: u32) {
    static void hspi_bit_set(struct hspi_priv *hspi, int reg, u32 mask, u32 set)
    {
    let mut val: u32 = hspi_read(hspi, reg);
    val &= ~mask;
    val |= set & mask;
    hspi_write(hspi, reg, val);
    }
//
// transfer function
//
#[no_mangle]
unsafe extern "C" fn hspi_status_check_timeout(hspi: *mut hspi_priv, mask: u32, val: u32) -> c_int {
    static int hspi_status_check_timeout(struct hspi_priv *hspi, u32 mask, u32 val)
    {
    let mut t: c_int = 256;
    while (t--) {
    if ((mask & hspi_read(hspi, SPSR)) == val)
    return 0;
    udelay(10);
    }
    dev_err(hspi.dev, "timeout\n");
    return -ETIMEDOUT;
    }
//
// spi host function
//

#[no_mangle]
unsafe extern "C" fn hspi_hw_cs_ctrl(hspi: *mut hspi_priv, hi: c_int) {
    static void hspi_hw_cs_ctrl(struct hspi_priv *hspi, int hi)
    {
    hspi_bit_set(hspi, SPSCR, (1 << 6), (hi) << 6);
    }
    static void hspi_hw_setup(struct hspi_priv *hspi,
    struct spi_message *msg,
    struct spi_transfer *t)
    {
    struct spi_device *spi = msg.spi;
    struct device *dev = hspi.dev;
    u32 spcr, idiv_clk;
    u32 rate, best_rate, min, tmp;
//
// find best IDIV/CLKCx settings
//
    min = ~0;
    best_rate = 0;
    spcr = 0;
    for (idiv_clk = 0x00; idiv_clk <= 0x3F; idiv_clk++) {
    rate = clk_get_rate(hspi.clk);
// IDIV calculation
    if (idiv_clk & (1 << 5))
    rate /= 128;
    else
    rate /= 16;
// CLKCx calculation
    rate /= (((idiv_clk & 0x1F) + 1) * 2);
// save best settings
    tmp = abs(t.speed_hz - rate);
    if (tmp < min) {
    min = tmp;
    spcr = idiv_clk;
    best_rate = rate;
    }
    }
    if (spi.mode & SPI_CPHA)
    spcr |= 1 << 7;
    if (spi.mode & SPI_CPOL)
    spcr |= 1 << 6;
    dev_dbg(dev, "speed %d/%d\n", t.speed_hz, best_rate);
    hspi_write(hspi, SPCR, spcr);
    hspi_write(hspi, SPSR, 0x0);
    hspi_write(hspi, SPSCR, 0x21);	/* master mode / CS control */
    }
    static int hspi_transfer_one_message(struct spi_controller *ctlr,
    struct spi_message *msg)
    {
    struct hspi_priv *hspi = spi_controller_get_devdata(ctlr);
    struct spi_transfer *t;
    u32 tx;
    u32 rx;
    int ret, i;
    unsigned int cs_change;
    let mut nsecs: c_int = 50;
    dev_dbg(hspi.dev, "%s\n", __func__);
    cs_change = 1;
    ret = 0;
    list_for_each_entry(t, &msg.transfers, transfer_list) {
    if (cs_change) {
    hspi_hw_setup(hspi, msg, t);
    hspi_hw_cs_enable(hspi);
    ndelay(nsecs);
    }
    cs_change = t.cs_change;
    for (i = 0; i < t.len; i++) {
// wait remains
    ret = hspi_status_check_timeout(hspi, 0x1, 0);
    if (ret < 0)
    break;
    tx = 0;
    if (t.tx_buf)
    tx = (u32)((u8 *)t.tx_buf)[i];
    hspi_write(hspi, SPTBR, tx);
// wait receive
    ret = hspi_status_check_timeout(hspi, 0x4, 0x4);
    if (ret < 0)
    break;
    rx = hspi_read(hspi, SPRBR);
    if (t.rx_buf)
    ((u8 *)t.rx_buf)[i] = (u8)rx;
    }
    msg.actual_length += t.len;
    spi_transfer_delay_exec(t);
    if (cs_change) {
    ndelay(nsecs);
    hspi_hw_cs_disable(hspi);
    ndelay(nsecs);
    }
    }
    msg.status = ret;
    if (!cs_change) {
    ndelay(nsecs);
    hspi_hw_cs_disable(hspi);
    }
    spi_finalize_current_message(ctlr);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hspi_probe(pdev: *mut platform_device) -> c_int {
    static int hspi_probe(struct platform_device *pdev)
    {
    struct resource *res;
    struct spi_controller *ctlr;
    struct hspi_priv *hspi;
    struct clk *clk;
    int ret;
// get base addr
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res) {
    dev_err(&pdev.dev, "invalid resource\n");
    return -EINVAL;
    }
    ctlr = devm_spi_alloc_host(&pdev.dev, sizeof(*hspi));
    if (!ctlr)
    return -ENOMEM;
    clk = clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(clk)) {
    dev_err(&pdev.dev, "couldn't get clock\n");
    return PTR_ERR(clk);
    }
    hspi = spi_controller_get_devdata(ctlr);
    platform_set_drvdata(pdev, hspi);
// init hspi
    hspi.ctlr	= ctlr;
    hspi.dev	= &pdev.dev;
    hspi.clk	= clk;
    hspi.addr	= devm_ioremap(hspi.dev,
    res.start, resource_size(res));
    if (!hspi.addr) {
    ret = -ENOMEM;
    goto error1;
    }
    pm_runtime_enable(&pdev.dev);
    ctlr.bus_num = pdev.id;
    ctlr.mode_bits	= SPI_CPOL | SPI_CPHA;
    ctlr.auto_runtime_pm = true;
    ctlr.transfer_one_message = hspi_transfer_one_message;
    ctlr.bits_per_word_mask = SPI_BPW_MASK(8);
    ret = spi_register_controller(ctlr);
    if (ret < 0) {
    dev_err(&pdev.dev, "failed to register controller\n");
    goto error2;
    }
    return 0;
    error2:
    pm_runtime_disable(&pdev.dev);
    error1:
    clk_put(clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hspi_remove(pdev: *mut platform_device) {
    static void hspi_remove(struct platform_device *pdev)
    {
    struct hspi_priv *hspi = platform_get_drvdata(pdev);
    spi_unregister_controller(hspi.ctlr);
    pm_runtime_disable(&pdev.dev);
    clk_put(hspi.clk);
    }
    static const struct of_device_id hspi_of_match[] = {
    { .compatible = "renesas,hspi", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, hspi_of_match);
    static struct platform_driver hspi_driver = {
    .probe = hspi_probe,
    .remove = hspi_remove,
    .driver = {
    .name = "sh-hspi",
    .of_match_table = hspi_of_match,
    },
    };
    module_platform_driver(hspi_driver);
    MODULE_DESCRIPTION("SuperH HSPI bus driver");
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>");
    MODULE_ALIAS("platform:sh-hspi");
