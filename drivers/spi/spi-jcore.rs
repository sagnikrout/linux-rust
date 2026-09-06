//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-jcore.c
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
// J-Core SPI controller driver
//
// Copyright (C) 2012-2016 Smart Energy Instruments, Inc.
//
// Current version by Rich Felker
// Based loosely on initial version by Oleksandr G Zhadan
//

pub const CTRL_REG: c_uint = 0x0;
pub const DATA_REG: c_uint = 0x4;
pub const JCORE_SPI_CTRL_XMIT: c_uint = 0x02;
pub const JCORE_SPI_STAT_BUSY: c_uint = 0x02;
pub const JCORE_SPI_CTRL_LOOP: c_uint = 0x08;
pub const JCORE_SPI_CTRL_CS_BITS: c_uint = 0x15;
pub const JCORE_SPI_WAIT_RDY_MAX_LOOP: c_int = 2000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jcore_spi {
    pub host: *mut spi_controller,
    pub base: *mut void __iomem,
    pub cs_reg: c_uint,
    pub speed_reg: c_uint,
    pub speed_hz: c_uint,
    pub clock_freq: c_uint,
}

#[no_mangle]
unsafe extern "C" fn jcore_spi_wait(ctrl_reg: *mut void __iomem) -> c_int {
    static int jcore_spi_wait(void __iomem *ctrl_reg)
    {
    let mut timeout: unsigned = JCORE_SPI_WAIT_RDY_MAX_LOOP;
    do {
    if (!(readl(ctrl_reg) & JCORE_SPI_STAT_BUSY))
    return 0;
    cpu_relax();
    } while (--timeout);
    return -EBUSY;
    }
#[no_mangle]
unsafe extern "C" fn jcore_spi_program(hw: *mut jcore_spi) {
    static void jcore_spi_program(struct jcore_spi *hw)
    {
    void __iomem *ctrl_reg = hw.base + CTRL_REG;
    if (jcore_spi_wait(ctrl_reg))
    dev_err(hw.host.dev.parent,
    "timeout waiting to program ctrl reg.\n");
    writel(hw.cs_reg | hw.speed_reg, ctrl_reg);
    }
#[no_mangle]
unsafe extern "C" fn jcore_spi_chipsel(spi: *mut spi_device, value: bool) {
    static void jcore_spi_chipsel(struct spi_device *spi, bool value)
    {
    struct jcore_spi *hw = spi_controller_get_devdata(spi.controller);
    let mut csbit: u32 = 1U << (2 * spi_get_chipselect(spi, 0));
    dev_dbg(hw.host.dev.parent, "chipselect %d\n", spi_get_chipselect(spi, 0));
    if (value)
    hw.cs_reg |= csbit;
    else
    hw.cs_reg &= ~csbit;
    jcore_spi_program(hw);
    }
#[no_mangle]
unsafe extern "C" fn jcore_spi_baudrate(hw: *mut jcore_spi, speed: c_int) {
    static void jcore_spi_baudrate(struct jcore_spi *hw, int speed)
    {
    if (speed == hw.speed_hz)
    return;
    hw.speed_hz = speed;
    if (speed >= hw.clock_freq / 2)
    hw.speed_reg = 0;
    else
    hw.speed_reg = ((hw.clock_freq / 2 / speed) - 1) << 27;
    jcore_spi_program(hw);
    dev_dbg(hw.host.dev.parent, "speed=%d reg=0x%x\n",
    speed, hw.speed_reg);
    }
    static int jcore_spi_txrx(struct spi_controller *host, struct spi_device *spi,
    struct spi_transfer *t)
    {
    struct jcore_spi *hw = spi_controller_get_devdata(host);
    void __iomem *ctrl_reg = hw.base + CTRL_REG;
    void __iomem *data_reg = hw.base + DATA_REG;
    u32 xmit;
// data buffers
    const unsigned char *tx;
    unsigned char *rx;
    unsigned int len;
    unsigned int count;
    jcore_spi_baudrate(hw, t.speed_hz);
    xmit = hw.cs_reg | hw.speed_reg | JCORE_SPI_CTRL_XMIT;
    tx = t.tx_buf;
    rx = t.rx_buf;
    len = t.len;
    for (count = 0; count < len; count++) {
    if (jcore_spi_wait(ctrl_reg))
    break;
    writel(tx ? *tx++ : 0, data_reg);
    writel(xmit, ctrl_reg);
    if (jcore_spi_wait(ctrl_reg))
    break;
    if (rx)
// rx++ = readl(data_reg);
    }
    spi_finalize_current_transfer(host);
    if (count < len)
    return -EREMOTEIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jcore_spi_probe(pdev: *mut platform_device) -> c_int {
    static int jcore_spi_probe(struct platform_device *pdev)
    {
    struct device_node *node = pdev.dev.of_node;
    struct jcore_spi *hw;
    struct spi_controller *host;
    struct resource *res;
    u32 clock_freq;
    struct clk *clk;
    host = devm_spi_alloc_host(&pdev.dev, sizeof(struct jcore_spi));
    if (!host)
    return -ENOMEM;
// Setup the host state.
    host.num_chipselect = 3;
    host.mode_bits = SPI_CPOL | SPI_CPHA | SPI_CS_HIGH;
    host.transfer_one = jcore_spi_txrx;
    host.set_cs = jcore_spi_chipsel;
    host.dev.of_node = node;
    host.bus_num = pdev.id;
    hw = spi_controller_get_devdata(host);
    hw.host = host;
    platform_set_drvdata(pdev, hw);
// Find and map our resources
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res)
    return -EBUSY;
    if (!devm_request_mem_region(&pdev.dev, res.start,
    resource_size(res), pdev.name))
    return -EBUSY;
    hw.base = devm_ioremap(&pdev.dev, res.start,
    resource_size(res));
    if (!hw.base)
    return -EBUSY;
//
// The SPI clock rate controlled via a configurable clock divider
// which is applied to the reference clock. A 50 MHz reference is
// most suitable for obtaining standard SPI clock rates, but some
// designs may have a different reference clock, and the DT must
// make the driver aware so that it can properly program the
// requested rate. If the clock is omitted, 50 MHz is assumed.
//
    clock_freq = 50000000;
    clk = devm_clk_get(&pdev.dev, "ref_clk");
    if (!IS_ERR(clk)) {
    if (clk_prepare_enable(clk) == 0) {
    clock_freq = clk_get_rate(clk);
    clk_disable_unprepare(clk);
    } else
    dev_warn(&pdev.dev, "could not enable ref_clk\n");
    }
    hw.clock_freq = clock_freq;
// Initialize all CS bits to high.
    hw.cs_reg = JCORE_SPI_CTRL_CS_BITS;
    jcore_spi_baudrate(hw, 400000);
// Register our spi controller
    return devm_spi_register_controller(&pdev.dev, host);
    }
    static const struct of_device_id jcore_spi_of_match[] = {
    { .compatible = "jcore,spi2" },
    {},
    };
    MODULE_DEVICE_TABLE(of, jcore_spi_of_match);
    static struct platform_driver jcore_spi_driver = {
    .probe = jcore_spi_probe,
    .driver = {
    .name = DRV_NAME,
    .of_match_table = jcore_spi_of_match,
    },
    };
    module_platform_driver(jcore_spi_driver);
    MODULE_DESCRIPTION("J-Core SPI driver");
    MODULE_AUTHOR("Rich Felker <dalias@libc.org>");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:" DRV_NAME);
