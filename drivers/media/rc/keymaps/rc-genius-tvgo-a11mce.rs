//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-genius-tvgo-a11mce.c
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
// genius-tvgo-a11mce.h - Keytable for genius_tvgo_a11mce Remote Controller
//
// keymap imported from ir-keymaps.c
//
// Copyright (c) 2010 by Mauro Carvalho Chehab

//
// Remote control for the Genius TVGO A11MCE
// Adrian Pardini <pardo.bsso@gmail.com>
//
    static struct rc_map_table genius_tvgo_a11mce[] = {
// Keys 0 to 9
    { 0x48, KEY_NUMERIC_0 },
    { 0x09, KEY_NUMERIC_1 },
    { 0x1d, KEY_NUMERIC_2 },
    { 0x1f, KEY_NUMERIC_3 },
    { 0x19, KEY_NUMERIC_4 },
    { 0x1b, KEY_NUMERIC_5 },
    { 0x11, KEY_NUMERIC_6 },
    { 0x17, KEY_NUMERIC_7 },
    { 0x12, KEY_NUMERIC_8 },
    { 0x16, KEY_NUMERIC_9 },
    { 0x54, KEY_RECORD },		/* recording */
    { 0x06, KEY_MUTE },		/* mute */
    { 0x10, KEY_POWER },
    { 0x40, KEY_LAST },		/* recall */
    { 0x4c, KEY_CHANNELUP },	/* channel / program + */
    { 0x00, KEY_CHANNELDOWN },	/* channel / program - */
    { 0x0d, KEY_VOLUMEUP },
    { 0x15, KEY_VOLUMEDOWN },
    { 0x4d, KEY_OK },		/* also labeled as Pause */
    { 0x1c, KEY_ZOOM },		/* full screen and Stop*/
    { 0x02, KEY_MODE },		/* AV Source or Rewind*/
    { 0x04, KEY_LIST },		/* -/-- */
// small arrows above numbers
    { 0x1a, KEY_NEXT },		/* also Fast Forward */
    { 0x0e, KEY_PREVIOUS },		/* also Rewind */
// these are in a rather non standard layout and have
    an alternate name written */
    { 0x1e, KEY_UP },		/* Video Setting */
    { 0x0a, KEY_DOWN },		/* Video Default */
    { 0x05, KEY_CAMERA },		/* Snapshot */
    { 0x0c, KEY_RIGHT },		/* Hide Panel */
// Four buttons without label
    { 0x49, KEY_RED },
    { 0x0b, KEY_GREEN },
    { 0x13, KEY_YELLOW },
    { 0x50, KEY_BLUE },
    };
    static struct rc_map_list genius_tvgo_a11mce_map = {
    .map = {
    .scan     = genius_tvgo_a11mce,
    .size     = ARRAY_SIZE(genius_tvgo_a11mce),
    .rc_proto = RC_PROTO_UNKNOWN,	/* Legacy IR type */
    .name     = RC_MAP_GENIUS_TVGO_A11MCE,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_genius_tvgo_a11mce() -> int __init {
    static int __init init_rc_map_genius_tvgo_a11mce(void)
    {
    return rc_map_register(&genius_tvgo_a11mce_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_genius_tvgo_a11mce() -> void __exit {
    static void __exit exit_rc_map_genius_tvgo_a11mce(void)
    {
    rc_map_unregister(&genius_tvgo_a11mce_map);
    }
    module_init(init_rc_map_genius_tvgo_a11mce)
    module_exit(exit_rc_map_genius_tvgo_a11mce)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Mauro Carvalho Chehab");
    MODULE_DESCRIPTION("Genius TVGO A11MCE remote controller keytable");
