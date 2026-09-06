//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/qnap-mcu-input.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Driver for input events on QNAP-MCUs
//
// Copyright (C) 2024 Heiko Stuebner <heiko@sntech.de>
//

//
// The power-key needs to be pressed for a while to create an event,
// so there is no use for overly frequent polling.
//
pub const POLL_INTERVAL: c_int = 500;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qnap_mcu_input_dev {
    pub input: *mut input_dev,
    pub mcu: *mut qnap_mcu,
    pub dev: *mut device,
    pub beep_work: work_struct,
    pub beep_type: c_int,
}

#[no_mangle]
unsafe extern "C" fn qnap_mcu_input_poll(input: *mut input_dev) {
    static void qnap_mcu_input_poll(struct input_dev *input)
    {
    struct qnap_mcu_input_dev *idev = input_get_drvdata(input);
    static const u8 cmd[] = { '@', 'C', 'V' };
    u8 reply[4];
    int state, ret;
// poll the power button
    ret = qnap_mcu_exec(idev.mcu, cmd, sizeof(cmd), reply, sizeof(reply));
    if (ret)
    return;
// First bytes must mirror the sent command
    if (memcmp(cmd, reply, sizeof(cmd))) {
    dev_err(idev.dev, "malformed data received\n");
    return;
    }
    state = reply[3] - 0x30;
    input_event(input, EV_KEY, KEY_POWER, state);
    input_sync(input);
    }
#[no_mangle]
unsafe extern "C" fn qnap_mcu_input_beeper_work(work: *mut work_struct) {
    static void qnap_mcu_input_beeper_work(struct work_struct *work)
    {
    struct qnap_mcu_input_dev *idev =
    container_of(work, struct qnap_mcu_input_dev, beep_work);
    const u8 cmd[] = { '@', 'C', (idev.beep_type == SND_TONE) ? '3' : '2' };
    qnap_mcu_exec_with_ack(idev.mcu, cmd, sizeof(cmd));
    }
    static int qnap_mcu_input_event(struct input_dev *input, unsigned int type,
    unsigned int code, int value)
    {
    struct qnap_mcu_input_dev *idev = input_get_drvdata(input);
    if (type != EV_SND || (code != SND_BELL && code != SND_TONE))
    return -EOPNOTSUPP;
    if (value < 0)
    return -EINVAL;
// beep runtime is determined by the MCU
    if (value == 0)
    return 0;
// Schedule work to actually turn the beeper on
    idev.beep_type = code;
    schedule_work(&idev.beep_work);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qnap_mcu_input_close(input: *mut input_dev) {
    static void qnap_mcu_input_close(struct input_dev *input)
    {
    struct qnap_mcu_input_dev *idev = input_get_drvdata(input);
    cancel_work_sync(&idev.beep_work);
    }
#[no_mangle]
unsafe extern "C" fn qnap_mcu_input_probe(pdev: *mut platform_device) -> c_int {
    static int qnap_mcu_input_probe(struct platform_device *pdev)
    {
    struct qnap_mcu *mcu = dev_get_drvdata(pdev.dev.parent);
    struct qnap_mcu_input_dev *idev;
    struct device *dev = &pdev.dev;
    struct input_dev *input;
    int ret;
    idev = devm_kzalloc(dev, sizeof(*idev), GFP_KERNEL);
    if (!idev)
    return -ENOMEM;
    input = devm_input_allocate_device(dev);
    if (!input)
    return -ENOMEM;
    idev.input = input;
    idev.dev = dev;
    idev.mcu = mcu;
    input_set_drvdata(input, idev);
    input.name		= "qnap-mcu";
    input.phys		= "qnap-mcu-input/input0";
    input.id.bustype	= BUS_HOST;
    input.id.vendor	= 0x0001;
    input.id.product	= 0x0001;
    input.id.version	= 0x0100;
    input.event		= qnap_mcu_input_event;
    input.close		= qnap_mcu_input_close;
    input_set_capability(input, EV_KEY, KEY_POWER);
    input_set_capability(input, EV_SND, SND_BELL);
    input_set_capability(input, EV_SND, SND_TONE);
    INIT_WORK(&idev.beep_work, qnap_mcu_input_beeper_work);
    ret = input_setup_polling(input, qnap_mcu_input_poll);
    if (ret)
    return dev_err_probe(dev, ret, "unable to set up polling\n");
    input_set_poll_interval(input, POLL_INTERVAL);
    ret = input_register_device(input);
    if (ret)
    return dev_err_probe(dev, ret, "unable to register input device\n");
    return 0;
    }
    static struct platform_driver qnap_mcu_input_driver = {
    .probe = qnap_mcu_input_probe,
    .driver = {
    .name = "qnap-mcu-input",
    },
    };
    module_platform_driver(qnap_mcu_input_driver);
    MODULE_ALIAS("platform:qnap-mcu-input");
    MODULE_AUTHOR("Heiko Stuebner <heiko@sntech.de>");
    MODULE_DESCRIPTION("QNAP MCU input driver");
    MODULE_LICENSE("GPL");
