//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-avermedia.c
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
// avermedia.h - Keytable for avermedia Remote Controller
//
// keymap imported from ir-keymaps.c
//
// Copyright (c) 2010 by Mauro Carvalho Chehab

// Alex Hermann <gaaf@gmx.net>
    static struct rc_map_table avermedia[] = {
    { 0x28, KEY_NUMERIC_1 },
    { 0x18, KEY_NUMERIC_2 },
    { 0x38, KEY_NUMERIC_3 },
    { 0x24, KEY_NUMERIC_4 },
    { 0x14, KEY_NUMERIC_5 },
    { 0x34, KEY_NUMERIC_6 },
    { 0x2c, KEY_NUMERIC_7 },
    { 0x1c, KEY_NUMERIC_8 },
    { 0x3c, KEY_NUMERIC_9 },
    { 0x22, KEY_NUMERIC_0 },
    { 0x20, KEY_TV },		/* TV/FM */
    { 0x10, KEY_CD },		/* CD */
    { 0x30, KEY_TEXT },		/* TELETEXT */
    { 0x00, KEY_POWER },		/* POWER */
    { 0x08, KEY_VIDEO },		/* VIDEO */
    { 0x04, KEY_AUDIO },		/* AUDIO */
    { 0x0c, KEY_ZOOM },		/* FULL SCREEN */
    { 0x12, KEY_SUBTITLE },		/* DISPLAY */
    { 0x32, KEY_REWIND },		/* LOOP	*/
    { 0x02, KEY_PRINT },		/* PREVIEW */
    { 0x2a, KEY_SEARCH },		/* AUTOSCAN */
    { 0x1a, KEY_SLEEP },		/* FREEZE */
    { 0x3a, KEY_CAMERA },		/* SNAPSHOT */
    { 0x0a, KEY_MUTE },		/* MUTE */
    { 0x26, KEY_RECORD },		/* RECORD */
    { 0x16, KEY_PAUSE },		/* PAUSE */
    { 0x36, KEY_STOP },		/* STOP */
    { 0x06, KEY_PLAY },		/* PLAY */
    { 0x2e, KEY_RED },		/* RED */
    { 0x21, KEY_GREEN },		/* GREEN */
    { 0x0e, KEY_YELLOW },		/* YELLOW */
    { 0x01, KEY_BLUE },		/* BLUE */
    { 0x1e, KEY_VOLUMEDOWN },	/* VOLUME- */
    { 0x3e, KEY_VOLUMEUP },		/* VOLUME+ */
    { 0x11, KEY_CHANNELDOWN },	/* CHANNEL/PAGE- */
    { 0x31, KEY_CHANNELUP }		/* CHANNEL/PAGE+ */
    };
    static struct rc_map_list avermedia_map = {
    .map = {
    .scan     = avermedia,
    .size     = ARRAY_SIZE(avermedia),
    .rc_proto = RC_PROTO_UNKNOWN,	/* Legacy IR type */
    .name     = RC_MAP_AVERMEDIA,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_avermedia() -> int __init {
    static int __init init_rc_map_avermedia(void)
    {
    return rc_map_register(&avermedia_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_avermedia() -> void __exit {
    static void __exit exit_rc_map_avermedia(void)
    {
    rc_map_unregister(&avermedia_map);
    }
    module_init(init_rc_map_avermedia)
    module_exit(exit_rc_map_avermedia)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Mauro Carvalho Chehab");
    MODULE_DESCRIPTION("avermedia remote controller keytable");
