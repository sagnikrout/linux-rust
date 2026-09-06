//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-avermedia-a16d.c
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
// avermedia-a16d.h - Keytable for avermedia_a16d Remote Controller
//
// keymap imported from ir-keymaps.c
//
// Copyright (c) 2010 by Mauro Carvalho Chehab

    static struct rc_map_table avermedia_a16d[] = {
    { 0x20, KEY_LIST},
    { 0x00, KEY_POWER},
    { 0x28, KEY_NUMERIC_1},
    { 0x18, KEY_NUMERIC_2},
    { 0x38, KEY_NUMERIC_3},
    { 0x24, KEY_NUMERIC_4},
    { 0x14, KEY_NUMERIC_5},
    { 0x34, KEY_NUMERIC_6},
    { 0x2c, KEY_NUMERIC_7},
    { 0x1c, KEY_NUMERIC_8},
    { 0x3c, KEY_NUMERIC_9},
    { 0x12, KEY_SUBTITLE},
    { 0x22, KEY_NUMERIC_0},
    { 0x32, KEY_REWIND},
    { 0x3a, KEY_SHUFFLE},
    { 0x02, KEY_PRINT},
    { 0x11, KEY_CHANNELDOWN},
    { 0x31, KEY_CHANNELUP},
    { 0x0c, KEY_ZOOM},
    { 0x1e, KEY_VOLUMEDOWN},
    { 0x3e, KEY_VOLUMEUP},
    { 0x0a, KEY_MUTE},
    { 0x04, KEY_AUDIO},
    { 0x26, KEY_RECORD},
    { 0x06, KEY_PLAY},
    { 0x36, KEY_STOP},
    { 0x16, KEY_PAUSE},
    { 0x2e, KEY_REWIND},
    { 0x0e, KEY_FASTFORWARD},
    { 0x30, KEY_TEXT},
    { 0x21, KEY_GREEN},
    { 0x01, KEY_BLUE},
    { 0x08, KEY_EPG},
    { 0x2a, KEY_MENU},
    };
    static struct rc_map_list avermedia_a16d_map = {
    .map = {
    .scan     = avermedia_a16d,
    .size     = ARRAY_SIZE(avermedia_a16d),
    .rc_proto = RC_PROTO_UNKNOWN,	/* Legacy IR type */
    .name     = RC_MAP_AVERMEDIA_A16D,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_avermedia_a16d() -> int __init {
    static int __init init_rc_map_avermedia_a16d(void)
    {
    return rc_map_register(&avermedia_a16d_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_avermedia_a16d() -> void __exit {
    static void __exit exit_rc_map_avermedia_a16d(void)
    {
    rc_map_unregister(&avermedia_a16d_map);
    }
    module_init(init_rc_map_avermedia_a16d)
    module_exit(exit_rc_map_avermedia_a16d)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Mauro Carvalho Chehab");
    MODULE_DESCRIPTION("avermedia-a16d remote controller keytable");
