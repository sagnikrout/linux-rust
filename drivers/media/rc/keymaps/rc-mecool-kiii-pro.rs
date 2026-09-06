//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-mecool-kiii-pro.c
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
//
// Copyright (C) 2021 Christian Hewitt <christianshewitt@gmail.com>

//
// Keytable for the Mecool Kiii Pro remote control
//
    static struct rc_map_table mecool_kiii_pro[] = {
    { 0x59, KEY_POWER },
    { 0x52, KEY_1 },
    { 0x50, KEY_2 },
    { 0x10, KEY_3 },
    { 0x56, KEY_4 },
    { 0x54, KEY_5 },
    { 0x14, KEY_6 },
    { 0x4e, KEY_7 },
    { 0x4c, KEY_8 },
    { 0x0c, KEY_9 },
    { 0x02, KEY_INFO },
    { 0x0f, KEY_0 },
    { 0x51, KEY_DELETE },
    { 0x1f, KEY_FAVORITES},
    { 0x09, KEY_SUBTITLE },
    { 0x01, KEY_LANGUAGE }, // AUDIO
    { 0x42, KEY_RED },
    { 0x40, KEY_GREEN },
    { 0x00, KEY_YELLOW},
    { 0x03, KEY_BLUE }, // RADIO
    { 0x0d, KEY_HOME },
    { 0x4d, KEY_EPG },
    { 0x45, KEY_MENU },
    { 0x05, KEY_EXIT },
    { 0x5a, KEY_LEFT },
    { 0x1b, KEY_RIGHT },
    { 0x06, KEY_UP },
    { 0x16, KEY_DOWN },
    { 0x1a, KEY_OK },
    { 0x13, KEY_VOLUMEUP },
    { 0x17, KEY_VOLUMEDOWN },
    { 0x19, KEY_MUTE },
    { 0x12, KEY_CONTEXT_MENU }, // MOUSE
    { 0x55, KEY_CHANNELUP }, // PAGE_UP
    { 0x15, KEY_CHANNELDOWN }, // PAGE_DOWN
    { 0x4a, KEY_REWIND },
    { 0x48, KEY_FORWARD },
    { 0x46, KEY_PLAYPAUSE },
    { 0x44, KEY_STOP },
    { 0x08, KEY_PREVIOUSSONG},
    { 0x0b, KEY_NEXTSONG},
    { 0x04, KEY_PVR },
    { 0x64, KEY_RECORD },
    };
    static struct rc_map_list mecool_kiii_pro_map = {
    .map = {
    .scan     = mecool_kiii_pro,
    .size     = ARRAY_SIZE(mecool_kiii_pro),
    .rc_proto = RC_PROTO_NEC,
    .name     = RC_MAP_MECOOL_KIII_PRO,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_mecool_kiii_pro() -> int __init {
    static int __init init_rc_map_mecool_kiii_pro(void)
    {
    return rc_map_register(&mecool_kiii_pro_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_mecool_kiii_pro() -> void __exit {
    static void __exit exit_rc_map_mecool_kiii_pro(void)
    {
    rc_map_unregister(&mecool_kiii_pro_map);
    }
    module_init(init_rc_map_mecool_kiii_pro)
    module_exit(exit_rc_map_mecool_kiii_pro)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Christian Hewitt <christianshewitt@gmail.com");
    MODULE_DESCRIPTION("Mecool Kiii Pro remote controller keytable");
