//! Automatically rewritten from C to Rust
//! Source: drivers/input/keyboard/snvs_pwrkey.c
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
// Driver for the IMX SNVS ON/OFF Power Key
// Copyright (C) 2015 Freescale Semiconductor, Inc. All Rights Reserved.

pub const SNVS_HPVIDR1_REG: c_uint = 0xBF8;
pub const SNVS_LPSR_REG: c_uint = 0x4C	/* LP Status Register */;
pub const SNVS_LPCR_REG: c_uint = 0x38	/* LP Control Register */;
pub const SNVS_HPSR_REG: c_uint = 0x14;

pub const SNVS_LPCR_BPT_SHIFT: c_int = 16;

pub const DEBOUNCE_TIME: c_int = 30;
pub const REPEAT_INTERVAL: c_int = 60;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pwrkey_drv_data {
    pub snvs: *mut regmap,
    pub irq: c_int,
    pub keycode: c_int,
    pub /: *mut *mut int keystate; / 1:pressed,
    pub wakeup: c_int,
    pub /: *mut *mut bool suspended; / Track suspend state,
    pub /: *mut *mut bool pending_press; / Key pressed during suspend, report from timer callback,
    pub /: *mut *mut spinlock_t lock; / Protects keystate, suspended and pending_press,
    pub check_timer: timer_list,
    pub input: *mut input_dev,
    pub minor_rev: u8,
}

#[no_mangle]
unsafe extern "C" fn imx_imx_snvs_check_for_events(t: *mut timer_list) {
    static void imx_imx_snvs_check_for_events(struct timer_list *t)
    {
    struct pwrkey_drv_data *pdata = timer_container_of(pdata, t,
    check_timer);
    struct input_dev *input = pdata.input;
    let mut state_changed: bool = false;
    bool pending_press;
    u32 state;
    regmap_read(pdata.snvs, SNVS_HPSR_REG, &state);
    state = state & SNVS_HPSR_BTN ? 1 : 0;
    scoped_guard(spinlock_irqsave, &pdata.lock) {
    pending_press = pdata.pending_press;
    if (pending_press) {
    pdata.pending_press = false;
    pdata.keystate = 1;
    }
// only report new event if status changed
    if (state ^ pdata.keystate) {
    pdata.keystate = state;
    state_changed = true;
    }
    }
//
// Report a press event latched during suspend. If the key is still
// held, state_changed will be 0 (keystate already set to 1 above),
// so no duplicate press is reported. If already released,
// state_changed will fire next to report the release.
//
    if (pending_press) {
    input_report_key(input, pdata.keycode, 1);
    input_sync(input);
    }
    if (state_changed) {
    input_event(input, EV_KEY, pdata.keycode, state);
    input_sync(input);
    pm_relax(pdata.input.dev.parent);
    }
// repeat check if pressed long
    if (state) {
    mod_timer(&pdata.check_timer,
    jiffies + msecs_to_jiffies(REPEAT_INTERVAL));
    }
    }
#[no_mangle]
unsafe extern "C" fn imx_snvs_pwrkey_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t imx_snvs_pwrkey_interrupt(int irq, void *dev_id)
    {
    struct platform_device *pdev = dev_id;
    struct pwrkey_drv_data *pdata = platform_get_drvdata(pdev);
    struct input_dev *input = pdata.input;
    u32 lp_status;
    pm_wakeup_event(input.dev.parent, 0);
    regmap_read(pdata.snvs, SNVS_LPSR_REG, &lp_status);
    if (lp_status & SNVS_LPSR_SPO) {
    if (pdata.minor_rev == 0) {
//
// The first generation i.MX6 SoCs only sends an
// interrupt on button release. To mimic power-key
// usage, we'll prepend a press event.
//
    input_report_key(input, pdata.keycode, 1);
    input_sync(input);
    input_report_key(input, pdata.keycode, 0);
    input_sync(input);
    pm_relax(input.dev.parent);
    } else {
//
// If the key is pressed during suspend, latch it so
// the timer callback can report the press event in
// softirq context, avoiding out-of-order events.
//
    scoped_guard(spinlock_irqsave, &pdata.lock) {
    if (pdata.suspended)
    pdata.pending_press = true;
    }
    mod_timer(&pdata.check_timer,
    jiffies + msecs_to_jiffies(DEBOUNCE_TIME));
    }
    }
// clear SPO status
    regmap_write(pdata.snvs, SNVS_LPSR_REG, SNVS_LPSR_SPO);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn imx_snvs_pwrkey_act(pdata: *mut c_void) {
    static void imx_snvs_pwrkey_act(void *pdata)
    {
    struct pwrkey_drv_data *pd = pdata;
    timer_delete_sync(&pd.check_timer);
    }
#[no_mangle]
unsafe extern "C" fn imx_snvs_pwrkey_probe(pdev: *mut platform_device) -> c_int {
    static int imx_snvs_pwrkey_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct pwrkey_drv_data *pdata;
    struct input_dev *input;
    struct device_node *np;
    struct clk *clk;
    int error;
    unsigned int val;
    unsigned int bpt;
    u32 vid;
// Get SNVS register Page
    np = dev.of_node;
    if (!np)
    return dev_err_probe(dev, -ENODEV, "Device tree node not found\n");
    pdata = devm_kzalloc(dev, sizeof(*pdata), GFP_KERNEL);
    if (!pdata)
    return -ENOMEM;
    pdata.snvs = syscon_regmap_lookup_by_phandle(np, "regmap");
    if (IS_ERR(pdata.snvs))
    return dev_err_probe(dev, PTR_ERR(pdata.snvs), "Can't get snvs syscon\n");
    if (of_property_read_u32(np, "linux,keycode", &pdata.keycode)) {
    pdata.keycode = KEY_POWER;
    dev_warn(dev, "KEY_POWER without setting in dts\n");
    }
    clk = devm_clk_get_optional_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(clk))
    return dev_err_probe(dev, PTR_ERR(clk),
    "Failed to get snvs clock (%pe)\n", clk);
    pdata.wakeup = of_property_read_bool(np, "wakeup-source");
    pdata.irq = platform_get_irq(pdev, 0);
    if (pdata.irq < 0)
    return pdata.irq;
    spin_lock_init(&pdata.lock);
    error = of_property_read_u32(np, "power-off-time-sec", &val);
    if (!error) {
    switch (val) {
    case 0:
    bpt = 0x3;
    break;
    case 5:
    case 10:
    case 15:
    bpt = (val / 5) - 1;
    break;
    default:
    return dev_err_probe(dev, -EINVAL,
    "power-off-time-sec %d out of range\n", val);
    }
    regmap_update_bits(pdata.snvs, SNVS_LPCR_REG, SNVS_LPCR_BPT_MASK,
    bpt << SNVS_LPCR_BPT_SHIFT);
    }
    regmap_read(pdata.snvs, SNVS_HPVIDR1_REG, &vid);
    pdata.minor_rev = vid & 0xff;
    regmap_update_bits(pdata.snvs, SNVS_LPCR_REG, SNVS_LPCR_DEP_EN, SNVS_LPCR_DEP_EN);
// clear the unexpected interrupt before driver ready
    regmap_write(pdata.snvs, SNVS_LPSR_REG, SNVS_LPSR_SPO);
    timer_setup(&pdata.check_timer, imx_imx_snvs_check_for_events, 0);
    input = devm_input_allocate_device(dev);
    if (!input)
    return dev_err_probe(dev, -ENOMEM, "failed to allocate the input device\n");
    input.name = pdev.name;
    input.phys = "snvs-pwrkey/input0";
    input.id.bustype = BUS_HOST;
    input_set_capability(input, EV_KEY, pdata.keycode);
// input customer action to cancel release timer
    error = devm_add_action(dev, imx_snvs_pwrkey_act, pdata);
    if (error)
    return dev_err_probe(dev, error, "failed to register remove action\n");
    pdata.input = input;
    platform_set_drvdata(pdev, pdata);
    error = devm_request_irq(dev, pdata.irq,
    imx_snvs_pwrkey_interrupt,
    0, pdev.name, pdev);
    if (error)
    return dev_err_probe(dev, error, "interrupt not available.\n");
    error = input_register_device(input);
    if (error < 0)
    return dev_err_probe(dev, error, "failed to register input device\n");
    device_init_wakeup(dev, pdata.wakeup);
    error = dev_pm_set_wake_irq(dev, pdata.irq);
    if (error)
    dev_err(dev, "irq wake enable failed.\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx_snvs_pwrkey_suspend(dev: *mut device) -> c_int {
    static int imx_snvs_pwrkey_suspend(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct pwrkey_drv_data *pdata = platform_get_drvdata(pdev);
    guard(spinlock_irq)(&pdata.lock);
    pdata.suspended = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx_snvs_pwrkey_resume(dev: *mut device) -> c_int {
    static int imx_snvs_pwrkey_resume(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct pwrkey_drv_data *pdata = platform_get_drvdata(pdev);
    guard(spinlock_irq)(&pdata.lock);
    pdata.suspended = false;
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(imx_snvs_pwrkey_pm_ops,
    imx_snvs_pwrkey_suspend,
    imx_snvs_pwrkey_resume);
    static const struct of_device_id imx_snvs_pwrkey_ids[] = {
    { .compatible = "fsl,sec-v4.0-pwrkey" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, imx_snvs_pwrkey_ids);
    static struct platform_driver imx_snvs_pwrkey_driver = {
    .driver = {
    .name = "snvs_pwrkey",
    .of_match_table = imx_snvs_pwrkey_ids,
    .pm = pm_ptr(&imx_snvs_pwrkey_pm_ops),
    },
    .probe = imx_snvs_pwrkey_probe,
    };
    module_platform_driver(imx_snvs_pwrkey_driver);
    MODULE_AUTHOR("Freescale Semiconductor");
    MODULE_DESCRIPTION("i.MX snvs power key Driver");
    MODULE_LICENSE("GPL");
