//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/atc260x-onkey.c
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
// Onkey driver for Actions Semi ATC260x PMICs.
//
// Copyright (c) 2020 Cristian Ciocaltea <cristian.ciocaltea@gmail.com>
//

// <2s for short press, >2s for long press
pub const KEY_PRESS_TIME_SEC: c_int = 2;
// Driver internals
    enum atc260x_onkey_reset_status {
    KEY_RESET_HW_DEFAULT,
    KEY_RESET_DISABLED,
    KEY_RESET_USER_SEL,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atc260x_onkey_params {
    pub reg_int_ctl: u32,
    pub kdwn_state_bm: u32,
    pub long_int_pnd_bm: u32,
    pub short_int_pnd_bm: u32,
    pub kdwn_int_pnd_bm: u32,
    pub press_int_en_bm: u32,
    pub kdwn_int_en_bm: u32,
    pub press_time_bm: u32,
    pub reset_en_bm: u32,
    pub reset_time_bm: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atc260x_onkey {
    pub atc260x: *mut atc260x,
    pub params: *const atc260x_onkey_params,
    pub input_dev: *mut input_dev,
    pub work: delayed_work,
    pub irq: c_int,
}

    static const struct atc260x_onkey_params atc2603c_onkey_params = {
    .reg_int_ctl		= ATC2603C_PMU_SYS_CTL2,
    .long_int_pnd_bm	= ATC2603C_PMU_SYS_CTL2_ONOFF_LONG_PRESS,
    .short_int_pnd_bm	= ATC2603C_PMU_SYS_CTL2_ONOFF_SHORT_PRESS,
    .kdwn_int_pnd_bm	= ATC2603C_PMU_SYS_CTL2_ONOFF_PRESS_PD,
    .press_int_en_bm	= ATC2603C_PMU_SYS_CTL2_ONOFF_INT_EN,
    .kdwn_int_en_bm		= ATC2603C_PMU_SYS_CTL2_ONOFF_PRESS_INT_EN,
    .kdwn_state_bm		= ATC2603C_PMU_SYS_CTL2_ONOFF_PRESS,
    .press_time_bm		= ATC2603C_PMU_SYS_CTL2_ONOFF_PRESS_TIME,
    .reset_en_bm		= ATC2603C_PMU_SYS_CTL2_ONOFF_PRESS_RESET_EN,
    .reset_time_bm		= ATC2603C_PMU_SYS_CTL2_ONOFF_RESET_TIME_SEL,
    };
    static const struct atc260x_onkey_params atc2609a_onkey_params = {
    .reg_int_ctl		= ATC2609A_PMU_SYS_CTL2,
    .long_int_pnd_bm	= ATC2609A_PMU_SYS_CTL2_ONOFF_LONG_PRESS,
    .short_int_pnd_bm	= ATC2609A_PMU_SYS_CTL2_ONOFF_SHORT_PRESS,
    .kdwn_int_pnd_bm	= ATC2609A_PMU_SYS_CTL2_ONOFF_PRESS_PD,
    .press_int_en_bm	= ATC2609A_PMU_SYS_CTL2_ONOFF_LSP_INT_EN,
    .kdwn_int_en_bm		= ATC2609A_PMU_SYS_CTL2_ONOFF_PRESS_INT_EN,
    .kdwn_state_bm		= ATC2609A_PMU_SYS_CTL2_ONOFF_PRESS,
    .press_time_bm		= ATC2609A_PMU_SYS_CTL2_ONOFF_PRESS_TIME,
    .reset_en_bm		= ATC2609A_PMU_SYS_CTL2_ONOFF_RESET_EN,
    .reset_time_bm		= ATC2609A_PMU_SYS_CTL2_ONOFF_RESET_TIME_SEL,
    };
    static int atc2603x_onkey_hw_init(struct atc260x_onkey *onkey,
    enum atc260x_onkey_reset_status reset_status,
    u32 reset_time, u32 press_time)
    {
    u32 reg_bm, reg_val;
    reg_bm = onkey.params.long_int_pnd_bm |
    onkey.params.short_int_pnd_bm |
    onkey.params.kdwn_int_pnd_bm |
    onkey.params.press_int_en_bm |
    onkey.params.kdwn_int_en_bm;
    reg_val = reg_bm | press_time;
    reg_bm |= onkey.params.press_time_bm;
    if (reset_status == KEY_RESET_DISABLED) {
    reg_bm |= onkey.params.reset_en_bm;
    } else if (reset_status == KEY_RESET_USER_SEL) {
    reg_bm |= onkey.params.reset_en_bm |
    onkey.params.reset_time_bm;
    reg_val |= onkey.params.reset_en_bm | reset_time;
    }
    return regmap_update_bits(onkey.atc260x.regmap,
    onkey.params.reg_int_ctl, reg_bm, reg_val);
    }
#[no_mangle]
unsafe extern "C" fn atc260x_onkey_query(onkey: *mut atc260x_onkey) {
    static void atc260x_onkey_query(struct atc260x_onkey *onkey)
    {
    u32 reg_bits;
    int ret, key_down;
    ret = regmap_read(onkey.atc260x.regmap,
    onkey.params.reg_int_ctl, &key_down);
    if (ret) {
    key_down = 1;
    dev_err(onkey.atc260x.dev,
    "Failed to read onkey status: %d\n", ret);
    } else {
    key_down &= onkey.params.kdwn_state_bm;
    }
//
// The hardware generates interrupt only when the onkey pin is
// asserted. Hence, the deassertion of the pin is simulated through
// work queue.
//
    if (key_down) {
    schedule_delayed_work(&onkey.work, msecs_to_jiffies(200));
    return;
    }
//
// The key-down status bit is cleared when the On/Off button
// is released.
//
    input_report_key(onkey.input_dev, KEY_POWER, 0);
    input_sync(onkey.input_dev);
    reg_bits = onkey.params.long_int_pnd_bm |
    onkey.params.short_int_pnd_bm |
    onkey.params.kdwn_int_pnd_bm |
    onkey.params.press_int_en_bm |
    onkey.params.kdwn_int_en_bm;
// Clear key press pending events and enable key press interrupts.
    regmap_update_bits(onkey.atc260x.regmap, onkey.params.reg_int_ctl,
    reg_bits, reg_bits);
    }
#[no_mangle]
unsafe extern "C" fn atc260x_onkey_work(work: *mut work_struct) {
    static void atc260x_onkey_work(struct work_struct *work)
    {
    struct atc260x_onkey *onkey = container_of(work, struct atc260x_onkey,
    work.work);
    atc260x_onkey_query(onkey);
    }
#[no_mangle]
unsafe extern "C" fn atc260x_onkey_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t atc260x_onkey_irq(int irq, void *data)
    {
    struct atc260x_onkey *onkey = data;
    int ret;
// Disable key press interrupts.
    ret = regmap_update_bits(onkey.atc260x.regmap,
    onkey.params.reg_int_ctl,
    onkey.params.press_int_en_bm |
    onkey.params.kdwn_int_en_bm, 0);
    if (ret)
    dev_err(onkey.atc260x.dev,
    "Failed to disable interrupts: %d\n", ret);
    input_report_key(onkey.input_dev, KEY_POWER, 1);
    input_sync(onkey.input_dev);
    atc260x_onkey_query(onkey);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn atc260x_onkey_open(dev: *mut input_dev) -> c_int {
    static int atc260x_onkey_open(struct input_dev *dev)
    {
    struct atc260x_onkey *onkey = input_get_drvdata(dev);
    enable_irq(onkey.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn atc260x_onkey_close(dev: *mut input_dev) {
    static void atc260x_onkey_close(struct input_dev *dev)
    {
    struct atc260x_onkey *onkey = input_get_drvdata(dev);
    disable_irq(onkey.irq);
    cancel_delayed_work_sync(&onkey.work);
    }
#[no_mangle]
unsafe extern "C" fn atc260x_onkey_probe(pdev: *mut platform_device) -> c_int {
    static int atc260x_onkey_probe(struct platform_device *pdev)
    {
    struct atc260x *atc260x = dev_get_drvdata(pdev.dev.parent);
    struct atc260x_onkey *onkey;
    struct input_dev *input_dev;
    enum atc260x_onkey_reset_status reset_status;
    let mut press_time: u32 = KEY_PRESS_TIME_SEC, reset_time = 0;
    int val, error;
    onkey = devm_kzalloc(&pdev.dev, sizeof(*onkey), GFP_KERNEL);
    if (!onkey)
    return -ENOMEM;
    error = device_property_read_u32(pdev.dev.parent,
    "reset-time-sec", &val);
    if (error) {
    reset_status = KEY_RESET_HW_DEFAULT;
    } else if (val) {
    if (val < 6 || val > 12) {
    dev_err(&pdev.dev, "reset-time-sec out of range\n");
    return -EINVAL;
    }
    reset_status = KEY_RESET_USER_SEL;
    reset_time = (val - 6) / 2;
    } else {
    reset_status = KEY_RESET_DISABLED;
    dev_dbg(&pdev.dev, "Disabled reset on long-press\n");
    }
    switch (atc260x.ic_type) {
    case ATC2603C:
    onkey.params = &atc2603c_onkey_params;
    press_time = FIELD_PREP(ATC2603C_PMU_SYS_CTL2_ONOFF_PRESS_TIME,
    press_time);
    reset_time = FIELD_PREP(ATC2603C_PMU_SYS_CTL2_ONOFF_RESET_TIME_SEL,
    reset_time);
    break;
    case ATC2609A:
    onkey.params = &atc2609a_onkey_params;
    press_time = FIELD_PREP(ATC2609A_PMU_SYS_CTL2_ONOFF_PRESS_TIME,
    press_time);
    reset_time = FIELD_PREP(ATC2609A_PMU_SYS_CTL2_ONOFF_RESET_TIME_SEL,
    reset_time);
    break;
    default:
    dev_err(&pdev.dev,
    "OnKey not supported for ATC260x PMIC type: %u\n",
    atc260x.ic_type);
    return -EINVAL;
    }
    input_dev = devm_input_allocate_device(&pdev.dev);
    if (!input_dev) {
    dev_err(&pdev.dev, "Failed to allocate input device\n");
    return -ENOMEM;
    }
    onkey.input_dev = input_dev;
    onkey.atc260x = atc260x;
    input_dev.name = "atc260x-onkey";
    input_dev.phys = "atc260x-onkey/input0";
    input_dev.open = atc260x_onkey_open;
    input_dev.close = atc260x_onkey_close;
    input_set_capability(input_dev, EV_KEY, KEY_POWER);
    input_set_drvdata(input_dev, onkey);
    INIT_DELAYED_WORK(&onkey.work, atc260x_onkey_work);
    onkey.irq = platform_get_irq(pdev, 0);
    if (onkey.irq < 0)
    return onkey.irq;
    error = devm_request_threaded_irq(&pdev.dev, onkey.irq, core::ptr::null_mut(),
    atc260x_onkey_irq, IRQF_ONESHOT,
    dev_name(&pdev.dev), onkey);
    if (error) {
    dev_err(&pdev.dev,
    "Failed to register IRQ %d: %d\n", onkey.irq, error);
    return error;
    }
// Keep IRQ disabled until atc260x_onkey_open() is called.
    disable_irq(onkey.irq);
    error = input_register_device(input_dev);
    if (error) {
    dev_err(&pdev.dev,
    "Failed to register input device: %d\n", error);
    return error;
    }
    error = atc2603x_onkey_hw_init(onkey, reset_status,
    reset_time, press_time);
    if (error)
    return error;
    device_init_wakeup(&pdev.dev, true);
    return 0;
    }
    static struct platform_driver atc260x_onkey_driver = {
    .probe	= atc260x_onkey_probe,
    .driver	= {
    .name = "atc260x-onkey",
    },
    };
    module_platform_driver(atc260x_onkey_driver);
    MODULE_DESCRIPTION("Onkey driver for ATC260x PMICs");
    MODULE_AUTHOR("Cristian Ciocaltea <cristian.ciocaltea@gmail.com>");
    MODULE_LICENSE("GPL");
