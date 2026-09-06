//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-axxia.c
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
// This driver implements I2C master functionality using the LSI API2C
// controller.
//
// NOTE: The controller has a limitation in that it can only do transfers of
// maximum 255 bytes at a time. If a larger transfer is attempted, error code
// (-EINVAL) is returned.
//

pub const SCL_WAIT_TIMEOUT_NS: c_int = 25000000;

pub const FIFO_SIZE: c_int = 8;
pub const SEQ_LEN: c_int = 2;
pub const GLOBAL_CONTROL: c_uint = 0x00;

pub const INTERRUPT_STATUS: c_uint = 0x04;
pub const INTERRUPT_ENABLE: c_uint = 0x08;

pub const WAIT_TIMER_CONTROL: c_uint = 0x0c;

pub const IBML_TIMEOUT: c_uint = 0x10;
pub const IBML_LOW_MEXT: c_uint = 0x14;
pub const IBML_LOW_SEXT: c_uint = 0x18;
pub const TIMER_CLOCK_DIV: c_uint = 0x1c;
pub const I2C_BUS_MONITOR: c_uint = 0x20;

pub const SOFT_RESET: c_uint = 0x24;
pub const MST_COMMAND: c_uint = 0x28;

pub const MST_RX_XFER: c_uint = 0x2c;
pub const MST_TX_XFER: c_uint = 0x30;
pub const MST_ADDR_1: c_uint = 0x34;
pub const MST_ADDR_2: c_uint = 0x38;
pub const MST_DATA: c_uint = 0x3c;
pub const MST_TX_FIFO: c_uint = 0x40;
pub const MST_RX_FIFO: c_uint = 0x44;
pub const MST_INT_ENABLE: c_uint = 0x48;
pub const MST_INT_STATUS: c_uint = 0x4c;

    MST_STATUS_ND)

    MST_STATUS_AL  | \
    MST_STATUS_IP)
pub const MST_TX_BYTES_XFRD: c_uint = 0x50;
pub const MST_RX_BYTES_XFRD: c_uint = 0x54;
pub const SLV_ADDR_DEC_CTL: c_uint = 0x58;

pub const SLV_ADDR_1: c_uint = 0x5c;
pub const SLV_ADDR_2: c_uint = 0x60;
pub const SLV_RX_CTL: c_uint = 0x64;

pub const SLV_DATA: c_uint = 0x68;
pub const SLV_RX_FIFO: c_uint = 0x6c;

pub const SLV_INT_ENABLE: c_uint = 0x70;
pub const SLV_INT_STATUS: c_uint = 0x74;

pub const SLV_READ_DUMMY: c_uint = 0x78;
pub const SCL_HIGH_PERIOD: c_uint = 0x80;
pub const SCL_LOW_PERIOD: c_uint = 0x84;
pub const SPIKE_FLTR_LEN: c_uint = 0x88;
pub const SDA_SETUP_TIME: c_uint = 0x8c;
pub const SDA_HOLD_TIME: c_uint = 0x90;
//
// struct axxia_i2c_dev - I2C device context
// @base: pointer to register struct
// @msg: pointer to current message
// @msg_r: pointer to current read message (sequence transfer)
// @msg_xfrd: number of bytes transferred in tx_fifo
// @msg_xfrd_r: number of bytes transferred in rx_fifo
// @msg_err: error code for completed message
// @msg_complete: xfer completion object
// @dev: device reference
// @adapter: core i2c abstraction
// @i2c_clk: clock reference for i2c input clock
// @bus_clk_rate: current i2c bus clock rate
// @last: a flag indicating is this is last message in transfer
// @slave: associated &i2c_client
// @irq: platform device IRQ number
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct axxia_i2c_dev {
    pub base: *mut void __iomem,
    pub msg: *mut i2c_msg,
    pub msg_r: *mut i2c_msg,
    pub msg_xfrd: usize,
    pub msg_xfrd_r: usize,
    pub msg_err: c_int,
    pub msg_complete: completion,
    pub dev: *mut device,
    pub adapter: i2c_adapter,
    pub i2c_clk: *mut clk,
    pub bus_clk_rate: u32,
    pub last: bool,
    pub slave: *mut i2c_client,
    pub irq: c_int,
}

#[no_mangle]
unsafe extern "C" fn i2c_int_disable(idev: *mut axxia_i2c_dev, mask: u32) {
    static void i2c_int_disable(struct axxia_i2c_dev *idev, u32 mask)
    {
    u32 int_en;
    int_en = readl(idev.base + MST_INT_ENABLE);
    writel(int_en & ~mask, idev.base + MST_INT_ENABLE);
    }
#[no_mangle]
unsafe extern "C" fn i2c_int_enable(idev: *mut axxia_i2c_dev, mask: u32) {
    static void i2c_int_enable(struct axxia_i2c_dev *idev, u32 mask)
    {
    u32 int_en;
    int_en = readl(idev.base + MST_INT_ENABLE);
    writel(int_en | mask, idev.base + MST_INT_ENABLE);
    }
//
// ns_to_clk - Convert time (ns) to clock cycles for the given clock frequency.
//
#[no_mangle]
unsafe extern "C" fn ns_to_clk(ns: u64, clk_mhz: u32) -> u32 {
    static u32 ns_to_clk(u64 ns, u32 clk_mhz)
    {
    return div_u64(ns * clk_mhz, 1000);
    }
#[no_mangle]
unsafe extern "C" fn axxia_i2c_init(idev: *mut axxia_i2c_dev) -> c_int {
    static int axxia_i2c_init(struct axxia_i2c_dev *idev)
    {
    let mut divisor: u32 = clk_get_rate(idev.i2c_clk) / idev.bus_clk_rate;
    let mut clk_mhz: u32 = clk_get_rate(idev.i2c_clk) / 1000000;
    u32 t_setup;
    u32 t_high, t_low;
    u32 tmo_clk;
    u32 prescale;
    unsigned long timeout;
    dev_dbg(idev.dev, "rate=%uHz per_clk=%uMHz . ratio=1:%u\n",
    idev.bus_clk_rate, clk_mhz, divisor);
// Reset controller
    writel(0x01, idev.base + SOFT_RESET);
    timeout = jiffies + msecs_to_jiffies(100);
    while (readl(idev.base + SOFT_RESET) & 1) {
    if (time_after(jiffies, timeout)) {
    dev_warn(idev.dev, "Soft reset failed\n");
    break;
    }
    }
// Enable Master Mode
    writel(0x1, idev.base + GLOBAL_CONTROL);
    if (idev.bus_clk_rate <= I2C_MAX_STANDARD_MODE_FREQ) {
// Standard mode SCL 50/50, tSU:DAT = 250 ns
    t_high = divisor * 1 / 2;
    t_low = divisor * 1 / 2;
    t_setup = ns_to_clk(250, clk_mhz);
    } else {
// Fast mode SCL 33/66, tSU:DAT = 100 ns
    t_high = divisor * 1 / 3;
    t_low = divisor * 2 / 3;
    t_setup = ns_to_clk(100, clk_mhz);
    }
// SCL High Time
    writel(t_high, idev.base + SCL_HIGH_PERIOD);
// SCL Low Time
    writel(t_low, idev.base + SCL_LOW_PERIOD);
// SDA Setup Time
    writel(t_setup, idev.base + SDA_SETUP_TIME);
// SDA Hold Time, 300ns
    writel(ns_to_clk(300, clk_mhz), idev.base + SDA_HOLD_TIME);
// Filter <50ns spikes
    writel(ns_to_clk(50, clk_mhz), idev.base + SPIKE_FLTR_LEN);
// Configure Time-Out Registers
    tmo_clk = ns_to_clk(SCL_WAIT_TIMEOUT_NS, clk_mhz);
// Find prescaler value that makes tmo_clk fit in 15-bits counter.
    for (prescale = 0; prescale < 15; ++prescale) {
    if (tmo_clk <= 0x7fff)
    break;
    tmo_clk >>= 1;
    }
    if (tmo_clk > 0x7fff)
    tmo_clk = 0x7fff;
// Prescale divider (log2)
    writel(prescale, idev.base + TIMER_CLOCK_DIV);
// Timeout in divided clocks
    writel(WT_EN | WT_VALUE(tmo_clk), idev.base + WAIT_TIMER_CONTROL);
// Mask all master interrupt bits
    i2c_int_disable(idev, ~0);
// Interrupt enable
    writel(0x01, idev.base + INTERRUPT_ENABLE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i2c_m_rd(msg: *const i2c_msg) -> c_int {
    static int i2c_m_rd(const struct i2c_msg *msg)
    {
    return (msg.flags & I2C_M_RD) != 0;
    }
#[no_mangle]
unsafe extern "C" fn i2c_m_recv_len(msg: *const i2c_msg) -> c_int {
    static int i2c_m_recv_len(const struct i2c_msg *msg)
    {
    return (msg.flags & I2C_M_RECV_LEN) != 0;
    }
//
// axxia_i2c_empty_rx_fifo - Fetch data from RX FIFO and update SMBus block
// transfer length if this is the first byte of such a transfer.
//
#[no_mangle]
unsafe extern "C" fn axxia_i2c_empty_rx_fifo(idev: *mut axxia_i2c_dev) -> c_int {
    static int axxia_i2c_empty_rx_fifo(struct axxia_i2c_dev *idev)
    {
    struct i2c_msg *msg = idev.msg_r;
    let mut rx_fifo_avail: usize = readl(idev.base + MST_RX_FIFO);
    let mut bytes_to_transfer: c_int = min(rx_fifo_avail, msg.len - idev.msg_xfrd_r);
    while (bytes_to_transfer-- > 0) {
    let mut c: c_int = readl(idev.base + MST_DATA);
    if (idev.msg_xfrd_r == 0 && i2c_m_recv_len(msg)) {
//
// Check length byte for SMBus block read
//
    if (c <= 0 || c > I2C_SMBUS_BLOCK_MAX) {
    idev.msg_err = -EPROTO;
    i2c_int_disable(idev, ~MST_STATUS_TSS);
    complete(&idev.msg_complete);
    break;
    }
    msg.len = 1 + c;
    writel(msg.len, idev.base + MST_RX_XFER);
    }
    msg.buf[idev.msg_xfrd_r++] = c;
    }
    return 0;
    }
//
// axxia_i2c_fill_tx_fifo - Fill TX FIFO from current message buffer.
// @return: Number of bytes left to transfer.
//
#[no_mangle]
unsafe extern "C" fn axxia_i2c_fill_tx_fifo(idev: *mut axxia_i2c_dev) -> c_int {
    static int axxia_i2c_fill_tx_fifo(struct axxia_i2c_dev *idev)
    {
    struct i2c_msg *msg = idev.msg;
    let mut tx_fifo_avail: usize = FIFO_SIZE - readl(idev.base + MST_TX_FIFO);
    let mut bytes_to_transfer: c_int = min(tx_fifo_avail, msg.len - idev.msg_xfrd);
    let mut ret: c_int = msg.len - idev.msg_xfrd - bytes_to_transfer;
    while (bytes_to_transfer-- > 0)
    writel(msg.buf[idev.msg_xfrd++], idev.base + MST_DATA);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn axxia_i2c_slv_fifo_event(idev: *mut axxia_i2c_dev) {
    static void axxia_i2c_slv_fifo_event(struct axxia_i2c_dev *idev)
    {
    let mut fifo_status: u32 = readl(idev.base + SLV_RX_FIFO);
    u8 val;
    dev_dbg(idev.dev, "slave irq fifo_status=0x%x\n", fifo_status);
    if (fifo_status & SLV_FIFO_DV1) {
    if (fifo_status & SLV_FIFO_STRC)
    i2c_slave_event(idev.slave,
    I2C_SLAVE_WRITE_REQUESTED, &val);
    val = readl(idev.base + SLV_DATA);
    i2c_slave_event(idev.slave, I2C_SLAVE_WRITE_RECEIVED, &val);
    }
    if (fifo_status & SLV_FIFO_STPC) {
    readl(idev.base + SLV_DATA); /* dummy read */
    i2c_slave_event(idev.slave, I2C_SLAVE_STOP, &val);
    }
    if (fifo_status & SLV_FIFO_RSC)
    readl(idev.base + SLV_DATA); /* dummy read */
    }
#[no_mangle]
unsafe extern "C" fn axxia_i2c_slv_isr(idev: *mut axxia_i2c_dev) -> irqreturn_t {
    static irqreturn_t axxia_i2c_slv_isr(struct axxia_i2c_dev *idev)
    {
    let mut status: u32 = readl(idev.base + SLV_INT_STATUS);
    u8 val;
    dev_dbg(idev.dev, "slave irq status=0x%x\n", status);
    if (status & SLV_STATUS_RFH)
    axxia_i2c_slv_fifo_event(idev);
    if (status & SLV_STATUS_SRS1) {
    i2c_slave_event(idev.slave, I2C_SLAVE_READ_REQUESTED, &val);
    writel(val, idev.base + SLV_DATA);
    }
    if (status & SLV_STATUS_SRND1) {
    i2c_slave_event(idev.slave, I2C_SLAVE_READ_PROCESSED, &val);
    writel(val, idev.base + SLV_DATA);
    }
    if (status & SLV_STATUS_SRC1)
    i2c_slave_event(idev.slave, I2C_SLAVE_STOP, &val);
    writel(INT_SLV, idev.base + INTERRUPT_STATUS);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn axxia_i2c_isr(irq: c_int, _dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t axxia_i2c_isr(int irq, void *_dev)
    {
    struct axxia_i2c_dev *idev = _dev;
    let mut ret: irqreturn_t = IRQ_NONE;
    u32 status;
    status = readl(idev.base + INTERRUPT_STATUS);
    if (status & INT_SLV)
    ret = axxia_i2c_slv_isr(idev);
    if (!(status & INT_MST))
    return ret;
// Read interrupt status bits
    status = readl(idev.base + MST_INT_STATUS);
    if (!idev.msg) {
    dev_warn(idev.dev, "unexpected interrupt\n");
    goto out;
    }
// RX FIFO needs service?
    if (i2c_m_rd(idev.msg_r) && (status & MST_STATUS_RFL))
    axxia_i2c_empty_rx_fifo(idev);
// TX FIFO needs service?
    if (!i2c_m_rd(idev.msg) && (status & MST_STATUS_TFL)) {
    if (axxia_i2c_fill_tx_fifo(idev) == 0)
    i2c_int_disable(idev, MST_STATUS_TFL);
    }
    if (unlikely(status & MST_STATUS_ERR)) {
// Transfer error
    i2c_int_disable(idev, ~0);
    if (status & MST_STATUS_AL)
    idev.msg_err = -EAGAIN;
#[no_mangle]
pub unsafe extern "C" fn if(MST_STATUS_NAK: status &) -> else {
    else if (status & MST_STATUS_NAK)
    idev.msg_err = -ENXIO;
    else
    idev.msg_err = -EIO;
    dev_dbg(idev.dev, "error %#x, addr=%#x rx=%u/%u tx=%u/%u\n",
    status,
    idev.msg.addr,
    readl(idev.base + MST_RX_BYTES_XFRD),
    readl(idev.base + MST_RX_XFER),
    readl(idev.base + MST_TX_BYTES_XFRD),
    readl(idev.base + MST_TX_XFER));
    complete(&idev.msg_complete);
    } else if (status & MST_STATUS_SCC) {
// Stop completed
    i2c_int_disable(idev, ~MST_STATUS_TSS);
    complete(&idev.msg_complete);
    } else if (status & (MST_STATUS_SNS | MST_STATUS_SS)) {
// Transfer done
    let mut mask: c_int = idev.last ? ~0 : ~MST_STATUS_TSS;
    i2c_int_disable(idev, mask);
    if (i2c_m_rd(idev.msg_r) && idev.msg_xfrd_r < idev.msg_r.len)
    axxia_i2c_empty_rx_fifo(idev);
    complete(&idev.msg_complete);
    } else if (status & MST_STATUS_TSS) {
// Transfer timeout
    idev.msg_err = -ETIMEDOUT;
    i2c_int_disable(idev, ~MST_STATUS_TSS);
    complete(&idev.msg_complete);
    }
    out:
// Clear interrupt
    writel(INT_MST, idev.base + INTERRUPT_STATUS);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn axxia_i2c_set_addr(idev: *mut axxia_i2c_dev, msg: *mut i2c_msg) {
    static void axxia_i2c_set_addr(struct axxia_i2c_dev *idev, struct i2c_msg *msg)
    {
    u32 addr_1, addr_2;
    if (msg.flags & I2C_M_TEN) {
    addr_1 = i2c_10bit_addr_hi_from_msg(msg);
    addr_2 = i2c_10bit_addr_lo_from_msg(msg);
    } else {
    addr_1 = i2c_8bit_addr_from_msg(msg);
    addr_2 = 0;
    }
    writel(addr_1, idev.base + MST_ADDR_1);
    writel(addr_2, idev.base + MST_ADDR_2);
    }
// The NAK interrupt will be sent _before_ issuing STOP command
// so the controller might still be busy processing it. No
// interrupt will be sent at the end so we have to poll for it
//
#[no_mangle]
unsafe extern "C" fn axxia_i2c_handle_seq_nak(idev: *mut axxia_i2c_dev) -> c_int {
    static int axxia_i2c_handle_seq_nak(struct axxia_i2c_dev *idev)
    {
    let mut timeout: c_ulong = jiffies + I2C_XFER_TIMEOUT;
    do {
    if ((readl(idev.base + MST_COMMAND) & CMD_BUSY) == 0)
    return 0;
    usleep_range(1, 100);
    } while (time_before(jiffies, timeout));
    return -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn axxia_i2c_xfer_seq(idev: *mut axxia_i2c_dev, msgs[]: i2c_msg) -> c_int {
    static int axxia_i2c_xfer_seq(struct axxia_i2c_dev *idev, struct i2c_msg msgs[])
    {
    let mut int_mask: u32 = MST_STATUS_ERR | MST_STATUS_SS | MST_STATUS_RFL;
    let mut rlen: u32 = i2c_m_recv_len(&msgs[1]) ? I2C_SMBUS_BLOCK_MAX : msgs[1].len;
    unsigned long time_left;
    axxia_i2c_set_addr(idev, &msgs[0]);
    writel(msgs[0].len, idev.base + MST_TX_XFER);
    writel(rlen, idev.base + MST_RX_XFER);
    idev.msg = &msgs[0];
    idev.msg_r = &msgs[1];
    idev.msg_xfrd = 0;
    idev.msg_xfrd_r = 0;
    idev.last = true;
    axxia_i2c_fill_tx_fifo(idev);
    writel(CMD_SEQUENCE, idev.base + MST_COMMAND);
    reinit_completion(&idev.msg_complete);
    i2c_int_enable(idev, int_mask);
    time_left = wait_for_completion_timeout(&idev.msg_complete,
    I2C_XFER_TIMEOUT);
    if (idev.msg_err == -ENXIO) {
    if (axxia_i2c_handle_seq_nak(idev))
    axxia_i2c_init(idev);
    } else if (readl(idev.base + MST_COMMAND) & CMD_BUSY) {
    dev_warn(idev.dev, "busy after xfer\n");
    }
    if (time_left == 0) {
    idev.msg_err = -ETIMEDOUT;
    i2c_recover_bus(&idev.adapter);
    axxia_i2c_init(idev);
    }
    if (unlikely(idev.msg_err) && idev.msg_err != -ENXIO)
    axxia_i2c_init(idev);
    return idev.msg_err;
    }
    static int axxia_i2c_xfer_msg(struct axxia_i2c_dev *idev, struct i2c_msg *msg,
    bool last)
    {
    let mut int_mask: u32 = MST_STATUS_ERR;
    u32 rx_xfer, tx_xfer;
    unsigned long time_left;
    unsigned int wt_value;
    idev.msg = msg;
    idev.msg_r = msg;
    idev.msg_xfrd = 0;
    idev.msg_xfrd_r = 0;
    idev.last = last;
    reinit_completion(&idev.msg_complete);
    axxia_i2c_set_addr(idev, msg);
    if (i2c_m_rd(msg)) {
// I2C read transfer
    rx_xfer = i2c_m_recv_len(msg) ? I2C_SMBUS_BLOCK_MAX : msg.len;
    tx_xfer = 0;
    } else {
// I2C write transfer
    rx_xfer = 0;
    tx_xfer = msg.len;
    }
    writel(rx_xfer, idev.base + MST_RX_XFER);
    writel(tx_xfer, idev.base + MST_TX_XFER);
    if (i2c_m_rd(msg))
    int_mask |= MST_STATUS_RFL;
#[no_mangle]
pub unsafe extern "C" fn if(0: axxia_i2c_fill_tx_fifo(idev) !=) -> else {
    else if (axxia_i2c_fill_tx_fifo(idev) != 0)
    int_mask |= MST_STATUS_TFL;
    wt_value = WT_VALUE(readl(idev.base + WAIT_TIMER_CONTROL));
// Disable wait timer temporarly
    writel(wt_value, idev.base + WAIT_TIMER_CONTROL);
// Check if timeout error happened
    if (idev.msg_err)
    goto out;
    if (!last) {
    writel(CMD_MANUAL, idev.base + MST_COMMAND);
    int_mask |= MST_STATUS_SNS;
    } else {
    writel(CMD_AUTO, idev.base + MST_COMMAND);
    int_mask |= MST_STATUS_SS;
    }
    writel(WT_EN | wt_value, idev.base + WAIT_TIMER_CONTROL);
    i2c_int_enable(idev, int_mask);
    time_left = wait_for_completion_timeout(&idev.msg_complete,
    I2C_XFER_TIMEOUT);
    i2c_int_disable(idev, int_mask);
    if (readl(idev.base + MST_COMMAND) & CMD_BUSY)
    dev_warn(idev.dev, "busy after xfer\n");
    if (time_left == 0) {
    idev.msg_err = -ETIMEDOUT;
    i2c_recover_bus(&idev.adapter);
    axxia_i2c_init(idev);
    }
    out:
    if (unlikely(idev.msg_err) && idev.msg_err != -ENXIO &&
    idev.msg_err != -ETIMEDOUT)
    axxia_i2c_init(idev);
    return idev.msg_err;
    }
// This function checks if the msgs[] array contains messages compatible with
// Sequence mode of operation. This mode assumes there will be exactly one
// write of non-zero length followed by exactly one read of non-zero length,
// both targeted at the same client device.
//
#[no_mangle]
unsafe extern "C" fn axxia_i2c_sequence_ok(msgs[]: i2c_msg, num: c_int) -> bool {
    static bool axxia_i2c_sequence_ok(struct i2c_msg msgs[], int num)
    {
    return num == SEQ_LEN && !i2c_m_rd(&msgs[0]) && i2c_m_rd(&msgs[1]) &&
    msgs[0].len > 0 && msgs[0].len <= FIFO_SIZE &&
    msgs[1].len > 0 && msgs[0].addr == msgs[1].addr;
    }
    static int
    axxia_i2c_xfer(struct i2c_adapter *adap, struct i2c_msg msgs[], int num)
    {
    struct axxia_i2c_dev *idev = i2c_get_adapdata(adap);
    int i;
    let mut ret: c_int = 0;
    idev.msg_err = 0;
    if (axxia_i2c_sequence_ok(msgs, num)) {
    ret = axxia_i2c_xfer_seq(idev, msgs);
    return ret ? : SEQ_LEN;
    }
    i2c_int_enable(idev, MST_STATUS_TSS);
    for (i = 0; ret == 0 && i < num; ++i)
    ret = axxia_i2c_xfer_msg(idev, &msgs[i], i == (num - 1));
    return ret ? : i;
    }
#[no_mangle]
unsafe extern "C" fn axxia_i2c_get_scl(adap: *mut i2c_adapter) -> c_int {
    static int axxia_i2c_get_scl(struct i2c_adapter *adap)
    {
    struct axxia_i2c_dev *idev = i2c_get_adapdata(adap);
    return !!(readl(idev.base + I2C_BUS_MONITOR) & BM_SCLS);
    }
#[no_mangle]
unsafe extern "C" fn axxia_i2c_set_scl(adap: *mut i2c_adapter, val: c_int) {
    static void axxia_i2c_set_scl(struct i2c_adapter *adap, int val)
    {
    struct axxia_i2c_dev *idev = i2c_get_adapdata(adap);
    u32 tmp;
// Preserve SDA Control
    tmp = readl(idev.base + I2C_BUS_MONITOR) & BM_SDAC;
    if (!val)
    tmp |= BM_SCLC;
    writel(tmp, idev.base + I2C_BUS_MONITOR);
    }
#[no_mangle]
unsafe extern "C" fn axxia_i2c_get_sda(adap: *mut i2c_adapter) -> c_int {
    static int axxia_i2c_get_sda(struct i2c_adapter *adap)
    {
    struct axxia_i2c_dev *idev = i2c_get_adapdata(adap);
    return !!(readl(idev.base + I2C_BUS_MONITOR) & BM_SDAS);
    }
    static struct i2c_bus_recovery_info axxia_i2c_recovery_info = {
    .recover_bus = i2c_generic_scl_recovery,
    .get_scl = axxia_i2c_get_scl,
    .set_scl = axxia_i2c_set_scl,
    .get_sda = axxia_i2c_get_sda,
    };
#[no_mangle]
unsafe extern "C" fn axxia_i2c_func(adap: *mut i2c_adapter) -> u32 {
    static u32 axxia_i2c_func(struct i2c_adapter *adap)
    {
    u32 caps = (I2C_FUNC_I2C | I2C_FUNC_10BIT_ADDR |
    I2C_FUNC_SMBUS_EMUL | I2C_FUNC_SMBUS_BLOCK_DATA);
    return caps;
    }
#[no_mangle]
unsafe extern "C" fn axxia_i2c_reg_slave(slave: *mut i2c_client) -> c_int {
    static int axxia_i2c_reg_slave(struct i2c_client *slave)
    {
    struct axxia_i2c_dev *idev = i2c_get_adapdata(slave.adapter);
    let mut slv_int_mask: u32 = SLV_STATUS_RFH;
    u32 dec_ctl;
    if (idev.slave)
    return -EBUSY;
    idev.slave = slave;
// Enable slave mode as well
    writel(GLOBAL_MST_EN | GLOBAL_SLV_EN, idev.base + GLOBAL_CONTROL);
    writel(INT_MST | INT_SLV, idev.base + INTERRUPT_ENABLE);
// Set slave address
    dec_ctl = SLV_ADDR_DEC_SA1E;
    if (slave.flags & I2C_CLIENT_TEN)
    dec_ctl |= SLV_ADDR_DEC_SA1M;
    writel(SLV_RX_ACSA1, idev.base + SLV_RX_CTL);
    writel(dec_ctl, idev.base + SLV_ADDR_DEC_CTL);
    writel(slave.addr, idev.base + SLV_ADDR_1);
// Enable interrupts
    slv_int_mask |= SLV_STATUS_SRS1 | SLV_STATUS_SRRS1 | SLV_STATUS_SRND1;
    slv_int_mask |= SLV_STATUS_SRC1;
    writel(slv_int_mask, idev.base + SLV_INT_ENABLE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn axxia_i2c_unreg_slave(slave: *mut i2c_client) -> c_int {
    static int axxia_i2c_unreg_slave(struct i2c_client *slave)
    {
    struct axxia_i2c_dev *idev = i2c_get_adapdata(slave.adapter);
// Disable slave mode
    writel(GLOBAL_MST_EN, idev.base + GLOBAL_CONTROL);
    writel(INT_MST, idev.base + INTERRUPT_ENABLE);
    synchronize_irq(idev.irq);
    idev.slave = core::ptr::null_mut();
    return 0;
    }
    static const struct i2c_algorithm axxia_i2c_algo = {
    .xfer = axxia_i2c_xfer,
    .functionality = axxia_i2c_func,
    .reg_slave = axxia_i2c_reg_slave,
    .unreg_slave = axxia_i2c_unreg_slave,
    };
    static const struct i2c_adapter_quirks axxia_i2c_quirks = {
    .max_read_len = 255,
    .max_write_len = 255,
    };
#[no_mangle]
unsafe extern "C" fn axxia_i2c_probe(pdev: *mut platform_device) -> c_int {
    static int axxia_i2c_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct axxia_i2c_dev *idev = core::ptr::null_mut();
    void __iomem *base;
    let mut ret: c_int = 0;
    idev = devm_kzalloc(&pdev.dev, sizeof(*idev), GFP_KERNEL);
    if (!idev)
    return -ENOMEM;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    idev.irq = platform_get_irq(pdev, 0);
    if (idev.irq < 0)
    return idev.irq;
    idev.i2c_clk = devm_clk_get(&pdev.dev, "i2c");
    if (IS_ERR(idev.i2c_clk)) {
    dev_err(&pdev.dev, "missing clock\n");
    return PTR_ERR(idev.i2c_clk);
    }
    idev.base = base;
    idev.dev = &pdev.dev;
    init_completion(&idev.msg_complete);
    of_property_read_u32(np, "clock-frequency", &idev.bus_clk_rate);
    if (idev.bus_clk_rate == 0)
    idev.bus_clk_rate = I2C_MAX_STANDARD_MODE_FREQ;	/* default clock rate */
    ret = clk_prepare_enable(idev.i2c_clk);
    if (ret) {
    dev_err(&pdev.dev, "failed to enable clock\n");
    return ret;
    }
    ret = axxia_i2c_init(idev);
    if (ret) {
    dev_err(&pdev.dev, "failed to initialize\n");
    goto error_disable_clk;
    }
    ret = devm_request_irq(&pdev.dev, idev.irq, axxia_i2c_isr, 0,
    pdev.name, idev);
    if (ret) {
    dev_err(&pdev.dev, "failed to claim IRQ%d\n", idev.irq);
    goto error_disable_clk;
    }
    i2c_set_adapdata(&idev.adapter, idev);
    strscpy(idev.adapter.name, pdev.name, sizeof(idev.adapter.name));
    idev.adapter.owner = THIS_MODULE;
    idev.adapter.algo = &axxia_i2c_algo;
    idev.adapter.bus_recovery_info = &axxia_i2c_recovery_info;
    idev.adapter.quirks = &axxia_i2c_quirks;
    idev.adapter.dev.parent = &pdev.dev;
    idev.adapter.dev.of_node = pdev.dev.of_node;
    platform_set_drvdata(pdev, idev);
    ret = i2c_add_adapter(&idev.adapter);
    if (ret)
    goto error_disable_clk;
    return 0;
    error_disable_clk:
    clk_disable_unprepare(idev.i2c_clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn axxia_i2c_remove(pdev: *mut platform_device) {
    static void axxia_i2c_remove(struct platform_device *pdev)
    {
    struct axxia_i2c_dev *idev = platform_get_drvdata(pdev);
    clk_disable_unprepare(idev.i2c_clk);
    i2c_del_adapter(&idev.adapter);
    }
// Match table for of_platform binding
    static const struct of_device_id axxia_i2c_of_match[] = {
    { .compatible = "lsi,api2c", },
    {},
    };
    MODULE_DEVICE_TABLE(of, axxia_i2c_of_match);
    static struct platform_driver axxia_i2c_driver = {
    .probe = axxia_i2c_probe,
    .remove = axxia_i2c_remove,
    .driver = {
    .name = "axxia-i2c",
    .of_match_table = axxia_i2c_of_match,
    },
    };
    module_platform_driver(axxia_i2c_driver);
    MODULE_DESCRIPTION("Axxia I2C Bus driver");
    MODULE_AUTHOR("Anders Berg <anders.berg@lsi.com>");
    MODULE_LICENSE("GPL v2");
