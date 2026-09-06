//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-zx-irdec.c
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
//
// Copyright (C) 2017 Sanechips Technology Co., Ltd.
// Copyright 2017 Linaro Ltd.
//

    static struct rc_map_table zx_irdec_table[] = {
    { 0x01, KEY_NUMERIC_1 },
    { 0x02, KEY_NUMERIC_2 },
    { 0x03, KEY_NUMERIC_3 },
    { 0x04, KEY_NUMERIC_4 },
    { 0x05, KEY_NUMERIC_5 },
    { 0x06, KEY_NUMERIC_6 },
    { 0x07, KEY_NUMERIC_7 },
    { 0x08, KEY_NUMERIC_8 },
    { 0x09, KEY_NUMERIC_9 },
    { 0x31, KEY_NUMERIC_0 },
    { 0x16, KEY_DELETE },
    { 0x0a, KEY_MODE },		/* Input method */
    { 0x0c, KEY_VOLUMEUP },
    { 0x18, KEY_VOLUMEDOWN },
    { 0x0b, KEY_CHANNELUP },
    { 0x15, KEY_CHANNELDOWN },
    { 0x0d, KEY_PAGEUP },
    { 0x13, KEY_PAGEDOWN },
    { 0x46, KEY_FASTFORWARD },
    { 0x43, KEY_REWIND },
    { 0x44, KEY_PLAYPAUSE },
    { 0x45, KEY_STOP },
    { 0x49, KEY_OK },
    { 0x47, KEY_UP },
    { 0x4b, KEY_DOWN },
    { 0x48, KEY_LEFT },
    { 0x4a, KEY_RIGHT },
    { 0x4d, KEY_MENU },
    { 0x56, KEY_APPSELECT },	/* Application */
    { 0x4c, KEY_BACK },
    { 0x1e, KEY_INFO },
    { 0x4e, KEY_F1 },
    { 0x4f, KEY_F2 },
    { 0x50, KEY_F3 },
    { 0x51, KEY_F4 },
    { 0x1c, KEY_AUDIO },
    { 0x12, KEY_MUTE },
    { 0x11, KEY_DOT },		/* Location */
    { 0x1d, KEY_SETUP },
    { 0x40, KEY_POWER },
    };
    static struct rc_map_list zx_irdec_map = {
    .map = {
    .scan = zx_irdec_table,
    .size = ARRAY_SIZE(zx_irdec_table),
    .rc_proto = RC_PROTO_NEC,
    .name = RC_MAP_ZX_IRDEC,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_zx_irdec() -> int __init {
    static int __init init_rc_map_zx_irdec(void)
    {
    return rc_map_register(&zx_irdec_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_zx_irdec() -> void __exit {
    static void __exit exit_rc_map_zx_irdec(void)
    {
    rc_map_unregister(&zx_irdec_map);
    }
    module_init(init_rc_map_zx_irdec)
    module_exit(exit_rc_map_zx_irdec)
    MODULE_AUTHOR("Shawn Guo <shawn.guo@linaro.org>");
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("zx-irdec remote controller keytable");
