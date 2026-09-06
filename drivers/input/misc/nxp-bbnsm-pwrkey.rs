//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/nxp-bbnsm-pwrkey.c
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
// Copyright 2022 NXP.

pub const BBNSM_CTRL: c_uint = 0x8;
pub const BBNSM_INT_EN: c_uint = 0x10;
pub const BBNSM_EVENTS: c_uint = 0x14;
pub const BBNSM_PAD_CTRL: c_uint = 0x24;

pub const DEBOUNCE_TIME: c_int = 30;
pub const REPEAT_INTERVAL: c_int = 60;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bbnsm_pwrkey {
    pub regmap: *mut regmap,
    pub irq: c_int,
    pub keycode: c_int,
    pub /: *mut *mut int keystate; / 1:pressed,
    pub suspended: bool,
    pub check_timer: timer_list,
    pub input: *mut input_dev,
}

#[no_mangle]
unsafe extern "C" fn bbnsm_pwrkey_check_for_events(t: *mut timer_list) {
    static void bbnsm_pwrkey_check_for_events(struct timer_list *t)
    {
    struct bbnsm_pwrkey *bbnsm = timer_container_of(bbnsm, t, check_timer);
    struct input_dev *input = bbnsm.input;
    u32 state;
    regmap_read(bbnsm.regmap, BBNSM_EVENTS, &state);
    state = state & BBNSM_BTN_PRESSED ? 1 : 0;
// only report new event if status changed
    if (state ^ bbnsm.keystate) {
    bbnsm.keystate = state;
    input_event(input, EV_KEY, bbnsm.keycode, state);
    input_sync(input);
    pm_relax(bbnsm.input.dev.parent);
    }
// repeat check if pressed long
    if (state)
    mod_timer(&bbnsm.check_timer,
    jiffies + msecs_to_jiffies(REPEAT_INTERVAL));
    }
#[no_mangle]
unsafe extern "C" fn bbnsm_pwrkey_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t bbnsm_pwrkey_interrupt(int irq, void *dev_id)
    {
    struct platform_device *pdev = dev_id;
    struct bbnsm_pwrkey *bbnsm = platform_get_drvdata(pdev);
    struct input_dev *input = bbnsm.input;
    u32 event;
    regmap_read(bbnsm.regmap, BBNSM_EVENTS, &event);
    if (!(event & BBNSM_BTN_OFF))
    return IRQ_NONE;
    pm_wakeup_event(bbnsm.input.dev.parent, 0);
//
// Directly report key event after resume to make sure key press
// event is never missed.
//
    if (bbnsm.suspended) {
    bbnsm.keystate = 1;
    input_event(input, EV_KEY, bbnsm.keycode, 1);
    input_sync(input);
// Fire at most once per suspend/resume cycle
    bbnsm.suspended = false;
    }
    mod_timer(&bbnsm.check_timer,
    jiffies + msecs_to_jiffies(DEBOUNCE_TIME));
// clear PWR OFF
    regmap_write(bbnsm.regmap, BBNSM_EVENTS, BBNSM_BTN_OFF);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn bbnsm_pwrkey_act(pdata: *mut c_void) {
    static void bbnsm_pwrkey_act(void *pdata)
    {
    struct bbnsm_pwrkey *bbnsm = pdata;
    timer_shutdown_sync(&bbnsm.check_timer);
    }
#[no_mangle]
unsafe extern "C" fn bbnsm_pwrkey_probe(pdev: *mut platform_device) -> c_int {
    static int bbnsm_pwrkey_probe(struct platform_device *pdev)
    {
    struct bbnsm_pwrkey *bbnsm;
    struct input_dev *input;
    struct device_node *np = pdev.dev.of_node;
    int error;
    bbnsm = devm_kzalloc(&pdev.dev, sizeof(*bbnsm), GFP_KERNEL);
    if (!bbnsm)
    return -ENOMEM;
    bbnsm.regmap = syscon_node_to_regmap(np.parent);
    if (IS_ERR(bbnsm.regmap)) {
    dev_err(&pdev.dev, "bbnsm pwerkey get regmap failed\n");
    return PTR_ERR(bbnsm.regmap);
    }
    if (device_property_read_u32(&pdev.dev, "linux,code",
    &bbnsm.keycode)) {
    bbnsm.keycode = KEY_POWER;
    dev_warn(&pdev.dev, "key code is not specified, using default KEY_POWER\n");
    }
    bbnsm.irq = platform_get_irq(pdev, 0);
    if (bbnsm.irq < 0)
    return -EINVAL;
// config the BBNSM power related register
    regmap_update_bits(bbnsm.regmap, BBNSM_CTRL, BBNSM_DP_EN, BBNSM_DP_EN);
// clear the unexpected interrupt before driver ready
    regmap_write_bits(bbnsm.regmap, BBNSM_EVENTS, BBNSM_PWRKEY_EVENTS,
    BBNSM_PWRKEY_EVENTS);
    timer_setup(&bbnsm.check_timer, bbnsm_pwrkey_check_for_events, 0);
    input = devm_input_allocate_device(&pdev.dev);
    if (!input) {
    dev_err(&pdev.dev, "failed to allocate the input device\n");
    return -ENOMEM;
    }
    input.name = pdev.name;
    input.phys = "bbnsm-pwrkey/input0";
    input.id.bustype = BUS_HOST;
    input_set_capability(input, EV_KEY, bbnsm.keycode);
// input customer action to cancel release timer
    error = devm_add_action(&pdev.dev, bbnsm_pwrkey_act, bbnsm);
    if (error) {
    dev_err(&pdev.dev, "failed to register remove action\n");
    return error;
    }
    bbnsm.input = input;
    platform_set_drvdata(pdev, bbnsm);
    error = devm_request_irq(&pdev.dev, bbnsm.irq, bbnsm_pwrkey_interrupt,
    IRQF_SHARED, pdev.name, pdev);
    if (error) {
    dev_err(&pdev.dev, "interrupt not available.\n");
    return error;
    }
    error = input_register_device(input);
    if (error) {
    dev_err(&pdev.dev, "failed to register input device\n");
    return error;
    }
    device_init_wakeup(&pdev.dev, true);
    error = dev_pm_set_wake_irq(&pdev.dev, bbnsm.irq);
    if (error)
    dev_warn(&pdev.dev, "irq wake enable failed.\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bbnsm_pwrkey_remove(pdev: *mut platform_device) {
    static void bbnsm_pwrkey_remove(struct platform_device *pdev)
    {
    dev_pm_clear_wake_irq(&pdev.dev);
    device_init_wakeup(&pdev.dev, false);
    }
#[no_mangle]
unsafe extern "C" fn bbnsm_pwrkey_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused bbnsm_pwrkey_suspend(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct bbnsm_pwrkey *bbnsm = platform_get_drvdata(pdev);
    bbnsm.suspended = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bbnsm_pwrkey_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused bbnsm_pwrkey_resume(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct bbnsm_pwrkey *bbnsm = platform_get_drvdata(pdev);
    bbnsm.suspended = false;
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(bbnsm_pwrkey_pm_ops, bbnsm_pwrkey_suspend,
    bbnsm_pwrkey_resume);
    static const struct of_device_id bbnsm_pwrkey_ids[] = {
    { .compatible = "nxp,imx93-bbnsm-pwrkey" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, bbnsm_pwrkey_ids);
    static struct platform_driver bbnsm_pwrkey_driver = {
    .driver = {
    .name = "bbnsm_pwrkey",
    .pm = &bbnsm_pwrkey_pm_ops,
    .of_match_table = bbnsm_pwrkey_ids,
    },
    .probe = bbnsm_pwrkey_probe,
    .remove = bbnsm_pwrkey_remove,
    };
    module_platform_driver(bbnsm_pwrkey_driver);
    MODULE_AUTHOR("Jacky Bai <ping.bai@nxp.com>");
    MODULE_DESCRIPTION("NXP bbnsm power key Driver");
    MODULE_LICENSE("GPL");
