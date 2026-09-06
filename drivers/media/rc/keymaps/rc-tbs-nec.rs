//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-tbs-nec.c
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
// tbs-nec.h - Keytable for tbs_nec Remote Controller
//
// keymap imported from ir-keymaps.c
//
// Copyright (c) 2010 by Mauro Carvalho Chehab

    static struct rc_map_table tbs_nec[] = {
    { 0x84, KEY_POWER2},		/* power */
    { 0x94, KEY_MUTE},		/* mute */
    { 0x87, KEY_NUMERIC_1},
    { 0x86, KEY_NUMERIC_2},
    { 0x85, KEY_NUMERIC_3},
    { 0x8b, KEY_NUMERIC_4},
    { 0x8a, KEY_NUMERIC_5},
    { 0x89, KEY_NUMERIC_6},
    { 0x8f, KEY_NUMERIC_7},
    { 0x8e, KEY_NUMERIC_8},
    { 0x8d, KEY_NUMERIC_9},
    { 0x92, KEY_NUMERIC_0},
    { 0xc0, KEY_10CHANNELSUP},	/* 10+ */
    { 0xd0, KEY_10CHANNELSDOWN},	/* 10- */
    { 0x96, KEY_CHANNELUP},		/* ch+ */
    { 0x91, KEY_CHANNELDOWN},	/* ch- */
    { 0x93, KEY_VOLUMEUP},		/* vol+ */
    { 0x8c, KEY_VOLUMEDOWN},	/* vol- */
    { 0x83, KEY_RECORD},		/* rec */
    { 0x98, KEY_PAUSE},		/* pause, yellow */
    { 0x99, KEY_OK},		/* ok */
    { 0x9a, KEY_CAMERA},		/* snapshot */
    { 0x81, KEY_UP},
    { 0x90, KEY_LEFT},
    { 0x82, KEY_RIGHT},
    { 0x88, KEY_DOWN},
    { 0x95, KEY_FAVORITES},		/* blue */
    { 0x97, KEY_SUBTITLE},		/* green */
    { 0x9d, KEY_ZOOM},
    { 0x9f, KEY_EXIT},
    { 0x9e, KEY_MENU},
    { 0x9c, KEY_EPG},
    { 0x80, KEY_PREVIOUS},		/* red */
    { 0x9b, KEY_MODE},
    };
    static struct rc_map_list tbs_nec_map = {
    .map = {
    .scan     = tbs_nec,
    .size     = ARRAY_SIZE(tbs_nec),
    .rc_proto = RC_PROTO_UNKNOWN,	/* Legacy IR type */
    .name     = RC_MAP_TBS_NEC,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_tbs_nec() -> int __init {
    static int __init init_rc_map_tbs_nec(void)
    {
    return rc_map_register(&tbs_nec_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_tbs_nec() -> void __exit {
    static void __exit exit_rc_map_tbs_nec(void)
    {
    rc_map_unregister(&tbs_nec_map);
    }
    module_init(init_rc_map_tbs_nec)
    module_exit(exit_rc_map_tbs_nec)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Mauro Carvalho Chehab");
    MODULE_DESCRIPTION("tbs-nec remote controller keytable");
