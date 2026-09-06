//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-avermedia-cardbus.c
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
// avermedia-cardbus.h - Keytable for avermedia_cardbus Remote Controller
//
// keymap imported from ir-keymaps.c
//
// Copyright (c) 2010 by Mauro Carvalho Chehab

// Oldrich Jedlicka <oldium.pro@seznam.cz>
    static struct rc_map_table avermedia_cardbus[] = {
    { 0x00, KEY_POWER },
    { 0x01, KEY_TUNER },		/* TV/FM */
    { 0x03, KEY_TEXT },		/* Teletext */
    { 0x04, KEY_EPG },
    { 0x05, KEY_NUMERIC_1 },
    { 0x06, KEY_NUMERIC_2 },
    { 0x07, KEY_NUMERIC_3 },
    { 0x08, KEY_AUDIO },
    { 0x09, KEY_NUMERIC_4 },
    { 0x0a, KEY_NUMERIC_5 },
    { 0x0b, KEY_NUMERIC_6 },
    { 0x0c, KEY_ZOOM },		/* Full screen */
    { 0x0d, KEY_NUMERIC_7 },
    { 0x0e, KEY_NUMERIC_8 },
    { 0x0f, KEY_NUMERIC_9 },
    { 0x10, KEY_PAGEUP },		/* 16-CH PREV */
    { 0x11, KEY_NUMERIC_0 },
    { 0x12, KEY_INFO },
    { 0x13, KEY_AGAIN },		/* CH RTN - channel return */
    { 0x14, KEY_MUTE },
    { 0x15, KEY_EDIT },		/* Autoscan */
    { 0x17, KEY_SAVE },		/* Screenshot */
    { 0x18, KEY_PLAYPAUSE },
    { 0x19, KEY_RECORD },
    { 0x1a, KEY_PLAY },
    { 0x1b, KEY_STOP },
    { 0x1c, KEY_FASTFORWARD },
    { 0x1d, KEY_REWIND },
    { 0x1e, KEY_VOLUMEDOWN },
    { 0x1f, KEY_VOLUMEUP },
    { 0x22, KEY_SLEEP },		/* Sleep */
    { 0x23, KEY_ZOOM },		/* Aspect */
    { 0x26, KEY_SCREEN },		/* Pos */
    { 0x27, KEY_ANGLE },		/* Size */
    { 0x28, KEY_SELECT },		/* Select */
    { 0x29, KEY_BLUE },		/* Blue/Picture */
    { 0x2a, KEY_BACKSPACE },	/* Back */
    { 0x2b, KEY_VIDEO },		/* PIP (Picture-in-picture) */
    { 0x2c, KEY_DOWN },
    { 0x2e, KEY_DOT },
    { 0x2f, KEY_TV },		/* Live TV */
    { 0x32, KEY_LEFT },
    { 0x33, KEY_CLEAR },		/* Clear */
    { 0x35, KEY_RED },		/* Red/TV */
    { 0x36, KEY_UP },
    { 0x37, KEY_HOME },		/* Home */
    { 0x39, KEY_GREEN },		/* Green/Video */
    { 0x3d, KEY_YELLOW },		/* Yellow/Music */
    { 0x3e, KEY_OK },		/* Ok */
    { 0x3f, KEY_RIGHT },
    { 0x40, KEY_NEXT },		/* Next */
    { 0x41, KEY_PREVIOUS },		/* Previous */
    { 0x42, KEY_CHANNELDOWN },	/* Channel down */
    { 0x43, KEY_CHANNELUP },	/* Channel up */
    };
    static struct rc_map_list avermedia_cardbus_map = {
    .map = {
    .scan     = avermedia_cardbus,
    .size     = ARRAY_SIZE(avermedia_cardbus),
    .rc_proto = RC_PROTO_UNKNOWN,	/* Legacy IR type */
    .name     = RC_MAP_AVERMEDIA_CARDBUS,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_avermedia_cardbus() -> int __init {
    static int __init init_rc_map_avermedia_cardbus(void)
    {
    return rc_map_register(&avermedia_cardbus_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_avermedia_cardbus() -> void __exit {
    static void __exit exit_rc_map_avermedia_cardbus(void)
    {
    rc_map_unregister(&avermedia_cardbus_map);
    }
    module_init(init_rc_map_avermedia_cardbus)
    module_exit(exit_rc_map_avermedia_cardbus)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Mauro Carvalho Chehab");
    MODULE_DESCRIPTION("avermedia-cardbus remote controller keytable");
