//! Automatically rewritten from C to Rust
//! Source: drivers/ptp/ptp_dte.c
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
// Copyright 2017 Broadcom

pub const DTE_NCO_LOW_TIME_REG: c_uint = 0x00;
pub const DTE_NCO_TIME_REG: c_uint = 0x04;
pub const DTE_NCO_OVERFLOW_REG: c_uint = 0x08;
pub const DTE_NCO_INC_REG: c_uint = 0x0c;
pub const DTE_NCO_SUM2_MASK: c_uint = 0xffffffff;

pub const DTE_NCO_SUM3_MASK: c_uint = 0xff;

pub const DTE_NCO_SUM3_WR_SHIFT: c_int = 8;
pub const DTE_NCO_TS_WRAP_MASK: c_uint = 0xfff;
pub const DTE_NCO_TS_WRAP_LSHIFT: c_int = 32;
pub const DTE_NCO_INC_DEFAULT: c_uint = 0x80000000;
pub const DTE_NUM_REGS_TO_RESTORE: c_int = 4;
// Full wrap around is 44bits in ns (~4.887 hrs)
pub const DTE_WRAP_AROUND_NSEC_SHIFT: c_int = 44;
// 44 bits NCO
pub const DTE_NCO_MAX_NS: c_uint = 0xFFFFFFFFFFFLL;
// 125MHz with 3.29 reg cfg

    62500000ULL), 125000000ULL))
// ptp dte priv structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptp_dte {
    pub regs: *mut void __iomem,
    pub ptp_clk: *mut ptp_clock,
    pub caps: ptp_clock_info,
    pub dev: *mut device,
    pub ts_ovf_last: u32,
    pub ts_wrap_cnt: u32,
    pub lock: spinlock_t,
    pub reg_val: [u32; DTE_NUM_REGS_TO_RESTORE],
}

#[no_mangle]
unsafe extern "C" fn dte_write_nco(regs: *mut void __iomem, ns: i64) {
    static void dte_write_nco(void __iomem *regs, s64 ns)
    {
    u32 sum2, sum3;
    sum2 = (u32)((ns >> DTE_NCO_SUM2_SHIFT) & DTE_NCO_SUM2_MASK);
// compensate for ignoring sum1
    if (sum2 != DTE_NCO_SUM2_MASK)
    sum2++;
// to write sum3, bits [15:8] needs to be written
    sum3 = (u32)(((ns >> DTE_NCO_SUM3_SHIFT) & DTE_NCO_SUM3_MASK) <<
    DTE_NCO_SUM3_WR_SHIFT);
    writel(0, (regs + DTE_NCO_LOW_TIME_REG));
    writel(sum2, (regs + DTE_NCO_TIME_REG));
    writel(sum3, (regs + DTE_NCO_OVERFLOW_REG));
    }
#[no_mangle]
unsafe extern "C" fn dte_read_nco(regs: *mut void __iomem) -> i64 {
    static s64 dte_read_nco(void __iomem *regs)
    {
    u32 sum2, sum3;
    s64 ns;
//
// ignoring sum1 (4 bits) gives a 16ns resolution, which
// works due to the async register read.
//
    sum3 = readl(regs + DTE_NCO_OVERFLOW_REG) & DTE_NCO_SUM3_MASK;
    sum2 = readl(regs + DTE_NCO_TIME_REG);
    ns = ((s64)sum3 << DTE_NCO_SUM3_SHIFT) |
    ((s64)sum2 << DTE_NCO_SUM2_SHIFT);
    return ns;
    }
#[no_mangle]
unsafe extern "C" fn dte_write_nco_delta(ptp_dte: *mut ptp_dte, delta: i64) {
    static void dte_write_nco_delta(struct ptp_dte *ptp_dte, s64 delta)
    {
    s64 ns;
    ns = dte_read_nco(ptp_dte.regs);
// handle wraparound conditions
    if ((delta < 0) && (abs(delta) > ns)) {
    if (ptp_dte.ts_wrap_cnt) {
    ns += DTE_NCO_MAX_NS + delta;
    ptp_dte.ts_wrap_cnt--;
    } else {
    ns = 0;
    }
    } else {
    ns += delta;
    if (ns > DTE_NCO_MAX_NS) {
    ptp_dte.ts_wrap_cnt++;
    ns -= DTE_NCO_MAX_NS;
    }
    }
    dte_write_nco(ptp_dte.regs, ns);
    ptp_dte.ts_ovf_last = (ns >> DTE_NCO_TS_WRAP_LSHIFT) &
    DTE_NCO_TS_WRAP_MASK;
    }
#[no_mangle]
unsafe extern "C" fn dte_read_nco_with_ovf(ptp_dte: *mut ptp_dte) -> i64 {
    static s64 dte_read_nco_with_ovf(struct ptp_dte *ptp_dte)
    {
    u32 ts_ovf;
    let mut ns: i64 = 0;
    ns = dte_read_nco(ptp_dte.regs);
// Timestamp overflow: 8 LSB bits of sum3, 4 MSB bits of sum2
    ts_ovf = (ns >> DTE_NCO_TS_WRAP_LSHIFT) & DTE_NCO_TS_WRAP_MASK;
// Check for wrap around
    if (ts_ovf < ptp_dte.ts_ovf_last)
    ptp_dte.ts_wrap_cnt++;
    ptp_dte.ts_ovf_last = ts_ovf;
// adjust for wraparounds
    ns += (s64)(BIT_ULL(DTE_WRAP_AROUND_NSEC_SHIFT) * ptp_dte.ts_wrap_cnt);
    return ns;
    }
#[no_mangle]
unsafe extern "C" fn ptp_dte_adjfine(ptp: *mut ptp_clock_info, scaled_ppm: c_long) -> c_int {
    static int ptp_dte_adjfine(struct ptp_clock_info *ptp, long scaled_ppm)
    {
    let mut ppb: i32 = scaled_ppm_to_ppb(scaled_ppm);
    u32 nco_incr;
    unsigned long flags;
    struct ptp_dte *ptp_dte = container_of(ptp, struct ptp_dte, caps);
    if (abs(ppb) > ptp_dte.caps.max_adj) {
    dev_err(ptp_dte.dev, "ppb adj too big\n");
    return -EINVAL;
    }
    if (ppb < 0)
    nco_incr = DTE_NCO_INC_DEFAULT - DTE_PPB_ADJ(ppb);
    else
    nco_incr = DTE_NCO_INC_DEFAULT + DTE_PPB_ADJ(ppb);
    spin_lock_irqsave(&ptp_dte.lock, flags);
    writel(nco_incr, ptp_dte.regs + DTE_NCO_INC_REG);
    spin_unlock_irqrestore(&ptp_dte.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ptp_dte_adjtime(ptp: *mut ptp_clock_info, delta: i64) -> c_int {
    static int ptp_dte_adjtime(struct ptp_clock_info *ptp, s64 delta)
    {
    unsigned long flags;
    struct ptp_dte *ptp_dte = container_of(ptp, struct ptp_dte, caps);
    spin_lock_irqsave(&ptp_dte.lock, flags);
    dte_write_nco_delta(ptp_dte, delta);
    spin_unlock_irqrestore(&ptp_dte.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ptp_dte_gettime(ptp: *mut ptp_clock_info, ts: *mut timespec64) -> c_int {
    static int ptp_dte_gettime(struct ptp_clock_info *ptp, struct timespec64 *ts)
    {
    unsigned long flags;
    struct ptp_dte *ptp_dte = container_of(ptp, struct ptp_dte, caps);
    spin_lock_irqsave(&ptp_dte.lock, flags);
// ts = ns_to_timespec64(dte_read_nco_with_ovf(ptp_dte));
    spin_unlock_irqrestore(&ptp_dte.lock, flags);
    return 0;
    }
    static int ptp_dte_settime(struct ptp_clock_info *ptp,
    const struct timespec64 *ts)
    {
    unsigned long flags;
    struct ptp_dte *ptp_dte = container_of(ptp, struct ptp_dte, caps);
    spin_lock_irqsave(&ptp_dte.lock, flags);
// Disable nco increment
    writel(0, ptp_dte.regs + DTE_NCO_INC_REG);
    dte_write_nco(ptp_dte.regs, timespec64_to_ns(ts));
// reset overflow and wrap counter
    ptp_dte.ts_ovf_last = 0;
    ptp_dte.ts_wrap_cnt = 0;
// Enable nco increment
    writel(DTE_NCO_INC_DEFAULT, ptp_dte.regs + DTE_NCO_INC_REG);
    spin_unlock_irqrestore(&ptp_dte.lock, flags);
    return 0;
    }
    static int ptp_dte_enable(struct ptp_clock_info *ptp,
    struct ptp_clock_request *rq, int on)
    {
    return -EOPNOTSUPP;
    }
    static const struct ptp_clock_info ptp_dte_caps = {
    .owner		= THIS_MODULE,
    .name		= "DTE PTP timer",
    .max_adj	= 50000000,
    .n_ext_ts	= 0,
    .n_pins		= 0,
    .pps		= 0,
    .adjfine	= ptp_dte_adjfine,
    .adjtime	= ptp_dte_adjtime,
    .gettime64	= ptp_dte_gettime,
    .settime64	= ptp_dte_settime,
    .enable		= ptp_dte_enable,
    };
#[no_mangle]
unsafe extern "C" fn ptp_dte_probe(pdev: *mut platform_device) -> c_int {
    static int ptp_dte_probe(struct platform_device *pdev)
    {
    struct ptp_dte *ptp_dte;
    struct device *dev = &pdev.dev;
    ptp_dte = devm_kzalloc(dev, sizeof(struct ptp_dte), GFP_KERNEL);
    if (!ptp_dte)
    return -ENOMEM;
    ptp_dte.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(ptp_dte.regs))
    return PTR_ERR(ptp_dte.regs);
    spin_lock_init(&ptp_dte.lock);
    ptp_dte.dev = dev;
    ptp_dte.caps = ptp_dte_caps;
    ptp_dte.ptp_clk = ptp_clock_register(&ptp_dte.caps, &pdev.dev);
    if (IS_ERR(ptp_dte.ptp_clk)) {
    dev_err(dev,
    "%s: Failed to register ptp clock\n", __func__);
    return PTR_ERR(ptp_dte.ptp_clk);
    }
    platform_set_drvdata(pdev, ptp_dte);
    dev_info(dev, "ptp clk probe done\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ptp_dte_remove(pdev: *mut platform_device) {
    static void ptp_dte_remove(struct platform_device *pdev)
    {
    struct ptp_dte *ptp_dte = platform_get_drvdata(pdev);
    u8 i;
    ptp_clock_unregister(ptp_dte.ptp_clk);
    for (i = 0; i < DTE_NUM_REGS_TO_RESTORE; i++)
    writel(0, ptp_dte.regs + (i * sizeof(u32)));
    }

#[no_mangle]
unsafe extern "C" fn ptp_dte_suspend(dev: *mut device) -> c_int {
    static int ptp_dte_suspend(struct device *dev)
    {
    struct ptp_dte *ptp_dte = dev_get_drvdata(dev);
    u8 i;
    for (i = 0; i < DTE_NUM_REGS_TO_RESTORE; i++) {
    ptp_dte.reg_val[i] =
    readl(ptp_dte.regs + (i * sizeof(u32)));
    }
// disable the nco
    writel(0, ptp_dte.regs + DTE_NCO_INC_REG);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ptp_dte_resume(dev: *mut device) -> c_int {
    static int ptp_dte_resume(struct device *dev)
    {
    struct ptp_dte *ptp_dte = dev_get_drvdata(dev);
    u8 i;
    for (i = 0; i < DTE_NUM_REGS_TO_RESTORE; i++) {
    if ((i * sizeof(u32)) != DTE_NCO_OVERFLOW_REG)
    writel(ptp_dte.reg_val[i],
    (ptp_dte.regs + (i * sizeof(u32))));
    else
    writel(((ptp_dte.reg_val[i] &
    DTE_NCO_SUM3_MASK) << DTE_NCO_SUM3_WR_SHIFT),
    (ptp_dte.regs + (i * sizeof(u32))));
    }
    return 0;
    }
    static const struct dev_pm_ops ptp_dte_pm_ops = {
    .suspend = ptp_dte_suspend,
    .resume = ptp_dte_resume
    };

    static const struct of_device_id ptp_dte_of_match[] = {
    { .compatible = "brcm,ptp-dte", },
    {},
    };
    MODULE_DEVICE_TABLE(of, ptp_dte_of_match);
    static struct platform_driver ptp_dte_driver = {
    .driver = {
    .name = "ptp-dte",
    .pm = PTP_DTE_PM_OPS,
    .of_match_table = ptp_dte_of_match,
    },
    .probe    = ptp_dte_probe,
    .remove   = ptp_dte_remove,
    };
    module_platform_driver(ptp_dte_driver);
    MODULE_AUTHOR("Broadcom");
    MODULE_DESCRIPTION("Broadcom DTE PTP Clock driver");
    MODULE_LICENSE("GPL v2");
