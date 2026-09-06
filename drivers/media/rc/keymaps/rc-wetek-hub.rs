//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-wetek-hub.c
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
// Copyright (c) 2018 Christian Hewitt

//
// This keymap is used with the WeTek Hub STB.
//
    static struct rc_map_table wetek_hub[] = {
    { 0x77f1, KEY_POWER },
    { 0x77f2, KEY_HOME },
    { 0x77f3, KEY_MUTE }, // mouse
    { 0x77f4, KEY_UP },
    { 0x77f5, KEY_DOWN },
    { 0x77f6, KEY_LEFT },
    { 0x77f7, KEY_RIGHT },
    { 0x77f8, KEY_OK },
    { 0x77f9, KEY_BACK },
    { 0x77fa, KEY_MENU },
    { 0x77fb, KEY_VOLUMEUP },
    { 0x77fc, KEY_VOLUMEDOWN },
    };
    static struct rc_map_list wetek_hub_map = {
    .map = {
    .scan     = wetek_hub,
    .size     = ARRAY_SIZE(wetek_hub),
    .rc_proto = RC_PROTO_NEC,
    .name     = RC_MAP_WETEK_HUB,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_wetek_hub() -> int __init {
    static int __init init_rc_map_wetek_hub(void)
    {
    return rc_map_register(&wetek_hub_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_wetek_hub() -> void __exit {
    static void __exit exit_rc_map_wetek_hub(void)
    {
    rc_map_unregister(&wetek_hub_map);
    }
    module_init(init_rc_map_wetek_hub)
    module_exit(exit_rc_map_wetek_hub)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Christian Hewitt <christianshewitt@gmail.com>");
    MODULE_DESCRIPTION("WeTek Hub STB remote controller keytable");
