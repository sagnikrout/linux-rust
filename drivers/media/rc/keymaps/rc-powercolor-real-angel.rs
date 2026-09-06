//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-powercolor-real-angel.c
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
// powercolor-real-angel.h - Keytable for powercolor_real_angel Remote Controller
//
// keymap imported from ir-keymaps.c
//
// Copyright (c) 2010 by Mauro Carvalho Chehab

//
// Remote control for Powercolor Real Angel 330
// Daniel Fraga <fragabr@gmail.com>
//
    static struct rc_map_table powercolor_real_angel[] = {
    { 0x38, KEY_SWITCHVIDEOMODE },	/* switch inputs */
    { 0x0c, KEY_MEDIA },		/* Turn ON/OFF App */
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
    { 0x0a, KEY_DIGITS },		/* single, double, triple digit */
    { 0x29, KEY_PREVIOUS },		/* previous channel */
    { 0x12, KEY_BRIGHTNESSUP },
    { 0x13, KEY_BRIGHTNESSDOWN },
    { 0x2b, KEY_MODE },		/* stereo/mono */
    { 0x2c, KEY_TEXT },		/* teletext */
    { 0x20, KEY_CHANNELUP },	/* channel up */
    { 0x21, KEY_CHANNELDOWN },	/* channel down */
    { 0x10, KEY_VOLUMEUP },		/* volume up */
    { 0x11, KEY_VOLUMEDOWN },	/* volume down */
    { 0x0d, KEY_MUTE },
    { 0x1f, KEY_RECORD },
    { 0x17, KEY_PLAY },
    { 0x16, KEY_PAUSE },
    { 0x0b, KEY_STOP },
    { 0x27, KEY_FASTFORWARD },
    { 0x26, KEY_REWIND },
    { 0x1e, KEY_SEARCH },		/* autoscan */
    { 0x0e, KEY_CAMERA },		/* snapshot */
    { 0x2d, KEY_SETUP },
    { 0x0f, KEY_SCREEN },		/* full screen */
    { 0x14, KEY_RADIO },		/* FM radio */
    { 0x25, KEY_POWER },		/* power */
    };
    static struct rc_map_list powercolor_real_angel_map = {
    .map = {
    .scan     = powercolor_real_angel,
    .size     = ARRAY_SIZE(powercolor_real_angel),
    .rc_proto = RC_PROTO_UNKNOWN,	/* Legacy IR type */
    .name     = RC_MAP_POWERCOLOR_REAL_ANGEL,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_powercolor_real_angel() -> int __init {
    static int __init init_rc_map_powercolor_real_angel(void)
    {
    return rc_map_register(&powercolor_real_angel_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_powercolor_real_angel() -> void __exit {
    static void __exit exit_rc_map_powercolor_real_angel(void)
    {
    rc_map_unregister(&powercolor_real_angel_map);
    }
    module_init(init_rc_map_powercolor_real_angel)
    module_exit(exit_rc_map_powercolor_real_angel)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Mauro Carvalho Chehab");
    MODULE_DESCRIPTION("Powercolor Real Angel 330 remote controller keytable");
