//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-lpc2k.c
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
// Copyright (C) 2011 NXP Semiconductors
//
// Code portions referenced from the i2x-pxa and i2c-pnx drivers
//
// Make SMBus byte and word transactions work on LPC178x/7x
// Copyright (c) 2012
// Alexander Potashev, Emcraft Systems, aspotashev@emcraft.com
// Anton Protopopov, Emcraft Systems, antonp@emcraft.com
//
// Copyright (C) 2015 Joachim Eastwood <manabian@gmail.com>
//

// LPC24xx register offsets and bits
pub const LPC24XX_I2CONSET: c_uint = 0x00;
pub const LPC24XX_I2STAT: c_uint = 0x04;
pub const LPC24XX_I2DAT: c_uint = 0x08;
pub const LPC24XX_I2ADDR: c_uint = 0x0c;
pub const LPC24XX_I2SCLH: c_uint = 0x10;
pub const LPC24XX_I2SCLL: c_uint = 0x14;
pub const LPC24XX_I2CONCLR: c_uint = 0x18;

    LPC24XX_STA | LPC24XX_I2EN)
// I2C SCL clock has different duty cycle depending on mode
pub const I2C_STD_MODE_DUTY: c_int = 46;
pub const I2C_FAST_MODE_DUTY: c_int = 36;
pub const I2C_FAST_MODE_PLUS_DUTY: c_int = 38;
//
// 26 possible I2C status codes, but codes applicable only
// to controller mode are listed here and used in this driver
//
    enum {
    M_BUS_ERROR		= 0x00,
    M_START			= 0x08,
    M_REPSTART		= 0x10,
    MX_ADDR_W_ACK		= 0x18,
    MX_ADDR_W_NACK		= 0x20,
    MX_DATA_W_ACK		= 0x28,
    MX_DATA_W_NACK		= 0x30,
    M_DATA_ARB_LOST		= 0x38,
    MR_ADDR_R_ACK		= 0x40,
    MR_ADDR_R_NACK		= 0x48,
    MR_DATA_R_ACK		= 0x50,
    MR_DATA_R_NACK		= 0x58,
    M_I2C_IDLE		= 0xf8,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpc2k_i2c {
    pub base: *mut void __iomem,
    pub clk: *mut clk,
    pub irq: c_int,
    pub wait: wait_queue_head_t,
    pub adap: i2c_adapter,
    pub msg: *mut i2c_msg,
    pub msg_idx: c_int,
    pub msg_status: c_int,
    pub is_last: c_int,
}

#[no_mangle]
unsafe extern "C" fn i2c_lpc2k_reset(i2c: *mut lpc2k_i2c) {
    static void i2c_lpc2k_reset(struct lpc2k_i2c *i2c)
    {
// Will force clear all statuses
    writel(LPC24XX_CLEAR_ALL, i2c.base + LPC24XX_I2CONCLR);
    writel(0, i2c.base + LPC24XX_I2ADDR);
    writel(LPC24XX_I2EN, i2c.base + LPC24XX_I2CONSET);
    }
#[no_mangle]
unsafe extern "C" fn i2c_lpc2k_clear_arb(i2c: *mut lpc2k_i2c) -> c_int {
    static int i2c_lpc2k_clear_arb(struct lpc2k_i2c *i2c)
    {
    let mut timeout: c_ulong = jiffies + msecs_to_jiffies(1000);
//
// If the transfer needs to abort for some reason, we'll try to
// force a stop condition to clear any pending bus conditions
//
    writel(LPC24XX_STO, i2c.base + LPC24XX_I2CONSET);
// Wait for status change
    while (readl(i2c.base + LPC24XX_I2STAT) != M_I2C_IDLE) {
    if (time_after(jiffies, timeout)) {
// Bus was not idle, try to reset adapter
    i2c_lpc2k_reset(i2c);
    return -EBUSY;
    }
    cpu_relax();
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i2c_lpc2k_pump_msg(i2c: *mut lpc2k_i2c) {
    static void i2c_lpc2k_pump_msg(struct lpc2k_i2c *i2c)
    {
    unsigned char data;
    u32 status;
//
// I2C in the LPC2xxx series is basically a state machine.
// Just run through the steps based on the current status.
//
    status = readl(i2c.base + LPC24XX_I2STAT);
    switch (status) {
    case M_START:
    case M_REPSTART:
// Start bit was just sent out, send out addr and dir
    data = i2c_8bit_addr_from_msg(i2c.msg);
    writel(data, i2c.base + LPC24XX_I2DAT);
    writel(LPC24XX_STA, i2c.base + LPC24XX_I2CONCLR);
    break;
    case MX_ADDR_W_ACK:
    case MX_DATA_W_ACK:
//
// Address or data was sent out with an ACK. If there is more
// data to send, send it now
//
    if (i2c.msg_idx < i2c.msg.len) {
    writel(i2c.msg.buf[i2c.msg_idx],
    i2c.base + LPC24XX_I2DAT);
    } else if (i2c.is_last) {
// Last message, send stop
    writel(LPC24XX_STO_AA, i2c.base + LPC24XX_I2CONSET);
    writel(LPC24XX_SI, i2c.base + LPC24XX_I2CONCLR);
    i2c.msg_status = 0;
    disable_irq_nosync(i2c.irq);
    } else {
    i2c.msg_status = 0;
    disable_irq_nosync(i2c.irq);
    }
    i2c.msg_idx++;
    break;
    case MR_ADDR_R_ACK:
// Receive first byte from target
    if (i2c.msg.len == 1) {
// Last byte, return NACK
    writel(LPC24XX_AA, i2c.base + LPC24XX_I2CONCLR);
    } else {
// Not last byte, return ACK
    writel(LPC24XX_AA, i2c.base + LPC24XX_I2CONSET);
    }
    writel(LPC24XX_STA, i2c.base + LPC24XX_I2CONCLR);
    break;
    case MR_DATA_R_NACK:
//
// The I2C shows NACK status on reads, so we need to accept
// the NACK as an ACK here. This should be ok, as the real
// BACK would of been caught on the address write.
//
    case MR_DATA_R_ACK:
// Data was received
    if (i2c.msg_idx < i2c.msg.len) {
    i2c.msg.buf[i2c.msg_idx] =
    readl(i2c.base + LPC24XX_I2DAT);
    }
// If transfer is done, send STOP
    if (i2c.msg_idx >= i2c.msg.len - 1 && i2c.is_last) {
    writel(LPC24XX_STO_AA, i2c.base + LPC24XX_I2CONSET);
    writel(LPC24XX_SI, i2c.base + LPC24XX_I2CONCLR);
    i2c.msg_status = 0;
    }
// Message is done
    if (i2c.msg_idx >= i2c.msg.len - 1) {
    i2c.msg_status = 0;
    disable_irq_nosync(i2c.irq);
    }
//
// One pre-last data input, send NACK to tell the target that
// this is going to be the last data byte to be transferred.
//
    if (i2c.msg_idx >= i2c.msg.len - 2) {
// One byte left to receive - NACK
    writel(LPC24XX_AA, i2c.base + LPC24XX_I2CONCLR);
    } else {
// More than one byte left to receive - ACK
    writel(LPC24XX_AA, i2c.base + LPC24XX_I2CONSET);
    }
    writel(LPC24XX_STA, i2c.base + LPC24XX_I2CONCLR);
    i2c.msg_idx++;
    break;
    case MX_ADDR_W_NACK:
    case MX_DATA_W_NACK:
    case MR_ADDR_R_NACK:
// NACK processing is done
    writel(LPC24XX_STO_AA, i2c.base + LPC24XX_I2CONSET);
    i2c.msg_status = -ENXIO;
    disable_irq_nosync(i2c.irq);
    break;
    case M_DATA_ARB_LOST:
// Arbitration lost
    i2c.msg_status = -EAGAIN;
// Release the I2C bus
    writel(LPC24XX_STA | LPC24XX_STO, i2c.base + LPC24XX_I2CONCLR);
    disable_irq_nosync(i2c.irq);
    break;
    default:
// Unexpected statuses
    i2c.msg_status = -EIO;
    disable_irq_nosync(i2c.irq);
    break;
    }
// Exit on failure or all bytes transferred
    if (i2c.msg_status != -EBUSY)
    wake_up(&i2c.wait);
//
// If `msg_status` is zero, then `lpc2k_process_msg()`
// is responsible for clearing the SI flag.
//
    if (i2c.msg_status != 0)
    writel(LPC24XX_SI, i2c.base + LPC24XX_I2CONCLR);
    }
#[no_mangle]
unsafe extern "C" fn lpc2k_process_msg(i2c: *mut lpc2k_i2c, msgidx: c_int) -> c_int {
    static int lpc2k_process_msg(struct lpc2k_i2c *i2c, int msgidx)
    {
// A new transfer is kicked off by initiating a start condition
    if (!msgidx) {
    writel(LPC24XX_STA, i2c.base + LPC24XX_I2CONSET);
    } else {
//
// A multi-message I2C transfer continues where the
// previous I2C transfer left off and uses the
// current condition of the I2C adapter.
//
    if (unlikely(i2c.msg.flags & I2C_M_NOSTART)) {
    WARN_ON(i2c.msg.len == 0);
    if (!(i2c.msg.flags & I2C_M_RD)) {
// Start transmit of data
    writel(i2c.msg.buf[0],
    i2c.base + LPC24XX_I2DAT);
    i2c.msg_idx++;
    }
    } else {
// Start or repeated start
    writel(LPC24XX_STA, i2c.base + LPC24XX_I2CONSET);
    }
    writel(LPC24XX_SI, i2c.base + LPC24XX_I2CONCLR);
    }
    enable_irq(i2c.irq);
// Wait for transfer completion
    if (wait_event_timeout(i2c.wait, i2c.msg_status != -EBUSY,
    msecs_to_jiffies(1000)) == 0) {
    disable_irq_nosync(i2c.irq);
    return -ETIMEDOUT;
    }
    return i2c.msg_status;
    }
    static int i2c_lpc2k_xfer(struct i2c_adapter *adap, struct i2c_msg *msgs,
    int msg_num)
    {
    struct lpc2k_i2c *i2c = i2c_get_adapdata(adap);
    int ret, i;
    u32 stat;
// Check for bus idle condition
    stat = readl(i2c.base + LPC24XX_I2STAT);
    if (stat != M_I2C_IDLE) {
// Something is holding the bus, try to clear it
    return i2c_lpc2k_clear_arb(i2c);
    }
// Process a single message at a time
    for (i = 0; i < msg_num; i++) {
// Save message pointer and current message data index
    i2c.msg = &msgs[i];
    i2c.msg_idx = 0;
    i2c.msg_status = -EBUSY;
    i2c.is_last = (i == (msg_num - 1));
    ret = lpc2k_process_msg(i2c, i);
    if (ret)
    return ret;
    }
    return msg_num;
    }
#[no_mangle]
unsafe extern "C" fn i2c_lpc2k_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t i2c_lpc2k_handler(int irq, void *dev_id)
    {
    struct lpc2k_i2c *i2c = dev_id;
    if (readl(i2c.base + LPC24XX_I2CONSET) & LPC24XX_SI) {
    i2c_lpc2k_pump_msg(i2c);
    return IRQ_HANDLED;
    }
    return IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn i2c_lpc2k_functionality(adap: *mut i2c_adapter) -> u32 {
    static u32 i2c_lpc2k_functionality(struct i2c_adapter *adap)
    {
// Only emulated SMBus for now
    return I2C_FUNC_I2C | I2C_FUNC_SMBUS_EMUL;
    }
    static const struct i2c_algorithm i2c_lpc2k_algorithm = {
    .xfer = i2c_lpc2k_xfer,
    .functionality = i2c_lpc2k_functionality,
    };
#[no_mangle]
unsafe extern "C" fn i2c_lpc2k_probe(pdev: *mut platform_device) -> c_int {
    static int i2c_lpc2k_probe(struct platform_device *pdev)
    {
    struct lpc2k_i2c *i2c;
    u32 bus_clk_rate;
    u32 scl_high;
    u32 clkrate;
    int ret;
    i2c = devm_kzalloc(&pdev.dev, sizeof(*i2c), GFP_KERNEL);
    if (!i2c)
    return -ENOMEM;
    i2c.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(i2c.base))
    return PTR_ERR(i2c.base);
    i2c.irq = platform_get_irq(pdev, 0);
    if (i2c.irq < 0)
    return i2c.irq;
    init_waitqueue_head(&i2c.wait);
    i2c.clk = devm_clk_get_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(i2c.clk)) {
    dev_err(&pdev.dev, "failed to enable clock.\n");
    return PTR_ERR(i2c.clk);
    }
    ret = devm_request_irq(&pdev.dev, i2c.irq, i2c_lpc2k_handler, 0,
    dev_name(&pdev.dev), i2c);
    if (ret < 0) {
    dev_err(&pdev.dev, "can't request interrupt.\n");
    return ret;
    }
    disable_irq_nosync(i2c.irq);
// Place controller is a known state
    i2c_lpc2k_reset(i2c);
    ret = of_property_read_u32(pdev.dev.of_node, "clock-frequency",
    &bus_clk_rate);
    if (ret)
    bus_clk_rate = I2C_MAX_STANDARD_MODE_FREQ;
    clkrate = clk_get_rate(i2c.clk);
    if (clkrate == 0) {
    dev_err(&pdev.dev, "can't get I2C base clock\n");
    return -EINVAL;
    }
// Setup I2C dividers to generate clock with proper duty cycle
    clkrate = clkrate / bus_clk_rate;
    if (bus_clk_rate <= I2C_MAX_STANDARD_MODE_FREQ)
    scl_high = (clkrate * I2C_STD_MODE_DUTY) / 100;
#[no_mangle]
pub unsafe extern "C" fn if(I2C_MAX_FAST_MODE_FREQ: bus_clk_rate <=) -> else {
    else if (bus_clk_rate <= I2C_MAX_FAST_MODE_FREQ)
    scl_high = (clkrate * I2C_FAST_MODE_DUTY) / 100;
    else
    scl_high = (clkrate * I2C_FAST_MODE_PLUS_DUTY) / 100;
    writel(scl_high, i2c.base + LPC24XX_I2SCLH);
    writel(clkrate - scl_high, i2c.base + LPC24XX_I2SCLL);
    platform_set_drvdata(pdev, i2c);
    i2c_set_adapdata(&i2c.adap, i2c);
    i2c.adap.owner = THIS_MODULE;
    strscpy(i2c.adap.name, "LPC2K I2C adapter", sizeof(i2c.adap.name));
    i2c.adap.algo = &i2c_lpc2k_algorithm;
    i2c.adap.dev.parent = &pdev.dev;
    i2c.adap.dev.of_node = pdev.dev.of_node;
    ret = i2c_add_adapter(&i2c.adap);
    if (ret < 0)
    return ret;
    dev_info(&pdev.dev, "LPC2K I2C adapter\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i2c_lpc2k_remove(dev: *mut platform_device) {
    static void i2c_lpc2k_remove(struct platform_device *dev)
    {
    struct lpc2k_i2c *i2c = platform_get_drvdata(dev);
    i2c_del_adapter(&i2c.adap);
    }
#[no_mangle]
unsafe extern "C" fn i2c_lpc2k_suspend(dev: *mut device) -> c_int {
    static int i2c_lpc2k_suspend(struct device *dev)
    {
    struct lpc2k_i2c *i2c = dev_get_drvdata(dev);
    clk_disable(i2c.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i2c_lpc2k_resume(dev: *mut device) -> c_int {
    static int i2c_lpc2k_resume(struct device *dev)
    {
    struct lpc2k_i2c *i2c = dev_get_drvdata(dev);
    int ret;
    ret = clk_enable(i2c.clk);
    if (ret) {
    dev_err(dev, "failed to enable clock.\n");
    return ret;
    }
    i2c_lpc2k_reset(i2c);
    return 0;
    }
    static const struct dev_pm_ops i2c_lpc2k_dev_pm_ops = {
    .suspend_noirq = i2c_lpc2k_suspend,
    .resume_noirq = i2c_lpc2k_resume,
    };
    static const struct of_device_id lpc2k_i2c_match[] = {
    { .compatible = "nxp,lpc1788-i2c" },
    {},
    };
    MODULE_DEVICE_TABLE(of, lpc2k_i2c_match);
    static struct platform_driver i2c_lpc2k_driver = {
    .probe	= i2c_lpc2k_probe,
    .remove = i2c_lpc2k_remove,
    .driver	= {
    .name		= "lpc2k-i2c",
    .pm		= pm_sleep_ptr(&i2c_lpc2k_dev_pm_ops),
    .of_match_table	= lpc2k_i2c_match,
    },
    };
    module_platform_driver(i2c_lpc2k_driver);
    MODULE_AUTHOR("Kevin Wells <kevin.wells@nxp.com>");
    MODULE_DESCRIPTION("I2C driver for LPC2xxx devices");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:lpc2k-i2c");
