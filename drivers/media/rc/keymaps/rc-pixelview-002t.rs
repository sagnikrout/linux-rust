//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-pixelview-002t.c
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
// rc-pixelview-mk12.h - Keytable for pixelview Remote Controller
//
// keymap imported from ir-keymaps.c
//
// Copyright (c) 2010 by Mauro Carvalho Chehab

//
// Keytable for 002-T IR remote provided together with Pixelview
// SBTVD Hybrid Remote Controller. Uses NEC extended format.
//
    static struct rc_map_table pixelview_002t[] = {
    { 0x866b13, KEY_MUTE },
    { 0x866b12, KEY_POWER2 },	/* power */
    { 0x866b01, KEY_NUMERIC_1 },
    { 0x866b02, KEY_NUMERIC_2 },
    { 0x866b03, KEY_NUMERIC_3 },
    { 0x866b04, KEY_NUMERIC_4 },
    { 0x866b05, KEY_NUMERIC_5 },
    { 0x866b06, KEY_NUMERIC_6 },
    { 0x866b07, KEY_NUMERIC_7 },
    { 0x866b08, KEY_NUMERIC_8 },
    { 0x866b09, KEY_NUMERIC_9 },
    { 0x866b00, KEY_NUMERIC_0 },
    { 0x866b0d, KEY_CHANNELUP },
    { 0x866b19, KEY_CHANNELDOWN },
    { 0x866b10, KEY_VOLUMEUP },	/* vol + */
    { 0x866b0c, KEY_VOLUMEDOWN },	/* vol - */
    { 0x866b0a, KEY_CAMERA },	/* snapshot */
    { 0x866b0b, KEY_ZOOM },		/* zoom */
    { 0x866b1b, KEY_BACKSPACE },
    { 0x866b15, KEY_ENTER },
    { 0x866b1d, KEY_UP },
    { 0x866b1e, KEY_DOWN },
    { 0x866b0e, KEY_LEFT },
    { 0x866b0f, KEY_RIGHT },
    { 0x866b18, KEY_RECORD },
    { 0x866b1a, KEY_STOP },
    };
    static struct rc_map_list pixelview_map = {
    .map = {
    .scan     = pixelview_002t,
    .size     = ARRAY_SIZE(pixelview_002t),
    .rc_proto = RC_PROTO_NECX,
    .name     = RC_MAP_PIXELVIEW_002T,
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
    MODULE_DESCRIPTION("002-T IR remote keytable");
