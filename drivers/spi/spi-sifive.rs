//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-sifive.c
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
// Copyright 2018 SiFive, Inc.
//
// SiFive SPI controller driver (master mode only)
//
// Author: SiFive, Inc.
// sifive@sifive.com

pub const SIFIVE_SPI_MAX_CS: c_int = 32;
pub const SIFIVE_SPI_DEFAULT_DEPTH: c_int = 8;
pub const SIFIVE_SPI_DEFAULT_MAX_BITS: c_int = 8;
// register offsets
pub const SIFIVE_SPI_REG_SCKDIV: c_uint = 0x00 /* Serial clock divisor */;
pub const SIFIVE_SPI_REG_SCKMODE: c_uint = 0x04 /* Serial clock mode */;
pub const SIFIVE_SPI_REG_CSID: c_uint = 0x10 /* Chip select ID */;
pub const SIFIVE_SPI_REG_CSDEF: c_uint = 0x14 /* Chip select default */;
pub const SIFIVE_SPI_REG_CSMODE: c_uint = 0x18 /* Chip select mode */;
pub const SIFIVE_SPI_REG_DELAY0: c_uint = 0x28 /* Delay control 0 */;
pub const SIFIVE_SPI_REG_DELAY1: c_uint = 0x2c /* Delay control 1 */;
pub const SIFIVE_SPI_REG_FMT: c_uint = 0x40 /* Frame format */;
pub const SIFIVE_SPI_REG_TXDATA: c_uint = 0x48 /* Tx FIFO data */;
pub const SIFIVE_SPI_REG_RXDATA: c_uint = 0x4c /* Rx FIFO data */;
pub const SIFIVE_SPI_REG_TXMARK: c_uint = 0x50 /* Tx FIFO watermark */;
pub const SIFIVE_SPI_REG_RXMARK: c_uint = 0x54 /* Rx FIFO watermark */;
pub const SIFIVE_SPI_REG_FCTRL: c_uint = 0x60 /* SPI flash interface control */;
pub const SIFIVE_SPI_REG_FFMT: c_uint = 0x64 /* SPI flash instruction format */;
pub const SIFIVE_SPI_REG_IE: c_uint = 0x70 /* Interrupt Enable Register */;
pub const SIFIVE_SPI_REG_IP: c_uint = 0x74 /* Interrupt Pendings Register */;
// sckdiv bits
pub const SIFIVE_SPI_SCKDIV_DIV_MASK: c_uint = 0xfffU;
// sckmode bits

    SIFIVE_SPI_SCKMODE_POL)
// csmode bits

// delay0 bits

pub const SIFIVE_SPI_DELAY0_CSSCK_MASK: c_uint = 0xffU;

// delay1 bits

pub const SIFIVE_SPI_DELAY1_INTERCS_MASK: c_uint = 0xffU;

// fmt bits

// txdata bits
pub const SIFIVE_SPI_TXDATA_DATA_MASK: c_uint = 0xffU;

// rxdata bits
pub const SIFIVE_SPI_RXDATA_DATA_MASK: c_uint = 0xffU;

// ie and ip bits

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sifive_spi {
    pub /: *mut *mut *mut void __iomem regs; / virt. address of control registers,
    pub /: *mut *mut *mut clk clk; / bus clock,
    pub /: *mut *mut unsigned int fifo_depth; / fifo depth in words,
    pub /: *mut *mut u32 cs_inactive; / level of the CS pins when inactive,
    pub /: *mut *mut completion done; / wake-up from interrupt,
}

#[no_mangle]
unsafe extern "C" fn sifive_spi_write(spi: *mut sifive_spi, offset: c_int, value: u32) {
    static void sifive_spi_write(struct sifive_spi *spi, int offset, u32 value)
    {
    iowrite32(value, spi.regs + offset);
    }
#[no_mangle]
unsafe extern "C" fn sifive_spi_read(spi: *mut sifive_spi, offset: c_int) -> u32 {
    static u32 sifive_spi_read(struct sifive_spi *spi, int offset)
    {
    return ioread32(spi.regs + offset);
    }
#[no_mangle]
unsafe extern "C" fn sifive_spi_init(spi: *mut sifive_spi) {
    static void sifive_spi_init(struct sifive_spi *spi)
    {
// Watermark interrupts are disabled by default
    sifive_spi_write(spi, SIFIVE_SPI_REG_IE, 0);
// Default watermark FIFO threshold values
    sifive_spi_write(spi, SIFIVE_SPI_REG_TXMARK, 1);
    sifive_spi_write(spi, SIFIVE_SPI_REG_RXMARK, 0);
// Set CS/SCK Delays and Inactive Time to defaults
    sifive_spi_write(spi, SIFIVE_SPI_REG_DELAY0,
    SIFIVE_SPI_DELAY0_CSSCK(1) |
    SIFIVE_SPI_DELAY0_SCKCS(1));
    sifive_spi_write(spi, SIFIVE_SPI_REG_DELAY1,
    SIFIVE_SPI_DELAY1_INTERCS(1) |
    SIFIVE_SPI_DELAY1_INTERXFR(0));
// Exit specialized memory-mapped SPI flash mode
    sifive_spi_write(spi, SIFIVE_SPI_REG_FCTRL, 0);
    }
    static int
    sifive_spi_prepare_message(struct spi_controller *host, struct spi_message *msg)
    {
    struct sifive_spi *spi = spi_controller_get_devdata(host);
    struct spi_device *device = msg.spi;
// Update the chip select polarity
    if (device.mode & SPI_CS_HIGH)
    spi.cs_inactive &= ~BIT(spi_get_chipselect(device, 0));
    else
    spi.cs_inactive |= BIT(spi_get_chipselect(device, 0));
    sifive_spi_write(spi, SIFIVE_SPI_REG_CSDEF, spi.cs_inactive);
// Select the correct device
    sifive_spi_write(spi, SIFIVE_SPI_REG_CSID, spi_get_chipselect(device, 0));
// Set clock mode
    sifive_spi_write(spi, SIFIVE_SPI_REG_SCKMODE,
    device.mode & SIFIVE_SPI_SCKMODE_MODE_MASK);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sifive_spi_set_cs(device: *mut spi_device, is_high: bool) {
    static void sifive_spi_set_cs(struct spi_device *device, bool is_high)
    {
    struct sifive_spi *spi = spi_controller_get_devdata(device.controller);
// Reverse polarity is handled by SCMR/CPOL. Not inverted CS.
    if (device.mode & SPI_CS_HIGH)
    is_high = !is_high;
    sifive_spi_write(spi, SIFIVE_SPI_REG_CSMODE, is_high ?
    SIFIVE_SPI_CSMODE_MODE_AUTO :
    SIFIVE_SPI_CSMODE_MODE_HOLD);
    }
    static int
    sifive_spi_prep_transfer(struct sifive_spi *spi, struct spi_device *device,
    struct spi_transfer *t)
    {
    u32 cr;
    unsigned int mode;
// Calculate and program the clock rate
    cr = DIV_ROUND_UP(clk_get_rate(spi.clk) >> 1, t.speed_hz) - 1;
    cr &= SIFIVE_SPI_SCKDIV_DIV_MASK;
    sifive_spi_write(spi, SIFIVE_SPI_REG_SCKDIV, cr);
    mode = max_t(unsigned int, t.rx_nbits, t.tx_nbits);
// Set frame format
    cr = SIFIVE_SPI_FMT_LEN(t.bits_per_word);
    switch (mode) {
    case SPI_NBITS_QUAD:
    cr |= SIFIVE_SPI_FMT_PROTO_QUAD;
    break;
    case SPI_NBITS_DUAL:
    cr |= SIFIVE_SPI_FMT_PROTO_DUAL;
    break;
    default:
    cr |= SIFIVE_SPI_FMT_PROTO_SINGLE;
    break;
    }
    if (device.mode & SPI_LSB_FIRST)
    cr |= SIFIVE_SPI_FMT_ENDIAN;
    if (!t.rx_buf)
    cr |= SIFIVE_SPI_FMT_DIR;
    sifive_spi_write(spi, SIFIVE_SPI_REG_FMT, cr);
// We will want to poll if the time we need to wait is
// less than the context switching time.
// Let's call that threshold 5us. The operation will take:
// (8/mode) * fifo_depth / hz <= 5 * 10^-6
// 1600000 * fifo_depth <= hz * mode
//
    return 1600000 * spi.fifo_depth <= t.speed_hz * mode;
    }
#[no_mangle]
unsafe extern "C" fn sifive_spi_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t sifive_spi_irq(int irq, void *dev_id)
    {
    struct sifive_spi *spi = dev_id;
    let mut ip: u32 = sifive_spi_read(spi, SIFIVE_SPI_REG_IP);
    if (ip & (SIFIVE_SPI_IP_TXWM | SIFIVE_SPI_IP_RXWM)) {
// Disable interrupts until next transfer
    sifive_spi_write(spi, SIFIVE_SPI_REG_IE, 0);
    complete(&spi.done);
    return IRQ_HANDLED;
    }
    return IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn sifive_spi_wait(spi: *mut sifive_spi, bit: u32, poll: c_int) {
    static void sifive_spi_wait(struct sifive_spi *spi, u32 bit, int poll)
    {
    if (poll) {
    u32 cr;
    do {
    cr = sifive_spi_read(spi, SIFIVE_SPI_REG_IP);
    } while (!(cr & bit));
    } else {
    reinit_completion(&spi.done);
    sifive_spi_write(spi, SIFIVE_SPI_REG_IE, bit);
    wait_for_completion(&spi.done);
    }
    }
#[no_mangle]
unsafe extern "C" fn sifive_spi_tx(spi: *mut sifive_spi, tx_ptr: *const u8) {
    static void sifive_spi_tx(struct sifive_spi *spi, const u8 *tx_ptr)
    {
    WARN_ON_ONCE((sifive_spi_read(spi, SIFIVE_SPI_REG_TXDATA)
    & SIFIVE_SPI_TXDATA_FULL) != 0);
    sifive_spi_write(spi, SIFIVE_SPI_REG_TXDATA,
// tx_ptr & SIFIVE_SPI_TXDATA_DATA_MASK);
    }
#[no_mangle]
unsafe extern "C" fn sifive_spi_rx(spi: *mut sifive_spi, rx_ptr: *mut u8) {
    static void sifive_spi_rx(struct sifive_spi *spi, u8 *rx_ptr)
    {
    let mut data: u32 = sifive_spi_read(spi, SIFIVE_SPI_REG_RXDATA);
    WARN_ON_ONCE((data & SIFIVE_SPI_RXDATA_EMPTY) != 0);
// rx_ptr = data & SIFIVE_SPI_RXDATA_DATA_MASK;
    }
    static int
    sifive_spi_transfer_one(struct spi_controller *host, struct spi_device *device,
    struct spi_transfer *t)
    {
    struct sifive_spi *spi = spi_controller_get_devdata(host);
    let mut poll: c_int = sifive_spi_prep_transfer(spi, device, t);
    const u8 *tx_ptr = t.tx_buf;
    u8 *rx_ptr = t.rx_buf;
    let mut remaining_words: c_uint = t.len;
    while (remaining_words) {
    let mut n_words: c_uint = min(remaining_words, spi.fifo_depth);
    unsigned int i;
// Enqueue n_words for transmission
    for (i = 0; i < n_words; i++)
    sifive_spi_tx(spi, tx_ptr++);
    if (rx_ptr) {
// Wait for transmission + reception to complete
    sifive_spi_write(spi, SIFIVE_SPI_REG_RXMARK,
    n_words - 1);
    sifive_spi_wait(spi, SIFIVE_SPI_IP_RXWM, poll);
// Read out all the data from the RX FIFO
    for (i = 0; i < n_words; i++)
    sifive_spi_rx(spi, rx_ptr++);
    } else {
// Wait for transmission to complete
    sifive_spi_wait(spi, SIFIVE_SPI_IP_TXWM, poll);
    }
    remaining_words -= n_words;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sifive_spi_probe(pdev: *mut platform_device) -> c_int {
    static int sifive_spi_probe(struct platform_device *pdev)
    {
    struct sifive_spi *spi;
    int ret, irq, num_cs;
    u32 cs_bits, max_bits_per_word;
    struct spi_controller *host;
    host = devm_spi_alloc_host(&pdev.dev, sizeof(struct sifive_spi));
    if (!host) {
    dev_err(&pdev.dev, "out of memory\n");
    return -ENOMEM;
    }
    spi = spi_controller_get_devdata(host);
    init_completion(&spi.done);
    platform_set_drvdata(pdev, host);
    spi.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(spi.regs))
    return PTR_ERR(spi.regs);
// Spin up the bus clock before hitting registers
    spi.clk = devm_clk_get_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(spi.clk)) {
    dev_err(&pdev.dev, "Unable to find bus clock\n");
    return PTR_ERR(spi.clk);
    }
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
// Optional parameters
    ret =
    of_property_read_u32(pdev.dev.of_node, "sifive,fifo-depth",
    &spi.fifo_depth);
    if (ret < 0)
    spi.fifo_depth = SIFIVE_SPI_DEFAULT_DEPTH;
    ret =
    of_property_read_u32(pdev.dev.of_node, "sifive,max-bits-per-word",
    &max_bits_per_word);
    if (!ret && max_bits_per_word < 8) {
    dev_err(&pdev.dev, "Only 8bit SPI words supported by the driver\n");
    return -EINVAL;
    }
// probe the number of CS lines
    spi.cs_inactive = sifive_spi_read(spi, SIFIVE_SPI_REG_CSDEF);
    sifive_spi_write(spi, SIFIVE_SPI_REG_CSDEF, 0xffffffffU);
    cs_bits = sifive_spi_read(spi, SIFIVE_SPI_REG_CSDEF);
    sifive_spi_write(spi, SIFIVE_SPI_REG_CSDEF, spi.cs_inactive);
    if (!cs_bits) {
    dev_err(&pdev.dev, "Could not auto probe CS lines\n");
    return -EINVAL;
    }
    num_cs = ilog2(cs_bits) + 1;
    if (num_cs > SIFIVE_SPI_MAX_CS) {
    dev_err(&pdev.dev, "Invalid number of spi targets\n");
    return -EINVAL;
    }
// Define our host
    host.bus_num = pdev.id;
    host.num_chipselect = num_cs;
    host.mode_bits = SPI_CPHA | SPI_CPOL
    | SPI_CS_HIGH | SPI_LSB_FIRST
    | SPI_TX_DUAL | SPI_TX_QUAD
    | SPI_RX_DUAL | SPI_RX_QUAD;
// TODO: add driver support for bits_per_word < 8
// we need to "left-align" the bits (unless SPI_LSB_FIRST)
//
    host.bits_per_word_mask = SPI_BPW_MASK(8);
    host.flags = SPI_CONTROLLER_MUST_TX | SPI_CONTROLLER_GPIO_SS;
    host.prepare_message = sifive_spi_prepare_message;
    host.set_cs = sifive_spi_set_cs;
    host.transfer_one = sifive_spi_transfer_one;
    pdev.dev.dma_mask = core::ptr::null_mut();
// Configure the SPI host hardware
    sifive_spi_init(spi);
// Register for SPI Interrupt
    ret = devm_request_irq(&pdev.dev, irq, sifive_spi_irq, 0,
    dev_name(&pdev.dev), spi);
    if (ret) {
    dev_err(&pdev.dev, "Unable to bind to interrupt\n");
    return ret;
    }
    dev_info(&pdev.dev, "mapped; irq=%d, cs=%d\n",
    irq, host.num_chipselect);
    ret = spi_register_controller(host);
    if (ret < 0) {
    dev_err(&pdev.dev, "spi_register_host failed\n");
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sifive_spi_remove(pdev: *mut platform_device) {
    static void sifive_spi_remove(struct platform_device *pdev)
    {
    struct spi_controller *host = platform_get_drvdata(pdev);
    struct sifive_spi *spi = spi_controller_get_devdata(host);
    spi_unregister_controller(host);
// Disable all the interrupts just in case
    sifive_spi_write(spi, SIFIVE_SPI_REG_IE, 0);
    }
#[no_mangle]
unsafe extern "C" fn sifive_spi_suspend(dev: *mut device) -> c_int {
    static int sifive_spi_suspend(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    struct sifive_spi *spi = spi_controller_get_devdata(host);
    int ret;
    ret = spi_controller_suspend(host);
    if (ret)
    return ret;
// Disable all the interrupts just in case
    sifive_spi_write(spi, SIFIVE_SPI_REG_IE, 0);
    clk_disable_unprepare(spi.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sifive_spi_resume(dev: *mut device) -> c_int {
    static int sifive_spi_resume(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    struct sifive_spi *spi = spi_controller_get_devdata(host);
    int ret;
    ret = clk_prepare_enable(spi.clk);
    if (ret)
    return ret;
    ret = spi_controller_resume(host);
    if (ret)
    clk_disable_unprepare(spi.clk);
    return ret;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(sifive_spi_pm_ops,
    sifive_spi_suspend, sifive_spi_resume);
    static const struct of_device_id sifive_spi_of_match[] = {
    { .compatible = "sifive,spi0", },
    {}
    };
    MODULE_DEVICE_TABLE(of, sifive_spi_of_match);
    static struct platform_driver sifive_spi_driver = {
    .probe = sifive_spi_probe,
    .remove = sifive_spi_remove,
    .driver = {
    .name = SIFIVE_SPI_DRIVER_NAME,
    .pm = &sifive_spi_pm_ops,
    .of_match_table = sifive_spi_of_match,
    },
    };
    module_platform_driver(sifive_spi_driver);
    MODULE_AUTHOR("SiFive, Inc. <sifive@sifive.com>");
    MODULE_DESCRIPTION("SiFive SPI driver");
    MODULE_LICENSE("GPL");
