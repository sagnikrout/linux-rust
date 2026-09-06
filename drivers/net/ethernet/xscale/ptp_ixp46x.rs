//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/xscale/ptp_ixp46x.c
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
// PTP 1588 clock using the IXP46X
//
// Copyright (C) 2010 OMICRON electronics GmbH
//

pub const N_EXT_TS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixp_clock {
    pub regs: *mut ixp46x_ts_regs,
    pub ptp_clock: *mut ptp_clock,
    pub caps: ptp_clock_info,
    pub exts0_enabled: c_int,
    pub exts1_enabled: c_int,
    pub slave_irq: c_int,
    pub master_irq: c_int,
}

    static DEFINE_SPINLOCK(register_lock);
//
// Register access functions
//
#[no_mangle]
unsafe extern "C" fn ixp_systime_read(regs: *mut ixp46x_ts_regs) -> u64 {
    static u64 ixp_systime_read(struct ixp46x_ts_regs *regs)
    {
    u64 ns;
    u32 lo, hi;
    lo = __raw_readl(&regs.systime_lo);
    hi = __raw_readl(&regs.systime_hi);
    ns = ((u64) hi) << 32;
    ns |= lo;
    ns <<= TICKS_NS_SHIFT;
    return ns;
    }
#[no_mangle]
unsafe extern "C" fn ixp_systime_write(regs: *mut ixp46x_ts_regs, ns: u64) {
    static void ixp_systime_write(struct ixp46x_ts_regs *regs, u64 ns)
    {
    u32 hi, lo;
    ns >>= TICKS_NS_SHIFT;
    hi = ns >> 32;
    lo = ns & 0xffffffff;
    __raw_writel(lo, &regs.systime_lo);
    __raw_writel(hi, &regs.systime_hi);
    }
//
// Interrupt service routine
//
#[no_mangle]
unsafe extern "C" fn isr(irq: c_int, priv: *mut c_void) -> irqreturn_t {
    static irqreturn_t isr(int irq, void *priv)
    {
    struct ixp_clock *ixp_clock = priv;
    struct ixp46x_ts_regs *regs = ixp_clock.regs;
    struct ptp_clock_event event;
    let mut ack: u32 = 0, lo, hi, val;
    val = __raw_readl(&regs.event);
    if (val & TSER_SNS) {
    ack |= TSER_SNS;
    if (ixp_clock.exts0_enabled) {
    hi = __raw_readl(&regs.asms_hi);
    lo = __raw_readl(&regs.asms_lo);
    event.type = PTP_CLOCK_EXTTS;
    event.index = 0;
    event.timestamp = ((u64) hi) << 32;
    event.timestamp |= lo;
    event.timestamp <<= TICKS_NS_SHIFT;
    ptp_clock_event(ixp_clock.ptp_clock, &event);
    }
    }
    if (val & TSER_SNM) {
    ack |= TSER_SNM;
    if (ixp_clock.exts1_enabled) {
    hi = __raw_readl(&regs.amms_hi);
    lo = __raw_readl(&regs.amms_lo);
    event.type = PTP_CLOCK_EXTTS;
    event.index = 1;
    event.timestamp = ((u64) hi) << 32;
    event.timestamp |= lo;
    event.timestamp <<= TICKS_NS_SHIFT;
    ptp_clock_event(ixp_clock.ptp_clock, &event);
    }
    }
    if (val & TTIPEND)
    ack |= TTIPEND; /* this bit seems to be always set */
    if (ack) {
    __raw_writel(ack, &regs.event);
    return IRQ_HANDLED;
    } else
    return IRQ_NONE;
    }
//
// PTP clock operations
//
#[no_mangle]
unsafe extern "C" fn ptp_ixp_adjfine(ptp: *mut ptp_clock_info, scaled_ppm: c_long) -> c_int {
    static int ptp_ixp_adjfine(struct ptp_clock_info *ptp, long scaled_ppm)
    {
    u32 addend;
    struct ixp_clock *ixp_clock = container_of(ptp, struct ixp_clock, caps);
    struct ixp46x_ts_regs *regs = ixp_clock.regs;
    addend = adjust_by_scaled_ppm(DEFAULT_ADDEND, scaled_ppm);
    __raw_writel(addend, &regs.addend);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ptp_ixp_adjtime(ptp: *mut ptp_clock_info, delta: i64) -> c_int {
    static int ptp_ixp_adjtime(struct ptp_clock_info *ptp, s64 delta)
    {
    s64 now;
    unsigned long flags;
    struct ixp_clock *ixp_clock = container_of(ptp, struct ixp_clock, caps);
    struct ixp46x_ts_regs *regs = ixp_clock.regs;
    spin_lock_irqsave(&register_lock, flags);
    now = ixp_systime_read(regs);
    now += delta;
    ixp_systime_write(regs, now);
    spin_unlock_irqrestore(&register_lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ptp_ixp_gettime(ptp: *mut ptp_clock_info, ts: *mut timespec64) -> c_int {
    static int ptp_ixp_gettime(struct ptp_clock_info *ptp, struct timespec64 *ts)
    {
    u64 ns;
    unsigned long flags;
    struct ixp_clock *ixp_clock = container_of(ptp, struct ixp_clock, caps);
    struct ixp46x_ts_regs *regs = ixp_clock.regs;
    spin_lock_irqsave(&register_lock, flags);
    ns = ixp_systime_read(regs);
    spin_unlock_irqrestore(&register_lock, flags);
// ts = ns_to_timespec64(ns);
    return 0;
    }
    static int ptp_ixp_settime(struct ptp_clock_info *ptp,
    const struct timespec64 *ts)
    {
    u64 ns;
    unsigned long flags;
    struct ixp_clock *ixp_clock = container_of(ptp, struct ixp_clock, caps);
    struct ixp46x_ts_regs *regs = ixp_clock.regs;
    ns = timespec64_to_ns(ts);
    spin_lock_irqsave(&register_lock, flags);
    ixp_systime_write(regs, ns);
    spin_unlock_irqrestore(&register_lock, flags);
    return 0;
    }
    static int ptp_ixp_enable(struct ptp_clock_info *ptp,
    struct ptp_clock_request *rq, int on)
    {
    struct ixp_clock *ixp_clock = container_of(ptp, struct ixp_clock, caps);
    switch (rq.type) {
    case PTP_CLK_REQ_EXTTS:
    switch (rq.extts.index) {
    case 0:
    ixp_clock.exts0_enabled = on ? 1 : 0;
    break;
    case 1:
    ixp_clock.exts1_enabled = on ? 1 : 0;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    default:
    break;
    }
    return -EOPNOTSUPP;
    }
    static const struct ptp_clock_info ptp_ixp_caps = {
    .owner		= THIS_MODULE,
    .name		= "IXP46X timer",
    .max_adj	= 66666655,
    .n_ext_ts	= N_EXT_TS,
    .n_pins		= 0,
    .pps		= 0,
    .adjfine	= ptp_ixp_adjfine,
    .adjtime	= ptp_ixp_adjtime,
    .gettime64	= ptp_ixp_gettime,
    .settime64	= ptp_ixp_settime,
    .enable		= ptp_ixp_enable,
    };
// module operations
    static struct ixp_clock ixp_clock;
#[no_mangle]
pub unsafe extern "C" fn ixp46x_ptp_find(regs: *mut *mut ixp46x_ts_regs __iomem, phc_index: *mut c_int) -> c_int {
    int ixp46x_ptp_find(struct ixp46x_ts_regs *__iomem *regs, int *phc_index)
    {
    if (!cpu_is_ixp46x())
    return -ENODEV;
// regs = ixp_clock.regs;
// phc_index = ptp_clock_index(ixp_clock.ptp_clock);
    if (!ixp_clock.ptp_clock)
    return -EPROBE_DEFER;
    return 0;
    }
    EXPORT_SYMBOL_GPL(ixp46x_ptp_find);
// Called from the registered devm action
#[no_mangle]
unsafe extern "C" fn ptp_ixp_unregister_action(d: *mut c_void) {
    static void ptp_ixp_unregister_action(void *d)
    {
    struct ptp_clock *ptp_clock = d;
    ptp_clock_unregister(ptp_clock);
    ixp_clock.ptp_clock = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn ptp_ixp_probe(pdev: *mut platform_device) -> c_int {
    static int ptp_ixp_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    int ret;
    ixp_clock.regs = devm_platform_ioremap_resource(pdev, 0);
    ixp_clock.master_irq = platform_get_irq(pdev, 0);
    ixp_clock.slave_irq = platform_get_irq(pdev, 1);
    if (IS_ERR(ixp_clock.regs) ||
    ixp_clock.master_irq < 0 || ixp_clock.slave_irq < 0)
    return -ENXIO;
    ixp_clock.caps = ptp_ixp_caps;
    ixp_clock.ptp_clock = ptp_clock_register(&ixp_clock.caps, core::ptr::null_mut());
    if (IS_ERR(ixp_clock.ptp_clock))
    return PTR_ERR(ixp_clock.ptp_clock);
    ret = devm_add_action_or_reset(dev, ptp_ixp_unregister_action,
    ixp_clock.ptp_clock);
    if (ret) {
    dev_err(dev, "failed to install clock removal handler\n");
    return ret;
    }
    __raw_writel(DEFAULT_ADDEND, &ixp_clock.regs.addend);
    __raw_writel(1, &ixp_clock.regs.trgt_lo);
    __raw_writel(0, &ixp_clock.regs.trgt_hi);
    __raw_writel(TTIPEND, &ixp_clock.regs.event);
    ret = devm_request_irq(dev, ixp_clock.master_irq, isr,
    0, DRIVER, &ixp_clock);
    if (ret)
    return dev_err_probe(dev, ret,
    "request_irq failed for irq %d\n",
    ixp_clock.master_irq);
    ret = devm_request_irq(dev, ixp_clock.slave_irq, isr,
    0, DRIVER, &ixp_clock);
    if (ret)
    return dev_err_probe(dev, ret,
    "request_irq failed for irq %d\n",
    ixp_clock.slave_irq);
    return 0;
    }
    static const struct of_device_id ptp_ixp_match[] = {
    {
    .compatible = "intel,ixp46x-ptp-timer",
    },
    { },
    };
    static struct platform_driver ptp_ixp_driver = {
    .driver = {
    .name = "ptp-ixp46x",
    .of_match_table = ptp_ixp_match,
    .suppress_bind_attrs = true,
    },
    .probe = ptp_ixp_probe,
    };
    module_platform_driver(ptp_ixp_driver);
    MODULE_AUTHOR("Richard Cochran <richardcochran@gmail.com>");
    MODULE_DESCRIPTION("PTP clock using the IXP46X timer");
    MODULE_LICENSE("GPL");
