//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-geekbox.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Keytable for the GeekBox remote controller
//
// Copyright (C) 2017 Martin Blumenstingl <martin.blumenstingl@googlemail.com>
//

    static struct rc_map_table geekbox[] = {
    { 0x01, KEY_BACK },
    { 0x02, KEY_DOWN },
    { 0x03, KEY_UP },
    { 0x07, KEY_OK },
    { 0x0b, KEY_VOLUMEUP },
    { 0x0e, KEY_LEFT },
    { 0x13, KEY_MENU },
    { 0x14, KEY_POWER },
    { 0x1a, KEY_RIGHT },
    { 0x48, KEY_HOME },
    { 0x58, KEY_VOLUMEDOWN },
    { 0x5c, KEY_SCREEN },
    };
    static struct rc_map_list geekbox_map = {
    .map = {
    .scan     = geekbox,
    .size     = ARRAY_SIZE(geekbox),
    .rc_proto = RC_PROTO_NEC,
    .name     = RC_MAP_GEEKBOX,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_geekbox() -> int __init {
    static int __init init_rc_map_geekbox(void)
    {
    return rc_map_register(&geekbox_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_geekbox() -> void __exit {
    static void __exit exit_rc_map_geekbox(void)
    {
    rc_map_unregister(&geekbox_map);
    }
    module_init(init_rc_map_geekbox)
    module_exit(exit_rc_map_geekbox)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Martin Blumenstingl <martin.blumenstingl@googlemail.com>");
    MODULE_DESCRIPTION("GeekBox remote controller keytable");
