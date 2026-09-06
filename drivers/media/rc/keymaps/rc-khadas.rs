//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-khadas.c
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
//
// Copyright (C) 2019 Christian Hewitt <christianshewitt@gmail.com>
//
// Keytable for the Khadas VIM/EDGE SBC remote control
//

    static struct rc_map_table khadas[] = {
    { 0x14, KEY_POWER },
    { 0x03, KEY_UP },
    { 0x02, KEY_DOWN },
    { 0x0e, KEY_LEFT },
    { 0x1a, KEY_RIGHT },
    { 0x07, KEY_OK },
    { 0x01, KEY_BACK },
    { 0x5b, KEY_MUTE }, // mouse
    { 0x13, KEY_MENU },
    { 0x58, KEY_VOLUMEDOWN },
    { 0x0b, KEY_VOLUMEUP },
    { 0x48, KEY_HOME },
    };
    static struct rc_map_list khadas_map = {
    .map = {
    .scan     = khadas,
    .size     = ARRAY_SIZE(khadas),
    .rc_proto = RC_PROTO_NEC,
    .name     = RC_MAP_KHADAS,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_khadas() -> int __init {
    static int __init init_rc_map_khadas(void)
    {
    return rc_map_register(&khadas_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_khadas() -> void __exit {
    static void __exit exit_rc_map_khadas(void)
    {
    rc_map_unregister(&khadas_map);
    }
    module_init(init_rc_map_khadas)
    module_exit(exit_rc_map_khadas)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Christian Hewitt <christianshewitt@gmail.com>");
    MODULE_DESCRIPTION("Khadas VIM/EDGE SBC remote controller keytable");
