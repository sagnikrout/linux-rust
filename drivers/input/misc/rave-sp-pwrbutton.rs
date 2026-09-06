//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/rave-sp-pwrbutton.c
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
// Power Button driver for RAVE SP
//
// Copyright (C) 2017 Zodiac Inflight Innovations
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rave_sp_power_button {
    pub idev: *mut input_dev,
    pub nb: notifier_block,
}

    static int rave_sp_power_button_event(struct notifier_block *nb,
    unsigned long action, void *data)
    {
    struct rave_sp_power_button *pb =
    container_of(nb, struct rave_sp_power_button, nb);
    let mut event: u8 = rave_sp_action_unpack_event(action);
    let mut value: u8 = rave_sp_action_unpack_value(action);
    struct input_dev *idev = pb.idev;
    if (event == RAVE_SP_EVNT_BUTTON_PRESS) {
    input_report_key(idev, KEY_POWER, value);
    input_sync(idev);
    return NOTIFY_STOP;
    }
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn rave_sp_pwrbutton_probe(pdev: *mut platform_device) -> c_int {
    static int rave_sp_pwrbutton_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct rave_sp_power_button *pb;
    struct input_dev *idev;
    int error;
    pb = devm_kzalloc(dev, sizeof(*pb), GFP_KERNEL);
    if (!pb)
    return -ENOMEM;
    idev = devm_input_allocate_device(dev);
    if (!idev)
    return -ENOMEM;
    idev.name = pdev.name;
    input_set_capability(idev, EV_KEY, KEY_POWER);
    error = input_register_device(idev);
    if (error)
    return error;
    pb.idev = idev;
    pb.nb.notifier_call = rave_sp_power_button_event;
    pb.nb.priority = 128;
    error = devm_rave_sp_register_event_notifier(dev, &pb.nb);
    if (error)
    return error;
    return 0;
    }
    static const struct of_device_id rave_sp_pwrbutton_of_match[] = {
    { .compatible = "zii,rave-sp-pwrbutton" },
    {}
    };
    static struct platform_driver rave_sp_pwrbutton_driver = {
    .probe = rave_sp_pwrbutton_probe,
    .driver	= {
    .name = KBUILD_MODNAME,
    .of_match_table = rave_sp_pwrbutton_of_match,
    },
    };
    module_platform_driver(rave_sp_pwrbutton_driver);
    MODULE_DEVICE_TABLE(of, rave_sp_pwrbutton_of_match);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Andrey Vostrikov <andrey.vostrikov@cogentembedded.com>");
    MODULE_AUTHOR("Nikita Yushchenko <nikita.yoush@cogentembedded.com>");
    MODULE_AUTHOR("Andrey Smirnov <andrew.smirnov@gmail.com>");
    MODULE_DESCRIPTION("RAVE SP Power Button driver");
