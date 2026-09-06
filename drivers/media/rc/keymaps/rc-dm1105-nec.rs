//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-dm1105-nec.c
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
// dm1105-nec.h - Keytable for dm1105_nec Remote Controller
//
// keymap imported from ir-keymaps.c
//
// Copyright (c) 2010 by Mauro Carvalho Chehab

// DVBWorld remotes
    Igor M. Liplianin <liplianin@me.by>
//
    static struct rc_map_table dm1105_nec[] = {
    { 0x0a, KEY_POWER2},		/* power */
    { 0x0c, KEY_MUTE},		/* mute */
    { 0x11, KEY_NUMERIC_1},
    { 0x12, KEY_NUMERIC_2},
    { 0x13, KEY_NUMERIC_3},
    { 0x14, KEY_NUMERIC_4},
    { 0x15, KEY_NUMERIC_5},
    { 0x16, KEY_NUMERIC_6},
    { 0x17, KEY_NUMERIC_7},
    { 0x18, KEY_NUMERIC_8},
    { 0x19, KEY_NUMERIC_9},
    { 0x10, KEY_NUMERIC_0},
    { 0x1c, KEY_CHANNELUP},		/* ch+ */
    { 0x0f, KEY_CHANNELDOWN},	/* ch- */
    { 0x1a, KEY_VOLUMEUP},		/* vol+ */
    { 0x0e, KEY_VOLUMEDOWN},	/* vol- */
    { 0x04, KEY_RECORD},		/* rec */
    { 0x09, KEY_CHANNEL},		/* fav */
    { 0x08, KEY_BACKSPACE},		/* rewind */
    { 0x07, KEY_FASTFORWARD},	/* fast */
    { 0x0b, KEY_PAUSE},		/* pause */
    { 0x02, KEY_ESC},		/* cancel */
    { 0x03, KEY_TAB},		/* tab */
    { 0x00, KEY_UP},		/* up */
    { 0x1f, KEY_ENTER},		/* ok */
    { 0x01, KEY_DOWN},		/* down */
    { 0x05, KEY_RECORD},		/* cap */
    { 0x06, KEY_STOP},		/* stop */
    { 0x40, KEY_ZOOM},		/* full */
    { 0x1e, KEY_TV},		/* tvmode */
    { 0x1b, KEY_B},			/* recall */
    };
    static struct rc_map_list dm1105_nec_map = {
    .map = {
    .scan     = dm1105_nec,
    .size     = ARRAY_SIZE(dm1105_nec),
    .rc_proto = RC_PROTO_UNKNOWN,	/* Legacy IR type */
    .name     = RC_MAP_DM1105_NEC,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_dm1105_nec() -> int __init {
    static int __init init_rc_map_dm1105_nec(void)
    {
    return rc_map_register(&dm1105_nec_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_dm1105_nec() -> void __exit {
    static void __exit exit_rc_map_dm1105_nec(void)
    {
    rc_map_unregister(&dm1105_nec_map);
    }
    module_init(init_rc_map_dm1105_nec)
    module_exit(exit_rc_map_dm1105_nec)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Mauro Carvalho Chehab");
    MODULE_DESCRIPTION("dm1105-nec remote controller keytable");
