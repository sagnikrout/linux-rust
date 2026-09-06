//! Automatically rewritten from C to Rust
//! Source: drivers/input/keyboard/newtonkbd.c
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
// Copyright (c) 2000 Justin Cormack
//
// Newton keyboard driver for Linux
//

    MODULE_AUTHOR("Justin Cormack <j.cormack@doc.ic.ac.uk>");
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_LICENSE("GPL");
pub const NKBD_KEY: c_uint = 0x7f;
pub const NKBD_PRESS: c_uint = 0x80;
    static unsigned char nkbd_keycode[128] = {
    KEY_A, KEY_S, KEY_D, KEY_F, KEY_H, KEY_G, KEY_Z, KEY_X,
    KEY_C, KEY_V, 0, KEY_B, KEY_Q, KEY_W, KEY_E, KEY_R,
    KEY_Y, KEY_T, KEY_1, KEY_2, KEY_3, KEY_4, KEY_6, KEY_5,
    KEY_EQUAL, KEY_9, KEY_7, KEY_MINUS, KEY_8, KEY_0, KEY_RIGHTBRACE, KEY_O,
    KEY_U, KEY_LEFTBRACE, KEY_I, KEY_P, KEY_ENTER, KEY_L, KEY_J, KEY_APOSTROPHE,
    KEY_K, KEY_SEMICOLON, KEY_BACKSLASH, KEY_COMMA, KEY_SLASH, KEY_N, KEY_M, KEY_DOT,
    KEY_TAB, KEY_SPACE, KEY_GRAVE, KEY_DELETE, 0, 0, 0, KEY_LEFTMETA,
    KEY_LEFTSHIFT, KEY_CAPSLOCK, KEY_LEFTALT, KEY_LEFTCTRL, KEY_RIGHTSHIFT, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    KEY_LEFT, KEY_RIGHT, KEY_DOWN, KEY_UP, 0
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nkbd {
    pub keycode: [c_uchar; 128],
    pub dev: *mut input_dev,
    pub serio: *mut serio,
    pub phys: [c_char; 32],
}

    static irqreturn_t nkbd_interrupt(struct serio *serio,
    unsigned char data, unsigned int flags)
    {
    struct nkbd *nkbd = serio_get_drvdata(serio);
// invalid scan codes are probably the init sequence, so we ignore them
    if (nkbd.keycode[data & NKBD_KEY]) {
    input_report_key(nkbd.dev, nkbd.keycode[data & NKBD_KEY], data & NKBD_PRESS);
    input_sync(nkbd.dev);
    }
    else if (data == 0xe7) /* end of init sequence */
    printk(KERN_INFO "input: %s on %s\n", nkbd.dev.name, serio.phys);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn nkbd_connect(serio: *mut serio, drv: *mut serio_driver) -> c_int {
    static int nkbd_connect(struct serio *serio, struct serio_driver *drv)
    {
    struct nkbd *nkbd;
    struct input_dev *input_dev;
    let mut err: c_int = -ENOMEM;
    int i;
    nkbd = kzalloc_obj(*nkbd);
    input_dev = input_allocate_device();
    if (!nkbd || !input_dev)
    goto fail1;
    nkbd.serio = serio;
    nkbd.dev = input_dev;
    snprintf(nkbd.phys, sizeof(nkbd.phys), "%s/input0", serio.phys);
    memcpy(nkbd.keycode, nkbd_keycode, sizeof(nkbd.keycode));
    input_dev.name = "Newton Keyboard";
    input_dev.phys = nkbd.phys;
    input_dev.id.bustype = BUS_RS232;
    input_dev.id.vendor = SERIO_NEWTON;
    input_dev.id.product = 0x0001;
    input_dev.id.version = 0x0100;
    input_dev.dev.parent = &serio.dev;
    input_dev.evbit[0] = BIT_MASK(EV_KEY) | BIT_MASK(EV_REP);
    input_dev.keycode = nkbd.keycode;
    input_dev.keycodesize = sizeof(unsigned char);
    input_dev.keycodemax = ARRAY_SIZE(nkbd_keycode);
    for (i = 0; i < 128; i++)
    set_bit(nkbd.keycode[i], input_dev.keybit);
    clear_bit(0, input_dev.keybit);
    serio_set_drvdata(serio, nkbd);
    err = serio_open(serio, drv);
    if (err)
    goto fail2;
    err = input_register_device(nkbd.dev);
    if (err)
    goto fail3;
    return 0;
    fail3:	serio_close(serio);
    fail2:	serio_set_drvdata(serio, core::ptr::null_mut());
    fail1:	input_free_device(input_dev);
    kfree(nkbd);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn nkbd_disconnect(serio: *mut serio) {
    static void nkbd_disconnect(struct serio *serio)
    {
    struct nkbd *nkbd = serio_get_drvdata(serio);
    serio_close(serio);
    serio_set_drvdata(serio, core::ptr::null_mut());
    input_unregister_device(nkbd.dev);
    kfree(nkbd);
    }
    static const struct serio_device_id nkbd_serio_ids[] = {
    {
    .type	= SERIO_RS232,
    .proto	= SERIO_NEWTON,
    .id	= SERIO_ANY,
    .extra	= SERIO_ANY,
    },
    { 0 }
    };
    MODULE_DEVICE_TABLE(serio, nkbd_serio_ids);
    static struct serio_driver nkbd_drv = {
    .driver		= {
    .name	= "newtonkbd",
    },
    .description	= DRIVER_DESC,
    .id_table	= nkbd_serio_ids,
    .interrupt	= nkbd_interrupt,
    .connect	= nkbd_connect,
    .disconnect	= nkbd_disconnect,
    };
    module_serio_driver(nkbd_drv);
