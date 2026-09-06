//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/pm8916_wdt.c
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

pub const PON_POFF_REASON1: c_uint = 0x0c;

pub const PON_POFF_REASON2: c_uint = 0x0d;

pub const PON_INT_RT_STS: c_uint = 0x10;

pub const PON_PMIC_WD_RESET_S1_TIMER: c_uint = 0x54;
pub const PON_PMIC_WD_RESET_S2_TIMER: c_uint = 0x55;
pub const PON_PMIC_WD_RESET_S2_CTL: c_uint = 0x56;
pub const RESET_TYPE_WARM: c_uint = 0x01;
pub const RESET_TYPE_SHUTDOWN: c_uint = 0x04;
pub const RESET_TYPE_HARD: c_uint = 0x07;
pub const PON_PMIC_WD_RESET_S2_CTL2: c_uint = 0x57;

pub const PON_PMIC_WD_RESET_PET: c_uint = 0x58;

pub const PM8916_WDT_DEFAULT_TIMEOUT: c_int = 32;
pub const PM8916_WDT_MIN_TIMEOUT: c_int = 1;
pub const PM8916_WDT_MAX_TIMEOUT: c_int = 127;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm8916_wdt {
    pub regmap: *mut regmap,
    pub wdev: watchdog_device,
    pub baseaddr: u32,
}

#[no_mangle]
unsafe extern "C" fn pm8916_wdt_start(wdev: *mut watchdog_device) -> c_int {
    static int pm8916_wdt_start(struct watchdog_device *wdev)
    {
    struct pm8916_wdt *wdt = watchdog_get_drvdata(wdev);
    return regmap_update_bits(wdt.regmap,
    wdt.baseaddr + PON_PMIC_WD_RESET_S2_CTL2,
    S2_RESET_EN_BIT, S2_RESET_EN_BIT);
    }
#[no_mangle]
unsafe extern "C" fn pm8916_wdt_stop(wdev: *mut watchdog_device) -> c_int {
    static int pm8916_wdt_stop(struct watchdog_device *wdev)
    {
    struct pm8916_wdt *wdt = watchdog_get_drvdata(wdev);
    return regmap_update_bits(wdt.regmap,
    wdt.baseaddr + PON_PMIC_WD_RESET_S2_CTL2,
    S2_RESET_EN_BIT, 0);
    }
#[no_mangle]
unsafe extern "C" fn pm8916_wdt_ping(wdev: *mut watchdog_device) -> c_int {
    static int pm8916_wdt_ping(struct watchdog_device *wdev)
    {
    struct pm8916_wdt *wdt = watchdog_get_drvdata(wdev);
    return regmap_write(wdt.regmap, wdt.baseaddr + PON_PMIC_WD_RESET_PET,
    WATCHDOG_PET_BIT);
    }
#[no_mangle]
unsafe extern "C" fn pm8916_wdt_configure_timers(wdev: *mut watchdog_device) -> c_int {
    static int pm8916_wdt_configure_timers(struct watchdog_device *wdev)
    {
    struct pm8916_wdt *wdt = watchdog_get_drvdata(wdev);
    int err;
    err = regmap_write(wdt.regmap,
    wdt.baseaddr + PON_PMIC_WD_RESET_S1_TIMER,
    wdev.timeout - wdev.pretimeout);
    if (err)
    return err;
    return regmap_write(wdt.regmap,
    wdt.baseaddr + PON_PMIC_WD_RESET_S2_TIMER,
    wdev.pretimeout);
    }
    static int pm8916_wdt_set_timeout(struct watchdog_device *wdev,
    unsigned int timeout)
    {
    wdev.timeout = timeout;
    return pm8916_wdt_configure_timers(wdev);
    }
    static int pm8916_wdt_set_pretimeout(struct watchdog_device *wdev,
    unsigned int pretimeout)
    {
    wdev.pretimeout = pretimeout;
    return pm8916_wdt_configure_timers(wdev);
    }
#[no_mangle]
unsafe extern "C" fn pm8916_wdt_isr(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t pm8916_wdt_isr(int irq, void *arg)
    {
    struct pm8916_wdt *wdt = arg;
    int err, sts;
    err = regmap_read(wdt.regmap, wdt.baseaddr + PON_INT_RT_STS, &sts);
    if (err)
    return IRQ_HANDLED;
    if (sts & PMIC_WD_BARK_STS_BIT)
    watchdog_notify_pretimeout(&wdt.wdev);
    return IRQ_HANDLED;
    }
    static const struct watchdog_info pm8916_wdt_ident = {
    .options = WDIOF_SETTIMEOUT | WDIOF_KEEPALIVEPING | WDIOF_MAGICCLOSE |
    WDIOF_OVERHEAT | WDIOF_CARDRESET | WDIOF_POWERUNDER,
    .identity = "QCOM PM8916 PON WDT",
    };
    static const struct watchdog_info pm8916_wdt_pt_ident = {
    .options = WDIOF_SETTIMEOUT | WDIOF_KEEPALIVEPING | WDIOF_MAGICCLOSE |
    WDIOF_OVERHEAT | WDIOF_CARDRESET | WDIOF_POWERUNDER |
    WDIOF_PRETIMEOUT,
    .identity = "QCOM PM8916 PON WDT",
    };
    static const struct watchdog_ops pm8916_wdt_ops = {
    .owner = THIS_MODULE,
    .start = pm8916_wdt_start,
    .stop = pm8916_wdt_stop,
    .ping = pm8916_wdt_ping,
    .set_timeout = pm8916_wdt_set_timeout,
    .set_pretimeout = pm8916_wdt_set_pretimeout,
    };
#[no_mangle]
unsafe extern "C" fn pm8916_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int pm8916_wdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct pm8916_wdt *wdt;
    struct device *parent;
    unsigned int val;
    int err, irq;
    u8 poff[2];
    wdt = devm_kzalloc(dev, sizeof(*wdt), GFP_KERNEL);
    if (!wdt)
    return -ENOMEM;
    parent = dev.parent;
//
// The pm8916-pon-wdt is a child of the pon device, which is a child
// of the pm8916 mfd device. We want access to the pm8916 registers.
// Retrieve regmap from pm8916 (parent->parent) and base address
// from pm8916-pon (pon).
//
    wdt.regmap = dev_get_regmap(parent.parent, core::ptr::null_mut());
    if (!wdt.regmap) {
    dev_err(dev, "failed to locate regmap\n");
    return -ENODEV;
    }
    err = device_property_read_u32(parent, "reg", &wdt.baseaddr);
    if (err) {
    dev_err(dev, "failed to get pm8916-pon address\n");
    return err;
    }
    irq = platform_get_irq(pdev, 0);
    if (irq > 0) {
    err = devm_request_irq(dev, irq, pm8916_wdt_isr, 0,
    "pm8916_wdt", wdt);
    if (err)
    return err;
    wdt.wdev.info = &pm8916_wdt_pt_ident;
    } else {
    if (irq == -EPROBE_DEFER)
    return -EPROBE_DEFER;
    wdt.wdev.info = &pm8916_wdt_ident;
    }
    err = regmap_bulk_read(wdt.regmap, wdt.baseaddr + PON_POFF_REASON1,
    &poff, ARRAY_SIZE(poff));
    if (err) {
    dev_err(dev, "failed to read POFF reason: %d\n", err);
    return err;
    }
    dev_dbg(dev, "POFF reason: %#x %#x\n", poff[0], poff[1]);
    if (poff[0] & PON_POFF_REASON1_PMIC_WD)
    wdt.wdev.bootstatus |= WDIOF_CARDRESET;
    if (poff[1] & PON_POFF_REASON2_UVLO)
    wdt.wdev.bootstatus |= WDIOF_POWERUNDER;
    if (poff[1] & PON_POFF_REASON2_OTST3)
    wdt.wdev.bootstatus |= WDIOF_OVERHEAT;
    err = regmap_read(wdt.regmap, wdt.baseaddr + PON_PMIC_WD_RESET_S2_CTL2,
    &val);
    if (err)  {
    dev_err(dev, "failed to check if watchdog is active: %d\n", err);
    return err;
    }
    if (val & S2_RESET_EN_BIT)
    set_bit(WDOG_HW_RUNNING, &wdt.wdev.status);
// Configure watchdog to hard-reset mode
    err = regmap_write(wdt.regmap,
    wdt.baseaddr + PON_PMIC_WD_RESET_S2_CTL,
    RESET_TYPE_HARD);
    if (err) {
    dev_err(dev, "failed configure watchdog\n");
    return err;
    }
    wdt.wdev.ops = &pm8916_wdt_ops;
    wdt.wdev.parent = dev;
    wdt.wdev.min_timeout = PM8916_WDT_MIN_TIMEOUT;
    wdt.wdev.max_timeout = PM8916_WDT_MAX_TIMEOUT;
    wdt.wdev.timeout = PM8916_WDT_DEFAULT_TIMEOUT;
    wdt.wdev.pretimeout = 0;
    watchdog_set_drvdata(&wdt.wdev, wdt);
    platform_set_drvdata(pdev, wdt);
    watchdog_init_timeout(&wdt.wdev, 0, dev);
    pm8916_wdt_configure_timers(&wdt.wdev);
    return devm_watchdog_register_device(dev, &wdt.wdev);
    }
#[no_mangle]
unsafe extern "C" fn pm8916_wdt_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused pm8916_wdt_suspend(struct device *dev)
    {
    struct pm8916_wdt *wdt = dev_get_drvdata(dev);
    if (watchdog_active(&wdt.wdev))
    return pm8916_wdt_stop(&wdt.wdev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pm8916_wdt_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused pm8916_wdt_resume(struct device *dev)
    {
    struct pm8916_wdt *wdt = dev_get_drvdata(dev);
    if (watchdog_active(&wdt.wdev))
    return pm8916_wdt_start(&wdt.wdev);
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(pm8916_wdt_pm_ops, pm8916_wdt_suspend,
    pm8916_wdt_resume);
    static const struct of_device_id pm8916_wdt_id_table[] = {
    { .compatible = "qcom,pm8916-wdt" },
    { }
    };
    MODULE_DEVICE_TABLE(of, pm8916_wdt_id_table);
    static struct platform_driver pm8916_wdt_driver = {
    .probe = pm8916_wdt_probe,
    .driver = {
    .name = "pm8916-wdt",
    .of_match_table = pm8916_wdt_id_table,
    .pm = &pm8916_wdt_pm_ops,
    },
    };
    module_platform_driver(pm8916_wdt_driver);
    MODULE_AUTHOR("Loic Poulain <loic.poulain@linaro.org>");
    MODULE_DESCRIPTION("Qualcomm pm8916 watchdog driver");
    MODULE_LICENSE("GPL v2");
