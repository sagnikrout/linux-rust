//! Automatically rewritten from C to Rust
//! Source: drivers/mailbox/pl320-ipc.c
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
// Copyright 2012 Calxeda, Inc.
//

pub const IPC_TX_MBOX: c_int = 1;
pub const IPC_RX_MBOX: c_int = 2;

pub const A9_SOURCE: c_int = 1;
pub const M3_SOURCE: c_int = 0;
    static void __iomem *ipc_base;
    static int ipc_irq;
    static DEFINE_MUTEX(ipc_m1_lock);
    static DECLARE_COMPLETION(ipc_completion);
    static ATOMIC_NOTIFIER_HEAD(ipc_notifier);
#[no_mangle]
unsafe extern "C" fn __ipc_send(mbox: c_int, data: *mut u32) {
    static void __ipc_send(int mbox, u32 *data)
    {
    int i;
    for (i = 0; i < 7; i++)
    writel_relaxed(data[i], ipc_base + IPCMxDR(mbox, i));
    writel_relaxed(0x1, ipc_base + IPCMxSEND(mbox));
    }
#[no_mangle]
unsafe extern "C" fn __ipc_rcv(mbox: c_int, data: *mut u32) -> u32 {
    static u32 __ipc_rcv(int mbox, u32 *data)
    {
    int i;
    for (i = 0; i < 7; i++)
    data[i] = readl_relaxed(ipc_base + IPCMxDR(mbox, i));
    return data[1];
    }
// blocking implementation from the A9 side, not usable in interrupts!
#[no_mangle]
pub unsafe extern "C" fn pl320_ipc_transmit(data: *mut u32) -> c_int {
    int pl320_ipc_transmit(u32 *data)
    {
    int ret;
    mutex_lock(&ipc_m1_lock);
    init_completion(&ipc_completion);
    __ipc_send(IPC_TX_MBOX, data);
    ret = wait_for_completion_timeout(&ipc_completion,
    msecs_to_jiffies(1000));
    if (ret == 0) {
    ret = -ETIMEDOUT;
    goto out;
    }
    ret = __ipc_rcv(IPC_TX_MBOX, data);
    out:
    mutex_unlock(&ipc_m1_lock);
    return ret;
    }
    EXPORT_SYMBOL_GPL(pl320_ipc_transmit);
#[no_mangle]
unsafe extern "C" fn ipc_handler(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t ipc_handler(int irq, void *dev)
    {
    u32 irq_stat;
    u32 data[7];
    irq_stat = readl_relaxed(ipc_base + IPCMMIS(1));
    if (irq_stat & MBOX_MASK(IPC_TX_MBOX)) {
    writel_relaxed(0, ipc_base + IPCMxSEND(IPC_TX_MBOX));
    complete(&ipc_completion);
    }
    if (irq_stat & MBOX_MASK(IPC_RX_MBOX)) {
    __ipc_rcv(IPC_RX_MBOX, data);
    atomic_notifier_call_chain(&ipc_notifier, data[0], data + 1);
    writel_relaxed(2, ipc_base + IPCMxSEND(IPC_RX_MBOX));
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
pub unsafe extern "C" fn pl320_ipc_register_notifier(nb: *mut notifier_block) -> c_int {
    int pl320_ipc_register_notifier(struct notifier_block *nb)
    {
    return atomic_notifier_chain_register(&ipc_notifier, nb);
    }
    EXPORT_SYMBOL_GPL(pl320_ipc_register_notifier);
#[no_mangle]
pub unsafe extern "C" fn pl320_ipc_unregister_notifier(nb: *mut notifier_block) -> c_int {
    int pl320_ipc_unregister_notifier(struct notifier_block *nb)
    {
    return atomic_notifier_chain_unregister(&ipc_notifier, nb);
    }
    EXPORT_SYMBOL_GPL(pl320_ipc_unregister_notifier);
#[no_mangle]
unsafe extern "C" fn pl320_probe(adev: *mut amba_device, id: *const amba_id) -> c_int {
    static int pl320_probe(struct amba_device *adev, const struct amba_id *id)
    {
    int ret;
    ipc_base = ioremap(adev.res.start, resource_size(&adev.res));
    if (ipc_base == core::ptr::null_mut())
    return -ENOMEM;
    writel_relaxed(0, ipc_base + IPCMxSEND(IPC_TX_MBOX));
    ipc_irq = adev.irq[0];
    ret = request_irq(ipc_irq, ipc_handler, 0, dev_name(&adev.dev), core::ptr::null_mut());
    if (ret < 0)
    goto err;
// Init slow mailbox
    writel_relaxed(CHAN_MASK(A9_SOURCE),
    ipc_base + IPCMxSOURCE(IPC_TX_MBOX));
    writel_relaxed(CHAN_MASK(M3_SOURCE),
    ipc_base + IPCMxDSET(IPC_TX_MBOX));
    writel_relaxed(CHAN_MASK(M3_SOURCE) | CHAN_MASK(A9_SOURCE),
    ipc_base + IPCMxMSET(IPC_TX_MBOX));
// Init receive mailbox
    writel_relaxed(CHAN_MASK(M3_SOURCE),
    ipc_base + IPCMxSOURCE(IPC_RX_MBOX));
    writel_relaxed(CHAN_MASK(A9_SOURCE),
    ipc_base + IPCMxDSET(IPC_RX_MBOX));
    writel_relaxed(CHAN_MASK(M3_SOURCE) | CHAN_MASK(A9_SOURCE),
    ipc_base + IPCMxMSET(IPC_RX_MBOX));
    return 0;
    err:
    iounmap(ipc_base);
    return ret;
    }
    static const struct amba_id pl320_ids[] = {
    {
    .id	= 0x00041320,
    .mask	= 0x000fffff,
    },
    { 0, 0 },
    };
    static struct amba_driver pl320_driver = {
    .drv = {
    .name	= "pl320",
    },
    .id_table	= pl320_ids,
    .probe		= pl320_probe,
    };
#[no_mangle]
unsafe extern "C" fn ipc_init() -> int __init {
    static int __init ipc_init(void)
    {
    return amba_driver_register(&pl320_driver);
    }
    subsys_initcall(ipc_init);
