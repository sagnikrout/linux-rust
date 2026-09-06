//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-leadtek-y04g0051.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// LeadTek Y04G0051 remote controller keytable
//
// Copyright (C) 2010 Antti Palosaari <crope@iki.fi>
//

    static struct rc_map_table leadtek_y04g0051[] = {
    { 0x0300, KEY_POWER2 },
    { 0x0303, KEY_SCREEN },
    { 0x0304, KEY_RIGHT },
    { 0x0305, KEY_NUMERIC_1 },
    { 0x0306, KEY_NUMERIC_2 },
    { 0x0307, KEY_NUMERIC_3 },
    { 0x0308, KEY_LEFT },
    { 0x0309, KEY_NUMERIC_4 },
    { 0x030a, KEY_NUMERIC_5 },
    { 0x030b, KEY_NUMERIC_6 },
    { 0x030c, KEY_UP },
    { 0x030d, KEY_NUMERIC_7 },
    { 0x030e, KEY_NUMERIC_8 },
    { 0x030f, KEY_NUMERIC_9 },
    { 0x0310, KEY_DOWN },
    { 0x0311, KEY_AGAIN },
    { 0x0312, KEY_NUMERIC_0 },
    { 0x0313, KEY_OK },              /* 1st ok */
    { 0x0314, KEY_MUTE },
    { 0x0316, KEY_OK },              /* 2nd ok */
    { 0x031e, KEY_VIDEO },           /* 2nd video */
    { 0x031b, KEY_AUDIO },
    { 0x031f, KEY_TEXT },
    { 0x0340, KEY_SLEEP },
    { 0x0341, KEY_DOT },
    { 0x0342, KEY_REWIND },
    { 0x0343, KEY_PLAY },
    { 0x0344, KEY_FASTFORWARD },
    { 0x0345, KEY_TIME },
    { 0x0346, KEY_STOP },            /* 2nd stop */
    { 0x0347, KEY_RECORD },
    { 0x0348, KEY_CAMERA },
    { 0x0349, KEY_ESC },
    { 0x034a, KEY_NEW },
    { 0x034b, KEY_RED },
    { 0x034c, KEY_GREEN },
    { 0x034d, KEY_YELLOW },
    { 0x034e, KEY_BLUE },
    { 0x034f, KEY_MENU },
    { 0x0350, KEY_STOP },            /* 1st stop */
    { 0x0351, KEY_CHANNEL },
    { 0x0352, KEY_VIDEO },           /* 1st video */
    { 0x0353, KEY_EPG },
    { 0x0354, KEY_PREVIOUS },
    { 0x0355, KEY_NEXT },
    { 0x0356, KEY_TV },
    { 0x035a, KEY_VOLUMEDOWN },
    { 0x035b, KEY_CHANNELUP },
    { 0x035e, KEY_VOLUMEUP },
    { 0x035f, KEY_CHANNELDOWN },
    };
    static struct rc_map_list leadtek_y04g0051_map = {
    .map = {
    .scan     = leadtek_y04g0051,
    .size     = ARRAY_SIZE(leadtek_y04g0051),
    .rc_proto = RC_PROTO_NEC,
    .name     = RC_MAP_LEADTEK_Y04G0051,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_leadtek_y04g0051() -> int __init {
    static int __init init_rc_map_leadtek_y04g0051(void)
    {
    return rc_map_register(&leadtek_y04g0051_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_leadtek_y04g0051() -> void __exit {
    static void __exit exit_rc_map_leadtek_y04g0051(void)
    {
    rc_map_unregister(&leadtek_y04g0051_map);
    }
    module_init(init_rc_map_leadtek_y04g0051)
    module_exit(exit_rc_map_leadtek_y04g0051)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Antti Palosaari <crope@iki.fi>");
    MODULE_DESCRIPTION("LeadTek Y04G0051 remote controller keytable");
