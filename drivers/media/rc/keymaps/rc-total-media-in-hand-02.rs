//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-total-media-in-hand-02.c
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
// Total Media In Hand_02 remote controller keytable for Mygica X8507
//
// Copyright (C) 2012 Alfredo J. Delaiti <alfredodelaiti@netscape.net>
//

    static struct rc_map_table total_media_in_hand_02[] = {
    { 0x0000, KEY_NUMERIC_0 },
    { 0x0001, KEY_NUMERIC_1 },
    { 0x0002, KEY_NUMERIC_2 },
    { 0x0003, KEY_NUMERIC_3 },
    { 0x0004, KEY_NUMERIC_4 },
    { 0x0005, KEY_NUMERIC_5 },
    { 0x0006, KEY_NUMERIC_6 },
    { 0x0007, KEY_NUMERIC_7 },
    { 0x0008, KEY_NUMERIC_8 },
    { 0x0009, KEY_NUMERIC_9 },
    { 0x000a, KEY_MUTE },
    { 0x000b, KEY_STOP },                   /* Stop */
    { 0x000c, KEY_POWER2 },                 /* Turn on/off application */
    { 0x000d, KEY_OK },                     /* OK */
    { 0x000e, KEY_CAMERA },                 /* Snapshot */
    { 0x000f, KEY_ZOOM },                   /* Full Screen/Restore */
    { 0x0010, KEY_RIGHT },                  /* Right arrow */
    { 0x0011, KEY_LEFT },                   /* Left arrow */
    { 0x0012, KEY_CHANNELUP },
    { 0x0013, KEY_CHANNELDOWN },
    { 0x0014, KEY_SHUFFLE },
    { 0x0016, KEY_PAUSE },
    { 0x0017, KEY_PLAY },                   /* Play */
    { 0x001e, KEY_TIME },                   /* Time Shift */
    { 0x001f, KEY_RECORD },
    { 0x0020, KEY_UP },
    { 0x0021, KEY_DOWN },
    { 0x0025, KEY_POWER },                  /* Turn off computer */
    { 0x0026, KEY_REWIND },                 /* FR << */
    { 0x0027, KEY_FASTFORWARD },            /* FF >> */
    { 0x0029, KEY_ESC },
    { 0x002b, KEY_VOLUMEUP },
    { 0x002c, KEY_VOLUMEDOWN },
    { 0x002d, KEY_CHANNEL },                /* CH Surfing */
    { 0x0038, KEY_VIDEO },                  /* TV/AV/S-Video/YPbPr */
    };
    static struct rc_map_list total_media_in_hand_02_map = {
    .map = {
    .scan     = total_media_in_hand_02,
    .size     = ARRAY_SIZE(total_media_in_hand_02),
    .rc_proto = RC_PROTO_RC5,
    .name     = RC_MAP_TOTAL_MEDIA_IN_HAND_02,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_total_media_in_hand_02() -> int __init {
    static int __init init_rc_map_total_media_in_hand_02(void)
    {
    return rc_map_register(&total_media_in_hand_02_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_total_media_in_hand_02() -> void __exit {
    static void __exit exit_rc_map_total_media_in_hand_02(void)
    {
    rc_map_unregister(&total_media_in_hand_02_map);
    }
    module_init(init_rc_map_total_media_in_hand_02)
    module_exit(exit_rc_map_total_media_in_hand_02)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR(" Alfredo J. Delaiti <alfredodelaiti@netscape.net>");
    MODULE_DESCRIPTION("Total Media In Hand_02 remote controller keytable");
