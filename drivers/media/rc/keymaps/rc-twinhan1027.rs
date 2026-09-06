//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-twinhan1027.c
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


// SPDX-License-Identifier: GPL-2.0-only

    static struct rc_map_table twinhan_vp1027[] = {
    { 0x16, KEY_POWER2 },
    { 0x17, KEY_FAVORITES },
    { 0x0f, KEY_TEXT },
    { 0x48, KEY_INFO},
    { 0x1c, KEY_EPG },
    { 0x04, KEY_LIST },
    { 0x03, KEY_NUMERIC_1 },
    { 0x01, KEY_NUMERIC_2 },
    { 0x06, KEY_NUMERIC_3 },
    { 0x09, KEY_NUMERIC_4 },
    { 0x1d, KEY_NUMERIC_5 },
    { 0x1f, KEY_NUMERIC_6 },
    { 0x0d, KEY_NUMERIC_7 },
    { 0x19, KEY_NUMERIC_8 },
    { 0x1b, KEY_NUMERIC_9 },
    { 0x15, KEY_NUMERIC_0 },
    { 0x0c, KEY_CANCEL },
    { 0x4a, KEY_CLEAR },
    { 0x13, KEY_BACKSPACE },
    { 0x00, KEY_TAB },
    { 0x4b, KEY_UP },
    { 0x51, KEY_DOWN },
    { 0x4e, KEY_LEFT },
    { 0x52, KEY_RIGHT },
    { 0x4f, KEY_ENTER },
    { 0x1e, KEY_VOLUMEUP },
    { 0x0a, KEY_VOLUMEDOWN },
    { 0x02, KEY_CHANNELDOWN },
    { 0x05, KEY_CHANNELUP },
    { 0x11, KEY_RECORD },
    { 0x14, KEY_PLAY },
    { 0x4c, KEY_PAUSE },
    { 0x1a, KEY_STOP },
    { 0x40, KEY_REWIND },
    { 0x12, KEY_FASTFORWARD },
    { 0x41, KEY_PREVIOUSSONG },
    { 0x42, KEY_NEXTSONG },
    { 0x54, KEY_SAVE },
    { 0x50, KEY_LANGUAGE },
    { 0x47, KEY_MEDIA },
    { 0x4d, KEY_SCREEN },
    { 0x43, KEY_SUBTITLE },
    { 0x10, KEY_MUTE },
    { 0x49, KEY_AUDIO },
    { 0x07, KEY_SLEEP },
    { 0x08, KEY_VIDEO },
    { 0x0e, KEY_AGAIN },
    { 0x45, KEY_EQUAL },
    { 0x46, KEY_MINUS },
    { 0x18, KEY_RED },
    { 0x53, KEY_GREEN },
    { 0x5e, KEY_YELLOW },
    { 0x5f, KEY_BLUE },
    };
    static struct rc_map_list twinhan_vp1027_map = {
    .map = {
    .scan     = twinhan_vp1027,
    .size     = ARRAY_SIZE(twinhan_vp1027),
    .rc_proto = RC_PROTO_NEC,
    .name     = RC_MAP_TWINHAN_VP1027_DVBS,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_twinhan_vp1027() -> int __init {
    static int __init init_rc_map_twinhan_vp1027(void)
    {
    return rc_map_register(&twinhan_vp1027_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_twinhan_vp1027() -> void __exit {
    static void __exit exit_rc_map_twinhan_vp1027(void)
    {
    rc_map_unregister(&twinhan_vp1027_map);
    }
    module_init(init_rc_map_twinhan_vp1027)
    module_exit(exit_rc_map_twinhan_vp1027)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Sergey Ivanov <123kash@gmail.com>");
    MODULE_DESCRIPTION("twinhan1027 remote controller keytable");
