//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-vega-s9x.c
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
// Keytable for the Tronsmart Vega S9x remote control
//
    static struct rc_map_table vega_s9x[] = {
    { 0x18, KEY_POWER },
    { 0x17, KEY_MUTE }, // mouse
    { 0x46, KEY_UP },
    { 0x47, KEY_LEFT },
    { 0x55, KEY_OK },
    { 0x15, KEY_RIGHT },
    { 0x16, KEY_DOWN },
    { 0x06, KEY_HOME },
    { 0x42, KEY_PLAYPAUSE},
    { 0x40, KEY_BACK },
    { 0x14, KEY_VOLUMEDOWN },
    { 0x04, KEY_MENU },
    { 0x10, KEY_VOLUMEUP },
    };
    static struct rc_map_list vega_s9x_map = {
    .map = {
    .scan     = vega_s9x,
    .size     = ARRAY_SIZE(vega_s9x),
    .rc_proto = RC_PROTO_NEC,
    .name     = RC_MAP_VEGA_S9X,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_vega_s9x() -> int __init {
    static int __init init_rc_map_vega_s9x(void)
    {
    return rc_map_register(&vega_s9x_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_vega_s9x() -> void __exit {
    static void __exit exit_rc_map_vega_s9x(void)
    {
    rc_map_unregister(&vega_s9x_map);
    }
    module_init(init_rc_map_vega_s9x)
    module_exit(exit_rc_map_vega_s9x)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Christian Hewitt <christianshewitt@gmail.com");
    MODULE_DESCRIPTION("Tronsmart Vega S9x remote controller keytable");
