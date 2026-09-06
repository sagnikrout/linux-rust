//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-apple.c
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
// Apple SoC SPI device driver
//
// Copyright The Asahi Linux Contributors
//
// Based on spi-sifive.c, Copyright 2018 SiFive, Inc.

pub const APPLE_SPI_CTRL: c_uint = 0x000;

pub const APPLE_SPI_CFG: c_uint = 0x004;

pub const APPLE_SPI_CFG_MODE_POLLED: c_int = 0;
pub const APPLE_SPI_CFG_MODE_IRQ: c_int = 1;
pub const APPLE_SPI_CFG_MODE_DMA: c_int = 2;

pub const APPLE_SPI_CFG_WORD_SIZE_8B: c_int = 0;
pub const APPLE_SPI_CFG_WORD_SIZE_16B: c_int = 1;
pub const APPLE_SPI_CFG_WORD_SIZE_32B: c_int = 2;

pub const APPLE_SPI_CFG_FIFO_THRESH_8B: c_int = 0;
pub const APPLE_SPI_CFG_FIFO_THRESH_4B: c_int = 1;
pub const APPLE_SPI_CFG_FIFO_THRESH_1B: c_int = 2;

pub const APPLE_SPI_STATUS: c_uint = 0x008;

pub const APPLE_SPI_PIN: c_uint = 0x00c;

pub const APPLE_SPI_TXDATA: c_uint = 0x010;
pub const APPLE_SPI_RXDATA: c_uint = 0x020;
pub const APPLE_SPI_CLKDIV: c_uint = 0x030;
pub const APPLE_SPI_CLKDIV_MAX: c_uint = 0x7ff;
pub const APPLE_SPI_RXCNT: c_uint = 0x034;
pub const APPLE_SPI_WORD_DELAY: c_uint = 0x038;
pub const APPLE_SPI_TXCNT: c_uint = 0x04c;
pub const APPLE_SPI_FIFOSTAT: c_uint = 0x10c;

pub const APPLE_SPI_IE_XFER: c_uint = 0x130;
pub const APPLE_SPI_IF_XFER: c_uint = 0x134;

pub const APPLE_SPI_IE_FIFO: c_uint = 0x138;
pub const APPLE_SPI_IF_FIFO: c_uint = 0x13c;

pub const APPLE_SPI_SHIFTCFG: c_uint = 0x150;

pub const APPLE_SPI_PINCFG: c_uint = 0x154;

pub const APPLE_SPI_DELAY_PRE: c_uint = 0x160;
pub const APPLE_SPI_DELAY_POST: c_uint = 0x168;

pub const APPLE_SPI_FIFO_DEPTH: c_int = 16;
//
// The slowest refclock available is 24MHz, the highest divider is 0x7ff,
// the largest word size is 32 bits, the FIFO depth is 16, the maximum
// intra-word delay is 0xffff refclocks. So the maximum time a transfer
// cycle can take is:
//
// (0x7ff * 32 + 0xffff) * 16 / 24e6 Hz ~= 87ms
//
// Double it and round it up to 200ms for good measure.
//
pub const APPLE_SPI_TIMEOUT_MS: c_int = 200;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apple_spi {
    pub /: *mut *mut *mut void __iomem regs; / MMIO register address,
    pub /: *mut *mut *mut clk clk; / bus clock,
    pub /: *mut *mut completion done; / wake-up from interrupt,
}

#[no_mangle]
pub unsafe extern "C" fn reg_write(spi: *mut apple_spi, offset: c_int, value: u32) {
    static inline void reg_write(struct apple_spi *spi, int offset, u32 value)
    {
    writel_relaxed(value, spi.regs + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn reg_read(spi: *mut apple_spi, offset: c_int) -> u32 {
    static inline u32 reg_read(struct apple_spi *spi, int offset)
    {
    return readl_relaxed(spi.regs + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn reg_mask(spi: *mut apple_spi, offset: c_int, clear: u32, set: u32) {
    static inline void reg_mask(struct apple_spi *spi, int offset, u32 clear, u32 set)
    {
    let mut val: u32 = reg_read(spi, offset);
    val &= ~clear;
    val |= set;
    reg_write(spi, offset, val);
    }
#[no_mangle]
unsafe extern "C" fn apple_spi_init(spi: *mut apple_spi) {
    static void apple_spi_init(struct apple_spi *spi)
    {
// Set CS high (inactive) and disable override and auto-CS
    reg_write(spi, APPLE_SPI_PIN, APPLE_SPI_PIN_CS);
    reg_mask(spi, APPLE_SPI_SHIFTCFG, APPLE_SPI_SHIFTCFG_OVERRIDE_CS, 0);
    reg_mask(spi, APPLE_SPI_PINCFG, APPLE_SPI_PINCFG_CS_IDLE_VAL, APPLE_SPI_PINCFG_KEEP_CS);
// Reset FIFOs
    reg_write(spi, APPLE_SPI_CTRL, APPLE_SPI_CTRL_RX_RESET | APPLE_SPI_CTRL_TX_RESET);
// Configure defaults
    reg_write(spi, APPLE_SPI_CFG,
    FIELD_PREP(APPLE_SPI_CFG_FIFO_THRESH, APPLE_SPI_CFG_FIFO_THRESH_8B) |
    FIELD_PREP(APPLE_SPI_CFG_MODE, APPLE_SPI_CFG_MODE_IRQ) |
    FIELD_PREP(APPLE_SPI_CFG_WORD_SIZE, APPLE_SPI_CFG_WORD_SIZE_8B));
// Disable IRQs
    reg_write(spi, APPLE_SPI_IE_FIFO, 0);
    reg_write(spi, APPLE_SPI_IE_XFER, 0);
// Disable delays
    reg_write(spi, APPLE_SPI_DELAY_PRE, 0);
    reg_write(spi, APPLE_SPI_DELAY_POST, 0);
    }
#[no_mangle]
unsafe extern "C" fn apple_spi_prepare_message(ctlr: *mut spi_controller, msg: *mut spi_message) -> c_int {
    static int apple_spi_prepare_message(struct spi_controller *ctlr, struct spi_message *msg)
    {
    struct apple_spi *spi = spi_controller_get_devdata(ctlr);
    struct spi_device *device = msg.spi;
    u32 cfg = ((device.mode & SPI_CPHA ? APPLE_SPI_CFG_CPHA : 0) |
    (device.mode & SPI_CPOL ? APPLE_SPI_CFG_CPOL : 0) |
    (device.mode & SPI_LSB_FIRST ? APPLE_SPI_CFG_LSB_FIRST : 0));
// Update core config
    reg_mask(spi, APPLE_SPI_CFG,
    APPLE_SPI_CFG_CPHA | APPLE_SPI_CFG_CPOL | APPLE_SPI_CFG_LSB_FIRST, cfg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn apple_spi_set_cs(device: *mut spi_device, is_high: bool) {
    static void apple_spi_set_cs(struct spi_device *device, bool is_high)
    {
    struct apple_spi *spi = spi_controller_get_devdata(device.controller);
    reg_mask(spi, APPLE_SPI_PIN, APPLE_SPI_PIN_CS, is_high ? APPLE_SPI_PIN_CS : 0);
    }
#[no_mangle]
unsafe extern "C" fn apple_spi_prep_transfer(spi: *mut apple_spi, t: *mut spi_transfer) -> bool {
    static bool apple_spi_prep_transfer(struct apple_spi *spi, struct spi_transfer *t)
    {
    u32 cr, fifo_threshold;
// Calculate and program the clock rate
    cr = DIV_ROUND_UP(clk_get_rate(spi.clk), t.speed_hz);
    reg_write(spi, APPLE_SPI_CLKDIV, min_t(u32, cr, APPLE_SPI_CLKDIV_MAX));
// Update bits per word
    reg_mask(spi, APPLE_SPI_SHIFTCFG, APPLE_SPI_SHIFTCFG_BITS,
    FIELD_PREP(APPLE_SPI_SHIFTCFG_BITS, t.bits_per_word));
// We will want to poll if the time we need to wait is
// less than the context switching time.
// Let's call that threshold 5us. The operation will take:
// bits_per_word * fifo_threshold / hz <= 5 * 10^-6
// 200000 * bits_per_word * fifo_threshold <= hz
//
    fifo_threshold = APPLE_SPI_FIFO_DEPTH / 2;
    return (200000 * t.bits_per_word * fifo_threshold) <= t.speed_hz;
    }
#[no_mangle]
unsafe extern "C" fn apple_spi_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t apple_spi_irq(int irq, void *dev_id)
    {
    struct apple_spi *spi = dev_id;
    let mut fifo: u32 = reg_read(spi, APPLE_SPI_IF_FIFO) & reg_read(spi, APPLE_SPI_IE_FIFO);
    let mut xfer: u32 = reg_read(spi, APPLE_SPI_IF_XFER) & reg_read(spi, APPLE_SPI_IE_XFER);
    if (fifo || xfer) {
// Disable interrupts until next transfer
    reg_write(spi, APPLE_SPI_IE_XFER, 0);
    reg_write(spi, APPLE_SPI_IE_FIFO, 0);
    complete(&spi.done);
    return IRQ_HANDLED;
    }
    return IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn apple_spi_wait(spi: *mut apple_spi, fifo_bit: u32, xfer_bit: u32, poll: c_int) -> c_int {
    static int apple_spi_wait(struct apple_spi *spi, u32 fifo_bit, u32 xfer_bit, int poll)
    {
    let mut ret: c_int = 0;
    if (poll) {
    u32 fifo, xfer;
    let mut timeout: c_ulong = jiffies + APPLE_SPI_TIMEOUT_MS * HZ / 1000;
    do {
    fifo = reg_read(spi, APPLE_SPI_IF_FIFO);
    xfer = reg_read(spi, APPLE_SPI_IF_XFER);
    if (time_after(jiffies, timeout)) {
    ret = -ETIMEDOUT;
    break;
    }
    } while (!((fifo & fifo_bit) || (xfer & xfer_bit)));
    } else {
    reinit_completion(&spi.done);
    reg_write(spi, APPLE_SPI_IE_XFER, xfer_bit);
    reg_write(spi, APPLE_SPI_IE_FIFO, fifo_bit);
    if (!wait_for_completion_timeout(&spi.done,
    msecs_to_jiffies(APPLE_SPI_TIMEOUT_MS)))
    ret = -ETIMEDOUT;
    reg_write(spi, APPLE_SPI_IE_XFER, 0);
    reg_write(spi, APPLE_SPI_IE_FIFO, 0);
    }
    return ret;
    }
    static void apple_spi_tx(struct apple_spi *spi, const void **tx_ptr, u32 *left,
    unsigned int bytes_per_word)
    {
    u32 inuse, words, wrote;
    if (!*tx_ptr)
    return;
    inuse = FIELD_GET(APPLE_SPI_FIFOSTAT_LEVEL_TX, reg_read(spi, APPLE_SPI_FIFOSTAT));
    words = wrote = min_t(u32, *left, APPLE_SPI_FIFO_DEPTH - inuse);
    if (!words)
    return;
// left -= words;
    switch (bytes_per_word) {
    case 1: {
    const u8 *p = *tx_ptr;
    while (words--)
    reg_write(spi, APPLE_SPI_TXDATA, *p++);
    break;
    }
    case 2: {
    const u16 *p = *tx_ptr;
    while (words--)
    reg_write(spi, APPLE_SPI_TXDATA, *p++);
    break;
    }
    case 4: {
    const u32 *p = *tx_ptr;
    while (words--)
    reg_write(spi, APPLE_SPI_TXDATA, *p++);
    break;
    }
    default:
    WARN_ON(1);
    }
// tx_ptr = ((u8 *)*tx_ptr) + bytes_per_word * wrote;
    }
    static void apple_spi_rx(struct apple_spi *spi, void **rx_ptr, u32 *left,
    unsigned int bytes_per_word)
    {
    u32 words, read;
    if (!*rx_ptr)
    return;
    words = read = FIELD_GET(APPLE_SPI_FIFOSTAT_LEVEL_RX, reg_read(spi, APPLE_SPI_FIFOSTAT));
    WARN_ON(words > *left);
    if (!words)
    return;
// left -= min_t(u32, *left, words);
    switch (bytes_per_word) {
    case 1: {
    u8 *p = *rx_ptr;
    while (words--)
// p++ = reg_read(spi, APPLE_SPI_RXDATA);
    break;
    }
    case 2: {
    u16 *p = *rx_ptr;
    while (words--)
// p++ = reg_read(spi, APPLE_SPI_RXDATA);
    break;
    }
    case 4: {
    u32 *p = *rx_ptr;
    while (words--)
// p++ = reg_read(spi, APPLE_SPI_RXDATA);
    break;
    }
    default:
    WARN_ON(1);
    }
// rx_ptr = ((u8 *)*rx_ptr) + bytes_per_word * read;
    }
    static int apple_spi_transfer_one(struct spi_controller *ctlr, struct spi_device *device,
    struct spi_transfer *t)
    {
    struct apple_spi *spi = spi_controller_get_devdata(ctlr);
    let mut poll: bool = apple_spi_prep_transfer(spi, t);
    const void *tx_ptr = t.tx_buf;
    void *rx_ptr = t.rx_buf;
    unsigned int bytes_per_word;
    u32 words, remaining_tx, remaining_rx;
    let mut xfer_flags: u32 = 0;
    u32 fifo_flags;
    let mut retries: c_int = 100;
    let mut ret: c_int = 0;
    if (t.bits_per_word > 16)
    bytes_per_word = 4;
#[no_mangle]
pub unsafe extern "C" fn if(8: t->bits_per_word >) -> else {
    else if (t.bits_per_word > 8)
    bytes_per_word = 2;
    else
    bytes_per_word = 1;
    words = t.len / bytes_per_word;
    remaining_tx = tx_ptr ? words : 0;
    remaining_rx = rx_ptr ? words : 0;
// Reset FIFOs
    reg_write(spi, APPLE_SPI_CTRL, APPLE_SPI_CTRL_RX_RESET | APPLE_SPI_CTRL_TX_RESET);
// Clear IRQ flags
    reg_write(spi, APPLE_SPI_IF_XFER, ~0);
    reg_write(spi, APPLE_SPI_IF_FIFO, ~0);
// Determine transfer completion flags we wait for
    if (tx_ptr)
    xfer_flags |= APPLE_SPI_XFER_TXCOMPLETE;
    if (rx_ptr)
    xfer_flags |= APPLE_SPI_XFER_RXCOMPLETE;
// Set transfer length
    reg_write(spi, APPLE_SPI_TXCNT, remaining_tx);
    reg_write(spi, APPLE_SPI_RXCNT, remaining_rx);
// Prime transmit FIFO
    apple_spi_tx(spi, &tx_ptr, &remaining_tx, bytes_per_word);
// Start transfer
    reg_write(spi, APPLE_SPI_CTRL, APPLE_SPI_CTRL_RUN);
// TX again since a few words get popped off immediately
    apple_spi_tx(spi, &tx_ptr, &remaining_tx, bytes_per_word);
    while (xfer_flags) {
    fifo_flags = 0;
    if (remaining_tx)
    fifo_flags |= APPLE_SPI_FIFO_TXTHRESH;
    if (remaining_rx)
    fifo_flags |= APPLE_SPI_FIFO_RXTHRESH;
// Wait for anything to happen
    ret = apple_spi_wait(spi, fifo_flags, xfer_flags, poll);
    if (ret) {
    dev_err(&ctlr.dev, "transfer timed out (remaining %d tx, %d rx)\n",
    remaining_tx, remaining_rx);
    goto err;
    }
// Stop waiting on transfer halves once they complete
    xfer_flags &= ~reg_read(spi, APPLE_SPI_IF_XFER);
// Transmit and receive everything we can
    apple_spi_tx(spi, &tx_ptr, &remaining_tx, bytes_per_word);
    apple_spi_rx(spi, &rx_ptr, &remaining_rx, bytes_per_word);
    }
//
// Sometimes the transfer completes before the last word is in the RX FIFO.
// Normally one retry is all it takes to get the last word out.
//
    while (remaining_rx && retries--)
    apple_spi_rx(spi, &rx_ptr, &remaining_rx, bytes_per_word);
    if (remaining_tx)
    dev_err(&ctlr.dev, "transfer completed with %d words left to transmit\n",
    remaining_tx);
    if (remaining_rx)
    dev_err(&ctlr.dev, "transfer completed with %d words left to receive\n",
    remaining_rx);
    err:
    fifo_flags = reg_read(spi, APPLE_SPI_IF_FIFO);
    WARN_ON(fifo_flags & APPLE_SPI_FIFO_TXOVERFLOW);
    WARN_ON(fifo_flags & APPLE_SPI_FIFO_RXUNDERRUN);
// Stop transfer
    reg_write(spi, APPLE_SPI_CTRL, 0);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn apple_spi_probe(pdev: *mut platform_device) -> c_int {
    static int apple_spi_probe(struct platform_device *pdev)
    {
    struct apple_spi *spi;
    int ret, irq;
    struct spi_controller *ctlr;
    ctlr = devm_spi_alloc_host(&pdev.dev, sizeof(struct apple_spi));
    if (!ctlr)
    return -ENOMEM;
    spi = spi_controller_get_devdata(ctlr);
    init_completion(&spi.done);
    spi.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(spi.regs))
    return PTR_ERR(spi.regs);
    spi.clk = devm_clk_get_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(spi.clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(spi.clk),
    "Unable to find or enable bus clock\n");
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    ret = devm_request_irq(&pdev.dev, irq, apple_spi_irq, 0,
    dev_name(&pdev.dev), spi);
    if (ret)
    return dev_err_probe(&pdev.dev, ret, "Unable to bind to interrupt\n");
    ctlr.bus_num = pdev.id;
    ctlr.num_chipselect = 1;
    ctlr.mode_bits = SPI_CPHA | SPI_CPOL | SPI_LSB_FIRST;
    ctlr.bits_per_word_mask = SPI_BPW_RANGE_MASK(1, 32);
    ctlr.prepare_message = apple_spi_prepare_message;
    ctlr.set_cs = apple_spi_set_cs;
    ctlr.transfer_one = apple_spi_transfer_one;
    ctlr.use_gpio_descriptors = true;
    ctlr.auto_runtime_pm = true;
    pm_runtime_set_active(&pdev.dev);
    ret = devm_pm_runtime_enable(&pdev.dev);
    if (ret < 0)
    return ret;
    apple_spi_init(spi);
    ret = devm_spi_register_controller(&pdev.dev, ctlr);
    if (ret < 0)
    return dev_err_probe(&pdev.dev, ret, "devm_spi_register_controller failed\n");
    return 0;
    }
    static const struct of_device_id apple_spi_of_match[] = {
    { .compatible = "apple,t8103-spi", },
    { .compatible = "apple,spi", },
    {}
    };
    MODULE_DEVICE_TABLE(of, apple_spi_of_match);
    static struct platform_driver apple_spi_driver = {
    .probe = apple_spi_probe,
    .driver = {
    .name = "apple-spi",
    .of_match_table = apple_spi_of_match,
    },
    };
    module_platform_driver(apple_spi_driver);
    MODULE_AUTHOR("Hector Martin <marcan@marcan.st>");
    MODULE_DESCRIPTION("Apple SoC SPI driver");
    MODULE_LICENSE("GPL");
