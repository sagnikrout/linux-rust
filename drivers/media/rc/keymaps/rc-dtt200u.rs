//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-dtt200u.c
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
// Keytable for Wideview WT-220U.
//
// Copyright (c) 2016 Jonathan McDowell <noodles@earth.li>
//

// key list for the tiny remote control (Yakumo, don't know about the others)
    static struct rc_map_table dtt200u_table[] = {
    { 0x8001, KEY_MUTE },
    { 0x8002, KEY_CHANNELDOWN },
    { 0x8003, KEY_VOLUMEDOWN },
    { 0x8004, KEY_NUMERIC_1 },
    { 0x8005, KEY_NUMERIC_2 },
    { 0x8006, KEY_NUMERIC_3 },
    { 0x8007, KEY_NUMERIC_4 },
    { 0x8008, KEY_NUMERIC_5 },
    { 0x8009, KEY_NUMERIC_6 },
    { 0x800a, KEY_NUMERIC_7 },
    { 0x800c, KEY_ZOOM },
    { 0x800d, KEY_NUMERIC_0 },
    { 0x800e, KEY_SELECT },
    { 0x8012, KEY_POWER },
    { 0x801a, KEY_CHANNELUP },
    { 0x801b, KEY_NUMERIC_8 },
    { 0x801e, KEY_VOLUMEUP },
    { 0x801f, KEY_NUMERIC_9 },
    };
    static struct rc_map_list dtt200u_map = {
    .map = {
    .scan     = dtt200u_table,
    .size     = ARRAY_SIZE(dtt200u_table),
    .rc_proto = RC_PROTO_NEC,
    .name     = RC_MAP_DTT200U,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_dtt200u() -> int __init {
    static int __init init_rc_map_dtt200u(void)
    {
    return rc_map_register(&dtt200u_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_dtt200u() -> void __exit {
    static void __exit exit_rc_map_dtt200u(void)
    {
    rc_map_unregister(&dtt200u_map);
    }
    module_init(init_rc_map_dtt200u)
    module_exit(exit_rc_map_dtt200u)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Jonathan McDowell <noodles@earth.li>");
    MODULE_DESCRIPTION("Wideview WT-220U remote controller keytable");
