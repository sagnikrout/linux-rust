//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-ati-tv-wonder-hd-600.c
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
// ati-tv-wonder-hd-600.h - Keytable for ati_tv_wonder_hd_600 Remote Controller
//
// keymap imported from ir-keymaps.c
//
// Copyright (c) 2010 by Mauro Carvalho Chehab

// ATI TV Wonder HD 600 USB
    Devin Heitmueller <devin.heitmueller@gmail.com>
//
    static struct rc_map_table ati_tv_wonder_hd_600[] = {
    { 0x00, KEY_RECORD},		/* Row 1 */
    { 0x01, KEY_PLAYPAUSE},
    { 0x02, KEY_STOP},
    { 0x03, KEY_POWER},
    { 0x04, KEY_PREVIOUS},	/* Row 2 */
    { 0x05, KEY_REWIND},
    { 0x06, KEY_FORWARD},
    { 0x07, KEY_NEXT},
    { 0x08, KEY_EPG},		/* Row 3 */
    { 0x09, KEY_HOME},
    { 0x0a, KEY_MENU},
    { 0x0b, KEY_CHANNELUP},
    { 0x0c, KEY_BACK},		/* Row 4 */
    { 0x0d, KEY_UP},
    { 0x0e, KEY_INFO},
    { 0x0f, KEY_CHANNELDOWN},
    { 0x10, KEY_LEFT},		/* Row 5 */
    { 0x11, KEY_SELECT},
    { 0x12, KEY_RIGHT},
    { 0x13, KEY_VOLUMEUP},
    { 0x14, KEY_LAST},		/* Row 6 */
    { 0x15, KEY_DOWN},
    { 0x16, KEY_MUTE},
    { 0x17, KEY_VOLUMEDOWN},
    };
    static struct rc_map_list ati_tv_wonder_hd_600_map = {
    .map = {
    .scan     = ati_tv_wonder_hd_600,
    .size     = ARRAY_SIZE(ati_tv_wonder_hd_600),
    .rc_proto = RC_PROTO_UNKNOWN,	/* Legacy IR type */
    .name     = RC_MAP_ATI_TV_WONDER_HD_600,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_ati_tv_wonder_hd_600() -> int __init {
    static int __init init_rc_map_ati_tv_wonder_hd_600(void)
    {
    return rc_map_register(&ati_tv_wonder_hd_600_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_ati_tv_wonder_hd_600() -> void __exit {
    static void __exit exit_rc_map_ati_tv_wonder_hd_600(void)
    {
    rc_map_unregister(&ati_tv_wonder_hd_600_map);
    }
    module_init(init_rc_map_ati_tv_wonder_hd_600)
    module_exit(exit_rc_map_ati_tv_wonder_hd_600)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Mauro Carvalho Chehab");
    MODULE_DESCRIPTION("ati-tv-wonder-hd-600 remote controller keytable");
