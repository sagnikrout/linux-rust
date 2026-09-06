//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-img-scb.c
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
// I2C adapter for the IMG Serial Control Bus (SCB) IP block.
//
// Copyright (C) 2009, 2010, 2012, 2014 Imagination Technologies Ltd.
//
// There are three ways that this I2C controller can be driven:
//
// - Raw control of the SDA and SCK signals.
//
// This corresponds to MODE_RAW, which takes control of the signals
// directly for a certain number of clock cycles (the INT_TIMING
// interrupt can be used for timing).
//
// - Atomic commands. A low level I2C symbol (such as generate
// start/stop/ack/nack bit, generate byte, receive byte, and receive
// ACK) is given to the hardware, with detection of completion by bits
// in the LINESTAT register.
//
// This mode of operation is used by MODE_ATOMIC, which uses an I2C
// state machine in the interrupt handler to compose/react to I2C
// transactions using atomic mode commands, and also by MODE_SEQUENCE,
// which emits a simple fixed sequence of atomic mode commands.
//
// Due to software control, the use of atomic commands usually results
// in suboptimal use of the bus, with gaps between the I2C symbols while
// the driver decides what to do next.
//
// - Automatic mode. A bus address, and whether to read/write is
// specified, and the hardware takes care of the I2C state machine,
// using a FIFO to send/receive bytes of data to an I2C slave. The
// driver just has to keep the FIFO drained or filled in response to the
// appropriate FIFO interrupts.
//
// This corresponds to MODE_AUTOMATIC, which manages the FIFOs and deals
// with control of repeated start bits between I2C messages.
//
// Use of automatic mode and the FIFO can make much more efficient use
// of the bus compared to individual atomic commands, with potentially
// no wasted time between I2C symbols or I2C messages.
//
// In most cases MODE_AUTOMATIC is used, however if any of the messages in
// a transaction are zero byte writes (e.g. used by i2cdetect for probing
// the bus), MODE_ATOMIC must be used since automatic mode is normally
// started by the writing of data into the FIFO.
//
// The other modes are used in specific circumstances where MODE_ATOMIC and
// MODE_AUTOMATIC aren't appropriate. MODE_RAW is used to implement a bus
// recovery routine. MODE_SEQUENCE is used to reset the bus and make sure
// it is in a sane state.
//
// Notice that the driver implements a timer-based timeout mechanism.
// The reason for this mechanism is to reduce the number of interrupts
// received in automatic mode.
//
// The driver would get a slave event and transaction done interrupts for
// each atomic mode command that gets completed. However, these events are
// not needed in automatic mode, becase those atomic mode commands are
// managed automatically by the hardware.
//
// In practice, normal I2C transactions will be complete well before you
// get the timer interrupt, as the timer is re-scheduled during FIFO
// maintenance and disabled after the transaction is complete.
//
// In this way normal automatic mode operation isn't impacted by
// unnecessary interrupts, but the exceptional abort condition can still be
// detected (with a slight delay).
//

// Register offsets
pub const SCB_STATUS_REG: c_uint = 0x00;
pub const SCB_OVERRIDE_REG: c_uint = 0x04;
pub const SCB_READ_ADDR_REG: c_uint = 0x08;
pub const SCB_READ_COUNT_REG: c_uint = 0x0c;
pub const SCB_WRITE_ADDR_REG: c_uint = 0x10;
pub const SCB_READ_DATA_REG: c_uint = 0x14;
pub const SCB_WRITE_DATA_REG: c_uint = 0x18;
pub const SCB_FIFO_STATUS_REG: c_uint = 0x1c;
pub const SCB_CONTROL_SOFT_RESET: c_uint = 0x1f;
pub const SCB_CLK_SET_REG: c_uint = 0x3c;
pub const SCB_INT_STATUS_REG: c_uint = 0x40;
pub const SCB_INT_CLEAR_REG: c_uint = 0x44;
pub const SCB_INT_MASK_REG: c_uint = 0x48;
pub const SCB_CONTROL_REG: c_uint = 0x4c;
pub const SCB_TIME_TPL_REG: c_uint = 0x50;
pub const SCB_TIME_TPH_REG: c_uint = 0x54;
pub const SCB_TIME_TP2S_REG: c_uint = 0x58;
pub const SCB_TIME_TBI_REG: c_uint = 0x60;
pub const SCB_TIME_TSL_REG: c_uint = 0x64;
pub const SCB_TIME_TDL_REG: c_uint = 0x68;
pub const SCB_TIME_TSDL_REG: c_uint = 0x6c;
pub const SCB_TIME_TSDH_REG: c_uint = 0x70;
pub const SCB_READ_XADDR_REG: c_uint = 0x74;
pub const SCB_WRITE_XADDR_REG: c_uint = 0x78;
pub const SCB_WRITE_COUNT_REG: c_uint = 0x7c;
pub const SCB_CORE_REV_REG: c_uint = 0x80;
pub const SCB_TIME_TCKH_REG: c_uint = 0x84;
pub const SCB_TIME_TCKL_REG: c_uint = 0x88;
pub const SCB_FIFO_FLUSH_REG: c_uint = 0x8c;
pub const SCB_READ_FIFO_REG: c_uint = 0x94;
pub const SCB_CLEAR_REG: c_uint = 0x98;
// SCB_CONTROL_REG bits
pub const SCB_CONTROL_CLK_ENABLE: c_uint = 0x1e0;
pub const SCB_CONTROL_TRANSACTION_HALT: c_uint = 0x200;

// SCB_CLK_SET_REG bits

pub const SCB_FILT_INC_MASK: c_uint = 0x7f;
pub const SCB_FILT_INC_SHIFT: c_int = 16;
pub const SCB_INC_MASK: c_uint = 0x7f;
pub const SCB_INC_SHIFT: c_int = 8;
// SCB_INT_*_REG bits

// Level interrupts need clearing after handling instead of before
pub const INT_LEVEL: c_uint = 0x01e00;
// Don't allow any interrupts while the clock may be off
pub const INT_ENABLE_MASK_INACTIVE: c_uint = 0x00000;
// Interrupt masks for the different driver modes

    INT_SLAVE_EVENT      | \
    INT_ADDR_ACK_ERR     | \
    INT_WRITE_ACK_ERR)

    INT_ADDR_ACK_ERR     | \
    INT_WRITE_ACK_ERR    | \
    INT_FIFO_FULL        | \
    INT_FIFO_FILLING     | \
    INT_FIFO_EMPTY       | \
    INT_MASTER_HALTED    | \
    INT_STOP_DETECTED)

    INT_ADDR_ACK_ERR     | \
    INT_WRITE_ACK_ERR)
// SCB_STATUS_REG fields

pub const LINESTAT_INPUT_DATA: c_uint = 0xff000000;
pub const LINESTAT_INPUT_DATA_SHIFT: c_int = 24;
pub const LINESTAT_CLEAR_SHIFT: c_int = 13;

// SCB_OVERRIDE_REG fields

pub const OVERRIDE_CMD_SHIFT: c_int = 4;
pub const OVERRIDE_CMD_MASK: c_uint = 0x1f;
pub const OVERRIDE_DATA_SHIFT: c_int = 24;

    OVERRIDE_SCLKEN_OVR)

    OVERRIDE_SCLKEN_OVR | \
    OVERRIDE_SCLK_OVR)

    OVERRIDE_SDATEN_OVR)

    OVERRIDE_SDATEN_OVR | \
    OVERRIDE_SDAT_OVR)
// OVERRIDE_CMD values
pub const CMD_PAUSE: c_uint = 0x00;
pub const CMD_GEN_DATA: c_uint = 0x01;
pub const CMD_GEN_START: c_uint = 0x02;
pub const CMD_GEN_STOP: c_uint = 0x03;
pub const CMD_GEN_ACK: c_uint = 0x04;
pub const CMD_GEN_NACK: c_uint = 0x05;
pub const CMD_RET_DATA: c_uint = 0x08;
pub const CMD_RET_ACK: c_uint = 0x09;
// Fixed timing values
pub const TIMEOUT_TBI: c_uint = 0x0;
pub const TIMEOUT_TSL: c_uint = 0xffff;
pub const TIMEOUT_TDL: c_uint = 0x0;
// Transaction timeout

//
// Worst incs are 1 (inaccurate) and 16*256 (irregular).
// So a sensible inc is the logarithmic mean: 64 (2^6), which is
// in the middle of the valid range (0-127).
//
pub const SCB_OPT_INC: c_int = 64;
// Setup the clock enable filtering for 25 ns
pub const SCB_FILT_GLITCH: c_int = 25;
//
// Bits to return from interrupt handler functions for different modes.
// This delays completion until we've finished with the registers, so that the
// function waiting for completion can safely disable the clock to save power.
//

pub const ISR_STATUS_M: c_uint = 0x0000ffff	/* contains +ve errno */;

    enum img_i2c_mode {
    MODE_INACTIVE,
    MODE_RAW,
    MODE_ATOMIC,
    MODE_AUTOMATIC,
    MODE_SEQUENCE,
    MODE_FATAL,
    MODE_WAITSTOP,
    MODE_SUSPEND,
    };
// Timing parameters for i2c modes (in ns)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_i2c_timings {
    pub name: *const c_char,
    pub max_bitrate: c_uint,
    pub tsdl: unsigned int tckh, tckl, tsdh,,
    pub tph: unsigned int tp2s, tpl,,
}

// The timings array must be ordered from slower to faster
    static struct img_i2c_timings timings[] = {
// Standard mode
    {
    .name = "standard",
    .max_bitrate = I2C_MAX_STANDARD_MODE_FREQ,
    .tckh = 4000,
    .tckl = 4700,
    .tsdh = 4700,
    .tsdl = 8700,
    .tp2s = 4700,
    .tpl = 4700,
    .tph = 4000,
    },
// Fast mode
    {
    .name = "fast",
    .max_bitrate = I2C_MAX_FAST_MODE_FREQ,
    .tckh = 600,
    .tckl = 1300,
    .tsdh = 600,
    .tsdl = 1200,
    .tp2s = 1300,
    .tpl = 600,
    .tph = 600,
    },
    };
// Reset dance
    static u8 img_i2c_reset_seq[] = { CMD_GEN_START,
    CMD_GEN_DATA, 0xff,
    CMD_RET_ACK,
    CMD_GEN_START,
    CMD_GEN_STOP,
    0 };
// Just issue a stop (after an abort condition)
    static u8 img_i2c_stop_seq[] = {  CMD_GEN_STOP,
    0 };
// We're interested in different interrupts depending on the mode
    static unsigned int img_i2c_int_enable_by_mode[] = {
    [MODE_INACTIVE]  = INT_ENABLE_MASK_INACTIVE,
    [MODE_RAW]       = INT_ENABLE_MASK_RAW,
    [MODE_ATOMIC]    = INT_ENABLE_MASK_ATOMIC,
    [MODE_AUTOMATIC] = INT_ENABLE_MASK_AUTOMATIC,
    [MODE_SEQUENCE]  = INT_ENABLE_MASK_ATOMIC,
    [MODE_FATAL]     = 0,
    [MODE_WAITSTOP]  = INT_ENABLE_MASK_WAITSTOP,
    [MODE_SUSPEND]   = 0,
    };
// Atomic command names
    static const char * const img_i2c_atomic_cmd_names[] = {
    [CMD_PAUSE]	= "PAUSE",
    [CMD_GEN_DATA]	= "GEN_DATA",
    [CMD_GEN_START]	= "GEN_START",
    [CMD_GEN_STOP]	= "GEN_STOP",
    [CMD_GEN_ACK]	= "GEN_ACK",
    [CMD_GEN_NACK]	= "GEN_NACK",
    [CMD_RET_DATA]	= "RET_DATA",
    [CMD_RET_ACK]	= "RET_ACK",
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_i2c {
    pub adap: i2c_adapter,
    pub base: *mut void __iomem,
//
// The scb core clock is used to get the input frequency, and to disable
// it after every set of transactions to save some power.
//
    pub sys_clk: *mut *mut clk scb_clk,,
    pub bitrate: c_uint,
    pub need_wr_rd_fence: bool,
// state
    pub msg_complete: completion,
    pub /: *mut *mut spinlock_t lock; / lock before doing anything with the state,
    pub msg: i2c_msg,
// After the last transaction, wait for a stop bit
    pub last_msg: bool,
    pub msg_status: c_int,
    pub mode: enum img_i2c_mode,
    pub /: *mut *mut u32 int_enable; / depends on mode,
    pub /: *mut *mut u32 line_status; / line status over command,
//
// To avoid slave event interrupts in automatic mode, use a timer to
// poll the abort condition if we don't get an interrupt for too long.
//
    pub check_timer: timer_list,
    pub t_halt: bool,
// atomic mode state
    pub at_t_done: bool,
    pub at_slave_event: bool,
    pub at_cur_cmd: c_int,
    pub at_cur_data: u8,
// Sequence: either reset or stop. See img_i2c_sequence.
    pub seq: *mut u8,
// raw mode
    pub raw_timeout: c_uint,
}

    static int img_i2c_runtime_suspend(struct device *dev);
    static int img_i2c_runtime_resume(struct device *dev);
#[no_mangle]
unsafe extern "C" fn img_i2c_writel(i2c: *mut img_i2c, offset: u32, value: u32) {
    static void img_i2c_writel(struct img_i2c *i2c, u32 offset, u32 value)
    {
    writel(value, i2c.base + offset);
    }
#[no_mangle]
unsafe extern "C" fn img_i2c_readl(i2c: *mut img_i2c, offset: u32) -> u32 {
    static u32 img_i2c_readl(struct img_i2c *i2c, u32 offset)
    {
    return readl(i2c.base + offset);
    }
//
// The code to read from the master read fifo, and write to the master
// write fifo, checks a bit in an SCB register before every byte to
// ensure that the fifo is not full (write fifo) or empty (read fifo).
// Due to clock domain crossing inside the SCB block the updated value
// of this bit is only visible after 2 cycles.
//
// The scb_wr_rd_fence() function does 2 dummy writes (to the read-only
// revision register), and it's called after reading from or writing to the
// fifos to ensure that subsequent reads of the fifo status bits do not read
// stale values.
//
#[no_mangle]
unsafe extern "C" fn img_i2c_wr_rd_fence(i2c: *mut img_i2c) {
    static void img_i2c_wr_rd_fence(struct img_i2c *i2c)
    {
    if (i2c.need_wr_rd_fence) {
    img_i2c_writel(i2c, SCB_CORE_REV_REG, 0);
    img_i2c_writel(i2c, SCB_CORE_REV_REG, 0);
    }
    }
#[no_mangle]
unsafe extern "C" fn img_i2c_switch_mode(i2c: *mut img_i2c, mode: enum img_i2c_mode) {
    static void img_i2c_switch_mode(struct img_i2c *i2c, enum img_i2c_mode mode)
    {
    i2c.mode = mode;
    i2c.int_enable = img_i2c_int_enable_by_mode[mode];
    i2c.line_status = 0;
    }
#[no_mangle]
unsafe extern "C" fn img_i2c_raw_op(i2c: *mut img_i2c) {
    static void img_i2c_raw_op(struct img_i2c *i2c)
    {
    i2c.raw_timeout = 0;
    img_i2c_writel(i2c, SCB_OVERRIDE_REG,
    OVERRIDE_SCLKEN_OVR |
    OVERRIDE_SDATEN_OVR |
    OVERRIDE_MASTER |
    OVERRIDE_LINE_OVR_EN |
    OVERRIDE_DIRECT |
    ((i2c.at_cur_cmd & OVERRIDE_CMD_MASK) << OVERRIDE_CMD_SHIFT) |
    (i2c.at_cur_data << OVERRIDE_DATA_SHIFT));
    }
    static const char *img_i2c_atomic_op_name(unsigned int cmd)
    {
    if (unlikely(cmd >= ARRAY_SIZE(img_i2c_atomic_cmd_names)))
    return "UNKNOWN";
    return img_i2c_atomic_cmd_names[cmd];
    }
// Send a single atomic mode command to the hardware
#[no_mangle]
unsafe extern "C" fn img_i2c_atomic_op(i2c: *mut img_i2c, cmd: c_int, data: u8) {
    static void img_i2c_atomic_op(struct img_i2c *i2c, int cmd, u8 data)
    {
    i2c.at_cur_cmd = cmd;
    i2c.at_cur_data = data;
// work around lack of data setup time when generating data
    if (cmd == CMD_GEN_DATA && i2c.mode == MODE_ATOMIC) {
    let mut line_status: u32 = img_i2c_readl(i2c, SCB_STATUS_REG);
    if (line_status & LINESTAT_SDAT_LINE_STATUS && !(data & 0x80)) {
// hold the data line down for a moment
    img_i2c_switch_mode(i2c, MODE_RAW);
    img_i2c_raw_op(i2c);
    return;
    }
    }
    dev_dbg(i2c.adap.dev.parent,
    "atomic cmd=%s (%d) data=%#x\n",
    img_i2c_atomic_op_name(cmd), cmd, data);
    i2c.at_t_done = (cmd == CMD_RET_DATA || cmd == CMD_RET_ACK);
    i2c.at_slave_event = false;
    i2c.line_status = 0;
    img_i2c_writel(i2c, SCB_OVERRIDE_REG,
    ((cmd & OVERRIDE_CMD_MASK) << OVERRIDE_CMD_SHIFT) |
    OVERRIDE_MASTER |
    OVERRIDE_DIRECT |
    (data << OVERRIDE_DATA_SHIFT));
    }
// Start a transaction in atomic mode
#[no_mangle]
unsafe extern "C" fn img_i2c_atomic_start(i2c: *mut img_i2c) {
    static void img_i2c_atomic_start(struct img_i2c *i2c)
    {
    img_i2c_switch_mode(i2c, MODE_ATOMIC);
    img_i2c_writel(i2c, SCB_INT_MASK_REG, i2c.int_enable);
    img_i2c_atomic_op(i2c, CMD_GEN_START, 0x00);
    }
#[no_mangle]
unsafe extern "C" fn img_i2c_soft_reset(i2c: *mut img_i2c) {
    static void img_i2c_soft_reset(struct img_i2c *i2c)
    {
    i2c.t_halt = false;
    img_i2c_writel(i2c, SCB_CONTROL_REG, 0);
    img_i2c_writel(i2c, SCB_CONTROL_REG,
    SCB_CONTROL_CLK_ENABLE | SCB_CONTROL_SOFT_RESET);
    }
//
// Enable or release transaction halt for control of repeated starts.
// In version 3.3 of the IP when transaction halt is set, an interrupt
// will be generated after each byte of a transfer instead of after
// every transfer but before the stop bit.
// Due to this behaviour we have to be careful that every time we
// release the transaction halt we have to re-enable it straight away
// so that we only process a single byte, not doing so will result in
// all remaining bytes been processed and a stop bit being issued,
// which will prevent us having a repeated start.
//
#[no_mangle]
unsafe extern "C" fn img_i2c_transaction_halt(i2c: *mut img_i2c, t_halt: bool) {
    static void img_i2c_transaction_halt(struct img_i2c *i2c, bool t_halt)
    {
    u32 val;
    if (i2c.t_halt == t_halt)
    return;
    i2c.t_halt = t_halt;
    val = img_i2c_readl(i2c, SCB_CONTROL_REG);
    if (t_halt)
    val |= SCB_CONTROL_TRANSACTION_HALT;
    else
    val &= ~SCB_CONTROL_TRANSACTION_HALT;
    img_i2c_writel(i2c, SCB_CONTROL_REG, val);
    }
// Drain data from the FIFO into the buffer (automatic mode)
#[no_mangle]
unsafe extern "C" fn img_i2c_read_fifo(i2c: *mut img_i2c) {
    static void img_i2c_read_fifo(struct img_i2c *i2c)
    {
    while (i2c.msg.len) {
    u32 fifo_status;
    u8 data;
    img_i2c_wr_rd_fence(i2c);
    fifo_status = img_i2c_readl(i2c, SCB_FIFO_STATUS_REG);
    if (fifo_status & FIFO_READ_EMPTY)
    break;
    data = img_i2c_readl(i2c, SCB_READ_DATA_REG);
// i2c->msg.buf = data;
    img_i2c_writel(i2c, SCB_READ_FIFO_REG, 0xff);
    i2c.msg.len--;
    i2c.msg.buf++;
    }
    }
// Fill the FIFO with data from the buffer (automatic mode)
#[no_mangle]
unsafe extern "C" fn img_i2c_write_fifo(i2c: *mut img_i2c) {
    static void img_i2c_write_fifo(struct img_i2c *i2c)
    {
    while (i2c.msg.len) {
    u32 fifo_status;
    img_i2c_wr_rd_fence(i2c);
    fifo_status = img_i2c_readl(i2c, SCB_FIFO_STATUS_REG);
    if (fifo_status & FIFO_WRITE_FULL)
    break;
    img_i2c_writel(i2c, SCB_WRITE_DATA_REG, *i2c.msg.buf);
    i2c.msg.len--;
    i2c.msg.buf++;
    }
// Disable fifo emptying interrupt if nothing more to write
    if (!i2c.msg.len)
    i2c.int_enable &= ~INT_FIFO_EMPTYING;
    }
// Start a read transaction in automatic mode
#[no_mangle]
unsafe extern "C" fn img_i2c_read(i2c: *mut img_i2c) {
    static void img_i2c_read(struct img_i2c *i2c)
    {
    img_i2c_switch_mode(i2c, MODE_AUTOMATIC);
    if (!i2c.last_msg)
    i2c.int_enable |= INT_SLAVE_EVENT;
    img_i2c_writel(i2c, SCB_INT_MASK_REG, i2c.int_enable);
    img_i2c_writel(i2c, SCB_READ_ADDR_REG, i2c.msg.addr);
    img_i2c_writel(i2c, SCB_READ_COUNT_REG, i2c.msg.len);
    mod_timer(&i2c.check_timer, jiffies + msecs_to_jiffies(1));
    }
// Start a write transaction in automatic mode
#[no_mangle]
unsafe extern "C" fn img_i2c_write(i2c: *mut img_i2c) {
    static void img_i2c_write(struct img_i2c *i2c)
    {
    img_i2c_switch_mode(i2c, MODE_AUTOMATIC);
    if (!i2c.last_msg)
    i2c.int_enable |= INT_SLAVE_EVENT;
    img_i2c_writel(i2c, SCB_WRITE_ADDR_REG, i2c.msg.addr);
    img_i2c_writel(i2c, SCB_WRITE_COUNT_REG, i2c.msg.len);
    mod_timer(&i2c.check_timer, jiffies + msecs_to_jiffies(1));
    img_i2c_write_fifo(i2c);
// img_i2c_write_fifo() may modify int_enable
    img_i2c_writel(i2c, SCB_INT_MASK_REG, i2c.int_enable);
    }
//
// Indicate that the transaction is complete. This is called from the
// ISR to wake up the waiting thread, after which the ISR must not
// access any more SCB registers.
//
#[no_mangle]
unsafe extern "C" fn img_i2c_complete_transaction(i2c: *mut img_i2c, status: c_int) {
    static void img_i2c_complete_transaction(struct img_i2c *i2c, int status)
    {
    img_i2c_switch_mode(i2c, MODE_INACTIVE);
    if (status) {
    i2c.msg_status = status;
    img_i2c_transaction_halt(i2c, false);
    }
    complete(&i2c.msg_complete);
    }
    static unsigned int img_i2c_raw_atomic_delay_handler(struct img_i2c *i2c,
    u32 int_status, u32 line_status)
    {
// Stay in raw mode for this, so we don't just loop infinitely
    img_i2c_atomic_op(i2c, i2c.at_cur_cmd, i2c.at_cur_data);
    img_i2c_switch_mode(i2c, MODE_ATOMIC);
    return 0;
    }
    static unsigned int img_i2c_raw(struct img_i2c *i2c, u32 int_status,
    u32 line_status)
    {
    if (int_status & INT_TIMING) {
    if (i2c.raw_timeout == 0)
    return img_i2c_raw_atomic_delay_handler(i2c,
    int_status, line_status);
    --i2c.raw_timeout;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn img_i2c_sequence(i2c: *mut img_i2c, int_status: u32) -> c_uint {
    static unsigned int img_i2c_sequence(struct img_i2c *i2c, u32 int_status)
    {
    static const unsigned int continue_bits[] = {
    [CMD_GEN_START] = LINESTAT_START_BIT_DET,
    [CMD_GEN_DATA]  = LINESTAT_INPUT_HELD_V,
    [CMD_RET_ACK]   = LINESTAT_ACK_DET | LINESTAT_NACK_DET,
    [CMD_RET_DATA]  = LINESTAT_INPUT_HELD_V,
    [CMD_GEN_STOP]  = LINESTAT_STOP_BIT_DET,
    };
    let mut next_cmd: c_int = -1;
    let mut next_data: u8 = 0x00;
    if (int_status & INT_SLAVE_EVENT)
    i2c.at_slave_event = true;
    if (int_status & INT_TRANSACTION_DONE)
    i2c.at_t_done = true;
    if (!i2c.at_slave_event || !i2c.at_t_done)
    return 0;
// wait if no continue bits are set
    if (i2c.at_cur_cmd >= 0 &&
    i2c.at_cur_cmd < ARRAY_SIZE(continue_bits)) {
    let mut cont_bits: c_uint = continue_bits[i2c.at_cur_cmd];
    if (cont_bits) {
    cont_bits |= LINESTAT_ABORT_DET;
    if (!(i2c.line_status & cont_bits))
    return 0;
    }
    }
// follow the sequence of commands in i2c->seq
    next_cmd = *i2c.seq;
// stop on a nil
    if (!next_cmd) {
    img_i2c_writel(i2c, SCB_OVERRIDE_REG, 0);
    return ISR_COMPLETE(0);
    }
// when generating data, the next byte is the data
    if (next_cmd == CMD_GEN_DATA) {
    ++i2c.seq;
    next_data = *i2c.seq;
    }
    ++i2c.seq;
    img_i2c_atomic_op(i2c, next_cmd, next_data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn img_i2c_reset_start(i2c: *mut img_i2c) {
    static void img_i2c_reset_start(struct img_i2c *i2c)
    {
// Initiate the magic dance
    img_i2c_switch_mode(i2c, MODE_SEQUENCE);
    img_i2c_writel(i2c, SCB_INT_MASK_REG, i2c.int_enable);
    i2c.seq = img_i2c_reset_seq;
    i2c.at_slave_event = true;
    i2c.at_t_done = true;
    i2c.at_cur_cmd = -1;
// img_i2c_reset_seq isn't empty so the following won't fail
    img_i2c_sequence(i2c, 0);
    }
#[no_mangle]
unsafe extern "C" fn img_i2c_stop_start(i2c: *mut img_i2c) {
    static void img_i2c_stop_start(struct img_i2c *i2c)
    {
// Initiate a stop bit sequence
    img_i2c_switch_mode(i2c, MODE_SEQUENCE);
    img_i2c_writel(i2c, SCB_INT_MASK_REG, i2c.int_enable);
    i2c.seq = img_i2c_stop_seq;
    i2c.at_slave_event = true;
    i2c.at_t_done = true;
    i2c.at_cur_cmd = -1;
// img_i2c_stop_seq isn't empty so the following won't fail
    img_i2c_sequence(i2c, 0);
    }
    static unsigned int img_i2c_atomic(struct img_i2c *i2c,
    u32 int_status,
    u32 line_status)
    {
    let mut next_cmd: c_int = -1;
    let mut next_data: u8 = 0x00;
    if (int_status & INT_SLAVE_EVENT)
    i2c.at_slave_event = true;
    if (int_status & INT_TRANSACTION_DONE)
    i2c.at_t_done = true;
    if (!i2c.at_slave_event || !i2c.at_t_done)
    goto next_atomic_cmd;
    if (i2c.line_status & LINESTAT_ABORT_DET) {
    dev_dbg(i2c.adap.dev.parent, "abort condition detected\n");
    next_cmd = CMD_GEN_STOP;
    i2c.msg_status = -EIO;
    goto next_atomic_cmd;
    }
// i2c->at_cur_cmd may have completed
    switch (i2c.at_cur_cmd) {
    case CMD_GEN_START:
    next_cmd = CMD_GEN_DATA;
    next_data = i2c_8bit_addr_from_msg(&i2c.msg);
    break;
    case CMD_GEN_DATA:
    if (i2c.line_status & LINESTAT_INPUT_HELD_V)
    next_cmd = CMD_RET_ACK;
    break;
    case CMD_RET_ACK:
    if (i2c.line_status & LINESTAT_ACK_DET ||
    (i2c.line_status & LINESTAT_NACK_DET &&
    i2c.msg.flags & I2C_M_IGNORE_NAK)) {
    if (i2c.msg.len == 0) {
    next_cmd = CMD_GEN_STOP;
    } else if (i2c.msg.flags & I2C_M_RD) {
    next_cmd = CMD_RET_DATA;
    } else {
    next_cmd = CMD_GEN_DATA;
    next_data = *i2c.msg.buf;
    --i2c.msg.len;
    ++i2c.msg.buf;
    }
    } else if (i2c.line_status & LINESTAT_NACK_DET) {
    i2c.msg_status = -EIO;
    next_cmd = CMD_GEN_STOP;
    }
    break;
    case CMD_RET_DATA:
    if (i2c.line_status & LINESTAT_INPUT_HELD_V) {
// i2c->msg.buf = (i2c->line_status &
    LINESTAT_INPUT_DATA)
    >> LINESTAT_INPUT_DATA_SHIFT;
    --i2c.msg.len;
    ++i2c.msg.buf;
    if (i2c.msg.len)
    next_cmd = CMD_GEN_ACK;
    else
    next_cmd = CMD_GEN_NACK;
    }
    break;
    case CMD_GEN_ACK:
    if (i2c.line_status & LINESTAT_ACK_DET) {
    next_cmd = CMD_RET_DATA;
    } else {
    i2c.msg_status = -EIO;
    next_cmd = CMD_GEN_STOP;
    }
    break;
    case CMD_GEN_NACK:
    next_cmd = CMD_GEN_STOP;
    break;
    case CMD_GEN_STOP:
    img_i2c_writel(i2c, SCB_OVERRIDE_REG, 0);
    return ISR_COMPLETE(0);
    default:
    dev_err(i2c.adap.dev.parent, "bad atomic command %d\n",
    i2c.at_cur_cmd);
    i2c.msg_status = -EIO;
    next_cmd = CMD_GEN_STOP;
    break;
    }
    next_atomic_cmd:
    if (next_cmd != -1) {
// don't actually stop unless we're the last transaction
    if (next_cmd == CMD_GEN_STOP && !i2c.msg_status &&
    !i2c.last_msg)
    return ISR_COMPLETE(0);
    img_i2c_atomic_op(i2c, next_cmd, next_data);
    }
    return 0;
    }
//
// Timer function to check if something has gone wrong in automatic mode (so we
// don't have to handle so many interrupts just to catch an exception).
//
#[no_mangle]
unsafe extern "C" fn img_i2c_check_timer(t: *mut timer_list) {
    static void img_i2c_check_timer(struct timer_list *t)
    {
    struct img_i2c *i2c = timer_container_of(i2c, t, check_timer);
    unsigned long flags;
    unsigned int line_status;
    spin_lock_irqsave(&i2c.lock, flags);
    line_status = img_i2c_readl(i2c, SCB_STATUS_REG);
// check for an abort condition
    if (line_status & LINESTAT_ABORT_DET) {
    dev_dbg(i2c.adap.dev.parent,
    "abort condition detected by check timer\n");
// enable slave event interrupt mask to trigger irq
    img_i2c_writel(i2c, SCB_INT_MASK_REG,
    i2c.int_enable | INT_SLAVE_EVENT);
    }
    spin_unlock_irqrestore(&i2c.lock, flags);
    }
    static unsigned int img_i2c_auto(struct img_i2c *i2c,
    unsigned int int_status,
    unsigned int line_status)
    {
    if (int_status & (INT_WRITE_ACK_ERR | INT_ADDR_ACK_ERR))
    return ISR_COMPLETE(EIO);
    if (line_status & LINESTAT_ABORT_DET) {
    dev_dbg(i2c.adap.dev.parent, "abort condition detected\n");
// empty the read fifo
    if ((i2c.msg.flags & I2C_M_RD) &&
    (int_status & INT_FIFO_FULL_FILLING))
    img_i2c_read_fifo(i2c);
// use atomic mode and try to force a stop bit
    i2c.msg_status = -EIO;
    img_i2c_stop_start(i2c);
    return 0;
    }
// Enable transaction halt on start bit
    if (!i2c.last_msg && line_status & LINESTAT_START_BIT_DET) {
    img_i2c_transaction_halt(i2c, !i2c.last_msg);
// we're no longer interested in the slave event
    i2c.int_enable &= ~INT_SLAVE_EVENT;
    }
    mod_timer(&i2c.check_timer, jiffies + msecs_to_jiffies(1));
    if (int_status & INT_STOP_DETECTED) {
// Drain remaining data in FIFO and complete transaction
    if (i2c.msg.flags & I2C_M_RD)
    img_i2c_read_fifo(i2c);
    return ISR_COMPLETE(0);
    }
    if (i2c.msg.flags & I2C_M_RD) {
    if (int_status & (INT_FIFO_FULL_FILLING | INT_MASTER_HALTED)) {
    img_i2c_read_fifo(i2c);
    if (i2c.msg.len == 0)
    return ISR_WAITSTOP;
    }
    } else {
    if (int_status & (INT_FIFO_EMPTY | INT_MASTER_HALTED)) {
    if ((int_status & INT_FIFO_EMPTY) &&
    i2c.msg.len == 0)
    return ISR_WAITSTOP;
    img_i2c_write_fifo(i2c);
    }
    }
    if (int_status & INT_MASTER_HALTED) {
//
// Release and then enable transaction halt, to
// allow only a single byte to proceed.
//
    img_i2c_transaction_halt(i2c, false);
    img_i2c_transaction_halt(i2c, !i2c.last_msg);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn img_i2c_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t img_i2c_isr(int irq, void *dev_id)
    {
    struct img_i2c *i2c = dev_id;
    u32 int_status, line_status;
// We handle transaction completion AFTER accessing registers
    unsigned int hret;
// Read interrupt status register.
    int_status = img_i2c_readl(i2c, SCB_INT_STATUS_REG);
// Clear detected interrupts.
    img_i2c_writel(i2c, SCB_INT_CLEAR_REG, int_status);
//
// Read line status and clear it until it actually is clear.  We have
// to be careful not to lose any line status bits that get latched.
//
    line_status = img_i2c_readl(i2c, SCB_STATUS_REG);
    if (line_status & LINESTAT_LATCHED) {
    img_i2c_writel(i2c, SCB_CLEAR_REG,
    (line_status & LINESTAT_LATCHED)
    >> LINESTAT_CLEAR_SHIFT);
    img_i2c_wr_rd_fence(i2c);
    }
    spin_lock(&i2c.lock);
// Keep track of line status bits received
    i2c.line_status &= ~LINESTAT_INPUT_DATA;
    i2c.line_status |= line_status;
//
// Certain interrupts indicate that sclk low timeout is not
// a problem. If any of these are set, just continue.
//
    if ((int_status & INT_SCLK_LOW_TIMEOUT) &&
    !(int_status & (INT_SLAVE_EVENT |
    INT_FIFO_EMPTY |
    INT_FIFO_FULL))) {
    dev_crit(i2c.adap.dev.parent,
    "fatal: clock low timeout occurred %s addr 0x%02x\n",
    (i2c.msg.flags & I2C_M_RD) ? "reading" : "writing",
    i2c.msg.addr);
    hret = ISR_FATAL(EIO);
    goto out;
    }
    if (i2c.mode == MODE_ATOMIC)
    hret = img_i2c_atomic(i2c, int_status, line_status);
#[no_mangle]
pub unsafe extern "C" fn if(MODE_AUTOMATIC: i2c->mode ==) -> else {
    else if (i2c.mode == MODE_AUTOMATIC)
    hret = img_i2c_auto(i2c, int_status, line_status);
#[no_mangle]
pub unsafe extern "C" fn if(MODE_SEQUENCE: i2c->mode ==) -> else {
    else if (i2c.mode == MODE_SEQUENCE)
    hret = img_i2c_sequence(i2c, int_status);
    else if (i2c.mode == MODE_WAITSTOP && (int_status & INT_SLAVE_EVENT) &&
    (line_status & LINESTAT_STOP_BIT_DET))
    hret = ISR_COMPLETE(0);
#[no_mangle]
pub unsafe extern "C" fn if(MODE_RAW: i2c->mode ==) -> else {
    else if (i2c.mode == MODE_RAW)
    hret = img_i2c_raw(i2c, int_status, line_status);
    else
    hret = 0;
// Clear detected level interrupts.
    img_i2c_writel(i2c, SCB_INT_CLEAR_REG, int_status & INT_LEVEL);
    out:
    if (hret & ISR_WAITSTOP) {
//
// Only wait for stop on last message.
// Also we may already have detected the stop bit.
//
    if (!i2c.last_msg || i2c.line_status & LINESTAT_STOP_BIT_DET)
    hret = ISR_COMPLETE(0);
    else
    img_i2c_switch_mode(i2c, MODE_WAITSTOP);
    }
// now we've finished using regs, handle transaction completion
    if (hret & ISR_COMPLETE_M) {
    let mut status: c_int = -(hret & ISR_STATUS_M);
    img_i2c_complete_transaction(i2c, status);
    if (hret & ISR_FATAL_M)
    img_i2c_switch_mode(i2c, MODE_FATAL);
    }
// Enable interrupts (int_enable may be altered by changing mode)
    img_i2c_writel(i2c, SCB_INT_MASK_REG, i2c.int_enable);
    spin_unlock(&i2c.lock);
    return IRQ_HANDLED;
    }
// Force a bus reset sequence and wait for it to complete
#[no_mangle]
unsafe extern "C" fn img_i2c_reset_bus(i2c: *mut img_i2c) -> c_int {
    static int img_i2c_reset_bus(struct img_i2c *i2c)
    {
    unsigned long flags;
    unsigned long time_left;
    spin_lock_irqsave(&i2c.lock, flags);
    reinit_completion(&i2c.msg_complete);
    img_i2c_reset_start(i2c);
    spin_unlock_irqrestore(&i2c.lock, flags);
    time_left = wait_for_completion_timeout(&i2c.msg_complete,
    IMG_I2C_TIMEOUT);
    if (time_left == 0)
    return -ETIMEDOUT;
    return 0;
    }
    static int img_i2c_xfer(struct i2c_adapter *adap, struct i2c_msg *msgs,
    int num)
    {
    struct img_i2c *i2c = i2c_get_adapdata(adap);
    let mut atomic: bool = false;
    int i, ret;
    unsigned long time_left;
    if (i2c.mode == MODE_SUSPEND) {
    WARN(1, "refusing to service transaction in suspended state\n");
    return -EIO;
    }
    if (i2c.mode == MODE_FATAL)
    return -EIO;
    for (i = 0; i < num; i++) {
//
// 0 byte reads are not possible because the slave could try
// and pull the data line low, preventing a stop bit.
//
    if (!msgs[i].len && msgs[i].flags & I2C_M_RD)
    return -EIO;
//
// 0 byte writes are possible and used for probing, but we
// cannot do them in automatic mode, so use atomic mode
// instead.
//
// Also, the I2C_M_IGNORE_NAK mode can only be implemented
// in atomic mode.
//
    if (!msgs[i].len ||
    (msgs[i].flags & I2C_M_IGNORE_NAK))
    atomic = true;
    }
    ret = pm_runtime_resume_and_get(adap.dev.parent);
    if (ret < 0)
    return ret;
    for (i = 0; i < num; i++) {
    struct i2c_msg *msg = &msgs[i];
    unsigned long flags;
    spin_lock_irqsave(&i2c.lock, flags);
//
// Make a copy of the message struct. We mustn't modify the
// original or we'll confuse drivers and i2c-dev.
//
    i2c.msg = *msg;
    i2c.msg_status = 0;
//
// After the last message we must have waited for a stop bit.
// Not waiting can cause problems when the clock is disabled
// before the stop bit is sent, and the linux I2C interface
// requires separate transfers not to joined with repeated
// start.
//
    i2c.last_msg = (i == num - 1);
    reinit_completion(&i2c.msg_complete);
//
// Clear line status and all interrupts before starting a
// transfer, as we may have unserviced interrupts from
// previous transfers that might be handled in the context
// of the new transfer.
//
    img_i2c_writel(i2c, SCB_INT_CLEAR_REG, ~0);
    img_i2c_writel(i2c, SCB_CLEAR_REG, ~0);
    if (atomic) {
    img_i2c_atomic_start(i2c);
    } else {
//
// Enable transaction halt if not the last message in
// the queue so that we can control repeated starts.
//
    img_i2c_transaction_halt(i2c, !i2c.last_msg);
    if (msg.flags & I2C_M_RD)
    img_i2c_read(i2c);
    else
    img_i2c_write(i2c);
//
// Release and then enable transaction halt, to
// allow only a single byte to proceed.
// This doesn't have an effect on the initial transfer
// but will allow the following transfers to start
// processing if the previous transfer was marked as
// complete while the i2c block was halted.
//
    img_i2c_transaction_halt(i2c, false);
    img_i2c_transaction_halt(i2c, !i2c.last_msg);
    }
    spin_unlock_irqrestore(&i2c.lock, flags);
    time_left = wait_for_completion_timeout(&i2c.msg_complete,
    IMG_I2C_TIMEOUT);
    timer_delete_sync(&i2c.check_timer);
    if (time_left == 0)
    i2c.msg_status = -ETIMEDOUT;
    if (i2c.msg_status)
    break;
    }
    pm_runtime_put_autosuspend(adap.dev.parent);
    return i2c.msg_status ? i2c.msg_status : num;
    }
#[no_mangle]
unsafe extern "C" fn img_i2c_func(adap: *mut i2c_adapter) -> u32 {
    static u32 img_i2c_func(struct i2c_adapter *adap)
    {
    return I2C_FUNC_I2C | I2C_FUNC_SMBUS_EMUL;
    }
    static const struct i2c_algorithm img_i2c_algo = {
    .xfer = img_i2c_xfer,
    .functionality = img_i2c_func,
    };
#[no_mangle]
unsafe extern "C" fn img_i2c_init(i2c: *mut img_i2c) -> c_int {
    static int img_i2c_init(struct img_i2c *i2c)
    {
    unsigned int clk_khz, bitrate_khz, clk_period, tckh, tckl, tsdh;
    unsigned int i, data, prescale, inc, int_bitrate, filt;
    struct img_i2c_timings timing;
    u32 rev;
    int ret;
    ret = pm_runtime_resume_and_get(i2c.adap.dev.parent);
    if (ret < 0)
    return ret;
    rev = img_i2c_readl(i2c, SCB_CORE_REV_REG);
    if ((rev & 0x00ffffff) < 0x00020200) {
    dev_info(i2c.adap.dev.parent,
    "Unknown hardware revision (%d.%d.%d.%d)\n",
    (rev >> 24) & 0xff, (rev >> 16) & 0xff,
    (rev >> 8) & 0xff, rev & 0xff);
    pm_runtime_put_autosuspend(i2c.adap.dev.parent);
    return -EINVAL;
    }
// Fencing enabled by default.
    i2c.need_wr_rd_fence = true;
// Determine what mode we're in from the bitrate
    timing = timings[0];
    for (i = 0; i < ARRAY_SIZE(timings); i++) {
    if (i2c.bitrate <= timings[i].max_bitrate) {
    timing = timings[i];
    break;
    }
    }
    if (i2c.bitrate > timings[ARRAY_SIZE(timings) - 1].max_bitrate) {
    dev_warn(i2c.adap.dev.parent,
    "requested bitrate (%u) is higher than the max bitrate supported (%u)\n",
    i2c.bitrate,
    timings[ARRAY_SIZE(timings) - 1].max_bitrate);
    timing = timings[ARRAY_SIZE(timings) - 1];
    i2c.bitrate = timing.max_bitrate;
    }
    bitrate_khz = i2c.bitrate / 1000;
    clk_khz = clk_get_rate(i2c.scb_clk) / 1000;
// Find the prescale that would give us that inc (approx delay = 0)
    prescale = SCB_OPT_INC * clk_khz / (256 * 16 * bitrate_khz);
    prescale = clamp_t(unsigned int, prescale, 1, 8);
    clk_khz /= prescale;
// Setup the clock increment value
    inc = (256 * 16 * bitrate_khz) / clk_khz;
//
// The clock generation logic allows to filter glitches on the bus.
// This filter is able to remove bus glitches shorter than 50ns.
// If the clock enable rate is greater than 20 MHz, no filtering
// is required, so we need to disable it.
// If it's between the 20-40 MHz range, there's no need to divide
// the clock to get a filter.
//
    if (clk_khz < 20000) {
    filt = SCB_FILT_DISABLE;
    } else if (clk_khz < 40000) {
    filt = SCB_FILT_BYPASS;
    } else {
// Calculate filter clock
    filt = (64000 / ((clk_khz / 1000) * SCB_FILT_GLITCH));
// Scale up if needed
    if (64000 % ((clk_khz / 1000) * SCB_FILT_GLITCH))
    inc++;
    if (filt > SCB_FILT_INC_MASK)
    filt = SCB_FILT_INC_MASK;
    filt = (filt & SCB_FILT_INC_MASK) << SCB_FILT_INC_SHIFT;
    }
    data = filt | ((inc & SCB_INC_MASK) << SCB_INC_SHIFT) | (prescale - 1);
    img_i2c_writel(i2c, SCB_CLK_SET_REG, data);
// Obtain the clock period of the fx16 clock in ns
    clk_period = (256 * 1000000) / (clk_khz * inc);
// Calculate the bitrate in terms of internal clock pulses
    int_bitrate = 1000000 / (bitrate_khz * clk_period);
    if ((1000000 % (bitrate_khz * clk_period)) >=
    ((bitrate_khz * clk_period) / 2))
    int_bitrate++;
//
// Setup clock duty cycle, start with 50% and adjust TCKH and TCKL
// values from there if they don't meet minimum timing requirements
//
    tckh = int_bitrate / 2;
    tckl = int_bitrate - tckh;
// Adjust TCKH and TCKL values
    data = DIV_ROUND_UP(timing.tckl, clk_period);
    if (tckl < data) {
    tckl = data;
    tckh = int_bitrate - tckl;
    }
    if (tckh > 0)
    --tckh;
    if (tckl > 0)
    --tckl;
    img_i2c_writel(i2c, SCB_TIME_TCKH_REG, tckh);
    img_i2c_writel(i2c, SCB_TIME_TCKL_REG, tckl);
// Setup TSDH value
    tsdh = DIV_ROUND_UP(timing.tsdh, clk_period);
    if (tsdh > 1)
    data = tsdh - 1;
    else
    data = 0x01;
    img_i2c_writel(i2c, SCB_TIME_TSDH_REG, data);
// This value is used later
    tsdh = data;
// Setup TPL value
    data = timing.tpl / clk_period;
    if (data > 0)
    --data;
    img_i2c_writel(i2c, SCB_TIME_TPL_REG, data);
// Setup TPH value
    data = timing.tph / clk_period;
    if (data > 0)
    --data;
    img_i2c_writel(i2c, SCB_TIME_TPH_REG, data);
// Setup TSDL value to TPL + TSDH + 2
    img_i2c_writel(i2c, SCB_TIME_TSDL_REG, data + tsdh + 2);
// Setup TP2S value
    data = timing.tp2s / clk_period;
    if (data > 0)
    --data;
    img_i2c_writel(i2c, SCB_TIME_TP2S_REG, data);
    img_i2c_writel(i2c, SCB_TIME_TBI_REG, TIMEOUT_TBI);
    img_i2c_writel(i2c, SCB_TIME_TSL_REG, TIMEOUT_TSL);
    img_i2c_writel(i2c, SCB_TIME_TDL_REG, TIMEOUT_TDL);
// Take module out of soft reset and enable clocks
    img_i2c_soft_reset(i2c);
// Disable all interrupts
    img_i2c_writel(i2c, SCB_INT_MASK_REG, 0);
// Clear all interrupts
    img_i2c_writel(i2c, SCB_INT_CLEAR_REG, ~0);
// Clear the scb_line_status events
    img_i2c_writel(i2c, SCB_CLEAR_REG, ~0);
// Enable interrupts
    img_i2c_writel(i2c, SCB_INT_MASK_REG, i2c.int_enable);
// Perform a synchronous sequence to reset the bus
    ret = img_i2c_reset_bus(i2c);
    pm_runtime_put_autosuspend(i2c.adap.dev.parent);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn img_i2c_probe(pdev: *mut platform_device) -> c_int {
    static int img_i2c_probe(struct platform_device *pdev)
    {
    struct device_node *node = pdev.dev.of_node;
    struct img_i2c *i2c;
    int irq, ret;
    u32 val;
    i2c = devm_kzalloc(&pdev.dev, sizeof(struct img_i2c), GFP_KERNEL);
    if (!i2c)
    return -ENOMEM;
    i2c.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(i2c.base))
    return PTR_ERR(i2c.base);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    i2c.sys_clk = devm_clk_get(&pdev.dev, "sys");
    if (IS_ERR(i2c.sys_clk)) {
    dev_err(&pdev.dev, "can't get system clock\n");
    return PTR_ERR(i2c.sys_clk);
    }
    i2c.scb_clk = devm_clk_get(&pdev.dev, "scb");
    if (IS_ERR(i2c.scb_clk)) {
    dev_err(&pdev.dev, "can't get core clock\n");
    return PTR_ERR(i2c.scb_clk);
    }
    ret = devm_request_irq(&pdev.dev, irq, img_i2c_isr, 0,
    pdev.name, i2c);
    if (ret) {
    dev_err(&pdev.dev, "can't request irq %d\n", irq);
    return ret;
    }
// Set up the exception check timer
    timer_setup(&i2c.check_timer, img_i2c_check_timer, 0);
    i2c.bitrate = timings[0].max_bitrate;
    if (!of_property_read_u32(node, "clock-frequency", &val))
    i2c.bitrate = val;
    i2c_set_adapdata(&i2c.adap, i2c);
    i2c.adap.dev.parent = &pdev.dev;
    i2c.adap.dev.of_node = node;
    i2c.adap.owner = THIS_MODULE;
    i2c.adap.algo = &img_i2c_algo;
    i2c.adap.retries = 5;
    i2c.adap.nr = pdev.id;
    snprintf(i2c.adap.name, sizeof(i2c.adap.name), "IMG SCB I2C");
    img_i2c_switch_mode(i2c, MODE_INACTIVE);
    spin_lock_init(&i2c.lock);
    init_completion(&i2c.msg_complete);
    platform_set_drvdata(pdev, i2c);
    pm_runtime_set_autosuspend_delay(&pdev.dev, IMG_I2C_PM_TIMEOUT);
    pm_runtime_use_autosuspend(&pdev.dev);
    pm_runtime_enable(&pdev.dev);
    if (!pm_runtime_enabled(&pdev.dev)) {
    ret = img_i2c_runtime_resume(&pdev.dev);
    if (ret)
    return ret;
    }
    ret = img_i2c_init(i2c);
    if (ret)
    goto rpm_disable;
    ret = i2c_add_numbered_adapter(&i2c.adap);
    if (ret < 0)
    goto rpm_disable;
    return 0;
    rpm_disable:
    if (!pm_runtime_enabled(&pdev.dev))
    img_i2c_runtime_suspend(&pdev.dev);
    pm_runtime_disable(&pdev.dev);
    pm_runtime_dont_use_autosuspend(&pdev.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn img_i2c_remove(dev: *mut platform_device) {
    static void img_i2c_remove(struct platform_device *dev)
    {
    struct img_i2c *i2c = platform_get_drvdata(dev);
    i2c_del_adapter(&i2c.adap);
    pm_runtime_disable(&dev.dev);
    if (!pm_runtime_status_suspended(&dev.dev))
    img_i2c_runtime_suspend(&dev.dev);
    }
#[no_mangle]
unsafe extern "C" fn img_i2c_runtime_suspend(dev: *mut device) -> c_int {
    static int img_i2c_runtime_suspend(struct device *dev)
    {
    struct img_i2c *i2c = dev_get_drvdata(dev);
    clk_disable_unprepare(i2c.scb_clk);
    clk_disable_unprepare(i2c.sys_clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn img_i2c_runtime_resume(dev: *mut device) -> c_int {
    static int img_i2c_runtime_resume(struct device *dev)
    {
    struct img_i2c *i2c = dev_get_drvdata(dev);
    int ret;
    ret = clk_prepare_enable(i2c.sys_clk);
    if (ret) {
    dev_err(dev, "Unable to enable sys clock\n");
    return ret;
    }
    ret = clk_prepare_enable(i2c.scb_clk);
    if (ret) {
    dev_err(dev, "Unable to enable scb clock\n");
    clk_disable_unprepare(i2c.sys_clk);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn img_i2c_suspend(dev: *mut device) -> c_int {
    static int img_i2c_suspend(struct device *dev)
    {
    struct img_i2c *i2c = dev_get_drvdata(dev);
    int ret;
    ret = pm_runtime_force_suspend(dev);
    if (ret)
    return ret;
    img_i2c_switch_mode(i2c, MODE_SUSPEND);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn img_i2c_resume(dev: *mut device) -> c_int {
    static int img_i2c_resume(struct device *dev)
    {
    struct img_i2c *i2c = dev_get_drvdata(dev);
    int ret;
    ret = pm_runtime_force_resume(dev);
    if (ret)
    return ret;
    img_i2c_init(i2c);
    return 0;
    }
    static const struct dev_pm_ops img_i2c_pm = {
    RUNTIME_PM_OPS(img_i2c_runtime_suspend, img_i2c_runtime_resume, core::ptr::null_mut())
    SYSTEM_SLEEP_PM_OPS(img_i2c_suspend, img_i2c_resume)
    };
    static const struct of_device_id img_scb_i2c_match[] = {
    { .compatible = "img,scb-i2c" },
    { }
    };
    MODULE_DEVICE_TABLE(of, img_scb_i2c_match);
    static struct platform_driver img_scb_i2c_driver = {
    .driver = {
    .name		= "img-i2c-scb",
    .of_match_table	= img_scb_i2c_match,
    .pm		= pm_ptr(&img_i2c_pm),
    },
    .probe = img_i2c_probe,
    .remove = img_i2c_remove,
    };
    module_platform_driver(img_scb_i2c_driver);
    MODULE_AUTHOR("James Hogan <jhogan@kernel.org>");
    MODULE_DESCRIPTION("IMG host I2C driver");
    MODULE_LICENSE("GPL v2");
