//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-snvs.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2011-2012 Freescale Semiconductor, Inc.

pub const SNVS_LPREGISTER_OFFSET: c_uint = 0x34;
// These register offsets are relative to LP (Low Power) range
pub const SNVS_LPCR: c_uint = 0x04;
pub const SNVS_LPSR: c_uint = 0x18;
pub const SNVS_LPSRTCMR: c_uint = 0x1c;
pub const SNVS_LPSRTCLR: c_uint = 0x20;
pub const SNVS_LPTAR: c_uint = 0x24;
pub const SNVS_LPPGDR: c_uint = 0x30;

pub const SNVS_LPPGDR_INIT: c_uint = 0x41736166;
pub const CNTR_TO_SECS_SH: c_int = 15;
// The maximum RTC clock cycles that are allowed to pass between two
// consecutive clock counter register reads. If the values are corrupted a
// bigger difference is expected. The RTC frequency is 32kHz. With 320 cycles
// we end at 10ms which should be enough for most cases. If it once takes
// longer than expected we do a retry.
//
pub const MAX_RTC_READ_DIFF_CYCLES: c_int = 320;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snvs_rtc_data {
    pub rtc: *mut rtc_device,
    pub regmap: *mut regmap,
    pub offset: c_int,
    pub irq: c_int,
    pub clk: *mut clk,
}

// Read 64 bit timer register, which could be in inconsistent state
#[no_mangle]
unsafe extern "C" fn rtc_read_lpsrt(data: *mut snvs_rtc_data) -> u64 {
    static u64 rtc_read_lpsrt(struct snvs_rtc_data *data)
    {
    u32 msb, lsb;
    regmap_read(data.regmap, data.offset + SNVS_LPSRTCMR, &msb);
    regmap_read(data.regmap, data.offset + SNVS_LPSRTCLR, &lsb);
    return (u64)msb << 32 | lsb;
    }
// Read the secure real time counter, taking care to deal with the cases of the
// counter updating while being read.
//
#[no_mangle]
unsafe extern "C" fn rtc_read_lp_counter(data: *mut snvs_rtc_data) -> u32 {
    static u32 rtc_read_lp_counter(struct snvs_rtc_data *data)
    {
    u64 read1, read2;
    s64 diff;
    let mut timeout: c_uint = 100;
// As expected, the registers might update between the read of the LSB
// reg and the MSB reg.  It's also possible that one register might be
// in partially modified state as well.
//
    read1 = rtc_read_lpsrt(data);
    do {
    read2 = read1;
    read1 = rtc_read_lpsrt(data);
    diff = read1 - read2;
    } while (((diff < 0) || (diff > MAX_RTC_READ_DIFF_CYCLES)) && --timeout);
    if (!timeout)
    dev_err(&data.rtc.dev, "Timeout trying to get valid LPSRT Counter read\n");
// Convert 47-bit counter to 32-bit raw second count
    return (u32) (read1 >> CNTR_TO_SECS_SH);
    }
// Just read the lsb from the counter, dealing with inconsistent state
#[no_mangle]
unsafe extern "C" fn rtc_read_lp_counter_lsb(data: *mut snvs_rtc_data, lsb: *mut u32) -> c_int {
    static int rtc_read_lp_counter_lsb(struct snvs_rtc_data *data, u32 *lsb)
    {
    u32 count1, count2;
    s32 diff;
    let mut timeout: c_uint = 100;
    regmap_read(data.regmap, data.offset + SNVS_LPSRTCLR, &count1);
    do {
    count2 = count1;
    regmap_read(data.regmap, data.offset + SNVS_LPSRTCLR, &count1);
    diff = count1 - count2;
    } while (((diff < 0) || (diff > MAX_RTC_READ_DIFF_CYCLES)) && --timeout);
    if (!timeout) {
    dev_err(&data.rtc.dev, "Timeout trying to get valid LPSRT Counter read\n");
    return -ETIMEDOUT;
    }
// lsb = count1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtc_write_sync_lp(data: *mut snvs_rtc_data) -> c_int {
    static int rtc_write_sync_lp(struct snvs_rtc_data *data)
    {
    u32 count1, count2;
    u32 elapsed;
    let mut timeout: c_uint = 1000;
    int ret;
    ret = rtc_read_lp_counter_lsb(data, &count1);
    if (ret)
    return ret;
// Wait for 3 CKIL cycles, about 61.0-91.5 µs
    do {
    ret = rtc_read_lp_counter_lsb(data, &count2);
    if (ret)
    return ret;
    elapsed = count2 - count1; /* wrap around _is_ handled! */
    } while (elapsed < 3 && --timeout);
    if (!timeout) {
    dev_err(&data.rtc.dev, "Timeout waiting for LPSRT Counter to change\n");
    return -ETIMEDOUT;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snvs_rtc_enable(data: *mut snvs_rtc_data, enable: bool) -> c_int {
    static int snvs_rtc_enable(struct snvs_rtc_data *data, bool enable)
    {
    let mut timeout: c_int = 1000;
    u32 lpcr;
    regmap_update_bits(data.regmap, data.offset + SNVS_LPCR, SNVS_LPCR_SRTC_ENV,
    enable ? SNVS_LPCR_SRTC_ENV : 0);
    while (--timeout) {
    regmap_read(data.regmap, data.offset + SNVS_LPCR, &lpcr);
    if (enable) {
    if (lpcr & SNVS_LPCR_SRTC_ENV)
    break;
    } else {
    if (!(lpcr & SNVS_LPCR_SRTC_ENV))
    break;
    }
    }
    if (!timeout)
    return -ETIMEDOUT;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snvs_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int snvs_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct snvs_rtc_data *data = dev_get_drvdata(dev);
    unsigned long time;
    int ret;
    ret = clk_enable(data.clk);
    if (ret)
    return ret;
    time = rtc_read_lp_counter(data);
    rtc_time64_to_tm(time, tm);
    clk_disable(data.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snvs_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int snvs_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct snvs_rtc_data *data = dev_get_drvdata(dev);
    let mut time: c_ulong = rtc_tm_to_time64(tm);
    int ret;
    ret = clk_enable(data.clk);
    if (ret)
    return ret;
// Disable RTC first
    ret = snvs_rtc_enable(data, false);
    if (ret)
    return ret;
// Write 32-bit time to 47-bit timer, leaving 15 LSBs blank
    regmap_write(data.regmap, data.offset + SNVS_LPSRTCLR, time << CNTR_TO_SECS_SH);
    regmap_write(data.regmap, data.offset + SNVS_LPSRTCMR, time >> (32 - CNTR_TO_SECS_SH));
// Enable RTC again
    ret = snvs_rtc_enable(data, true);
    clk_disable(data.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn snvs_rtc_read_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int snvs_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct snvs_rtc_data *data = dev_get_drvdata(dev);
    u32 lptar, lpsr;
    int ret;
    ret = clk_enable(data.clk);
    if (ret)
    return ret;
    regmap_read(data.regmap, data.offset + SNVS_LPTAR, &lptar);
    rtc_time64_to_tm(lptar, &alrm.time);
    regmap_read(data.regmap, data.offset + SNVS_LPSR, &lpsr);
    alrm.pending = (lpsr & SNVS_LPSR_LPTA) ? 1 : 0;
    clk_disable(data.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snvs_rtc_alarm_irq_enable(dev: *mut device, enable: c_uint) -> c_int {
    static int snvs_rtc_alarm_irq_enable(struct device *dev, unsigned int enable)
    {
    struct snvs_rtc_data *data = dev_get_drvdata(dev);
    int ret;
    ret = clk_enable(data.clk);
    if (ret)
    return ret;
    regmap_update_bits(data.regmap, data.offset + SNVS_LPCR,
    (SNVS_LPCR_LPTA_EN | SNVS_LPCR_LPWUI_EN),
    enable ? (SNVS_LPCR_LPTA_EN | SNVS_LPCR_LPWUI_EN) : 0);
    ret = rtc_write_sync_lp(data);
    clk_disable(data.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn snvs_rtc_set_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int snvs_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct snvs_rtc_data *data = dev_get_drvdata(dev);
    let mut time: c_ulong = rtc_tm_to_time64(&alrm.time);
    int ret;
    ret = clk_enable(data.clk);
    if (ret)
    return ret;
    regmap_update_bits(data.regmap, data.offset + SNVS_LPCR, SNVS_LPCR_LPTA_EN, 0);
    ret = rtc_write_sync_lp(data);
    if (ret)
    return ret;
    regmap_write(data.regmap, data.offset + SNVS_LPTAR, time);
// Clear alarm interrupt status bit
    regmap_write(data.regmap, data.offset + SNVS_LPSR, SNVS_LPSR_LPTA);
    clk_disable(data.clk);
    return snvs_rtc_alarm_irq_enable(dev, alrm.enabled);
    }
    static const struct rtc_class_ops snvs_rtc_ops = {
    .read_time = snvs_rtc_read_time,
    .set_time = snvs_rtc_set_time,
    .read_alarm = snvs_rtc_read_alarm,
    .set_alarm = snvs_rtc_set_alarm,
    .alarm_irq_enable = snvs_rtc_alarm_irq_enable,
    };
#[no_mangle]
unsafe extern "C" fn snvs_rtc_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t snvs_rtc_irq_handler(int irq, void *dev_id)
    {
    struct device *dev = dev_id;
    struct snvs_rtc_data *data = dev_get_drvdata(dev);
    u32 lpsr;
    let mut events: u32 = 0;
    clk_enable(data.clk);
    regmap_read(data.regmap, data.offset + SNVS_LPSR, &lpsr);
    if (lpsr & SNVS_LPSR_LPTA) {
    events |= (RTC_AF | RTC_IRQF);
// RTC alarm should be one-shot
    snvs_rtc_alarm_irq_enable(dev, 0);
    rtc_update_irq(data.rtc, 1, events);
    }
// clear interrupt status
    regmap_write(data.regmap, data.offset + SNVS_LPSR, lpsr);
    clk_disable(data.clk);
    return events ? IRQ_HANDLED : IRQ_NONE;
    }
    static const struct regmap_config snvs_rtc_config = {
    .reg_bits = 32,
    .val_bits = 32,
    .reg_stride = 4,
    };
#[no_mangle]
unsafe extern "C" fn snvs_rtc_action(data: *mut c_void) {
    static void snvs_rtc_action(void *data)
    {
    clk_disable_unprepare(data);
    }
#[no_mangle]
unsafe extern "C" fn snvs_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int snvs_rtc_probe(struct platform_device *pdev)
    {
    struct snvs_rtc_data *data;
    int ret;
    void __iomem *mmio;
    data = devm_kzalloc(&pdev.dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.rtc = devm_rtc_allocate_device(&pdev.dev);
    if (IS_ERR(data.rtc))
    return PTR_ERR(data.rtc);
    data.regmap = syscon_regmap_lookup_by_phandle(pdev.dev.of_node, "regmap");
    if (IS_ERR(data.regmap)) {
    dev_warn(&pdev.dev, "snvs rtc: you use old dts file, please update it\n");
    mmio = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(mmio))
    return PTR_ERR(mmio);
    data.regmap = devm_regmap_init_mmio(&pdev.dev, mmio, &snvs_rtc_config);
    } else {
    data.offset = SNVS_LPREGISTER_OFFSET;
    of_property_read_u32(pdev.dev.of_node, "offset", &data.offset);
    }
    if (IS_ERR(data.regmap)) {
    dev_err(&pdev.dev, "Can't find snvs syscon\n");
    return -ENODEV;
    }
    data.irq = platform_get_irq(pdev, 0);
    if (data.irq < 0)
    return data.irq;
    data.clk = devm_clk_get(&pdev.dev, "snvs-rtc");
    if (IS_ERR(data.clk)) {
    data.clk = core::ptr::null_mut();
    } else {
    ret = clk_prepare_enable(data.clk);
    if (ret) {
    dev_err(&pdev.dev,
    "Could not prepare or enable the snvs clock\n");
    return ret;
    }
    }
    ret = devm_add_action_or_reset(&pdev.dev, snvs_rtc_action, data.clk);
    if (ret)
    return ret;
    platform_set_drvdata(pdev, data);
// Initialize glitch detect
    regmap_write(data.regmap, data.offset + SNVS_LPPGDR, SNVS_LPPGDR_INIT);
// Clear interrupt status
    regmap_write(data.regmap, data.offset + SNVS_LPSR, 0xffffffff);
// Enable RTC
    ret = snvs_rtc_enable(data, true);
    if (ret) {
    dev_err(&pdev.dev, "failed to enable rtc %d\n", ret);
    return ret;
    }
    device_init_wakeup(&pdev.dev, true);
    ret = dev_pm_set_wake_irq(&pdev.dev, data.irq);
    if (ret)
    dev_err(&pdev.dev, "failed to enable irq wake\n");
    ret = devm_request_irq(&pdev.dev, data.irq, snvs_rtc_irq_handler,
    IRQF_SHARED, "rtc alarm", &pdev.dev);
    if (ret) {
    dev_err(&pdev.dev, "failed to request irq %d: %d\n",
    data.irq, ret);
    return ret;
    }
    data.rtc.ops = &snvs_rtc_ops;
    data.rtc.range_max = U32_MAX;
    return devm_rtc_register_device(data.rtc);
    }
#[no_mangle]
unsafe extern "C" fn snvs_rtc_suspend_noirq(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused snvs_rtc_suspend_noirq(struct device *dev)
    {
    struct snvs_rtc_data *data = dev_get_drvdata(dev);
    clk_disable(data.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snvs_rtc_resume_noirq(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused snvs_rtc_resume_noirq(struct device *dev)
    {
    struct snvs_rtc_data *data = dev_get_drvdata(dev);
    if (data.clk)
    return clk_enable(data.clk);
    return 0;
    }
    static const struct dev_pm_ops snvs_rtc_pm_ops = {
    SET_NOIRQ_SYSTEM_SLEEP_PM_OPS(snvs_rtc_suspend_noirq, snvs_rtc_resume_noirq)
    };
    static const struct of_device_id snvs_dt_ids[] = {
    { .compatible = "fsl,sec-v4.0-mon-rtc-lp", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, snvs_dt_ids);
    static struct platform_driver snvs_rtc_driver = {
    .driver = {
    .name	= "snvs_rtc",
    .pm	= &snvs_rtc_pm_ops,
    .of_match_table = snvs_dt_ids,
    },
    .probe		= snvs_rtc_probe,
    };
    module_platform_driver(snvs_rtc_driver);
    MODULE_AUTHOR("Freescale Semiconductor, Inc.");
    MODULE_DESCRIPTION("Freescale SNVS RTC Driver");
    MODULE_LICENSE("GPL");
