//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-gotview7135.c
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
// gotview7135.h - Keytable for gotview7135 Remote Controller
//
// keymap imported from ir-keymaps.c
//
// Copyright (c) 2010 by Mauro Carvalho Chehab

// Mike Baikov <mike@baikov.com>
    static struct rc_map_table gotview7135[] = {
    { 0x11, KEY_POWER },
    { 0x35, KEY_TV },
    { 0x1b, KEY_NUMERIC_0 },
    { 0x29, KEY_NUMERIC_1 },
    { 0x19, KEY_NUMERIC_2 },
    { 0x39, KEY_NUMERIC_3 },
    { 0x1f, KEY_NUMERIC_4 },
    { 0x2c, KEY_NUMERIC_5 },
    { 0x21, KEY_NUMERIC_6 },
    { 0x24, KEY_NUMERIC_7 },
    { 0x18, KEY_NUMERIC_8 },
    { 0x2b, KEY_NUMERIC_9 },
    { 0x3b, KEY_AGAIN },	/* LOOP */
    { 0x06, KEY_AUDIO },
    { 0x31, KEY_PRINT },	/* PREVIEW */
    { 0x3e, KEY_VIDEO },
    { 0x10, KEY_CHANNELUP },
    { 0x20, KEY_CHANNELDOWN },
    { 0x0c, KEY_VOLUMEDOWN },
    { 0x28, KEY_VOLUMEUP },
    { 0x08, KEY_MUTE },
    { 0x26, KEY_SEARCH },	/* SCAN */
    { 0x3f, KEY_CAMERA },	/* SNAPSHOT */
    { 0x12, KEY_RECORD },
    { 0x32, KEY_STOP },
    { 0x3c, KEY_PLAY },
    { 0x1d, KEY_REWIND },
    { 0x2d, KEY_PAUSE },
    { 0x0d, KEY_FORWARD },
    { 0x05, KEY_ZOOM },	/*FULL*/
    { 0x2a, KEY_F21 },	/* LIVE TIMESHIFT */
    { 0x0e, KEY_F22 },	/* MIN TIMESHIFT */
    { 0x1e, KEY_TIME },	/* TIMESHIFT */
    { 0x38, KEY_F24 },	/* NORMAL TIMESHIFT */
    };
    static struct rc_map_list gotview7135_map = {
    .map = {
    .scan     = gotview7135,
    .size     = ARRAY_SIZE(gotview7135),
    .rc_proto = RC_PROTO_UNKNOWN,	/* Legacy IR type */
    .name     = RC_MAP_GOTVIEW7135,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_gotview7135() -> int __init {
    static int __init init_rc_map_gotview7135(void)
    {
    return rc_map_register(&gotview7135_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_gotview7135() -> void __exit {
    static void __exit exit_rc_map_gotview7135(void)
    {
    rc_map_unregister(&gotview7135_map);
    }
    module_init(init_rc_map_gotview7135)
    module_exit(exit_rc_map_gotview7135)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Mauro Carvalho Chehab");
    MODULE_DESCRIPTION("gotview7135 remote controller keytable");
