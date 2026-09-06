//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-pv951.c
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
// pv951.h - Keytable for pv951 Remote Controller
//
// keymap imported from ir-keymaps.c
//
// Copyright (c) 2010 by Mauro Carvalho Chehab

// Mark Phalan <phalanm@o2.ie>
    static struct rc_map_table pv951[] = {
    { 0x00, KEY_NUMERIC_0 },
    { 0x01, KEY_NUMERIC_1 },
    { 0x02, KEY_NUMERIC_2 },
    { 0x03, KEY_NUMERIC_3 },
    { 0x04, KEY_NUMERIC_4 },
    { 0x05, KEY_NUMERIC_5 },
    { 0x06, KEY_NUMERIC_6 },
    { 0x07, KEY_NUMERIC_7 },
    { 0x08, KEY_NUMERIC_8 },
    { 0x09, KEY_NUMERIC_9 },
    { 0x12, KEY_POWER },
    { 0x10, KEY_MUTE },
    { 0x1f, KEY_VOLUMEDOWN },
    { 0x1b, KEY_VOLUMEUP },
    { 0x1a, KEY_CHANNELUP },
    { 0x1e, KEY_CHANNELDOWN },
    { 0x0e, KEY_PAGEUP },
    { 0x1d, KEY_PAGEDOWN },
    { 0x13, KEY_SOUND },
    { 0x18, KEY_KPPLUSMINUS },	/* CH +/- */
    { 0x16, KEY_SUBTITLE },		/* CC */
    { 0x0d, KEY_TEXT },		/* TTX */
    { 0x0b, KEY_TV },		/* AIR/CBL */
    { 0x11, KEY_PC },		/* PC/TV */
    { 0x17, KEY_OK },		/* CH RTN */
    { 0x19, KEY_MODE },		/* FUNC */
    { 0x0c, KEY_SEARCH },		/* AUTOSCAN */
// Not sure what to do with these ones!
    { 0x0f, KEY_VIDEO },		/* SOURCE */
    { 0x0a, KEY_KPPLUS },		/* +100 */
    { 0x14, KEY_EQUAL },		/* SYNC */
    { 0x1c, KEY_TV },		/* PC/TV */
    };
    static struct rc_map_list pv951_map = {
    .map = {
    .scan     = pv951,
    .size     = ARRAY_SIZE(pv951),
    .rc_proto = RC_PROTO_UNKNOWN,	/* Legacy IR type */
    .name     = RC_MAP_PV951,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_pv951() -> int __init {
    static int __init init_rc_map_pv951(void)
    {
    return rc_map_register(&pv951_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_pv951() -> void __exit {
    static void __exit exit_rc_map_pv951(void)
    {
    rc_map_unregister(&pv951_map);
    }
    module_init(init_rc_map_pv951)
    module_exit(exit_rc_map_pv951)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Mauro Carvalho Chehab");
    MODULE_DESCRIPTION("pv951 remote controller keytable");
