//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/mmp/hw/mmp_spi.c
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
//
// linux/drivers/video/mmp/hw/mmp_spi.c
// using the spi in LCD controler for commands send
//
// Copyright (C) 2012 Marvell Technology Group Ltd.
// Authors:  Guoqing Li <ligq@marvell.com>
// Lisa Du <cldu@marvell.com>
// Zhou Zhu <zzhu3@marvell.com>
//

//
// spi_write - write command to the SPI port
// @spi:  the SPI device.
// @data: can be 8/16/32-bit, MSB justified data to write.
//
// Wait bus transfer complete IRQ.
// The caller is expected to perform the necessary locking.
//
// Returns:
// %-ETIMEDOUT	timeout occurred
// 0			success
//
#[no_mangle]
pub unsafe extern "C" fn lcd_spi_write(spi: *mut spi_device, data: u32) -> c_int {
    static inline int lcd_spi_write(struct spi_device *spi, u32 data)
    {
    let mut timeout: c_int = 100000, isr, ret = 0;
    u32 tmp;
    void __iomem *reg_base = (void __iomem *)
// (void **) spi_controller_get_devdata(spi->controller);
// clear ISR
    writel_relaxed(~SPI_IRQ_MASK, reg_base + SPU_IRQ_ISR);
    switch (spi.bits_per_word) {
    case 8:
    writel_relaxed((u8)data, reg_base + LCD_SPU_SPI_TXDATA);
    break;
    case 16:
    writel_relaxed((u16)data, reg_base + LCD_SPU_SPI_TXDATA);
    break;
    case 32:
    writel_relaxed((u32)data, reg_base + LCD_SPU_SPI_TXDATA);
    break;
    default:
    dev_err(&spi.dev, "Wrong spi bit length\n");
    }
// SPI start to send command
    tmp = readl_relaxed(reg_base + LCD_SPU_SPI_CTRL);
    tmp &= ~CFG_SPI_START_MASK;
    tmp |= CFG_SPI_START(1);
    writel(tmp, reg_base + LCD_SPU_SPI_CTRL);
    isr = readl_relaxed(reg_base + SPU_IRQ_ISR);
    while (!(isr & SPI_IRQ_ENA_MASK)) {
    udelay(100);
    isr = readl_relaxed(reg_base + SPU_IRQ_ISR);
    if (!--timeout) {
    ret = -ETIMEDOUT;
    dev_err(&spi.dev, "spi cmd send time out\n");
    break;
    }
    }
    tmp = readl_relaxed(reg_base + LCD_SPU_SPI_CTRL);
    tmp &= ~CFG_SPI_START_MASK;
    tmp |= CFG_SPI_START(0);
    writel_relaxed(tmp, reg_base + LCD_SPU_SPI_CTRL);
    writel_relaxed(~SPI_IRQ_MASK, reg_base + SPU_IRQ_ISR);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn lcd_spi_setup(spi: *mut spi_device) -> c_int {
    static int lcd_spi_setup(struct spi_device *spi)
    {
    void __iomem *reg_base = (void __iomem *)
// (void **) spi_controller_get_devdata(spi->controller);
    u32 tmp;
    tmp = CFG_SCLKCNT(16) |
    CFG_TXBITS(spi.bits_per_word) |
    CFG_SPI_SEL(1) | CFG_SPI_ENA(1) |
    CFG_SPI_3W4WB(1);
    writel(tmp, reg_base + LCD_SPU_SPI_CTRL);
//
// After set mode it needs some time to pull up the spi signals,
// or it would cause the wrong waveform when send spi command,
// especially on pxa910h
//
    tmp = readl_relaxed(reg_base + SPU_IOPAD_CONTROL);
    if ((tmp & CFG_IOPADMODE_MASK) != IOPAD_DUMB18SPI)
    writel_relaxed(IOPAD_DUMB18SPI |
    (tmp & ~CFG_IOPADMODE_MASK),
    reg_base + SPU_IOPAD_CONTROL);
    udelay(20);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lcd_spi_one_transfer(spi: *mut spi_device, m: *mut spi_message) -> c_int {
    static int lcd_spi_one_transfer(struct spi_device *spi, struct spi_message *m)
    {
    struct spi_transfer *t;
    int i;
    list_for_each_entry(t, &m.transfers, transfer_list) {
    switch (spi.bits_per_word) {
    case 8:
    for (i = 0; i < t.len; i++)
    lcd_spi_write(spi, ((u8 *)t.tx_buf)[i]);
    break;
    case 16:
    for (i = 0; i < t.len/2; i++)
    lcd_spi_write(spi, ((u16 *)t.tx_buf)[i]);
    break;
    case 32:
    for (i = 0; i < t.len/4; i++)
    lcd_spi_write(spi, ((u32 *)t.tx_buf)[i]);
    break;
    default:
    dev_err(&spi.dev, "Wrong spi bit length\n");
    }
    }
    m.status = 0;
    if (m.complete)
    m.complete(m.context);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn lcd_spi_register(ctrl: *mut mmphw_ctrl) -> c_int {
    int lcd_spi_register(struct mmphw_ctrl *ctrl)
    {
    struct spi_controller *ctlr;
    void **p_regbase;
    int err;
    ctlr = spi_alloc_host(ctrl.dev, sizeof(void *));
    if (!ctlr) {
    dev_err(ctrl.dev, "unable to allocate SPI host\n");
    return -ENOMEM;
    }
    p_regbase = spi_controller_get_devdata(ctlr);
// p_regbase = (void  *)ctrl->reg_base;
// set bus num to 5 to avoid conflict with other spi hosts
    ctlr.bus_num = 5;
    ctlr.num_chipselect = 1;
    ctlr.setup = lcd_spi_setup;
    ctlr.transfer = lcd_spi_one_transfer;
    err = spi_register_controller(ctlr);
    if (err < 0) {
    dev_err(ctrl.dev, "unable to register SPI host\n");
    spi_controller_put(ctlr);
    return err;
    }
    dev_info(&ctlr.dev, "registered\n");
    return 0;
    }
