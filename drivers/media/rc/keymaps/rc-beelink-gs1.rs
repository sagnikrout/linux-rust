//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-beelink-gs1.c
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
// Copyright (c) 2019 Clément Péron

//
// Keymap for the Beelink GS1 remote control
//
    static struct rc_map_table beelink_gs1_table[] = {
//
// TV Keys (Power, Learn and Volume)
// { 0x40400d, KEY_TV },
// { 0x80f1, KEY_TV },
// { 0x80f3, KEY_TV },
// { 0x80f4, KEY_TV },
//
    { 0x8051, KEY_POWER },
    { 0x804d, KEY_MUTE },
    { 0x8040, KEY_CONFIG },
    { 0x8026, KEY_UP },
    { 0x8028, KEY_DOWN },
    { 0x8025, KEY_LEFT },
    { 0x8027, KEY_RIGHT },
    { 0x800d, KEY_OK },
    { 0x8053, KEY_HOME },
    { 0x80bc, KEY_MEDIA },
    { 0x801b, KEY_BACK },
    { 0x8049, KEY_MENU },
    { 0x804e, KEY_VOLUMEUP },
    { 0x8056, KEY_VOLUMEDOWN },
    { 0x8054, KEY_SUBTITLE }, /* Web */
    { 0x8052, KEY_EPG }, /* Media */
    { 0x8041, KEY_CHANNELUP },
    { 0x8042, KEY_CHANNELDOWN },
    { 0x8031, KEY_1 },
    { 0x8032, KEY_2 },
    { 0x8033, KEY_3 },
    { 0x8034, KEY_4 },
    { 0x8035, KEY_5 },
    { 0x8036, KEY_6 },
    { 0x8037, KEY_7 },
    { 0x8038, KEY_8 },
    { 0x8039, KEY_9 },
    { 0x8044, KEY_DELETE },
    { 0x8030, KEY_0 },
    { 0x8058, KEY_MODE }, /* # Input Method */
    };
    static struct rc_map_list beelink_gs1_map = {
    .map = {
    .scan     = beelink_gs1_table,
    .size     = ARRAY_SIZE(beelink_gs1_table),
    .rc_proto = RC_PROTO_NEC,
    .name     = RC_MAP_BEELINK_GS1,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_beelink_gs1() -> int __init {
    static int __init init_rc_map_beelink_gs1(void)
    {
    return rc_map_register(&beelink_gs1_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_beelink_gs1() -> void __exit {
    static void __exit exit_rc_map_beelink_gs1(void)
    {
    rc_map_unregister(&beelink_gs1_map);
    }
    module_init(init_rc_map_beelink_gs1)
    module_exit(exit_rc_map_beelink_gs1)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Clément Péron <peron.clem@gmail.com>");
    MODULE_DESCRIPTION("Beelink GS1 remote controller keytable");
