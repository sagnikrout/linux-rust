//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-cadence.c
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
// I2C bus driver for the Cadence I2C controller.
//
// Copyright (C) 2009 - 2014 Xilinx, Inc.
//

// Register offsets for the I2C device.
pub const CDNS_I2C_CR_OFFSET: c_uint = 0x00 /* Control Register, RW */;
pub const CDNS_I2C_SR_OFFSET: c_uint = 0x04 /* Status Register, RO */;
pub const CDNS_I2C_ADDR_OFFSET: c_uint = 0x08 /* I2C Address Register, RW */;
pub const CDNS_I2C_DATA_OFFSET: c_uint = 0x0C /* I2C Data Register, RW */;
pub const CDNS_I2C_ISR_OFFSET: c_uint = 0x10 /* IRQ Status Register, RW */;
pub const CDNS_I2C_XFER_SIZE_OFFSET: c_uint = 0x14 /* Transfer Size Register, RW */;
pub const CDNS_I2C_TIME_OUT_OFFSET: c_uint = 0x1C /* Time Out Register, RW */;
pub const CDNS_I2C_IMR_OFFSET: c_uint = 0x20 /* IRQ Mask Register, RO */;
pub const CDNS_I2C_IER_OFFSET: c_uint = 0x24 /* IRQ Enable Register, WO */;
pub const CDNS_I2C_IDR_OFFSET: c_uint = 0x28 /* IRQ Disable Register, WO */;
// Control Register Bit mask definitions

// Read or Write Master transfer 0 = Transmitter, 1 = Receiver

// 1 = Auto init FIFO to zeroes

pub const CDNS_I2C_CR_DIVA_SHIFT: c_int = 14;

pub const CDNS_I2C_CR_DIVB_SHIFT: c_int = 8;

    CDNS_I2C_CR_ACK_EN | \
    CDNS_I2C_CR_MS)

// Status Register Bit mask definitions

//
// I2C Address Register Bit mask definitions
// Normal addressing mode uses [6:0] bits. Extended addressing mode uses [9:0]
// bits. A write access to this register always initiates a transfer if the I2C
// is in master mode.
//
pub const CDNS_I2C_ADDR_MASK: c_uint = 0x000003FF /* I2C Address Mask */;
//
// I2C Interrupt Registers Bit mask definitions
// All the four interrupt registers (Status/Mask/Enable/Disable) have the same
// bit definitions.
//

    CDNS_I2C_IXR_RX_UNF | \
    CDNS_I2C_IXR_TX_OVF | \
    CDNS_I2C_IXR_RX_OVF | \
    CDNS_I2C_IXR_SLV_RDY | \
    CDNS_I2C_IXR_TO | \
    CDNS_I2C_IXR_NACK | \
    CDNS_I2C_IXR_DATA | \
    CDNS_I2C_IXR_COMP)

    CDNS_I2C_IXR_RX_UNF | \
    CDNS_I2C_IXR_TX_OVF | \
    CDNS_I2C_IXR_RX_OVF | \
    CDNS_I2C_IXR_NACK)

    CDNS_I2C_IXR_RX_UNF | \
    CDNS_I2C_IXR_TX_OVF | \
    CDNS_I2C_IXR_RX_OVF | \
    CDNS_I2C_IXR_NACK | \
    CDNS_I2C_IXR_DATA | \
    CDNS_I2C_IXR_COMP)

    CDNS_I2C_IXR_TX_OVF | \
    CDNS_I2C_IXR_RX_OVF | \
    CDNS_I2C_IXR_TO | \
    CDNS_I2C_IXR_NACK | \
    CDNS_I2C_IXR_DATA | \
    CDNS_I2C_IXR_COMP)

// timeout for pm runtime autosuspend

pub const CDNS_I2C_FIFO_DEPTH_DEFAULT: c_int = 16;
pub const CDNS_I2C_MAX_TRANSFER_SIZE: c_int = 255;
// Transfer size in multiples of data interrupt depth

pub const CDNS_I2C_DIVA_MAX: c_int = 4;
pub const CDNS_I2C_DIVB_MAX: c_int = 64;
pub const CDNS_I2C_TIMEOUT_MAX: c_uint = 0xFF;

pub const CDNS_I2C_POLL_US: c_int = 100000;
pub const CDNS_I2C_POLL_US_ATOMIC: c_int = 10;
pub const CDNS_I2C_TIMEOUT_US: c_int = 500000;

//
// enum cdns_i2c_mode - I2C Controller current operating mode
//
// @CDNS_I2C_MODE_SLAVE:       I2C controller operating in slave mode
// @CDNS_I2C_MODE_MASTER:      I2C Controller operating in master mode
//
    enum cdns_i2c_mode {
    CDNS_I2C_MODE_SLAVE,
    CDNS_I2C_MODE_MASTER,
    };
//
// enum cdns_i2c_slave_state - Slave state when I2C is operating in slave mode
//
// @CDNS_I2C_SLAVE_STATE_IDLE: I2C slave idle
// @CDNS_I2C_SLAVE_STATE_SEND: I2C slave sending data to master
// @CDNS_I2C_SLAVE_STATE_RECV: I2C slave receiving data from master
//
    enum cdns_i2c_slave_state {
    CDNS_I2C_SLAVE_STATE_IDLE,
    CDNS_I2C_SLAVE_STATE_SEND,
    CDNS_I2C_SLAVE_STATE_RECV,
    };

//
// struct cdns_i2c - I2C device private data structure
//
// @dev:		Pointer to device structure
// @membase:		Base address of the I2C device
// @adap:		I2C adapter instance
// @p_msg:		Message pointer
// @err_status:		Error status in Interrupt Status Register
// @xfer_done:		Transfer complete status
// @p_send_buf:		Pointer to transmit buffer
// @p_recv_buf:		Pointer to receive buffer
// @send_count:		Number of bytes still expected to send
// @recv_count:		Number of bytes still expected to receive
// @curr_recv_count:	Number of bytes to be received in current transfer
// @input_clk:		Input clock to I2C controller
// @i2c_clk:		Maximum I2C clock speed
// @bus_hold_flag:	Flag used in repeated start for clearing HOLD bit
// @clk:		Pointer to struct clk
// @clk_rate_change_nb:	Notifier block for clock rate changes
// @reset:		Reset control for the device
// @quirks:		flag for broken hold bit usage in r1p10
// @ctrl_reg:		Cached value of the control register.
// @rinfo:		I2C GPIO recovery information
// @ctrl_reg_diva_divb: value of fields DIV_A and DIV_B from CR register
// @slave:		Registered slave instance.
// @dev_mode:		I2C operating role(master/slave).
// @slave_state:	I2C Slave state(idle/read/write).
// @fifo_depth:		The depth of the transfer FIFO
// @transfer_size:	The maximum number of bytes in one transfer
// @atomic:		Mode of transfer
// @err_status_atomic:	Error status in atomic mode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_i2c {
    pub dev: *mut device,
    pub membase: *mut void __iomem,
    pub adap: i2c_adapter,
    pub p_msg: *mut i2c_msg,
    pub err_status: c_int,
    pub xfer_done: completion,
    pub p_send_buf: *mut c_uchar,
    pub p_recv_buf: *mut c_uchar,
    pub send_count: c_uint,
    pub recv_count: c_uint,
    pub curr_recv_count: c_uint,
    pub input_clk: c_ulong,
    pub i2c_clk: c_uint,
    pub bus_hold_flag: c_uint,
    pub clk: *mut clk,
    pub clk_rate_change_nb: notifier_block,
    pub reset: *mut reset_control,
    pub quirks: u32,
    pub ctrl_reg: u32,
    pub rinfo: i2c_bus_recovery_info,

    pub ctrl_reg_diva_divb: u16,
    pub slave: *mut i2c_client,
    pub dev_mode: enum cdns_i2c_mode,
    pub slave_state: enum cdns_i2c_slave_state,

    pub fifo_depth: u32,
    pub transfer_size: c_uint,
    pub atomic: bool,
    pub err_status_atomic: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_platform_data {
    pub quirks: u32,
}

    clk_rate_change_nb)
//
// cdns_i2c_init -  Controller initialisation
// @id:		Device private data structure
//
// Initialise the i2c controller.
//
#[no_mangle]
unsafe extern "C" fn cdns_i2c_init(id: *mut cdns_i2c) {
    static void cdns_i2c_init(struct cdns_i2c *id)
    {
    cdns_i2c_writereg(id.ctrl_reg, CDNS_I2C_CR_OFFSET);
//
// Cadence I2C controller has a bug wherein it generates
// invalid read transaction after HW timeout in master receiver mode.
// HW timeout is not used by this driver and the interrupt is disabled.
// But the feature itself cannot be disabled. Hence maximum value
// is written to this register to reduce the chances of error.
//
    cdns_i2c_writereg(CDNS_I2C_TIMEOUT_MAX, CDNS_I2C_TIME_OUT_OFFSET);
    }
//
// cdns_i2c_runtime_suspend -  Runtime suspend method for the driver
// @dev:	Address of the platform_device structure
//
// Put the driver into low power mode.
//
// Return: 0 always
//
#[no_mangle]
unsafe extern "C" fn cdns_i2c_runtime_suspend(dev: *mut device) -> c_int {
    static int cdns_i2c_runtime_suspend(struct device *dev)
    {
    struct cdns_i2c *xi2c = dev_get_drvdata(dev);
    clk_disable(xi2c.clk);
    return 0;
    }
//
// cdns_i2c_runtime_resume - Runtime resume
// @dev:	Address of the platform_device structure
//
// Runtime resume callback.
//
// Return: 0 on success and error value on error
//
#[no_mangle]
unsafe extern "C" fn cdns_i2c_runtime_resume(dev: *mut device) -> c_int {
    static int cdns_i2c_runtime_resume(struct device *dev)
    {
    struct cdns_i2c *xi2c = dev_get_drvdata(dev);
    int ret;
    ret = clk_enable(xi2c.clk);
    if (ret) {
    dev_err(dev, "Cannot enable clock.\n");
    return ret;
    }
    cdns_i2c_init(xi2c);
    return 0;
    }
//
// cdns_i2c_clear_bus_hold - Clear bus hold bit
// @id:	Pointer to driver data struct
//
// Helper to clear the controller's bus hold bit.
//
#[no_mangle]
unsafe extern "C" fn cdns_i2c_clear_bus_hold(id: *mut cdns_i2c) {
    static void cdns_i2c_clear_bus_hold(struct cdns_i2c *id)
    {
    let mut reg: u32 = cdns_i2c_readreg(CDNS_I2C_CR_OFFSET);
    if (reg & CDNS_I2C_CR_HOLD)
    cdns_i2c_writereg(reg & ~CDNS_I2C_CR_HOLD, CDNS_I2C_CR_OFFSET);
    }
#[no_mangle]
pub unsafe extern "C" fn cdns_is_holdquirk(id: *mut cdns_i2c, hold_wrkaround: bool) -> bool {
    static inline bool cdns_is_holdquirk(struct cdns_i2c *id, bool hold_wrkaround)
    {
    return (hold_wrkaround &&
    (id.curr_recv_count == id.fifo_depth + 1));
    }

#[no_mangle]
unsafe extern "C" fn cdns_i2c_set_mode(mode: enum cdns_i2c_mode, id: *mut cdns_i2c) {
    static void cdns_i2c_set_mode(enum cdns_i2c_mode mode, struct cdns_i2c *id)
    {
// Disable all interrupts
    cdns_i2c_writereg(CDNS_I2C_IXR_ALL_INTR_MASK, CDNS_I2C_IDR_OFFSET);
// Clear FIFO and transfer size
    cdns_i2c_writereg(CDNS_I2C_CR_CLR_FIFO, CDNS_I2C_CR_OFFSET);
// Update device mode and state
    id.dev_mode = mode;
    id.slave_state = CDNS_I2C_SLAVE_STATE_IDLE;
    switch (mode) {
    case CDNS_I2C_MODE_MASTER:
// Enable i2c master
    cdns_i2c_writereg(id.ctrl_reg_diva_divb |
    CDNS_I2C_CR_MASTER_EN_MASK,
    CDNS_I2C_CR_OFFSET);
//
// This delay is needed to give the IP some time to switch to
// the master mode. With lower values(like 110 us) i2cdetect
// will not detect any slave and without this delay, the IP will
// trigger a timeout interrupt.
//
    usleep_range(115, 125);
    break;
    case CDNS_I2C_MODE_SLAVE:
// Enable i2c slave
    cdns_i2c_writereg(id.ctrl_reg_diva_divb &
    CDNS_I2C_CR_SLAVE_EN_MASK,
    CDNS_I2C_CR_OFFSET);
// Setting slave address
    cdns_i2c_writereg(id.slave.addr & CDNS_I2C_ADDR_MASK,
    CDNS_I2C_ADDR_OFFSET);
// Enable slave send/receive interrupts
    cdns_i2c_writereg(CDNS_I2C_IXR_SLAVE_INTR_MASK,
    CDNS_I2C_IER_OFFSET);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn cdns_i2c_slave_rcv_data(id: *mut cdns_i2c) {
    static void cdns_i2c_slave_rcv_data(struct cdns_i2c *id)
    {
    u8 bytes;
    unsigned char data;
// Prepare backend for data reception
    if (id.slave_state == CDNS_I2C_SLAVE_STATE_IDLE) {
    id.slave_state = CDNS_I2C_SLAVE_STATE_RECV;
    i2c_slave_event(id.slave, I2C_SLAVE_WRITE_REQUESTED, core::ptr::null_mut());
    }
// Fetch number of bytes to receive
    bytes = cdns_i2c_readreg(CDNS_I2C_XFER_SIZE_OFFSET);
// Read data and send to backend
    while (bytes--) {
    data = cdns_i2c_readreg(CDNS_I2C_DATA_OFFSET);
    i2c_slave_event(id.slave, I2C_SLAVE_WRITE_RECEIVED, &data);
    }
    }
#[no_mangle]
unsafe extern "C" fn cdns_i2c_slave_send_data(id: *mut cdns_i2c) {
    static void cdns_i2c_slave_send_data(struct cdns_i2c *id)
    {
    u8 data;
// Prepare backend for data transmission
    if (id.slave_state == CDNS_I2C_SLAVE_STATE_IDLE) {
    id.slave_state = CDNS_I2C_SLAVE_STATE_SEND;
    i2c_slave_event(id.slave, I2C_SLAVE_READ_REQUESTED, &data);
    } else {
    i2c_slave_event(id.slave, I2C_SLAVE_READ_PROCESSED, &data);
    }
// Send data over bus
    cdns_i2c_writereg(data, CDNS_I2C_DATA_OFFSET);
    }
//
// cdns_i2c_slave_isr - Interrupt handler for the I2C device in slave role
// @ptr:       Pointer to I2C device private data
//
// This function handles the data interrupt and transfer complete interrupt of
// the I2C device in slave role.
//
// Return: IRQ_HANDLED always
//
#[no_mangle]
unsafe extern "C" fn cdns_i2c_slave_isr(ptr: *mut c_void) -> irqreturn_t {
    static irqreturn_t cdns_i2c_slave_isr(void *ptr)
    {
    struct cdns_i2c *id = ptr;
    unsigned int isr_status, i2c_status;
// Fetch the interrupt status
    isr_status = cdns_i2c_readreg(CDNS_I2C_ISR_OFFSET);
    cdns_i2c_writereg(isr_status, CDNS_I2C_ISR_OFFSET);
// Ignore masked interrupts
    isr_status &= ~cdns_i2c_readreg(CDNS_I2C_IMR_OFFSET);
// Fetch transfer mode (send/receive)
    i2c_status = cdns_i2c_readreg(CDNS_I2C_SR_OFFSET);
// Handle data send/receive
    if (i2c_status & CDNS_I2C_SR_RXRW) {
// Send data to master
    if (isr_status & CDNS_I2C_IXR_DATA)
    cdns_i2c_slave_send_data(id);
    if (isr_status & CDNS_I2C_IXR_COMP) {
    id.slave_state = CDNS_I2C_SLAVE_STATE_IDLE;
    i2c_slave_event(id.slave, I2C_SLAVE_STOP, core::ptr::null_mut());
    }
    } else {
// Receive data from master
    if (isr_status & CDNS_I2C_IXR_DATA)
    cdns_i2c_slave_rcv_data(id);
    if (isr_status & CDNS_I2C_IXR_COMP) {
    cdns_i2c_slave_rcv_data(id);
    id.slave_state = CDNS_I2C_SLAVE_STATE_IDLE;
    i2c_slave_event(id.slave, I2C_SLAVE_STOP, core::ptr::null_mut());
    }
    }
// Master indicated xfer stop or fifo underflow/overflow
    if (isr_status & (CDNS_I2C_IXR_NACK | CDNS_I2C_IXR_RX_OVF |
    CDNS_I2C_IXR_RX_UNF | CDNS_I2C_IXR_TX_OVF)) {
    id.slave_state = CDNS_I2C_SLAVE_STATE_IDLE;
    i2c_slave_event(id.slave, I2C_SLAVE_STOP, core::ptr::null_mut());
    cdns_i2c_writereg(CDNS_I2C_CR_CLR_FIFO, CDNS_I2C_CR_OFFSET);
    }
    return IRQ_HANDLED;
    }

//
// cdns_i2c_master_isr - Interrupt handler for the I2C device in master role
// @ptr:       Pointer to I2C device private data
//
// This function handles the data interrupt, transfer complete interrupt and
// the error interrupts of the I2C device in master role.
//
// Return: IRQ_HANDLED always
//
#[no_mangle]
unsafe extern "C" fn cdns_i2c_master_isr(ptr: *mut c_void) -> irqreturn_t {
    static irqreturn_t cdns_i2c_master_isr(void *ptr)
    {
    unsigned int isr_status, avail_bytes;
    unsigned int bytes_to_send;
    bool updatetx;
    struct cdns_i2c *id = ptr;
// Signal completion only after everything is updated
    let mut done_flag: c_int = 0;
    let mut status: irqreturn_t = IRQ_NONE;
    isr_status = cdns_i2c_readreg(CDNS_I2C_ISR_OFFSET);
    cdns_i2c_writereg(isr_status, CDNS_I2C_ISR_OFFSET);
    id.err_status = 0;
// Handling nack and arbitration lost interrupt
    if (isr_status & (CDNS_I2C_IXR_NACK | CDNS_I2C_IXR_ARB_LOST)) {
    done_flag = 1;
    status = IRQ_HANDLED;
    }
//
// Check if transfer size register needs to be updated again for a
// large data receive operation.
//
    updatetx = id.recv_count > id.curr_recv_count;
// When receiving, handle data interrupt and completion interrupt
    if (id.p_recv_buf &&
    ((isr_status & CDNS_I2C_IXR_COMP) ||
    (isr_status & CDNS_I2C_IXR_DATA))) {
// Read data if receive data valid is set
    while (cdns_i2c_readreg(CDNS_I2C_SR_OFFSET) &
    CDNS_I2C_SR_RXDV) {
    if (id.recv_count > 0) {
// (id->p_recv_buf)++ =
    cdns_i2c_readreg(CDNS_I2C_DATA_OFFSET);
    id.recv_count--;
    id.curr_recv_count--;
//
// Clear hold bit that was set for FIFO control
// if RX data left is less than or equal to
// FIFO DEPTH unless repeated start is selected
//
    if (id.recv_count <= id.fifo_depth &&
    !id.bus_hold_flag)
    cdns_i2c_clear_bus_hold(id);
    } else {
    dev_err(id.adap.dev.parent,
    "xfer_size reg rollover. xfer aborted!\n");
    id.err_status |= CDNS_I2C_IXR_TO;
    break;
    }
    if (cdns_is_holdquirk(id, updatetx))
    break;
    }
//
// The controller sends NACK to the slave when transfer size
// register reaches zero without considering the HOLD bit.
// This workaround is implemented for large data transfers to
// maintain transfer size non-zero while performing a large
// receive operation.
//
    if (cdns_is_holdquirk(id, updatetx)) {
// wait while fifo is full
    while (cdns_i2c_readreg(CDNS_I2C_XFER_SIZE_OFFSET) !=
    (id.curr_recv_count - id.fifo_depth))
    ;
//
// Check number of bytes to be received against maximum
// transfer size and update register accordingly.
//
    if (((int)(id.recv_count) - id.fifo_depth) >
    id.transfer_size) {
    cdns_i2c_writereg(id.transfer_size,
    CDNS_I2C_XFER_SIZE_OFFSET);
    id.curr_recv_count = id.transfer_size +
    id.fifo_depth;
    } else {
    cdns_i2c_writereg(id.recv_count -
    id.fifo_depth,
    CDNS_I2C_XFER_SIZE_OFFSET);
    id.curr_recv_count = id.recv_count;
    }
    }
// Clear hold (if not repeated start) and signal completion
    if ((isr_status & CDNS_I2C_IXR_COMP) && !id.recv_count) {
    if (!id.bus_hold_flag)
    cdns_i2c_clear_bus_hold(id);
    done_flag = 1;
    }
    status = IRQ_HANDLED;
    }
// When sending, handle transfer complete interrupt
    if ((isr_status & CDNS_I2C_IXR_COMP) && !id.p_recv_buf) {
//
// If there is more data to be sent, calculate the
// space available in FIFO and fill with that many bytes.
//
    if (id.send_count) {
    avail_bytes = id.fifo_depth -
    cdns_i2c_readreg(CDNS_I2C_XFER_SIZE_OFFSET);
    if (id.send_count > avail_bytes)
    bytes_to_send = avail_bytes;
    else
    bytes_to_send = id.send_count;
    while (bytes_to_send--) {
    cdns_i2c_writereg(
    (*(id.p_send_buf)++),
    CDNS_I2C_DATA_OFFSET);
    id.send_count--;
    }
    } else {
//
// Signal the completion of transaction and
// clear the hold bus bit if there are no
// further messages to be processed.
//
    done_flag = 1;
    }
    if (!id.send_count && !id.bus_hold_flag)
    cdns_i2c_clear_bus_hold(id);
    status = IRQ_HANDLED;
    }
// Update the status for errors
    id.err_status |= isr_status & CDNS_I2C_IXR_ERR_INTR_MASK;
    if (id.err_status)
    status = IRQ_HANDLED;
    if (done_flag)
    complete(&id.xfer_done);
    return status;
    }
//
// cdns_i2c_isr - Interrupt handler for the I2C device
// @irq:	irq number for the I2C device
// @ptr:	void pointer to cdns_i2c structure
//
// This function passes the control to slave/master based on current role of
// i2c controller.
//
// Return: IRQ_HANDLED always
//
#[no_mangle]
unsafe extern "C" fn cdns_i2c_isr(irq: c_int, ptr: *mut c_void) -> irqreturn_t {
    static irqreturn_t cdns_i2c_isr(int irq, void *ptr)
    {

    struct cdns_i2c *id = ptr;
    if (id.dev_mode == CDNS_I2C_MODE_SLAVE)
    return cdns_i2c_slave_isr(ptr);

    return cdns_i2c_master_isr(ptr);
    }
#[no_mangle]
unsafe extern "C" fn cdns_i2c_error_check(id: *mut cdns_i2c) -> bool {
    static bool cdns_i2c_error_check(struct cdns_i2c *id)
    {
    unsigned int isr_status;
    id.err_status = 0;
    isr_status = cdns_i2c_readreg(CDNS_I2C_ISR_OFFSET);
    cdns_i2c_writereg(isr_status & CDNS_I2C_IXR_ERR_INTR_MASK, CDNS_I2C_ISR_OFFSET);
    id.err_status = isr_status & CDNS_I2C_IXR_ERR_INTR_MASK;
    return !!id.err_status;
    }
#[no_mangle]
unsafe extern "C" fn cdns_i2c_mrecv_atomic(id: *mut cdns_i2c) {
    static void cdns_i2c_mrecv_atomic(struct cdns_i2c *id)
    {
    while (id.recv_count > 0) {
    bool updatetx;
//
// Check if transfer size register needs to be updated again for a
// large data receive operation.
//
    updatetx = id.recv_count > id.curr_recv_count;
    while (id.curr_recv_count > 0) {
    if (cdns_i2c_readreg(CDNS_I2C_SR_OFFSET) & CDNS_I2C_SR_RXDV) {
// id->p_recv_buf = cdns_i2c_readreg(CDNS_I2C_DATA_OFFSET);
    id.p_recv_buf++;
    id.recv_count--;
    id.curr_recv_count--;
//
// Clear the hold bit that was set for FIFO control,
// if the remaining RX data is less than or equal to
// the FIFO depth, unless a repeated start is selected.
//
    if (id.recv_count <= id.fifo_depth && !id.bus_hold_flag)
    cdns_i2c_clear_bus_hold(id);
    }
    if (cdns_i2c_error_check(id))
    return;
    if (cdns_is_holdquirk(id, updatetx))
    break;
    }
//
// The controller sends NACK to the slave/target when transfer size
// register reaches zero without considering the HOLD bit.
// This workaround is implemented for large data transfers to
// maintain transfer size non-zero while performing a large
// receive operation.
//
    if (cdns_is_holdquirk(id, updatetx)) {
// wait while fifo is full
    while (cdns_i2c_readreg(CDNS_I2C_XFER_SIZE_OFFSET) !=
    (id.curr_recv_count - id.fifo_depth))
    ;
//
// Check number of bytes to be received against maximum
// transfer size and update register accordingly.
//
    if ((id.recv_count - id.fifo_depth) >
    id.transfer_size) {
    cdns_i2c_writereg(id.transfer_size,
    CDNS_I2C_XFER_SIZE_OFFSET);
    id.curr_recv_count = id.transfer_size +
    id.fifo_depth;
    } else {
    cdns_i2c_writereg(id.recv_count -
    id.fifo_depth,
    CDNS_I2C_XFER_SIZE_OFFSET);
    id.curr_recv_count = id.recv_count;
    }
    }
    }
// Clear hold (if not repeated start)
    if (!id.recv_count && !id.bus_hold_flag)
    cdns_i2c_clear_bus_hold(id);
    }
//
// cdns_i2c_mrecv - Prepare and start a master receive operation
// @id:		pointer to the i2c device structure
//
#[no_mangle]
unsafe extern "C" fn cdns_i2c_mrecv(id: *mut cdns_i2c) {
    static void cdns_i2c_mrecv(struct cdns_i2c *id)
    {
    unsigned int ctrl_reg;
    unsigned int isr_status;
    unsigned long flags;
    let mut hold_clear: bool = false;
    let mut irq_save: bool = false;
    u32 addr;
    id.p_recv_buf = id.p_msg.buf;
    id.recv_count = id.p_msg.len;
// Put the controller in master receive mode and clear the FIFO
    ctrl_reg = cdns_i2c_readreg(CDNS_I2C_CR_OFFSET);
    ctrl_reg |= CDNS_I2C_CR_RW | CDNS_I2C_CR_CLR_FIFO;
//
// Receive up to I2C_SMBUS_BLOCK_MAX data bytes, plus one message length
// byte, plus one checksum byte if PEC is enabled. p_msg->len will be 2 if
// PEC is enabled, otherwise 1.
//
    if (id.p_msg.flags & I2C_M_RECV_LEN)
    id.recv_count = I2C_SMBUS_BLOCK_MAX + id.p_msg.len;
    id.curr_recv_count = id.recv_count;
//
// Check for the message size against FIFO depth and set the
// 'hold bus' bit if it is greater than FIFO depth.
//
    if (id.recv_count > id.fifo_depth)
    ctrl_reg |= CDNS_I2C_CR_HOLD;
    cdns_i2c_writereg(ctrl_reg, CDNS_I2C_CR_OFFSET);
// Clear the interrupts in interrupt status register
    isr_status = cdns_i2c_readreg(CDNS_I2C_ISR_OFFSET);
    cdns_i2c_writereg(isr_status, CDNS_I2C_ISR_OFFSET);
//
// The no. of bytes to receive is checked against the limit of
// max transfer size. Set transfer size register with no of bytes
// receive if it is less than transfer size and transfer size if
// it is more. Enable the interrupts.
//
    if (id.recv_count > id.transfer_size) {
    cdns_i2c_writereg(id.transfer_size,
    CDNS_I2C_XFER_SIZE_OFFSET);
    id.curr_recv_count = id.transfer_size;
    } else {
    cdns_i2c_writereg(id.recv_count, CDNS_I2C_XFER_SIZE_OFFSET);
    }
// Determine hold_clear based on number of bytes to receive and hold flag
    if (!id.bus_hold_flag && id.recv_count <= id.fifo_depth) {
    if (ctrl_reg & CDNS_I2C_CR_HOLD) {
    hold_clear = true;
    if (id.quirks & CDNS_I2C_BROKEN_HOLD_BIT)
    irq_save = true;
    }
    }
    addr = id.p_msg.addr;
    addr &= CDNS_I2C_ADDR_MASK;
    if (hold_clear) {
    ctrl_reg &= ~CDNS_I2C_CR_HOLD;
    ctrl_reg &= ~CDNS_I2C_CR_CLR_FIFO;
//
// In case of Xilinx Zynq SOC, clear the HOLD bit before transfer size
// register reaches '0'. This is an IP bug which causes transfer size
// register overflow to 0xFF. To satisfy this timing requirement,
// disable the interrupts on current processor core between register
// writes to slave address register and control register.
//
    if (irq_save)
    local_irq_save(flags);
    cdns_i2c_writereg(addr, CDNS_I2C_ADDR_OFFSET);
    cdns_i2c_writereg(ctrl_reg, CDNS_I2C_CR_OFFSET);
// Read it back to avoid bufferring and make sure write happens
    cdns_i2c_readreg(CDNS_I2C_CR_OFFSET);
    if (irq_save)
    local_irq_restore(flags);
    } else {
    cdns_i2c_writereg(addr, CDNS_I2C_ADDR_OFFSET);
    }
    if (!id.atomic)
    cdns_i2c_writereg(CDNS_I2C_ENABLED_INTR_MASK, CDNS_I2C_IER_OFFSET);
    else
    cdns_i2c_mrecv_atomic(id);
    }
#[no_mangle]
unsafe extern "C" fn cdns_i2c_msend_rem_atomic(id: *mut cdns_i2c) {
    static void cdns_i2c_msend_rem_atomic(struct cdns_i2c *id)
    {
    while (id.send_count) {
    unsigned int avail_bytes;
    unsigned int bytes_to_send;
    avail_bytes = id.fifo_depth - cdns_i2c_readreg(CDNS_I2C_XFER_SIZE_OFFSET);
    if (id.send_count > avail_bytes)
    bytes_to_send = avail_bytes;
    else
    bytes_to_send = id.send_count;
    while (bytes_to_send--) {
    cdns_i2c_writereg((*id.p_send_buf++), CDNS_I2C_DATA_OFFSET);
    id.send_count--;
    }
    if (cdns_i2c_error_check(id))
    return;
    }
    if (!id.send_count && !id.bus_hold_flag)
    cdns_i2c_clear_bus_hold(id);
    }
//
// cdns_i2c_msend - Prepare and start a master send operation
// @id:		pointer to the i2c device
//
#[no_mangle]
unsafe extern "C" fn cdns_i2c_msend(id: *mut cdns_i2c) {
    static void cdns_i2c_msend(struct cdns_i2c *id)
    {
    unsigned int avail_bytes;
    unsigned int bytes_to_send;
    unsigned int ctrl_reg;
    unsigned int isr_status;
    id.p_recv_buf = core::ptr::null_mut();
    id.p_send_buf = id.p_msg.buf;
    id.send_count = id.p_msg.len;
// Set the controller in Master transmit mode and clear the FIFO.
    ctrl_reg = cdns_i2c_readreg(CDNS_I2C_CR_OFFSET);
    ctrl_reg &= ~CDNS_I2C_CR_RW;
    ctrl_reg |= CDNS_I2C_CR_CLR_FIFO;
//
// Check for the message size against FIFO depth and set the
// 'hold bus' bit if it is greater than FIFO depth.
//
    if (id.send_count > id.fifo_depth)
    ctrl_reg |= CDNS_I2C_CR_HOLD;
    cdns_i2c_writereg(ctrl_reg, CDNS_I2C_CR_OFFSET);
// Clear the interrupts in interrupt status register.
    isr_status = cdns_i2c_readreg(CDNS_I2C_ISR_OFFSET);
    cdns_i2c_writereg(isr_status, CDNS_I2C_ISR_OFFSET);
//
// Calculate the space available in FIFO. Check the message length
// against the space available, and fill the FIFO accordingly.
// Enable the interrupts.
//
    avail_bytes = id.fifo_depth -
    cdns_i2c_readreg(CDNS_I2C_XFER_SIZE_OFFSET);
    if (id.send_count > avail_bytes)
    bytes_to_send = avail_bytes;
    else
    bytes_to_send = id.send_count;
    while (bytes_to_send--) {
    cdns_i2c_writereg((*(id.p_send_buf)++), CDNS_I2C_DATA_OFFSET);
    id.send_count--;
    }
//
// Clear the bus hold flag if there is no more data
// and if it is the last message.
//
    if (!id.bus_hold_flag && !id.send_count)
    cdns_i2c_clear_bus_hold(id);
// Set the slave address in address register - triggers operation.
    cdns_i2c_writereg(id.p_msg.addr & CDNS_I2C_ADDR_MASK,
    CDNS_I2C_ADDR_OFFSET);
    if (!id.atomic)
    cdns_i2c_writereg(CDNS_I2C_ENABLED_INTR_MASK, CDNS_I2C_IER_OFFSET);
#[no_mangle]
pub unsafe extern "C" fn if(0: id->send_count >) -> else {
    else if (id.send_count > 0)
    cdns_i2c_msend_rem_atomic(id);
    }
//
// cdns_i2c_master_reset - Reset the interface
// @adap:	pointer to the i2c adapter driver instance
//
// This function cleanup the fifos, clear the hold bit and status
// and disable the interrupts.
//
#[no_mangle]
unsafe extern "C" fn cdns_i2c_master_reset(adap: *mut i2c_adapter) {
    static void cdns_i2c_master_reset(struct i2c_adapter *adap)
    {
    struct cdns_i2c *id = adap.algo_data;
    u32 regval;
// Disable the interrupts
    cdns_i2c_writereg(CDNS_I2C_IXR_ALL_INTR_MASK, CDNS_I2C_IDR_OFFSET);
// Clear the hold bit and fifos
    regval = cdns_i2c_readreg(CDNS_I2C_CR_OFFSET);
    regval &= ~CDNS_I2C_CR_HOLD;
    regval |= CDNS_I2C_CR_CLR_FIFO;
    cdns_i2c_writereg(regval, CDNS_I2C_CR_OFFSET);
// Update the transfercount register to zero
    cdns_i2c_writereg(0, CDNS_I2C_XFER_SIZE_OFFSET);
// Clear the interrupt status register
    regval = cdns_i2c_readreg(CDNS_I2C_ISR_OFFSET);
    cdns_i2c_writereg(regval, CDNS_I2C_ISR_OFFSET);
// Clear the status register
    regval = cdns_i2c_readreg(CDNS_I2C_SR_OFFSET);
    cdns_i2c_writereg(regval, CDNS_I2C_SR_OFFSET);
    }
    static int cdns_i2c_process_msg(struct cdns_i2c *id, struct i2c_msg *msg,
    struct i2c_adapter *adap)
    {
    unsigned long time_left, msg_timeout;
    u32 reg;
    id.p_msg = msg;
    id.err_status = 0;
    if (!id.atomic)
    reinit_completion(&id.xfer_done);
// Check for the TEN Bit mode on each msg
    reg = cdns_i2c_readreg(CDNS_I2C_CR_OFFSET);
    if (msg.flags & I2C_M_TEN) {
    if (reg & CDNS_I2C_CR_NEA)
    cdns_i2c_writereg(reg & ~CDNS_I2C_CR_NEA,
    CDNS_I2C_CR_OFFSET);
    } else {
    if (!(reg & CDNS_I2C_CR_NEA))
    cdns_i2c_writereg(reg | CDNS_I2C_CR_NEA,
    CDNS_I2C_CR_OFFSET);
    }
// Check for the R/W flag on each msg
    if (msg.flags & I2C_M_RD)
    cdns_i2c_mrecv(id);
    else
    cdns_i2c_msend(id);
// Minimal time to execute this message
    msg_timeout = msecs_to_jiffies((1000 * msg.len * BITS_PER_BYTE) / id.i2c_clk);
//
// Plus some wiggle room.
// For non-atomic contexts, 500 ms is added to the timeout.
// For atomic contexts, 2000 ms is added because transfers happen in polled
// mode, requiring more time to account for the polling overhead.
//
    if (!id.atomic)
    msg_timeout += msecs_to_jiffies(500);
    else
    msg_timeout += msecs_to_jiffies(2000);
    if (msg_timeout < adap.timeout)
    msg_timeout = adap.timeout;
    if (!id.atomic) {
// Wait for the signal of completion
    time_left = wait_for_completion_timeout(&id.xfer_done, msg_timeout);
    } else {
// 0 is success, -ETIMEDOUT is error
    time_left = !readl_poll_timeout_atomic(id.membase + CDNS_I2C_ISR_OFFSET,
    reg, (reg & CDNS_I2C_IXR_COMP),
    CDNS_I2C_POLL_US_ATOMIC, msg_timeout);
    }
    if (time_left == 0) {
    cdns_i2c_master_reset(adap);
    return -ETIMEDOUT;
    }
    cdns_i2c_writereg(CDNS_I2C_IXR_ALL_INTR_MASK,
    CDNS_I2C_IDR_OFFSET);
// If it is bus arbitration error, try again
    if (id.err_status & CDNS_I2C_IXR_ARB_LOST)
    return -EAGAIN;
    if (msg.flags & I2C_M_RECV_LEN)
    msg.len += min_t(unsigned int, msg.buf[0], I2C_SMBUS_BLOCK_MAX);
    return 0;
    }
    static int cdns_i2c_master_common_xfer(struct i2c_adapter *adap,
    struct i2c_msg *msgs,
    int num)
    {
    int ret, count;
    u32 reg;
    struct cdns_i2c *id = adap.algo_data;
    bool hold_quirk;
// Check if the bus is free
    if (!id.atomic)
    ret = readl_relaxed_poll_timeout(id.membase + CDNS_I2C_SR_OFFSET,
    reg,
    !(reg & CDNS_I2C_SR_BA),
    CDNS_I2C_POLL_US, CDNS_I2C_TIMEOUT_US);
    else
    ret = readl_poll_timeout_atomic(id.membase + CDNS_I2C_SR_OFFSET,
    reg,
    !(reg & CDNS_I2C_SR_BA),
    CDNS_I2C_POLL_US_ATOMIC, CDNS_I2C_TIMEOUT_US);
    if (ret) {
    ret = -EAGAIN;
    if (id.adap.bus_recovery_info)
    i2c_recover_bus(adap);
    return ret;
    }
    hold_quirk = !!(id.quirks & CDNS_I2C_BROKEN_HOLD_BIT);
//
// Set the flag to one when multiple messages are to be
// processed with a repeated start.
//
    if (num > 1) {
//
// This controller does not give completion interrupt after a
// master receive message if HOLD bit is set (repeated start),
// resulting in SW timeout. Hence, if a receive message is
// followed by any other message, an error is returned
// indicating that this sequence is not supported.
//
    for (count = 0; (count < num - 1 && hold_quirk); count++) {
    if (msgs[count].flags & I2C_M_RD) {
    dev_warn(adap.dev.parent,
    "Can't do repeated start after a receive message\n");
    return -EOPNOTSUPP;
    }
    }
    id.bus_hold_flag = 1;
    reg = cdns_i2c_readreg(CDNS_I2C_CR_OFFSET);
    reg |= CDNS_I2C_CR_HOLD;
    cdns_i2c_writereg(reg, CDNS_I2C_CR_OFFSET);
    } else {
    id.bus_hold_flag = 0;
    }
// Process the msg one by one
    for (count = 0; count < num; count++, msgs++) {
    if (count == (num - 1))
    id.bus_hold_flag = 0;
    ret = cdns_i2c_process_msg(id, msgs, adap);
    if (ret)
    return ret;
// Report the other error interrupts to application
    if (id.err_status || id.err_status_atomic) {
    cdns_i2c_master_reset(adap);
    if (id.err_status & CDNS_I2C_IXR_NACK)
    return -ENXIO;
    return -EIO;
    }
    }
    return 0;
    }
//
// cdns_i2c_master_xfer - The main i2c transfer function
// @adap:	pointer to the i2c adapter driver instance
// @msgs:	pointer to the i2c message structure
// @num:	the number of messages to transfer
//
// Initiates the send/recv activity based on the transfer message received.
//
// Return: number of msgs processed on success, negative error otherwise
//
    static int cdns_i2c_master_xfer(struct i2c_adapter *adap, struct i2c_msg *msgs,
    int num)
    {
    int ret;
    struct cdns_i2c *id = adap.algo_data;

    let mut change_role: bool = false;

    ret = pm_runtime_resume_and_get(id.dev);
    if (ret < 0)
    return ret;

// Check i2c operating mode and switch if possible
    if (id.dev_mode == CDNS_I2C_MODE_SLAVE) {
    if (id.slave_state != CDNS_I2C_SLAVE_STATE_IDLE) {
    ret = -EAGAIN;
    goto out;
    }
// Set mode to master
    cdns_i2c_set_mode(CDNS_I2C_MODE_MASTER, id);
// Mark flag to change role once xfer is completed
    change_role = true;
    }

    ret = cdns_i2c_master_common_xfer(adap, msgs, num);
    if (!ret)
    ret = num;

    out:
// Switch i2c mode to slave
    if (change_role)
    cdns_i2c_set_mode(CDNS_I2C_MODE_SLAVE, id);

    pm_runtime_put_autosuspend(id.dev);
    return ret;
    }
//
// cdns_i2c_master_xfer_atomic - The i2c transfer function in atomic mode
// @adap:	pointer to the i2c adapter driver instance
// @msgs:	pointer to the i2c message structure
// @num:	the number of messages to transfer
//
// Return: number of msgs processed on success, negative error otherwise
//
    static int cdns_i2c_master_xfer_atomic(struct i2c_adapter *adap, struct i2c_msg *msgs,
    int num)
    {
    int ret;
    struct cdns_i2c *id = adap.algo_data;
    ret = cdns_i2c_runtime_resume(id.dev);
    if (ret)
    return ret;
    if (id.quirks & CDNS_I2C_BROKEN_HOLD_BIT) {
    dev_warn(id.adap.dev.parent,
    "Atomic xfer not supported for version 1.0\n");
    return 0;
    }
    id.atomic = true;
    ret = cdns_i2c_master_common_xfer(adap, msgs, num);
    if (!ret)
    ret = num;
    id.atomic = false;
    cdns_i2c_runtime_suspend(id.dev);
    return ret;
    }
//
// cdns_i2c_func - Returns the supported features of the I2C driver
// @adap:	pointer to the i2c adapter structure
//
// Return: 32 bit value, each bit corresponding to a feature
//
#[no_mangle]
unsafe extern "C" fn cdns_i2c_func(adap: *mut i2c_adapter) -> u32 {
    static u32 cdns_i2c_func(struct i2c_adapter *adap)
    {
    u32 func = I2C_FUNC_I2C | I2C_FUNC_10BIT_ADDR |
    (I2C_FUNC_SMBUS_EMUL & ~I2C_FUNC_SMBUS_QUICK) |
    I2C_FUNC_SMBUS_BLOCK_DATA;

    func |= I2C_FUNC_SLAVE;

    return func;
    }

#[no_mangle]
unsafe extern "C" fn cdns_reg_slave(slave: *mut i2c_client) -> c_int {
    static int cdns_reg_slave(struct i2c_client *slave)
    {
    int ret;
    struct cdns_i2c *id = container_of(slave.adapter, struct cdns_i2c,
    adap);
    if (id.slave)
    return -EBUSY;
    if (slave.flags & I2C_CLIENT_TEN)
    return -EAFNOSUPPORT;
    ret = pm_runtime_resume_and_get(id.dev);
    if (ret < 0)
    return ret;
// Store slave information
    id.slave = slave;
// Enable I2C slave
    cdns_i2c_set_mode(CDNS_I2C_MODE_SLAVE, id);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cdns_unreg_slave(slave: *mut i2c_client) -> c_int {
    static int cdns_unreg_slave(struct i2c_client *slave)
    {
    struct cdns_i2c *id = container_of(slave.adapter, struct cdns_i2c,
    adap);
    pm_runtime_put(id.dev);
// Remove slave information
    id.slave = core::ptr::null_mut();
// Enable I2C master
    cdns_i2c_set_mode(CDNS_I2C_MODE_MASTER, id);
    return 0;
    }

    static const struct i2c_algorithm cdns_i2c_algo = {
    .xfer = cdns_i2c_master_xfer,
    .xfer_atomic = cdns_i2c_master_xfer_atomic,
    .functionality = cdns_i2c_func,

    .reg_slave = cdns_reg_slave,
    .unreg_slave = cdns_unreg_slave,

    };
//
// cdns_i2c_calc_divs - Calculate clock dividers
// @f:		I2C clock frequency
// @input_clk:	Input clock frequency
// @a:		First divider (return value)
// @b:		Second divider (return value)
//
// f is used as input and output variable. As input it is used as target I2C
// frequency. On function exit f holds the actually resulting I2C frequency.
//
// Return: 0 on success, negative errno otherwise.
//
    static int cdns_i2c_calc_divs(unsigned long *f, unsigned long input_clk,
    unsigned int *a, unsigned int *b)
    {
    let mut fscl: c_ulong = *f, best_fscl = *f, actual_fscl, temp;
    unsigned int div_a, div_b, calc_div_a = 0, calc_div_b = 0;
    unsigned int last_error, current_error;
// calculate (divisor_a+1) x (divisor_b+1)
    temp = input_clk / (22 * fscl);
//
// If the calculated value is negative or 0, the fscl input is out of
// range. Return error.
//
    if (!temp || (temp > (CDNS_I2C_DIVA_MAX * CDNS_I2C_DIVB_MAX)))
    return -EINVAL;
    last_error = -1;
    for (div_a = 0; div_a < CDNS_I2C_DIVA_MAX; div_a++) {
    div_b = DIV_ROUND_UP(input_clk, 22 * fscl * (div_a + 1));
    if ((div_b < 1) || (div_b > CDNS_I2C_DIVB_MAX))
    continue;
    div_b--;
    actual_fscl = input_clk / (22 * (div_a + 1) * (div_b + 1));
    if (actual_fscl > fscl)
    continue;
    current_error = fscl - actual_fscl;
    if (last_error > current_error) {
    calc_div_a = div_a;
    calc_div_b = div_b;
    best_fscl = actual_fscl;
    last_error = current_error;
    }
    }
// a = calc_div_a;
// b = calc_div_b;
// f = best_fscl;
    return 0;
    }
//
// cdns_i2c_setclk - This function sets the serial clock rate for the I2C device
// @clk_in:	I2C clock input frequency in Hz
// @id:		Pointer to the I2C device structure
//
// The device must be idle rather than busy transferring data before setting
// these device options.
// The data rate is set by values in the control register.
// The formula for determining the correct register values is
// Fscl = Fpclk/(22 x (divisor_a+1) x (divisor_b+1))
// See the hardware data sheet for a full explanation of setting the serial
// clock rate. The clock can not be faster than the input clock divide by 22.
// The two most common clock rates are 100KHz and 400KHz.
//
// Return: 0 on success, negative error otherwise
//
#[no_mangle]
unsafe extern "C" fn cdns_i2c_setclk(clk_in: c_ulong, id: *mut cdns_i2c) -> c_int {
    static int cdns_i2c_setclk(unsigned long clk_in, struct cdns_i2c *id)
    {
    unsigned int div_a, div_b;
    unsigned int ctrl_reg;
    let mut ret: c_int = 0;
    let mut fscl: c_ulong = id.i2c_clk;
    ret = cdns_i2c_calc_divs(&fscl, clk_in, &div_a, &div_b);
    if (ret)
    return ret;
    ctrl_reg = id.ctrl_reg;
    ctrl_reg &= ~(CDNS_I2C_CR_DIVA_MASK | CDNS_I2C_CR_DIVB_MASK);
    ctrl_reg |= ((div_a << CDNS_I2C_CR_DIVA_SHIFT) |
    (div_b << CDNS_I2C_CR_DIVB_SHIFT));
    id.ctrl_reg = ctrl_reg;
    cdns_i2c_writereg(ctrl_reg, CDNS_I2C_CR_OFFSET);

    id.ctrl_reg_diva_divb = ctrl_reg & (CDNS_I2C_CR_DIVA_MASK |
    CDNS_I2C_CR_DIVB_MASK);

    return 0;
    }
//
// cdns_i2c_clk_notifier_cb - Clock rate change callback
// @nb:		Pointer to notifier block
// @event:	Notification reason
// @data:	Pointer to notification data object
//
// This function is called when the cdns_i2c input clock frequency changes.
// The callback checks whether a valid bus frequency can be generated after the
// change. If so, the change is acknowledged, otherwise the change is aborted.
// New dividers are written to the HW in the pre- or post change notification
// depending on the scaling direction.
//
// Return:	NOTIFY_STOP if the rate change should be aborted, NOTIFY_OK
// to acknowledge the change, NOTIFY_DONE if the notification is
// considered irrelevant.
//
    static int cdns_i2c_clk_notifier_cb(struct notifier_block *nb, unsigned long
    event, void *data)
    {
    struct clk_notifier_data *ndata = data;
    struct cdns_i2c *id = to_cdns_i2c(nb);
    if (pm_runtime_suspended(id.dev))
    return NOTIFY_OK;
    switch (event) {
    case PRE_RATE_CHANGE:
    {
    let mut input_clk: c_ulong = ndata.new_rate;
    let mut fscl: c_ulong = id.i2c_clk;
    unsigned int div_a, div_b;
    int ret;
    ret = cdns_i2c_calc_divs(&fscl, input_clk, &div_a, &div_b);
    if (ret) {
    dev_warn(id.adap.dev.parent,
    "clock rate change rejected\n");
    return NOTIFY_STOP;
    }
// scale up
    if (ndata.new_rate > ndata.old_rate)
    cdns_i2c_setclk(ndata.new_rate, id);
    return NOTIFY_OK;
    }
    case POST_RATE_CHANGE:
    id.input_clk = ndata.new_rate;
// scale down
    if (ndata.new_rate < ndata.old_rate)
    cdns_i2c_setclk(ndata.new_rate, id);
    return NOTIFY_OK;
    case ABORT_RATE_CHANGE:
// scale up
    if (ndata.new_rate > ndata.old_rate)
    cdns_i2c_setclk(ndata.old_rate, id);
    return NOTIFY_OK;
    default:
    return NOTIFY_DONE;
    }
    }
#[no_mangle]
unsafe extern "C" fn cdns_i2c_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused cdns_i2c_suspend(struct device *dev)
    {
    struct cdns_i2c *xi2c = dev_get_drvdata(dev);
    i2c_mark_adapter_suspended(&xi2c.adap);
    if (!pm_runtime_status_suspended(dev))
    return cdns_i2c_runtime_suspend(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cdns_i2c_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused cdns_i2c_resume(struct device *dev)
    {
    struct cdns_i2c *xi2c = dev_get_drvdata(dev);
    int err;
    err = cdns_i2c_runtime_resume(dev);
    if (err)
    return err;
    if (pm_runtime_status_suspended(dev)) {
    err = cdns_i2c_runtime_suspend(dev);
    if (err)
    return err;
    }
    i2c_mark_adapter_resumed(&xi2c.adap);
    return 0;
    }
    static const struct dev_pm_ops cdns_i2c_dev_pm_ops = {
    SET_NOIRQ_SYSTEM_SLEEP_PM_OPS(cdns_i2c_suspend, cdns_i2c_resume)
    SET_RUNTIME_PM_OPS(cdns_i2c_runtime_suspend,
    cdns_i2c_runtime_resume, core::ptr::null_mut())
    };
    static const struct cdns_platform_data r1p10_i2c_def = {
    .quirks = CDNS_I2C_BROKEN_HOLD_BIT,
    };
    static const struct of_device_id cdns_i2c_of_match[] = {
    { .compatible = "cdns,i2c-r1p10", .data = &r1p10_i2c_def },
    { .compatible = "cdns,i2c-r1p14",},
    { /* end of table */ }
    };
    MODULE_DEVICE_TABLE(of, cdns_i2c_of_match);
//
// cdns_i2c_detect_transfer_size - Detect the maximum transfer size supported
// @id: Device private data structure
//
// Detect the maximum transfer size that is supported by this instance of the
// Cadence I2C controller.
//
#[no_mangle]
unsafe extern "C" fn cdns_i2c_detect_transfer_size(id: *mut cdns_i2c) {
    static void cdns_i2c_detect_transfer_size(struct cdns_i2c *id)
    {
    u32 val;
//
// Writing to the transfer size register is only possible if these two bits
// are set in the control register.
//
    cdns_i2c_writereg(CDNS_I2C_CR_MS | CDNS_I2C_CR_RW, CDNS_I2C_CR_OFFSET);
//
// The number of writable bits of the transfer size register can be between
// 4 and 8. This is a controlled through a synthesis parameter of the IP
// core and can vary from instance to instance. The unused MSBs always read
// back as 0. Writing 0xff and then reading the value back will report the
// maximum supported transfer size.
//
    cdns_i2c_writereg(CDNS_I2C_MAX_TRANSFER_SIZE, CDNS_I2C_XFER_SIZE_OFFSET);
    val = cdns_i2c_readreg(CDNS_I2C_XFER_SIZE_OFFSET);
    id.transfer_size = CDNS_I2C_TRANSFER_SIZE(val);
    cdns_i2c_writereg(0, CDNS_I2C_XFER_SIZE_OFFSET);
    cdns_i2c_writereg(0, CDNS_I2C_CR_OFFSET);
    }
//
// cdns_i2c_probe - Platform registration call
// @pdev:	Handle to the platform device structure
//
// This function does all the memory allocation and registration for the i2c
// device. User can modify the address mode to 10 bit address mode using the
// ioctl call with option I2C_TENBIT.
//
// Return: 0 on success, negative error otherwise
//
#[no_mangle]
unsafe extern "C" fn cdns_i2c_probe(pdev: *mut platform_device) -> c_int {
    static int cdns_i2c_probe(struct platform_device *pdev)
    {
    struct resource *r_mem;
    struct cdns_i2c *id;
    int ret, irq;
    const struct of_device_id *match;
    id = devm_kzalloc(&pdev.dev, sizeof(*id), GFP_KERNEL);
    if (!id)
    return -ENOMEM;
    id.dev = &pdev.dev;
    platform_set_drvdata(pdev, id);
    match = of_match_node(cdns_i2c_of_match, pdev.dev.of_node);
    if (match && match.data) {
    const struct cdns_platform_data *data = match.data;
    id.quirks = data.quirks;
    }
    id.rinfo.pinctrl = devm_pinctrl_get(&pdev.dev);
    if (IS_ERR(id.rinfo.pinctrl)) {
    let mut err: c_int = PTR_ERR(id.rinfo.pinctrl);
    dev_info(&pdev.dev, "can't get pinctrl, bus recovery not supported\n");
    if (err != -ENODEV)
    return err;
    } else {
    id.adap.bus_recovery_info = &id.rinfo;
    }
    id.membase = devm_platform_get_and_ioremap_resource(pdev, 0, &r_mem);
    if (IS_ERR(id.membase))
    return PTR_ERR(id.membase);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    id.adap.owner = THIS_MODULE;
    id.adap.dev.of_node = pdev.dev.of_node;
    id.adap.algo = &cdns_i2c_algo;
    id.adap.timeout = CDNS_I2C_TIMEOUT;
    id.adap.retries = 3;		/* Default retry value. */
    id.adap.algo_data = id;
    id.adap.dev.parent = &pdev.dev;
    init_completion(&id.xfer_done);
    snprintf(id.adap.name, sizeof(id.adap.name),
    "Cadence I2C at %08lx", (unsigned long)r_mem.start);
    id.clk = devm_clk_get_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(id.clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(id.clk),
    "input clock not found.\n");
    id.reset = devm_reset_control_get_optional_shared(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(id.reset))
    return dev_err_probe(&pdev.dev, PTR_ERR(id.reset),
    "Failed to request reset.\n");
    ret = reset_control_deassert(id.reset);
    if (ret)
    return dev_err_probe(&pdev.dev, ret,
    "Failed to de-assert reset.\n");
    pm_runtime_set_autosuspend_delay(id.dev, CNDS_I2C_PM_TIMEOUT);
    pm_runtime_use_autosuspend(id.dev);
    pm_runtime_set_active(id.dev);
    pm_runtime_enable(id.dev);
    id.clk_rate_change_nb.notifier_call = cdns_i2c_clk_notifier_cb;
    if (clk_notifier_register(id.clk, &id.clk_rate_change_nb))
    dev_warn(&pdev.dev, "Unable to register clock notifier.\n");
    id.input_clk = clk_get_rate(id.clk);
    ret = of_property_read_u32(pdev.dev.of_node, "clock-frequency",
    &id.i2c_clk);
    if (ret || (id.i2c_clk > I2C_MAX_FAST_MODE_FREQ))
    id.i2c_clk = I2C_MAX_STANDARD_MODE_FREQ;

// Set initial mode to master
    id.dev_mode = CDNS_I2C_MODE_MASTER;
    id.slave_state = CDNS_I2C_SLAVE_STATE_IDLE;

    id.ctrl_reg = CDNS_I2C_CR_ACK_EN | CDNS_I2C_CR_NEA | CDNS_I2C_CR_MS;
    id.fifo_depth = CDNS_I2C_FIFO_DEPTH_DEFAULT;
    of_property_read_u32(pdev.dev.of_node, "fifo-depth", &id.fifo_depth);
    cdns_i2c_detect_transfer_size(id);
    ret = cdns_i2c_setclk(id.input_clk, id);
    if (ret) {
    dev_err(&pdev.dev, "invalid SCL clock: %u Hz\n", id.i2c_clk);
    ret = -EINVAL;
    goto err_clk_notifier_unregister;
    }
    ret = devm_request_irq(&pdev.dev, irq, cdns_i2c_isr, 0,
    DRIVER_NAME, id);
    if (ret) {
    dev_err(&pdev.dev, "cannot get irq %d\n", irq);
    goto err_clk_notifier_unregister;
    }
    cdns_i2c_init(id);
    ret = i2c_add_adapter(&id.adap);
    if (ret < 0)
    goto err_clk_notifier_unregister;
    dev_info(&pdev.dev, "%u kHz mmio %08lx irq %d\n",
    id.i2c_clk / 1000, (unsigned long)r_mem.start, irq);
    return 0;
    err_clk_notifier_unregister:
    clk_notifier_unregister(id.clk, &id.clk_rate_change_nb);
    pm_runtime_disable(&pdev.dev);
    pm_runtime_set_suspended(&pdev.dev);
    reset_control_assert(id.reset);
    return ret;
    }
//
// cdns_i2c_remove - Unregister the device after releasing the resources
// @pdev:	Handle to the platform device structure
//
// This function frees all the resources allocated to the device.
//
// Return: 0 always
//
#[no_mangle]
unsafe extern "C" fn cdns_i2c_remove(pdev: *mut platform_device) {
    static void cdns_i2c_remove(struct platform_device *pdev)
    {
    struct cdns_i2c *id = platform_get_drvdata(pdev);
    pm_runtime_disable(&pdev.dev);
    pm_runtime_set_suspended(&pdev.dev);
    pm_runtime_dont_use_autosuspend(&pdev.dev);
    i2c_del_adapter(&id.adap);
    clk_notifier_unregister(id.clk, &id.clk_rate_change_nb);
    reset_control_assert(id.reset);
    }
//
// cdns_i2c_shutdown - Prepare I2C controller for system shutdown
// @pdev:	Handle to the platform device structure
//
// Mark the adapter as suspended and reset the controller to ensure a clean
// handoff during system reboot or kexec.
//
#[no_mangle]
unsafe extern "C" fn cdns_i2c_shutdown(pdev: *mut platform_device) {
    static void cdns_i2c_shutdown(struct platform_device *pdev)
    {
    struct cdns_i2c *id = platform_get_drvdata(pdev);
// Mark the adapter as suspended to prevent further I2C transfers
    i2c_mark_adapter_suspended(&id.adap);
// Reset the controller state if active - clocks are disabled when suspended
    if (!pm_runtime_status_suspended(&pdev.dev))
    cdns_i2c_master_reset(&id.adap);
    }
    static struct platform_driver cdns_i2c_drv = {
    .driver = {
    .name  = DRIVER_NAME,
    .of_match_table = cdns_i2c_of_match,
    .pm = &cdns_i2c_dev_pm_ops,
    },
    .probe  = cdns_i2c_probe,
    .remove = cdns_i2c_remove,
    .shutdown = cdns_i2c_shutdown,
    };
    module_platform_driver(cdns_i2c_drv);
    MODULE_AUTHOR("Xilinx Inc.");
    MODULE_DESCRIPTION("Cadence I2C bus driver");
    MODULE_LICENSE("GPL");
