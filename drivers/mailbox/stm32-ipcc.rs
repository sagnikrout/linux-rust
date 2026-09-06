//! Automatically rewritten from C to Rust
//! Source: drivers/mailbox/stm32-ipcc.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) STMicroelectronics 2018 - All Rights Reserved
// Authors: Ludovic Barre <ludovic.barre@st.com> for STMicroelectronics.
// Fabien Dessenne <fabien.dessenne@st.com> for STMicroelectronics.
//

pub const IPCC_XCR: c_uint = 0x000;

pub const IPCC_XMR: c_uint = 0x004;
pub const IPCC_XSCR: c_uint = 0x008;
pub const IPCC_XTOYSR: c_uint = 0x00c;
pub const IPCC_PROC_OFFST: c_uint = 0x010;
pub const IPCC_HWCFGR: c_uint = 0x3f0;

pub const IPCC_VER: c_uint = 0x3f4;

pub const TX_BIT_SHIFT: c_int = 16;

pub const STM32_MAX_PROCS: c_int = 2;
    enum {
    IPCC_IRQ_RX,
    IPCC_IRQ_TX,
    IPCC_IRQ_NUM,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_ipcc {
    pub controller: mbox_controller,
    pub reg_base: *mut void __iomem,
    pub reg_proc: *mut void __iomem,
    pub clk: *mut clk,
    pub /: *mut *mut spinlock_t lock; / protect access to IPCC registers,
    pub irqs: [c_int; IPCC_IRQ_NUM],
    pub proc_id: u32,
    pub n_chans: u32,
    pub xcr: u32,
    pub xmr: u32,
}

    static inline void stm32_ipcc_set_bits(spinlock_t *lock, void __iomem *reg,
    u32 mask)
    {
    unsigned long flags;
    spin_lock_irqsave(lock, flags);
    writel_relaxed(readl_relaxed(reg) | mask, reg);
    spin_unlock_irqrestore(lock, flags);
    }
    static inline void stm32_ipcc_clr_bits(spinlock_t *lock, void __iomem *reg,
    u32 mask)
    {
    unsigned long flags;
    spin_lock_irqsave(lock, flags);
    writel_relaxed(readl_relaxed(reg) & ~mask, reg);
    spin_unlock_irqrestore(lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn stm32_ipcc_rx_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t stm32_ipcc_rx_irq(int irq, void *data)
    {
    struct stm32_ipcc *ipcc = data;
    struct device *dev = ipcc.controller.dev;
    u32 status, mr, tosr, chan;
    let mut ret: irqreturn_t = IRQ_NONE;
    int proc_offset;
// read 'channel occupied' status from other proc
    proc_offset = ipcc.proc_id ? -IPCC_PROC_OFFST : IPCC_PROC_OFFST;
    tosr = readl_relaxed(ipcc.reg_proc + proc_offset + IPCC_XTOYSR);
    mr = readl_relaxed(ipcc.reg_proc + IPCC_XMR);
// search for unmasked 'channel occupied'
    status = tosr & FIELD_GET(RX_BIT_MASK, ~mr);
    for (chan = 0; chan < ipcc.n_chans; chan++) {
    if (!(status & (1 << chan)))
    continue;
    dev_dbg(dev, "%s: chan:%d rx\n", __func__, chan);
    mbox_chan_received_data(&ipcc.controller.chans[chan], core::ptr::null_mut());
    stm32_ipcc_set_bits(&ipcc.lock, ipcc.reg_proc + IPCC_XSCR,
    RX_BIT_CHAN(chan));
    ret = IRQ_HANDLED;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stm32_ipcc_tx_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t stm32_ipcc_tx_irq(int irq, void *data)
    {
    struct stm32_ipcc *ipcc = data;
    struct device *dev = ipcc.controller.dev;
    u32 status, mr, tosr, chan;
    let mut ret: irqreturn_t = IRQ_NONE;
    tosr = readl_relaxed(ipcc.reg_proc + IPCC_XTOYSR);
    mr = readl_relaxed(ipcc.reg_proc + IPCC_XMR);
// search for unmasked 'channel free'
    status = ~tosr & FIELD_GET(TX_BIT_MASK, ~mr);
    for (chan = 0; chan < ipcc.n_chans ; chan++) {
    if (!(status & (1 << chan)))
    continue;
    dev_dbg(dev, "%s: chan:%d tx\n", __func__, chan);
// mask 'tx channel free' interrupt
    stm32_ipcc_set_bits(&ipcc.lock, ipcc.reg_proc + IPCC_XMR,
    TX_BIT_CHAN(chan));
    mbox_chan_txdone(&ipcc.controller.chans[chan], 0);
    ret = IRQ_HANDLED;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stm32_ipcc_send_data(link: *mut mbox_chan, data: *mut c_void) -> c_int {
    static int stm32_ipcc_send_data(struct mbox_chan *link, void *data)
    {
    let mut chan: c_ulong = (unsigned long)link.con_priv;
    struct stm32_ipcc *ipcc = container_of(link.mbox, struct stm32_ipcc,
    controller);
    dev_dbg(ipcc.controller.dev, "%s: chan:%lu\n", __func__, chan);
// set channel n occupied
    stm32_ipcc_set_bits(&ipcc.lock, ipcc.reg_proc + IPCC_XSCR,
    TX_BIT_CHAN(chan));
// unmask 'tx channel free' interrupt
    stm32_ipcc_clr_bits(&ipcc.lock, ipcc.reg_proc + IPCC_XMR,
    TX_BIT_CHAN(chan));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_ipcc_startup(link: *mut mbox_chan) -> c_int {
    static int stm32_ipcc_startup(struct mbox_chan *link)
    {
    let mut chan: c_ulong = (unsigned long)link.con_priv;
    struct stm32_ipcc *ipcc = container_of(link.mbox, struct stm32_ipcc,
    controller);
    int ret;
    ret = clk_prepare_enable(ipcc.clk);
    if (ret) {
    dev_err(ipcc.controller.dev, "can not enable the clock\n");
    return ret;
    }
// unmask 'rx channel occupied' interrupt
    stm32_ipcc_clr_bits(&ipcc.lock, ipcc.reg_proc + IPCC_XMR,
    RX_BIT_CHAN(chan));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_ipcc_shutdown(link: *mut mbox_chan) {
    static void stm32_ipcc_shutdown(struct mbox_chan *link)
    {
    let mut chan: c_ulong = (unsigned long)link.con_priv;
    struct stm32_ipcc *ipcc = container_of(link.mbox, struct stm32_ipcc,
    controller);
// mask rx/tx interrupt
    stm32_ipcc_set_bits(&ipcc.lock, ipcc.reg_proc + IPCC_XMR,
    RX_BIT_CHAN(chan) | TX_BIT_CHAN(chan));
    clk_disable_unprepare(ipcc.clk);
    }
    static const struct mbox_chan_ops stm32_ipcc_ops = {
    .send_data	= stm32_ipcc_send_data,
    .startup	= stm32_ipcc_startup,
    .shutdown	= stm32_ipcc_shutdown,
    };
#[no_mangle]
unsafe extern "C" fn stm32_ipcc_probe(pdev: *mut platform_device) -> c_int {
    static int stm32_ipcc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct stm32_ipcc *ipcc;
    unsigned long i;
    int ret;
    u32 ip_ver;
    static const char * const irq_name[] = {"rx", "tx"};
    irq_handler_t irq_thread[] = {stm32_ipcc_rx_irq, stm32_ipcc_tx_irq};
    if (!np) {
    dev_err(dev, "No DT found\n");
    return -ENODEV;
    }
    ipcc = devm_kzalloc(dev, sizeof(*ipcc), GFP_KERNEL);
    if (!ipcc)
    return -ENOMEM;
    spin_lock_init(&ipcc.lock);
// proc_id
    if (of_property_read_u32(np, "st,proc-id", &ipcc.proc_id)) {
    dev_err(dev, "Missing st,proc-id\n");
    return -ENODEV;
    }
    if (ipcc.proc_id >= STM32_MAX_PROCS) {
    dev_err(dev, "Invalid proc_id (%d)\n", ipcc.proc_id);
    return -EINVAL;
    }
// regs
    ipcc.reg_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(ipcc.reg_base))
    return PTR_ERR(ipcc.reg_base);
    ipcc.reg_proc = ipcc.reg_base + ipcc.proc_id * IPCC_PROC_OFFST;
// clock
    ipcc.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(ipcc.clk))
    return PTR_ERR(ipcc.clk);
    ret = clk_prepare_enable(ipcc.clk);
    if (ret) {
    dev_err(dev, "can not enable the clock\n");
    return ret;
    }
// irq
    for (i = 0; i < IPCC_IRQ_NUM; i++) {
    ipcc.irqs[i] = platform_get_irq_byname(pdev, irq_name[i]);
    if (ipcc.irqs[i] < 0) {
    ret = ipcc.irqs[i];
    goto err_clk;
    }
    ret = devm_request_threaded_irq(dev, ipcc.irqs[i], core::ptr::null_mut(),
    irq_thread[i], IRQF_ONESHOT,
    dev_name(dev), ipcc);
    if (ret)
    goto err_clk;
    }
// mask and enable rx/tx irq
    stm32_ipcc_set_bits(&ipcc.lock, ipcc.reg_proc + IPCC_XMR,
    RX_BIT_MASK | TX_BIT_MASK);
    stm32_ipcc_set_bits(&ipcc.lock, ipcc.reg_proc + IPCC_XCR,
    XCR_RXOIE | XCR_TXOIE);
// wakeup
    if (of_property_read_bool(np, "wakeup-source")) {
    device_set_wakeup_capable(dev, true);
    ret = dev_pm_set_wake_irq(dev, ipcc.irqs[IPCC_IRQ_RX]);
    if (ret) {
    dev_err(dev, "Failed to set wake up irq\n");
    goto err_init_wkp;
    }
    }
// mailbox controller
    ipcc.n_chans = readl_relaxed(ipcc.reg_base + IPCC_HWCFGR);
    ipcc.n_chans &= IPCFGR_CHAN_MASK;
    ipcc.controller.dev = dev;
    ipcc.controller.txdone_irq = true;
    ipcc.controller.ops = &stm32_ipcc_ops;
    ipcc.controller.num_chans = ipcc.n_chans;
    ipcc.controller.chans = devm_kcalloc(dev, ipcc.controller.num_chans,
    sizeof(*ipcc.controller.chans),
    GFP_KERNEL);
    if (!ipcc.controller.chans) {
    ret = -ENOMEM;
    goto err_irq_wkp;
    }
    for (i = 0; i < ipcc.controller.num_chans; i++)
    ipcc.controller.chans[i].con_priv = (void *)i;
    ret = devm_mbox_controller_register(dev, &ipcc.controller);
    if (ret)
    goto err_irq_wkp;
    platform_set_drvdata(pdev, ipcc);
    ip_ver = readl_relaxed(ipcc.reg_base + IPCC_VER);
    dev_info(dev, "ipcc rev:%ld.%ld enabled, %d chans, proc %d\n",
    FIELD_GET(VER_MAJREV_MASK, ip_ver),
    FIELD_GET(VER_MINREV_MASK, ip_ver),
    ipcc.controller.num_chans, ipcc.proc_id);
    clk_disable_unprepare(ipcc.clk);
    return 0;
    err_irq_wkp:
    if (of_property_read_bool(np, "wakeup-source"))
    dev_pm_clear_wake_irq(dev);
    err_init_wkp:
    device_set_wakeup_capable(dev, false);
    err_clk:
    clk_disable_unprepare(ipcc.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stm32_ipcc_remove(pdev: *mut platform_device) {
    static void stm32_ipcc_remove(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    if (of_property_read_bool(dev.of_node, "wakeup-source"))
    dev_pm_clear_wake_irq(&pdev.dev);
    device_set_wakeup_capable(dev, false);
    }

#[no_mangle]
unsafe extern "C" fn stm32_ipcc_suspend(dev: *mut device) -> c_int {
    static int stm32_ipcc_suspend(struct device *dev)
    {
    struct stm32_ipcc *ipcc = dev_get_drvdata(dev);
    ipcc.xmr = readl_relaxed(ipcc.reg_proc + IPCC_XMR);
    ipcc.xcr = readl_relaxed(ipcc.reg_proc + IPCC_XCR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_ipcc_resume(dev: *mut device) -> c_int {
    static int stm32_ipcc_resume(struct device *dev)
    {
    struct stm32_ipcc *ipcc = dev_get_drvdata(dev);
    writel_relaxed(ipcc.xmr, ipcc.reg_proc + IPCC_XMR);
    writel_relaxed(ipcc.xcr, ipcc.reg_proc + IPCC_XCR);
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(stm32_ipcc_pm_ops,
    stm32_ipcc_suspend, stm32_ipcc_resume);
    static const struct of_device_id stm32_ipcc_of_match[] = {
    { .compatible = "st,stm32mp1-ipcc" },
    {},
    };
    MODULE_DEVICE_TABLE(of, stm32_ipcc_of_match);
    static struct platform_driver stm32_ipcc_driver = {
    .driver = {
    .name = "stm32-ipcc",
    .pm = &stm32_ipcc_pm_ops,
    .of_match_table = stm32_ipcc_of_match,
    },
    .probe		= stm32_ipcc_probe,
    .remove		= stm32_ipcc_remove,
    };
    module_platform_driver(stm32_ipcc_driver);
    MODULE_AUTHOR("Ludovic Barre <ludovic.barre@st.com>");
    MODULE_AUTHOR("Fabien Dessenne <fabien.dessenne@st.com>");
    MODULE_DESCRIPTION("STM32 IPCC driver");
    MODULE_LICENSE("GPL v2");
