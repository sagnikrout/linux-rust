//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-pinnacle-pctv-hd.c
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
// pinnacle-pctv-hd.h - Keytable for pinnacle_pctv_hd Remote Controller
//
// keymap imported from ir-keymaps.c
//
// Copyright (c) 2010 by Mauro Carvalho Chehab

// Pinnacle PCTV HD 800i mini remote
    static struct rc_map_table pinnacle_pctv_hd[] = {
// Key codes for the tiny Pinnacle remote
    { 0x0700, KEY_MUTE },
    { 0x0701, KEY_MENU }, /* Pinnacle logo */
    { 0x0739, KEY_POWER },
    { 0x0703, KEY_VOLUMEUP },
    { 0x0705, KEY_OK },
    { 0x0709, KEY_VOLUMEDOWN },
    { 0x0706, KEY_CHANNELUP },
    { 0x070c, KEY_CHANNELDOWN },
    { 0x070f, KEY_NUMERIC_1 },
    { 0x0715, KEY_NUMERIC_2 },
    { 0x0710, KEY_NUMERIC_3 },
    { 0x0718, KEY_NUMERIC_4 },
    { 0x071b, KEY_NUMERIC_5 },
    { 0x071e, KEY_NUMERIC_6 },
    { 0x0711, KEY_NUMERIC_7 },
    { 0x0721, KEY_NUMERIC_8 },
    { 0x0712, KEY_NUMERIC_9 },
    { 0x0727, KEY_NUMERIC_0 },
    { 0x0724, KEY_ZOOM }, /* 'Square' key */
    { 0x072a, KEY_SUBTITLE },   /* 'T' key */
    { 0x072d, KEY_REWIND },
    { 0x0730, KEY_PLAYPAUSE },
    { 0x0733, KEY_FASTFORWARD },
    { 0x0736, KEY_RECORD },
    { 0x073c, KEY_STOP },
    { 0x073f, KEY_HELP }, /* '?' key */
    };
    static struct rc_map_list pinnacle_pctv_hd_map = {
    .map = {
    .scan     = pinnacle_pctv_hd,
    .size     = ARRAY_SIZE(pinnacle_pctv_hd),
    .rc_proto = RC_PROTO_RC5,
    .name     = RC_MAP_PINNACLE_PCTV_HD,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_pinnacle_pctv_hd() -> int __init {
    static int __init init_rc_map_pinnacle_pctv_hd(void)
    {
    return rc_map_register(&pinnacle_pctv_hd_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_pinnacle_pctv_hd() -> void __exit {
    static void __exit exit_rc_map_pinnacle_pctv_hd(void)
    {
    rc_map_unregister(&pinnacle_pctv_hd_map);
    }
    module_init(init_rc_map_pinnacle_pctv_hd)
    module_exit(exit_rc_map_pinnacle_pctv_hd)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Mauro Carvalho Chehab");
    MODULE_DESCRIPTION("Pinnacle PCTV HD 800i mini remote controller keytable");
