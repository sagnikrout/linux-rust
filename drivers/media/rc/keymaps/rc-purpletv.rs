//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-purpletv.c
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
// purpletv.h - Keytable for purpletv Remote Controller
//
// keymap imported from ir-keymaps.c
//
// Copyright (c) 2010 by Mauro Carvalho Chehab

    static struct rc_map_table purpletv[] = {
    { 0x03, KEY_POWER },
    { 0x6f, KEY_MUTE },
    { 0x10, KEY_BACKSPACE },	/* Recall */
    { 0x11, KEY_NUMERIC_0 },
    { 0x04, KEY_NUMERIC_1 },
    { 0x05, KEY_NUMERIC_2 },
    { 0x06, KEY_NUMERIC_3 },
    { 0x08, KEY_NUMERIC_4 },
    { 0x09, KEY_NUMERIC_5 },
    { 0x0a, KEY_NUMERIC_6 },
    { 0x0c, KEY_NUMERIC_7 },
    { 0x0d, KEY_NUMERIC_8 },
    { 0x0e, KEY_NUMERIC_9 },
    { 0x12, KEY_DOT },	/* 100+ */
    { 0x07, KEY_VOLUMEUP },
    { 0x0b, KEY_VOLUMEDOWN },
    { 0x1a, KEY_KPPLUS },
    { 0x18, KEY_KPMINUS },
    { 0x15, KEY_UP },
    { 0x1d, KEY_DOWN },
    { 0x0f, KEY_CHANNELUP },
    { 0x13, KEY_CHANNELDOWN },
    { 0x48, KEY_ZOOM },
    { 0x1b, KEY_VIDEO },	/* Video source */
    { 0x1f, KEY_CAMERA },	/* Snapshot */
    { 0x49, KEY_LANGUAGE },	/* MTS Select */
    { 0x19, KEY_SEARCH },	/* Auto Scan */
    { 0x4b, KEY_RECORD },
    { 0x46, KEY_PLAY },
    { 0x45, KEY_PAUSE },	/* Pause */
    { 0x44, KEY_STOP },
    { 0x43, KEY_TIME },	/* Time Shift */
    { 0x17, KEY_CHANNEL },	/* SURF CH */
    { 0x40, KEY_FORWARD },	/* Forward ? */
    { 0x42, KEY_REWIND },	/* Backward ? */
    };
    static struct rc_map_list purpletv_map = {
    .map = {
    .scan     = purpletv,
    .size     = ARRAY_SIZE(purpletv),
    .rc_proto = RC_PROTO_UNKNOWN,	/* Legacy IR type */
    .name     = RC_MAP_PURPLETV,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_purpletv() -> int __init {
    static int __init init_rc_map_purpletv(void)
    {
    return rc_map_register(&purpletv_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_purpletv() -> void __exit {
    static void __exit exit_rc_map_purpletv(void)
    {
    rc_map_unregister(&purpletv_map);
    }
    module_init(init_rc_map_purpletv)
    module_exit(exit_rc_map_purpletv)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Mauro Carvalho Chehab");
    MODULE_DESCRIPTION("purpletv remote controller keytable");
