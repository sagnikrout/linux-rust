//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-trekstor.c
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
// TrekStor remote controller keytable
//
// Copyright (C) 2010 Antti Palosaari <crope@iki.fi>
//

// TrekStor DVB-T USB Stick remote controller.
// Imported from af9015.h.
    Initial keytable was from Marc Schneider <macke@macke.org> */
    static struct rc_map_table trekstor[] = {
    { 0x0084, KEY_NUMERIC_0 },
    { 0x0085, KEY_MUTE },            /* Mute */
    { 0x0086, KEY_HOMEPAGE },        /* Home */
    { 0x0087, KEY_UP },              /* Up */
    { 0x0088, KEY_OK },              /* OK */
    { 0x0089, KEY_RIGHT },           /* Right */
    { 0x008a, KEY_FASTFORWARD },     /* Fast forward */
    { 0x008b, KEY_VOLUMEUP },        /* Volume + */
    { 0x008c, KEY_DOWN },            /* Down */
    { 0x008d, KEY_PLAY },            /* Play/Pause */
    { 0x008e, KEY_STOP },            /* Stop */
    { 0x008f, KEY_EPG },             /* Info/EPG */
    { 0x0090, KEY_NUMERIC_7 },
    { 0x0091, KEY_NUMERIC_4 },
    { 0x0092, KEY_NUMERIC_1 },
    { 0x0093, KEY_CHANNELDOWN },     /* Channel - */
    { 0x0094, KEY_NUMERIC_8 },
    { 0x0095, KEY_NUMERIC_5 },
    { 0x0096, KEY_NUMERIC_2 },
    { 0x0097, KEY_CHANNELUP },       /* Channel + */
    { 0x0098, KEY_NUMERIC_9 },
    { 0x0099, KEY_NUMERIC_6 },
    { 0x009a, KEY_NUMERIC_3 },
    { 0x009b, KEY_VOLUMEDOWN },      /* Volume - */
    { 0x009c, KEY_TV },              /* TV */
    { 0x009d, KEY_RECORD },          /* Record */
    { 0x009e, KEY_REWIND },          /* Rewind */
    { 0x009f, KEY_LEFT },            /* Left */
    };
    static struct rc_map_list trekstor_map = {
    .map = {
    .scan     = trekstor,
    .size     = ARRAY_SIZE(trekstor),
    .rc_proto = RC_PROTO_NEC,
    .name     = RC_MAP_TREKSTOR,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_trekstor() -> int __init {
    static int __init init_rc_map_trekstor(void)
    {
    return rc_map_register(&trekstor_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_trekstor() -> void __exit {
    static void __exit exit_rc_map_trekstor(void)
    {
    rc_map_unregister(&trekstor_map);
    }
    module_init(init_rc_map_trekstor)
    module_exit(exit_rc_map_trekstor)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Antti Palosaari <crope@iki.fi>");
    MODULE_DESCRIPTION("TrekStor remote controller keytable");
