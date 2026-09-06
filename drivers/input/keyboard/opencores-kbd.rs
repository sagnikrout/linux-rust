//! Automatically rewritten from C to Rust
//! Source: drivers/input/keyboard/opencores-kbd.c
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
// OpenCores Keyboard Controller Driver
// http://www.opencores.org/project,keyboardcontroller
//
// Copyright 2007-2009 HV Sistemas S.L.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opencores_kbd {
    pub input: *mut input_dev,
    pub addr: *mut void __iomem,
    pub irq: c_int,
    pub keycodes: [c_ushort; 128],
}

#[no_mangle]
unsafe extern "C" fn opencores_kbd_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t opencores_kbd_isr(int irq, void *dev_id)
    {
    struct opencores_kbd *opencores_kbd = dev_id;
    struct input_dev *input = opencores_kbd.input;
    unsigned char c;
    c = readb(opencores_kbd.addr);
    input_report_key(input, c & 0x7f, c & 0x80 ? 0 : 1);
    input_sync(input);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn opencores_kbd_probe(pdev: *mut platform_device) -> c_int {
    static int opencores_kbd_probe(struct platform_device *pdev)
    {
    struct input_dev *input;
    struct opencores_kbd *opencores_kbd;
    int irq, i, error;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return -EINVAL;
    opencores_kbd = devm_kzalloc(&pdev.dev, sizeof(*opencores_kbd),
    GFP_KERNEL);
    if (!opencores_kbd)
    return -ENOMEM;
    input = devm_input_allocate_device(&pdev.dev);
    if (!input) {
    dev_err(&pdev.dev, "failed to allocate input device\n");
    return -ENOMEM;
    }
    opencores_kbd.input = input;
    opencores_kbd.addr = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(opencores_kbd.addr))
    return PTR_ERR(opencores_kbd.addr);
    input.name = pdev.name;
    input.phys = "opencores-kbd/input0";
    input.id.bustype = BUS_HOST;
    input.id.vendor = 0x0001;
    input.id.product = 0x0001;
    input.id.version = 0x0100;
    input.keycode = opencores_kbd.keycodes;
    input.keycodesize = sizeof(opencores_kbd.keycodes[0]);
    input.keycodemax = ARRAY_SIZE(opencores_kbd.keycodes);
    __set_bit(EV_KEY, input.evbit);
    for (i = 0; i < ARRAY_SIZE(opencores_kbd.keycodes); i++) {
//
// OpenCores controller happens to have scancodes match
// our KEY_* definitions.
//
    opencores_kbd.keycodes[i] = i;
    __set_bit(opencores_kbd.keycodes[i], input.keybit);
    }
    __clear_bit(KEY_RESERVED, input.keybit);
    error = devm_request_irq(&pdev.dev, irq, &opencores_kbd_isr,
    IRQF_TRIGGER_RISING,
    pdev.name, opencores_kbd);
    if (error) {
    dev_err(&pdev.dev, "unable to claim irq %d\n", irq);
    return error;
    }
    error = input_register_device(input);
    if (error) {
    dev_err(&pdev.dev, "unable to register input device\n");
    return error;
    }
    return 0;
    }
    static struct platform_driver opencores_kbd_device_driver = {
    .probe    = opencores_kbd_probe,
    .driver   = {
    .name = "opencores-kbd",
    },
    };
    module_platform_driver(opencores_kbd_device_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Javier Herrero <jherrero@hvsistemas.es>");
    MODULE_DESCRIPTION("Keyboard driver for OpenCores Keyboard Controller");
