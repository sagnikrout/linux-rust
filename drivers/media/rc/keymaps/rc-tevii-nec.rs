//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-tevii-nec.c
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
// tevii-nec.h - Keytable for tevii_nec Remote Controller
//
// keymap imported from ir-keymaps.c
//
// Copyright (c) 2010 by Mauro Carvalho Chehab

    static struct rc_map_table tevii_nec[] = {
    { 0x0a, KEY_POWER2},
    { 0x0c, KEY_MUTE},
    { 0x11, KEY_NUMERIC_1},
    { 0x12, KEY_NUMERIC_2},
    { 0x13, KEY_NUMERIC_3},
    { 0x14, KEY_NUMERIC_4},
    { 0x15, KEY_NUMERIC_5},
    { 0x16, KEY_NUMERIC_6},
    { 0x17, KEY_NUMERIC_7},
    { 0x18, KEY_NUMERIC_8},
    { 0x19, KEY_NUMERIC_9},
    { 0x10, KEY_NUMERIC_0},
    { 0x1c, KEY_MENU},
    { 0x0f, KEY_VOLUMEDOWN},
    { 0x1a, KEY_LAST},
    { 0x0e, KEY_OPEN},
    { 0x04, KEY_RECORD},
    { 0x09, KEY_VOLUMEUP},
    { 0x08, KEY_CHANNELUP},
    { 0x07, KEY_PVR},
    { 0x0b, KEY_TIME},
    { 0x02, KEY_RIGHT},
    { 0x03, KEY_LEFT},
    { 0x00, KEY_UP},
    { 0x1f, KEY_OK},
    { 0x01, KEY_DOWN},
    { 0x05, KEY_TUNER},
    { 0x06, KEY_CHANNELDOWN},
    { 0x40, KEY_PLAYPAUSE},
    { 0x1e, KEY_REWIND},
    { 0x1b, KEY_FAVORITES},
    { 0x1d, KEY_BACK},
    { 0x4d, KEY_FASTFORWARD},
    { 0x44, KEY_EPG},
    { 0x4c, KEY_INFO},
    { 0x41, KEY_AB},
    { 0x43, KEY_AUDIO},
    { 0x45, KEY_SUBTITLE},
    { 0x4a, KEY_LIST},
    { 0x46, KEY_F1},
    { 0x47, KEY_F2},
    { 0x5e, KEY_F3},
    { 0x5c, KEY_F4},
    { 0x52, KEY_F5},
    { 0x5a, KEY_F6},
    { 0x56, KEY_MODE},
    { 0x58, KEY_SWITCHVIDEOMODE},
    };
    static struct rc_map_list tevii_nec_map = {
    .map = {
    .scan     = tevii_nec,
    .size     = ARRAY_SIZE(tevii_nec),
    .rc_proto = RC_PROTO_UNKNOWN,	/* Legacy IR type */
    .name     = RC_MAP_TEVII_NEC,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_tevii_nec() -> int __init {
    static int __init init_rc_map_tevii_nec(void)
    {
    return rc_map_register(&tevii_nec_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_tevii_nec() -> void __exit {
    static void __exit exit_rc_map_tevii_nec(void)
    {
    rc_map_unregister(&tevii_nec_map);
    }
    module_init(init_rc_map_tevii_nec)
    module_exit(exit_rc_map_tevii_nec)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Mauro Carvalho Chehab");
    MODULE_DESCRIPTION("tevii-nec remote controller keytable");
