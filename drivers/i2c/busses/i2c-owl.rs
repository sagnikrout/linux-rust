//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-owl.c
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
// Actions Semiconductor Owl SoC's I2C driver
//
// Copyright (c) 2014 Actions Semi Inc.
// Author: David Liu <liuwei@actions-semi.com>
//
// Copyright (c) 2018 Linaro Ltd.
// Author: Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>
//

// I2C registers
pub const OWL_I2C_REG_CTL: c_uint = 0x0000;
pub const OWL_I2C_REG_CLKDIV: c_uint = 0x0004;
pub const OWL_I2C_REG_STAT: c_uint = 0x0008;
pub const OWL_I2C_REG_ADDR: c_uint = 0x000C;
pub const OWL_I2C_REG_TXDAT: c_uint = 0x0010;
pub const OWL_I2C_REG_RXDAT: c_uint = 0x0014;
pub const OWL_I2C_REG_CMD: c_uint = 0x0018;
pub const OWL_I2C_REG_FIFOCTL: c_uint = 0x001C;
pub const OWL_I2C_REG_FIFOSTAT: c_uint = 0x0020;
pub const OWL_I2C_REG_DATCNT: c_uint = 0x0024;
pub const OWL_I2C_REG_RCNT: c_uint = 0x0028;
// I2Cx_CTL Bit Mask

// I2Cx_STAT Bit Mask

// I2Cx_CMD Bit Mask

// I2Cx_FIFOCTL Bit Mask

// I2Cc_FIFOSTAT Bit Mask

// I2C bus timeout

pub const OWL_I2C_MAX_RETRIES: c_int = 50;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct owl_i2c_dev {
    pub adap: i2c_adapter,
    pub msg: *mut i2c_msg,
    pub msg_complete: completion,
    pub clk: *mut clk,
    pub lock: spinlock_t,
    pub base: *mut void __iomem,
    pub clk_rate: c_ulong,
    pub bus_freq: u32,
    pub msg_ptr: u32,
    pub err: c_int,
}

#[no_mangle]
unsafe extern "C" fn owl_i2c_update_reg(reg: *mut void __iomem, val: c_uint, state: bool) {
    static void owl_i2c_update_reg(void __iomem *reg, unsigned int val, bool state)
    {
    unsigned int regval;
    regval = readl(reg);
    if (state)
    regval |= val;
    else
    regval &= ~val;
    writel(regval, reg);
    }
#[no_mangle]
unsafe extern "C" fn owl_i2c_reset(i2c_dev: *mut owl_i2c_dev) {
    static void owl_i2c_reset(struct owl_i2c_dev *i2c_dev)
    {
    owl_i2c_update_reg(i2c_dev.base + OWL_I2C_REG_CTL,
    OWL_I2C_CTL_EN, false);
    mdelay(1);
    owl_i2c_update_reg(i2c_dev.base + OWL_I2C_REG_CTL,
    OWL_I2C_CTL_EN, true);
// Clear status registers
    writel(0, i2c_dev.base + OWL_I2C_REG_STAT);
    }
#[no_mangle]
unsafe extern "C" fn owl_i2c_reset_fifo(i2c_dev: *mut owl_i2c_dev) -> c_int {
    static int owl_i2c_reset_fifo(struct owl_i2c_dev *i2c_dev)
    {
    unsigned int val, timeout = 0;
// Reset FIFO
    owl_i2c_update_reg(i2c_dev.base + OWL_I2C_REG_FIFOCTL,
    OWL_I2C_FIFOCTL_RFR | OWL_I2C_FIFOCTL_TFR,
    true);
// Wait 50ms for FIFO reset complete
    do {
    val = readl(i2c_dev.base + OWL_I2C_REG_FIFOCTL);
    if (!(val & (OWL_I2C_FIFOCTL_RFR | OWL_I2C_FIFOCTL_TFR)))
    break;
    usleep_range(500, 1000);
    } while (timeout++ < OWL_I2C_MAX_RETRIES);
    if (timeout > OWL_I2C_MAX_RETRIES) {
    dev_err(&i2c_dev.adap.dev, "FIFO reset timeout\n");
    return -ETIMEDOUT;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn owl_i2c_set_freq(i2c_dev: *mut owl_i2c_dev) {
    static void owl_i2c_set_freq(struct owl_i2c_dev *i2c_dev)
    {
    unsigned int val;
    val = DIV_ROUND_UP(i2c_dev.clk_rate, i2c_dev.bus_freq * 16);
// Set clock divider factor
    writel(OWL_I2C_DIV_FACTOR(val), i2c_dev.base + OWL_I2C_REG_CLKDIV);
    }
#[no_mangle]
unsafe extern "C" fn owl_i2c_xfer_data(i2c_dev: *mut owl_i2c_dev) {
    static void owl_i2c_xfer_data(struct owl_i2c_dev *i2c_dev)
    {
    struct i2c_msg *msg = i2c_dev.msg;
    unsigned int stat, fifostat;
    i2c_dev.err = 0;
// Handle NACK from target
    fifostat = readl(i2c_dev.base + OWL_I2C_REG_FIFOSTAT);
    if (fifostat & OWL_I2C_FIFOSTAT_RNB) {
    i2c_dev.err = -ENXIO;
// Clear NACK error bit by writing "1"
    owl_i2c_update_reg(i2c_dev.base + OWL_I2C_REG_FIFOSTAT,
    OWL_I2C_FIFOSTAT_RNB, true);
    return;
    }
// Handle bus error
    stat = readl(i2c_dev.base + OWL_I2C_REG_STAT);
    if (stat & OWL_I2C_STAT_BEB) {
    i2c_dev.err = -EIO;
// Clear BUS error bit by writing "1"
    owl_i2c_update_reg(i2c_dev.base + OWL_I2C_REG_STAT,
    OWL_I2C_STAT_BEB, true);
    return;
    }
// Handle FIFO read
    if (msg.flags & I2C_M_RD) {
    while ((readl(i2c_dev.base + OWL_I2C_REG_FIFOSTAT) &
    OWL_I2C_FIFOSTAT_RFE) && i2c_dev.msg_ptr < msg.len) {
    msg.buf[i2c_dev.msg_ptr++] = readl(i2c_dev.base +
    OWL_I2C_REG_RXDAT);
    }
    } else {
// Handle the remaining bytes which were not sent
    while (!(readl(i2c_dev.base + OWL_I2C_REG_FIFOSTAT) &
    OWL_I2C_FIFOSTAT_TFF) && i2c_dev.msg_ptr < msg.len) {
    writel(msg.buf[i2c_dev.msg_ptr++],
    i2c_dev.base + OWL_I2C_REG_TXDAT);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn owl_i2c_interrupt(irq: c_int, _dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t owl_i2c_interrupt(int irq, void *_dev)
    {
    struct owl_i2c_dev *i2c_dev = _dev;
    spin_lock(&i2c_dev.lock);
    owl_i2c_xfer_data(i2c_dev);
// Clear pending interrupts
    owl_i2c_update_reg(i2c_dev.base + OWL_I2C_REG_STAT,
    OWL_I2C_STAT_IRQP, true);
    complete_all(&i2c_dev.msg_complete);
    spin_unlock(&i2c_dev.lock);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn owl_i2c_func(adap: *mut i2c_adapter) -> u32 {
    static u32 owl_i2c_func(struct i2c_adapter *adap)
    {
    return I2C_FUNC_I2C | I2C_FUNC_SMBUS_EMUL;
    }
#[no_mangle]
unsafe extern "C" fn owl_i2c_check_bus_busy(adap: *mut i2c_adapter) -> c_int {
    static int owl_i2c_check_bus_busy(struct i2c_adapter *adap)
    {
    struct owl_i2c_dev *i2c_dev = i2c_get_adapdata(adap);
    unsigned long timeout;
// Check for Bus busy
    timeout = jiffies + OWL_I2C_TIMEOUT;
    while (readl(i2c_dev.base + OWL_I2C_REG_STAT) & OWL_I2C_STAT_BBB) {
    if (time_after(jiffies, timeout)) {
    dev_err(&adap.dev, "Bus busy timeout\n");
    return -ETIMEDOUT;
    }
    }
    return 0;
    }
    static int owl_i2c_xfer_common(struct i2c_adapter *adap, struct i2c_msg *msgs,
    int num, bool atomic)
    {
    struct owl_i2c_dev *i2c_dev = i2c_get_adapdata(adap);
    struct i2c_msg *msg;
    unsigned long time_left, flags;
    unsigned int i2c_cmd, val;
    unsigned int addr;
    int ret, idx;
    spin_lock_irqsave(&i2c_dev.lock, flags);
// Reset I2C controller
    owl_i2c_reset(i2c_dev);
// Set bus frequency
    owl_i2c_set_freq(i2c_dev);
//
// Spinlock should be released before calling reset FIFO and
// bus busy check since those functions may sleep
//
    spin_unlock_irqrestore(&i2c_dev.lock, flags);
// Reset FIFO
    ret = owl_i2c_reset_fifo(i2c_dev);
    if (ret)
    goto unlocked_err_exit;
// Check for bus busy
    ret = owl_i2c_check_bus_busy(adap);
    if (ret)
    goto unlocked_err_exit;
    spin_lock_irqsave(&i2c_dev.lock, flags);
// Check for Arbitration lost
    val = readl(i2c_dev.base + OWL_I2C_REG_STAT);
    if (val & OWL_I2C_STAT_LAB) {
    val &= ~OWL_I2C_STAT_LAB;
    writel(val, i2c_dev.base + OWL_I2C_REG_STAT);
    ret = -EAGAIN;
    goto err_exit;
    }
    if (!atomic)
    reinit_completion(&i2c_dev.msg_complete);
// Enable/disable I2C controller interrupt
    owl_i2c_update_reg(i2c_dev.base + OWL_I2C_REG_CTL,
    OWL_I2C_CTL_IRQE, !atomic);
//
// Select: FIFO enable, controller mode, Stop enable, Data count enable,
// Send start bit
//
    i2c_cmd = OWL_I2C_CMD_SECL | OWL_I2C_CMD_MSS | OWL_I2C_CMD_SE |
    OWL_I2C_CMD_NS | OWL_I2C_CMD_DE | OWL_I2C_CMD_SBE;
// Handle repeated start condition
    if (num > 1) {
// Set internal address length and enable repeated start
    i2c_cmd |= OWL_I2C_CMD_AS(msgs[0].len + 1) |
    OWL_I2C_CMD_SAS(1) | OWL_I2C_CMD_RBE;
// Write target address
    addr = i2c_8bit_addr_from_msg(&msgs[0]);
    writel(addr, i2c_dev.base + OWL_I2C_REG_TXDAT);
// Write internal register address
    for (idx = 0; idx < msgs[0].len; idx++)
    writel(msgs[0].buf[idx],
    i2c_dev.base + OWL_I2C_REG_TXDAT);
    msg = &msgs[1];
    } else {
// Set address length
    i2c_cmd |= OWL_I2C_CMD_AS(1);
    msg = &msgs[0];
    }
    i2c_dev.msg = msg;
    i2c_dev.msg_ptr = 0;
// Set data count for the message
    writel(msg.len, i2c_dev.base + OWL_I2C_REG_DATCNT);
    addr = i2c_8bit_addr_from_msg(msg);
    writel(addr, i2c_dev.base + OWL_I2C_REG_TXDAT);
    if (!(msg.flags & I2C_M_RD)) {
// Write data to FIFO
    for (idx = 0; idx < msg.len; idx++) {
// Check for FIFO full
    if (readl(i2c_dev.base + OWL_I2C_REG_FIFOSTAT) &
    OWL_I2C_FIFOSTAT_TFF)
    break;
    writel(msg.buf[idx],
    i2c_dev.base + OWL_I2C_REG_TXDAT);
    }
    i2c_dev.msg_ptr = idx;
    }
// Ignore the NACK if needed
    if (msg.flags & I2C_M_IGNORE_NAK)
    owl_i2c_update_reg(i2c_dev.base + OWL_I2C_REG_FIFOCTL,
    OWL_I2C_FIFOCTL_NIB, true);
    else
    owl_i2c_update_reg(i2c_dev.base + OWL_I2C_REG_FIFOCTL,
    OWL_I2C_FIFOCTL_NIB, false);
// Start the transfer
    writel(i2c_cmd, i2c_dev.base + OWL_I2C_REG_CMD);
    spin_unlock_irqrestore(&i2c_dev.lock, flags);
    if (atomic) {
// Wait for Command Execute Completed or NACK Error bits
    ret = readl_poll_timeout_atomic(i2c_dev.base + OWL_I2C_REG_FIFOSTAT,
    val, val & (OWL_I2C_FIFOSTAT_CECB |
    OWL_I2C_FIFOSTAT_RNB),
    10, OWL_I2C_TIMEOUT_MS * 1000);
    } else {
    time_left = wait_for_completion_timeout(&i2c_dev.msg_complete,
    adap.timeout);
    if (!time_left)
    ret = -ETIMEDOUT;
    }
    spin_lock_irqsave(&i2c_dev.lock, flags);
    if (ret) {
    dev_err(&adap.dev, "Transaction timed out\n");
// Send stop condition and release the bus
    owl_i2c_update_reg(i2c_dev.base + OWL_I2C_REG_CTL,
    OWL_I2C_CTL_GBCC_STOP | OWL_I2C_CTL_RB,
    true);
    goto err_exit;
    }
    if (atomic)
    owl_i2c_xfer_data(i2c_dev);
    ret = i2c_dev.err < 0 ? i2c_dev.err : num;
    err_exit:
    spin_unlock_irqrestore(&i2c_dev.lock, flags);
    unlocked_err_exit:
// Disable I2C controller
    owl_i2c_update_reg(i2c_dev.base + OWL_I2C_REG_CTL,
    OWL_I2C_CTL_EN, false);
    return ret;
    }
    static int owl_i2c_xfer(struct i2c_adapter *adap, struct i2c_msg *msgs,
    int num)
    {
    return owl_i2c_xfer_common(adap, msgs, num, false);
    }
    static int owl_i2c_xfer_atomic(struct i2c_adapter *adap,
    struct i2c_msg *msgs, int num)
    {
    return owl_i2c_xfer_common(adap, msgs, num, true);
    }
    static const struct i2c_algorithm owl_i2c_algorithm = {
    .xfer = owl_i2c_xfer,
    .xfer_atomic = owl_i2c_xfer_atomic,
    .functionality = owl_i2c_func,
    };
    static const struct i2c_adapter_quirks owl_i2c_quirks = {
    .flags		= I2C_AQ_COMB | I2C_AQ_COMB_WRITE_FIRST,
    .max_read_len   = 240,
    .max_write_len  = 240,
    .max_comb_1st_msg_len = 6,
    .max_comb_2nd_msg_len = 240,
    };
#[no_mangle]
unsafe extern "C" fn owl_i2c_probe(pdev: *mut platform_device) -> c_int {
    static int owl_i2c_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct owl_i2c_dev *i2c_dev;
    int ret, irq;
    i2c_dev = devm_kzalloc(dev, sizeof(*i2c_dev), GFP_KERNEL);
    if (!i2c_dev)
    return -ENOMEM;
    i2c_dev.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(i2c_dev.base))
    return PTR_ERR(i2c_dev.base);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    if (of_property_read_u32(dev.of_node, "clock-frequency",
    &i2c_dev.bus_freq))
    i2c_dev.bus_freq = I2C_MAX_STANDARD_MODE_FREQ;
// We support only frequencies of 100k and 400k for now
    if (i2c_dev.bus_freq != I2C_MAX_STANDARD_MODE_FREQ &&
    i2c_dev.bus_freq != I2C_MAX_FAST_MODE_FREQ) {
    dev_err(dev, "invalid clock-frequency %d\n", i2c_dev.bus_freq);
    return -EINVAL;
    }
    i2c_dev.clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(i2c_dev.clk)) {
    dev_err(dev, "failed to enable clock\n");
    return PTR_ERR(i2c_dev.clk);
    }
    i2c_dev.clk_rate = clk_get_rate(i2c_dev.clk);
    if (!i2c_dev.clk_rate) {
    dev_err(dev, "input clock rate should not be zero\n");
    return -EINVAL;
    }
    init_completion(&i2c_dev.msg_complete);
    spin_lock_init(&i2c_dev.lock);
    i2c_dev.adap.owner = THIS_MODULE;
    i2c_dev.adap.algo = &owl_i2c_algorithm;
    i2c_dev.adap.timeout = OWL_I2C_TIMEOUT;
    i2c_dev.adap.quirks = &owl_i2c_quirks;
    i2c_dev.adap.dev.parent = dev;
    i2c_dev.adap.dev.of_node = dev.of_node;
    snprintf(i2c_dev.adap.name, sizeof(i2c_dev.adap.name),
    "%s", "OWL I2C adapter");
    i2c_set_adapdata(&i2c_dev.adap, i2c_dev);
    platform_set_drvdata(pdev, i2c_dev);
    ret = devm_request_irq(dev, irq, owl_i2c_interrupt, 0, pdev.name,
    i2c_dev);
    if (ret) {
    dev_err(dev, "failed to request irq %d\n", irq);
    return ret;
    }
    return i2c_add_adapter(&i2c_dev.adap);
    }
    static const struct of_device_id owl_i2c_of_match[] = {
    { .compatible = "actions,s500-i2c" },
    { .compatible = "actions,s700-i2c" },
    { .compatible = "actions,s900-i2c" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, owl_i2c_of_match);
    static struct platform_driver owl_i2c_driver = {
    .probe		= owl_i2c_probe,
    .driver		= {
    .name	= "owl-i2c",
    .of_match_table = owl_i2c_of_match,
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    };
    module_platform_driver(owl_i2c_driver);
    MODULE_AUTHOR("David Liu <liuwei@actions-semi.com>");
    MODULE_AUTHOR("Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>");
    MODULE_DESCRIPTION("Actions Semiconductor Owl SoC's I2C driver");
    MODULE_LICENSE("GPL");
