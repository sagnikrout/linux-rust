//! Automatically rewritten from C to Rust
//! Source: drivers/input/keyboard/stowaway.c
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
// Stowaway keyboard driver for Linux
//
// Copyright (c) 2006 Marek Vasut
//
// Based on Newton keyboard driver for Linux
// by Justin Cormack
//

    MODULE_AUTHOR("Marek Vasut <marek.vasut@gmail.com>");
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_LICENSE("GPL");
pub const SKBD_KEY_MASK: c_uint = 0x7f;
pub const SKBD_RELEASE: c_uint = 0x80;
    static unsigned char skbd_keycode[128] = {
    KEY_1, KEY_2, KEY_3, KEY_Z, KEY_4, KEY_5, KEY_6, KEY_7,
    0, KEY_Q, KEY_W, KEY_E, KEY_R, KEY_T, KEY_Y, KEY_GRAVE,
    KEY_X, KEY_A, KEY_S, KEY_D, KEY_F, KEY_G, KEY_H, KEY_SPACE,
    KEY_CAPSLOCK, KEY_TAB, KEY_LEFTCTRL, 0, 0, 0, 0, 0,
    0, 0, 0, KEY_LEFTALT, 0, 0, 0, 0,
    0, 0, 0, 0, KEY_C, KEY_V, KEY_B, KEY_N,
    KEY_MINUS, KEY_EQUAL, KEY_BACKSPACE, KEY_HOME, KEY_8, KEY_9, KEY_0, KEY_ESC,
    KEY_LEFTBRACE, KEY_RIGHTBRACE, KEY_BACKSLASH, KEY_END, KEY_U, KEY_I, KEY_O, KEY_P,
    KEY_APOSTROPHE, KEY_ENTER, KEY_PAGEUP,0, KEY_J, KEY_K, KEY_L, KEY_SEMICOLON,
    KEY_SLASH, KEY_UP, KEY_PAGEDOWN, 0,KEY_M, KEY_COMMA, KEY_DOT, KEY_INSERT,
    KEY_DELETE, KEY_LEFT, KEY_DOWN, KEY_RIGHT,  0, 0, 0,
    KEY_LEFTSHIFT, KEY_RIGHTSHIFT, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, KEY_F1, KEY_F2, KEY_F3, KEY_F4, KEY_F5, KEY_F6, KEY_F7,
    KEY_F8, KEY_F9, KEY_F10, KEY_F11, KEY_F12, 0, 0, 0
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct skbd {
    pub keycode: [c_uchar; 128],
    pub dev: *mut input_dev,
    pub serio: *mut serio,
    pub phys: [c_char; 32],
}

    static irqreturn_t skbd_interrupt(struct serio *serio, unsigned char data,
    unsigned int flags)
    {
    struct skbd *skbd = serio_get_drvdata(serio);
    struct input_dev *dev = skbd.dev;
    if (skbd.keycode[data & SKBD_KEY_MASK]) {
    input_report_key(dev, skbd.keycode[data & SKBD_KEY_MASK],
    !(data & SKBD_RELEASE));
    input_sync(dev);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn skbd_connect(serio: *mut serio, drv: *mut serio_driver) -> c_int {
    static int skbd_connect(struct serio *serio, struct serio_driver *drv)
    {
    struct skbd *skbd;
    struct input_dev *input_dev;
    let mut err: c_int = -ENOMEM;
    int i;
    skbd = kzalloc_obj(*skbd);
    input_dev = input_allocate_device();
    if (!skbd || !input_dev)
    goto fail1;
    skbd.serio = serio;
    skbd.dev = input_dev;
    snprintf(skbd.phys, sizeof(skbd.phys), "%s/input0", serio.phys);
    memcpy(skbd.keycode, skbd_keycode, sizeof(skbd.keycode));
    input_dev.name = "Stowaway Keyboard";
    input_dev.phys = skbd.phys;
    input_dev.id.bustype = BUS_RS232;
    input_dev.id.vendor = SERIO_STOWAWAY;
    input_dev.id.product = 0x0001;
    input_dev.id.version = 0x0100;
    input_dev.dev.parent = &serio.dev;
    input_dev.evbit[0] = BIT_MASK(EV_KEY) | BIT_MASK(EV_REP);
    input_dev.keycode = skbd.keycode;
    input_dev.keycodesize = sizeof(unsigned char);
    input_dev.keycodemax = ARRAY_SIZE(skbd_keycode);
    for (i = 0; i < ARRAY_SIZE(skbd_keycode); i++)
    set_bit(skbd_keycode[i], input_dev.keybit);
    clear_bit(0, input_dev.keybit);
    serio_set_drvdata(serio, skbd);
    err = serio_open(serio, drv);
    if (err)
    goto fail2;
    err = input_register_device(skbd.dev);
    if (err)
    goto fail3;
    return 0;
    fail3: serio_close(serio);
    fail2:	serio_set_drvdata(serio, core::ptr::null_mut());
    fail1:	input_free_device(input_dev);
    kfree(skbd);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn skbd_disconnect(serio: *mut serio) {
    static void skbd_disconnect(struct serio *serio)
    {
    struct skbd *skbd = serio_get_drvdata(serio);
    serio_close(serio);
    serio_set_drvdata(serio, core::ptr::null_mut());
    input_unregister_device(skbd.dev);
    kfree(skbd);
    }
    static const struct serio_device_id skbd_serio_ids[] = {
    {
    .type	= SERIO_RS232,
    .proto	= SERIO_STOWAWAY,
    .id	= SERIO_ANY,
    .extra	= SERIO_ANY,
    },
    { 0 }
    };
    MODULE_DEVICE_TABLE(serio, skbd_serio_ids);
    static struct serio_driver skbd_drv = {
    .driver		= {
    .name	= "stowaway",
    },
    .description	= DRIVER_DESC,
    .id_table	= skbd_serio_ids,
    .interrupt	= skbd_interrupt,
    .connect	= skbd_connect,
    .disconnect	= skbd_disconnect,
    };
    module_serio_driver(skbd_drv);
