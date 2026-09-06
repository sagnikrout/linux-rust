//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-zynq-qspi.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2019 Xilinx, Inc.
//
// Author: Naga Sureshkumar Relli <nagasure@xilinx.com>
//

// Register offset definitions
pub const ZYNQ_QSPI_CONFIG_OFFSET: c_uint = 0x00 /* Configuration  Register, RW */;
pub const ZYNQ_QSPI_STATUS_OFFSET: c_uint = 0x04 /* Interrupt Status Register, RO */;
pub const ZYNQ_QSPI_IEN_OFFSET: c_uint = 0x08 /* Interrupt Enable Register, WO */;
pub const ZYNQ_QSPI_IDIS_OFFSET: c_uint = 0x0C /* Interrupt Disable Reg, WO */;
pub const ZYNQ_QSPI_IMASK_OFFSET: c_uint = 0x10 /* Interrupt Enabled Mask Reg,RO */;
pub const ZYNQ_QSPI_ENABLE_OFFSET: c_uint = 0x14 /* Enable/Disable Register, RW */;
pub const ZYNQ_QSPI_DELAY_OFFSET: c_uint = 0x18 /* Delay Register, RW */;
pub const ZYNQ_QSPI_TXD_00_00_OFFSET: c_uint = 0x1C /* Transmit 4-byte inst, WO */;
pub const ZYNQ_QSPI_TXD_00_01_OFFSET: c_uint = 0x80 /* Transmit 1-byte inst, WO */;
pub const ZYNQ_QSPI_TXD_00_10_OFFSET: c_uint = 0x84 /* Transmit 2-byte inst, WO */;
pub const ZYNQ_QSPI_TXD_00_11_OFFSET: c_uint = 0x88 /* Transmit 3-byte inst, WO */;
pub const ZYNQ_QSPI_RXD_OFFSET: c_uint = 0x20 /* Data Receive Register, RO */;
pub const ZYNQ_QSPI_SIC_OFFSET: c_uint = 0x24 /* Slave Idle Count Register, RW */;
pub const ZYNQ_QSPI_TX_THRESH_OFFSET: c_uint = 0x28 /* TX FIFO Watermark Reg, RW */;
pub const ZYNQ_QSPI_RX_THRESH_OFFSET: c_uint = 0x2C /* RX FIFO Watermark Reg, RW */;
pub const ZYNQ_QSPI_GPIO_OFFSET: c_uint = 0x30 /* GPIO Register, RW */;
pub const ZYNQ_QSPI_LINEAR_CFG_OFFSET: c_uint = 0xA0 /* Linear Adapter Config Ref, RW */;
pub const ZYNQ_QSPI_MOD_ID_OFFSET: c_uint = 0xFC /* Module ID Register, RO */;
//
// QSPI Configuration Register bit Masks
//
// This register contains various control bits that effect the operation
// of the QSPI controller
//

//
// QSPI Configuration Register - Baud rate and target select
//
// These are the values used in the calculation of baud rate divisor and
// setting the target select.
//

//
// QSPI Interrupt Registers bit Masks
//
// All the four interrupt registers (Status/Mask/Enable/Disable) have the same
// bit definitions.
//

    ZYNQ_QSPI_IXR_TXNFULL_MASK | \
    ZYNQ_QSPI_IXR_TXFULL_MASK | \
    ZYNQ_QSPI_IXR_RXNEMTY_MASK | \
    ZYNQ_QSPI_IXR_RXF_FULL_MASK | \
    ZYNQ_QSPI_IXR_TXF_UNDRFLOW_MASK)

    ZYNQ_QSPI_IXR_RXNEMTY_MASK)
//
// QSPI Enable Register bit Masks
//
// This register is used to enable or disable the QSPI controller
//

//
// QSPI Linear Configuration Register
//
// It is named Linear Configuration but it controls other modes when not in
// linear mode also.
//

pub const ZYNQ_QSPI_LCFG_DUMMY_SHIFT: c_int = 8;
pub const ZYNQ_QSPI_FAST_READ_QOUT_CODE: c_uint = 0x6B /* read instruction code */;

//
// The modebits configurable by the driver to make the SPI support different
// data formats
//

// Maximum number of chip selects
pub const ZYNQ_QSPI_MAX_NUM_CS: c_int = 2;
//
// struct zynq_qspi - Defines qspi driver instance
// @dev:		Pointer to the this device's information
// @regs:		Virtual address of the QSPI controller registers
// @refclk:		Pointer to the peripheral clock
// @pclk:		Pointer to the APB clock
// @irq:		IRQ number
// @txbuf:		Pointer to the TX buffer
// @rxbuf:		Pointer to the RX buffer
// @tx_bytes:		Number of bytes left to transfer
// @rx_bytes:		Number of bytes left to receive
// @data_completion:	completion structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zynq_qspi {
    pub dev: *mut device,
    pub regs: *mut void __iomem,
    pub refclk: *mut clk,
    pub pclk: *mut clk,
    pub irq: c_int,
    pub txbuf: *mut u8,
    pub rxbuf: *mut u8,
    pub tx_bytes: c_int,
    pub rx_bytes: c_int,
    pub data_completion: completion,
}

//
// Inline functions for the QSPI controller read/write
//
#[no_mangle]
pub unsafe extern "C" fn zynq_qspi_read(xqspi: *mut zynq_qspi, offset: u32) -> u32 {
    static inline u32 zynq_qspi_read(struct zynq_qspi *xqspi, u32 offset)
    {
    return readl_relaxed(xqspi.regs + offset);
    }
    static inline void zynq_qspi_write(struct zynq_qspi *xqspi, u32 offset,
    u32 val)
    {
    writel_relaxed(val, xqspi.regs + offset);
    }
//
// zynq_qspi_init_hw - Initialize the hardware
// @xqspi:	Pointer to the zynq_qspi structure
// @num_cs:	Number of connected CS (to enable dual memories if needed)
//
// The default settings of the QSPI controller's configurable parameters on
// reset are
// - Host mode
// - Baud rate divisor is set to 2
// - Tx threshold set to 1l Rx threshold set to 32
// - Flash memory interface mode enabled
// - Size of the word to be transferred as 8 bit
// This function performs the following actions
// - Disable and clear all the interrupts
// - Enable manual target select
// - Enable manual start
// - Deselect all the chip select lines
// - Set the size of the word to be transferred as 32 bit
// - Set the little endian mode of TX FIFO and
// - Enable the QSPI controller
//
#[no_mangle]
unsafe extern "C" fn zynq_qspi_init_hw(xqspi: *mut zynq_qspi, num_cs: c_uint) {
    static void zynq_qspi_init_hw(struct zynq_qspi *xqspi, unsigned int num_cs)
    {
    u32 config_reg;
    zynq_qspi_write(xqspi, ZYNQ_QSPI_ENABLE_OFFSET, 0);
    zynq_qspi_write(xqspi, ZYNQ_QSPI_IDIS_OFFSET, ZYNQ_QSPI_IXR_ALL_MASK);
// Disable linear mode as the boot loader may have used it
    config_reg = 0;
// At the same time, enable dual mode if more than 1 CS is available
    if (num_cs > 1)
    config_reg |= ZYNQ_QSPI_LCFG_TWO_MEM;
    zynq_qspi_write(xqspi, ZYNQ_QSPI_LINEAR_CFG_OFFSET, config_reg);
// Clear the RX FIFO
    while (zynq_qspi_read(xqspi, ZYNQ_QSPI_STATUS_OFFSET) &
    ZYNQ_QSPI_IXR_RXNEMTY_MASK)
    zynq_qspi_read(xqspi, ZYNQ_QSPI_RXD_OFFSET);
    zynq_qspi_write(xqspi, ZYNQ_QSPI_STATUS_OFFSET, ZYNQ_QSPI_IXR_ALL_MASK);
    config_reg = zynq_qspi_read(xqspi, ZYNQ_QSPI_CONFIG_OFFSET);
    config_reg &= ~(ZYNQ_QSPI_CONFIG_MSTREN_MASK |
    ZYNQ_QSPI_CONFIG_CPOL_MASK |
    ZYNQ_QSPI_CONFIG_CPHA_MASK |
    ZYNQ_QSPI_CONFIG_BDRATE_MASK |
    ZYNQ_QSPI_CONFIG_SSFORCE_MASK |
    ZYNQ_QSPI_CONFIG_MANSRTEN_MASK |
    ZYNQ_QSPI_CONFIG_MANSRT_MASK);
    config_reg |= (ZYNQ_QSPI_CONFIG_MSTREN_MASK |
    ZYNQ_QSPI_CONFIG_SSFORCE_MASK |
    ZYNQ_QSPI_CONFIG_FWIDTH_MASK |
    ZYNQ_QSPI_CONFIG_IFMODE_MASK);
    zynq_qspi_write(xqspi, ZYNQ_QSPI_CONFIG_OFFSET, config_reg);
    zynq_qspi_write(xqspi, ZYNQ_QSPI_RX_THRESH_OFFSET,
    ZYNQ_QSPI_RX_THRESHOLD);
    zynq_qspi_write(xqspi, ZYNQ_QSPI_TX_THRESH_OFFSET,
    ZYNQ_QSPI_TX_THRESHOLD);
    zynq_qspi_write(xqspi, ZYNQ_QSPI_ENABLE_OFFSET,
    ZYNQ_QSPI_ENABLE_ENABLE_MASK);
    }
    static bool zynq_qspi_supports_op(struct spi_mem *mem,
    const struct spi_mem_op *op)
    {
    if (!spi_mem_default_supports_op(mem, op))
    return false;
//
// The number of address bytes should be equal to or less than 3 bytes.
//
    if (op.addr.nbytes > 3)
    return false;
    return true;
    }
//
// zynq_qspi_rxfifo_op - Read 1..4 bytes from RxFIFO to RX buffer
// @xqspi:	Pointer to the zynq_qspi structure
// @size:	Number of bytes to be read (1..4)
//
#[no_mangle]
unsafe extern "C" fn zynq_qspi_rxfifo_op(xqspi: *mut zynq_qspi, size: c_uint) {
    static void zynq_qspi_rxfifo_op(struct zynq_qspi *xqspi, unsigned int size)
    {
    u32 data;
    data = zynq_qspi_read(xqspi, ZYNQ_QSPI_RXD_OFFSET);
    if (xqspi.rxbuf) {
    memcpy(xqspi.rxbuf, ((u8 *)&data) + 4 - size, size);
    xqspi.rxbuf += size;
    }
    xqspi.rx_bytes -= size;
    if (xqspi.rx_bytes < 0)
    xqspi.rx_bytes = 0;
    }
//
// zynq_qspi_txfifo_op - Write 1..4 bytes from TX buffer to TxFIFO
// @xqspi:	Pointer to the zynq_qspi structure
// @size:	Number of bytes to be written (1..4)
//
#[no_mangle]
unsafe extern "C" fn zynq_qspi_txfifo_op(xqspi: *mut zynq_qspi, size: c_uint) {
    static void zynq_qspi_txfifo_op(struct zynq_qspi *xqspi, unsigned int size)
    {
    static const unsigned int offset[4] = {
    ZYNQ_QSPI_TXD_00_01_OFFSET, ZYNQ_QSPI_TXD_00_10_OFFSET,
    ZYNQ_QSPI_TXD_00_11_OFFSET, ZYNQ_QSPI_TXD_00_00_OFFSET };
    u32 data;
    if (xqspi.txbuf) {
    data = 0xffffffff;
    memcpy(&data, xqspi.txbuf, size);
    xqspi.txbuf += size;
    } else {
    data = 0;
    }
    xqspi.tx_bytes -= size;
    zynq_qspi_write(xqspi, offset[size - 1], data);
    }
//
// zynq_qspi_chipselect - Select or deselect the chip select line
// @spi:	Pointer to the spi_device structure
// @assert:	1 for select or 0 for deselect the chip select line
//
#[no_mangle]
unsafe extern "C" fn zynq_qspi_chipselect(spi: *mut spi_device, assert: bool) {
    static void zynq_qspi_chipselect(struct spi_device *spi, bool assert)
    {
    struct spi_controller *ctlr = spi.controller;
    struct zynq_qspi *xqspi = spi_controller_get_devdata(ctlr);
    u32 config_reg;
// Select the lower (CS0) or upper (CS1) memory
    if (ctlr.num_chipselect > 1) {
    config_reg = zynq_qspi_read(xqspi, ZYNQ_QSPI_LINEAR_CFG_OFFSET);
    if (!spi_get_chipselect(spi, 0))
    config_reg &= ~ZYNQ_QSPI_LCFG_U_PAGE;
    else
    config_reg |= ZYNQ_QSPI_LCFG_U_PAGE;
    zynq_qspi_write(xqspi, ZYNQ_QSPI_LINEAR_CFG_OFFSET, config_reg);
    }
// Ground the line to assert the CS
    config_reg = zynq_qspi_read(xqspi, ZYNQ_QSPI_CONFIG_OFFSET);
    if (assert)
    config_reg &= ~ZYNQ_QSPI_CONFIG_PCS;
    else
    config_reg |= ZYNQ_QSPI_CONFIG_PCS;
    zynq_qspi_write(xqspi, ZYNQ_QSPI_CONFIG_OFFSET, config_reg);
    }
//
// zynq_qspi_config_op - Configure QSPI controller for specified transfer
// @xqspi:	Pointer to the zynq_qspi structure
// @spi:	Pointer to the spi_device structure
// @op:		The memory operation to execute
//
// Sets the operational mode of QSPI controller for the next QSPI transfer and
// sets the requested clock frequency.
//
// Return:	0 on success and -EINVAL on invalid input parameter
//
// Note: If the requested frequency is not an exact match with what can be
// obtained using the prescalar value, the driver sets the clock frequency which
// is lower than the requested frequency (maximum lower) for the transfer. If
// the requested frequency is higher or lower than that is supported by the QSPI
// controller the driver will set the highest or lowest frequency supported by
// controller.
//
    static int zynq_qspi_config_op(struct zynq_qspi *xqspi, struct spi_device *spi,
    const struct spi_mem_op *op)
    {
    u32 config_reg, baud_rate_val = 0;
//
// Set the clock frequency
// The baud rate divisor is not a direct mapping to the value written
// into the configuration register (config_reg[5:3])
// i.e. 000 - divide by 2
// 001 - divide by 4
// ----------------
// 111 - divide by 256
//
    while ((baud_rate_val < ZYNQ_QSPI_CONFIG_BAUD_DIV_MAX)  &&
    (clk_get_rate(xqspi.refclk) / (2 << baud_rate_val)) >
    op.max_freq)
    baud_rate_val++;
    config_reg = zynq_qspi_read(xqspi, ZYNQ_QSPI_CONFIG_OFFSET);
// Set the QSPI clock phase and clock polarity
    config_reg &= (~ZYNQ_QSPI_CONFIG_CPHA_MASK) &
    (~ZYNQ_QSPI_CONFIG_CPOL_MASK);
    if (spi.mode & SPI_CPHA)
    config_reg |= ZYNQ_QSPI_CONFIG_CPHA_MASK;
    if (spi.mode & SPI_CPOL)
    config_reg |= ZYNQ_QSPI_CONFIG_CPOL_MASK;
    config_reg &= ~ZYNQ_QSPI_CONFIG_BDRATE_MASK;
    config_reg |= (baud_rate_val << ZYNQ_QSPI_CONFIG_BAUD_DIV_SHIFT);
    zynq_qspi_write(xqspi, ZYNQ_QSPI_CONFIG_OFFSET, config_reg);
    return 0;
    }
//
// zynq_qspi_setup_op - Configure the QSPI controller
// @spi:	Pointer to the spi_device structure
//
// Sets the operational mode of QSPI controller for the next QSPI transfer, baud
// rate and divisor value to setup the requested qspi clock.
//
// Return:	0 on success and error value on failure
//
#[no_mangle]
unsafe extern "C" fn zynq_qspi_setup_op(spi: *mut spi_device) -> c_int {
    static int zynq_qspi_setup_op(struct spi_device *spi)
    {
    struct spi_controller *ctlr = spi.controller;
    struct zynq_qspi *qspi = spi_controller_get_devdata(ctlr);
    if (ctlr.busy)
    return -EBUSY;
    zynq_qspi_write(qspi, ZYNQ_QSPI_ENABLE_OFFSET,
    ZYNQ_QSPI_ENABLE_ENABLE_MASK);
    return 0;
    }
//
// zynq_qspi_write_op - Fills the TX FIFO with as many bytes as possible
// @xqspi:	Pointer to the zynq_qspi structure
// @txcount:	Maximum number of words to write
// @txempty:	Indicates that TxFIFO is empty
//
    static void zynq_qspi_write_op(struct zynq_qspi *xqspi, int txcount,
    bool txempty)
    {
    int count, len, k;
    len = xqspi.tx_bytes;
    if (len && len < 4) {
//
// We must empty the TxFIFO between accesses to TXD0,
// TXD1, TXD2, TXD3.
//
    if (txempty)
    zynq_qspi_txfifo_op(xqspi, len);
    return;
    }
    count = len / 4;
    if (count > txcount)
    count = txcount;
    if (xqspi.txbuf) {
    iowrite32_rep(xqspi.regs + ZYNQ_QSPI_TXD_00_00_OFFSET,
    xqspi.txbuf, count);
    xqspi.txbuf += count * 4;
    } else {
    for (k = 0; k < count; k++)
    writel_relaxed(0, xqspi.regs +
    ZYNQ_QSPI_TXD_00_00_OFFSET);
    }
    xqspi.tx_bytes -= count * 4;
    }
//
// zynq_qspi_read_op - Drains the RX FIFO by as many bytes as possible
// @xqspi:	Pointer to the zynq_qspi structure
// @rxcount:	Maximum number of words to read
//
#[no_mangle]
unsafe extern "C" fn zynq_qspi_read_op(xqspi: *mut zynq_qspi, rxcount: c_int) {
    static void zynq_qspi_read_op(struct zynq_qspi *xqspi, int rxcount)
    {
    int count, len, k;
    len = xqspi.rx_bytes - xqspi.tx_bytes;
    count = len / 4;
    if (count > rxcount)
    count = rxcount;
    if (xqspi.rxbuf) {
    ioread32_rep(xqspi.regs + ZYNQ_QSPI_RXD_OFFSET,
    xqspi.rxbuf, count);
    xqspi.rxbuf += count * 4;
    } else {
    for (k = 0; k < count; k++)
    readl_relaxed(xqspi.regs + ZYNQ_QSPI_RXD_OFFSET);
    }
    xqspi.rx_bytes -= count * 4;
    len -= count * 4;
    if (len && len < 4 && count < rxcount)
    zynq_qspi_rxfifo_op(xqspi, len);
    }
//
// zynq_qspi_irq - Interrupt service routine of the QSPI controller
// @irq:	IRQ number
// @dev_id:	Pointer to the xqspi structure
//
// This function handles TX empty only.
// On TX empty interrupt this function reads the received data from RX FIFO and
// fills the TX FIFO if there is any data remaining to be transferred.
//
// Return:	IRQ_HANDLED when interrupt is handled; IRQ_NONE otherwise.
//
#[no_mangle]
unsafe extern "C" fn zynq_qspi_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t zynq_qspi_irq(int irq, void *dev_id)
    {
    u32 intr_status;
    bool txempty;
    struct zynq_qspi *xqspi = (struct zynq_qspi *)dev_id;
    intr_status = zynq_qspi_read(xqspi, ZYNQ_QSPI_STATUS_OFFSET);
    zynq_qspi_write(xqspi, ZYNQ_QSPI_STATUS_OFFSET, intr_status);
    if ((intr_status & ZYNQ_QSPI_IXR_TXNFULL_MASK) ||
    (intr_status & ZYNQ_QSPI_IXR_RXNEMTY_MASK)) {
//
// This bit is set when Tx FIFO has < THRESHOLD entries.
// We have the THRESHOLD value set to 1,
// so this bit indicates Tx FIFO is empty.
//
    txempty = !!(intr_status & ZYNQ_QSPI_IXR_TXNFULL_MASK);
// Read out the data from the RX FIFO
    zynq_qspi_read_op(xqspi, ZYNQ_QSPI_RX_THRESHOLD);
    if (xqspi.tx_bytes) {
// There is more data to send
    zynq_qspi_write_op(xqspi, ZYNQ_QSPI_RX_THRESHOLD,
    txempty);
    } else {
//
// If transfer and receive is completed then only send
// complete signal.
//
    if (!xqspi.rx_bytes) {
    zynq_qspi_write(xqspi,
    ZYNQ_QSPI_IDIS_OFFSET,
    ZYNQ_QSPI_IXR_RXTX_MASK);
    complete(&xqspi.data_completion);
    }
    }
    return IRQ_HANDLED;
    }
    return IRQ_NONE;
    }
//
// zynq_qspi_exec_mem_op() - Initiates the QSPI transfer
// @mem: the SPI memory
// @op: the memory operation to execute
//
// Executes a memory operation.
//
// This function first selects the chip and starts the memory operation.
//
// Return: 0 in case of success, a negative error code otherwise.
//
    static int zynq_qspi_exec_mem_op(struct spi_mem *mem,
    const struct spi_mem_op *op)
    {
    struct zynq_qspi *xqspi = spi_controller_get_devdata(mem.spi.controller);
    let mut err: c_int = 0, i;
    u8 *tmpbuf;
    zynq_qspi_chipselect(mem.spi, true);
    zynq_qspi_config_op(xqspi, mem.spi, op);
    if (op.cmd.opcode) {
    reinit_completion(&xqspi.data_completion);
    xqspi.txbuf = (u8 *)&op.cmd.opcode;
    xqspi.rxbuf = core::ptr::null_mut();
    xqspi.tx_bytes = op.cmd.nbytes;
    xqspi.rx_bytes = op.cmd.nbytes;
    zynq_qspi_write_op(xqspi, ZYNQ_QSPI_FIFO_DEPTH, true);
    zynq_qspi_write(xqspi, ZYNQ_QSPI_IEN_OFFSET,
    ZYNQ_QSPI_IXR_RXTX_MASK);
    if (!wait_for_completion_timeout(&xqspi.data_completion,
    msecs_to_jiffies(1000)))
    err = -ETIMEDOUT;
    }
    if (op.addr.nbytes) {
    for (i = 0; i < op.addr.nbytes; i++) {
    xqspi.txbuf[i] = op.addr.val >>
    (8 * (op.addr.nbytes - i - 1));
    }
    reinit_completion(&xqspi.data_completion);
    xqspi.rxbuf = core::ptr::null_mut();
    xqspi.tx_bytes = op.addr.nbytes;
    xqspi.rx_bytes = op.addr.nbytes;
    zynq_qspi_write_op(xqspi, ZYNQ_QSPI_FIFO_DEPTH, true);
    zynq_qspi_write(xqspi, ZYNQ_QSPI_IEN_OFFSET,
    ZYNQ_QSPI_IXR_RXTX_MASK);
    if (!wait_for_completion_timeout(&xqspi.data_completion,
    msecs_to_jiffies(1000)))
    err = -ETIMEDOUT;
    }
    if (op.dummy.nbytes) {
    tmpbuf = kmalloc(op.dummy.nbytes, GFP_KERNEL);
    if (!tmpbuf)
    return -ENOMEM;
    memset(tmpbuf, 0xff, op.dummy.nbytes);
    reinit_completion(&xqspi.data_completion);
    xqspi.txbuf = tmpbuf;
    xqspi.rxbuf = core::ptr::null_mut();
    xqspi.tx_bytes = op.dummy.nbytes;
    xqspi.rx_bytes = op.dummy.nbytes;
    zynq_qspi_write_op(xqspi, ZYNQ_QSPI_FIFO_DEPTH, true);
    zynq_qspi_write(xqspi, ZYNQ_QSPI_IEN_OFFSET,
    ZYNQ_QSPI_IXR_RXTX_MASK);
    if (!wait_for_completion_timeout(&xqspi.data_completion,
    msecs_to_jiffies(1000)))
    err = -ETIMEDOUT;
    kfree(tmpbuf);
    }
    if (op.data.nbytes) {
    reinit_completion(&xqspi.data_completion);
    if (op.data.dir == SPI_MEM_DATA_OUT) {
    xqspi.txbuf = (u8 *)op.data.buf.out;
    xqspi.tx_bytes = op.data.nbytes;
    xqspi.rxbuf = core::ptr::null_mut();
    xqspi.rx_bytes = op.data.nbytes;
    } else {
    xqspi.txbuf = core::ptr::null_mut();
    xqspi.rxbuf = (u8 *)op.data.buf.in;
    xqspi.rx_bytes = op.data.nbytes;
    xqspi.tx_bytes = op.data.nbytes;
    }
    zynq_qspi_write_op(xqspi, ZYNQ_QSPI_FIFO_DEPTH, true);
    zynq_qspi_write(xqspi, ZYNQ_QSPI_IEN_OFFSET,
    ZYNQ_QSPI_IXR_RXTX_MASK);
    if (!wait_for_completion_timeout(&xqspi.data_completion,
    msecs_to_jiffies(1000)))
    err = -ETIMEDOUT;
    }
    zynq_qspi_chipselect(mem.spi, false);
    return err;
    }
    static const struct spi_controller_mem_ops zynq_qspi_mem_ops = {
    .supports_op = zynq_qspi_supports_op,
    .exec_op = zynq_qspi_exec_mem_op,
    };
    static const struct spi_controller_mem_caps zynq_qspi_mem_caps = {
    .per_op_freq = true,
    };
//
// zynq_qspi_probe - Probe method for the QSPI driver
// @pdev:	Pointer to the platform_device structure
//
// This function initializes the driver data structures and the hardware.
//
// Return:	0 on success and error value on failure
//
#[no_mangle]
unsafe extern "C" fn zynq_qspi_probe(pdev: *mut platform_device) -> c_int {
    static int zynq_qspi_probe(struct platform_device *pdev)
    {
    let mut ret: c_int = 0;
    struct spi_controller *ctlr;
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct zynq_qspi *xqspi;
    u32 num_cs;
    ctlr = devm_spi_alloc_host(&pdev.dev, sizeof(*xqspi));
    if (!ctlr)
    return -ENOMEM;
    xqspi = spi_controller_get_devdata(ctlr);
    xqspi.dev = dev;
    platform_set_drvdata(pdev, ctlr);
    xqspi.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(xqspi.regs))
    return PTR_ERR(xqspi.regs);
    xqspi.pclk = devm_clk_get_enabled(&pdev.dev, "pclk");
    if (IS_ERR(xqspi.pclk)) {
    dev_err(&pdev.dev, "pclk clock not found.\n");
    return PTR_ERR(xqspi.pclk);
    }
    init_completion(&xqspi.data_completion);
    xqspi.refclk = devm_clk_get_enabled(&pdev.dev, "ref_clk");
    if (IS_ERR(xqspi.refclk)) {
    dev_err(&pdev.dev, "ref_clk clock not found.\n");
    return PTR_ERR(xqspi.refclk);
    }
    xqspi.irq = platform_get_irq(pdev, 0);
    if (xqspi.irq < 0)
    return xqspi.irq;
    ret = devm_request_irq(&pdev.dev, xqspi.irq, zynq_qspi_irq,
    0, pdev.name, xqspi);
    if (ret != 0) {
    dev_err(&pdev.dev, "request_irq failed\n");
    return -ENXIO;
    }
    ret = of_property_read_u32(np, "num-cs",
    &num_cs);
    if (ret < 0) {
    ctlr.num_chipselect = 1;
    } else if (num_cs > ZYNQ_QSPI_MAX_NUM_CS) {
    dev_err(&pdev.dev, "only 2 chip selects are available\n");
    return -EINVAL;
    } else {
    ctlr.num_chipselect = num_cs;
    }
    ctlr.mode_bits =  SPI_RX_DUAL | SPI_RX_QUAD |
    SPI_TX_DUAL | SPI_TX_QUAD;
    ctlr.mem_ops = &zynq_qspi_mem_ops;
    ctlr.mem_caps = &zynq_qspi_mem_caps;
    ctlr.setup = zynq_qspi_setup_op;
    ctlr.max_speed_hz = clk_get_rate(xqspi.refclk) / 2;
    ctlr.dev.of_node = np;
// QSPI controller initializations
    zynq_qspi_init_hw(xqspi, ctlr.num_chipselect);
    ret = spi_register_controller(ctlr);
    if (ret) {
    dev_err(&pdev.dev, "failed to register controller\n");
    return ret;
    }
    return 0;
    }
//
// zynq_qspi_remove - Remove method for the QSPI driver
// @pdev:	Pointer to the platform_device structure
//
// This function is called if a device is physically removed from the system or
// if the driver module is being unloaded. It frees all resources allocated to
// the device.
//
// Return:	0 on success and error value on failure
//
#[no_mangle]
unsafe extern "C" fn zynq_qspi_remove(pdev: *mut platform_device) {
    static void zynq_qspi_remove(struct platform_device *pdev)
    {
    struct spi_controller *ctlr = platform_get_drvdata(pdev);
    struct zynq_qspi *xqspi = spi_controller_get_devdata(ctlr);
    spi_unregister_controller(ctlr);
    zynq_qspi_write(xqspi, ZYNQ_QSPI_ENABLE_OFFSET, 0);
    }
    static const struct of_device_id zynq_qspi_of_match[] = {
    { .compatible = "xlnx,zynq-qspi-1.0", },
    { /* end of table */ }
    };
    MODULE_DEVICE_TABLE(of, zynq_qspi_of_match);
//
// zynq_qspi_driver - This structure defines the QSPI platform driver
//
    static struct platform_driver zynq_qspi_driver = {
    .probe = zynq_qspi_probe,
    .remove = zynq_qspi_remove,
    .driver = {
    .name = "zynq-qspi",
    .of_match_table = zynq_qspi_of_match,
    },
    };
    module_platform_driver(zynq_qspi_driver);
    MODULE_AUTHOR("Xilinx, Inc.");
    MODULE_DESCRIPTION("Xilinx Zynq QSPI driver");
    MODULE_LICENSE("GPL");
