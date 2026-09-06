//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-winfast-usbii-deluxe.c
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
// winfast-usbii-deluxe.h - Keytable for winfast_usbii_deluxe Remote Controller
//
// keymap imported from ir-keymaps.c
//
// Copyright (c) 2010 by Mauro Carvalho Chehab

// Leadtek Winfast TV USB II Deluxe remote
    Magnus Alm <magnus.alm@gmail.com>
//
    static struct rc_map_table winfast_usbii_deluxe[] = {
    { 0x62, KEY_NUMERIC_0},
    { 0x75, KEY_NUMERIC_1},
    { 0x76, KEY_NUMERIC_2},
    { 0x77, KEY_NUMERIC_3},
    { 0x79, KEY_NUMERIC_4},
    { 0x7a, KEY_NUMERIC_5},
    { 0x7b, KEY_NUMERIC_6},
    { 0x7d, KEY_NUMERIC_7},
    { 0x7e, KEY_NUMERIC_8},
    { 0x7f, KEY_NUMERIC_9},
    { 0x38, KEY_CAMERA},		/* SNAPSHOT */
    { 0x37, KEY_RECORD},		/* RECORD */
    { 0x35, KEY_TIME},		/* TIMESHIFT */
    { 0x74, KEY_VOLUMEUP},		/* VOLUMEUP */
    { 0x78, KEY_VOLUMEDOWN},	/* VOLUMEDOWN */
    { 0x64, KEY_MUTE},		/* MUTE */
    { 0x21, KEY_CHANNEL},		/* SURF */
    { 0x7c, KEY_CHANNELUP},		/* CHANNELUP */
    { 0x60, KEY_CHANNELDOWN},	/* CHANNELDOWN */
    { 0x61, KEY_LAST},		/* LAST CHANNEL (RECALL) */
    { 0x72, KEY_VIDEO},		/* INPUT MODES (TV/FM) */
    { 0x70, KEY_POWER2},		/* TV ON/OFF */
    { 0x39, KEY_CYCLEWINDOWS},	/* MINIMIZE (BOSS) */
    { 0x3a, KEY_NEW},		/* PIP */
    { 0x73, KEY_ZOOM},		/* FULLSECREEN */
    { 0x66, KEY_INFO},		/* OSD (DISPLAY) */
    { 0x31, KEY_DOT},		/* '.' */
    { 0x63, KEY_ENTER},		/* ENTER */
    };
    static struct rc_map_list winfast_usbii_deluxe_map = {
    .map = {
    .scan     = winfast_usbii_deluxe,
    .size     = ARRAY_SIZE(winfast_usbii_deluxe),
    .rc_proto = RC_PROTO_UNKNOWN,	/* Legacy IR type */
    .name     = RC_MAP_WINFAST_USBII_DELUXE,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_winfast_usbii_deluxe() -> int __init {
    static int __init init_rc_map_winfast_usbii_deluxe(void)
    {
    return rc_map_register(&winfast_usbii_deluxe_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_winfast_usbii_deluxe() -> void __exit {
    static void __exit exit_rc_map_winfast_usbii_deluxe(void)
    {
    rc_map_unregister(&winfast_usbii_deluxe_map);
    }
    module_init(init_rc_map_winfast_usbii_deluxe)
    module_exit(exit_rc_map_winfast_usbii_deluxe)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Mauro Carvalho Chehab");
    MODULE_DESCRIPTION("Leadtek Winfast TV USB II Deluxe remote controller keytable");
