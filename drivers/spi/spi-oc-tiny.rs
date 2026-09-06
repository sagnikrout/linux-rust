//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-oc-tiny.c
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
// OpenCores tiny SPI host driver
//
// https://opencores.org/project,tiny_spi
//
// Copyright (C) 2011 Thomas Chou <thomas@wytron.com.tw>
//
// Based on spi_s3c24xx.c, which is:
// Copyright (c) 2006 Ben Dooks
// Copyright (c) 2006 Simtec Electronics
// Ben Dooks <ben@simtec.co.uk>
//

pub const TINY_SPI_RXDATA: c_int = 0;
pub const TINY_SPI_TXDATA: c_int = 4;
pub const TINY_SPI_STATUS: c_int = 8;
pub const TINY_SPI_CONTROL: c_int = 12;
pub const TINY_SPI_BAUD: c_int = 16;
pub const TINY_SPI_STATUS_TXE: c_uint = 0x1;
pub const TINY_SPI_STATUS_TXR: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tiny_spi {
// bitbang has to be first
    pub bitbang: spi_bitbang,
    pub done: completion,
    pub base: *mut void __iomem,
    pub irq: c_int,
    pub freq: c_uint,
    pub baudwidth: c_uint,
    pub baud: c_uint,
    pub speed_hz: c_uint,
    pub mode: c_uint,
    pub len: c_uint,
    pub rxc: unsigned int txc,,
    pub txp: *const u8,
    pub rxp: *mut u8,
}

    static inline struct tiny_spi *tiny_spi_to_hw(struct spi_device *sdev)
    {
    return spi_controller_get_devdata(sdev.controller);
    }
#[no_mangle]
unsafe extern "C" fn tiny_spi_baud(spi: *mut spi_device, hz: c_uint) -> c_uint {
    static unsigned int tiny_spi_baud(struct spi_device *spi, unsigned int hz)
    {
    struct tiny_spi *hw = tiny_spi_to_hw(spi);
    return min(DIV_ROUND_UP(hw.freq, hz * 2), (1U << hw.baudwidth)) - 1;
    }
    static int tiny_spi_setup_transfer(struct spi_device *spi,
    struct spi_transfer *t)
    {
    struct tiny_spi *hw = tiny_spi_to_hw(spi);
    let mut baud: c_uint = hw.baud;
    if (t) {
    if (t.speed_hz && t.speed_hz != hw.speed_hz)
    baud = tiny_spi_baud(spi, t.speed_hz);
    }
    writel(baud, hw.base + TINY_SPI_BAUD);
    writel(hw.mode, hw.base + TINY_SPI_CONTROL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tiny_spi_setup(spi: *mut spi_device) -> c_int {
    static int tiny_spi_setup(struct spi_device *spi)
    {
    struct tiny_spi *hw = tiny_spi_to_hw(spi);
    if (spi.max_speed_hz != hw.speed_hz) {
    hw.speed_hz = spi.max_speed_hz;
    hw.baud = tiny_spi_baud(spi, hw.speed_hz);
    }
    hw.mode = spi.mode & SPI_MODE_X_MASK;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn tiny_spi_wait_txr(hw: *mut tiny_spi) {
    static inline void tiny_spi_wait_txr(struct tiny_spi *hw)
    {
    while (!(readb(hw.base + TINY_SPI_STATUS) &
    TINY_SPI_STATUS_TXR))
    cpu_relax();
    }
#[no_mangle]
pub unsafe extern "C" fn tiny_spi_wait_txe(hw: *mut tiny_spi) {
    static inline void tiny_spi_wait_txe(struct tiny_spi *hw)
    {
    while (!(readb(hw.base + TINY_SPI_STATUS) &
    TINY_SPI_STATUS_TXE))
    cpu_relax();
    }
#[no_mangle]
unsafe extern "C" fn tiny_spi_txrx_bufs(spi: *mut spi_device, t: *mut spi_transfer) -> c_int {
    static int tiny_spi_txrx_bufs(struct spi_device *spi, struct spi_transfer *t)
    {
    struct tiny_spi *hw = tiny_spi_to_hw(spi);
    const u8 *txp = t.tx_buf;
    u8 *rxp = t.rx_buf;
    unsigned int i;
    if (hw.irq >= 0) {
// use interrupt driven data transfer
    hw.len = t.len;
    hw.txp = t.tx_buf;
    hw.rxp = t.rx_buf;
    hw.txc = 0;
    hw.rxc = 0;
// send the first byte
    if (t.len > 1) {
    writeb(hw.txp ? *hw.txp++ : 0,
    hw.base + TINY_SPI_TXDATA);
    hw.txc++;
    writeb(hw.txp ? *hw.txp++ : 0,
    hw.base + TINY_SPI_TXDATA);
    hw.txc++;
    writeb(TINY_SPI_STATUS_TXR, hw.base + TINY_SPI_STATUS);
    } else {
    writeb(hw.txp ? *hw.txp++ : 0,
    hw.base + TINY_SPI_TXDATA);
    hw.txc++;
    writeb(TINY_SPI_STATUS_TXE, hw.base + TINY_SPI_STATUS);
    }
    wait_for_completion(&hw.done);
    } else {
// we need to tighten the transfer loop
    writeb(txp ? *txp++ : 0, hw.base + TINY_SPI_TXDATA);
    for (i = 1; i < t.len; i++) {
    writeb(txp ? *txp++ : 0, hw.base + TINY_SPI_TXDATA);
    if (rxp || (i != t.len - 1))
    tiny_spi_wait_txr(hw);
    if (rxp)
// rxp++ = readb(hw->base + TINY_SPI_TXDATA);
    }
    tiny_spi_wait_txe(hw);
    if (rxp)
// rxp++ = readb(hw->base + TINY_SPI_RXDATA);
    }
    return t.len;
    }
#[no_mangle]
unsafe extern "C" fn tiny_spi_irq(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t tiny_spi_irq(int irq, void *dev)
    {
    struct tiny_spi *hw = dev;
    writeb(0, hw.base + TINY_SPI_STATUS);
    if (hw.rxc + 1 == hw.len) {
    if (hw.rxp)
// hw->rxp++ = readb(hw->base + TINY_SPI_RXDATA);
    hw.rxc++;
    complete(&hw.done);
    } else {
    if (hw.rxp)
// hw->rxp++ = readb(hw->base + TINY_SPI_TXDATA);
    hw.rxc++;
    if (hw.txc < hw.len) {
    writeb(hw.txp ? *hw.txp++ : 0,
    hw.base + TINY_SPI_TXDATA);
    hw.txc++;
    writeb(TINY_SPI_STATUS_TXR,
    hw.base + TINY_SPI_STATUS);
    } else {
    writeb(TINY_SPI_STATUS_TXE,
    hw.base + TINY_SPI_STATUS);
    }
    }
    return IRQ_HANDLED;
    }

#[no_mangle]
unsafe extern "C" fn tiny_spi_of_probe(pdev: *mut platform_device) -> c_int {
    static int tiny_spi_of_probe(struct platform_device *pdev)
    {
    struct tiny_spi *hw = platform_get_drvdata(pdev);
    struct device_node *np = pdev.dev.of_node;
    u32 val;
    if (!np)
    return 0;
    if (!of_property_read_u32(np, "clock-frequency", &val))
    hw.freq = val;
    if (!of_property_read_u32(np, "baud-width", &val))
    hw.baudwidth = val;
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn tiny_spi_of_probe(pdev: *mut platform_device) -> c_int {
    static int tiny_spi_of_probe(struct platform_device *pdev)
    {
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn tiny_spi_probe(pdev: *mut platform_device) -> c_int {
    static int tiny_spi_probe(struct platform_device *pdev)
    {
    struct tiny_spi_platform_data *platp = dev_get_platdata(&pdev.dev);
    struct tiny_spi *hw;
    struct spi_controller *host;
    int err;
    host = devm_spi_alloc_host(&pdev.dev, sizeof(struct tiny_spi));
    if (!host)
    return -ENOMEM;
// setup the host state.
    host.bus_num = pdev.id;
    host.mode_bits = SPI_CPOL | SPI_CPHA | SPI_CS_HIGH;
    host.setup = tiny_spi_setup;
    host.use_gpio_descriptors = true;
    hw = spi_controller_get_devdata(host);
    platform_set_drvdata(pdev, hw);
// setup the state for the bitbang driver
    hw.bitbang.ctlr = host;
    hw.bitbang.setup_transfer = tiny_spi_setup_transfer;
    hw.bitbang.txrx_bufs = tiny_spi_txrx_bufs;
// find and map our resources
    hw.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(hw.base))
    return PTR_ERR(hw.base);
// irq is optional
    hw.irq = platform_get_irq(pdev, 0);
    if (hw.irq >= 0) {
    init_completion(&hw.done);
    err = devm_request_irq(&pdev.dev, hw.irq, tiny_spi_irq, 0,
    pdev.name, hw);
    if (err)
    return err;
    }
// find platform data
    if (platp) {
    hw.freq = platp.freq;
    hw.baudwidth = platp.baudwidth;
    } else {
    err = tiny_spi_of_probe(pdev);
    if (err)
    return err;
    }
// register our spi controller
    err = spi_bitbang_start(&hw.bitbang);
    if (err)
    return err;
    dev_info(&pdev.dev, "base %p, irq %d\n", hw.base, hw.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tiny_spi_remove(pdev: *mut platform_device) {
    static void tiny_spi_remove(struct platform_device *pdev)
    {
    struct tiny_spi *hw = platform_get_drvdata(pdev);
    spi_bitbang_stop(&hw.bitbang);
    }

    static const struct of_device_id tiny_spi_match[] = {
    { .compatible = "opencores,tiny-spi-rtlsvn2", },
    {},
    };
    MODULE_DEVICE_TABLE(of, tiny_spi_match);

    static struct platform_driver tiny_spi_driver = {
    .probe = tiny_spi_probe,
    .remove = tiny_spi_remove,
    .driver = {
    .name = DRV_NAME,
    .pm = core::ptr::null_mut(),
    .of_match_table = of_match_ptr(tiny_spi_match),
    },
    };
    module_platform_driver(tiny_spi_driver);
    MODULE_DESCRIPTION("OpenCores tiny SPI driver");
    MODULE_AUTHOR("Thomas Chou <thomas@wytron.com.tw>");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:" DRV_NAME);
