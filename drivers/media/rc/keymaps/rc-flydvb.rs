//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-flydvb.c
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
// flydvb.h - Keytable for flydvb Remote Controller
//
// keymap imported from ir-keymaps.c
//
// Copyright (c) 2010 by Mauro Carvalho Chehab

    static struct rc_map_table flydvb[] = {
    { 0x01, KEY_ZOOM },		/* Full Screen */
    { 0x00, KEY_POWER },		/* Power */
    { 0x03, KEY_NUMERIC_1 },
    { 0x04, KEY_NUMERIC_2 },
    { 0x05, KEY_NUMERIC_3 },
    { 0x07, KEY_NUMERIC_4 },
    { 0x08, KEY_NUMERIC_5 },
    { 0x09, KEY_NUMERIC_6 },
    { 0x0b, KEY_NUMERIC_7 },
    { 0x0c, KEY_NUMERIC_8 },
    { 0x0d, KEY_NUMERIC_9 },
    { 0x06, KEY_AGAIN },		/* Recall */
    { 0x0f, KEY_NUMERIC_0 },
    { 0x10, KEY_MUTE },		/* Mute */
    { 0x02, KEY_RADIO },		/* TV/Radio */
    { 0x1b, KEY_LANGUAGE },		/* SAP (Second Audio Program) */
    { 0x14, KEY_VOLUMEUP },		/* VOL+ */
    { 0x17, KEY_VOLUMEDOWN },	/* VOL- */
    { 0x12, KEY_CHANNELUP },	/* CH+ */
    { 0x13, KEY_CHANNELDOWN },	/* CH- */
    { 0x1d, KEY_ENTER },		/* Enter */
    { 0x1a, KEY_TV2 },		/* PIP */
    { 0x18, KEY_VIDEO },		/* Source */
    { 0x1e, KEY_RECORD },		/* Record/Pause */
    { 0x15, KEY_ANGLE },		/* Swap (no label on key) */
    { 0x1c, KEY_PAUSE },		/* Timeshift/Pause */
    { 0x19, KEY_BACK },		/* Rewind << */
    { 0x0a, KEY_PLAYPAUSE },	/* Play/Pause */
    { 0x1f, KEY_FORWARD },		/* Forward >> */
    { 0x16, KEY_PREVIOUS },		/* Back |<< */
    { 0x11, KEY_STOP },		/* Stop */
    { 0x0e, KEY_NEXT },		/* End >>| */
    };
    static struct rc_map_list flydvb_map = {
    .map = {
    .scan     = flydvb,
    .size     = ARRAY_SIZE(flydvb),
    .rc_proto = RC_PROTO_UNKNOWN,	/* Legacy IR type */
    .name     = RC_MAP_FLYDVB,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_flydvb() -> int __init {
    static int __init init_rc_map_flydvb(void)
    {
    return rc_map_register(&flydvb_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_flydvb() -> void __exit {
    static void __exit exit_rc_map_flydvb(void)
    {
    rc_map_unregister(&flydvb_map);
    }
    module_init(init_rc_map_flydvb)
    module_exit(exit_rc_map_flydvb)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Mauro Carvalho Chehab");
    MODULE_DESCRIPTION("flydvb remote controller keytable");
