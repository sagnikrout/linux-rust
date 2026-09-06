//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-hisi-tv-demo.c
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
// Keytable for remote controller of HiSilicon tv demo board.
//
// Copyright (c) 2017 HiSilicon Technologies Co., Ltd.
//

    static struct rc_map_table hisi_tv_demo_keymap[] = {
    { 0x00000092, KEY_NUMERIC_1},
    { 0x00000093, KEY_NUMERIC_2},
    { 0x000000cc, KEY_NUMERIC_3},
    { 0x0000009f, KEY_NUMERIC_4},
    { 0x0000008e, KEY_NUMERIC_5},
    { 0x0000008f, KEY_NUMERIC_6},
    { 0x000000c8, KEY_NUMERIC_7},
    { 0x00000094, KEY_NUMERIC_8},
    { 0x0000008a, KEY_NUMERIC_9},
    { 0x0000008b, KEY_NUMERIC_0},
    { 0x000000ce, KEY_ENTER},
    { 0x000000ca, KEY_UP},
    { 0x00000099, KEY_LEFT},
    { 0x00000084, KEY_PAGEUP},
    { 0x000000c1, KEY_RIGHT},
    { 0x000000d2, KEY_DOWN},
    { 0x00000089, KEY_PAGEDOWN},
    { 0x000000d1, KEY_MUTE},
    { 0x00000098, KEY_VOLUMEDOWN},
    { 0x00000090, KEY_VOLUMEUP},
    { 0x0000009c, KEY_POWER},
    { 0x000000d6, KEY_STOP},
    { 0x00000097, KEY_MENU},
    { 0x000000cb, KEY_BACK},
    { 0x000000da, KEY_PLAYPAUSE},
    { 0x00000080, KEY_INFO},
    { 0x000000c3, KEY_REWIND},
    { 0x00000087, KEY_HOMEPAGE},
    { 0x000000d0, KEY_FASTFORWARD},
    { 0x000000c4, KEY_SOUND},
    { 0x00000082, BTN_1},
    { 0x000000c7, BTN_2},
    { 0x00000086, KEY_PROGRAM},
    { 0x000000d9, KEY_SUBTITLE},
    { 0x00000085, KEY_ZOOM},
    { 0x0000009b, KEY_RED},
    { 0x0000009a, KEY_GREEN},
    { 0x000000c0, KEY_YELLOW},
    { 0x000000c2, KEY_BLUE},
    { 0x0000009d, KEY_CHANNELDOWN},
    { 0x000000cf, KEY_CHANNELUP},
    };
    static struct rc_map_list hisi_tv_demo_map = {
    .map = {
    .scan	  = hisi_tv_demo_keymap,
    .size	  = ARRAY_SIZE(hisi_tv_demo_keymap),
    .rc_proto = RC_PROTO_NEC,
    .name	  = RC_MAP_HISI_TV_DEMO,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_hisi_tv_demo() -> int __init {
    static int __init init_rc_map_hisi_tv_demo(void)
    {
    return rc_map_register(&hisi_tv_demo_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_hisi_tv_demo() -> void __exit {
    static void __exit exit_rc_map_hisi_tv_demo(void)
    {
    rc_map_unregister(&hisi_tv_demo_map);
    }
    module_init(init_rc_map_hisi_tv_demo)
    module_exit(exit_rc_map_hisi_tv_demo)
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("HiSilicon tv demo remote controller keytable");
