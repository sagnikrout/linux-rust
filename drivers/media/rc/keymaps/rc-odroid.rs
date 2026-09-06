//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-odroid.c
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
// Keytable for the HardKernel ODROID remote control
//
    static struct rc_map_table odroid[] = {
    { 0xb2dc, KEY_POWER },
    { 0xb288, KEY_MUTE },
    { 0xb282, KEY_HOME },
    { 0xb2ca, KEY_UP },
    { 0xb299, KEY_LEFT },
    { 0xb2ce, KEY_OK },
    { 0xb2c1, KEY_RIGHT },
    { 0xb2d2, KEY_DOWN },
    { 0xb2c5, KEY_MENU },
    { 0xb29a, KEY_BACK },
    { 0xb281, KEY_VOLUMEDOWN },
    { 0xb280, KEY_VOLUMEUP },
    };
    static struct rc_map_list odroid_map = {
    .map = {
    .scan     = odroid,
    .size     = ARRAY_SIZE(odroid),
    .rc_proto = RC_PROTO_NEC,
    .name     = RC_MAP_ODROID,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_odroid() -> int __init {
    static int __init init_rc_map_odroid(void)
    {
    return rc_map_register(&odroid_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_odroid() -> void __exit {
    static void __exit exit_rc_map_odroid(void)
    {
    rc_map_unregister(&odroid_map);
    }
    module_init(init_rc_map_odroid)
    module_exit(exit_rc_map_odroid)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Christian Hewitt <christianshewitt@gmail.com");
    MODULE_DESCRIPTION("HardKernel ODROID remote controller keytable");
