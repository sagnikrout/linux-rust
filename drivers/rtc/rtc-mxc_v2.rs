//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-mxc_v2.c
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
// Real Time Clock (RTC) Driver for i.MX53
// Copyright (c) 2004-2011 Freescale Semiconductor, Inc.
// Copyright (c) 2017 Beckhoff Automation GmbH & Co. KG
//

pub const SRTC_LPPDR_INIT: c_uint = 0x41736166	/* init for glitch detect */;

pub const SRTC_LPSCMR: c_uint = 0x00	/* LP Secure Counter MSB Reg */;
pub const SRTC_LPSCLR: c_uint = 0x04	/* LP Secure Counter LSB Reg */;
pub const SRTC_LPSAR: c_uint = 0x08	/* LP Secure Alarm Reg */;
pub const SRTC_LPCR: c_uint = 0x10	/* LP Control Reg */;
pub const SRTC_LPSR: c_uint = 0x14	/* LP Status Reg */;
pub const SRTC_LPPDR: c_uint = 0x18	/* LP Power Supply Glitch Detector Reg */;
// max. number of retries to read registers, 120 was max during test
pub const REG_READ_TIMEOUT: c_int = 2000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxc_rtc_data {
    pub rtc: *mut rtc_device,
    pub ioaddr: *mut void __iomem,
    pub clk: *mut clk,
    pub /: *mut *mut spinlock_t lock; / protects register access,
    pub irq: c_int,
}

//
// This function does write synchronization for writes to the lp srtc block.
// To take care of the asynchronous CKIL clock, all writes from the IP domain
// will be synchronized to the CKIL domain.
// The caller should hold the pdata->lock
//
#[no_mangle]
unsafe extern "C" fn mxc_rtc_sync_lp_locked(dev: *mut device, ioaddr: *mut void __iomem) {
    static void mxc_rtc_sync_lp_locked(struct device *dev, void __iomem *ioaddr)
    {
    unsigned int i;
// Wait for 3 CKIL cycles
    for (i = 0; i < 3; i++) {
    let mut count: u32 = readl(ioaddr + SRTC_LPSCLR);
    let mut timeout: c_uint = REG_READ_TIMEOUT;
    while ((readl(ioaddr + SRTC_LPSCLR)) == count) {
    if (!--timeout) {
    dev_err_once(dev, "SRTC_LPSCLR stuck! Check your hw.\n");
    return;
    }
    }
    }
    }
// This function is the RTC interrupt service routine.
#[no_mangle]
unsafe extern "C" fn mxc_rtc_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t mxc_rtc_interrupt(int irq, void *dev_id)
    {
    struct device *dev = dev_id;
    struct mxc_rtc_data *pdata = dev_get_drvdata(dev);
    void __iomem *ioaddr = pdata.ioaddr;
    u32 lp_status;
    u32 lp_cr;
    spin_lock(&pdata.lock);
    if (clk_enable(pdata.clk)) {
    spin_unlock(&pdata.lock);
    return IRQ_NONE;
    }
    lp_status = readl(ioaddr + SRTC_LPSR);
    lp_cr = readl(ioaddr + SRTC_LPCR);
// update irq data & counter
    if (lp_status & SRTC_LPSR_ALP) {
    if (lp_cr & SRTC_LPCR_ALP)
    rtc_update_irq(pdata.rtc, 1, RTC_AF | RTC_IRQF);
// disable further lp alarm interrupts
    lp_cr &= ~(SRTC_LPCR_ALP | SRTC_LPCR_WAE);
    }
// Update interrupt enables
    writel(lp_cr, ioaddr + SRTC_LPCR);
// clear interrupt status
    writel(lp_status, ioaddr + SRTC_LPSR);
    mxc_rtc_sync_lp_locked(dev, ioaddr);
    clk_disable(pdata.clk);
    spin_unlock(&pdata.lock);
    return IRQ_HANDLED;
    }
//
// Enable clk and aquire spinlock
// @return  0 if successful; non-zero otherwise.
//
#[no_mangle]
unsafe extern "C" fn mxc_rtc_lock(pdata: *const *const mxc_rtc_data) -> c_int {
    static int mxc_rtc_lock(struct mxc_rtc_data *const pdata)
    {
    int ret;
    spin_lock_irq(&pdata.lock);
    ret = clk_enable(pdata.clk);
    if (ret) {
    spin_unlock_irq(&pdata.lock);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mxc_rtc_unlock(pdata: *const *const mxc_rtc_data) -> c_int {
    static int mxc_rtc_unlock(struct mxc_rtc_data *const pdata)
    {
    clk_disable(pdata.clk);
    spin_unlock_irq(&pdata.lock);
    return 0;
    }
//
// This function reads the current RTC time into tm in Gregorian date.
//
// @param  tm           contains the RTC time value upon return
//
// @return  0 if successful; non-zero otherwise.
//
#[no_mangle]
unsafe extern "C" fn mxc_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int mxc_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct mxc_rtc_data *pdata = dev_get_drvdata(dev);
    let mut clk_failed: c_int = clk_enable(pdata.clk);
    if (!clk_failed) {
    let mut now: time64_t = readl(pdata.ioaddr + SRTC_LPSCMR);
    rtc_time64_to_tm(now, tm);
    clk_disable(pdata.clk);
    return 0;
    }
    return clk_failed;
    }
//
// This function sets the internal RTC time based on tm in Gregorian date.
//
// @param  tm           the time value to be set in the RTC
//
// @return  0 if successful; non-zero otherwise.
//
#[no_mangle]
unsafe extern "C" fn mxc_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int mxc_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct mxc_rtc_data *pdata = dev_get_drvdata(dev);
    let mut time: time64_t = rtc_tm_to_time64(tm);
    int ret;
    ret = mxc_rtc_lock(pdata);
    if (ret)
    return ret;
    writel(time, pdata.ioaddr + SRTC_LPSCMR);
    mxc_rtc_sync_lp_locked(dev, pdata.ioaddr);
    return mxc_rtc_unlock(pdata);
    }
//
// This function reads the current alarm value into the passed in \b alrm
// argument. It updates the \b alrm's pending field value based on the whether
// an alarm interrupt occurs or not.
//
// @param  alrm         contains the RTC alarm value upon return
//
// @return  0 if successful; non-zero otherwise.
//
#[no_mangle]
unsafe extern "C" fn mxc_rtc_read_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int mxc_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct mxc_rtc_data *pdata = dev_get_drvdata(dev);
    void __iomem *ioaddr = pdata.ioaddr;
    int ret;
    ret = mxc_rtc_lock(pdata);
    if (ret)
    return ret;
    rtc_time64_to_tm(readl(ioaddr + SRTC_LPSAR), &alrm.time);
    alrm.pending = !!(readl(ioaddr + SRTC_LPSR) & SRTC_LPSR_ALP);
    return mxc_rtc_unlock(pdata);
    }
//
// Enable/Disable alarm interrupt
// The caller should hold the pdata->lock
//
    static void mxc_rtc_alarm_irq_enable_locked(struct mxc_rtc_data *pdata,
    unsigned int enable)
    {
    let mut lp_cr: u32 = readl(pdata.ioaddr + SRTC_LPCR);
    if (enable)
    lp_cr |= (SRTC_LPCR_ALP | SRTC_LPCR_WAE);
    else
    lp_cr &= ~(SRTC_LPCR_ALP | SRTC_LPCR_WAE);
    writel(lp_cr, pdata.ioaddr + SRTC_LPCR);
    }
#[no_mangle]
unsafe extern "C" fn mxc_rtc_alarm_irq_enable(dev: *mut device, enable: c_uint) -> c_int {
    static int mxc_rtc_alarm_irq_enable(struct device *dev, unsigned int enable)
    {
    struct mxc_rtc_data *pdata = dev_get_drvdata(dev);
    let mut ret: c_int = mxc_rtc_lock(pdata);
    if (ret)
    return ret;
    mxc_rtc_alarm_irq_enable_locked(pdata, enable);
    return mxc_rtc_unlock(pdata);
    }
//
// This function sets the RTC alarm based on passed in alrm.
//
// @param  alrm         the alarm value to be set in the RTC
//
// @return  0 if successful; non-zero otherwise.
//
#[no_mangle]
unsafe extern "C" fn mxc_rtc_set_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int mxc_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    let mut time: time64_t = rtc_tm_to_time64(&alrm.time);
    struct mxc_rtc_data *pdata = dev_get_drvdata(dev);
    let mut ret: c_int = mxc_rtc_lock(pdata);
    if (ret)
    return ret;
    writel((u32)time, pdata.ioaddr + SRTC_LPSAR);
// clear alarm interrupt status bit
    writel(SRTC_LPSR_ALP, pdata.ioaddr + SRTC_LPSR);
    mxc_rtc_sync_lp_locked(dev, pdata.ioaddr);
    mxc_rtc_alarm_irq_enable_locked(pdata, alrm.enabled);
    mxc_rtc_sync_lp_locked(dev, pdata.ioaddr);
    mxc_rtc_unlock(pdata);
    return ret;
    }
    static const struct rtc_class_ops mxc_rtc_ops = {
    .read_time = mxc_rtc_read_time,
    .set_time = mxc_rtc_set_time,
    .read_alarm = mxc_rtc_read_alarm,
    .set_alarm = mxc_rtc_set_alarm,
    .alarm_irq_enable = mxc_rtc_alarm_irq_enable,
    };
#[no_mangle]
unsafe extern "C" fn mxc_rtc_wait_for_flag(ioaddr: *mut void __iomem, flag: c_int) -> c_int {
    static int mxc_rtc_wait_for_flag(void __iomem *ioaddr, int flag)
    {
    let mut timeout: c_uint = REG_READ_TIMEOUT;
    while (!(readl(ioaddr) & flag)) {
    if (!--timeout)
    return -EBUSY;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mxc_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int mxc_rtc_probe(struct platform_device *pdev)
    {
    struct mxc_rtc_data *pdata;
    void __iomem *ioaddr;
    let mut ret: c_int = 0;
    pdata = devm_kzalloc(&pdev.dev, sizeof(*pdata), GFP_KERNEL);
    if (!pdata)
    return -ENOMEM;
    pdata.ioaddr = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(pdata.ioaddr))
    return PTR_ERR(pdata.ioaddr);
    ioaddr = pdata.ioaddr;
    pdata.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(pdata.clk)) {
    dev_err(&pdev.dev, "unable to get rtc clock!\n");
    return PTR_ERR(pdata.clk);
    }
    spin_lock_init(&pdata.lock);
    pdata.irq = platform_get_irq(pdev, 0);
    if (pdata.irq < 0)
    return pdata.irq;
    device_init_wakeup(&pdev.dev, true);
    ret = dev_pm_set_wake_irq(&pdev.dev, pdata.irq);
    if (ret)
    dev_err(&pdev.dev, "failed to enable irq wake\n");
    ret = clk_prepare_enable(pdata.clk);
    if (ret)
    return ret;
// initialize glitch detect
    writel(SRTC_LPPDR_INIT, ioaddr + SRTC_LPPDR);
// clear lp interrupt status
    writel(0xFFFFFFFF, ioaddr + SRTC_LPSR);
// move out of init state
    writel((SRTC_LPCR_IE | SRTC_LPCR_NSA), ioaddr + SRTC_LPCR);
    ret = mxc_rtc_wait_for_flag(ioaddr + SRTC_LPSR, SRTC_LPSR_IES);
    if (ret) {
    dev_err(&pdev.dev, "Timeout waiting for SRTC_LPSR_IES\n");
    clk_disable_unprepare(pdata.clk);
    return ret;
    }
// move out of non-valid state
    writel((SRTC_LPCR_IE | SRTC_LPCR_NVE | SRTC_LPCR_NSA |
    SRTC_LPCR_EN_LP), ioaddr + SRTC_LPCR);
    ret = mxc_rtc_wait_for_flag(ioaddr + SRTC_LPSR, SRTC_LPSR_NVES);
    if (ret) {
    dev_err(&pdev.dev, "Timeout waiting for SRTC_LPSR_NVES\n");
    clk_disable_unprepare(pdata.clk);
    return ret;
    }
    pdata.rtc = devm_rtc_allocate_device(&pdev.dev);
    if (IS_ERR(pdata.rtc)) {
    clk_disable_unprepare(pdata.clk);
    return PTR_ERR(pdata.rtc);
    }
    pdata.rtc.ops = &mxc_rtc_ops;
    pdata.rtc.range_max = U32_MAX;
    clk_disable(pdata.clk);
    platform_set_drvdata(pdev, pdata);
    ret =
    devm_request_irq(&pdev.dev, pdata.irq, mxc_rtc_interrupt, 0,
    pdev.name, &pdev.dev);
    if (ret < 0) {
    dev_err(&pdev.dev, "interrupt not available.\n");
    clk_unprepare(pdata.clk);
    return ret;
    }
    ret = devm_rtc_register_device(pdata.rtc);
    if (ret < 0)
    clk_unprepare(pdata.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mxc_rtc_remove(pdev: *mut platform_device) {
    static void mxc_rtc_remove(struct platform_device *pdev)
    {
    struct mxc_rtc_data *pdata = platform_get_drvdata(pdev);
    clk_disable_unprepare(pdata.clk);
    }
    static const struct of_device_id mxc_ids[] = {
    { .compatible = "fsl,imx53-rtc", },
    {}
    };
    MODULE_DEVICE_TABLE(of, mxc_ids);
    static struct platform_driver mxc_rtc_driver = {
    .driver = {
    .name = "mxc_rtc_v2",
    .of_match_table = mxc_ids,
    },
    .probe = mxc_rtc_probe,
    .remove = mxc_rtc_remove,
    };
    module_platform_driver(mxc_rtc_driver);
    MODULE_AUTHOR("Freescale Semiconductor, Inc.");
    MODULE_DESCRIPTION("Real Time Clock (RTC) Driver for i.MX53");
    MODULE_LICENSE("GPL");
