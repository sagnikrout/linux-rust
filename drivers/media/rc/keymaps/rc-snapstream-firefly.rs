//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-snapstream-firefly.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// SnapStream Firefly X10 RF remote keytable
//
// Copyright (C) 2011 Anssi Hannula <anssi.hannula@?ki.fi>
//

    static struct rc_map_table snapstream_firefly[] = {
    { 0x2c, KEY_ZOOM },       /* Maximize */
    { 0x02, KEY_CLOSE },
    { 0x0d, KEY_NUMERIC_1 },
    { 0x0e, KEY_NUMERIC_2 },
    { 0x0f, KEY_NUMERIC_3 },
    { 0x10, KEY_NUMERIC_4 },
    { 0x11, KEY_NUMERIC_5 },
    { 0x12, KEY_NUMERIC_6 },
    { 0x13, KEY_NUMERIC_7 },
    { 0x14, KEY_NUMERIC_8 },
    { 0x15, KEY_NUMERIC_9 },
    { 0x17, KEY_NUMERIC_0 },
    { 0x16, KEY_BACK },
    { 0x18, KEY_KPENTER },    /* ent */
    { 0x09, KEY_VOLUMEUP },
    { 0x08, KEY_VOLUMEDOWN },
    { 0x0a, KEY_MUTE },
    { 0x0b, KEY_CHANNELUP },
    { 0x0c, KEY_CHANNELDOWN },
    { 0x00, KEY_VENDOR },     /* firefly */
    { 0x2e, KEY_INFO },
    { 0x2f, KEY_OPTION },
    { 0x1d, KEY_LEFT },
    { 0x1f, KEY_RIGHT },
    { 0x22, KEY_DOWN },
    { 0x1a, KEY_UP },
    { 0x1e, KEY_OK },
    { 0x1c, KEY_MENU },
    { 0x20, KEY_EXIT },
    { 0x27, KEY_RECORD },
    { 0x25, KEY_PLAY },
    { 0x28, KEY_STOP },
    { 0x24, KEY_REWIND },
    { 0x26, KEY_FORWARD },
    { 0x29, KEY_PAUSE },
    { 0x2b, KEY_PREVIOUS },
    { 0x2a, KEY_NEXT },
    { 0x06, KEY_AUDIO },      /* Music */
    { 0x05, KEY_IMAGES },     /* Photos */
    { 0x04, KEY_DVD },
    { 0x03, KEY_TV },
    { 0x07, KEY_VIDEO },
    { 0x01, KEY_HELP },
    { 0x2d, KEY_MODE },       /* Mouse */
    { 0x19, KEY_A },
    { 0x1b, KEY_B },
    { 0x21, KEY_C },
    { 0x23, KEY_D },
    };
    static struct rc_map_list snapstream_firefly_map = {
    .map = {
    .scan     = snapstream_firefly,
    .size     = ARRAY_SIZE(snapstream_firefly),
    .rc_proto = RC_PROTO_OTHER,
    .name     = RC_MAP_SNAPSTREAM_FIREFLY,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_snapstream_firefly() -> int __init {
    static int __init init_rc_map_snapstream_firefly(void)
    {
    return rc_map_register(&snapstream_firefly_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_snapstream_firefly() -> void __exit {
    static void __exit exit_rc_map_snapstream_firefly(void)
    {
    rc_map_unregister(&snapstream_firefly_map);
    }
    module_init(init_rc_map_snapstream_firefly)
    module_exit(exit_rc_map_snapstream_firefly)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Anssi Hannula <anssi.hannula@iki.fi>");
    MODULE_DESCRIPTION("SnapStream Firefly X10 RF remote controller keytable");
