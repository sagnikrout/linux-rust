//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-msi-digivox-ii.c
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
// MSI DIGIVOX mini II remote controller keytable
//
// Copyright (C) 2010 Antti Palosaari <crope@iki.fi>
//

    static struct rc_map_table msi_digivox_ii[] = {
    { 0x0302, KEY_NUMERIC_2 },
    { 0x0303, KEY_UP },              /* up */
    { 0x0304, KEY_NUMERIC_3 },
    { 0x0305, KEY_CHANNELDOWN },
    { 0x0308, KEY_NUMERIC_5 },
    { 0x0309, KEY_NUMERIC_0 },
    { 0x030b, KEY_NUMERIC_8 },
    { 0x030d, KEY_DOWN },            /* down */
    { 0x0310, KEY_NUMERIC_9 },
    { 0x0311, KEY_NUMERIC_7 },
    { 0x0314, KEY_VOLUMEUP },
    { 0x0315, KEY_CHANNELUP },
    { 0x0316, KEY_OK },
    { 0x0317, KEY_POWER2 },
    { 0x031a, KEY_NUMERIC_1 },
    { 0x031c, KEY_NUMERIC_4 },
    { 0x031d, KEY_NUMERIC_6 },
    { 0x031f, KEY_VOLUMEDOWN },
    };
    static struct rc_map_list msi_digivox_ii_map = {
    .map = {
    .scan     = msi_digivox_ii,
    .size     = ARRAY_SIZE(msi_digivox_ii),
    .rc_proto = RC_PROTO_NEC,
    .name     = RC_MAP_MSI_DIGIVOX_II,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_msi_digivox_ii() -> int __init {
    static int __init init_rc_map_msi_digivox_ii(void)
    {
    return rc_map_register(&msi_digivox_ii_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_msi_digivox_ii() -> void __exit {
    static void __exit exit_rc_map_msi_digivox_ii(void)
    {
    rc_map_unregister(&msi_digivox_ii_map);
    }
    module_init(init_rc_map_msi_digivox_ii)
    module_exit(exit_rc_map_msi_digivox_ii)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Antti Palosaari <crope@iki.fi>");
    MODULE_DESCRIPTION("MSI DIGIVOX mini II remote controller keytable");
