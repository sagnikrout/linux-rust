//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/sgi_btns.c
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
// SGI Volume Button interface driver
//
// Copyright (C) 2008  Thomas Bogendoerfer <tsbogend@alpha.franken.de>
//

#[no_mangle]
pub unsafe extern "C" fn button_status() -> u8 {
    static inline u8 button_status(void)
    {
    u8 status;
    status = readb(&sgioc.panel) ^ 0xa0;
    return ((status & 0x80) >> 6) | ((status & 0x20) >> 5);
    }

#[no_mangle]
pub unsafe extern "C" fn button_status() -> u8 {
    static inline u8 button_status(void)
    {
    u64 status;
    status = readq(&mace.perif.audio.control);
    writeq(status & ~(3U << 23), &mace.perif.audio.control);
    return (status >> 23) & 3;
    }

pub const BUTTONS_COUNT_THRESHOLD: c_int = 3;
    static const unsigned short sgi_map[] = {
    KEY_VOLUMEDOWN,
    KEY_VOLUMEUP
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct buttons_dev {
    pub keymap: [c_ushort; ARRAY_SIZE(sgi_map)],
    pub count: [c_int; ARRAY_SIZE(sgi_map)],
}

#[no_mangle]
unsafe extern "C" fn handle_buttons(input: *mut input_dev) {
    static void handle_buttons(struct input_dev *input)
    {
    struct buttons_dev *bdev = input_get_drvdata(input);
    u8 status;
    int i;
    status = button_status();
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
unsafe extern "C" fn sgi_buttons_probe(pdev: *mut platform_device) -> c_int {
    static int sgi_buttons_probe(struct platform_device *pdev)
    {
    struct buttons_dev *bdev;
    struct input_dev *input;
    int error, i;
    bdev = devm_kzalloc(&pdev.dev, sizeof(*bdev), GFP_KERNEL);
    if (!bdev)
    return -ENOMEM;
    input = devm_input_allocate_device(&pdev.dev);
    if (!input)
    return -ENOMEM;
    memcpy(bdev.keymap, sgi_map, sizeof(bdev.keymap));
    input_set_drvdata(input, bdev);
    input.name = "SGI buttons";
    input.phys = "sgi/input0";
    input.id.bustype = BUS_HOST;
    input.keycode = bdev.keymap;
    input.keycodemax = ARRAY_SIZE(bdev.keymap);
    input.keycodesize = sizeof(unsigned short);
    input_set_capability(input, EV_MSC, MSC_SCAN);
    __set_bit(EV_KEY, input.evbit);
    for (i = 0; i < ARRAY_SIZE(sgi_map); i++)
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
    static struct platform_driver sgi_buttons_driver = {
    .probe	= sgi_buttons_probe,
    .driver	= {
    .name	= "sgibtns",
    },
    };
    module_platform_driver(sgi_buttons_driver);
    MODULE_DESCRIPTION("SGI Indy/O2 volume button interface driver");
    MODULE_LICENSE("GPL");
