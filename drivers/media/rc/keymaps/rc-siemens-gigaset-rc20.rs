//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-siemens-gigaset-rc20.c
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
// rc-siemens-gigaset-rc20.c - Keytable for the Siemens Gigaset RC 20 remote
//
// Copyright (c) 2025 by Michael Klein
//

    static struct rc_map_table siemens_gigaset_rc20[] = {
    { 0x1501, KEY_POWER },
    { 0x1502, KEY_MUTE },
    { 0x1503, KEY_NUMERIC_1 },
    { 0x1504, KEY_NUMERIC_2 },
    { 0x1505, KEY_NUMERIC_3 },
    { 0x1506, KEY_NUMERIC_4 },
    { 0x1507, KEY_NUMERIC_5 },
    { 0x1508, KEY_NUMERIC_6 },
    { 0x1509, KEY_NUMERIC_7 },
    { 0x150a, KEY_NUMERIC_8 },
    { 0x150b, KEY_NUMERIC_9 },
    { 0x150c, KEY_NUMERIC_0 },
    { 0x150d, KEY_UP },
    { 0x150e, KEY_LEFT },
    { 0x150f, KEY_OK },
    { 0x1510, KEY_RIGHT },
    { 0x1511, KEY_DOWN },
    { 0x1512, KEY_SHUFFLE },        /* double-arrow */
    { 0x1513, KEY_EXIT },
    { 0x1514, KEY_RED },
    { 0x1515, KEY_GREEN },
    { 0x1516, KEY_YELLOW },         /* OPT */
    { 0x1517, KEY_BLUE },
    { 0x1518, KEY_MENU },
    { 0x1519, KEY_TEXT },
    { 0x151a, KEY_MODE },           /* TV/Radio */
    { 0x1521, KEY_EPG },
    { 0x1522, KEY_FAVORITES },
    { 0x1523, KEY_CHANNELUP },
    { 0x1524, KEY_CHANNELDOWN },
    { 0x1525, KEY_VOLUMEUP },
    { 0x1526, KEY_VOLUMEDOWN },
    { 0x1527, KEY_INFO },
    };
    static struct rc_map_list siemens_gigaset_rc20_map = {
    .map = {
    .scan     = siemens_gigaset_rc20,
    .size     = ARRAY_SIZE(siemens_gigaset_rc20),
    .rc_proto = RC_PROTO_RC5,
    .name     = RC_MAP_SIEMENS_GIGASET_RC20,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_siemens_gigaset_rc20() -> int __init {
    static int __init init_rc_map_siemens_gigaset_rc20(void)
    {
    return rc_map_register(&siemens_gigaset_rc20_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_siemens_gigaset_rc20() -> void __exit {
    static void __exit exit_rc_map_siemens_gigaset_rc20(void)
    {
    rc_map_unregister(&siemens_gigaset_rc20_map);
    }
    module_init(init_rc_map_siemens_gigaset_rc20)
    module_exit(exit_rc_map_siemens_gigaset_rc20)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Michael Klein");
    MODULE_DESCRIPTION("Siemens Gigaset RC20 remote keytable");
