//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-at91rm9200.c
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
// Real Time Clock interface for Linux on Atmel AT91RM9200
//
// Copyright (C) 2002 Rick Bronson
//
// Converted to RTC class model by Andrew Victor
//
// Ported to Linux 2.6 by Steven Scholz
// Based on s3c2410-rtc.c Simtec Electronics
//
// Based on sa1100-rtc.c by Nils Faerber
// Based on rtc.c by Paul Gortmaker
//

pub const AT91_RTC_CR: c_uint = 0x00			/* Control Register */;

pub const AT91_RTC_MR: c_uint = 0x04			/* Mode Register */;

pub const AT91_RTC_TIMR: c_uint = 0x08			/* Time Register */;

pub const AT91_RTC_CALR: c_uint = 0x0c			/* Calendar Register */;

pub const AT91_RTC_TIMALR: c_uint = 0x10			/* Time Alarm Register */;

pub const AT91_RTC_CALALR: c_uint = 0x14			/* Calendar Alarm Register */;

pub const AT91_RTC_SR: c_uint = 0x18			/* Status Register */;

pub const AT91_RTC_SCCR: c_uint = 0x1c			/* Status Clear Command Register */;
pub const AT91_RTC_IER: c_uint = 0x20			/* Interrupt Enable Register */;
pub const AT91_RTC_IDR: c_uint = 0x24			/* Interrupt Disable Register */;
pub const AT91_RTC_IMR: c_uint = 0x28			/* Interrupt Mask Register */;
pub const AT91_RTC_VER: c_uint = 0x2c			/* Valid Entry Register */;

pub const AT91_RTC_CORR_DIVIDEND: c_int = 3906000;
pub const AT91_RTC_CORR_LOW_RATIO: c_int = 20;

    readl_relaxed(at91_rtc_regs + field)

    writel_relaxed((val), at91_rtc_regs + field)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct at91_rtc_config {
    pub use_shadow_imr: bool,
    pub has_correction: bool,
}

    static const struct at91_rtc_config *at91_rtc_config;
    static DECLARE_COMPLETION(at91_rtc_updated);
    static DECLARE_COMPLETION(at91_rtc_upd_rdy);
    static void __iomem *at91_rtc_regs;
    static int irq;
    static DEFINE_SPINLOCK(at91_rtc_lock);
    static u32 at91_rtc_shadow_imr;
    static bool suspended;
    static DEFINE_SPINLOCK(suspended_lock);
    static unsigned long cached_events;
    static u32 at91_rtc_imr;
    static struct clk *sclk;
#[no_mangle]
unsafe extern "C" fn at91_rtc_write_ier(mask: u32) {
    static void at91_rtc_write_ier(u32 mask)
    {
    unsigned long flags;
    spin_lock_irqsave(&at91_rtc_lock, flags);
    at91_rtc_shadow_imr |= mask;
    at91_rtc_write(AT91_RTC_IER, mask);
    spin_unlock_irqrestore(&at91_rtc_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn at91_rtc_write_idr(mask: u32) {
    static void at91_rtc_write_idr(u32 mask)
    {
    unsigned long flags;
    spin_lock_irqsave(&at91_rtc_lock, flags);
    at91_rtc_write(AT91_RTC_IDR, mask);
//
// Register read back (of any RTC-register) needed to make sure
// IDR-register write has reached the peripheral before updating
// shadow mask.
//
// Note that there is still a possibility that the mask is updated
// before interrupts have actually been disabled in hardware. The only
// way to be certain would be to poll the IMR-register, which is
// the very register we are trying to emulate. The register read back
// is a reasonable heuristic.
//
    at91_rtc_read(AT91_RTC_SR);
    at91_rtc_shadow_imr &= ~mask;
    spin_unlock_irqrestore(&at91_rtc_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn at91_rtc_read_imr() -> u32 {
    static u32 at91_rtc_read_imr(void)
    {
    unsigned long flags;
    u32 mask;
    if (at91_rtc_config.use_shadow_imr) {
    spin_lock_irqsave(&at91_rtc_lock, flags);
    mask = at91_rtc_shadow_imr;
    spin_unlock_irqrestore(&at91_rtc_lock, flags);
    } else {
    mask = at91_rtc_read(AT91_RTC_IMR);
    }
    return mask;
    }
//
// Decode time/date into rtc_time structure
//
    static void at91_rtc_decodetime(unsigned int timereg, unsigned int calreg,
    struct rtc_time *tm)
    {
    unsigned int time, date;
// must read twice in case it changes
    do {
    time = at91_rtc_read(timereg);
    date = at91_rtc_read(calreg);
    } while ((time != at91_rtc_read(timereg)) ||
    (date != at91_rtc_read(calreg)));
    tm.tm_sec  = bcd2bin(FIELD_GET(AT91_RTC_SEC, time));
    tm.tm_min  = bcd2bin(FIELD_GET(AT91_RTC_MIN, time));
    tm.tm_hour = bcd2bin(FIELD_GET(AT91_RTC_HOUR, time));
//
// The Calendar Alarm register does not have a field for
// the year - so these will return an invalid value.
//
    tm.tm_year  = bcd2bin(date & AT91_RTC_CENT) * 100;	/* century */
    tm.tm_year += bcd2bin(FIELD_GET(AT91_RTC_YEAR, date));	/* year */
    tm.tm_wday = bcd2bin(FIELD_GET(AT91_RTC_DAY, date)) - 1;	/* day of the week [0-6], Sunday=0 */
    tm.tm_mon  = bcd2bin(FIELD_GET(AT91_RTC_MONTH, date)) - 1;
    tm.tm_mday = bcd2bin(FIELD_GET(AT91_RTC_DATE, date));
    }
//
// Read current time and date in RTC
//
#[no_mangle]
unsafe extern "C" fn at91_rtc_readtime(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int at91_rtc_readtime(struct device *dev, struct rtc_time *tm)
    {
    at91_rtc_decodetime(AT91_RTC_TIMR, AT91_RTC_CALR, tm);
    tm.tm_yday = rtc_year_days(tm.tm_mday, tm.tm_mon, tm.tm_year);
    tm.tm_year = tm.tm_year - 1900;
    dev_dbg(dev, "%s(): %ptR\n", __func__, tm);
    return 0;
    }
//
// Set current time and date in RTC
//
#[no_mangle]
unsafe extern "C" fn at91_rtc_settime(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int at91_rtc_settime(struct device *dev, struct rtc_time *tm)
    {
    unsigned long cr;
    dev_dbg(dev, "%s(): %ptR\n", __func__, tm);
    wait_for_completion(&at91_rtc_upd_rdy);
// Stop Time/Calendar from counting
    cr = at91_rtc_read(AT91_RTC_CR);
    at91_rtc_write(AT91_RTC_CR, cr | AT91_RTC_UPDCAL | AT91_RTC_UPDTIM);
    at91_rtc_write_ier(AT91_RTC_ACKUPD);
    wait_for_completion(&at91_rtc_updated);	/* wait for ACKUPD interrupt */
    at91_rtc_write_idr(AT91_RTC_ACKUPD);
    at91_rtc_write(AT91_RTC_TIMR,
    FIELD_PREP(AT91_RTC_SEC, bin2bcd(tm.tm_sec))
    | FIELD_PREP(AT91_RTC_MIN, bin2bcd(tm.tm_min))
    | FIELD_PREP(AT91_RTC_HOUR, bin2bcd(tm.tm_hour)));
    at91_rtc_write(AT91_RTC_CALR,
    FIELD_PREP(AT91_RTC_CENT,
    bin2bcd((tm.tm_year + 1900) / 100))
    | FIELD_PREP(AT91_RTC_YEAR, bin2bcd(tm.tm_year % 100))
    | FIELD_PREP(AT91_RTC_MONTH, bin2bcd(tm.tm_mon + 1))
    | FIELD_PREP(AT91_RTC_DAY, bin2bcd(tm.tm_wday + 1))
    | FIELD_PREP(AT91_RTC_DATE, bin2bcd(tm.tm_mday)));
// Restart Time/Calendar
    cr = at91_rtc_read(AT91_RTC_CR);
    at91_rtc_write(AT91_RTC_SCCR, AT91_RTC_SECEV);
    at91_rtc_write(AT91_RTC_CR, cr & ~(AT91_RTC_UPDCAL | AT91_RTC_UPDTIM));
    at91_rtc_write_ier(AT91_RTC_SECEV);
    return 0;
    }
//
// Read alarm time and date in RTC
//
#[no_mangle]
unsafe extern "C" fn at91_rtc_readalarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int at91_rtc_readalarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct rtc_time *tm = &alrm.time;
    at91_rtc_decodetime(AT91_RTC_TIMALR, AT91_RTC_CALALR, tm);
    tm.tm_year = -1;
    alrm.enabled = (at91_rtc_read_imr() & AT91_RTC_ALARM)
    ? 1 : 0;
    dev_dbg(dev, "%s(): %ptR %sabled\n", __func__, tm,
    alrm.enabled ? "en" : "dis");
    return 0;
    }
//
// Set alarm time and date in RTC
//
#[no_mangle]
unsafe extern "C" fn at91_rtc_setalarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int at91_rtc_setalarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    let mut tm: rtc_time = alrm.time;
    at91_rtc_write_idr(AT91_RTC_ALARM);
    at91_rtc_write(AT91_RTC_TIMALR,
    FIELD_PREP(AT91_RTC_SEC, bin2bcd(alrm.time.tm_sec))
    | FIELD_PREP(AT91_RTC_MIN, bin2bcd(alrm.time.tm_min))
    | FIELD_PREP(AT91_RTC_HOUR, bin2bcd(alrm.time.tm_hour))
    | AT91_RTC_HOUREN | AT91_RTC_MINEN | AT91_RTC_SECEN);
    at91_rtc_write(AT91_RTC_CALALR,
    FIELD_PREP(AT91_RTC_MONTH, bin2bcd(alrm.time.tm_mon + 1))
    | FIELD_PREP(AT91_RTC_DATE, bin2bcd(alrm.time.tm_mday))
    | AT91_RTC_DATEEN | AT91_RTC_MTHEN);
    if (alrm.enabled) {
    at91_rtc_write(AT91_RTC_SCCR, AT91_RTC_ALARM);
    at91_rtc_write_ier(AT91_RTC_ALARM);
    }
    dev_dbg(dev, "%s(): %ptR\n", __func__, &tm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn at91_rtc_alarm_irq_enable(dev: *mut device, enabled: c_uint) -> c_int {
    static int at91_rtc_alarm_irq_enable(struct device *dev, unsigned int enabled)
    {
    dev_dbg(dev, "%s(): cmd=%08x\n", __func__, enabled);
    if (enabled) {
    at91_rtc_write(AT91_RTC_SCCR, AT91_RTC_ALARM);
    at91_rtc_write_ier(AT91_RTC_ALARM);
    } else
    at91_rtc_write_idr(AT91_RTC_ALARM);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn at91_rtc_readoffset(dev: *mut device, offset: *mut c_long) -> c_int {
    static int at91_rtc_readoffset(struct device *dev, long *offset)
    {
    let mut mr: u32 = at91_rtc_read(AT91_RTC_MR);
    let mut val: c_long = FIELD_GET(AT91_RTC_CORRECTION, mr);
    if (!val) {
// offset = 0;
    return 0;
    }
    val++;
    if (!(mr & AT91_RTC_NEGPPM))
    val = -val;
    if (!(mr & AT91_RTC_HIGHPPM))
    val *= AT91_RTC_CORR_LOW_RATIO;
// offset = DIV_ROUND_CLOSEST(AT91_RTC_CORR_DIVIDEND, val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn at91_rtc_setoffset(dev: *mut device, offset: c_long) -> c_int {
    static int at91_rtc_setoffset(struct device *dev, long offset)
    {
    long corr;
    u32 mr;
    if (offset > AT91_RTC_CORR_DIVIDEND / 2)
    return -ERANGE;
    if (offset < -AT91_RTC_CORR_DIVIDEND / 2)
    return -ERANGE;
    mr = at91_rtc_read(AT91_RTC_MR);
    mr &= ~(AT91_RTC_NEGPPM | AT91_RTC_CORRECTION | AT91_RTC_HIGHPPM);
    if (offset > 0)
    mr |= AT91_RTC_NEGPPM;
    else
    offset = -offset;
// offset less than 764 ppb, disable correction
    if (offset < 764) {
    at91_rtc_write(AT91_RTC_MR, mr & ~AT91_RTC_NEGPPM);
    return 0;
    }
//
// 29208 ppb is the perfect cutoff between low range and high range
// low range values are never better than high range value after that.
//
    if (offset < 29208) {
    corr = DIV_ROUND_CLOSEST(AT91_RTC_CORR_DIVIDEND, offset * AT91_RTC_CORR_LOW_RATIO);
    } else {
    corr = DIV_ROUND_CLOSEST(AT91_RTC_CORR_DIVIDEND, offset);
    mr |= AT91_RTC_HIGHPPM;
    }
    if (corr > 128)
    corr = 128;
    mr |= FIELD_PREP(AT91_RTC_CORRECTION, corr - 1);
    at91_rtc_write(AT91_RTC_MR, mr);
    return 0;
    }
//
// IRQ handler for the RTC
//
#[no_mangle]
unsafe extern "C" fn at91_rtc_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t at91_rtc_interrupt(int irq, void *dev_id)
    {
    struct platform_device *pdev = dev_id;
    struct rtc_device *rtc = platform_get_drvdata(pdev);
    unsigned int rtsr;
    let mut events: c_ulong = 0;
    let mut ret: c_int = IRQ_NONE;
    spin_lock(&suspended_lock);
    rtsr = at91_rtc_read(AT91_RTC_SR) & at91_rtc_read_imr();
    if (rtsr) {		/* this interrupt is shared!  Is it ours? */
    if (rtsr & AT91_RTC_ALARM)
    events |= (RTC_AF | RTC_IRQF);
    if (rtsr & AT91_RTC_SECEV) {
    complete(&at91_rtc_upd_rdy);
    at91_rtc_write_idr(AT91_RTC_SECEV);
    }
    if (rtsr & AT91_RTC_ACKUPD)
    complete(&at91_rtc_updated);
    at91_rtc_write(AT91_RTC_SCCR, rtsr);	/* clear status reg */
    if (!suspended) {
    rtc_update_irq(rtc, 1, events);
    dev_dbg(&pdev.dev, "%s(): num=%ld, events=0x%02lx\n",
    __func__, events >> 8, events & 0x000000FF);
    } else {
    cached_events |= events;
    at91_rtc_write_idr(at91_rtc_imr);
    pm_system_wakeup();
    }
    ret = IRQ_HANDLED;
    }
    spin_unlock(&suspended_lock);
    return ret;
    }
    static const struct at91_rtc_config at91rm9200_config = {
    };
    static const struct at91_rtc_config at91sam9x5_config = {
    .use_shadow_imr	= true,
    };
    static const struct at91_rtc_config sama5d4_config = {
    .has_correction = true,
    };
    static const struct of_device_id at91_rtc_dt_ids[] = {
    {
    .compatible = "atmel,at91rm9200-rtc",
    .data = &at91rm9200_config,
    }, {
    .compatible = "atmel,at91sam9x5-rtc",
    .data = &at91sam9x5_config,
    }, {
    .compatible = "atmel,sama5d4-rtc",
    .data = &sama5d4_config,
    }, {
    .compatible = "atmel,sama5d2-rtc",
    .data = &sama5d4_config,
    }, {
    .compatible = "microchip,sam9x60-rtc",
    .data = &sama5d4_config,
    }, {
// sentinel
    }
    };
    MODULE_DEVICE_TABLE(of, at91_rtc_dt_ids);
    static const struct rtc_class_ops at91_rtc_ops = {
    .read_time	= at91_rtc_readtime,
    .set_time	= at91_rtc_settime,
    .read_alarm	= at91_rtc_readalarm,
    .set_alarm	= at91_rtc_setalarm,
    .alarm_irq_enable = at91_rtc_alarm_irq_enable,
    };
    static const struct rtc_class_ops sama5d4_rtc_ops = {
    .read_time	= at91_rtc_readtime,
    .set_time	= at91_rtc_settime,
    .read_alarm	= at91_rtc_readalarm,
    .set_alarm	= at91_rtc_setalarm,
    .alarm_irq_enable = at91_rtc_alarm_irq_enable,
    .set_offset	= at91_rtc_setoffset,
    .read_offset	= at91_rtc_readoffset,
    };
//
// Initialize and install RTC driver
//
#[no_mangle]
unsafe extern "C" fn at91_rtc_probe(pdev: *mut platform_device) -> int __init {
    static int __init at91_rtc_probe(struct platform_device *pdev)
    {
    struct rtc_device *rtc;
    struct resource *regs;
    let mut ret: c_int = 0;
    at91_rtc_config = of_device_get_match_data(&pdev.dev);
    if (!at91_rtc_config)
    return -ENODEV;
    regs = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!regs) {
    dev_err(&pdev.dev, "no mmio resource defined\n");
    return -ENXIO;
    }
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return -ENXIO;
    at91_rtc_regs = devm_ioremap(&pdev.dev, regs.start,
    resource_size(regs));
    if (!at91_rtc_regs) {
    dev_err(&pdev.dev, "failed to map registers, aborting.\n");
    return -ENOMEM;
    }
    rtc = devm_rtc_allocate_device(&pdev.dev);
    if (IS_ERR(rtc))
    return PTR_ERR(rtc);
    platform_set_drvdata(pdev, rtc);
    sclk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(sclk))
    return PTR_ERR(sclk);
    ret = clk_prepare_enable(sclk);
    if (ret) {
    dev_err(&pdev.dev, "Could not enable slow clock\n");
    return ret;
    }
    at91_rtc_write(AT91_RTC_CR, 0);
    at91_rtc_write(AT91_RTC_MR, at91_rtc_read(AT91_RTC_MR) & ~AT91_RTC_HRMOD);
// Disable all interrupts
    at91_rtc_write_idr(AT91_RTC_ACKUPD | AT91_RTC_ALARM |
    AT91_RTC_SECEV | AT91_RTC_TIMEV |
    AT91_RTC_CALEV);
    ret = devm_request_irq(&pdev.dev, irq, at91_rtc_interrupt,
    IRQF_SHARED | IRQF_COND_SUSPEND,
    "at91_rtc", pdev);
    if (ret) {
    dev_err(&pdev.dev, "IRQ %d already in use.\n", irq);
    goto err_clk;
    }
// cpu init code should really have flagged this device as
// being wake-capable; if it didn't, do that here.
//
    if (!device_can_wakeup(&pdev.dev))
    device_init_wakeup(&pdev.dev, true);
    if (at91_rtc_config.has_correction)
    rtc.ops = &sama5d4_rtc_ops;
    else
    rtc.ops = &at91_rtc_ops;
    rtc.range_min = RTC_TIMESTAMP_BEGIN_1900;
    rtc.range_max = RTC_TIMESTAMP_END_2099;
    ret = devm_rtc_register_device(rtc);
    if (ret)
    goto err_clk;
// enable SECEV interrupt in order to initialize at91_rtc_upd_rdy
// completion.
//
    at91_rtc_write_ier(AT91_RTC_SECEV);
    dev_info(&pdev.dev, "AT91 Real Time Clock driver.\n");
    return 0;
    err_clk:
    clk_disable_unprepare(sclk);
    return ret;
    }
//
// Disable and remove the RTC driver
//
#[no_mangle]
unsafe extern "C" fn at91_rtc_remove(pdev: *mut platform_device) -> void __exit {
    static void __exit at91_rtc_remove(struct platform_device *pdev)
    {
// Disable all interrupts
    at91_rtc_write_idr(AT91_RTC_ACKUPD | AT91_RTC_ALARM |
    AT91_RTC_SECEV | AT91_RTC_TIMEV |
    AT91_RTC_CALEV);
    clk_disable_unprepare(sclk);
    }
#[no_mangle]
unsafe extern "C" fn at91_rtc_shutdown(pdev: *mut platform_device) {
    static void at91_rtc_shutdown(struct platform_device *pdev)
    {
// Disable all interrupts
    at91_rtc_write(AT91_RTC_IDR, AT91_RTC_ACKUPD | AT91_RTC_ALARM |
    AT91_RTC_SECEV | AT91_RTC_TIMEV |
    AT91_RTC_CALEV);
    }

// AT91RM9200 RTC Power management control
#[no_mangle]
unsafe extern "C" fn at91_rtc_suspend(dev: *mut device) -> c_int {
    static int at91_rtc_suspend(struct device *dev)
    {
// this IRQ is shared with DBGU and other hardware which isn't
// necessarily doing PM like we are...
//
    at91_rtc_write(AT91_RTC_SCCR, AT91_RTC_ALARM);
    at91_rtc_imr = at91_rtc_read_imr()
    & (AT91_RTC_ALARM|AT91_RTC_SECEV);
    if (at91_rtc_imr) {
    if (device_may_wakeup(dev)) {
    unsigned long flags;
    enable_irq_wake(irq);
    spin_lock_irqsave(&suspended_lock, flags);
    suspended = true;
    spin_unlock_irqrestore(&suspended_lock, flags);
    } else {
    at91_rtc_write_idr(at91_rtc_imr);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn at91_rtc_resume(dev: *mut device) -> c_int {
    static int at91_rtc_resume(struct device *dev)
    {
    struct rtc_device *rtc = dev_get_drvdata(dev);
    if (at91_rtc_imr) {
    if (device_may_wakeup(dev)) {
    unsigned long flags;
    spin_lock_irqsave(&suspended_lock, flags);
    if (cached_events) {
    rtc_update_irq(rtc, 1, cached_events);
    cached_events = 0;
    }
    suspended = false;
    spin_unlock_irqrestore(&suspended_lock, flags);
    disable_irq_wake(irq);
    }
    at91_rtc_write_ier(at91_rtc_imr);
    }
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(at91_rtc_pm_ops, at91_rtc_suspend, at91_rtc_resume);
//
// at91_rtc_remove() lives in .exit.text. For drivers registered via
// module_platform_driver_probe() this is ok because they cannot get unbound at
// runtime. So mark the driver struct with __refdata to prevent modpost
// triggering a section mismatch warning.
//
    static struct platform_driver at91_rtc_driver __refdata = {
    .remove		= __exit_p(at91_rtc_remove),
    .shutdown	= at91_rtc_shutdown,
    .driver		= {
    .name	= "at91_rtc",
    .pm	= &at91_rtc_pm_ops,
    .of_match_table = at91_rtc_dt_ids,
    },
    };
    module_platform_driver_probe(at91_rtc_driver, at91_rtc_probe);
    MODULE_AUTHOR("Rick Bronson");
    MODULE_DESCRIPTION("RTC driver for Atmel AT91RM9200");
    MODULE_LICENSE("GPL");
