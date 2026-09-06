//! Automatically rewritten from C to Rust
//! Source: drivers/input/mouse/atarimouse.c
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
// Atari mouse driver for Linux/m68k
//
// Copyright (c) 2005 Michael Schmitz
//
// Based on:
// Amiga mouse driver for Linux/m68k
//
// Copyright (c) 2000-2002 Vojtech Pavlik
//
// The low level init and interrupt stuff is handled in arch/mm68k/atari/atakeyb.c
// (the keyboard ACIA also handles the mouse and joystick data, and the keyboard
// interrupt is shared with the MIDI ACIA so MIDI data also get handled there).
// This driver only deals with handing key events off to the input layer.
//
// Largely based on the old:
//
// Atari Mouse Driver for Linux
// by Robert de Vries (robert@and.nl) 19Jul93
//
// 16 Nov 1994 Andreas Schwab
// Compatibility with busmouse
// Support for three button mouse (shamelessly stolen from MiNT)
// third button wired to one of the joystick directions on joystick 1
//
// 1996/02/11 Andreas Schwab
// Module support
// Allow multiple open's
//
// Converted to use new generic busmouse code.  5 Apr 1998
// Russell King <rmk@arm.uk.linux.org>
//

    MODULE_AUTHOR("Michael Schmitz <schmitz@biophys.uni-duesseldorf.de>");
    MODULE_DESCRIPTION("Atari mouse driver");
    MODULE_LICENSE("GPL");
    static int mouse_threshold[2] = {2, 2};
    module_param_array(mouse_threshold, int, core::ptr::null_mut(), 0);

    extern int atari_mouse_buttons;

    static struct input_dev *atamouse_dev;
#[no_mangle]
unsafe extern "C" fn atamouse_interrupt(buf: *mut c_char) {
    static void atamouse_interrupt(char *buf)
    {
    int buttons, dx, dy;
    buttons = (buf[0] & 1) | ((buf[0] & 2) << 1);

    buttons |= atari_mouse_buttons & 2;
    atari_mouse_buttons = buttons;

// only relative events get here
    dx = buf[1];
    dy = buf[2];
    input_report_rel(atamouse_dev, REL_X, dx);
    input_report_rel(atamouse_dev, REL_Y, dy);
    input_report_key(atamouse_dev, BTN_LEFT,   buttons & 0x4);
    input_report_key(atamouse_dev, BTN_MIDDLE, buttons & 0x2);
    input_report_key(atamouse_dev, BTN_RIGHT,  buttons & 0x1);
    input_sync(atamouse_dev);
    return;
    }
#[no_mangle]
unsafe extern "C" fn atamouse_open(dev: *mut input_dev) -> c_int {
    static int atamouse_open(struct input_dev *dev)
    {

    atari_mouse_buttons = 0;

    ikbd_mouse_y0_top();
    ikbd_mouse_thresh(mouse_threshold[0], mouse_threshold[1]);
    ikbd_mouse_rel_pos();
    atari_input_mouse_interrupt_hook = atamouse_interrupt;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn atamouse_close(dev: *mut input_dev) {
    static void atamouse_close(struct input_dev *dev)
    {
    ikbd_mouse_disable();
    atari_input_mouse_interrupt_hook = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn atamouse_init() -> int __init {
    static int __init atamouse_init(void)
    {
    int error;
    if (!MACH_IS_ATARI || !ATARIHW_PRESENT(ST_MFP))
    return -ENODEV;
    error = atari_keyb_init();
    if (error)
    return error;
    atamouse_dev = input_allocate_device();
    if (!atamouse_dev)
    return -ENOMEM;
    atamouse_dev.name = "Atari mouse";
    atamouse_dev.phys = "atamouse/input0";
    atamouse_dev.id.bustype = BUS_HOST;
    atamouse_dev.id.vendor = 0x0001;
    atamouse_dev.id.product = 0x0002;
    atamouse_dev.id.version = 0x0100;
    atamouse_dev.evbit[0] = BIT_MASK(EV_KEY) | BIT_MASK(EV_REL);
    atamouse_dev.relbit[0] = BIT_MASK(REL_X) | BIT_MASK(REL_Y);
    atamouse_dev.keybit[BIT_WORD(BTN_LEFT)] = BIT_MASK(BTN_LEFT) |
    BIT_MASK(BTN_MIDDLE) | BIT_MASK(BTN_RIGHT);
    atamouse_dev.open = atamouse_open;
    atamouse_dev.close = atamouse_close;
    error = input_register_device(atamouse_dev);
    if (error) {
    input_free_device(atamouse_dev);
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn atamouse_exit() -> void __exit {
    static void __exit atamouse_exit(void)
    {
    input_unregister_device(atamouse_dev);
    }
    module_init(atamouse_init);
    module_exit(atamouse_exit);
