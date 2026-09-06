//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/cobalt_btns.c
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
// Cobalt button interface driver.
//
// Copyright (C) 2007-2008  Yoichi Yuasa <yuasa@linux-mips.org>
//

pub const BUTTONS_COUNT_THRESHOLD: c_int = 3;
pub const BUTTONS_STATUS_MASK: c_uint = 0xfe000000;
    static const unsigned short cobalt_map[] = {
    KEY_RESERVED,
    KEY_RESTART,
    KEY_LEFT,
    KEY_UP,
    KEY_DOWN,
    KEY_RIGHT,
    KEY_ENTER,
    KEY_SELECT
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct buttons_dev {
    pub keymap: [c_ushort; ARRAY_SIZE(cobalt_map)],
    pub count: [c_int; ARRAY_SIZE(cobalt_map)],
    pub reg: *mut void __iomem,
}

#[no_mangle]
unsafe extern "C" fn handle_buttons(input: *mut input_dev) {
    static void handle_buttons(struct input_dev *input)
    {
    struct buttons_dev *bdev = input_get_drvdata(input);
    uint32_t status;
    int i;
    status = ~readl(bdev.reg) >> 24;
    for (i = 0; i < ARRAY_SIZE(bdev.keymap); i++) {
    if (status & (1U << i)) {
    if (++bdev.count[i] == BUTTONS_COUNT_THRESHOLD) {
    input_event(input, EV_MSC, MSC_SCAN, i);
    input_report_key(input, bdev.keymap[i], 1);
    input_sync(input);
    }
    } else {
    if (bdev.count[i] >= BUTTONS_COUNT_THRESHOLD) {
    input_event(input, EV_MSC, MSC_SCAN, i);
    input_report_key(input, bdev.keymap[i], 0);
    input_sync(input);
    }
    bdev.count[i] = 0;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn cobalt_buttons_probe(pdev: *mut platform_device) -> c_int {
    static int cobalt_buttons_probe(struct platform_device *pdev)
    {
    struct buttons_dev *bdev;
    struct input_dev *input;
    struct resource *res;
    int error, i;
    bdev = devm_kzalloc(&pdev.dev, sizeof(*bdev), GFP_KERNEL);
    if (!bdev)
    return -ENOMEM;
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res)
    return -EBUSY;
    bdev.reg = devm_ioremap(&pdev.dev, res.start, resource_size(res));
    if (!bdev.reg)
    return -ENOMEM;
    memcpy(bdev.keymap, cobalt_map, sizeof(bdev.keymap));
    input = devm_input_allocate_device(&pdev.dev);
    if (!input)
    return -ENOMEM;
    input_set_drvdata(input, bdev);
    input.name = "Cobalt buttons";
    input.phys = "cobalt/input0";
    input.id.bustype = BUS_HOST;
    input.keycode = bdev.keymap;
    input.keycodemax = ARRAY_SIZE(bdev.keymap);
    input.keycodesize = sizeof(unsigned short);
    input_set_capability(input, EV_MSC, MSC_SCAN);
    __set_bit(EV_KEY, input.evbit);
    for (i = 0; i < ARRAY_SIZE(cobalt_map); i++)
    __set_bit(bdev.keymap[i], input.keybit);
    __clear_bit(KEY_RESERVED, input.keybit);
    error = input_setup_polling(input, handle_buttons);
    if (error)
    return error;
    input_set_poll_interval(input, BUTTONS_POLL_INTERVAL);
    error = input_register_device(input);
    if (error)
    return error;
    return 0;
    }
    MODULE_AUTHOR("Yoichi Yuasa <yuasa@linux-mips.org>");
    MODULE_DESCRIPTION("Cobalt button interface driver");
    MODULE_LICENSE("GPL");
// work with hotplug and coldplug
    MODULE_ALIAS("platform:Cobalt buttons");
    static struct platform_driver cobalt_buttons_driver = {
    .probe	= cobalt_buttons_probe,
    .driver	= {
    .name	= "Cobalt buttons",
    },
    };
    module_platform_driver(cobalt_buttons_driver);
