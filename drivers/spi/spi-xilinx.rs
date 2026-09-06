//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-xilinx.c
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
// Xilinx SPI controller driver (host mode only)
//
// Author: MontaVista Software, Inc.
// source@mvista.com
//
// Copyright (c) 2010 Secret Lab Technologies, Ltd.
// Copyright (c) 2009 Intel Corporation
// 2002-2007 (c) MontaVista Software, Inc.
//

pub const XILINX_SPI_MAX_CS: c_int = 32;

// Register definitions as per "OPB Serial Peripheral Interface (SPI) (v1.00e)
// Product Specification", DS464
//
pub const XSPI_CR_OFFSET: c_uint = 0x60	/* Control Register */;
pub const XSPI_CR_LOOP: c_uint = 0x01;
pub const XSPI_CR_ENABLE: c_uint = 0x02;
pub const XSPI_CR_MASTER_MODE: c_uint = 0x04;
pub const XSPI_CR_CPOL: c_uint = 0x08;
pub const XSPI_CR_CPHA: c_uint = 0x10;

    XSPI_CR_LSB_FIRST | XSPI_CR_LOOP)
pub const XSPI_CR_TXFIFO_RESET: c_uint = 0x20;
pub const XSPI_CR_RXFIFO_RESET: c_uint = 0x40;
pub const XSPI_CR_MANUAL_SSELECT: c_uint = 0x80;
pub const XSPI_CR_TRANS_INHIBIT: c_uint = 0x100;
pub const XSPI_CR_LSB_FIRST: c_uint = 0x200;
pub const XSPI_SR_OFFSET: c_uint = 0x64	/* Status Register */;
pub const XSPI_SR_RX_EMPTY_MASK: c_uint = 0x01	/* Receive FIFO is empty */;
pub const XSPI_SR_RX_FULL_MASK: c_uint = 0x02	/* Receive FIFO is full */;
pub const XSPI_SR_TX_EMPTY_MASK: c_uint = 0x04	/* Transmit FIFO is empty */;
pub const XSPI_SR_TX_FULL_MASK: c_uint = 0x08	/* Transmit FIFO is full */;
pub const XSPI_SR_MODE_FAULT_MASK: c_uint = 0x10	/* Mode fault error */;
pub const XSPI_TXD_OFFSET: c_uint = 0x68	/* Data Transmit Register */;
pub const XSPI_RXD_OFFSET: c_uint = 0x6c	/* Data Receive Register */;
pub const XSPI_SSR_OFFSET: c_uint = 0x70	/* 32-bit Slave Select Register */;
// Register definitions as per "OPB IPIF (v3.01c) Product Specification", DS414
// IPIF registers are 32 bit
//
pub const XIPIF_V123B_DGIER_OFFSET: c_uint = 0x1c	/* IPIF global int enable reg */;
pub const XIPIF_V123B_GINTR_ENABLE: c_uint = 0x80000000;
pub const XIPIF_V123B_IISR_OFFSET: c_uint = 0x20	/* IPIF interrupt status reg */;
pub const XIPIF_V123B_IIER_OFFSET: c_uint = 0x28	/* IPIF interrupt enable reg */;
pub const XSPI_INTR_MODE_FAULT: c_uint = 0x01	/* Mode fault error */;
pub const XSPI_INTR_SLAVE_MODE_FAULT: c_uint = 0x02	/* Selected as slave while;
// disabled
pub const XSPI_INTR_TX_EMPTY: c_uint = 0x04	/* TxFIFO is empty */;
pub const XSPI_INTR_TX_UNDERRUN: c_uint = 0x08	/* TxFIFO was underrun */;
pub const XSPI_INTR_RX_FULL: c_uint = 0x10	/* RxFIFO is full */;
pub const XSPI_INTR_RX_OVERRUN: c_uint = 0x20	/* RxFIFO was overrun */;
pub const XSPI_INTR_TX_HALF_EMPTY: c_uint = 0x40	/* TxFIFO is half empty */;
pub const XIPIF_V123B_RESETR_OFFSET: c_uint = 0x40	/* IPIF reset register */;
pub const XIPIF_V123B_RESET_MASK: c_uint = 0x0a	/* the value to write */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xilinx_spi {
// bitbang has to be first
    pub bitbang: spi_bitbang,
    pub done: completion,
    pub /: *mut *mut *mut void __iomem regs; / virt. address of the control registers,
    pub irq: c_int,
    pub /: *mut *mut bool force_irq; / force irq to setup host inhibit,
    pub /: *mut *mut *mut u8 rx_ptr; / pointer in the Tx buffer,
    pub /: *const *const *const u8 tx_ptr; / pointer in the Rx buffer,
    pub bytes_per_word: u8,
    pub /: *mut *mut int buffer_size; / buffer size in words,
    pub inactive*/: *mut *mut u32 cs_inactive; / Level of the CS pins when,
    pub addr): *mut *mut unsigned int (read_fn)(void __iomem,
    pub addr): *mut *mut void (write_fn)(u32 val, void __iomem,
}

#[no_mangle]
unsafe extern "C" fn xspi_write32(val: u32, addr: *mut void __iomem) {
    static void xspi_write32(u32 val, void __iomem *addr)
    {
    iowrite32(val, addr);
    }
#[no_mangle]
unsafe extern "C" fn xspi_read32(addr: *mut void __iomem) -> c_uint {
    static unsigned int xspi_read32(void __iomem *addr)
    {
    return ioread32(addr);
    }
#[no_mangle]
unsafe extern "C" fn xspi_write32_be(val: u32, addr: *mut void __iomem) {
    static void xspi_write32_be(u32 val, void __iomem *addr)
    {
    iowrite32be(val, addr);
    }
#[no_mangle]
unsafe extern "C" fn xspi_read32_be(addr: *mut void __iomem) -> c_uint {
    static unsigned int xspi_read32_be(void __iomem *addr)
    {
    return ioread32be(addr);
    }
#[no_mangle]
unsafe extern "C" fn xilinx_spi_tx(xspi: *mut xilinx_spi) {
    static void xilinx_spi_tx(struct xilinx_spi *xspi)
    {
    let mut data: u32 = 0;
    if (!xspi.tx_ptr) {
    xspi.write_fn(0, xspi.regs + XSPI_TXD_OFFSET);
    return;
    }
    switch (xspi.bytes_per_word) {
    case 1:
    data = *(u8 *)(xspi.tx_ptr);
    break;
    case 2:
    data = *(u16 *)(xspi.tx_ptr);
    break;
    case 4:
    data = *(u32 *)(xspi.tx_ptr);
    break;
    }
    xspi.write_fn(data, xspi.regs + XSPI_TXD_OFFSET);
    xspi.tx_ptr += xspi.bytes_per_word;
    }
#[no_mangle]
unsafe extern "C" fn xilinx_spi_rx(xspi: *mut xilinx_spi) {
    static void xilinx_spi_rx(struct xilinx_spi *xspi)
    {
    let mut data: u32 = xspi.read_fn(xspi.regs + XSPI_RXD_OFFSET);
    if (!xspi.rx_ptr)
    return;
    switch (xspi.bytes_per_word) {
    case 1:
// (u8 *)(xspi->rx_ptr) = data;
    break;
    case 2:
// (u16 *)(xspi->rx_ptr) = data;
    break;
    case 4:
// (u32 *)(xspi->rx_ptr) = data;
    break;
    }
    xspi.rx_ptr += xspi.bytes_per_word;
    }
#[no_mangle]
unsafe extern "C" fn xspi_init_hw(xspi: *mut xilinx_spi) {
    static void xspi_init_hw(struct xilinx_spi *xspi)
    {
    void __iomem *regs_base = xspi.regs;
// Reset the SPI device
    xspi.write_fn(XIPIF_V123B_RESET_MASK,
    regs_base + XIPIF_V123B_RESETR_OFFSET);
// Enable the transmit empty interrupt, which we use to determine
// progress on the transmission.
//
    xspi.write_fn(XSPI_INTR_TX_EMPTY,
    regs_base + XIPIF_V123B_IIER_OFFSET);
// Disable the global IPIF interrupt
    xspi.write_fn(0, regs_base + XIPIF_V123B_DGIER_OFFSET);
// Deselect the Target on the SPI bus
    xspi.write_fn(0xffff, regs_base + XSPI_SSR_OFFSET);
// Disable the transmitter, enable Manual Target Select Assertion,
// put SPI controller into host mode, and enable it
    xspi.write_fn(XSPI_CR_MANUAL_SSELECT |	XSPI_CR_MASTER_MODE |
    XSPI_CR_ENABLE | XSPI_CR_TXFIFO_RESET |	XSPI_CR_RXFIFO_RESET,
    regs_base + XSPI_CR_OFFSET);
    }
#[no_mangle]
unsafe extern "C" fn xilinx_spi_chipselect(spi: *mut spi_device, is_on: c_int) {
    static void xilinx_spi_chipselect(struct spi_device *spi, int is_on)
    {
    struct xilinx_spi *xspi = spi_controller_get_devdata(spi.controller);
    u16 cr;
    u32 cs;
    if (is_on == BITBANG_CS_INACTIVE) {
// Deselect the target on the SPI bus
    xspi.write_fn(xspi.cs_inactive, xspi.regs + XSPI_SSR_OFFSET);
    return;
    }
// Set the SPI clock phase and polarity
    cr = xspi.read_fn(xspi.regs + XSPI_CR_OFFSET)	& ~XSPI_CR_MODE_MASK;
    if (spi.mode & SPI_CPHA)
    cr |= XSPI_CR_CPHA;
    if (spi.mode & SPI_CPOL)
    cr |= XSPI_CR_CPOL;
    if (spi.mode & SPI_LSB_FIRST)
    cr |= XSPI_CR_LSB_FIRST;
    if (spi.mode & SPI_LOOP)
    cr |= XSPI_CR_LOOP;
    xspi.write_fn(cr, xspi.regs + XSPI_CR_OFFSET);
// We do not check spi->max_speed_hz here as the SPI clock
// frequency is not software programmable (the IP block design
// parameter)
//
    cs = xspi.cs_inactive;
    cs ^= BIT(spi_get_chipselect(spi, 0));
// Activate the chip select
    xspi.write_fn(cs, xspi.regs + XSPI_SSR_OFFSET);
    }
// spi_bitbang requires custom setup_transfer() to be defined if there is a
// custom txrx_bufs().
//
    static int xilinx_spi_setup_transfer(struct spi_device *spi,
    struct spi_transfer *t)
    {
    struct xilinx_spi *xspi = spi_controller_get_devdata(spi.controller);
    if (spi.mode & SPI_CS_HIGH)
    xspi.cs_inactive &= ~BIT(spi_get_chipselect(spi, 0));
    else
    xspi.cs_inactive |= BIT(spi_get_chipselect(spi, 0));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xilinx_spi_txrx_bufs(spi: *mut spi_device, t: *mut spi_transfer) -> c_int {
    static int xilinx_spi_txrx_bufs(struct spi_device *spi, struct spi_transfer *t)
    {
    struct xilinx_spi *xspi = spi_controller_get_devdata(spi.controller);
    int remaining_words;	/* the number of words left to transfer */
    let mut use_irq: bool = false;
    let mut cr: u16 = 0;
// We get here with transmitter inhibited
    xspi.tx_ptr = t.tx_buf;
    xspi.rx_ptr = t.rx_buf;
    remaining_words = t.len / xspi.bytes_per_word;
    if (xspi.irq >= 0 &&
    (xspi.force_irq || remaining_words > xspi.buffer_size)) {
    u32 isr;
    use_irq = true;
// Inhibit irq to avoid spurious irqs on tx_empty
    cr = xspi.read_fn(xspi.regs + XSPI_CR_OFFSET);
    xspi.write_fn(cr | XSPI_CR_TRANS_INHIBIT,
    xspi.regs + XSPI_CR_OFFSET);
// ACK old irqs (if any)
    isr = xspi.read_fn(xspi.regs + XIPIF_V123B_IISR_OFFSET);
    if (isr)
    xspi.write_fn(isr,
    xspi.regs + XIPIF_V123B_IISR_OFFSET);
// Enable the global IPIF interrupt
    xspi.write_fn(XIPIF_V123B_GINTR_ENABLE,
    xspi.regs + XIPIF_V123B_DGIER_OFFSET);
    reinit_completion(&xspi.done);
    }
    while (remaining_words) {
    int n_words, tx_words, rx_words;
    u32 sr;
    int stalled;
    n_words = min(remaining_words, xspi.buffer_size);
    tx_words = n_words;
    while (tx_words--)
    xilinx_spi_tx(xspi);
// Start the transfer by not inhibiting the transmitter any
// longer
//
    if (use_irq) {
    xspi.write_fn(cr, xspi.regs + XSPI_CR_OFFSET);
    if (!wait_for_completion_timeout(&xspi.done, secs_to_jiffies(1))) {
    dev_err(&spi.dev, "SPI transfer timed out\n");
    xspi_init_hw(xspi);
    return -ETIMEDOUT;
    }
// A transmit has just completed. Process received data
// and check for more data to transmit. Always inhibit
// the transmitter while the Isr refills the transmit
// register/FIFO, or make sure it is stopped if we're
// done.
//
    xspi.write_fn(cr | XSPI_CR_TRANS_INHIBIT,
    xspi.regs + XSPI_CR_OFFSET);
    sr = XSPI_SR_TX_EMPTY_MASK;
    } else
    sr = xspi.read_fn(xspi.regs + XSPI_SR_OFFSET);
// Read out all the data from the Rx FIFO
    rx_words = n_words;
    stalled = 32;
    while (rx_words) {
    if (rx_words == n_words && !(stalled--) &&
    !(sr & XSPI_SR_TX_EMPTY_MASK) &&
    (sr & XSPI_SR_RX_EMPTY_MASK)) {
    dev_err(&spi.dev,
    "Detected stall. Check C_SPI_MODE and C_SPI_MEMORY\n");
    xspi_init_hw(xspi);
    return -EIO;
    }
    if ((sr & XSPI_SR_TX_EMPTY_MASK) && (rx_words > 1)) {
    xilinx_spi_rx(xspi);
    rx_words--;
    continue;
    }
    sr = xspi.read_fn(xspi.regs + XSPI_SR_OFFSET);
    if (!(sr & XSPI_SR_RX_EMPTY_MASK)) {
    xilinx_spi_rx(xspi);
    rx_words--;
    }
    }
    remaining_words -= n_words;
    }
    if (use_irq) {
    xspi.write_fn(0, xspi.regs + XIPIF_V123B_DGIER_OFFSET);
    xspi.write_fn(cr, xspi.regs + XSPI_CR_OFFSET);
    }
    return t.len;
    }
// This driver supports single host mode only. Hence Tx FIFO Empty
// is the only interrupt we care about.
// Receive FIFO Overrun, Transmit FIFO Underrun, Mode Fault, and Target Mode
// Fault are not to happen.
//
#[no_mangle]
unsafe extern "C" fn xilinx_spi_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t xilinx_spi_irq(int irq, void *dev_id)
    {
    struct xilinx_spi *xspi = dev_id;
    u32 ipif_isr;
// Get the IPIF interrupts, and clear them immediately
    ipif_isr = xspi.read_fn(xspi.regs + XIPIF_V123B_IISR_OFFSET);
    xspi.write_fn(ipif_isr, xspi.regs + XIPIF_V123B_IISR_OFFSET);
    if (ipif_isr & XSPI_INTR_TX_EMPTY) {	/* Transmission completed */
    complete(&xspi.done);
    return IRQ_HANDLED;
    }
    return IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn xilinx_spi_find_buffer_size(xspi: *mut xilinx_spi) -> c_int {
    static int xilinx_spi_find_buffer_size(struct xilinx_spi *xspi)
    {
    u8 sr;
    let mut n_words: c_int = 0;
//
// Before the buffer_size detection we reset the core
// to make sure we start with a clean state.
//
    xspi.write_fn(XIPIF_V123B_RESET_MASK,
    xspi.regs + XIPIF_V123B_RESETR_OFFSET);
// Fill the Tx FIFO with as many words as possible
    while (1) {
    xspi.write_fn(0, xspi.regs + XSPI_TXD_OFFSET);
    sr = xspi.read_fn(xspi.regs + XSPI_SR_OFFSET);
    if (sr & XSPI_SR_TX_FULL_MASK)
    break;
    n_words++;
    }
// Handle the NO FIFO case separately
    if (!n_words)
    return 1;
    return n_words;
    }
    static const struct of_device_id xilinx_spi_of_match[] = {
    { .compatible = "xlnx,axi-quad-spi-1.00.a", },
    { .compatible = "xlnx,xps-spi-2.00.a", },
    { .compatible = "xlnx,xps-spi-2.00.b", },
    {}
    };
    MODULE_DEVICE_TABLE(of, xilinx_spi_of_match);
#[no_mangle]
unsafe extern "C" fn xilinx_spi_probe(pdev: *mut platform_device) -> c_int {
    static int xilinx_spi_probe(struct platform_device *pdev)
    {
    struct xilinx_spi *xspi;
    struct xspi_platform_data *pdata;
    struct resource *res;
    int ret, num_cs = 0, bits_per_word;
    struct spi_controller *host;
    let mut force_irq: bool = false;
    u32 tmp;
    u8 i;
    pdata = dev_get_platdata(&pdev.dev);
    if (pdata) {
    num_cs = pdata.num_chipselect;
    bits_per_word = pdata.bits_per_word;
    force_irq = pdata.force_irq;
    } else {
    device_property_read_u32(&pdev.dev, "xlnx,num-ss-bits",
    &num_cs);
    ret = device_property_read_u32(&pdev.dev,
    "xlnx,num-transfer-bits",
    &bits_per_word);
    if (ret)
    bits_per_word = 8;
    }
    if (!num_cs) {
    dev_err(&pdev.dev,
    "Missing target select configuration data\n");
    return -EINVAL;
    }
    if (num_cs > XILINX_SPI_MAX_CS) {
    dev_err(&pdev.dev, "Invalid number of spi targets\n");
    return -EINVAL;
    }
    host = devm_spi_alloc_host(&pdev.dev, sizeof(struct xilinx_spi));
    if (!host)
    return -ENODEV;
// the spi->mode bits understood by this driver:
    host.mode_bits = SPI_CPOL | SPI_CPHA | SPI_LSB_FIRST | SPI_LOOP |
    SPI_CS_HIGH;
    xspi = spi_controller_get_devdata(host);
    xspi.cs_inactive = 0xffffffff;
    xspi.bitbang.ctlr = host;
    xspi.bitbang.chipselect = xilinx_spi_chipselect;
    xspi.bitbang.setup_transfer = xilinx_spi_setup_transfer;
    xspi.bitbang.txrx_bufs = xilinx_spi_txrx_bufs;
    init_completion(&xspi.done);
    xspi.regs = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(xspi.regs))
    return PTR_ERR(xspi.regs);
    host.bus_num = pdev.id;
    host.num_chipselect = num_cs;
//
// Detect endianess on the IP via loop bit in CR. Detection
// must be done before reset is sent because incorrect reset
// value generates error interrupt.
// Setup little endian helper functions first and try to use them
// and check if bit was correctly setup or not.
//
    xspi.read_fn = xspi_read32;
    xspi.write_fn = xspi_write32;
    xspi.write_fn(XSPI_CR_LOOP, xspi.regs + XSPI_CR_OFFSET);
    tmp = xspi.read_fn(xspi.regs + XSPI_CR_OFFSET);
    tmp &= XSPI_CR_LOOP;
    if (tmp != XSPI_CR_LOOP) {
    xspi.read_fn = xspi_read32_be;
    xspi.write_fn = xspi_write32_be;
    }
    host.bits_per_word_mask = SPI_BPW_MASK(bits_per_word);
    xspi.bytes_per_word = bits_per_word / 8;
    xspi.buffer_size = xilinx_spi_find_buffer_size(xspi);
    xspi.irq = platform_get_irq_optional(pdev, 0);
    if (xspi.irq < 0 && xspi.irq != -ENXIO) {
    return xspi.irq;
    } else if (xspi.irq >= 0) {
// Register for SPI Interrupt
    ret = devm_request_irq(&pdev.dev, xspi.irq, xilinx_spi_irq, 0,
    dev_name(&pdev.dev), xspi);
    if (ret)
    return ret;
    xspi.force_irq = force_irq;
    }
// SPI controller initializations
    xspi_init_hw(xspi);
    ret = spi_bitbang_start(&xspi.bitbang);
    if (ret) {
    dev_err(&pdev.dev, "spi_bitbang_start FAILED\n");
    return ret;
    }
    dev_info(&pdev.dev, "at %pR, irq=%d\n", res, xspi.irq);
    if (pdata) {
    for (i = 0; i < pdata.num_devices; i++)
    spi_new_device(host, pdata.devices + i);
    }
    platform_set_drvdata(pdev, host);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xilinx_spi_remove(pdev: *mut platform_device) {
    static void xilinx_spi_remove(struct platform_device *pdev)
    {
    struct spi_controller *host = platform_get_drvdata(pdev);
    struct xilinx_spi *xspi = spi_controller_get_devdata(host);
    void __iomem *regs_base = xspi.regs;
    spi_bitbang_stop(&xspi.bitbang);
// Disable all the interrupts just in case
    xspi.write_fn(0, regs_base + XIPIF_V123B_IIER_OFFSET);
// Disable the global IPIF interrupt
    xspi.write_fn(0, regs_base + XIPIF_V123B_DGIER_OFFSET);
    }
// work with hotplug and coldplug
    MODULE_ALIAS("platform:" XILINX_SPI_NAME);
    static struct platform_driver xilinx_spi_driver = {
    .probe = xilinx_spi_probe,
    .remove = xilinx_spi_remove,
    .driver = {
    .name = XILINX_SPI_NAME,
    .of_match_table = xilinx_spi_of_match,
    },
    };
    module_platform_driver(xilinx_spi_driver);
    MODULE_AUTHOR("MontaVista Software, Inc. <source@mvista.com>");
    MODULE_DESCRIPTION("Xilinx SPI driver");
    MODULE_LICENSE("GPL");
