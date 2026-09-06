//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-avertv-303.c
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
// avertv-303.h - Keytable for avertv_303 Remote Controller
//
// keymap imported from ir-keymaps.c
//
// Copyright (c) 2010 by Mauro Carvalho Chehab

// AVERTV STUDIO 303 Remote
    static struct rc_map_table avertv_303[] = {
    { 0x2a, KEY_NUMERIC_1 },
    { 0x32, KEY_NUMERIC_2 },
    { 0x3a, KEY_NUMERIC_3 },
    { 0x4a, KEY_NUMERIC_4 },
    { 0x52, KEY_NUMERIC_5 },
    { 0x5a, KEY_NUMERIC_6 },
    { 0x6a, KEY_NUMERIC_7 },
    { 0x72, KEY_NUMERIC_8 },
    { 0x7a, KEY_NUMERIC_9 },
    { 0x0e, KEY_NUMERIC_0 },
    { 0x02, KEY_POWER },
    { 0x22, KEY_VIDEO },
    { 0x42, KEY_AUDIO },
    { 0x62, KEY_ZOOM },
    { 0x0a, KEY_TV },
    { 0x12, KEY_CD },
    { 0x1a, KEY_TEXT },
    { 0x16, KEY_SUBTITLE },
    { 0x1e, KEY_REWIND },
    { 0x06, KEY_PRINT },
    { 0x2e, KEY_SEARCH },
    { 0x36, KEY_SLEEP },
    { 0x3e, KEY_SHUFFLE },
    { 0x26, KEY_MUTE },
    { 0x4e, KEY_RECORD },
    { 0x56, KEY_PAUSE },
    { 0x5e, KEY_STOP },
    { 0x46, KEY_PLAY },
    { 0x6e, KEY_RED },
    { 0x0b, KEY_GREEN },
    { 0x66, KEY_YELLOW },
    { 0x03, KEY_BLUE },
    { 0x76, KEY_LEFT },
    { 0x7e, KEY_RIGHT },
    { 0x13, KEY_DOWN },
    { 0x1b, KEY_UP },
    };
    static struct rc_map_list avertv_303_map = {
    .map = {
    .scan     = avertv_303,
    .size     = ARRAY_SIZE(avertv_303),
    .rc_proto = RC_PROTO_UNKNOWN,	/* Legacy IR type */
    .name     = RC_MAP_AVERTV_303,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_avertv_303() -> int __init {
    static int __init init_rc_map_avertv_303(void)
    {
    return rc_map_register(&avertv_303_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_avertv_303() -> void __exit {
    static void __exit exit_rc_map_avertv_303(void)
    {
    rc_map_unregister(&avertv_303_map);
    }
    module_init(init_rc_map_avertv_303)
    module_exit(exit_rc_map_avertv_303)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Mauro Carvalho Chehab");
    MODULE_DESCRIPTION("AVERTV STUDIO 303 Remote controller keytable");
