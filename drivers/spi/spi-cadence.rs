//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-cadence.c
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
// Cadence SPI controller driver (host and target mode)
//
// Copyright (C) 2008 - 2014 Xilinx, Inc.
//
// based on Blackfin On-Chip SPI Driver (spi_bfin5xx.c)
//

// Name of this driver

// Register offset definitions
pub const CDNS_SPI_CR: c_uint = 0x00 /* Configuration  Register, RW */;
pub const CDNS_SPI_ISR: c_uint = 0x04 /* Interrupt Status Register, RO */;
pub const CDNS_SPI_IER: c_uint = 0x08 /* Interrupt Enable Register, WO */;
pub const CDNS_SPI_IDR: c_uint = 0x0c /* Interrupt Disable Register, WO */;
pub const CDNS_SPI_IMR: c_uint = 0x10 /* Interrupt Enabled Mask Register, RO */;
pub const CDNS_SPI_ER: c_uint = 0x14 /* Enable/Disable Register, RW */;
pub const CDNS_SPI_DR: c_uint = 0x18 /* Delay Register, RW */;
pub const CDNS_SPI_TXD: c_uint = 0x1C /* Data Transmit Register, WO */;
pub const CDNS_SPI_RXD: c_uint = 0x20 /* Data Receive Register, RO */;
pub const CDNS_SPI_SICR: c_uint = 0x24 /* Slave Idle Count Register, RW */;
pub const CDNS_SPI_THLD: c_uint = 0x28 /* Transmit FIFO Watermark Register,RW */;
pub const SPI_AUTOSUSPEND_TIMEOUT: c_int = 3000;
//
// SPI Configuration Register bit Masks
//
// This register contains various control bits that affect the operation
// of the SPI controller
//
pub const CDNS_SPI_CR_MANSTRT: c_uint = 0x00010000 /* Manual TX Start */;
pub const CDNS_SPI_CR_CPHA: c_uint = 0x00000004 /* Clock Phase Control */;
pub const CDNS_SPI_CR_CPOL: c_uint = 0x00000002 /* Clock Polarity Control */;
pub const CDNS_SPI_CR_SSCTRL: c_uint = 0x00003C00 /* Slave Select Mask */;
pub const CDNS_SPI_CR_PERI_SEL: c_uint = 0x00000200 /* Peripheral Select Decode */;
pub const CDNS_SPI_CR_BAUD_DIV: c_uint = 0x00000038 /* Baud Rate Divisor Mask */;
pub const CDNS_SPI_CR_MSTREN: c_uint = 0x00000001 /* Master Enable Mask */;
pub const CDNS_SPI_CR_MANSTRTEN: c_uint = 0x00008000 /* Manual TX Enable Mask */;
pub const CDNS_SPI_CR_SSFORCE: c_uint = 0x00004000 /* Manual SS Enable Mask */;
pub const CDNS_SPI_CR_BAUD_DIV_4: c_uint = 0x00000008 /* Default Baud Div Mask */;

    CDNS_SPI_CR_SSCTRL | \
    CDNS_SPI_CR_SSFORCE | \
    CDNS_SPI_CR_BAUD_DIV_4)
//
// SPI Configuration Register - Baud rate and target select
//
// These are the values used in the calculation of baud rate divisor and
// setting the target select.
//

pub const CDNS_SPI_SS0: c_uint = 0x1 /* Slave Select zero */;
pub const CDNS_SPI_NOSS: c_uint = 0xF /* No Slave select */;
//
// SPI Interrupt Registers bit Masks
//
// All the four interrupt registers (Status/Mask/Enable/Disable) have the same
// bit definitions.
//
pub const CDNS_SPI_IXR_TXOW: c_uint = 0x00000004 /* SPI TX FIFO Overwater */;
pub const CDNS_SPI_IXR_MODF: c_uint = 0x00000002 /* SPI Mode Fault */;
pub const CDNS_SPI_IXR_RXNEMTY: c_uint = 0x00000010 /* SPI RX FIFO Not Empty */;

    CDNS_SPI_IXR_MODF)
pub const CDNS_SPI_IXR_TXFULL: c_uint = 0x00000008 /* SPI TX Full */;
pub const CDNS_SPI_IXR_ALL: c_uint = 0x0000007F /* SPI all interrupts */;
//
// SPI Enable Register bit Masks
//
// This register is used to enable or disable the SPI controller
//
pub const CDNS_SPI_ER_ENABLE: c_uint = 0x00000001 /* SPI Enable Bit Mask */;
pub const CDNS_SPI_ER_DISABLE: c_uint = 0x0 /* SPI Disable Bit Mask */;
// Default number of chip select lines
pub const CDNS_SPI_DEFAULT_NUM_CS: c_int = 4;
//
// struct cdns_spi - This definition defines spi driver instance
// @regs:		Virtual address of the SPI controller registers
// @ref_clk:		Pointer to the peripheral clock
// @pclk:		Pointer to the APB clock
// @clk_rate:		Reference clock frequency, taken from @ref_clk
// @speed_hz:		Current SPI bus clock speed in Hz
// @txbuf:		Pointer	to the TX buffer
// @rxbuf:		Pointer to the RX buffer
// @tx_bytes:		Number of bytes left to transfer
// @rx_bytes:		Number of bytes requested
// @n_bytes:		Number of bytes per word
// @dev_busy:		Device busy flag
// @is_decoded_cs:	Flag for decoder property set or not
// @tx_fifo_depth:	Depth of the TX FIFO
// @rstc:		Optional reset control for SPI controller
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_spi {
    pub regs: *mut void __iomem,
    pub ref_clk: *mut clk,
    pub pclk: *mut clk,
    pub clk_rate: c_uint,
    pub speed_hz: u32,
    pub txbuf: *const c_void,
    pub rxbuf: *mut c_void,
    pub tx_bytes: c_int,
    pub rx_bytes: c_int,
    pub n_bytes: u8,
    pub dev_busy: u8,
    pub is_decoded_cs: u32,
    pub tx_fifo_depth: c_uint,
    pub rstc: *mut reset_control,
}

    enum cdns_spi_frame_n_bytes {
    CDNS_SPI_N_BYTES_NULL = 0,
    CDNS_SPI_N_BYTES_U8 = 1,
    CDNS_SPI_N_BYTES_U16 = 2,
    CDNS_SPI_N_BYTES_U32 = 4
    };
// Macros for the SPI controller read/write
#[no_mangle]
pub unsafe extern "C" fn cdns_spi_read(xspi: *mut cdns_spi, offset: u32) -> u32 {
    static inline u32 cdns_spi_read(struct cdns_spi *xspi, u32 offset)
    {
    return readl_relaxed(xspi.regs + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn cdns_spi_write(xspi: *mut cdns_spi, offset: u32, val: u32) {
    static inline void cdns_spi_write(struct cdns_spi *xspi, u32 offset, u32 val)
    {
    writel_relaxed(val, xspi.regs + offset);
    }
//
// cdns_spi_init_hw - Initialize the hardware and configure the SPI controller
// @xspi:	Pointer to the cdns_spi structure
// @is_target:	Flag to indicate target or host mode
// * On reset the SPI controller is configured to target or host mode.
// In host mode baud rate divisor is set to 4, threshold value for TX FIFO
// not full interrupt is set to 1 and size of the word to be transferred as 8 bit.
//
// This function initializes the SPI controller to disable and clear all the
// interrupts, enable manual target select and manual start, deselect all the
// chip select lines, and enable the SPI controller.
//
#[no_mangle]
unsafe extern "C" fn cdns_spi_init_hw(xspi: *mut cdns_spi, is_target: bool) {
    static void cdns_spi_init_hw(struct cdns_spi *xspi, bool is_target)
    {
    let mut ctrl_reg: u32 = 0;
    if (!is_target)
    ctrl_reg |= CDNS_SPI_CR_DEFAULT;
    if (xspi.is_decoded_cs)
    ctrl_reg |= CDNS_SPI_CR_PERI_SEL;
    cdns_spi_write(xspi, CDNS_SPI_ER, CDNS_SPI_ER_DISABLE);
    cdns_spi_write(xspi, CDNS_SPI_IDR, CDNS_SPI_IXR_ALL);
// Clear the RX FIFO
    while (cdns_spi_read(xspi, CDNS_SPI_ISR) & CDNS_SPI_IXR_RXNEMTY)
    cdns_spi_read(xspi, CDNS_SPI_RXD);
    cdns_spi_write(xspi, CDNS_SPI_ISR, CDNS_SPI_IXR_ALL);
    cdns_spi_write(xspi, CDNS_SPI_CR, ctrl_reg);
    cdns_spi_write(xspi, CDNS_SPI_ER, CDNS_SPI_ER_ENABLE);
    }
//
// cdns_spi_chipselect - Select or deselect the chip select line
// @spi:	Pointer to the spi_device structure
// @is_high:	Select(0) or deselect (1) the chip select line
//
#[no_mangle]
unsafe extern "C" fn cdns_spi_chipselect(spi: *mut spi_device, is_high: bool) {
    static void cdns_spi_chipselect(struct spi_device *spi, bool is_high)
    {
    struct cdns_spi *xspi = spi_controller_get_devdata(spi.controller);
    u32 ctrl_reg;
    ctrl_reg = cdns_spi_read(xspi, CDNS_SPI_CR);
    if (is_high) {
// Deselect the target
    ctrl_reg |= CDNS_SPI_CR_SSCTRL;
    } else {
// Select the target
    ctrl_reg &= ~CDNS_SPI_CR_SSCTRL;
    if (!(xspi.is_decoded_cs))
    ctrl_reg |= ((~(CDNS_SPI_SS0 << spi_get_chipselect(spi, 0))) <<
    CDNS_SPI_SS_SHIFT) &
    CDNS_SPI_CR_SSCTRL;
    else
    ctrl_reg |= (spi_get_chipselect(spi, 0) << CDNS_SPI_SS_SHIFT) &
    CDNS_SPI_CR_SSCTRL;
    }
    cdns_spi_write(xspi, CDNS_SPI_CR, ctrl_reg);
    }
//
// cdns_spi_config_clock_mode - Sets clock polarity and phase
// @spi:	Pointer to the spi_device structure
//
// Sets the requested clock polarity and phase.
//
#[no_mangle]
unsafe extern "C" fn cdns_spi_config_clock_mode(spi: *mut spi_device) {
    static void cdns_spi_config_clock_mode(struct spi_device *spi)
    {
    struct cdns_spi *xspi = spi_controller_get_devdata(spi.controller);
    u32 ctrl_reg, new_ctrl_reg;
    new_ctrl_reg = cdns_spi_read(xspi, CDNS_SPI_CR);
    ctrl_reg = new_ctrl_reg;
// Set the SPI clock phase and clock polarity
    new_ctrl_reg &= ~(CDNS_SPI_CR_CPHA | CDNS_SPI_CR_CPOL);
    if (spi.mode & SPI_CPHA)
    new_ctrl_reg |= CDNS_SPI_CR_CPHA;
    if (spi.mode & SPI_CPOL)
    new_ctrl_reg |= CDNS_SPI_CR_CPOL;
    if (new_ctrl_reg != ctrl_reg) {
//
// Just writing the CR register does not seem to apply the clock
// setting changes. This is problematic when changing the clock
// polarity as it will cause the SPI target to see spurious clock
// transitions. To workaround the issue toggle the ER register.
//
    cdns_spi_write(xspi, CDNS_SPI_ER, CDNS_SPI_ER_DISABLE);
    cdns_spi_write(xspi, CDNS_SPI_CR, new_ctrl_reg);
    cdns_spi_write(xspi, CDNS_SPI_ER, CDNS_SPI_ER_ENABLE);
    }
    }
//
// cdns_spi_config_clock_freq - Sets clock frequency
// @spi:	Pointer to the spi_device structure
// @transfer:	Pointer to the spi_transfer structure which provides
// information about next transfer setup parameters
//
// Sets the requested clock frequency.
// Note: If the requested frequency is not an exact match with what can be
// obtained using the prescalar value the driver sets the clock frequency which
// is lower than the requested frequency (maximum lower) for the transfer. If
// the requested frequency is higher or lower than that is supported by the SPI
// controller the driver will set the highest or lowest frequency supported by
// controller.
//
    static void cdns_spi_config_clock_freq(struct spi_device *spi,
    struct spi_transfer *transfer)
    {
    struct cdns_spi *xspi = spi_controller_get_devdata(spi.controller);
    u32 ctrl_reg, baud_rate_val;
    unsigned long frequency;
    frequency = xspi.clk_rate;
    ctrl_reg = cdns_spi_read(xspi, CDNS_SPI_CR);
// Set the clock frequency
    if (xspi.speed_hz != transfer.speed_hz) {
// first valid value is 1
    baud_rate_val = CDNS_SPI_BAUD_DIV_MIN;
    while ((baud_rate_val < CDNS_SPI_BAUD_DIV_MAX) &&
    (frequency / (2 << baud_rate_val)) > transfer.speed_hz)
    baud_rate_val++;
    ctrl_reg &= ~CDNS_SPI_CR_BAUD_DIV;
    ctrl_reg |= baud_rate_val << CDNS_SPI_BAUD_DIV_SHIFT;
    xspi.speed_hz = frequency / (2 << baud_rate_val);
    }
    cdns_spi_write(xspi, CDNS_SPI_CR, ctrl_reg);
    }
//
// cdns_spi_setup_transfer - Configure SPI controller for specified transfer
// @spi:	Pointer to the spi_device structure
// @transfer:	Pointer to the spi_transfer structure which provides
// information about next transfer setup parameters
//
// Sets the operational mode of SPI controller for the next SPI transfer and
// sets the requested clock frequency.
//
// Return:	Always 0
//
    static int cdns_spi_setup_transfer(struct spi_device *spi,
    struct spi_transfer *transfer)
    {
    struct cdns_spi *xspi = spi_controller_get_devdata(spi.controller);
    cdns_spi_config_clock_freq(spi, transfer);
    dev_dbg(&spi.dev, "%s, mode %d, %u bits/w, %u clock speed\n",
    __func__, spi.mode, spi.bits_per_word,
    xspi.speed_hz);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cdns_spi_n_bytes(transfer: *mut spi_transfer) -> u8 {
    static u8 cdns_spi_n_bytes(struct spi_transfer *transfer)
    {
    if (transfer.bits_per_word <= 8)
    return CDNS_SPI_N_BYTES_U8;
#[no_mangle]
pub unsafe extern "C" fn if(16: transfer->bits_per_word <=) -> else {
    else if (transfer.bits_per_word <= 16)
    return CDNS_SPI_N_BYTES_U16;
    else
    return CDNS_SPI_N_BYTES_U32;
    }
#[no_mangle]
pub unsafe extern "C" fn cdns_spi_reader(xspi: *mut cdns_spi) {
    static inline void cdns_spi_reader(struct cdns_spi *xspi)
    {
    let mut rxw: u32 = 0;
    if (xspi.rxbuf && !IS_ALIGNED((uintptr_t)xspi.rxbuf, xspi.n_bytes)) {
    pr_err("%s: rxbuf address is not aligned for %d bytes\n",
    __func__, xspi.n_bytes);
    return;
    }
    rxw = cdns_spi_read(xspi, CDNS_SPI_RXD);
    if (xspi.rxbuf) {
    switch (xspi.n_bytes) {
    case CDNS_SPI_N_BYTES_U8:
// (u8 *)xspi->rxbuf = rxw;
    break;
    case CDNS_SPI_N_BYTES_U16:
// (u16 *)xspi->rxbuf = rxw;
    break;
    case CDNS_SPI_N_BYTES_U32:
// (u32 *)xspi->rxbuf = rxw;
    break;
    default:
    pr_err("%s invalid n_bytes %d\n", __func__,
    xspi.n_bytes);
    return;
    }
    xspi.rxbuf = (u8 *)xspi.rxbuf + xspi.n_bytes;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn cdns_spi_writer(xspi: *mut cdns_spi) {
    static inline void cdns_spi_writer(struct cdns_spi *xspi)
    {
    let mut txw: u32 = 0;
    if (xspi.txbuf && !IS_ALIGNED((uintptr_t)xspi.txbuf, xspi.n_bytes)) {
    pr_err("%s: txbuf address is not aligned for %d bytes\n",
    __func__, xspi.n_bytes);
    return;
    }
    if (xspi.txbuf) {
    switch (xspi.n_bytes) {
    case CDNS_SPI_N_BYTES_U8:
    txw = *(u8 *)xspi.txbuf;
    break;
    case CDNS_SPI_N_BYTES_U16:
    txw = *(u16 *)xspi.txbuf;
    break;
    case CDNS_SPI_N_BYTES_U32:
    txw = *(u32 *)xspi.txbuf;
    break;
    default:
    pr_err("%s invalid n_bytes %d\n", __func__,
    xspi.n_bytes);
    return;
    }
    cdns_spi_write(xspi, CDNS_SPI_TXD, txw);
    xspi.txbuf = (u8 *)xspi.txbuf + xspi.n_bytes;
    }
    }
//
// cdns_spi_process_fifo - Fills the TX FIFO, and drain the RX FIFO
// @ctlr:	Pointer to the spi_controller structure
// @xspi:	Pointer to the cdns_spi structure
// @ntx:	Number of bytes to pack into the TX FIFO
// @nrx:	Number of bytes to drain from the RX FIFO
//
    static void cdns_spi_process_fifo(struct spi_controller *ctlr,
    struct cdns_spi *xspi, int ntx, int nrx)
    {
    ntx = clamp(ntx, 0, xspi.tx_bytes);
    nrx = clamp(nrx, 0, xspi.rx_bytes);
    xspi.tx_bytes -= ntx;
    xspi.rx_bytes -= nrx;
    while (ntx || nrx) {
    if (nrx) {
    cdns_spi_reader(xspi);
    nrx--;
    }
    if (ntx) {
// When xspi in busy condition, bytes may send failed,
// then spi control didn't work thoroughly, add one byte
// delay. Only in host mode; in target mode this delay
// causes data corruption as the target fails to prepare
// data in time.
//
    if (!spi_controller_is_target(ctlr) &&
    (cdns_spi_read(xspi, CDNS_SPI_ISR) & CDNS_SPI_IXR_TXFULL))
    udelay(10);
    cdns_spi_writer(xspi);
    ntx--;
    }
    }
    }
//
// cdns_spi_irq - Interrupt service routine of the SPI controller
// @irq:	IRQ number
// @dev_id:	Pointer to the xspi structure
//
// This function handles TX empty and Mode Fault interrupts only.
// On TX empty interrupt this function reads the received data from RX FIFO and
// fills the TX FIFO if there is any data remaining to be transferred.
// On Mode Fault interrupt this function indicates that transfer is completed,
// the SPI subsystem will identify the error as the remaining bytes to be
// transferred is non-zero.
//
// Return:	IRQ_HANDLED when handled; IRQ_NONE otherwise.
//
#[no_mangle]
unsafe extern "C" fn cdns_spi_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t cdns_spi_irq(int irq, void *dev_id)
    {
    struct spi_controller *ctlr = dev_id;
    struct cdns_spi *xspi = spi_controller_get_devdata(ctlr);
    irqreturn_t status;
    u32 intr_status;
    status = IRQ_NONE;
    intr_status = cdns_spi_read(xspi, CDNS_SPI_ISR);
    cdns_spi_write(xspi, CDNS_SPI_ISR, intr_status);
    if (intr_status & CDNS_SPI_IXR_MODF) {
// Indicate that transfer is completed, the SPI subsystem will
// identify the error as the remaining bytes to be
// transferred is non-zero
//
    cdns_spi_write(xspi, CDNS_SPI_IDR, CDNS_SPI_IXR_DEFAULT);
    spi_finalize_current_transfer(ctlr);
    status = IRQ_HANDLED;
    } else if (intr_status & CDNS_SPI_IXR_TXOW) {
    let mut threshold: c_int = cdns_spi_read(xspi, CDNS_SPI_THLD);
    let mut trans_cnt: c_int = xspi.rx_bytes - xspi.tx_bytes;
    if (threshold > 1)
    trans_cnt -= threshold;
// Set threshold to one if number of pending are
// less than half fifo
//
    if (xspi.tx_bytes < xspi.tx_fifo_depth >> 1)
    cdns_spi_write(xspi, CDNS_SPI_THLD, 1);
    if (xspi.tx_bytes) {
    cdns_spi_process_fifo(ctlr, xspi, trans_cnt, trans_cnt);
    } else {
// Fixed delay due to controller limitation with
// RX_NEMPTY incorrect status
// Xilinx AR:65885 contains more details
//
    udelay(10);
    cdns_spi_process_fifo(ctlr, xspi, 0, trans_cnt);
    cdns_spi_write(xspi, CDNS_SPI_IDR,
    CDNS_SPI_IXR_DEFAULT);
    spi_finalize_current_transfer(ctlr);
    }
    status = IRQ_HANDLED;
    }
    return status;
    }
    static int cdns_prepare_message(struct spi_controller *ctlr,
    struct spi_message *msg)
    {
    if (!spi_controller_is_target(ctlr))
    cdns_spi_config_clock_mode(msg.spi);
    return 0;
    }
//
// cdns_transfer_one - Initiates the SPI transfer
// @ctlr:	Pointer to spi_controller structure
// @spi:	Pointer to the spi_device structure
// @transfer:	Pointer to the spi_transfer structure which provides
// information about next transfer parameters
//
// This function in host mode fills the TX FIFO, starts the SPI transfer and
// returns a positive transfer count so that core will wait for completion.
// This function in target mode fills the TX FIFO and wait for transfer trigger.
//
// Return:	Number of bytes transferred in the last transfer
//
    static int cdns_transfer_one(struct spi_controller *ctlr,
    struct spi_device *spi,
    struct spi_transfer *transfer)
    {
    struct cdns_spi *xspi = spi_controller_get_devdata(ctlr);
    xspi.txbuf = transfer.tx_buf;
    xspi.rxbuf = transfer.rx_buf;
    xspi.tx_bytes = transfer.len;
    xspi.rx_bytes = transfer.len;
    if (!spi_controller_is_target(ctlr)) {
    cdns_spi_setup_transfer(spi, transfer);
    } else {
// Set TX empty threshold to half of FIFO depth
// only if TX bytes are more than FIFO depth.
//
    if (xspi.tx_bytes > xspi.tx_fifo_depth)
    cdns_spi_write(xspi, CDNS_SPI_THLD, xspi.tx_fifo_depth >> 1);
    }
    xspi.n_bytes = cdns_spi_n_bytes(transfer);
    xspi.tx_bytes = DIV_ROUND_UP(xspi.tx_bytes, xspi.n_bytes);
    xspi.rx_bytes = DIV_ROUND_UP(xspi.rx_bytes, xspi.n_bytes);
    cdns_spi_process_fifo(ctlr, xspi, xspi.tx_fifo_depth, 0);
    cdns_spi_write(xspi, CDNS_SPI_IER, CDNS_SPI_IXR_DEFAULT);
    return transfer.len;
    }
//
// cdns_prepare_transfer_hardware - Prepares hardware for transfer.
// @ctlr:	Pointer to the spi_controller structure which provides
// information about the controller.
//
// This function enables SPI host controller.
//
// Return:	0 always
//
#[no_mangle]
unsafe extern "C" fn cdns_prepare_transfer_hardware(ctlr: *mut spi_controller) -> c_int {
    static int cdns_prepare_transfer_hardware(struct spi_controller *ctlr)
    {
    struct cdns_spi *xspi = spi_controller_get_devdata(ctlr);
    cdns_spi_write(xspi, CDNS_SPI_ER, CDNS_SPI_ER_ENABLE);
    return 0;
    }
//
// cdns_unprepare_transfer_hardware - Relaxes hardware after transfer
// @ctlr:	Pointer to the spi_controller structure which provides
// information about the controller.
//
// This function disables the SPI host controller when no target selected.
// This function flush out if any pending data in FIFO.
//
// Return:	0 always
//
#[no_mangle]
unsafe extern "C" fn cdns_unprepare_transfer_hardware(ctlr: *mut spi_controller) -> c_int {
    static int cdns_unprepare_transfer_hardware(struct spi_controller *ctlr)
    {
    struct cdns_spi *xspi = spi_controller_get_devdata(ctlr);
    u32 ctrl_reg;
    let mut cnt: c_uint = xspi.tx_fifo_depth;
    if (spi_controller_is_target(ctlr)) {
    while (cnt--)
    cdns_spi_read(xspi, CDNS_SPI_RXD);
    }
// Disable the SPI if target is deselected
    ctrl_reg = cdns_spi_read(xspi, CDNS_SPI_CR);
    ctrl_reg = (ctrl_reg & CDNS_SPI_CR_SSCTRL) >>  CDNS_SPI_SS_SHIFT;
    if (ctrl_reg == CDNS_SPI_NOSS || spi_controller_is_target(ctlr))
    cdns_spi_write(xspi, CDNS_SPI_ER, CDNS_SPI_ER_DISABLE);
// Reset to default
    cdns_spi_write(xspi, CDNS_SPI_THLD, 0x1);
    return 0;
    }
//
// cdns_spi_detect_fifo_depth - Detect the FIFO depth of the hardware
// @xspi:	Pointer to the cdns_spi structure
//
// The depth of the TX FIFO is a synthesis configuration parameter of the SPI
// IP. The FIFO threshold register is sized so that its maximum value can be the
// FIFO size - 1. This is used to detect the size of the FIFO.
//
#[no_mangle]
unsafe extern "C" fn cdns_spi_detect_fifo_depth(xspi: *mut cdns_spi) {
    static void cdns_spi_detect_fifo_depth(struct cdns_spi *xspi)
    {
// The MSBs will get truncated giving us the size of the FIFO
    cdns_spi_write(xspi, CDNS_SPI_THLD, 0xffff);
    xspi.tx_fifo_depth = cdns_spi_read(xspi, CDNS_SPI_THLD) + 1;
// Reset to default
    cdns_spi_write(xspi, CDNS_SPI_THLD, 0x1);
    }
//
// cdns_target_abort - Abort target transfer
// @ctlr:	Pointer to the spi_controller structure
//
// This function abort target transfer if there any transfer timeout.
//
// Return:      0 always
//
#[no_mangle]
unsafe extern "C" fn cdns_target_abort(ctlr: *mut spi_controller) -> c_int {
    static int cdns_target_abort(struct spi_controller *ctlr)
    {
    struct cdns_spi *xspi = spi_controller_get_devdata(ctlr);
    u32 intr_status;
    intr_status = cdns_spi_read(xspi, CDNS_SPI_ISR);
    cdns_spi_write(xspi, CDNS_SPI_ISR, intr_status);
    cdns_spi_write(xspi, CDNS_SPI_IDR, (CDNS_SPI_IXR_MODF | CDNS_SPI_IXR_RXNEMTY));
    spi_finalize_current_transfer(ctlr);
    return 0;
    }
//
// cdns_spi_probe - Probe method for the SPI driver
// @pdev:	Pointer to the platform_device structure
//
// This function initializes the driver data structures and the hardware.
//
// Return:	0 on success and error value on error
//
#[no_mangle]
unsafe extern "C" fn cdns_spi_probe(pdev: *mut platform_device) -> c_int {
    static int cdns_spi_probe(struct platform_device *pdev)
    {
    int ret, irq;
    struct spi_controller *ctlr;
    struct cdns_spi *xspi;
    u32 num_cs;
    bool target;
    target = of_property_read_bool(pdev.dev.of_node, "spi-slave");
    if (target)
    ctlr = devm_spi_alloc_target(&pdev.dev, sizeof(*xspi));
    else
    ctlr = devm_spi_alloc_host(&pdev.dev, sizeof(*xspi));
    if (!ctlr)
    return -ENOMEM;
    xspi = spi_controller_get_devdata(ctlr);
    platform_set_drvdata(pdev, ctlr);
    xspi.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(xspi.regs))
    return PTR_ERR(xspi.regs);
    xspi.pclk = devm_clk_get_enabled(&pdev.dev, "pclk");
    if (IS_ERR(xspi.pclk)) {
    dev_err(&pdev.dev, "pclk clock not found.\n");
    return PTR_ERR(xspi.pclk);
    }
    xspi.rstc = devm_reset_control_get_optional_exclusive(&pdev.dev, "spi");
    if (IS_ERR(xspi.rstc)) {
    return dev_err_probe(&pdev.dev, PTR_ERR(xspi.rstc),
    "Cannot get SPI reset.\n");
    }
    reset_control_assert(xspi.rstc);
    reset_control_deassert(xspi.rstc);
    xspi.ref_clk = devm_clk_get_enabled(&pdev.dev, "ref_clk");
    if (IS_ERR(xspi.ref_clk)) {
    dev_err(&pdev.dev, "ref_clk clock not found.\n");
    return PTR_ERR(xspi.ref_clk);
    }
    if (!spi_controller_is_target(ctlr)) {
    pm_runtime_use_autosuspend(&pdev.dev);
    pm_runtime_set_autosuspend_delay(&pdev.dev, SPI_AUTOSUSPEND_TIMEOUT);
    pm_runtime_get_noresume(&pdev.dev);
    pm_runtime_set_active(&pdev.dev);
    pm_runtime_enable(&pdev.dev);
    ret = of_property_read_u32(pdev.dev.of_node, "num-cs", &num_cs);
    if (ret < 0)
    ctlr.num_chipselect = CDNS_SPI_DEFAULT_NUM_CS;
    else
    ctlr.num_chipselect = num_cs;
    ret = of_property_read_u32(pdev.dev.of_node, "is-decoded-cs",
    &xspi.is_decoded_cs);
    if (ret < 0)
    xspi.is_decoded_cs = 0;
    }
    cdns_spi_detect_fifo_depth(xspi);
// SPI controller initializations
    cdns_spi_init_hw(xspi, spi_controller_is_target(ctlr));
    irq = platform_get_irq(pdev, 0);
    if (irq < 0) {
    ret = irq;
    goto err_disable_rpm;
    }
    ret = devm_request_irq(&pdev.dev, irq, cdns_spi_irq,
    0, pdev.name, ctlr);
    if (ret != 0) {
    ret = -ENXIO;
    dev_err(&pdev.dev, "request_irq failed\n");
    goto err_disable_rpm;
    }
    ctlr.use_gpio_descriptors = true;
    ctlr.prepare_transfer_hardware = cdns_prepare_transfer_hardware;
    ctlr.prepare_message = cdns_prepare_message;
    ctlr.transfer_one = cdns_transfer_one;
    ctlr.unprepare_transfer_hardware = cdns_unprepare_transfer_hardware;
    ctlr.mode_bits = SPI_CPOL | SPI_CPHA;
    ctlr.bits_per_word_mask = SPI_BPW_MASK(8);
    ctlr.flags = SPI_CONTROLLER_MUST_TX;
    if (of_device_is_compatible(pdev.dev.of_node, "cix,sky1-spi-r1p6"))
    ctlr.bits_per_word_mask |= SPI_BPW_MASK(16) | SPI_BPW_MASK(32);
    if (!spi_controller_is_target(ctlr)) {
    ctlr.mode_bits |=  SPI_CS_HIGH;
    ctlr.set_cs = cdns_spi_chipselect;
    ctlr.auto_runtime_pm = true;
    xspi.clk_rate = clk_get_rate(xspi.ref_clk);
// Set to default valid value
    ctlr.max_speed_hz = xspi.clk_rate / 4;
    xspi.speed_hz = ctlr.max_speed_hz;
    } else {
    ctlr.mode_bits |= SPI_NO_CS;
    ctlr.target_abort = cdns_target_abort;
    }
    ret = spi_register_controller(ctlr);
    if (ret) {
    dev_err(&pdev.dev, "spi_register_controller failed\n");
    goto err_disable_rpm;
    }
    if (!spi_controller_is_target(ctlr))
    pm_runtime_put_autosuspend(&pdev.dev);
    return 0;
    err_disable_rpm:
    if (!spi_controller_is_target(ctlr)) {
    pm_runtime_disable(&pdev.dev);
    pm_runtime_set_suspended(&pdev.dev);
    pm_runtime_put_noidle(&pdev.dev);
    pm_runtime_dont_use_autosuspend(&pdev.dev);
    }
    return ret;
    }
//
// cdns_spi_remove - Remove method for the SPI driver
// @pdev:	Pointer to the platform_device structure
//
// This function is called if a device is physically removed from the system or
// if the driver module is being unloaded. It frees all resources allocated to
// the device.
//
#[no_mangle]
unsafe extern "C" fn cdns_spi_remove(pdev: *mut platform_device) {
    static void cdns_spi_remove(struct platform_device *pdev)
    {
    struct spi_controller *ctlr = platform_get_drvdata(pdev);
    struct cdns_spi *xspi = spi_controller_get_devdata(ctlr);
    let mut ret: c_int = 0;
    if (!spi_controller_is_target(ctlr))
    ret = pm_runtime_get_sync(&pdev.dev);
    spi_unregister_controller(ctlr);
    if (ret >= 0)
    cdns_spi_write(xspi, CDNS_SPI_ER, CDNS_SPI_ER_DISABLE);
    if (!spi_controller_is_target(ctlr)) {
    pm_runtime_disable(&pdev.dev);
    pm_runtime_set_suspended(&pdev.dev);
    pm_runtime_put_noidle(&pdev.dev);
    pm_runtime_dont_use_autosuspend(&pdev.dev);
    }
    }
//
// cdns_spi_suspend - Suspend method for the SPI driver
// @dev:	Address of the platform_device structure
//
// This function disables the SPI controller and
// changes the driver state to "suspend"
//
// Return:	0 on success and error value on error
//
#[no_mangle]
unsafe extern "C" fn cdns_spi_suspend(dev: *mut device) -> c_int {
    static int cdns_spi_suspend(struct device *dev)
    {
    struct spi_controller *ctlr = dev_get_drvdata(dev);
    return spi_controller_suspend(ctlr);
    }
//
// cdns_spi_resume - Resume method for the SPI driver
// @dev:	Address of the platform_device structure
//
// This function changes the driver state to "ready"
//
// Return:	0 on success and error value on error
//
#[no_mangle]
unsafe extern "C" fn cdns_spi_resume(dev: *mut device) -> c_int {
    static int cdns_spi_resume(struct device *dev)
    {
    struct spi_controller *ctlr = dev_get_drvdata(dev);
    struct cdns_spi *xspi = spi_controller_get_devdata(ctlr);
    cdns_spi_init_hw(xspi, spi_controller_is_target(ctlr));
    return spi_controller_resume(ctlr);
    }
//
// cdns_spi_runtime_resume - Runtime resume method for the SPI driver
// @dev:	Address of the platform_device structure
//
// This function enables the clocks
//
// Return:	0 on success and error value on error
//
#[no_mangle]
unsafe extern "C" fn cdns_spi_runtime_resume(dev: *mut device) -> c_int {
    static int cdns_spi_runtime_resume(struct device *dev)
    {
    struct spi_controller *ctlr = dev_get_drvdata(dev);
    struct cdns_spi *xspi = spi_controller_get_devdata(ctlr);
    int ret;
    ret = clk_prepare_enable(xspi.pclk);
    if (ret) {
    dev_err(dev, "Cannot enable APB clock.\n");
    return ret;
    }
    ret = clk_prepare_enable(xspi.ref_clk);
    if (ret) {
    dev_err(dev, "Cannot enable device clock.\n");
    clk_disable_unprepare(xspi.pclk);
    return ret;
    }
    return 0;
    }
//
// cdns_spi_runtime_suspend - Runtime suspend method for the SPI driver
// @dev:	Address of the platform_device structure
//
// This function disables the clocks
//
// Return:	Always 0
//
#[no_mangle]
unsafe extern "C" fn cdns_spi_runtime_suspend(dev: *mut device) -> c_int {
    static int cdns_spi_runtime_suspend(struct device *dev)
    {
    struct spi_controller *ctlr = dev_get_drvdata(dev);
    struct cdns_spi *xspi = spi_controller_get_devdata(ctlr);
    clk_disable_unprepare(xspi.ref_clk);
    clk_disable_unprepare(xspi.pclk);
    return 0;
    }
    static const struct dev_pm_ops cdns_spi_dev_pm_ops = {
    RUNTIME_PM_OPS(cdns_spi_runtime_suspend, cdns_spi_runtime_resume, core::ptr::null_mut())
    SYSTEM_SLEEP_PM_OPS(cdns_spi_suspend, cdns_spi_resume)
    };
    static const struct of_device_id cdns_spi_of_match[] = {
    { .compatible = "xlnx,zynq-spi-r1p6" },
    { .compatible = "cix,sky1-spi-r1p6" },
    { .compatible = "cdns,spi-r1p6" },
    { /* end of table */ }
    };
    MODULE_DEVICE_TABLE(of, cdns_spi_of_match);
// cdns_spi_driver - This structure defines the SPI subsystem platform driver
    static struct platform_driver cdns_spi_driver = {
    .probe	= cdns_spi_probe,
    .remove = cdns_spi_remove,
    .driver = {
    .name = CDNS_SPI_NAME,
    .of_match_table = cdns_spi_of_match,
    .pm = pm_ptr(&cdns_spi_dev_pm_ops),
    },
    };
    module_platform_driver(cdns_spi_driver);
    MODULE_AUTHOR("Xilinx, Inc.");
    MODULE_DESCRIPTION("Cadence SPI driver");
    MODULE_LICENSE("GPL");
