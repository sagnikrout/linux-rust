//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-st-ssc4.c
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
// Copyright (c) 2008-2014 STMicroelectronics Limited
//
// Author: Angus Clark <Angus.Clark@st.com>
// Patrice Chotard <patrice.chotard@st.com>
// Lee Jones <lee.jones@linaro.org>
//
// SPI host mode controller driver, used in STMicroelectronics devices.
//

// SSC registers
pub const SSC_BRG: c_uint = 0x000;
pub const SSC_TBUF: c_uint = 0x004;
pub const SSC_RBUF: c_uint = 0x008;
pub const SSC_CTL: c_uint = 0x00C;
pub const SSC_IEN: c_uint = 0x010;
pub const SSC_I2C: c_uint = 0x018;
// SSC Control
pub const SSC_CTL_DATA_WIDTH_9: c_uint = 0x8;
pub const SSC_CTL_DATA_WIDTH_MSK: c_uint = 0xf;
pub const SSC_CTL_BM: c_uint = 0xf;

// SSC Interrupt Enable

pub const FIFO_SIZE: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_st {
// SSC SPI Controller
    pub base: *mut void __iomem,
    pub clk: *mut clk,
    pub dev: *mut device,
// SSC SPI current transaction
    pub tx_ptr: *const u8,
    pub rx_ptr: *mut u8,
    pub bytes_per_word: u16,
    pub words_remaining: c_uint,
    pub baud: c_uint,
    pub done: completion,
}

// Load the TX FIFO
#[no_mangle]
unsafe extern "C" fn ssc_write_tx_fifo(spi_st: *mut spi_st) {
    static void ssc_write_tx_fifo(struct spi_st *spi_st)
    {
    unsigned int count, i;
    let mut word: u32 = 0;
    if (spi_st.words_remaining > FIFO_SIZE)
    count = FIFO_SIZE;
    else
    count = spi_st.words_remaining;
    for (i = 0; i < count; i++) {
    if (spi_st.tx_ptr) {
    if (spi_st.bytes_per_word == 1) {
    word = *spi_st.tx_ptr++;
    } else {
    word = *spi_st.tx_ptr++;
    word = *spi_st.tx_ptr++ | (word << 8);
    }
    }
    writel_relaxed(word, spi_st.base + SSC_TBUF);
    }
    }
// Read the RX FIFO
#[no_mangle]
unsafe extern "C" fn ssc_read_rx_fifo(spi_st: *mut spi_st) {
    static void ssc_read_rx_fifo(struct spi_st *spi_st)
    {
    unsigned int count, i;
    let mut word: u32 = 0;
    if (spi_st.words_remaining > FIFO_SIZE)
    count = FIFO_SIZE;
    else
    count = spi_st.words_remaining;
    for (i = 0; i < count; i++) {
    word = readl_relaxed(spi_st.base + SSC_RBUF);
    if (spi_st.rx_ptr) {
    if (spi_st.bytes_per_word == 1) {
// spi_st->rx_ptr++ = (uint8_t)word;
    } else {
// spi_st->rx_ptr++ = (word >> 8);
// spi_st->rx_ptr++ = word & 0xff;
    }
    }
    }
    spi_st.words_remaining -= count;
    }
    static int spi_st_transfer_one(struct spi_controller *host,
    struct spi_device *spi, struct spi_transfer *t)
    {
    struct spi_st *spi_st = spi_controller_get_devdata(host);
    let mut ctl: u32 = 0;
// Setup transfer
    spi_st.tx_ptr = t.tx_buf;
    spi_st.rx_ptr = t.rx_buf;
    if (spi.bits_per_word > 8) {
//
// Anything greater than 8 bits-per-word requires 2
// bytes-per-word in the RX/TX buffers
//
    spi_st.bytes_per_word = 2;
    spi_st.words_remaining = t.len / 2;
    } else if (spi.bits_per_word == 8 && !(t.len & 0x1)) {
//
// If transfer is even-length, and 8 bits-per-word, then
// implement as half-length 16 bits-per-word transfer
//
    spi_st.bytes_per_word = 2;
    spi_st.words_remaining = t.len / 2;
// Set SSC_CTL to 16 bits-per-word
    ctl = readl_relaxed(spi_st.base + SSC_CTL);
    writel_relaxed((ctl | 0xf), spi_st.base + SSC_CTL);
    readl_relaxed(spi_st.base + SSC_RBUF);
    } else {
    spi_st.bytes_per_word = 1;
    spi_st.words_remaining = t.len;
    }
    reinit_completion(&spi_st.done);
// Start transfer by writing to the TX FIFO
    ssc_write_tx_fifo(spi_st);
    writel_relaxed(SSC_IEN_TEEN, spi_st.base + SSC_IEN);
// Wait for transfer to complete
    wait_for_completion(&spi_st.done);
// Restore SSC_CTL if necessary
    if (ctl)
    writel_relaxed(ctl, spi_st.base + SSC_CTL);
    spi_finalize_current_transfer(spi.controller);
    return t.len;
    }
// the spi->mode bits understood by this driver:

#[no_mangle]
unsafe extern "C" fn spi_st_setup(spi: *mut spi_device) -> c_int {
    static int spi_st_setup(struct spi_device *spi)
    {
    struct spi_st *spi_st = spi_controller_get_devdata(spi.controller);
    u32 spi_st_clk, sscbrg, var;
    let mut hz: u32 = spi.max_speed_hz;
    if (!hz)  {
    dev_err(&spi.dev, "max_speed_hz unspecified\n");
    return -EINVAL;
    }
    if (!spi_get_csgpiod(spi, 0)) {
    dev_err(&spi.dev, "no valid gpio assigned\n");
    return -EINVAL;
    }
    spi_st_clk = clk_get_rate(spi_st.clk);
// Set SSC_BRF
    sscbrg = spi_st_clk / (2 * hz);
    if (sscbrg < 0x07 || sscbrg > BIT(16)) {
    dev_err(&spi.dev,
    "baudrate %d outside valid range %d\n", sscbrg, hz);
    return -EINVAL;
    }
    spi_st.baud = spi_st_clk / (2 * sscbrg);
    if (sscbrg == BIT(16)) /* 16-bit counter wraps */
    sscbrg = 0x0;
    writel_relaxed(sscbrg, spi_st.base + SSC_BRG);
    dev_dbg(&spi.dev,
    "setting baudrate:target= %u hz, actual= %u hz, sscbrg= %u\n",
    hz, spi_st.baud, sscbrg);
// Set SSC_CTL and enable SSC
    var = readl_relaxed(spi_st.base + SSC_CTL);
    var |= SSC_CTL_MS;
    if (spi.mode & SPI_CPOL)
    var |= SSC_CTL_PO;
    else
    var &= ~SSC_CTL_PO;
    if (spi.mode & SPI_CPHA)
    var |= SSC_CTL_PH;
    else
    var &= ~SSC_CTL_PH;
    if ((spi.mode & SPI_LSB_FIRST) == 0)
    var |= SSC_CTL_HB;
    else
    var &= ~SSC_CTL_HB;
    if (spi.mode & SPI_LOOP)
    var |= SSC_CTL_LPB;
    else
    var &= ~SSC_CTL_LPB;
    var &= ~SSC_CTL_DATA_WIDTH_MSK;
    var |= (spi.bits_per_word - 1);
    var |= SSC_CTL_EN_TX_FIFO | SSC_CTL_EN_RX_FIFO;
    var |= SSC_CTL_EN;
    writel_relaxed(var, spi_st.base + SSC_CTL);
// Clear the status register
    readl_relaxed(spi_st.base + SSC_RBUF);
    return 0;
    }
// Interrupt fired when TX shift register becomes empty
#[no_mangle]
unsafe extern "C" fn spi_st_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t spi_st_irq(int irq, void *dev_id)
    {
    struct spi_st *spi_st = (struct spi_st *)dev_id;
// Read RX FIFO
    ssc_read_rx_fifo(spi_st);
// Fill TX FIFO
    if (spi_st.words_remaining) {
    ssc_write_tx_fifo(spi_st);
    } else {
// TX/RX complete
    writel_relaxed(0x0, spi_st.base + SSC_IEN);
//
// read SSC_IEN to ensure that this bit is set
// before re-enabling interrupt
//
    readl(spi_st.base + SSC_IEN);
    complete(&spi_st.done);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn spi_st_probe(pdev: *mut platform_device) -> c_int {
    static int spi_st_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct spi_controller *host;
    struct spi_st *spi_st;
    int irq, ret = 0;
    u32 var;
    host = devm_spi_alloc_host(&pdev.dev, sizeof(*spi_st));
    if (!host)
    return -ENOMEM;
    host.dev.of_node		= np;
    host.mode_bits			= MODEBITS;
    host.setup			= spi_st_setup;
    host.transfer_one		= spi_st_transfer_one;
    host.bits_per_word_mask	= SPI_BPW_MASK(8) | SPI_BPW_MASK(16);
    host.auto_runtime_pm		= true;
    host.bus_num			= pdev.id;
    host.use_gpio_descriptors	= true;
    spi_st				= spi_controller_get_devdata(host);
    spi_st.clk = devm_clk_get(&pdev.dev, "ssc");
    if (IS_ERR(spi_st.clk)) {
    dev_err(&pdev.dev, "Unable to request clock\n");
    return PTR_ERR(spi_st.clk);
    }
    ret = clk_prepare_enable(spi_st.clk);
    if (ret)
    return ret;
    init_completion(&spi_st.done);
// Get resources
    spi_st.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(spi_st.base)) {
    ret = PTR_ERR(spi_st.base);
    goto clk_disable;
    }
// Disable I2C and Reset SSC
    writel_relaxed(0x0, spi_st.base + SSC_I2C);
    var = readw_relaxed(spi_st.base + SSC_CTL);
    var |= SSC_CTL_SR;
    writel_relaxed(var, spi_st.base + SSC_CTL);
    udelay(1);
    var = readl_relaxed(spi_st.base + SSC_CTL);
    var &= ~SSC_CTL_SR;
    writel_relaxed(var, spi_st.base + SSC_CTL);
// Set SSC into target mode before reconfiguring PIO pins
    var = readl_relaxed(spi_st.base + SSC_CTL);
    var &= ~SSC_CTL_MS;
    writel_relaxed(var, spi_st.base + SSC_CTL);
    irq = irq_of_parse_and_map(np, 0);
    if (!irq) {
    dev_err(&pdev.dev, "IRQ missing or invalid\n");
    ret = -EINVAL;
    goto clk_disable;
    }
    ret = devm_request_irq(&pdev.dev, irq, spi_st_irq, 0,
    pdev.name, spi_st);
    if (ret) {
    dev_err(&pdev.dev, "Failed to request irq %d\n", irq);
    goto clk_disable;
    }
// by default the device is on
    pm_runtime_set_active(&pdev.dev);
    pm_runtime_enable(&pdev.dev);
    platform_set_drvdata(pdev, host);
    ret = spi_register_controller(host);
    if (ret) {
    dev_err(&pdev.dev, "Failed to register host\n");
    goto rpm_disable;
    }
    return 0;
    rpm_disable:
    pm_runtime_disable(&pdev.dev);
    clk_disable:
    clk_disable_unprepare(spi_st.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn spi_st_remove(pdev: *mut platform_device) {
    static void spi_st_remove(struct platform_device *pdev)
    {
    struct spi_controller *host = platform_get_drvdata(pdev);
    struct spi_st *spi_st = spi_controller_get_devdata(host);
    spi_unregister_controller(host);
    pm_runtime_disable(&pdev.dev);
    clk_disable_unprepare(spi_st.clk);
    pinctrl_pm_select_sleep_state(&pdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn spi_st_runtime_suspend(dev: *mut device) -> c_int {
    static int spi_st_runtime_suspend(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    struct spi_st *spi_st = spi_controller_get_devdata(host);
    writel_relaxed(0, spi_st.base + SSC_IEN);
    pinctrl_pm_select_sleep_state(dev);
    clk_disable_unprepare(spi_st.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spi_st_runtime_resume(dev: *mut device) -> c_int {
    static int spi_st_runtime_resume(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    struct spi_st *spi_st = spi_controller_get_devdata(host);
    int ret;
    ret = clk_prepare_enable(spi_st.clk);
    pinctrl_pm_select_default_state(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn spi_st_suspend(dev: *mut device) -> c_int {
    static int spi_st_suspend(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    int ret;
    ret = spi_controller_suspend(host);
    if (ret)
    return ret;
    return pm_runtime_force_suspend(dev);
    }
#[no_mangle]
unsafe extern "C" fn spi_st_resume(dev: *mut device) -> c_int {
    static int spi_st_resume(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    int ret;
    ret = spi_controller_resume(host);
    if (ret)
    return ret;
    return pm_runtime_force_resume(dev);
    }
    static const struct dev_pm_ops spi_st_pm = {
    SYSTEM_SLEEP_PM_OPS(spi_st_suspend, spi_st_resume)
    RUNTIME_PM_OPS(spi_st_runtime_suspend, spi_st_runtime_resume, core::ptr::null_mut())
    };
    static const struct of_device_id stm_spi_match[] = {
    { .compatible = "st,comms-ssc4-spi", },
    {},
    };
    MODULE_DEVICE_TABLE(of, stm_spi_match);
    static struct platform_driver spi_st_driver = {
    .driver = {
    .name = "spi-st",
    .pm = pm_ptr(&spi_st_pm),
    .of_match_table = of_match_ptr(stm_spi_match),
    },
    .probe = spi_st_probe,
    .remove = spi_st_remove,
    };
    module_platform_driver(spi_st_driver);
    MODULE_AUTHOR("Patrice Chotard <patrice.chotard@st.com>");
    MODULE_DESCRIPTION("STM SSC SPI driver");
    MODULE_LICENSE("GPL v2");
