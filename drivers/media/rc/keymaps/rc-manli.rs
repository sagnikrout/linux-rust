//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-manli.c
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
// manli.h - Keytable for manli Remote Controller
//
// keymap imported from ir-keymaps.c
//
// Copyright (c) 2010 by Mauro Carvalho Chehab

// Michael Tokarev <mjt@tls.msk.ru>
    keytable is used by MANLI MTV00[0x0c] and BeholdTV 40[13] at
    least, and probably other cards too.
    The "ascii-art picture" below (in comments, first row
    is the keycode in hex, and subsequent row(s) shows
#[no_mangle]
pub unsafe extern "C" fn labels(appropriate: several variants when) -> the button {
    the button labels (several variants when appropriate)
    helps to decide which keycodes to assign to the buttons.
//
    static struct rc_map_table manli[] = {
// 0x1c            0x12
// FUNCTION         POWER
// FM              (|)
//
    { 0x1c, KEY_RADIO },	/*XXX*/
    { 0x12, KEY_POWER },
// 0x01    0x02    0x03
// 1       2       3
//
// 0x04    0x05    0x06
// 4       5       6
//
// 0x07    0x08    0x09
// 7       8       9
//
    { 0x01, KEY_NUMERIC_1 },
    { 0x02, KEY_NUMERIC_2 },
    { 0x03, KEY_NUMERIC_3 },
    { 0x04, KEY_NUMERIC_4 },
    { 0x05, KEY_NUMERIC_5 },
    { 0x06, KEY_NUMERIC_6 },
    { 0x07, KEY_NUMERIC_7 },
    { 0x08, KEY_NUMERIC_8 },
    { 0x09, KEY_NUMERIC_9 },
// 0x0a    0x00    0x17
// RECALL    0      +100
// PLUS
//
    { 0x0a, KEY_AGAIN },	/*XXX KEY_REWIND? */
    { 0x00, KEY_NUMERIC_0 },
    { 0x17, KEY_DIGITS },	/*XXX*/
// 0x14            0x10
// MENU            INFO
// OSD
    { 0x14, KEY_MENU },
    { 0x10, KEY_INFO },
// 0x0b
// Up
//
// 0x18    0x16    0x0c
// Left     Ok     Right
//
// 0x015
// Down
//
    { 0x0b, KEY_UP },
    { 0x18, KEY_LEFT },
    { 0x16, KEY_OK },	/*XXX KEY_SELECT? KEY_ENTER? */
    { 0x0c, KEY_RIGHT },
    { 0x15, KEY_DOWN },
// 0x11            0x0d
// TV/AV           MODE
// SOURCE         STEREO
//
    { 0x11, KEY_TV },	/*XXX*/
    { 0x0d, KEY_MODE },	/*XXX there's no KEY_STEREO	*/
// 0x0f    0x1b    0x1a
// AUDIO   Vol+    Chan+
// TIMESHIFT???
//
// 0x0e    0x1f    0x1e
// SLEEP   Vol-    Chan-
//
    { 0x0f, KEY_AUDIO },
    { 0x1b, KEY_VOLUMEUP },
    { 0x1a, KEY_CHANNELUP },
    { 0x0e, KEY_TIME },
    { 0x1f, KEY_VOLUMEDOWN },
    { 0x1e, KEY_CHANNELDOWN },
// 0x13     0x19
// MUTE   SNAPSHOT
//
    { 0x13, KEY_MUTE },
    { 0x19, KEY_CAMERA },
// 0x1d unused ?
    };
    static struct rc_map_list manli_map = {
    .map = {
    .scan     = manli,
    .size     = ARRAY_SIZE(manli),
    .rc_proto = RC_PROTO_UNKNOWN,	/* Legacy IR type */
    .name     = RC_MAP_MANLI,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_manli() -> int __init {
    static int __init init_rc_map_manli(void)
    {
    return rc_map_register(&manli_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_manli() -> void __exit {
    static void __exit exit_rc_map_manli(void)
    {
    rc_map_unregister(&manli_map);
    }
    module_init(init_rc_map_manli)
    module_exit(exit_rc_map_manli)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Mauro Carvalho Chehab");
    MODULE_DESCRIPTION("MANLI MTV00[0x0c] and BeholdTV 40[13] remote controller keytable");
