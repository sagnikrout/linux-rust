//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-dvbsky.c
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
// rc-dvbsky.c - Keytable for DVBSky Remote Controllers
//
// keymap imported from ir-keymaps.c
//
// Copyright (c) 2010-2012 by Nibble Max <nibble.max@gmail.com>
//

//
// This table contains the complete RC5 code, instead of just the data part
//
    static struct rc_map_table rc5_dvbsky[] = {
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
    { 0x000d, KEY_OK },
    { 0x000b, KEY_STOP },
    { 0x000c, KEY_EXIT },
    { 0x000e, KEY_CAMERA }, /*Snap shot*/
    { 0x000f, KEY_SUBTITLE }, /*PIP*/
    { 0x0010, KEY_VOLUMEUP },
    { 0x0011, KEY_VOLUMEDOWN },
    { 0x0012, KEY_FAVORITES },
    { 0x0013, KEY_LIST }, /*Info*/
    { 0x0016, KEY_PAUSE },
    { 0x0017, KEY_PLAY },
    { 0x001f, KEY_RECORD },
    { 0x0020, KEY_CHANNELDOWN },
    { 0x0021, KEY_CHANNELUP },
    { 0x0025, KEY_POWER2 },
    { 0x0026, KEY_REWIND },
    { 0x0027, KEY_FASTFORWARD },
    { 0x0029, KEY_LAST },
    { 0x002b, KEY_MENU },
    { 0x002c, KEY_EPG },
    { 0x002d, KEY_ZOOM },
    };
    static struct rc_map_list rc5_dvbsky_map = {
    .map = {
    .scan     = rc5_dvbsky,
    .size     = ARRAY_SIZE(rc5_dvbsky),
    .rc_proto = RC_PROTO_RC5,
    .name     = RC_MAP_DVBSKY,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_rc5_dvbsky() -> int __init {
    static int __init init_rc_map_rc5_dvbsky(void)
    {
    return rc_map_register(&rc5_dvbsky_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_rc5_dvbsky() -> void __exit {
    static void __exit exit_rc_map_rc5_dvbsky(void)
    {
    rc_map_unregister(&rc5_dvbsky_map);
    }
    module_init(init_rc_map_rc5_dvbsky)
    module_exit(exit_rc_map_rc5_dvbsky)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Nibble Max <nibble.max@gmail.com>");
    MODULE_DESCRIPTION("DVBSky remote controller keytable");
