//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-videomate-tv-pvr.c
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
// videomate-tv-pvr.h - Keytable for videomate_tv_pvr Remote Controller
//
// keymap imported from ir-keymaps.c
//
// Copyright (c) 2010 by Mauro Carvalho Chehab

    static struct rc_map_table videomate_tv_pvr[] = {
    { 0x14, KEY_MUTE },
    { 0x24, KEY_ZOOM },
    { 0x01, KEY_DVD },
    { 0x23, KEY_RADIO },
    { 0x00, KEY_TV },
    { 0x0a, KEY_REWIND },
    { 0x08, KEY_PLAYPAUSE },
    { 0x0f, KEY_FORWARD },
    { 0x02, KEY_PREVIOUS },
    { 0x07, KEY_STOP },
    { 0x06, KEY_NEXT },
    { 0x0c, KEY_UP },
    { 0x0e, KEY_DOWN },
    { 0x0b, KEY_LEFT },
    { 0x0d, KEY_RIGHT },
    { 0x11, KEY_OK },
    { 0x03, KEY_MENU },
    { 0x09, KEY_SETUP },
    { 0x05, KEY_VIDEO },
    { 0x22, KEY_CHANNEL },
    { 0x12, KEY_VOLUMEUP },
    { 0x15, KEY_VOLUMEDOWN },
    { 0x10, KEY_CHANNELUP },
    { 0x13, KEY_CHANNELDOWN },
    { 0x04, KEY_RECORD },
    { 0x16, KEY_NUMERIC_1 },
    { 0x17, KEY_NUMERIC_2 },
    { 0x18, KEY_NUMERIC_3 },
    { 0x19, KEY_NUMERIC_4 },
    { 0x1a, KEY_NUMERIC_5 },
    { 0x1b, KEY_NUMERIC_6 },
    { 0x1c, KEY_NUMERIC_7 },
    { 0x1d, KEY_NUMERIC_8 },
    { 0x1e, KEY_NUMERIC_9 },
    { 0x1f, KEY_NUMERIC_0 },
    { 0x20, KEY_LANGUAGE },
    { 0x21, KEY_SLEEP },
    };
    static struct rc_map_list videomate_tv_pvr_map = {
    .map = {
    .scan     = videomate_tv_pvr,
    .size     = ARRAY_SIZE(videomate_tv_pvr),
    .rc_proto = RC_PROTO_UNKNOWN,	/* Legacy IR type */
    .name     = RC_MAP_VIDEOMATE_TV_PVR,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_videomate_tv_pvr() -> int __init {
    static int __init init_rc_map_videomate_tv_pvr(void)
    {
    return rc_map_register(&videomate_tv_pvr_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_videomate_tv_pvr() -> void __exit {
    static void __exit exit_rc_map_videomate_tv_pvr(void)
    {
    rc_map_unregister(&videomate_tv_pvr_map);
    }
    module_init(init_rc_map_videomate_tv_pvr)
    module_exit(exit_rc_map_videomate_tv_pvr)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Mauro Carvalho Chehab");
    MODULE_DESCRIPTION("videomate-tv-pvr remote controller keytable");
