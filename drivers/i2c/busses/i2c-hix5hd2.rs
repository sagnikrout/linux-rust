//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-hix5hd2.c
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
// Copyright (c) 2014 Linaro Ltd.
// Copyright (c) 2014 HiSilicon Limited.
//
// Now only support 7 bit address.
//

// Register Map
pub const HIX5I2C_CTRL: c_uint = 0x00;
pub const HIX5I2C_COM: c_uint = 0x04;
pub const HIX5I2C_ICR: c_uint = 0x08;
pub const HIX5I2C_SR: c_uint = 0x0c;
pub const HIX5I2C_SCL_H: c_uint = 0x10;
pub const HIX5I2C_SCL_L: c_uint = 0x14;
pub const HIX5I2C_TXR: c_uint = 0x18;
pub const HIX5I2C_RXR: c_uint = 0x1c;
// I2C_CTRL_REG

// I2C_COM_REG

// I2C_ICR_REG

    I2C_CLEAR_SEND | I2C_CLEAR_RECEIVE | \
    I2C_CLEAR_ACK | I2C_CLEAR_ARBITRATE | \
    I2C_CLEAR_OVER)
// I2C_SR_REG

    enum hix5hd2_i2c_state {
    HIX5I2C_STAT_RW_ERR = -1,
    HIX5I2C_STAT_INIT,
    HIX5I2C_STAT_RW,
    HIX5I2C_STAT_SND_STOP,
    HIX5I2C_STAT_RW_SUCCESS,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hix5hd2_i2c_priv {
    pub adap: i2c_adapter,
    pub msg: *mut i2c_msg,
    pub msg_complete: completion,
    pub msg_idx: c_uint,
    pub msg_len: c_uint,
    pub stop: c_int,
    pub regs: *mut void __iomem,
    pub clk: *mut clk,
    pub dev: *mut device,
    pub /: *mut *mut spinlock_t lock; / IRQ synchronization,
    pub err: c_int,
    pub freq: c_uint,
    pub state: enum hix5hd2_i2c_state,
}

#[no_mangle]
unsafe extern "C" fn hix5hd2_i2c_clr_pend_irq(priv: *mut hix5hd2_i2c_priv) -> u32 {
    static u32 hix5hd2_i2c_clr_pend_irq(struct hix5hd2_i2c_priv *priv)
    {
    let mut val: u32 = readl_relaxed(priv.regs + HIX5I2C_SR);
    writel_relaxed(val, priv.regs + HIX5I2C_ICR);
    return val;
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_i2c_clr_all_irq(priv: *mut hix5hd2_i2c_priv) {
    static void hix5hd2_i2c_clr_all_irq(struct hix5hd2_i2c_priv *priv)
    {
    writel_relaxed(I2C_CLEAR_ALL, priv.regs + HIX5I2C_ICR);
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_i2c_disable_irq(priv: *mut hix5hd2_i2c_priv) {
    static void hix5hd2_i2c_disable_irq(struct hix5hd2_i2c_priv *priv)
    {
    writel_relaxed(0, priv.regs + HIX5I2C_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_i2c_enable_irq(priv: *mut hix5hd2_i2c_priv) {
    static void hix5hd2_i2c_enable_irq(struct hix5hd2_i2c_priv *priv)
    {
    writel_relaxed(I2C_ENABLE | I2C_UNMASK_TOTAL | I2C_UNMASK_ALL,
    priv.regs + HIX5I2C_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_i2c_drv_setrate(priv: *mut hix5hd2_i2c_priv) {
    static void hix5hd2_i2c_drv_setrate(struct hix5hd2_i2c_priv *priv)
    {
    u32 rate, val;
    u32 scl, sysclock;
// close all i2c interrupt
    val = readl_relaxed(priv.regs + HIX5I2C_CTRL);
    writel_relaxed(val & (~I2C_UNMASK_TOTAL), priv.regs + HIX5I2C_CTRL);
    rate = priv.freq;
    sysclock = clk_get_rate(priv.clk);
    scl = (sysclock / (rate * 2)) / 2 - 1;
    writel_relaxed(scl, priv.regs + HIX5I2C_SCL_H);
    writel_relaxed(scl, priv.regs + HIX5I2C_SCL_L);
// restore original interrupt
    writel_relaxed(val, priv.regs + HIX5I2C_CTRL);
    dev_dbg(priv.dev, "%s: sysclock=%d, rate=%d, scl=%d\n",
    __func__, sysclock, rate, scl);
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_i2c_init(priv: *mut hix5hd2_i2c_priv) {
    static void hix5hd2_i2c_init(struct hix5hd2_i2c_priv *priv)
    {
    hix5hd2_i2c_disable_irq(priv);
    hix5hd2_i2c_drv_setrate(priv);
    hix5hd2_i2c_clr_all_irq(priv);
    hix5hd2_i2c_enable_irq(priv);
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_i2c_reset(priv: *mut hix5hd2_i2c_priv) {
    static void hix5hd2_i2c_reset(struct hix5hd2_i2c_priv *priv)
    {
    clk_disable_unprepare(priv.clk);
    msleep(20);
    clk_prepare_enable(priv.clk);
    hix5hd2_i2c_init(priv);
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_i2c_wait_bus_idle(priv: *mut hix5hd2_i2c_priv) -> c_int {
    static int hix5hd2_i2c_wait_bus_idle(struct hix5hd2_i2c_priv *priv)
    {
    unsigned long stop_time;
    u32 int_status;
// wait for 100 milli seconds for the bus to be idle
    stop_time = jiffies + msecs_to_jiffies(100);
    do {
    int_status = hix5hd2_i2c_clr_pend_irq(priv);
    if (!(int_status & I2C_BUSY))
    return 0;
    usleep_range(50, 200);
    } while (time_before(jiffies, stop_time));
    return -EBUSY;
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_rw_over(priv: *mut hix5hd2_i2c_priv) {
    static void hix5hd2_rw_over(struct hix5hd2_i2c_priv *priv)
    {
    if (priv.state == HIX5I2C_STAT_SND_STOP)
    dev_dbg(priv.dev, "%s: rw and send stop over\n", __func__);
    else
    dev_dbg(priv.dev, "%s: have not data to send\n", __func__);
    priv.state = HIX5I2C_STAT_RW_SUCCESS;
    priv.err = 0;
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_rw_handle_stop(priv: *mut hix5hd2_i2c_priv) {
    static void hix5hd2_rw_handle_stop(struct hix5hd2_i2c_priv *priv)
    {
    if (priv.stop) {
    priv.state = HIX5I2C_STAT_SND_STOP;
    writel_relaxed(I2C_STOP, priv.regs + HIX5I2C_COM);
    } else {
    hix5hd2_rw_over(priv);
    }
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_read_handle(priv: *mut hix5hd2_i2c_priv) {
    static void hix5hd2_read_handle(struct hix5hd2_i2c_priv *priv)
    {
    if (priv.msg_len == 1) {
// the last byte don't need send ACK
    writel_relaxed(I2C_READ | I2C_NO_ACK, priv.regs + HIX5I2C_COM);
    } else if (priv.msg_len > 1) {
// if i2c controller receive data will send ACK
    writel_relaxed(I2C_READ, priv.regs + HIX5I2C_COM);
    } else {
    hix5hd2_rw_handle_stop(priv);
    }
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_write_handle(priv: *mut hix5hd2_i2c_priv) {
    static void hix5hd2_write_handle(struct hix5hd2_i2c_priv *priv)
    {
    u8 data;
    if (priv.msg_len > 0) {
    data = priv.msg.buf[priv.msg_idx++];
    writel_relaxed(data, priv.regs + HIX5I2C_TXR);
    writel_relaxed(I2C_WRITE, priv.regs + HIX5I2C_COM);
    } else {
    hix5hd2_rw_handle_stop(priv);
    }
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_rw_preprocess(priv: *mut hix5hd2_i2c_priv) -> c_int {
    static int hix5hd2_rw_preprocess(struct hix5hd2_i2c_priv *priv)
    {
    u8 data;
    if (priv.state == HIX5I2C_STAT_INIT) {
    priv.state = HIX5I2C_STAT_RW;
    } else if (priv.state == HIX5I2C_STAT_RW) {
    if (priv.msg.flags & I2C_M_RD) {
    data = readl_relaxed(priv.regs + HIX5I2C_RXR);
    priv.msg.buf[priv.msg_idx++] = data;
    }
    priv.msg_len--;
    } else {
    dev_dbg(priv.dev, "%s: error: priv.state = %d, msg_len = %d\n",
    __func__, priv.state, priv.msg_len);
    return -EAGAIN;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_i2c_irq(irqno: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t hix5hd2_i2c_irq(int irqno, void *dev_id)
    {
    struct hix5hd2_i2c_priv *priv = dev_id;
    u32 int_status;
    int ret;
    spin_lock(&priv.lock);
    int_status = hix5hd2_i2c_clr_pend_irq(priv);
// handle error
    if (int_status & I2C_ARBITRATE_INTR) {
// bus error
    dev_dbg(priv.dev, "ARB bus loss\n");
    priv.err = -EAGAIN;
    priv.state = HIX5I2C_STAT_RW_ERR;
    goto stop;
    } else if (int_status & I2C_ACK_INTR) {
// ack error
    dev_dbg(priv.dev, "No ACK from device\n");
    priv.err = -ENXIO;
    priv.state = HIX5I2C_STAT_RW_ERR;
    goto stop;
    }
    if (int_status & I2C_OVER_INTR) {
    if (priv.msg_len > 0) {
    ret = hix5hd2_rw_preprocess(priv);
    if (ret) {
    priv.err = ret;
    priv.state = HIX5I2C_STAT_RW_ERR;
    goto stop;
    }
    if (priv.msg.flags & I2C_M_RD)
    hix5hd2_read_handle(priv);
    else
    hix5hd2_write_handle(priv);
    } else {
    hix5hd2_rw_over(priv);
    }
    }
    stop:
    if ((priv.state == HIX5I2C_STAT_RW_SUCCESS &&
    priv.msg.len == priv.msg_idx) ||
    (priv.state == HIX5I2C_STAT_RW_ERR)) {
    hix5hd2_i2c_disable_irq(priv);
    hix5hd2_i2c_clr_pend_irq(priv);
    complete(&priv.msg_complete);
    }
    spin_unlock(&priv.lock);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_i2c_message_start(priv: *mut hix5hd2_i2c_priv, stop: c_int) {
    static void hix5hd2_i2c_message_start(struct hix5hd2_i2c_priv *priv, int stop)
    {
    unsigned long flags;
    spin_lock_irqsave(&priv.lock, flags);
    hix5hd2_i2c_clr_all_irq(priv);
    hix5hd2_i2c_enable_irq(priv);
    writel_relaxed(i2c_8bit_addr_from_msg(priv.msg),
    priv.regs + HIX5I2C_TXR);
    writel_relaxed(I2C_WRITE | I2C_START, priv.regs + HIX5I2C_COM);
    spin_unlock_irqrestore(&priv.lock, flags);
    }
    static int hix5hd2_i2c_xfer_msg(struct hix5hd2_i2c_priv *priv,
    struct i2c_msg *msgs, int stop)
    {
    unsigned long time_left;
    int ret;
    priv.msg = msgs;
    priv.msg_idx = 0;
    priv.msg_len = priv.msg.len;
    priv.stop = stop;
    priv.err = 0;
    priv.state = HIX5I2C_STAT_INIT;
    reinit_completion(&priv.msg_complete);
    hix5hd2_i2c_message_start(priv, stop);
    time_left = wait_for_completion_timeout(&priv.msg_complete,
    priv.adap.timeout);
    if (time_left == 0) {
    priv.state = HIX5I2C_STAT_RW_ERR;
    priv.err = -ETIMEDOUT;
    dev_warn(priv.dev, "%s timeout=%d\n",
    msgs.flags & I2C_M_RD ? "rx" : "tx",
    priv.adap.timeout);
    }
    ret = priv.state;
//
// If this is the last message to be transferred (stop == 1)
// Then check if the bus can be brought back to idle.
//
    if (priv.state == HIX5I2C_STAT_RW_SUCCESS && stop)
    ret = hix5hd2_i2c_wait_bus_idle(priv);
    if (ret < 0)
    hix5hd2_i2c_reset(priv);
    return priv.err;
    }
    static int hix5hd2_i2c_xfer(struct i2c_adapter *adap,
    struct i2c_msg *msgs, int num)
    {
    struct hix5hd2_i2c_priv *priv = i2c_get_adapdata(adap);
    int i, ret, stop;
    pm_runtime_get_sync(priv.dev);
    for (i = 0; i < num; i++, msgs++) {
    if ((i == num - 1) || (msgs.flags & I2C_M_STOP))
    stop = 1;
    else
    stop = 0;
    ret = hix5hd2_i2c_xfer_msg(priv, msgs, stop);
    if (ret < 0)
    goto out;
    }
    ret = num;
    out:
    pm_runtime_put_autosuspend(priv.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_i2c_func(adap: *mut i2c_adapter) -> u32 {
    static u32 hix5hd2_i2c_func(struct i2c_adapter *adap)
    {
    return I2C_FUNC_I2C | (I2C_FUNC_SMBUS_EMUL & ~I2C_FUNC_SMBUS_QUICK);
    }
    static const struct i2c_algorithm hix5hd2_i2c_algorithm = {
    .xfer = hix5hd2_i2c_xfer,
    .functionality = hix5hd2_i2c_func,
    };
#[no_mangle]
unsafe extern "C" fn hix5hd2_i2c_probe(pdev: *mut platform_device) -> c_int {
    static int hix5hd2_i2c_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct hix5hd2_i2c_priv *priv;
    unsigned int freq;
    int irq, ret;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    if (of_property_read_u32(np, "clock-frequency", &freq)) {
// use 100k as default value
    priv.freq = I2C_MAX_STANDARD_MODE_FREQ;
    } else {
    if (freq > I2C_MAX_FAST_MODE_FREQ) {
    priv.freq = I2C_MAX_FAST_MODE_FREQ;
    dev_warn(priv.dev, "use max freq %d instead\n",
    I2C_MAX_FAST_MODE_FREQ);
    } else {
    priv.freq = freq;
    }
    }
    priv.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.regs))
    return PTR_ERR(priv.regs);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    priv.clk = devm_clk_get_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(priv.clk)) {
    dev_err(&pdev.dev, "cannot enable clock\n");
    return PTR_ERR(priv.clk);
    }
    strscpy(priv.adap.name, "hix5hd2-i2c", sizeof(priv.adap.name));
    priv.dev = &pdev.dev;
    priv.adap.owner = THIS_MODULE;
    priv.adap.algo = &hix5hd2_i2c_algorithm;
    priv.adap.retries = 3;
    priv.adap.dev.of_node = np;
    priv.adap.algo_data = priv;
    priv.adap.dev.parent = &pdev.dev;
    i2c_set_adapdata(&priv.adap, priv);
    platform_set_drvdata(pdev, priv);
    spin_lock_init(&priv.lock);
    init_completion(&priv.msg_complete);
    hix5hd2_i2c_init(priv);
    ret = devm_request_irq(&pdev.dev, irq, hix5hd2_i2c_irq,
    IRQF_NO_SUSPEND, dev_name(&pdev.dev), priv);
    if (ret != 0) {
    dev_err(&pdev.dev, "cannot request HS-I2C IRQ %d\n", irq);
    return ret;
    }
    pm_runtime_set_autosuspend_delay(priv.dev, MSEC_PER_SEC);
    pm_runtime_use_autosuspend(priv.dev);
    pm_runtime_set_active(priv.dev);
    pm_runtime_enable(priv.dev);
    ret = i2c_add_adapter(&priv.adap);
    if (ret < 0)
    goto err_runtime;
    return ret;
    err_runtime:
    pm_runtime_disable(priv.dev);
    pm_runtime_set_suspended(priv.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_i2c_remove(pdev: *mut platform_device) {
    static void hix5hd2_i2c_remove(struct platform_device *pdev)
    {
    struct hix5hd2_i2c_priv *priv = platform_get_drvdata(pdev);
    i2c_del_adapter(&priv.adap);
    pm_runtime_disable(priv.dev);
    pm_runtime_set_suspended(priv.dev);
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_i2c_runtime_suspend(dev: *mut device) -> c_int {
    static int hix5hd2_i2c_runtime_suspend(struct device *dev)
    {
    struct hix5hd2_i2c_priv *priv = dev_get_drvdata(dev);
    clk_disable_unprepare(priv.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_i2c_runtime_resume(dev: *mut device) -> c_int {
    static int hix5hd2_i2c_runtime_resume(struct device *dev)
    {
    struct hix5hd2_i2c_priv *priv = dev_get_drvdata(dev);
    clk_prepare_enable(priv.clk);
    hix5hd2_i2c_init(priv);
    return 0;
    }
    static const struct dev_pm_ops hix5hd2_i2c_pm_ops = {
    RUNTIME_PM_OPS(hix5hd2_i2c_runtime_suspend,
    hix5hd2_i2c_runtime_resume,
    core::ptr::null_mut())
    };
    static const struct of_device_id hix5hd2_i2c_match[] = {
    { .compatible = "hisilicon,hix5hd2-i2c" },
    {},
    };
    MODULE_DEVICE_TABLE(of, hix5hd2_i2c_match);
    static struct platform_driver hix5hd2_i2c_driver = {
    .probe		= hix5hd2_i2c_probe,
    .remove		= hix5hd2_i2c_remove,
    .driver		= {
    .name	= "hix5hd2-i2c",
    .pm	= pm_ptr(&hix5hd2_i2c_pm_ops),
    .of_match_table = hix5hd2_i2c_match,
    },
    };
    module_platform_driver(hix5hd2_i2c_driver);
    MODULE_DESCRIPTION("Hix5hd2 I2C Bus driver");
    MODULE_AUTHOR("Wei Yan <sledge.yanwei@huawei.com>");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:hix5hd2-i2c");
