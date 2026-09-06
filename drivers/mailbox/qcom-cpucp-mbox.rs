//! Automatically rewritten from C to Rust
//! Source: drivers/mailbox/qcom-cpucp-mbox.c
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
// Copyright (c) 2024, Qualcomm Innovation Center, Inc. All rights reserved.
//

pub const APSS_CPUCP_MBOX_CMD_OFF: c_uint = 0x4;
// Tx Registers

// Rx Registers

pub const APSS_CPUCP_RX_MBOX_MAP: c_uint = 0x4000;
pub const APSS_CPUCP_RX_MBOX_STAT: c_uint = 0x4400;
pub const APSS_CPUCP_RX_MBOX_CLEAR: c_uint = 0x4800;
pub const APSS_CPUCP_RX_MBOX_EN: c_uint = 0x4c00;

//
// struct qcom_cpucp_mbox_data - Per-hardware mailbox configuration data
// @num_chans:			Number of IPC channels supported by this hardware
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_cpucp_mbox_data {
    pub num_chans: c_int,
}

//
// struct qcom_cpucp_mbox - Holder for the mailbox driver
// @chans:			The mailbox channel
// @mbox:			The mailbox controller
// @tx_base:			Base address of the CPUCP tx registers
// @rx_base:			Base address of the CPUCP rx registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_cpucp_mbox {
    pub chans: *mut mbox_chan,
    pub mbox: mbox_controller,
    pub tx_base: *mut void __iomem,
    pub rx_base: *mut void __iomem,
}

#[no_mangle]
pub unsafe extern "C" fn channel_number(chan: *mut mbox_chan) -> c_int {
    static inline int channel_number(struct mbox_chan *chan)
    {
    return chan - chan.mbox.chans;
    }
#[no_mangle]
unsafe extern "C" fn qcom_cpucp_mbox_irq_fn(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t qcom_cpucp_mbox_irq_fn(int irq, void *data)
    {
    struct qcom_cpucp_mbox *cpucp = data;
    u64 status;
    int i;
    status = readq(cpucp.rx_base + APSS_CPUCP_RX_MBOX_STAT);
    for_each_set_bit(i, (unsigned long *)&status, cpucp.mbox.num_chans) {
    let mut val: u32 = readl(cpucp.rx_base + APSS_CPUCP_RX_MBOX_CMD(i) + APSS_CPUCP_MBOX_CMD_OFF);
    struct mbox_chan *chan = &cpucp.chans[i];
    struct mbox_client *cl;
    unsigned long flags;
//
// Provide mutual exclusion with changes to chan->cl.
// Save cl locally and clear the HW interrupt inside the lock,
// then invoke mbox_chan_received_data() outside the lock to
// avoid a PREEMPT_RT self-deadlock: mbox_chan_received_data()
// can call back into mbox_send_message() via scmi_rx_callback()
// -> mailbox_clear_channel(), which re-acquires chan->lock
// (converted to an rt_spinlock under PREEMPT_RT).
//
    spin_lock_irqsave(&chan.lock, flags);
    cl = chan.cl;
    writeq(BIT(i), cpucp.rx_base + APSS_CPUCP_RX_MBOX_CLEAR);
    spin_unlock_irqrestore(&chan.lock, flags);
    if (cl)
    mbox_chan_received_data(chan, &val);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn qcom_cpucp_mbox_startup(chan: *mut mbox_chan) -> c_int {
    static int qcom_cpucp_mbox_startup(struct mbox_chan *chan)
    {
    struct qcom_cpucp_mbox *cpucp = container_of(chan.mbox, struct qcom_cpucp_mbox, mbox);
    let mut chan_id: c_ulong = channel_number(chan);
    u64 val;
    val = readq(cpucp.rx_base + APSS_CPUCP_RX_MBOX_EN);
    val |= BIT(chan_id);
    writeq(val, cpucp.rx_base + APSS_CPUCP_RX_MBOX_EN);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_cpucp_mbox_shutdown(chan: *mut mbox_chan) {
    static void qcom_cpucp_mbox_shutdown(struct mbox_chan *chan)
    {
    struct qcom_cpucp_mbox *cpucp = container_of(chan.mbox, struct qcom_cpucp_mbox, mbox);
    let mut chan_id: c_ulong = channel_number(chan);
    u64 val;
    val = readq(cpucp.rx_base + APSS_CPUCP_RX_MBOX_EN);
    val &= ~BIT(chan_id);
    writeq(val, cpucp.rx_base + APSS_CPUCP_RX_MBOX_EN);
    }
#[no_mangle]
unsafe extern "C" fn qcom_cpucp_mbox_send_data(chan: *mut mbox_chan, data: *mut c_void) -> c_int {
    static int qcom_cpucp_mbox_send_data(struct mbox_chan *chan, void *data)
    {
    struct qcom_cpucp_mbox *cpucp = container_of(chan.mbox, struct qcom_cpucp_mbox, mbox);
    let mut chan_id: c_ulong = channel_number(chan);
    u32 *val = data;
//
// mailbox_clear_channel() calls mbox_send_message() with NULL data to
// signal the remote side that the channel has been cleared.  Nothing
// needs to be written to the TX register in that case, so just return.
//
    if (!val)
    return 0;
    writel(*val, cpucp.tx_base + APSS_CPUCP_TX_MBOX_CMD(chan_id) + APSS_CPUCP_MBOX_CMD_OFF);
    return 0;
    }
    static const struct mbox_chan_ops qcom_cpucp_mbox_chan_ops = {
    .startup = qcom_cpucp_mbox_startup,
    .send_data = qcom_cpucp_mbox_send_data,
    .shutdown = qcom_cpucp_mbox_shutdown
    };
#[no_mangle]
unsafe extern "C" fn qcom_cpucp_mbox_probe(pdev: *mut platform_device) -> c_int {
    static int qcom_cpucp_mbox_probe(struct platform_device *pdev)
    {
    const struct qcom_cpucp_mbox_data *data;
    struct device *dev = &pdev.dev;
    struct qcom_cpucp_mbox *cpucp;
    struct mbox_controller *mbox;
    int irq, ret;
    data = of_device_get_match_data(dev);
    if (!data)
    return dev_err_probe(dev, -EINVAL, "No match data found\n");
    cpucp = devm_kzalloc(dev, sizeof(*cpucp), GFP_KERNEL);
    if (!cpucp)
    return -ENOMEM;
    cpucp.chans = devm_kcalloc(dev, data.num_chans, sizeof(*cpucp.chans), GFP_KERNEL);
    if (!cpucp.chans)
    return -ENOMEM;
    cpucp.rx_base = devm_of_iomap(dev, dev.of_node, 0, core::ptr::null_mut());
    if (IS_ERR(cpucp.rx_base))
    return PTR_ERR(cpucp.rx_base);
    cpucp.tx_base = devm_of_iomap(dev, dev.of_node, 1, core::ptr::null_mut());
    if (IS_ERR(cpucp.tx_base))
    return PTR_ERR(cpucp.tx_base);
    writeq(0, cpucp.rx_base + APSS_CPUCP_RX_MBOX_EN);
    writeq(0, cpucp.rx_base + APSS_CPUCP_RX_MBOX_CLEAR);
    writeq(0, cpucp.rx_base + APSS_CPUCP_RX_MBOX_MAP);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    ret = devm_request_irq(dev, irq, qcom_cpucp_mbox_irq_fn,
    IRQF_TRIGGER_HIGH | IRQF_NO_SUSPEND, "apss_cpucp_mbox", cpucp);
    if (ret < 0)
    return ret;
    writeq(APSS_CPUCP_RX_MBOX_CMD_MASK, cpucp.rx_base + APSS_CPUCP_RX_MBOX_MAP);
    mbox = &cpucp.mbox;
    mbox.dev = dev;
    mbox.num_chans = data.num_chans;
    mbox.chans = cpucp.chans;
    mbox.ops = &qcom_cpucp_mbox_chan_ops;
    ret = devm_mbox_controller_register(dev, mbox);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to create mailbox\n");
    return 0;
    }
    static const struct qcom_cpucp_mbox_data qcom_x1e80100_mbox_data = {
    .num_chans = 3,
    };
    static const struct qcom_cpucp_mbox_data qcom_nord_mbox_data = {
    .num_chans = 16,
    };
    static const struct of_device_id qcom_cpucp_mbox_of_match[] = {
    { .compatible = "qcom,nord-cpucp-mbox", .data = &qcom_nord_mbox_data },
    { .compatible = "qcom,x1e80100-cpucp-mbox", .data = &qcom_x1e80100_mbox_data },
    {}
    };
    MODULE_DEVICE_TABLE(of, qcom_cpucp_mbox_of_match);
    static struct platform_driver qcom_cpucp_mbox_driver = {
    .probe = qcom_cpucp_mbox_probe,
    .driver = {
    .name = "qcom_cpucp_mbox",
    .of_match_table = qcom_cpucp_mbox_of_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn qcom_cpucp_mbox_init() -> int __init {
    static int __init qcom_cpucp_mbox_init(void)
    {
    return platform_driver_register(&qcom_cpucp_mbox_driver);
    }
    core_initcall(qcom_cpucp_mbox_init);
#[no_mangle]
unsafe extern "C" fn qcom_cpucp_mbox_exit() -> void __exit {
    static void __exit qcom_cpucp_mbox_exit(void)
    {
    platform_driver_unregister(&qcom_cpucp_mbox_driver);
    }
    module_exit(qcom_cpucp_mbox_exit);
    MODULE_DESCRIPTION("QTI CPUCP MBOX Driver");
    MODULE_LICENSE("GPL");
