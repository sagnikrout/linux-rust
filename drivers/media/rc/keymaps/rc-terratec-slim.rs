//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-terratec-slim.c
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
// TerraTec remote controller keytable
//
// Copyright (C) 2010 Antti Palosaari <crope@iki.fi>
//

// TerraTec slim remote, 7 rows, 4 columns.
// Uses NEC extended 0x02bd.
    static struct rc_map_table terratec_slim[] = {
    { 0x02bd00, KEY_NUMERIC_1 },
    { 0x02bd01, KEY_NUMERIC_2 },
    { 0x02bd02, KEY_NUMERIC_3 },
    { 0x02bd03, KEY_NUMERIC_4 },
    { 0x02bd04, KEY_NUMERIC_5 },
    { 0x02bd05, KEY_NUMERIC_6 },
    { 0x02bd06, KEY_NUMERIC_7 },
    { 0x02bd07, KEY_NUMERIC_8 },
    { 0x02bd08, KEY_NUMERIC_9 },
    { 0x02bd09, KEY_NUMERIC_0 },
    { 0x02bd0a, KEY_MUTE },
    { 0x02bd0b, KEY_NEW },             /* symbol: PIP */
    { 0x02bd0e, KEY_VOLUMEDOWN },
    { 0x02bd0f, KEY_PLAYPAUSE },
    { 0x02bd10, KEY_RIGHT },
    { 0x02bd11, KEY_LEFT },
    { 0x02bd12, KEY_UP },
    { 0x02bd13, KEY_DOWN },
    { 0x02bd15, KEY_OK },
    { 0x02bd16, KEY_STOP },
    { 0x02bd17, KEY_CAMERA },          /* snapshot */
    { 0x02bd18, KEY_CHANNELUP },
    { 0x02bd19, KEY_RECORD },
    { 0x02bd1a, KEY_CHANNELDOWN },
    { 0x02bd1c, KEY_ESC },
    { 0x02bd1f, KEY_VOLUMEUP },
    { 0x02bd44, KEY_EPG },
    { 0x02bd45, KEY_POWER2 },          /* [red power button] */
    };
    static struct rc_map_list terratec_slim_map = {
    .map = {
    .scan     = terratec_slim,
    .size     = ARRAY_SIZE(terratec_slim),
    .rc_proto = RC_PROTO_NECX,
    .name     = RC_MAP_TERRATEC_SLIM,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_terratec_slim() -> int __init {
    static int __init init_rc_map_terratec_slim(void)
    {
    return rc_map_register(&terratec_slim_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_terratec_slim() -> void __exit {
    static void __exit exit_rc_map_terratec_slim(void)
    {
    rc_map_unregister(&terratec_slim_map);
    }
    module_init(init_rc_map_terratec_slim)
    module_exit(exit_rc_map_terratec_slim)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Antti Palosaari <crope@iki.fi>");
    MODULE_DESCRIPTION("TerraTec slim remote controller keytable");
