//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-apac-viewcomp.c
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
// apac-viewcomp.h - Keytable for apac_viewcomp Remote Controller
//
// keymap imported from ir-keymaps.c
//
// Copyright (c) 2010 by Mauro Carvalho Chehab

// Attila Kondoros <attila.kondoros@chello.hu>
    static struct rc_map_table apac_viewcomp[] = {
    { 0x01, KEY_NUMERIC_1 },
    { 0x02, KEY_NUMERIC_2 },
    { 0x03, KEY_NUMERIC_3 },
    { 0x04, KEY_NUMERIC_4 },
    { 0x05, KEY_NUMERIC_5 },
    { 0x06, KEY_NUMERIC_6 },
    { 0x07, KEY_NUMERIC_7 },
    { 0x08, KEY_NUMERIC_8 },
    { 0x09, KEY_NUMERIC_9 },
    { 0x00, KEY_NUMERIC_0 },
    { 0x17, KEY_LAST },		/* +100 */
    { 0x0a, KEY_LIST },		/* recall */
    { 0x1c, KEY_TUNER },		/* TV/FM */
    { 0x15, KEY_SEARCH },		/* scan */
    { 0x12, KEY_POWER },		/* power */
    { 0x1f, KEY_VOLUMEDOWN },	/* vol up */
    { 0x1b, KEY_VOLUMEUP },		/* vol down */
    { 0x1e, KEY_CHANNELDOWN },	/* chn up */
    { 0x1a, KEY_CHANNELUP },	/* chn down */
    { 0x11, KEY_VIDEO },		/* video */
    { 0x0f, KEY_ZOOM },		/* full screen */
    { 0x13, KEY_MUTE },		/* mute/unmute */
    { 0x10, KEY_TEXT },		/* min */
    { 0x0d, KEY_STOP },		/* freeze */
    { 0x0e, KEY_RECORD },		/* record */
    { 0x1d, KEY_PLAYPAUSE },	/* stop */
    { 0x19, KEY_PLAY },		/* play */
    { 0x16, KEY_GOTO },		/* osd */
    { 0x14, KEY_REFRESH },		/* default */
    { 0x0c, KEY_KPPLUS },		/* fine tune >>>> */
    { 0x18, KEY_KPMINUS },		/* fine tune <<<< */
    };
    static struct rc_map_list apac_viewcomp_map = {
    .map = {
    .scan     = apac_viewcomp,
    .size     = ARRAY_SIZE(apac_viewcomp),
    .rc_proto = RC_PROTO_UNKNOWN,	/* Legacy IR type */
    .name     = RC_MAP_APAC_VIEWCOMP,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_apac_viewcomp() -> int __init {
    static int __init init_rc_map_apac_viewcomp(void)
    {
    return rc_map_register(&apac_viewcomp_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_apac_viewcomp() -> void __exit {
    static void __exit exit_rc_map_apac_viewcomp(void)
    {
    rc_map_unregister(&apac_viewcomp_map);
    }
    module_init(init_rc_map_apac_viewcomp)
    module_exit(exit_rc_map_apac_viewcomp)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Mauro Carvalho Chehab");
    MODULE_DESCRIPTION("apac-viewcomp remote controller keytable");
