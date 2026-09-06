//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-minix-neo.c
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
// Copyright (C) 2021 Christian Hewitt <christianshewitt@gmail.com>

//
// Keytable for the Minix NEO remote control
//
    static struct rc_map_table minix_neo[] = {
    { 0x118, KEY_POWER },
    { 0x146, KEY_UP },
    { 0x116, KEY_DOWN },
    { 0x147, KEY_LEFT },
    { 0x115, KEY_RIGHT },
    { 0x155, KEY_ENTER },
    { 0x110, KEY_VOLUMEDOWN },
    { 0x140, KEY_BACK },
    { 0x114, KEY_VOLUMEUP },
    { 0x10d, KEY_HOME },
    { 0x104, KEY_MENU },
    { 0x112, KEY_CONFIG },
    };
    static struct rc_map_list minix_neo_map = {
    .map = {
    .scan     = minix_neo,
    .size     = ARRAY_SIZE(minix_neo),
    .rc_proto = RC_PROTO_NEC,
    .name     = RC_MAP_MINIX_NEO,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_minix_neo() -> int __init {
    static int __init init_rc_map_minix_neo(void)
    {
    return rc_map_register(&minix_neo_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_minix_neo() -> void __exit {
    static void __exit exit_rc_map_minix_neo(void)
    {
    rc_map_unregister(&minix_neo_map);
    }
    module_init(init_rc_map_minix_neo)
    module_exit(exit_rc_map_minix_neo)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Christian Hewitt <christianshewitt@gmail.com");
    MODULE_DESCRIPTION("Minix NEO remote controller keytable");
