//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/imx_sc_wdt.c
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
// Copyright 2018-2019 NXP.
//

pub const DEFAULT_TIMEOUT: c_int = 60;
//
// Software timer tick implemented in scfw side, support 10ms to 0xffffffff ms
// in theory, but for normal case, 1s~128s is enough, you can change this max
// value in case it's not enough.
//
pub const MAX_TIMEOUT: c_int = 128;
pub const IMX_SIP_TIMER: c_uint = 0xC2000002;
pub const IMX_SIP_TIMER_START_WDOG: c_uint = 0x01;
pub const IMX_SIP_TIMER_STOP_WDOG: c_uint = 0x02;
pub const IMX_SIP_TIMER_SET_WDOG_ACT: c_uint = 0x03;
pub const IMX_SIP_TIMER_PING_WDOG: c_uint = 0x04;
pub const IMX_SIP_TIMER_SET_TIMEOUT_WDOG: c_uint = 0x05;
pub const IMX_SIP_TIMER_GET_WDOG_STAT: c_uint = 0x06;
pub const IMX_SIP_TIMER_SET_PRETIME_WDOG: c_uint = 0x07;
pub const SC_TIMER_WDOG_ACTION_PARTITION: c_int = 0;
pub const SC_IRQ_WDOG: c_int = 1;
pub const SC_IRQ_GROUP_WDOG: c_int = 1;
pub const SC_TIMER_ERR_BUSY: c_int = 10;
    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0000);
    MODULE_PARM_DESC(nowayout, "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_sc_wdt_device {
    pub wdd: watchdog_device,
    pub wdt_notifier: notifier_block,
}

#[no_mangle]
unsafe extern "C" fn imx_sc_wdt_ping(wdog: *mut watchdog_device) -> c_int {
    static int imx_sc_wdt_ping(struct watchdog_device *wdog)
    {
    struct arm_smccc_res res;
    arm_smccc_smc(IMX_SIP_TIMER, IMX_SIP_TIMER_PING_WDOG,
    0, 0, 0, 0, 0, 0, &res);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx_sc_wdt_is_running() -> bool {
    static bool imx_sc_wdt_is_running(void)
    {
    struct arm_smccc_res res;
    arm_smccc_smc(IMX_SIP_TIMER, IMX_SIP_TIMER_START_WDOG,
    0, 0, 0, 0, 0, 0, &res);
// Already enabled (SC_TIMER_ERR_BUSY)?
    if (res.a0 == SC_TIMER_ERR_BUSY)
    return true;
// Undo only if that was us who has (successfully) enabled the WDT
    if (!res.a0)
    arm_smccc_smc(IMX_SIP_TIMER, IMX_SIP_TIMER_STOP_WDOG,
    0, 0, 0, 0, 0, 0, &res);
    return false;
    }
#[no_mangle]
unsafe extern "C" fn imx_sc_wdt_start(wdog: *mut watchdog_device) -> c_int {
    static int imx_sc_wdt_start(struct watchdog_device *wdog)
    {
    struct arm_smccc_res res;
    arm_smccc_smc(IMX_SIP_TIMER, IMX_SIP_TIMER_START_WDOG,
    0, 0, 0, 0, 0, 0, &res);
// Ignore if already enabled(SC_TIMER_ERR_BUSY)
    if (res.a0 && res.a0 != SC_TIMER_ERR_BUSY)
    return -EACCES;
    arm_smccc_smc(IMX_SIP_TIMER, IMX_SIP_TIMER_SET_WDOG_ACT,
    SC_TIMER_WDOG_ACTION_PARTITION,
    0, 0, 0, 0, 0, &res);
    return res.a0 ? -EACCES : 0;
    }
#[no_mangle]
unsafe extern "C" fn imx_sc_wdt_stop(wdog: *mut watchdog_device) -> c_int {
    static int imx_sc_wdt_stop(struct watchdog_device *wdog)
    {
    struct arm_smccc_res res;
    arm_smccc_smc(IMX_SIP_TIMER, IMX_SIP_TIMER_STOP_WDOG,
    0, 0, 0, 0, 0, 0, &res);
    return res.a0 ? -EACCES : 0;
    }
    static int imx_sc_wdt_set_timeout(struct watchdog_device *wdog,
    unsigned int timeout)
    {
    struct arm_smccc_res res;
    wdog.timeout = timeout;
    arm_smccc_smc(IMX_SIP_TIMER, IMX_SIP_TIMER_SET_TIMEOUT_WDOG,
    timeout * 1000, 0, 0, 0, 0, 0, &res);
    return res.a0 ? -EACCES : 0;
    }
    static int imx_sc_wdt_set_pretimeout(struct watchdog_device *wdog,
    unsigned int pretimeout)
    {
    struct arm_smccc_res res;
//
// SCU firmware calculates pretimeout based on current time
// stamp instead of watchdog timeout stamp, need to convert
// the pretimeout to SCU firmware's timeout value.
//
    arm_smccc_smc(IMX_SIP_TIMER, IMX_SIP_TIMER_SET_PRETIME_WDOG,
    (wdog.timeout - pretimeout) * 1000, 0, 0, 0,
    0, 0, &res);
    if (res.a0)
    return -EACCES;
    wdog.pretimeout = pretimeout;
    return 0;
    }
    static int imx_sc_wdt_notify(struct notifier_block *nb,
    unsigned long event, void *group)
    {
    struct imx_sc_wdt_device *imx_sc_wdd =
    container_of(nb,
    struct imx_sc_wdt_device,
    wdt_notifier);
    if (event & SC_IRQ_WDOG &&
// (u8 *)group == SC_IRQ_GROUP_WDOG)
    watchdog_notify_pretimeout(&imx_sc_wdd.wdd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx_sc_wdt_action(data: *mut c_void) {
    static void imx_sc_wdt_action(void *data)
    {
    struct notifier_block *wdt_notifier = data;
    imx_scu_irq_unregister_notifier(wdt_notifier);
    imx_scu_irq_group_enable(SC_IRQ_GROUP_WDOG,
    SC_IRQ_WDOG,
    false);
    }
    static const struct watchdog_ops imx_sc_wdt_ops = {
    .owner = THIS_MODULE,
    .start = imx_sc_wdt_start,
    .stop  = imx_sc_wdt_stop,
    .ping  = imx_sc_wdt_ping,
    .set_timeout = imx_sc_wdt_set_timeout,
    .set_pretimeout = imx_sc_wdt_set_pretimeout,
    };
    static struct watchdog_info imx_sc_wdt_info = {
    .identity	= "i.MX SC watchdog timer",
    .options	= WDIOF_SETTIMEOUT | WDIOF_KEEPALIVEPING |
    WDIOF_MAGICCLOSE,
    };
#[no_mangle]
unsafe extern "C" fn imx_sc_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int imx_sc_wdt_probe(struct platform_device *pdev)
    {
    struct imx_sc_wdt_device *imx_sc_wdd;
    struct watchdog_device *wdog;
    struct device *dev = &pdev.dev;
    int ret;
    imx_sc_wdd = devm_kzalloc(dev, sizeof(*imx_sc_wdd), GFP_KERNEL);
    if (!imx_sc_wdd)
    return -ENOMEM;
    platform_set_drvdata(pdev, imx_sc_wdd);
    wdog = &imx_sc_wdd.wdd;
    wdog.info = &imx_sc_wdt_info;
    wdog.ops = &imx_sc_wdt_ops;
    wdog.min_timeout = 1;
    wdog.max_timeout = MAX_TIMEOUT;
    wdog.parent = dev;
    wdog.timeout = DEFAULT_TIMEOUT;
    watchdog_init_timeout(wdog, 0, dev);
    ret = imx_sc_wdt_set_timeout(wdog, wdog.timeout);
    if (ret)
    return ret;
    if (imx_sc_wdt_is_running())
    set_bit(WDOG_HW_RUNNING, &wdog.status);
    watchdog_stop_on_reboot(wdog);
    watchdog_stop_on_unregister(wdog);
    ret = imx_scu_irq_group_enable(SC_IRQ_GROUP_WDOG,
    SC_IRQ_WDOG,
    true);
    if (ret) {
    dev_warn(dev, "Enable irq failed, pretimeout NOT supported\n");
    goto register_device;
    }
    imx_sc_wdd.wdt_notifier.notifier_call = imx_sc_wdt_notify;
    ret = imx_scu_irq_register_notifier(&imx_sc_wdd.wdt_notifier);
    if (ret) {
    imx_scu_irq_group_enable(SC_IRQ_GROUP_WDOG,
    SC_IRQ_WDOG,
    false);
    dev_warn(dev,
    "Register irq notifier failed, pretimeout NOT supported\n");
    goto register_device;
    }
    ret = devm_add_action_or_reset(dev, imx_sc_wdt_action,
    &imx_sc_wdd.wdt_notifier);
    if (!ret)
    imx_sc_wdt_info.options |= WDIOF_PRETIMEOUT;
    else
    dev_warn(dev, "Add action failed, pretimeout NOT supported\n");
    register_device:
    return devm_watchdog_register_device(dev, wdog);
    }
    static const struct of_device_id imx_sc_wdt_dt_ids[] = {
    { .compatible = "fsl,imx-sc-wdt", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, imx_sc_wdt_dt_ids);
    static struct platform_driver imx_sc_wdt_driver = {
    .probe		= imx_sc_wdt_probe,
    .driver		= {
    .name	= "imx-sc-wdt",
    .of_match_table = imx_sc_wdt_dt_ids,
    },
    };
    module_platform_driver(imx_sc_wdt_driver);
    MODULE_AUTHOR("Robin Gong <yibin.gong@nxp.com>");
    MODULE_DESCRIPTION("NXP i.MX system controller watchdog driver");
    MODULE_LICENSE("GPL v2");
