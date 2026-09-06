//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-altera.c
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
// Copyright Intel Corporation (C) 2017.
//
// Based on the i2c-axxia.c driver.
//

pub const ALTR_I2C_TFR_CMD: c_uint = 0x00	/* Transfer Command register */;

pub const ALTR_I2C_RX_DATA: c_uint = 0x04	/* RX data FIFO register */;
pub const ALTR_I2C_CTRL: c_uint = 0x08	/* Control register */;

pub const ALTR_I2C_ISER: c_uint = 0x0C	/* Interrupt Status Enable register */;

pub const ALTR_I2C_ISR: c_uint = 0x10	/* Interrupt Status register */;

pub const ALTR_I2C_STATUS: c_uint = 0x14	/* Status register */;

pub const ALTR_I2C_TC_FIFO_LVL: c_uint = 0x18	/* Transfer FIFO LVL register */;
pub const ALTR_I2C_RX_FIFO_LVL: c_uint = 0x1C	/* Receive FIFO LVL register */;
pub const ALTR_I2C_SCL_LOW: c_uint = 0x20	/* SCL low count register */;
pub const ALTR_I2C_SCL_HIGH: c_uint = 0x24	/* SCL high count register */;
pub const ALTR_I2C_SDA_HOLD: c_uint = 0x28	/* SDA hold count register */;

    ALTR_I2C_ISR_NACK | ALTR_I2C_ISR_RXRDY | \
    ALTR_I2C_ISR_TXRDY)

pub const ALTR_I2C_DFLT_FIFO_SZ: c_int = 4;

//
// struct altr_i2c_dev - I2C device context
// @base: pointer to register struct
// @msg: pointer to current message
// @msg_len: number of bytes transferred in msg
// @msg_err: error code for completed message
// @msg_complete: xfer completion object
// @dev: device reference
// @adapter: core i2c abstraction
// @i2c_clk: clock reference for i2c input clock
// @bus_clk_rate: current i2c bus clock rate
// @buf: ptr to msg buffer for easier use.
// @fifo_size: size of the FIFO passed in.
// @isr_mask: cached copy of local ISR enables.
// @isr_status: cached copy of local ISR status.
// @isr_mutex: mutex for IRQ thread.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct altr_i2c_dev {
    pub base: *mut void __iomem,
    pub msg: *mut i2c_msg,
    pub msg_len: usize,
    pub msg_err: c_int,
    pub msg_complete: completion,
    pub dev: *mut device,
    pub adapter: i2c_adapter,
    pub i2c_clk: *mut clk,
    pub bus_clk_rate: u32,
    pub buf: *mut u8,
    pub fifo_size: u32,
    pub isr_mask: u32,
    pub isr_status: u32,
    pub isr_mutex: mutex,
}

    static void
    altr_i2c_int_enable(struct altr_i2c_dev *idev, u32 mask, bool enable)
    {
    u32 int_en;
    int_en = readl(idev.base + ALTR_I2C_ISER);
    if (enable)
    idev.isr_mask = int_en | mask;
    else
    idev.isr_mask = int_en & ~mask;
    writel(idev.isr_mask, idev.base + ALTR_I2C_ISER);
    }
#[no_mangle]
unsafe extern "C" fn altr_i2c_int_clear(idev: *mut altr_i2c_dev, mask: u32) {
    static void altr_i2c_int_clear(struct altr_i2c_dev *idev, u32 mask)
    {
    let mut int_en: u32 = readl(idev.base + ALTR_I2C_ISR);
    writel(int_en | mask, idev.base + ALTR_I2C_ISR);
    }
#[no_mangle]
unsafe extern "C" fn altr_i2c_core_disable(idev: *mut altr_i2c_dev) {
    static void altr_i2c_core_disable(struct altr_i2c_dev *idev)
    {
    let mut tmp: u32 = readl(idev.base + ALTR_I2C_CTRL);
    writel(tmp & ~ALTR_I2C_CTRL_EN, idev.base + ALTR_I2C_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn altr_i2c_core_enable(idev: *mut altr_i2c_dev) {
    static void altr_i2c_core_enable(struct altr_i2c_dev *idev)
    {
    let mut tmp: u32 = readl(idev.base + ALTR_I2C_CTRL);
    writel(tmp | ALTR_I2C_CTRL_EN, idev.base + ALTR_I2C_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn altr_i2c_reset(idev: *mut altr_i2c_dev) {
    static void altr_i2c_reset(struct altr_i2c_dev *idev)
    {
    altr_i2c_core_disable(idev);
    altr_i2c_core_enable(idev);
    }
#[no_mangle]
pub unsafe extern "C" fn altr_i2c_stop(idev: *mut altr_i2c_dev) {
    static inline void altr_i2c_stop(struct altr_i2c_dev *idev)
    {
    writel(ALTR_I2C_TFR_CMD_STO, idev.base + ALTR_I2C_TFR_CMD);
    }
#[no_mangle]
unsafe extern "C" fn altr_i2c_init(idev: *mut altr_i2c_dev) {
    static void altr_i2c_init(struct altr_i2c_dev *idev)
    {
    let mut divisor: u32 = clk_get_rate(idev.i2c_clk) / idev.bus_clk_rate;
    let mut clk_mhz: u32 = clk_get_rate(idev.i2c_clk) / 1000000;
    u32 tmp = (ALTR_I2C_THRESHOLD << ALTR_I2C_CTRL_RXT_SHFT) |
    (ALTR_I2C_THRESHOLD << ALTR_I2C_CTRL_TCT_SHFT);
    u32 t_high, t_low;
    if (idev.bus_clk_rate <= I2C_MAX_STANDARD_MODE_FREQ) {
    tmp &= ~ALTR_I2C_CTRL_BSPEED;
// Standard mode SCL 50/50
    t_high = divisor * 1 / 2;
    t_low = divisor * 1 / 2;
    } else {
    tmp |= ALTR_I2C_CTRL_BSPEED;
// Fast mode SCL 33/66
    t_high = divisor * 1 / 3;
    t_low = divisor * 2 / 3;
    }
    writel(tmp, idev.base + ALTR_I2C_CTRL);
    dev_dbg(idev.dev, "rate=%uHz per_clk=%uMHz . ratio=1:%u\n",
    idev.bus_clk_rate, clk_mhz, divisor);
// Reset controller
    altr_i2c_reset(idev);
// SCL High Time
    writel(t_high, idev.base + ALTR_I2C_SCL_HIGH);
// SCL Low Time
    writel(t_low, idev.base + ALTR_I2C_SCL_LOW);
// SDA Hold Time, 300ns
    writel(3 * clk_mhz / 10, idev.base + ALTR_I2C_SDA_HOLD);
// Mask all interrupt bits
    altr_i2c_int_enable(idev, ALTR_I2C_ALL_IRQ, false);
    }
//
// altr_i2c_transfer - On the last byte to be transmitted, send
// a Stop bit on the last byte.
//
#[no_mangle]
unsafe extern "C" fn altr_i2c_transfer(idev: *mut altr_i2c_dev, data: u32) {
    static void altr_i2c_transfer(struct altr_i2c_dev *idev, u32 data)
    {
// On the last byte to be transmitted, send STOP
    if (idev.msg_len == 1)
    data |= ALTR_I2C_TFR_CMD_STO;
    if (idev.msg_len > 0)
    writel(data, idev.base + ALTR_I2C_TFR_CMD);
    }
//
// altr_i2c_empty_rx_fifo - Fetch data from RX FIFO until end of
// transfer. Send a Stop bit on the last byte.
//
#[no_mangle]
unsafe extern "C" fn altr_i2c_empty_rx_fifo(idev: *mut altr_i2c_dev) {
    static void altr_i2c_empty_rx_fifo(struct altr_i2c_dev *idev)
    {
    let mut rx_fifo_avail: usize = readl(idev.base + ALTR_I2C_RX_FIFO_LVL);
    let mut bytes_to_transfer: c_int = min(rx_fifo_avail, idev.msg_len);
    while (bytes_to_transfer-- > 0) {
// idev->buf++ = readl(idev->base + ALTR_I2C_RX_DATA);
    idev.msg_len--;
    altr_i2c_transfer(idev, 0);
    }
    }
//
// altr_i2c_fill_tx_fifo - Fill TX FIFO from current message buffer.
//
#[no_mangle]
unsafe extern "C" fn altr_i2c_fill_tx_fifo(idev: *mut altr_i2c_dev) -> c_int {
    static int altr_i2c_fill_tx_fifo(struct altr_i2c_dev *idev)
    {
    size_t tx_fifo_avail = idev.fifo_size - readl(idev.base +
    ALTR_I2C_TC_FIFO_LVL);
    let mut bytes_to_transfer: c_int = min(tx_fifo_avail, idev.msg_len);
    let mut ret: c_int = idev.msg_len - bytes_to_transfer;
    while (bytes_to_transfer-- > 0) {
    altr_i2c_transfer(idev, *idev.buf++);
    idev.msg_len--;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn altr_i2c_isr_quick(irq: c_int, _dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t altr_i2c_isr_quick(int irq, void *_dev)
    {
    struct altr_i2c_dev *idev = _dev;
    let mut ret: irqreturn_t = IRQ_HANDLED;
// Read IRQ status but only interested in Enabled IRQs.
    idev.isr_status = readl(idev.base + ALTR_I2C_ISR) & idev.isr_mask;
    if (idev.isr_status)
    ret = IRQ_WAKE_THREAD;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn altr_i2c_isr(irq: c_int, _dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t altr_i2c_isr(int irq, void *_dev)
    {
    int ret;
    bool read, finish = false;
    struct altr_i2c_dev *idev = _dev;
    let mut status: u32 = idev.isr_status;
    mutex_lock(&idev.isr_mutex);
    if (!idev.msg) {
    dev_warn(idev.dev, "unexpected interrupt\n");
    altr_i2c_int_clear(idev, ALTR_I2C_ALL_IRQ);
    goto out;
    }
    read = (idev.msg.flags & I2C_M_RD) != 0;
// handle Lost Arbitration
    if (unlikely(status & ALTR_I2C_ISR_ARB)) {
    altr_i2c_int_clear(idev, ALTR_I2C_ISR_ARB);
    idev.msg_err = -EAGAIN;
    finish = true;
    } else if (unlikely(status & ALTR_I2C_ISR_NACK)) {
    dev_dbg(idev.dev, "Could not get ACK\n");
    idev.msg_err = -ENXIO;
    altr_i2c_int_clear(idev, ALTR_I2C_ISR_NACK);
    altr_i2c_stop(idev);
    finish = true;
    } else if (read && unlikely(status & ALTR_I2C_ISR_RXOF)) {
// handle RX FIFO Overflow
    altr_i2c_empty_rx_fifo(idev);
    altr_i2c_int_clear(idev, ALTR_I2C_ISR_RXRDY);
    altr_i2c_stop(idev);
    dev_err(idev.dev, "RX FIFO Overflow\n");
    finish = true;
    } else if (read && (status & ALTR_I2C_ISR_RXRDY)) {
// RX FIFO needs service?
    altr_i2c_empty_rx_fifo(idev);
    altr_i2c_int_clear(idev, ALTR_I2C_ISR_RXRDY);
    if (!idev.msg_len)
    finish = true;
    } else if (!read && (status & ALTR_I2C_ISR_TXRDY)) {
// TX FIFO needs service?
    altr_i2c_int_clear(idev, ALTR_I2C_ISR_TXRDY);
    if (idev.msg_len > 0)
    altr_i2c_fill_tx_fifo(idev);
    else
    finish = true;
    } else {
    dev_warn(idev.dev, "Unexpected interrupt: 0x%x\n", status);
    altr_i2c_int_clear(idev, ALTR_I2C_ALL_IRQ);
    }
    if (finish) {
// Wait for the Core to finish
    ret = readl_poll_timeout_atomic(idev.base + ALTR_I2C_STATUS,
    status,
    !(status & ALTR_I2C_STAT_CORE),
    1, ALTR_I2C_TIMEOUT);
    if (ret)
    dev_err(idev.dev, "message timeout\n");
    altr_i2c_int_enable(idev, ALTR_I2C_ALL_IRQ, false);
    altr_i2c_int_clear(idev, ALTR_I2C_ALL_IRQ);
    complete(&idev.msg_complete);
    dev_dbg(idev.dev, "Message Complete\n");
    }
    out:
    mutex_unlock(&idev.isr_mutex);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn altr_i2c_xfer_msg(idev: *mut altr_i2c_dev, msg: *mut i2c_msg) -> c_int {
    static int altr_i2c_xfer_msg(struct altr_i2c_dev *idev, struct i2c_msg *msg)
    {
    let mut imask: u32 = ALTR_I2C_ISR_RXOF | ALTR_I2C_ISR_ARB | ALTR_I2C_ISR_NACK;
    unsigned long time_left;
    u32 value;
    let mut addr: u8 = i2c_8bit_addr_from_msg(msg);
    mutex_lock(&idev.isr_mutex);
    idev.msg = msg;
    idev.msg_len = msg.len;
    idev.buf = msg.buf;
    idev.msg_err = 0;
    reinit_completion(&idev.msg_complete);
    altr_i2c_core_enable(idev);
// Make sure RX FIFO is empty
    do {
    readl(idev.base + ALTR_I2C_RX_DATA);
    } while (readl(idev.base + ALTR_I2C_RX_FIFO_LVL));
    writel(ALTR_I2C_TFR_CMD_STA | addr, idev.base + ALTR_I2C_TFR_CMD);
    if ((msg.flags & I2C_M_RD) != 0) {
    imask |= ALTR_I2C_ISER_RXOF_EN | ALTR_I2C_ISER_RXRDY_EN;
    altr_i2c_int_enable(idev, imask, true);
// write the first byte to start the RX
    altr_i2c_transfer(idev, 0);
    } else {
    imask |= ALTR_I2C_ISR_TXRDY;
    altr_i2c_int_enable(idev, imask, true);
    altr_i2c_fill_tx_fifo(idev);
    }
    mutex_unlock(&idev.isr_mutex);
    time_left = wait_for_completion_timeout(&idev.msg_complete,
    ALTR_I2C_XFER_TIMEOUT);
    mutex_lock(&idev.isr_mutex);
    altr_i2c_int_enable(idev, imask, false);
    value = readl(idev.base + ALTR_I2C_STATUS) & ALTR_I2C_STAT_CORE;
    if (value)
    dev_err(idev.dev, "Core Status not IDLE...\n");
    if (time_left == 0) {
    idev.msg_err = -ETIMEDOUT;
    dev_dbg(idev.dev, "Transaction timed out.\n");
    }
    altr_i2c_core_disable(idev);
    mutex_unlock(&idev.isr_mutex);
    return idev.msg_err;
    }
    static int
    altr_i2c_xfer(struct i2c_adapter *adap, struct i2c_msg *msgs, int num)
    {
    struct altr_i2c_dev *idev = i2c_get_adapdata(adap);
    int i, ret;
    for (i = 0; i < num; i++) {
    ret = altr_i2c_xfer_msg(idev, msgs++);
    if (ret)
    return ret;
    }
    return num;
    }
#[no_mangle]
unsafe extern "C" fn altr_i2c_func(adap: *mut i2c_adapter) -> u32 {
    static u32 altr_i2c_func(struct i2c_adapter *adap)
    {
    return I2C_FUNC_I2C | I2C_FUNC_SMBUS_EMUL;
    }
    static const struct i2c_algorithm altr_i2c_algo = {
    .xfer = altr_i2c_xfer,
    .functionality = altr_i2c_func,
    };
#[no_mangle]
unsafe extern "C" fn altr_i2c_probe(pdev: *mut platform_device) -> c_int {
    static int altr_i2c_probe(struct platform_device *pdev)
    {
    struct altr_i2c_dev *idev = core::ptr::null_mut();
    int irq, ret;
    idev = devm_kzalloc(&pdev.dev, sizeof(*idev), GFP_KERNEL);
    if (!idev)
    return -ENOMEM;
    idev.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(idev.base))
    return PTR_ERR(idev.base);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    idev.i2c_clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(idev.i2c_clk)) {
    dev_err(&pdev.dev, "missing clock\n");
    return PTR_ERR(idev.i2c_clk);
    }
    idev.dev = &pdev.dev;
    init_completion(&idev.msg_complete);
    mutex_init(&idev.isr_mutex);
    ret = device_property_read_u32(idev.dev, "fifo-size",
    &idev.fifo_size);
    if (ret) {
    dev_err(&pdev.dev, "FIFO size set to default of %d\n",
    ALTR_I2C_DFLT_FIFO_SZ);
    idev.fifo_size = ALTR_I2C_DFLT_FIFO_SZ;
    }
    ret = device_property_read_u32(idev.dev, "clock-frequency",
    &idev.bus_clk_rate);
    if (ret) {
    dev_err(&pdev.dev, "Default to 100kHz\n");
    idev.bus_clk_rate = I2C_MAX_STANDARD_MODE_FREQ;	/* default clock rate */
    }
    if (idev.bus_clk_rate > I2C_MAX_FAST_MODE_FREQ) {
    dev_err(&pdev.dev, "invalid clock-frequency %d\n",
    idev.bus_clk_rate);
    return -EINVAL;
    }
    ret = devm_request_threaded_irq(&pdev.dev, irq, altr_i2c_isr_quick,
    altr_i2c_isr, IRQF_ONESHOT,
    pdev.name, idev);
    if (ret) {
    dev_err(&pdev.dev, "failed to claim IRQ %d\n", irq);
    return ret;
    }
    ret = clk_prepare_enable(idev.i2c_clk);
    if (ret) {
    dev_err(&pdev.dev, "failed to enable clock\n");
    return ret;
    }
    mutex_lock(&idev.isr_mutex);
    altr_i2c_init(idev);
    mutex_unlock(&idev.isr_mutex);
    i2c_set_adapdata(&idev.adapter, idev);
    strscpy(idev.adapter.name, pdev.name, sizeof(idev.adapter.name));
    idev.adapter.owner = THIS_MODULE;
    idev.adapter.algo = &altr_i2c_algo;
    idev.adapter.dev.parent = &pdev.dev;
    idev.adapter.dev.of_node = pdev.dev.of_node;
    platform_set_drvdata(pdev, idev);
    ret = i2c_add_adapter(&idev.adapter);
    if (ret) {
    clk_disable_unprepare(idev.i2c_clk);
    return ret;
    }
    dev_info(&pdev.dev, "Altera SoftIP I2C Probe Complete\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn altr_i2c_remove(pdev: *mut platform_device) {
    static void altr_i2c_remove(struct platform_device *pdev)
    {
    struct altr_i2c_dev *idev = platform_get_drvdata(pdev);
    clk_disable_unprepare(idev.i2c_clk);
    i2c_del_adapter(&idev.adapter);
    }
// Match table for of_platform binding
    static const struct of_device_id altr_i2c_of_match[] = {
    { .compatible = "altr,softip-i2c-v1.0" },
    {},
    };
    MODULE_DEVICE_TABLE(of, altr_i2c_of_match);
    static struct platform_driver altr_i2c_driver = {
    .probe = altr_i2c_probe,
    .remove = altr_i2c_remove,
    .driver = {
    .name = "altera-i2c",
    .of_match_table = altr_i2c_of_match,
    },
    };
    module_platform_driver(altr_i2c_driver);
    MODULE_DESCRIPTION("Altera Soft IP I2C bus driver");
    MODULE_AUTHOR("Thor Thayer <thor.thayer@linux.intel.com>");
    MODULE_LICENSE("GPL v2");
