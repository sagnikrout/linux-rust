//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-altera-core.c
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
// Altera SPI driver
//
// Copyright (C) 2008 Thomas Chou <thomas@wytron.com.tw>
//
// Based on spi_s3c24xx.c, which is:
// Copyright (c) 2006 Ben Dooks
// Copyright (c) 2006 Simtec Electronics
// Ben Dooks <ben@simtec.co.uk>
//

pub const ALTERA_SPI_RXDATA: c_int = 0;
pub const ALTERA_SPI_TXDATA: c_int = 4;
pub const ALTERA_SPI_STATUS: c_int = 8;
pub const ALTERA_SPI_CONTROL: c_int = 12;
pub const ALTERA_SPI_TARGET_SEL: c_int = 20;
pub const ALTERA_SPI_STATUS_ROE_MSK: c_uint = 0x8;
pub const ALTERA_SPI_STATUS_TOE_MSK: c_uint = 0x10;
pub const ALTERA_SPI_STATUS_TMT_MSK: c_uint = 0x20;
pub const ALTERA_SPI_STATUS_TRDY_MSK: c_uint = 0x40;
pub const ALTERA_SPI_STATUS_RRDY_MSK: c_uint = 0x80;
pub const ALTERA_SPI_STATUS_E_MSK: c_uint = 0x100;
pub const ALTERA_SPI_CONTROL_IROE_MSK: c_uint = 0x8;
pub const ALTERA_SPI_CONTROL_ITOE_MSK: c_uint = 0x10;
pub const ALTERA_SPI_CONTROL_ITRDY_MSK: c_uint = 0x40;
pub const ALTERA_SPI_CONTROL_IRRDY_MSK: c_uint = 0x80;
pub const ALTERA_SPI_CONTROL_IE_MSK: c_uint = 0x100;
pub const ALTERA_SPI_CONTROL_SSO_MSK: c_uint = 0x400;
    static int altr_spi_writel(struct altera_spi *hw, unsigned int reg,
    unsigned int val)
    {
    int ret;
    ret = regmap_write(hw.regmap, hw.regoff + reg, val);
    if (ret)
    dev_err(hw.dev, "fail to write reg 0x%x val 0x%x: %d\n",
    reg, val, ret);
    return ret;
    }
    static int altr_spi_readl(struct altera_spi *hw, unsigned int reg,
    unsigned int *val)
    {
    int ret;
    ret = regmap_read(hw.regmap, hw.regoff + reg, val);
    if (ret)
    dev_err(hw.dev, "fail to read reg 0x%x: %d\n", reg, ret);
    return ret;
    }
    static inline struct altera_spi *altera_spi_to_hw(struct spi_device *sdev)
    {
    return spi_controller_get_devdata(sdev.controller);
    }
#[no_mangle]
unsafe extern "C" fn altera_spi_set_cs(spi: *mut spi_device, is_high: bool) {
    static void altera_spi_set_cs(struct spi_device *spi, bool is_high)
    {
    struct altera_spi *hw = altera_spi_to_hw(spi);
    if (is_high) {
    hw.imr &= ~ALTERA_SPI_CONTROL_SSO_MSK;
    altr_spi_writel(hw, ALTERA_SPI_CONTROL, hw.imr);
    altr_spi_writel(hw, ALTERA_SPI_TARGET_SEL, 0);
    } else {
    altr_spi_writel(hw, ALTERA_SPI_TARGET_SEL,
    BIT(spi_get_chipselect(spi, 0)));
    hw.imr |= ALTERA_SPI_CONTROL_SSO_MSK;
    altr_spi_writel(hw, ALTERA_SPI_CONTROL, hw.imr);
    }
    }
#[no_mangle]
unsafe extern "C" fn altera_spi_tx_word(hw: *mut altera_spi) {
    static void altera_spi_tx_word(struct altera_spi *hw)
    {
    let mut txd: c_uint = 0;
    if (hw.tx) {
    switch (hw.bytes_per_word) {
    case 1:
    txd = hw.tx[hw.count];
    break;
    case 2:
    txd = (hw.tx[hw.count * 2]
    | (hw.tx[hw.count * 2 + 1] << 8));
    break;
    case 4:
    txd = (hw.tx[hw.count * 4]
    | (hw.tx[hw.count * 4 + 1] << 8)
    | (hw.tx[hw.count * 4 + 2] << 16)
    | (hw.tx[hw.count * 4 + 3] << 24));
    break;
    }
    }
    altr_spi_writel(hw, ALTERA_SPI_TXDATA, txd);
    }
#[no_mangle]
unsafe extern "C" fn altera_spi_rx_word(hw: *mut altera_spi) {
    static void altera_spi_rx_word(struct altera_spi *hw)
    {
    unsigned int rxd;
    altr_spi_readl(hw, ALTERA_SPI_RXDATA, &rxd);
    if (hw.rx) {
    switch (hw.bytes_per_word) {
    case 1:
    hw.rx[hw.count] = rxd;
    break;
    case 2:
    hw.rx[hw.count * 2] = rxd;
    hw.rx[hw.count * 2 + 1] = rxd >> 8;
    break;
    case 4:
    hw.rx[hw.count * 4] = rxd;
    hw.rx[hw.count * 4 + 1] = rxd >> 8;
    hw.rx[hw.count * 4 + 2] = rxd >> 16;
    hw.rx[hw.count * 4 + 3] = rxd >> 24;
    break;
    }
    }
    hw.count++;
    }
    static int altera_spi_txrx(struct spi_controller *host,
    struct spi_device *spi, struct spi_transfer *t)
    {
    struct altera_spi *hw = spi_controller_get_devdata(host);
    u32 val;
    hw.tx = t.tx_buf;
    hw.rx = t.rx_buf;
    hw.count = 0;
    hw.bytes_per_word = DIV_ROUND_UP(t.bits_per_word, 8);
    hw.len = t.len / hw.bytes_per_word;
    if (hw.irq >= 0) {
// enable receive interrupt
    hw.imr |= ALTERA_SPI_CONTROL_IRRDY_MSK;
    altr_spi_writel(hw, ALTERA_SPI_CONTROL, hw.imr);
// send the first byte
    altera_spi_tx_word(hw);
    return 1;
    }
    while (hw.count < hw.len) {
    altera_spi_tx_word(hw);
    for (;;) {
    altr_spi_readl(hw, ALTERA_SPI_STATUS, &val);
    if (val & ALTERA_SPI_STATUS_RRDY_MSK)
    break;
    cpu_relax();
    }
    altera_spi_rx_word(hw);
    }
    spi_finalize_current_transfer(host);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn altera_spi_irq(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    irqreturn_t altera_spi_irq(int irq, void *dev)
    {
    struct spi_controller *host = dev;
    struct altera_spi *hw = spi_controller_get_devdata(host);
    altera_spi_rx_word(hw);
    if (hw.count < hw.len) {
    altera_spi_tx_word(hw);
    } else {
// disable receive interrupt
    hw.imr &= ~ALTERA_SPI_CONTROL_IRRDY_MSK;
    altr_spi_writel(hw, ALTERA_SPI_CONTROL, hw.imr);
    spi_finalize_current_transfer(host);
    }
    return IRQ_HANDLED;
    }
    EXPORT_SYMBOL_GPL(altera_spi_irq);
#[no_mangle]
pub unsafe extern "C" fn altera_spi_init_host(host: *mut spi_controller) {
    void altera_spi_init_host(struct spi_controller *host)
    {
    struct altera_spi *hw = spi_controller_get_devdata(host);
    u32 val;
    host.transfer_one = altera_spi_txrx;
    host.set_cs = altera_spi_set_cs;
// program defaults into the registers
    hw.imr = 0;		/* disable spi interrupts */
    altr_spi_writel(hw, ALTERA_SPI_CONTROL, hw.imr);
    altr_spi_writel(hw, ALTERA_SPI_STATUS, 0);	/* clear status reg */
    altr_spi_readl(hw, ALTERA_SPI_STATUS, &val);
    if (val & ALTERA_SPI_STATUS_RRDY_MSK)
    altr_spi_readl(hw, ALTERA_SPI_RXDATA, &val); /* flush rxdata */
    }
    EXPORT_SYMBOL_GPL(altera_spi_init_host);
    MODULE_DESCRIPTION("Altera SPI Controller driver core");
    MODULE_LICENSE("GPL");
