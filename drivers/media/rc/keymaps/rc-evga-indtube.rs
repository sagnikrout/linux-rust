//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-evga-indtube.c
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
// evga-indtube.h - Keytable for evga_indtube Remote Controller
//
// keymap imported from ir-keymaps.c
//
// Copyright (c) 2010 by Mauro Carvalho Chehab

// EVGA inDtube
    Devin Heitmueller <devin.heitmueller@gmail.com>
//
    static struct rc_map_table evga_indtube[] = {
    { 0x12, KEY_POWER},
    { 0x02, KEY_MODE},	/* TV */
    { 0x14, KEY_MUTE},
    { 0x1a, KEY_CHANNELUP},
    { 0x16, KEY_TV2},	/* PIP */
    { 0x1d, KEY_VOLUMEUP},
    { 0x05, KEY_CHANNELDOWN},
    { 0x0f, KEY_PLAYPAUSE},
    { 0x19, KEY_VOLUMEDOWN},
    { 0x1c, KEY_REWIND},
    { 0x0d, KEY_RECORD},
    { 0x18, KEY_FORWARD},
    { 0x1e, KEY_PREVIOUS},
    { 0x1b, KEY_STOP},
    { 0x1f, KEY_NEXT},
    { 0x13, KEY_CAMERA},
    };
    static struct rc_map_list evga_indtube_map = {
    .map = {
    .scan     = evga_indtube,
    .size     = ARRAY_SIZE(evga_indtube),
    .rc_proto = RC_PROTO_UNKNOWN,	/* Legacy IR type */
    .name     = RC_MAP_EVGA_INDTUBE,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_evga_indtube() -> int __init {
    static int __init init_rc_map_evga_indtube(void)
    {
    return rc_map_register(&evga_indtube_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_evga_indtube() -> void __exit {
    static void __exit exit_rc_map_evga_indtube(void)
    {
    rc_map_unregister(&evga_indtube_map);
    }
    module_init(init_rc_map_evga_indtube)
    module_exit(exit_rc_map_evga_indtube)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Mauro Carvalho Chehab");
    MODULE_DESCRIPTION("EVGA inDtube remote controller keytable");
