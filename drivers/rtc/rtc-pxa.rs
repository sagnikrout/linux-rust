//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-pxa.c
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
// Real Time Clock interface for XScale PXA27x and PXA3xx
//
// Copyright (C) 2008 Robert Jarzmik
//

pub const RTC_DEF_TRIM: c_int = 0;
pub const MAXFREQ_PERIODIC: c_int = 1000;
//
// PXA Registers and bits definitions
//

    | RTSR_SWAL1 | RTSR_SWAL2)
pub const RYxR_YEAR_S: c_int = 9;

pub const RYxR_MONTH_S: c_int = 5;

pub const RYxR_DAY_MASK: c_uint = 0x1f;
pub const RDxR_WOM_S: c_int = 20;

pub const RDxR_DOW_S: c_int = 17;

pub const RDxR_HOUR_S: c_int = 12;

pub const RDxR_MIN_S: c_int = 6;

pub const RDxR_SEC_MASK: c_uint = 0x3f;
pub const RTSR: c_uint = 0x08;
pub const RTTR: c_uint = 0x0c;
pub const RDCR: c_uint = 0x10;
pub const RYCR: c_uint = 0x14;
pub const RDAR1: c_uint = 0x18;
pub const RYAR1: c_uint = 0x1c;
pub const RTCPICR: c_uint = 0x34;
pub const PIAR: c_uint = 0x38;

    __raw_readl((pxa_rtc).base + (reg))

    __raw_writel((value), (pxa_rtc).base + (reg))
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxa_rtc {
    pub sa1100_rtc: sa1100_rtc,
    pub ress: *mut resource,
    pub base: *mut void __iomem,
    pub rtc: *mut rtc_device,
    pub /: *mut *mut spinlock_t lock; / Protects this structure,
}

#[no_mangle]
unsafe extern "C" fn ryxr_calc(tm: *mut rtc_time) -> u32 {
    static u32 ryxr_calc(struct rtc_time *tm)
    {
    return ((tm.tm_year + 1900) << RYxR_YEAR_S)
    | ((tm.tm_mon + 1) << RYxR_MONTH_S)
    | tm.tm_mday;
    }
#[no_mangle]
unsafe extern "C" fn rdxr_calc(tm: *mut rtc_time) -> u32 {
    static u32 rdxr_calc(struct rtc_time *tm)
    {
    return ((((tm.tm_mday + 6) / 7) << RDxR_WOM_S) & RDxR_WOM_MASK)
    | (((tm.tm_wday + 1) << RDxR_DOW_S) & RDxR_DOW_MASK)
    | (tm.tm_hour << RDxR_HOUR_S)
    | (tm.tm_min << RDxR_MIN_S)
    | tm.tm_sec;
    }
#[no_mangle]
unsafe extern "C" fn tm_calc(rycr: u32, rdcr: u32, tm: *mut rtc_time) {
    static void tm_calc(u32 rycr, u32 rdcr, struct rtc_time *tm)
    {
    tm.tm_year = ((rycr & RYxR_YEAR_MASK) >> RYxR_YEAR_S) - 1900;
    tm.tm_mon = (((rycr & RYxR_MONTH_MASK) >> RYxR_MONTH_S)) - 1;
    tm.tm_mday = (rycr & RYxR_DAY_MASK);
    tm.tm_wday = ((rycr & RDxR_DOW_MASK) >> RDxR_DOW_S) - 1;
    tm.tm_hour = (rdcr & RDxR_HOUR_MASK) >> RDxR_HOUR_S;
    tm.tm_min = (rdcr & RDxR_MIN_MASK) >> RDxR_MIN_S;
    tm.tm_sec = rdcr & RDxR_SEC_MASK;
    }
#[no_mangle]
unsafe extern "C" fn rtsr_clear_bits(pxa_rtc: *mut pxa_rtc, mask: u32) {
    static void rtsr_clear_bits(struct pxa_rtc *pxa_rtc, u32 mask)
    {
    u32 rtsr;
    rtsr = rtc_readl(pxa_rtc, RTSR);
    rtsr &= ~RTSR_TRIG_MASK;
    rtsr &= ~mask;
    rtc_writel(pxa_rtc, RTSR, rtsr);
    }
#[no_mangle]
unsafe extern "C" fn rtsr_set_bits(pxa_rtc: *mut pxa_rtc, mask: u32) {
    static void rtsr_set_bits(struct pxa_rtc *pxa_rtc, u32 mask)
    {
    u32 rtsr;
    rtsr = rtc_readl(pxa_rtc, RTSR);
    rtsr &= ~RTSR_TRIG_MASK;
    rtsr |= mask;
    rtc_writel(pxa_rtc, RTSR, rtsr);
    }
#[no_mangle]
unsafe extern "C" fn pxa_rtc_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t pxa_rtc_irq(int irq, void *dev_id)
    {
    struct pxa_rtc *pxa_rtc = dev_get_drvdata(dev_id);
    u32 rtsr;
    let mut events: c_ulong = 0;
    spin_lock(&pxa_rtc.lock);
// clear interrupt sources
    rtsr = rtc_readl(pxa_rtc, RTSR);
    rtc_writel(pxa_rtc, RTSR, rtsr);
// temporary disable rtc interrupts
    rtsr_clear_bits(pxa_rtc, RTSR_RDALE1 | RTSR_PIALE | RTSR_HZE);
// clear alarm interrupt if it has occurred
    if (rtsr & RTSR_RDAL1)
    rtsr &= ~RTSR_RDALE1;
// update irq data & counter
    if (rtsr & RTSR_RDAL1)
    events |= RTC_AF | RTC_IRQF;
    if (rtsr & RTSR_HZ)
    events |= RTC_UF | RTC_IRQF;
    if (rtsr & RTSR_PIAL)
    events |= RTC_PF | RTC_IRQF;
    rtc_update_irq(pxa_rtc.rtc, 1, events);
// enable back rtc interrupts
    rtc_writel(pxa_rtc, RTSR, rtsr & ~RTSR_TRIG_MASK);
    spin_unlock(&pxa_rtc.lock);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn pxa_rtc_open(dev: *mut device) -> c_int {
    static int pxa_rtc_open(struct device *dev)
    {
    struct pxa_rtc *pxa_rtc = dev_get_drvdata(dev);
    int ret;
    ret = request_irq(pxa_rtc.sa1100_rtc.irq_1hz, pxa_rtc_irq, 0,
    "rtc 1Hz", dev);
    if (ret < 0) {
    dev_err(dev, "can't get irq %i, err %d\n",
    pxa_rtc.sa1100_rtc.irq_1hz, ret);
    goto err_irq_1Hz;
    }
    ret = request_irq(pxa_rtc.sa1100_rtc.irq_alarm, pxa_rtc_irq, 0,
    "rtc Alrm", dev);
    if (ret < 0) {
    dev_err(dev, "can't get irq %i, err %d\n",
    pxa_rtc.sa1100_rtc.irq_alarm, ret);
    goto err_irq_Alrm;
    }
    return 0;
    err_irq_Alrm:
    free_irq(pxa_rtc.sa1100_rtc.irq_1hz, dev);
    err_irq_1Hz:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pxa_rtc_release(dev: *mut device) {
    static void pxa_rtc_release(struct device *dev)
    {
    struct pxa_rtc *pxa_rtc = dev_get_drvdata(dev);
    spin_lock_irq(&pxa_rtc.lock);
    rtsr_clear_bits(pxa_rtc, RTSR_PIALE | RTSR_RDALE1 | RTSR_HZE);
    spin_unlock_irq(&pxa_rtc.lock);
    free_irq(pxa_rtc.sa1100_rtc.irq_1hz, dev);
    free_irq(pxa_rtc.sa1100_rtc.irq_alarm, dev);
    }
#[no_mangle]
unsafe extern "C" fn pxa_alarm_irq_enable(dev: *mut device, enabled: c_uint) -> c_int {
    static int pxa_alarm_irq_enable(struct device *dev, unsigned int enabled)
    {
    struct pxa_rtc *pxa_rtc = dev_get_drvdata(dev);
    spin_lock_irq(&pxa_rtc.lock);
    if (enabled)
    rtsr_set_bits(pxa_rtc, RTSR_RDALE1);
    else
    rtsr_clear_bits(pxa_rtc, RTSR_RDALE1);
    spin_unlock_irq(&pxa_rtc.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pxa_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int pxa_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct pxa_rtc *pxa_rtc = dev_get_drvdata(dev);
    u32 rycr, rdcr;
    rycr = rtc_readl(pxa_rtc, RYCR);
    rdcr = rtc_readl(pxa_rtc, RDCR);
    tm_calc(rycr, rdcr, tm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pxa_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int pxa_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct pxa_rtc *pxa_rtc = dev_get_drvdata(dev);
    rtc_writel(pxa_rtc, RYCR, ryxr_calc(tm));
    rtc_writel(pxa_rtc, RDCR, rdxr_calc(tm));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pxa_rtc_read_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int pxa_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct pxa_rtc *pxa_rtc = dev_get_drvdata(dev);
    u32 rtsr, ryar, rdar;
    ryar = rtc_readl(pxa_rtc, RYAR1);
    rdar = rtc_readl(pxa_rtc, RDAR1);
    tm_calc(ryar, rdar, &alrm.time);
    rtsr = rtc_readl(pxa_rtc, RTSR);
    alrm.enabled = (rtsr & RTSR_RDALE1) ? 1 : 0;
    alrm.pending = (rtsr & RTSR_RDAL1) ? 1 : 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pxa_rtc_set_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int pxa_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct pxa_rtc *pxa_rtc = dev_get_drvdata(dev);
    u32 rtsr;
    spin_lock_irq(&pxa_rtc.lock);
    rtc_writel(pxa_rtc, RYAR1, ryxr_calc(&alrm.time));
    rtc_writel(pxa_rtc, RDAR1, rdxr_calc(&alrm.time));
    rtsr = rtc_readl(pxa_rtc, RTSR);
    if (alrm.enabled)
    rtsr |= RTSR_RDALE1;
    else
    rtsr &= ~RTSR_RDALE1;
    rtc_writel(pxa_rtc, RTSR, rtsr);
    spin_unlock_irq(&pxa_rtc.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pxa_rtc_proc(dev: *mut device, seq: *mut seq_file) -> c_int {
    static int pxa_rtc_proc(struct device *dev, struct seq_file *seq)
    {
    struct pxa_rtc *pxa_rtc = dev_get_drvdata(dev);
    seq_printf(seq, "trim/divider\t: 0x%08x\n", rtc_readl(pxa_rtc, RTTR));
    seq_printf(seq, "update_IRQ\t: %s\n",
    (rtc_readl(pxa_rtc, RTSR) & RTSR_HZE) ? "yes" : "no");
    seq_printf(seq, "periodic_IRQ\t: %s\n",
    (rtc_readl(pxa_rtc, RTSR) & RTSR_PIALE) ? "yes" : "no");
    seq_printf(seq, "periodic_freq\t: %u\n", rtc_readl(pxa_rtc, PIAR));
    return 0;
    }
    static const struct rtc_class_ops pxa_rtc_ops = {
    .read_time = pxa_rtc_read_time,
    .set_time = pxa_rtc_set_time,
    .read_alarm = pxa_rtc_read_alarm,
    .set_alarm = pxa_rtc_set_alarm,
    .alarm_irq_enable = pxa_alarm_irq_enable,
    .proc = pxa_rtc_proc,
    };
#[no_mangle]
unsafe extern "C" fn pxa_rtc_probe(pdev: *mut platform_device) -> int __init {
    static int __init pxa_rtc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct pxa_rtc *pxa_rtc;
    struct sa1100_rtc *sa1100_rtc;
    int ret;
    pxa_rtc = devm_kzalloc(dev, sizeof(*pxa_rtc), GFP_KERNEL);
    if (!pxa_rtc)
    return -ENOMEM;
    sa1100_rtc = &pxa_rtc.sa1100_rtc;
    spin_lock_init(&pxa_rtc.lock);
    platform_set_drvdata(pdev, pxa_rtc);
    pxa_rtc.ress = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!pxa_rtc.ress) {
    dev_err(dev, "No I/O memory resource defined\n");
    return -ENXIO;
    }
    sa1100_rtc.irq_1hz = platform_get_irq(pdev, 0);
    if (sa1100_rtc.irq_1hz < 0)
    return -ENXIO;
    sa1100_rtc.irq_alarm = platform_get_irq(pdev, 1);
    if (sa1100_rtc.irq_alarm < 0)
    return -ENXIO;
    sa1100_rtc.rtc = devm_rtc_allocate_device(&pdev.dev);
    if (IS_ERR(sa1100_rtc.rtc))
    return PTR_ERR(sa1100_rtc.rtc);
    pxa_rtc.base = devm_ioremap(dev, pxa_rtc.ress.start,
    resource_size(pxa_rtc.ress));
    if (!pxa_rtc.base) {
    dev_err(dev, "Unable to map pxa RTC I/O memory\n");
    return -ENOMEM;
    }
    pxa_rtc_open(dev);
    sa1100_rtc.rcnr = pxa_rtc.base + 0x0;
    sa1100_rtc.rtsr = pxa_rtc.base + 0x8;
    sa1100_rtc.rtar = pxa_rtc.base + 0x4;
    sa1100_rtc.rttr = pxa_rtc.base + 0xc;
    ret = sa1100_rtc_init(pdev, sa1100_rtc);
    if (ret) {
    dev_err(dev, "Unable to init SA1100 RTC sub-device\n");
    return ret;
    }
    rtsr_clear_bits(pxa_rtc, RTSR_PIALE | RTSR_RDALE1 | RTSR_HZE);
    pxa_rtc.rtc = devm_rtc_device_register(&pdev.dev, "pxa-rtc",
    &pxa_rtc_ops, THIS_MODULE);
    if (IS_ERR(pxa_rtc.rtc)) {
    ret = PTR_ERR(pxa_rtc.rtc);
    dev_err(dev, "Failed to register RTC device . %d\n", ret);
    return ret;
    }
    device_init_wakeup(dev, true);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pxa_rtc_remove(pdev: *mut platform_device) -> void __exit {
    static void __exit pxa_rtc_remove(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    pxa_rtc_release(dev);
    }

    static const struct of_device_id pxa_rtc_dt_ids[] = {
    { .compatible = "marvell,pxa-rtc" },
    {}
    };
    MODULE_DEVICE_TABLE(of, pxa_rtc_dt_ids);

#[no_mangle]
unsafe extern "C" fn pxa_rtc_suspend(dev: *mut device) -> c_int {
    static int pxa_rtc_suspend(struct device *dev)
    {
    struct pxa_rtc *pxa_rtc = dev_get_drvdata(dev);
    if (device_may_wakeup(dev))
    enable_irq_wake(pxa_rtc.sa1100_rtc.irq_alarm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pxa_rtc_resume(dev: *mut device) -> c_int {
    static int pxa_rtc_resume(struct device *dev)
    {
    struct pxa_rtc *pxa_rtc = dev_get_drvdata(dev);
    if (device_may_wakeup(dev))
    disable_irq_wake(pxa_rtc.sa1100_rtc.irq_alarm);
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(pxa_rtc_pm_ops, pxa_rtc_suspend, pxa_rtc_resume);
//
// pxa_rtc_remove() lives in .exit.text. For drivers registered via
// module_platform_driver_probe() this is ok because they cannot get unbound at
// runtime. So mark the driver struct with __refdata to prevent modpost
// triggering a section mismatch warning.
//
    static struct platform_driver pxa_rtc_driver __refdata = {
    .remove		= __exit_p(pxa_rtc_remove),
    .driver		= {
    .name	= "pxa-rtc",
    .of_match_table = of_match_ptr(pxa_rtc_dt_ids),
    .pm	= &pxa_rtc_pm_ops,
    },
    };
    module_platform_driver_probe(pxa_rtc_driver, pxa_rtc_probe);
    MODULE_AUTHOR("Robert Jarzmik <robert.jarzmik@free.fr>");
    MODULE_DESCRIPTION("PXA27x/PXA3xx Realtime Clock Driver (RTC)");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:pxa-rtc");
