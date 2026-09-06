//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-zynqmp.c
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
// Xilinx Zynq Ultrascale+ MPSoC Real Time Clock Driver
//
// Copyright (C) 2015 Xilinx, Inc.
//

// RTC Registers
pub const RTC_SET_TM_WR: c_uint = 0x00;
pub const RTC_SET_TM_RD: c_uint = 0x04;
pub const RTC_CALIB_WR: c_uint = 0x08;
pub const RTC_CALIB_RD: c_uint = 0x0C;
pub const RTC_CUR_TM: c_uint = 0x10;
pub const RTC_CUR_TICK: c_uint = 0x14;
pub const RTC_ALRM: c_uint = 0x18;
pub const RTC_INT_STS: c_uint = 0x20;
pub const RTC_INT_MASK: c_uint = 0x24;
pub const RTC_INT_EN: c_uint = 0x28;
pub const RTC_INT_DIS: c_uint = 0x2C;
pub const RTC_CTRL: c_uint = 0x40;

pub const RTC_FR_DATSHIFT: c_int = 16;
pub const RTC_TICK_MASK: c_uint = 0xFFFF;

pub const RTC_CALIB_DEF: c_uint = 0x7FFF;
pub const RTC_CALIB_MASK: c_uint = 0x1FFFFF;

pub const RTC_MSEC: c_int = 1000;
pub const RTC_FR_MASK: c_uint = 0xF0000;
pub const RTC_FR_MAX_TICKS: c_int = 16;
pub const RTC_PPB: c_int = 1000000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xlnx_rtc_dev {
    pub rtc: *mut rtc_device,
    pub reg_base: *mut void __iomem,
    pub alarm_irq: c_int,
    pub sec_irq: c_int,
    pub rtc_clk: *mut clk,
    pub freq: c_uint,
}

#[no_mangle]
unsafe extern "C" fn xlnx_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int xlnx_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct xlnx_rtc_dev *xrtcdev = dev_get_drvdata(dev);
    unsigned long new_time;
//
// The value written will be updated after 1 sec into the
// seconds read register, so we need to program time +1 sec
// to get the correct time on read.
//
    new_time = rtc_tm_to_time64(tm) + 1;
    writel(new_time, xrtcdev.reg_base + RTC_SET_TM_WR);
//
// Clear the rtc interrupt status register after setting the
// time. During a read_time function, the code should read the
// RTC_INT_STATUS register and if bit 0 is still 0, it means
// that one second has not elapsed yet since RTC was set and
// the current time should be read from SET_TIME_READ register;
// otherwise, CURRENT_TIME register is read to report the time
//
    writel(RTC_INT_SEC, xrtcdev.reg_base + RTC_INT_STS);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xlnx_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int xlnx_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    u32 status;
    unsigned long read_time;
    struct xlnx_rtc_dev *xrtcdev = dev_get_drvdata(dev);
    status = readl(xrtcdev.reg_base + RTC_INT_STS);
    if (status & RTC_INT_SEC) {
//
// RTC has updated the CURRENT_TIME with the time written into
// SET_TIME_WRITE register.
//
    read_time = readl(xrtcdev.reg_base + RTC_CUR_TM);
    } else {
//
// Time written in SET_TIME_WRITE has not yet updated into
// the seconds read register, so read the time from the
// SET_TIME_WRITE instead of CURRENT_TIME register.
// Since we add +1 sec while writing, we need to -1 sec while
// reading.
//
    read_time = readl(xrtcdev.reg_base + RTC_SET_TM_RD) - 1;
    }
    rtc_time64_to_tm(read_time, tm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xlnx_rtc_read_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int xlnx_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct xlnx_rtc_dev *xrtcdev = dev_get_drvdata(dev);
    rtc_time64_to_tm(readl(xrtcdev.reg_base + RTC_ALRM), &alrm.time);
    alrm.enabled = readl(xrtcdev.reg_base + RTC_INT_MASK) & RTC_INT_ALRM;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xlnx_rtc_alarm_irq_enable(dev: *mut device, enabled: u32) -> c_int {
    static int xlnx_rtc_alarm_irq_enable(struct device *dev, u32 enabled)
    {
    struct xlnx_rtc_dev *xrtcdev = dev_get_drvdata(dev);
    unsigned int status;
    ulong timeout;
    timeout = jiffies + msecs_to_jiffies(RTC_MSEC);
    if (enabled) {
    while (1) {
    status = readl(xrtcdev.reg_base + RTC_INT_STS);
    if (!((status & RTC_ALRM_MASK) == RTC_ALRM_MASK))
    break;
    if (time_after_eq(jiffies, timeout)) {
    dev_err(dev, "Time out occur, while clearing alarm status bit\n");
    return -ETIMEDOUT;
    }
    writel(RTC_INT_ALRM, xrtcdev.reg_base + RTC_INT_STS);
    }
    writel(RTC_INT_ALRM, xrtcdev.reg_base + RTC_INT_EN);
    } else {
    writel(RTC_INT_ALRM, xrtcdev.reg_base + RTC_INT_DIS);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xlnx_rtc_set_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int xlnx_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct xlnx_rtc_dev *xrtcdev = dev_get_drvdata(dev);
    unsigned long alarm_time;
    alarm_time = rtc_tm_to_time64(&alrm.time);
    writel((u32)alarm_time, (xrtcdev.reg_base + RTC_ALRM));
    xlnx_rtc_alarm_irq_enable(dev, alrm.enabled);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xlnx_init_rtc(xrtcdev: *mut xlnx_rtc_dev) {
    static void xlnx_init_rtc(struct xlnx_rtc_dev *xrtcdev)
    {
    u32 rtc_ctrl;
// Enable RTC switch to battery when VCC_PSAUX is not available
    rtc_ctrl = readl(xrtcdev.reg_base + RTC_CTRL);
    rtc_ctrl |= RTC_BATT_EN;
    writel(rtc_ctrl, xrtcdev.reg_base + RTC_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn xlnx_rtc_read_offset(dev: *mut device, offset: *mut c_long) -> c_int {
    static int xlnx_rtc_read_offset(struct device *dev, long *offset)
    {
    struct xlnx_rtc_dev *xrtcdev = dev_get_drvdata(dev);
    unsigned int calibval, fract_data, fract_part;
    let mut freq: c_int = xrtcdev.freq;
    int max_tick, tick_mult;
    long offset_val;
// Tick to offset multiplier
    tick_mult = DIV_ROUND_CLOSEST(RTC_PPB, freq);
    calibval = readl(xrtcdev.reg_base + RTC_CALIB_RD);
// Offset with seconds ticks
    max_tick = calibval & RTC_TICK_MASK;
    offset_val = max_tick - freq;
// Convert to ppb
    offset_val *= tick_mult;
// Offset with fractional ticks
    if (calibval & RTC_FR_EN) {
    fract_data = (calibval & RTC_FR_MASK) >> RTC_FR_DATSHIFT;
    fract_part = DIV_ROUND_UP(tick_mult, RTC_FR_MAX_TICKS);
    offset_val += (fract_part * fract_data);
    }
// offset = offset_val;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xlnx_rtc_set_offset(dev: *mut device, offset: c_long) -> c_int {
    static int xlnx_rtc_set_offset(struct device *dev, long offset)
    {
    struct xlnx_rtc_dev *xrtcdev = dev_get_drvdata(dev);
    int max_tick, tick_mult, fract_offset, fract_part;
    let mut freq: c_int = xrtcdev.freq;
    unsigned int calibval;
    let mut fract_data: c_int = 0;
// Tick to offset multiplier
    tick_mult = DIV_ROUND_CLOSEST(RTC_PPB, freq);
// Number ticks for given offset
    max_tick = div_s64_rem(offset, tick_mult, &fract_offset);
    if (freq + max_tick > RTC_TICK_MASK || (freq + max_tick < 1))
    return -ERANGE;
// Number fractional ticks for given offset
    if (fract_offset) {
    fract_part = DIV_ROUND_UP(tick_mult, RTC_FR_MAX_TICKS);
    fract_data = fract_offset / fract_part;
// Subtract one from max_tick while adding fract_offset
    if (fract_offset < 0 && fract_data) {
    max_tick--;
    fract_data += RTC_FR_MAX_TICKS;
    }
    }
// Zynqmp RTC uses second and fractional tick
// counters for compensation
//
    calibval = max_tick + freq;
    if (fract_data)
    calibval |= (RTC_FR_EN | (fract_data << RTC_FR_DATSHIFT));
    writel(calibval, (xrtcdev.reg_base + RTC_CALIB_WR));
    return 0;
    }
    static const struct rtc_class_ops xlnx_rtc_ops = {
    .set_time	  = xlnx_rtc_set_time,
    .read_time	  = xlnx_rtc_read_time,
    .read_alarm	  = xlnx_rtc_read_alarm,
    .set_alarm	  = xlnx_rtc_set_alarm,
    .alarm_irq_enable = xlnx_rtc_alarm_irq_enable,
    .read_offset	  = xlnx_rtc_read_offset,
    .set_offset	  = xlnx_rtc_set_offset,
    };
#[no_mangle]
unsafe extern "C" fn xlnx_rtc_interrupt(irq: c_int, id: *mut c_void) -> irqreturn_t {
    static irqreturn_t xlnx_rtc_interrupt(int irq, void *id)
    {
    struct xlnx_rtc_dev *xrtcdev = (struct xlnx_rtc_dev *)id;
    unsigned int status;
    status = readl(xrtcdev.reg_base + RTC_INT_STS);
// Check if interrupt asserted
    if (!(status & (RTC_INT_SEC | RTC_INT_ALRM)))
    return IRQ_NONE;
// Disable RTC_INT_ALRM interrupt only
    writel(RTC_INT_ALRM, xrtcdev.reg_base + RTC_INT_DIS);
    if (status & RTC_INT_ALRM)
    rtc_update_irq(xrtcdev.rtc, 1, RTC_IRQF | RTC_AF);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn xlnx_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int xlnx_rtc_probe(struct platform_device *pdev)
    {
    struct xlnx_rtc_dev *xrtcdev;
    let mut is_alarm_set: bool = false;
    u32 pending_alrm_irq;
    u32 current_time;
    u32 alarm_time;
    int ret;
    xrtcdev = devm_kzalloc(&pdev.dev, sizeof(*xrtcdev), GFP_KERNEL);
    if (!xrtcdev)
    return -ENOMEM;
    platform_set_drvdata(pdev, xrtcdev);
    xrtcdev.rtc = devm_rtc_allocate_device(&pdev.dev);
    if (IS_ERR(xrtcdev.rtc))
    return PTR_ERR(xrtcdev.rtc);
    xrtcdev.rtc.ops = &xlnx_rtc_ops;
    xrtcdev.rtc.range_max = U32_MAX;
    xrtcdev.reg_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(xrtcdev.reg_base))
    return PTR_ERR(xrtcdev.reg_base);
// Clear any pending alarm interrupts from previous kernel/boot
    pending_alrm_irq = readl(xrtcdev.reg_base + RTC_INT_STS) & RTC_INT_ALRM;
    if (pending_alrm_irq)
    writel(pending_alrm_irq, xrtcdev.reg_base + RTC_INT_STS);
// Check if a valid alarm is already set from previous kernel/boot
    alarm_time = readl(xrtcdev.reg_base + RTC_ALRM);
    current_time = readl(xrtcdev.reg_base + RTC_CUR_TM);
    if (alarm_time > current_time && alarm_time != 0)
    is_alarm_set = true;
    xrtcdev.alarm_irq = platform_get_irq_byname(pdev, "alarm");
    if (xrtcdev.alarm_irq < 0)
    return xrtcdev.alarm_irq;
    ret = devm_request_irq(&pdev.dev, xrtcdev.alarm_irq,
    xlnx_rtc_interrupt, 0,
    dev_name(&pdev.dev), xrtcdev);
    if (ret) {
    dev_err(&pdev.dev, "request irq failed\n");
    return ret;
    }
    xrtcdev.sec_irq = platform_get_irq_byname(pdev, "sec");
    if (xrtcdev.sec_irq < 0)
    return xrtcdev.sec_irq;
    ret = devm_request_irq(&pdev.dev, xrtcdev.sec_irq,
    xlnx_rtc_interrupt, 0,
    dev_name(&pdev.dev), xrtcdev);
    if (ret) {
    dev_err(&pdev.dev, "request irq failed\n");
    return ret;
    }
// Getting the rtc info
    xrtcdev.rtc_clk = devm_clk_get_optional(&pdev.dev, "rtc");
    if (IS_ERR(xrtcdev.rtc_clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(xrtcdev.rtc_clk),
    "Failed to get rtc clock\n");
    xrtcdev.freq = clk_get_rate(xrtcdev.rtc_clk);
    if (!xrtcdev.freq) {
    ret = of_property_read_u32(pdev.dev.of_node, "calibration",
    &xrtcdev.freq);
    if (ret)
    xrtcdev.freq = RTC_CALIB_DEF;
    } else {
    xrtcdev.freq--;
    }
    if (xrtcdev.freq > RTC_TICK_MASK) {
    dev_err(&pdev.dev, "Invalid RTC calibration value\n");
    return -EINVAL;
    }
    ret = readl(xrtcdev.reg_base + RTC_CALIB_RD);
    if (!ret)
    writel(xrtcdev.freq, (xrtcdev.reg_base + RTC_CALIB_WR));
    xlnx_init_rtc(xrtcdev);
// Re-enable alarm interrupt if a valid alarm was found
    if (is_alarm_set)
    writel(RTC_INT_ALRM, xrtcdev.reg_base + RTC_INT_EN);
    device_init_wakeup(&pdev.dev, true);
    return devm_rtc_register_device(xrtcdev.rtc);
    }
#[no_mangle]
unsafe extern "C" fn xlnx_rtc_remove(pdev: *mut platform_device) {
    static void xlnx_rtc_remove(struct platform_device *pdev)
    {
    xlnx_rtc_alarm_irq_enable(&pdev.dev, 0);
    device_init_wakeup(&pdev.dev, false);
    }
#[no_mangle]
unsafe extern "C" fn xlnx_rtc_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused xlnx_rtc_suspend(struct device *dev)
    {
    struct xlnx_rtc_dev *xrtcdev = dev_get_drvdata(dev);
    if (device_may_wakeup(dev))
    enable_irq_wake(xrtcdev.alarm_irq);
    else
    xlnx_rtc_alarm_irq_enable(dev, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xlnx_rtc_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused xlnx_rtc_resume(struct device *dev)
    {
    struct xlnx_rtc_dev *xrtcdev = dev_get_drvdata(dev);
    if (device_may_wakeup(dev))
    disable_irq_wake(xrtcdev.alarm_irq);
    else
    xlnx_rtc_alarm_irq_enable(dev, 1);
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(xlnx_rtc_pm_ops, xlnx_rtc_suspend, xlnx_rtc_resume);
    static const struct of_device_id xlnx_rtc_of_match[] = {
    {.compatible = "xlnx,zynqmp-rtc" },
    { }
    };
    MODULE_DEVICE_TABLE(of, xlnx_rtc_of_match);
    static struct platform_driver xlnx_rtc_driver = {
    .probe		= xlnx_rtc_probe,
    .remove		= xlnx_rtc_remove,
    .driver		= {
    .name	= KBUILD_MODNAME,
    .pm	= &xlnx_rtc_pm_ops,
    .of_match_table	= xlnx_rtc_of_match,
    },
    };
    module_platform_driver(xlnx_rtc_driver);
    MODULE_DESCRIPTION("Xilinx Zynq MPSoC RTC driver");
    MODULE_AUTHOR("Xilinx Inc.");
    MODULE_LICENSE("GPL v2");
