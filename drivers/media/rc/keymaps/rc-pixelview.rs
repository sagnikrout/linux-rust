//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-pixelview.c
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
// pixelview.h - Keytable for pixelview Remote Controller
//
// keymap imported from ir-keymaps.c
//
// Copyright (c) 2010 by Mauro Carvalho Chehab

    static struct rc_map_table pixelview[] = {
    { 0x1e, KEY_POWER },	/* power */
    { 0x07, KEY_VIDEO },	/* source */
    { 0x1c, KEY_SEARCH },	/* scan */
    { 0x03, KEY_TUNER },		/* TV/FM */
    { 0x00, KEY_RECORD },
    { 0x08, KEY_STOP },
    { 0x11, KEY_PLAY },
    { 0x1a, KEY_PLAYPAUSE },	/* freeze */
    { 0x19, KEY_ZOOM },		/* zoom */
    { 0x0f, KEY_TEXT },		/* min */
    { 0x01, KEY_NUMERIC_1 },
    { 0x0b, KEY_NUMERIC_2 },
    { 0x1b, KEY_NUMERIC_3 },
    { 0x05, KEY_NUMERIC_4 },
    { 0x09, KEY_NUMERIC_5 },
    { 0x15, KEY_NUMERIC_6 },
    { 0x06, KEY_NUMERIC_7 },
    { 0x0a, KEY_NUMERIC_8 },
    { 0x12, KEY_NUMERIC_9 },
    { 0x02, KEY_NUMERIC_0 },
    { 0x10, KEY_LAST },		/* +100 */
    { 0x13, KEY_LIST },		/* recall */
    { 0x1f, KEY_CHANNELUP },	/* chn down */
    { 0x17, KEY_CHANNELDOWN },	/* chn up */
    { 0x16, KEY_VOLUMEUP },		/* vol down */
    { 0x14, KEY_VOLUMEDOWN },	/* vol up */
    { 0x04, KEY_KPMINUS },		/* <<< */
    { 0x0e, KEY_SETUP },		/* function */
    { 0x0c, KEY_KPPLUS },		/* >>> */
    { 0x0d, KEY_GOTO },		/* mts */
    { 0x1d, KEY_REFRESH },		/* reset */
    { 0x18, KEY_MUTE },		/* mute/unmute */
    };
    static struct rc_map_list pixelview_map = {
    .map = {
    .scan     = pixelview,
    .size     = ARRAY_SIZE(pixelview),
    .rc_proto = RC_PROTO_UNKNOWN,	/* Legacy IR type */
    .name     = RC_MAP_PIXELVIEW,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_pixelview() -> int __init {
    static int __init init_rc_map_pixelview(void)
    {
    return rc_map_register(&pixelview_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_pixelview() -> void __exit {
    static void __exit exit_rc_map_pixelview(void)
    {
    rc_map_unregister(&pixelview_map);
    }
    module_init(init_rc_map_pixelview)
    module_exit(exit_rc_map_pixelview)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Mauro Carvalho Chehab");
    MODULE_DESCRIPTION("pixelview remote controller keytable");
