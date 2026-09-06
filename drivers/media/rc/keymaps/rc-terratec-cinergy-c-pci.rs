//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-terratec-cinergy-c-pci.c
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
// keytable for Terratec Cinergy C PCI Remote Controller
//
// Copyright (c) 2010 by Igor M. Liplianin <liplianin@me.by>
//

    static struct rc_map_table terratec_cinergy_c_pci[] = {
    { 0x3e, KEY_POWER},
    { 0x3d, KEY_NUMERIC_1},
    { 0x3c, KEY_NUMERIC_2},
    { 0x3b, KEY_NUMERIC_3},
    { 0x3a, KEY_NUMERIC_4},
    { 0x39, KEY_NUMERIC_5},
    { 0x38, KEY_NUMERIC_6},
    { 0x37, KEY_NUMERIC_7},
    { 0x36, KEY_NUMERIC_8},
    { 0x35, KEY_NUMERIC_9},
    { 0x34, KEY_VIDEO_NEXT}, /* AV */
    { 0x33, KEY_NUMERIC_0},
    { 0x32, KEY_REFRESH},
    { 0x30, KEY_EPG},
    { 0x2f, KEY_UP},
    { 0x2e, KEY_LEFT},
    { 0x2d, KEY_OK},
    { 0x2c, KEY_RIGHT},
    { 0x2b, KEY_DOWN},
    { 0x29, KEY_INFO},
    { 0x28, KEY_RED},
    { 0x27, KEY_GREEN},
    { 0x26, KEY_YELLOW},
    { 0x25, KEY_BLUE},
    { 0x24, KEY_CHANNELUP},
    { 0x23, KEY_VOLUMEUP},
    { 0x22, KEY_MUTE},
    { 0x21, KEY_VOLUMEDOWN},
    { 0x20, KEY_CHANNELDOWN},
    { 0x1f, KEY_PAUSE},
    { 0x1e, KEY_HOME},
    { 0x1d, KEY_MENU}, /* DVD Menu */
    { 0x1c, KEY_SUBTITLE},
    { 0x1b, KEY_TEXT}, /* Teletext */
    { 0x1a, KEY_DELETE},
    { 0x19, KEY_TV},
    { 0x18, KEY_DVD},
    { 0x17, KEY_STOP},
    { 0x16, KEY_VIDEO},
    { 0x15, KEY_AUDIO}, /* Music */
    { 0x14, KEY_SCREEN}, /* Pic */
    { 0x13, KEY_PLAY},
    { 0x12, KEY_BACK},
    { 0x11, KEY_REWIND},
    { 0x10, KEY_FASTFORWARD},
    { 0x0b, KEY_PREVIOUS},
    { 0x07, KEY_RECORD},
    { 0x03, KEY_NEXT},
    };
    static struct rc_map_list terratec_cinergy_c_pci_map = {
    .map = {
    .scan     = terratec_cinergy_c_pci,
    .size     = ARRAY_SIZE(terratec_cinergy_c_pci),
    .rc_proto = RC_PROTO_UNKNOWN,	/* Legacy IR type */
    .name     = RC_MAP_TERRATEC_CINERGY_C_PCI,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_terratec_cinergy_c_pci() -> int __init {
    static int __init init_rc_map_terratec_cinergy_c_pci(void)
    {
    return rc_map_register(&terratec_cinergy_c_pci_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_terratec_cinergy_c_pci() -> void __exit {
    static void __exit exit_rc_map_terratec_cinergy_c_pci(void)
    {
    rc_map_unregister(&terratec_cinergy_c_pci_map);
    }
    module_init(init_rc_map_terratec_cinergy_c_pci);
    module_exit(exit_rc_map_terratec_cinergy_c_pci);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Terratec Cinergy C PCI remote controller keytable");
