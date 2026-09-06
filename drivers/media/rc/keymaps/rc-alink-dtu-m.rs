//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-alink-dtu-m.c
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
// A-Link DTU(m) remote controller keytable
//
// Copyright (C) 2010 Antti Palosaari <crope@iki.fi>
//

// A-Link DTU(m) slim remote, 6 rows, 3 columns.
    static struct rc_map_table alink_dtu_m[] = {
    { 0x0800, KEY_VOLUMEUP },
    { 0x0801, KEY_NUMERIC_1 },
    { 0x0802, KEY_NUMERIC_3 },
    { 0x0803, KEY_NUMERIC_7 },
    { 0x0804, KEY_NUMERIC_9 },
    { 0x0805, KEY_NEW },             /* symbol: PIP */
    { 0x0806, KEY_NUMERIC_0 },
    { 0x0807, KEY_CHANNEL },         /* JUMP */
    { 0x080d, KEY_NUMERIC_5 },
    { 0x080f, KEY_NUMERIC_2 },
    { 0x0812, KEY_POWER2 },
    { 0x0814, KEY_CHANNELUP },
    { 0x0816, KEY_VOLUMEDOWN },
    { 0x0818, KEY_NUMERIC_6 },
    { 0x081a, KEY_MUTE },
    { 0x081b, KEY_NUMERIC_8 },
    { 0x081c, KEY_NUMERIC_4 },
    { 0x081d, KEY_CHANNELDOWN },
    };
    static struct rc_map_list alink_dtu_m_map = {
    .map = {
    .scan     = alink_dtu_m,
    .size     = ARRAY_SIZE(alink_dtu_m),
    .rc_proto = RC_PROTO_NEC,
    .name     = RC_MAP_ALINK_DTU_M,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_alink_dtu_m() -> int __init {
    static int __init init_rc_map_alink_dtu_m(void)
    {
    return rc_map_register(&alink_dtu_m_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_alink_dtu_m() -> void __exit {
    static void __exit exit_rc_map_alink_dtu_m(void)
    {
    rc_map_unregister(&alink_dtu_m_map);
    }
    module_init(init_rc_map_alink_dtu_m)
    module_exit(exit_rc_map_alink_dtu_m)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Antti Palosaari <crope@iki.fi>");
    MODULE_DESCRIPTION("A-Link DTU(m) slim remote, 6 rows, 3 columns.");
