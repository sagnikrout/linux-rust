//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-davinci.c
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
// Copyright (C) 2009 Texas Instruments.
// Copyright (C) 2010 EF Johnson Technologies
//

pub const CS_DEFAULT: c_uint = 0xFF;

pub const SPIFMT_WDELAY_MASK: c_uint = 0x3f000000u;
pub const SPIFMT_WDELAY_SHIFT: c_int = 24;
pub const SPIFMT_PRESCALE_SHIFT: c_int = 8;
// SPIPC0

pub const SPIINT_MASKALL: c_uint = 0x0101035F;
pub const SPIINT_MASKINT: c_uint = 0x0000015F;
pub const SPI_INTLVL_1: c_uint = 0x000001FF;
pub const SPI_INTLVL_0: c_uint = 0x00000000;
// SPIDAT1 (upper 16 bit defines)

// SPIGCR1

// SPIBUF

// SPIDELAY
pub const SPIDELAY_C2TDELAY_SHIFT: c_int = 24;

pub const SPIDELAY_T2CDELAY_SHIFT: c_int = 16;

pub const SPIDELAY_T2EDELAY_SHIFT: c_int = 8;

pub const SPIDELAY_C2EDELAY_SHIFT: c_int = 0;
pub const SPIDELAY_C2EDELAY_MASK: c_uint = 0xFF;
// Error Masks

    | SPIFLG_TIMEOUT_MASK | SPIFLG_PARERR_MASK \
    | SPIFLG_DESYNC_MASK | SPIFLG_BITERR_MASK \
    | SPIFLG_OVRRUN_MASK)

// SPI Controller registers
pub const SPIGCR0: c_uint = 0x00;
pub const SPIGCR1: c_uint = 0x04;
pub const SPIINT: c_uint = 0x08;
pub const SPILVL: c_uint = 0x0c;
pub const SPIFLG: c_uint = 0x10;
pub const SPIPC0: c_uint = 0x14;
pub const SPIDAT1: c_uint = 0x3c;
pub const SPIBUF: c_uint = 0x40;
pub const SPIDELAY: c_uint = 0x48;
pub const SPIDEF: c_uint = 0x4c;
pub const SPIFMT0: c_uint = 0x50;
pub const SPI_IO_TYPE_POLL: c_int = 1;
pub const SPI_IO_TYPE_DMA: c_int = 2;
pub const DMA_MIN_BYTES: c_int = 16;
    enum {
    SPI_VERSION_1, /* For DM355/DM365/DM6467 */
    SPI_VERSION_2, /* For DA8xx */
    };
//
// struct davinci_spi_platform_data - Platform data for SPI master device on DaVinci
//
// @version:	version of the SPI IP. Different DaVinci devices have slightly
// varying versions of the same IP.
// @num_chipselect: number of chipselects supported by this SPI master
// @intr_line:	interrupt line used to connect the SPI IP to the ARM interrupt
// controller withn the SoC. Possible values are 0 and 1.
// @prescaler_limit: max clock prescaler value
// @cshold_bug:	set this to true if the SPI controller on your chip requires
// a write to CSHOLD bit in between transfers (like in DM355).
// @dma_event_q: DMA event queue to use if SPI_IO_TYPE_DMA is used for any
// device on the bus.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct davinci_spi_platform_data {
    pub version: u8,
    pub num_chipselect: u8,
    pub intr_line: u8,
    pub prescaler_limit: u8,
    pub cshold_bug: bool,
    pub dma_event_q: enum dma_event_q,
}

//
// struct davinci_spi_config - Per-chip-select configuration for SPI slave devices
//
// @wdelay:	amount of delay between transmissions. Measured in number of
// SPI module clocks.
// @odd_parity:	polarity of parity flag at the end of transmit data stream.
// 0 - odd parity, 1 - even parity.
// @parity_enable: enable transmission of parity at end of each transmit
// data stream.
// @io_type:	type of IO transfer. Choose between polled, interrupt and DMA.
// @timer_disable: disable chip-select timers (setup and hold)
// @c2tdelay:	chip-select setup time. Measured in number of SPI module clocks.
// @t2cdelay:	chip-select hold time. Measured in number of SPI module clocks.
// @t2edelay:	transmit data finished to SPI ENAn pin inactive time. Measured
// in number of SPI clocks.
// @c2edelay:	chip-select active to SPI ENAn signal active time. Measured in
// number of SPI clocks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct davinci_spi_config {
    pub wdelay: u8,
    pub odd_parity: u8,
    pub parity_enable: u8,
    pub io_type: u8,
    pub timer_disable: u8,
    pub c2tdelay: u8,
    pub t2cdelay: u8,
    pub t2edelay: u8,
    pub c2edelay: u8,
}

// SPI Controller driver's private data.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct davinci_spi {
    pub bitbang: spi_bitbang,
    pub clk: *mut clk,
    pub version: u8,
    pub pbase: resource_size_t,
    pub base: *mut void __iomem,
    pub irq: u32,
    pub done: completion,
    pub tx: *const c_void,
    pub rx: *mut c_void,
    pub rcount: c_int,
    pub wcount: c_int,
    pub dma_rx: *mut dma_chan,
    pub dma_tx: *mut dma_chan,
    pub pdata: davinci_spi_platform_data,
    pub ): *mut *mut void (get_rx)(u32 rx_data, struct davinci_spi,
    pub ): *mut *mut u32 (get_tx)(struct davinci_spi,
    pub bytes_per_word: *mut u8,
    pub prescaler_limit: u8,
}

    static struct davinci_spi_config davinci_spi_default_cfg;
#[no_mangle]
unsafe extern "C" fn davinci_spi_rx_buf_u8(data: u32, dspi: *mut davinci_spi) {
    static void davinci_spi_rx_buf_u8(u32 data, struct davinci_spi *dspi)
    {
    if (dspi.rx) {
    u8 *rx = dspi.rx;
// rx++ = (u8)data;
    dspi.rx = rx;
    }
    }
#[no_mangle]
unsafe extern "C" fn davinci_spi_rx_buf_u16(data: u32, dspi: *mut davinci_spi) {
    static void davinci_spi_rx_buf_u16(u32 data, struct davinci_spi *dspi)
    {
    if (dspi.rx) {
    u16 *rx = dspi.rx;
// rx++ = (u16)data;
    dspi.rx = rx;
    }
    }
#[no_mangle]
unsafe extern "C" fn davinci_spi_tx_buf_u8(dspi: *mut davinci_spi) -> u32 {
    static u32 davinci_spi_tx_buf_u8(struct davinci_spi *dspi)
    {
    let mut data: u32 = 0;
    if (dspi.tx) {
    const u8 *tx = dspi.tx;
    data = *tx++;
    dspi.tx = tx;
    }
    return data;
    }
#[no_mangle]
unsafe extern "C" fn davinci_spi_tx_buf_u16(dspi: *mut davinci_spi) -> u32 {
    static u32 davinci_spi_tx_buf_u16(struct davinci_spi *dspi)
    {
    let mut data: u32 = 0;
    if (dspi.tx) {
    const u16 *tx = dspi.tx;
    data = *tx++;
    dspi.tx = tx;
    }
    return data;
    }
#[no_mangle]
pub unsafe extern "C" fn set_io_bits(addr: *mut void __iomem, bits: u32) {
    static inline void set_io_bits(void __iomem *addr, u32 bits)
    {
    let mut v: u32 = ioread32(addr);
    v |= bits;
    iowrite32(v, addr);
    }
#[no_mangle]
pub unsafe extern "C" fn clear_io_bits(addr: *mut void __iomem, bits: u32) {
    static inline void clear_io_bits(void __iomem *addr, u32 bits)
    {
    let mut v: u32 = ioread32(addr);
    v &= ~bits;
    iowrite32(v, addr);
    }
//
// Interface to control the chip select signal
//
#[no_mangle]
unsafe extern "C" fn davinci_spi_chipselect(spi: *mut spi_device, value: c_int) {
    static void davinci_spi_chipselect(struct spi_device *spi, int value)
    {
    struct davinci_spi *dspi;
    struct davinci_spi_config *spicfg = spi.controller_data;
    let mut chip_sel: u8 = spi_get_chipselect(spi, 0);
    let mut spidat1: u16 = CS_DEFAULT;
    dspi = spi_controller_get_devdata(spi.controller);
// program delay transfers if tx_delay is non zero
    if (spicfg && spicfg.wdelay)
    spidat1 |= SPIDAT1_WDEL;
//
// Board specific chip select logic decides the polarity and cs
// line for the controller
//
    if (spi_get_csgpiod(spi, 0)) {
    if (value == BITBANG_CS_ACTIVE)
    gpiod_set_value(spi_get_csgpiod(spi, 0), 1);
    else
    gpiod_set_value(spi_get_csgpiod(spi, 0), 0);
    } else {
    if (value == BITBANG_CS_ACTIVE) {
    if (!(spi.mode & SPI_CS_WORD))
    spidat1 |= SPIDAT1_CSHOLD_MASK;
    spidat1 &= ~(0x1 << chip_sel);
    }
    }
    iowrite16(spidat1, dspi.base + SPIDAT1 + 2);
    }
//
// davinci_spi_get_prescale - Calculates the correct prescale value
// @dspi: the controller data
// @max_speed_hz: the maximum rate the SPI clock can run at
//
// This function calculates the prescale value that generates a clock rate
// less than or equal to the specified maximum.
//
// Returns: calculated prescale value for easy programming into SPI registers
// or negative error number if valid prescalar cannot be updated.
//
    static inline int davinci_spi_get_prescale(struct davinci_spi *dspi,
    u32 max_speed_hz)
    {
    int ret;
// Subtract 1 to match what will be programmed into SPI register.
    ret = DIV_ROUND_UP(clk_get_rate(dspi.clk), max_speed_hz) - 1;
    if (ret < dspi.prescaler_limit || ret > 255)
    return -EINVAL;
    return ret;
    }
//
// davinci_spi_setup_transfer - This functions will determine transfer method
// @spi: spi device on which data transfer to be done
// @t: spi transfer in which transfer info is filled
//
// This function determines data transfer method (8/16/32 bit transfer).
// It will also set the SPI Clock Control register according to
// SPI slave device freq.
//
    static int davinci_spi_setup_transfer(struct spi_device *spi,
    struct spi_transfer *t)
    {
    struct davinci_spi *dspi;
    struct davinci_spi_config *spicfg;
    let mut bits_per_word: u8 = 0;
    let mut hz: u32 = 0, spifmt = 0;
    int prescale;
    dspi = spi_controller_get_devdata(spi.controller);
    spicfg = spi.controller_data;
    if (!spicfg)
    spicfg = &davinci_spi_default_cfg;
    if (t) {
    bits_per_word = t.bits_per_word;
    hz = t.speed_hz;
    }
// if bits_per_word is not set then set it default
    if (!bits_per_word)
    bits_per_word = spi.bits_per_word;
//
// Assign function pointer to appropriate transfer method
// 8bit, 16bit or 32bit transfer
//
    if (bits_per_word <= 8) {
    dspi.get_rx = davinci_spi_rx_buf_u8;
    dspi.get_tx = davinci_spi_tx_buf_u8;
    dspi.bytes_per_word[spi_get_chipselect(spi, 0)] = 1;
    } else {
    dspi.get_rx = davinci_spi_rx_buf_u16;
    dspi.get_tx = davinci_spi_tx_buf_u16;
    dspi.bytes_per_word[spi_get_chipselect(spi, 0)] = 2;
    }
    if (!hz)
    hz = spi.max_speed_hz;
// Set up SPIFMTn register, unique to this chipselect.
    prescale = davinci_spi_get_prescale(dspi, hz);
    if (prescale < 0)
    return prescale;
    spifmt = (prescale << SPIFMT_PRESCALE_SHIFT) | (bits_per_word & 0x1f);
    if (spi.mode & SPI_LSB_FIRST)
    spifmt |= SPIFMT_SHIFTDIR_MASK;
    if (spi.mode & SPI_CPOL)
    spifmt |= SPIFMT_POLARITY_MASK;
    if (!(spi.mode & SPI_CPHA))
    spifmt |= SPIFMT_PHASE_MASK;
//
// Assume wdelay is used only on SPI peripherals that has this field
// in SPIFMTn register and when it's configured from board file or DT.
//
    if (spicfg.wdelay)
    spifmt |= ((spicfg.wdelay << SPIFMT_WDELAY_SHIFT)
    & SPIFMT_WDELAY_MASK);
//
// Version 1 hardware supports two basic SPI modes:
// - Standard SPI mode uses 4 pins, with chipselect
// - 3 pin SPI is a 4 pin variant without CS (SPI_NO_CS)
// (distinct from SPI_3WIRE, with just one data wire;
// or similar variants without MOSI or without MISO)
//
// Version 2 hardware supports an optional handshaking signal,
// so it can support two more modes:
// - 5 pin SPI variant is standard SPI plus SPI_READY
// - 4 pin with enable is (SPI_READY | SPI_NO_CS)
//
    if (dspi.version == SPI_VERSION_2) {
    let mut delay: u32 = 0;
    if (spicfg.odd_parity)
    spifmt |= SPIFMT_ODD_PARITY_MASK;
    if (spicfg.parity_enable)
    spifmt |= SPIFMT_PARITYENA_MASK;
    if (spicfg.timer_disable) {
    spifmt |= SPIFMT_DISTIMER_MASK;
    } else {
    delay |= (spicfg.c2tdelay << SPIDELAY_C2TDELAY_SHIFT)
    & SPIDELAY_C2TDELAY_MASK;
    delay |= (spicfg.t2cdelay << SPIDELAY_T2CDELAY_SHIFT)
    & SPIDELAY_T2CDELAY_MASK;
    }
    if (spi.mode & SPI_READY) {
    spifmt |= SPIFMT_WAITENA_MASK;
    delay |= (spicfg.t2edelay << SPIDELAY_T2EDELAY_SHIFT)
    & SPIDELAY_T2EDELAY_MASK;
    delay |= (spicfg.c2edelay << SPIDELAY_C2EDELAY_SHIFT)
    & SPIDELAY_C2EDELAY_MASK;
    }
    iowrite32(delay, dspi.base + SPIDELAY);
    }
    iowrite32(spifmt, dspi.base + SPIFMT0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn davinci_spi_of_setup(spi: *mut spi_device) -> c_int {
    static int davinci_spi_of_setup(struct spi_device *spi)
    {
    struct davinci_spi_config *spicfg = spi.controller_data;
    struct device_node *np = spi.dev.of_node;
    struct davinci_spi *dspi = spi_controller_get_devdata(spi.controller);
    u32 prop;
    if (spicfg == core::ptr::null_mut() && np) {
    spicfg = kzalloc_obj(*spicfg);
    if (!spicfg)
    return -ENOMEM;
// spicfg = davinci_spi_default_cfg;
// override with dt configured values
    if (!of_property_read_u32(np, "ti,spi-wdelay", &prop))
    spicfg.wdelay = (u8)prop;
    spi.controller_data = spicfg;
    if (dspi.dma_rx && dspi.dma_tx)
    spicfg.io_type = SPI_IO_TYPE_DMA;
    }
    return 0;
    }
//
// davinci_spi_setup - This functions will set default transfer method
// @spi: spi device on which data transfer to be done
//
// This functions sets the default transfer method.
//
#[no_mangle]
unsafe extern "C" fn davinci_spi_setup(spi: *mut spi_device) -> c_int {
    static int davinci_spi_setup(struct spi_device *spi)
    {
    struct davinci_spi *dspi;
    struct device_node *np = spi.dev.of_node;
    let mut internal_cs: bool = true;
    dspi = spi_controller_get_devdata(spi.controller);
    if (!(spi.mode & SPI_NO_CS)) {
    if (np && spi_get_csgpiod(spi, 0))
    internal_cs = false;
    if (internal_cs)
    set_io_bits(dspi.base + SPIPC0, 1 << spi_get_chipselect(spi, 0));
    }
    if (spi.mode & SPI_READY)
    set_io_bits(dspi.base + SPIPC0, SPIPC0_SPIENA_MASK);
    if (spi.mode & SPI_LOOP)
    set_io_bits(dspi.base + SPIGCR1, SPIGCR1_LOOPBACK_MASK);
    else
    clear_io_bits(dspi.base + SPIGCR1, SPIGCR1_LOOPBACK_MASK);
    return davinci_spi_of_setup(spi);
    }
#[no_mangle]
unsafe extern "C" fn davinci_spi_cleanup(spi: *mut spi_device) {
    static void davinci_spi_cleanup(struct spi_device *spi)
    {
    struct davinci_spi_config *spicfg = spi.controller_data;
    spi.controller_data = core::ptr::null_mut();
    if (spi.dev.of_node)
    kfree(spicfg);
    }
    static bool davinci_spi_can_dma(struct spi_controller *host,
    struct spi_device *spi,
    struct spi_transfer *xfer)
    {
    struct davinci_spi_config *spicfg = spi.controller_data;
    let mut can_dma: bool = false;
    if (spicfg)
    can_dma = (spicfg.io_type == SPI_IO_TYPE_DMA) &&
    (xfer.len >= DMA_MIN_BYTES) &&
    !is_vmalloc_addr(xfer.rx_buf) &&
    !is_vmalloc_addr(xfer.tx_buf);
    return can_dma;
    }
#[no_mangle]
unsafe extern "C" fn davinci_spi_check_error(dspi: *mut davinci_spi, int_status: c_int) -> c_int {
    static int davinci_spi_check_error(struct davinci_spi *dspi, int int_status)
    {
    struct device *sdev = dspi.bitbang.ctlr.dev.parent;
    if (int_status & SPIFLG_TIMEOUT_MASK) {
    dev_err(sdev, "SPI Time-out Error\n");
    return -ETIMEDOUT;
    }
    if (int_status & SPIFLG_DESYNC_MASK) {
    dev_err(sdev, "SPI Desynchronization Error\n");
    return -EIO;
    }
    if (int_status & SPIFLG_BITERR_MASK) {
    dev_err(sdev, "SPI Bit error\n");
    return -EIO;
    }
    if (dspi.version == SPI_VERSION_2) {
    if (int_status & SPIFLG_DLEN_ERR_MASK) {
    dev_err(sdev, "SPI Data Length Error\n");
    return -EIO;
    }
    if (int_status & SPIFLG_PARERR_MASK) {
    dev_err(sdev, "SPI Parity Error\n");
    return -EIO;
    }
    if (int_status & SPIFLG_OVRRUN_MASK) {
    dev_err(sdev, "SPI Data Overrun error\n");
    return -EIO;
    }
    if (int_status & SPIFLG_BUF_INIT_ACTIVE_MASK) {
    dev_err(sdev, "SPI Buffer Init Active\n");
    return -EBUSY;
    }
    }
    return 0;
    }
//
// davinci_spi_process_events - check for and handle any SPI controller events
// @dspi: the controller data
//
// This function will check the SPIFLG register and handle any events that are
// detected there
//
#[no_mangle]
unsafe extern "C" fn davinci_spi_process_events(dspi: *mut davinci_spi) -> c_int {
    static int davinci_spi_process_events(struct davinci_spi *dspi)
    {
    u32 buf, status, errors = 0, spidat1;
    buf = ioread32(dspi.base + SPIBUF);
    if (dspi.rcount > 0 && !(buf & SPIBUF_RXEMPTY_MASK)) {
    dspi.get_rx(buf & 0xFFFF, dspi);
    dspi.rcount--;
    }
    status = ioread32(dspi.base + SPIFLG);
    if (unlikely(status & SPIFLG_ERROR_MASK)) {
    errors = status & SPIFLG_ERROR_MASK;
    goto out;
    }
    if (dspi.wcount > 0 && !(buf & SPIBUF_TXFULL_MASK)) {
    spidat1 = ioread32(dspi.base + SPIDAT1);
    dspi.wcount--;
    spidat1 &= ~0xFFFF;
    spidat1 |= 0xFFFF & dspi.get_tx(dspi);
    iowrite32(spidat1, dspi.base + SPIDAT1);
    }
    out:
    return errors;
    }
#[no_mangle]
unsafe extern "C" fn davinci_spi_dma_rx_callback(data: *mut c_void) {
    static void davinci_spi_dma_rx_callback(void *data)
    {
    struct davinci_spi *dspi = (struct davinci_spi *)data;
    dspi.rcount = 0;
    if (!dspi.wcount && !dspi.rcount)
    complete(&dspi.done);
    }
#[no_mangle]
unsafe extern "C" fn davinci_spi_dma_tx_callback(data: *mut c_void) {
    static void davinci_spi_dma_tx_callback(void *data)
    {
    struct davinci_spi *dspi = (struct davinci_spi *)data;
    dspi.wcount = 0;
    if (!dspi.wcount && !dspi.rcount)
    complete(&dspi.done);
    }
//
// davinci_spi_bufs - functions which will handle transfer data
// @spi: spi device on which data transfer to be done
// @t: spi transfer in which transfer info is filled
//
// This function will put data to be transferred into data register
// of SPI controller and then wait until the completion will be marked
// by the IRQ Handler.
//
#[no_mangle]
unsafe extern "C" fn davinci_spi_bufs(spi: *mut spi_device, t: *mut spi_transfer) -> c_int {
    static int davinci_spi_bufs(struct spi_device *spi, struct spi_transfer *t)
    {
    struct davinci_spi *dspi;
    int data_type, ret = -ENOMEM;
    u32 tx_data, spidat1;
    let mut errors: u32 = 0;
    struct davinci_spi_config *spicfg;
    struct davinci_spi_platform_data *pdata;
    unsigned long timeout;
    dspi = spi_controller_get_devdata(spi.controller);
    pdata = &dspi.pdata;
    spicfg = (struct davinci_spi_config *)spi.controller_data;
    if (!spicfg)
    spicfg = &davinci_spi_default_cfg;
// convert len to words based on bits_per_word
    data_type = dspi.bytes_per_word[spi_get_chipselect(spi, 0)];
    dspi.tx = t.tx_buf;
    dspi.rx = t.rx_buf;
    dspi.wcount = t.len / data_type;
    dspi.rcount = dspi.wcount;
    spidat1 = ioread32(dspi.base + SPIDAT1);
    clear_io_bits(dspi.base + SPIGCR1, SPIGCR1_POWERDOWN_MASK);
    set_io_bits(dspi.base + SPIGCR1, SPIGCR1_SPIENA_MASK);
    reinit_completion(&dspi.done);
    if (!davinci_spi_can_dma(spi.controller, spi, t)) {
    if (spicfg.io_type != SPI_IO_TYPE_POLL)
    set_io_bits(dspi.base + SPIINT, SPIINT_MASKINT);
// start the transfer
    dspi.wcount--;
    tx_data = dspi.get_tx(dspi);
    spidat1 &= 0xFFFF0000;
    spidat1 |= tx_data & 0xFFFF;
    iowrite32(spidat1, dspi.base + SPIDAT1);
    } else {
    struct dma_slave_config dma_rx_conf = {
    .direction = DMA_DEV_TO_MEM,
    .src_addr = (unsigned long)dspi.pbase + SPIBUF,
    .src_addr_width = data_type,
    .src_maxburst = 1,
    };
    struct dma_slave_config dma_tx_conf = {
    .direction = DMA_MEM_TO_DEV,
    .dst_addr = (unsigned long)dspi.pbase + SPIDAT1,
    .dst_addr_width = data_type,
    .dst_maxburst = 1,
    };
    struct dma_async_tx_descriptor *rxdesc;
    struct dma_async_tx_descriptor *txdesc;
    dmaengine_slave_config(dspi.dma_rx, &dma_rx_conf);
    dmaengine_slave_config(dspi.dma_tx, &dma_tx_conf);
    rxdesc = dmaengine_prep_slave_sg(dspi.dma_rx,
    t.rx_sg.sgl, t.rx_sg.nents, DMA_DEV_TO_MEM,
    DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    if (!rxdesc)
    goto err_desc;
    if (!t.tx_buf) {
// To avoid errors when doing rx-only transfers with
// many SG entries (> 20), use the rx buffer as the
// dummy tx buffer so that dma reloads are done at the
// same time for rx and tx.
//
    t.tx_sg.sgl = t.rx_sg.sgl;
    t.tx_sg.nents = t.rx_sg.nents;
    }
    txdesc = dmaengine_prep_slave_sg(dspi.dma_tx,
    t.tx_sg.sgl, t.tx_sg.nents, DMA_MEM_TO_DEV,
    DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    if (!txdesc)
    goto err_desc;
    rxdesc.callback = davinci_spi_dma_rx_callback;
    rxdesc.callback_param = (void *)dspi;
    txdesc.callback = davinci_spi_dma_tx_callback;
    txdesc.callback_param = (void *)dspi;
    if (pdata.cshold_bug)
    iowrite16(spidat1 >> 16, dspi.base + SPIDAT1 + 2);
    dmaengine_submit(rxdesc);
    dmaengine_submit(txdesc);
    dma_async_issue_pending(dspi.dma_rx);
    dma_async_issue_pending(dspi.dma_tx);
    set_io_bits(dspi.base + SPIINT, SPIINT_DMA_REQ_EN);
    }
// Wait for the transfer to complete
    if (spicfg.io_type != SPI_IO_TYPE_POLL) {
    timeout = DIV_ROUND_UP(t.speed_hz, MSEC_PER_SEC);
    timeout = DIV_ROUND_UP(t.len * 8, timeout);
// Assume we are at most 2x slower than the nominal bus speed
    timeout = 2 * msecs_to_jiffies(timeout);
    if (wait_for_completion_timeout(&dspi.done, timeout) == 0)
    errors = SPIFLG_TIMEOUT_MASK;
    } else {
    while (dspi.rcount > 0 || dspi.wcount > 0) {
    errors = davinci_spi_process_events(dspi);
    if (errors)
    break;
    cpu_relax();
    }
    }
    clear_io_bits(dspi.base + SPIINT, SPIINT_MASKALL);
    if (davinci_spi_can_dma(spi.controller, spi, t))
    clear_io_bits(dspi.base + SPIINT, SPIINT_DMA_REQ_EN);
    clear_io_bits(dspi.base + SPIGCR1, SPIGCR1_SPIENA_MASK);
    set_io_bits(dspi.base + SPIGCR1, SPIGCR1_POWERDOWN_MASK);
//
// Check for bit error, desync error,parity error,timeout error and
// receive overflow errors
//
    if (errors) {
    ret = davinci_spi_check_error(dspi, errors);
    WARN(!ret, "%s: error reported but no error found!\n",
    dev_name(&spi.dev));
    return ret;
    }
    if (dspi.rcount != 0 || dspi.wcount != 0) {
    dev_err(&spi.dev, "SPI data transfer error\n");
    return -EIO;
    }
    return t.len;
    err_desc:
    return ret;
    }
//
// dummy_thread_fn - dummy thread function
// @irq: IRQ number for this SPI Master
// @data: structure for SPI Master controller davinci_spi
//
// This is to satisfy the request_threaded_irq() API so that the irq
// handler is called in interrupt context.
//
#[no_mangle]
unsafe extern "C" fn dummy_thread_fn(irq: i32, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t dummy_thread_fn(s32 irq, void *data)
    {
    return IRQ_HANDLED;
    }
//
// davinci_spi_irq - Interrupt handler for SPI Master Controller
// @irq: IRQ number for this SPI Master
// @data: structure for SPI Master controller davinci_spi
//
// ISR will determine that interrupt arrives either for READ or WRITE command.
// According to command it will do the appropriate action. It will check
// transfer length and if it is not zero then dispatch transfer command again.
// If transfer length is zero then it will indicate the COMPLETION so that
// davinci_spi_bufs function can go ahead.
//
#[no_mangle]
unsafe extern "C" fn davinci_spi_irq(irq: i32, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t davinci_spi_irq(s32 irq, void *data)
    {
    struct davinci_spi *dspi = data;
    int status;
    status = davinci_spi_process_events(dspi);
    if (unlikely(status != 0))
    clear_io_bits(dspi.base + SPIINT, SPIINT_MASKINT);
    if ((!dspi.rcount && !dspi.wcount) || status)
    complete(&dspi.done);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn davinci_spi_request_dma(dspi: *mut davinci_spi) -> c_int {
    static int davinci_spi_request_dma(struct davinci_spi *dspi)
    {
    struct device *sdev = dspi.bitbang.ctlr.dev.parent;
    dspi.dma_rx = dma_request_chan(sdev, "rx");
    if (IS_ERR(dspi.dma_rx))
    return PTR_ERR(dspi.dma_rx);
    dspi.dma_tx = dma_request_chan(sdev, "tx");
    if (IS_ERR(dspi.dma_tx)) {
    dma_release_channel(dspi.dma_rx);
    return PTR_ERR(dspi.dma_tx);
    }
    return 0;
    }

// OF SPI data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct davinci_spi_of_data {
    pub version: u8,
    pub prescaler_limit: u8,
}

    static const struct davinci_spi_of_data dm6441_spi_data = {
    .version = SPI_VERSION_1,
    .prescaler_limit = 2,
    };
    static const struct davinci_spi_of_data da830_spi_data = {
    .version = SPI_VERSION_2,
    .prescaler_limit = 2,
    };
    static const struct davinci_spi_of_data keystone_spi_data = {
    .version = SPI_VERSION_1,
    .prescaler_limit = 0,
    };
    static const struct of_device_id davinci_spi_of_match[] = {
    {
    .compatible = "ti,dm6441-spi",
    .data = &dm6441_spi_data,
    },
    {
    .compatible = "ti,da830-spi",
    .data = &da830_spi_data,
    },
    {
    .compatible = "ti,keystone-spi",
    .data = &keystone_spi_data,
    },
    { },
    };
    MODULE_DEVICE_TABLE(of, davinci_spi_of_match);
//
// spi_davinci_get_pdata - Get platform data from DTS binding
// @pdev: ptr to platform data
// @dspi: ptr to driver data
//
// Parses and populates pdata in dspi from device tree bindings.
//
// NOTE: Not all platform data params are supported currently.
//
    static int spi_davinci_get_pdata(struct platform_device *pdev,
    struct davinci_spi *dspi)
    {
    struct device_node *node = pdev.dev.of_node;
    const struct davinci_spi_of_data *spi_data;
    struct davinci_spi_platform_data *pdata;
    unsigned int num_cs, intr_line = 0;
    pdata = &dspi.pdata;
    spi_data = device_get_match_data(&pdev.dev);
    pdata.version = spi_data.version;
    pdata.prescaler_limit = spi_data.prescaler_limit;
//
// default num_cs is 1 and all chipsel are internal to the chip
// indicated by chip_sel being NULL or cs_gpios being NULL or
// set to -ENOENT. num-cs includes internal as well as gpios.
// indicated by chip_sel being NULL. GPIO based CS is not
// supported yet in DT bindings.
//
    num_cs = 1;
    of_property_read_u32(node, "num-cs", &num_cs);
    pdata.num_chipselect = num_cs;
    of_property_read_u32(node, "ti,davinci-spi-intr-line", &intr_line);
    pdata.intr_line = intr_line;
    return 0;
    }

    static int spi_davinci_get_pdata(struct platform_device *pdev,
    struct davinci_spi *dspi)
    {
    return -ENODEV;
    }

//
// davinci_spi_probe - probe function for SPI Master Controller
// @pdev: platform_device structure which contains plateform specific data
//
// According to Linux Device Model this function will be invoked by Linux
// with platform_device struct which contains the device specific info.
// This function will map the SPI controller's memory, register IRQ,
// Reset SPI controller and setting its registers to default value.
// It will invoke spi_bitbang_start to create work queue so that client driver
// can register transfer method to work queue.
//
#[no_mangle]
unsafe extern "C" fn davinci_spi_probe(pdev: *mut platform_device) -> c_int {
    static int davinci_spi_probe(struct platform_device *pdev)
    {
    struct spi_controller *host;
    struct davinci_spi *dspi;
    struct davinci_spi_platform_data *pdata;
    struct resource *r;
    let mut ret: c_int = 0;
    u32 spipc0;
    host = devm_spi_alloc_host(&pdev.dev, sizeof(struct davinci_spi));
    if (host == core::ptr::null_mut()) {
    ret = -ENOMEM;
    goto err;
    }
    platform_set_drvdata(pdev, host);
    dspi = spi_controller_get_devdata(host);
    if (dev_get_platdata(&pdev.dev)) {
    pdata = dev_get_platdata(&pdev.dev);
    dspi.pdata = *pdata;
    } else {
// update dspi pdata with that from the DT
    ret = spi_davinci_get_pdata(pdev, dspi);
    if (ret < 0)
    goto free_host;
    }
// pdata in dspi is now updated and point pdata to that
    pdata = &dspi.pdata;
    dspi.bytes_per_word = devm_kcalloc(&pdev.dev,
    pdata.num_chipselect,
    sizeof(*dspi.bytes_per_word),
    GFP_KERNEL);
    if (dspi.bytes_per_word == core::ptr::null_mut()) {
    ret = -ENOMEM;
    goto free_host;
    }
    dspi.base = devm_platform_get_and_ioremap_resource(pdev, 0, &r);
    if (IS_ERR(dspi.base)) {
    ret = PTR_ERR(dspi.base);
    goto free_host;
    }
    dspi.pbase = r.start;
    init_completion(&dspi.done);
    ret = platform_get_irq(pdev, 0);
    if (ret < 0)
    goto free_host;
    dspi.irq = ret;
    ret = devm_request_threaded_irq(&pdev.dev, dspi.irq, davinci_spi_irq,
    dummy_thread_fn, 0, dev_name(&pdev.dev), dspi);
    if (ret)
    goto free_host;
    dspi.bitbang.ctlr = host;
    dspi.clk = devm_clk_get_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(dspi.clk)) {
    ret = -ENODEV;
    goto free_host;
    }
    host.use_gpio_descriptors = true;
    host.bus_num = pdev.id;
    host.num_chipselect = pdata.num_chipselect;
    host.bits_per_word_mask = SPI_BPW_RANGE_MASK(2, 16);
    host.flags = SPI_CONTROLLER_MUST_RX | SPI_CONTROLLER_GPIO_SS;
    host.setup = davinci_spi_setup;
    host.cleanup = davinci_spi_cleanup;
    host.can_dma = davinci_spi_can_dma;
    dspi.bitbang.chipselect = davinci_spi_chipselect;
    dspi.bitbang.setup_transfer = davinci_spi_setup_transfer;
    dspi.prescaler_limit = pdata.prescaler_limit;
    dspi.version = pdata.version;
    dspi.bitbang.flags = SPI_NO_CS | SPI_LSB_FIRST | SPI_LOOP | SPI_CS_WORD;
    if (dspi.version == SPI_VERSION_2)
    dspi.bitbang.flags |= SPI_READY;
    dspi.bitbang.txrx_bufs = davinci_spi_bufs;
    ret = davinci_spi_request_dma(dspi);
    if (ret == -EPROBE_DEFER) {
    goto free_host;
    } else if (ret) {
    dev_info(&pdev.dev, "DMA is not supported (%d)\n", ret);
    dspi.dma_rx = core::ptr::null_mut();
    dspi.dma_tx = core::ptr::null_mut();
    }
    dspi.get_rx = davinci_spi_rx_buf_u8;
    dspi.get_tx = davinci_spi_tx_buf_u8;
// Reset In/OUT SPI module
    iowrite32(0, dspi.base + SPIGCR0);
    udelay(100);
    iowrite32(1, dspi.base + SPIGCR0);
// Set up SPIPC0.  CS and ENA init is done in davinci_spi_setup
    spipc0 = SPIPC0_DIFUN_MASK | SPIPC0_DOFUN_MASK | SPIPC0_CLKFUN_MASK;
    iowrite32(spipc0, dspi.base + SPIPC0);
    if (pdata.intr_line)
    iowrite32(SPI_INTLVL_1, dspi.base + SPILVL);
    else
    iowrite32(SPI_INTLVL_0, dspi.base + SPILVL);
    iowrite32(CS_DEFAULT, dspi.base + SPIDEF);
// host mode default
    set_io_bits(dspi.base + SPIGCR1, SPIGCR1_CLKMOD_MASK);
    set_io_bits(dspi.base + SPIGCR1, SPIGCR1_MASTER_MASK);
    set_io_bits(dspi.base + SPIGCR1, SPIGCR1_POWERDOWN_MASK);
    ret = spi_bitbang_start(&dspi.bitbang);
    if (ret)
    goto free_dma;
    dev_info(&pdev.dev, "Controller at 0x%p\n", dspi.base);
    return ret;
    free_dma:
// This bit needs to be cleared to disable dpsi->clk
    clear_io_bits(dspi.base + SPIGCR1, SPIGCR1_POWERDOWN_MASK);
    if (dspi.dma_rx) {
    dma_release_channel(dspi.dma_rx);
    dma_release_channel(dspi.dma_tx);
    }
    free_host:
    err:
    return ret;
    }
//
// davinci_spi_remove - remove function for SPI Master Controller
// @pdev: platform_device structure which contains plateform specific data
//
// This function will do the reverse action of davinci_spi_probe function
// It will free the IRQ and SPI controller's memory region.
// It will also call spi_bitbang_stop to destroy the work queue which was
// created by spi_bitbang_start.
//
#[no_mangle]
unsafe extern "C" fn davinci_spi_remove(pdev: *mut platform_device) {
    static void davinci_spi_remove(struct platform_device *pdev)
    {
    struct davinci_spi *dspi;
    struct spi_controller *host;
    host = platform_get_drvdata(pdev);
    dspi = spi_controller_get_devdata(host);
    spi_bitbang_stop(&dspi.bitbang);
    devm_free_irq(&pdev.dev, dspi.irq, dspi);
// This bit needs to be cleared to disable dpsi->clk
    clear_io_bits(dspi.base + SPIGCR1, SPIGCR1_POWERDOWN_MASK);
    if (dspi.dma_rx) {
    dma_release_channel(dspi.dma_rx);
    dma_release_channel(dspi.dma_tx);
    }
    }
    static struct platform_driver davinci_spi_driver = {
    .driver = {
    .name = "spi_davinci",
    .of_match_table = of_match_ptr(davinci_spi_of_match),
    },
    .probe = davinci_spi_probe,
    .remove = davinci_spi_remove,
    };
    module_platform_driver(davinci_spi_driver);
    MODULE_DESCRIPTION("TI DaVinci SPI Master Controller Driver");
    MODULE_LICENSE("GPL");
