//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-cinergy-1400.c
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
// cinergy-1400.h - Keytable for cinergy_1400 Remote Controller
//
// keymap imported from ir-keymaps.c
//
// Copyright (c) 2010 by Mauro Carvalho Chehab

// Cinergy 1400 DVB-T
    static struct rc_map_table cinergy_1400[] = {
    { 0x01, KEY_POWER },
    { 0x02, KEY_NUMERIC_1 },
    { 0x03, KEY_NUMERIC_2 },
    { 0x04, KEY_NUMERIC_3 },
    { 0x05, KEY_NUMERIC_4 },
    { 0x06, KEY_NUMERIC_5 },
    { 0x07, KEY_NUMERIC_6 },
    { 0x08, KEY_NUMERIC_7 },
    { 0x09, KEY_NUMERIC_8 },
    { 0x0a, KEY_NUMERIC_9 },
    { 0x0c, KEY_NUMERIC_0 },
    { 0x0b, KEY_VIDEO },
    { 0x0d, KEY_REFRESH },
    { 0x0e, KEY_SELECT },
    { 0x0f, KEY_EPG },
    { 0x10, KEY_UP },
    { 0x11, KEY_LEFT },
    { 0x12, KEY_OK },
    { 0x13, KEY_RIGHT },
    { 0x14, KEY_DOWN },
    { 0x15, KEY_TEXT },
    { 0x16, KEY_INFO },
    { 0x17, KEY_RED },
    { 0x18, KEY_GREEN },
    { 0x19, KEY_YELLOW },
    { 0x1a, KEY_BLUE },
    { 0x1b, KEY_CHANNELUP },
    { 0x1c, KEY_VOLUMEUP },
    { 0x1d, KEY_MUTE },
    { 0x1e, KEY_VOLUMEDOWN },
    { 0x1f, KEY_CHANNELDOWN },
    { 0x40, KEY_PAUSE },
    { 0x4c, KEY_PLAY },
    { 0x58, KEY_RECORD },
    { 0x54, KEY_PREVIOUS },
    { 0x48, KEY_STOP },
    { 0x5c, KEY_NEXT },
    };
    static struct rc_map_list cinergy_1400_map = {
    .map = {
    .scan     = cinergy_1400,
    .size     = ARRAY_SIZE(cinergy_1400),
    .rc_proto = RC_PROTO_UNKNOWN,	/* Legacy IR type */
    .name     = RC_MAP_CINERGY_1400,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_cinergy_1400() -> int __init {
    static int __init init_rc_map_cinergy_1400(void)
    {
    return rc_map_register(&cinergy_1400_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_cinergy_1400() -> void __exit {
    static void __exit exit_rc_map_cinergy_1400(void)
    {
    rc_map_unregister(&cinergy_1400_map);
    }
    module_init(init_rc_map_cinergy_1400)
    module_exit(exit_rc_map_cinergy_1400)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Mauro Carvalho Chehab");
    MODULE_DESCRIPTION("Cinergy 1400 DVB-T remote controller keytable");
