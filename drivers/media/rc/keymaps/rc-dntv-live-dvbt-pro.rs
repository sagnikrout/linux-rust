//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-dntv-live-dvbt-pro.c
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
// dntv-live-dvbt-pro.h - Keytable for dntv_live_dvbt_pro Remote Controller
//
// keymap imported from ir-keymaps.c
//
// Copyright (c) 2010 by Mauro Carvalho Chehab

// DigitalNow DNTV Live! DVB-T Pro Remote
    static struct rc_map_table dntv_live_dvbt_pro[] = {
    { 0x16, KEY_POWER },
    { 0x5b, KEY_HOME },
    { 0x55, KEY_TV },		/* live tv */
    { 0x58, KEY_TUNER },		/* digital Radio */
    { 0x5a, KEY_RADIO },		/* FM radio */
    { 0x59, KEY_DVD },		/* dvd menu */
    { 0x03, KEY_NUMERIC_1 },
    { 0x01, KEY_NUMERIC_2 },
    { 0x06, KEY_NUMERIC_3 },
    { 0x09, KEY_NUMERIC_4 },
    { 0x1d, KEY_NUMERIC_5 },
    { 0x1f, KEY_NUMERIC_6 },
    { 0x0d, KEY_NUMERIC_7 },
    { 0x19, KEY_NUMERIC_8 },
    { 0x1b, KEY_NUMERIC_9 },
    { 0x0c, KEY_CANCEL },
    { 0x15, KEY_NUMERIC_0 },
    { 0x4a, KEY_CLEAR },
    { 0x13, KEY_BACK },
    { 0x00, KEY_TAB },
    { 0x4b, KEY_UP },
    { 0x4e, KEY_LEFT },
    { 0x4f, KEY_OK },
    { 0x52, KEY_RIGHT },
    { 0x51, KEY_DOWN },
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
    { 0x41, KEY_PREVIOUSSONG },	/* replay |< */
    { 0x42, KEY_NEXTSONG },		/* skip >| */
    { 0x54, KEY_CAMERA },		/* capture */
    { 0x50, KEY_LANGUAGE },		/* sap */
    { 0x47, KEY_TV2 },		/* pip */
    { 0x4d, KEY_SCREEN },
    { 0x43, KEY_SUBTITLE },
    { 0x10, KEY_MUTE },
    { 0x49, KEY_AUDIO },		/* l/r */
    { 0x07, KEY_SLEEP },
    { 0x08, KEY_VIDEO },		/* a/v */
    { 0x0e, KEY_PREVIOUS },		/* recall */
    { 0x45, KEY_ZOOM },		/* zoom + */
    { 0x46, KEY_ANGLE },		/* zoom - */
    { 0x56, KEY_RED },
    { 0x57, KEY_GREEN },
    { 0x5c, KEY_YELLOW },
    { 0x5d, KEY_BLUE },
    };
    static struct rc_map_list dntv_live_dvbt_pro_map = {
    .map = {
    .scan     = dntv_live_dvbt_pro,
    .size     = ARRAY_SIZE(dntv_live_dvbt_pro),
    .rc_proto = RC_PROTO_UNKNOWN,	/* Legacy IR type */
    .name     = RC_MAP_DNTV_LIVE_DVBT_PRO,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_dntv_live_dvbt_pro() -> int __init {
    static int __init init_rc_map_dntv_live_dvbt_pro(void)
    {
    return rc_map_register(&dntv_live_dvbt_pro_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_dntv_live_dvbt_pro() -> void __exit {
    static void __exit exit_rc_map_dntv_live_dvbt_pro(void)
    {
    rc_map_unregister(&dntv_live_dvbt_pro_map);
    }
    module_init(init_rc_map_dntv_live_dvbt_pro)
    module_exit(exit_rc_map_dntv_live_dvbt_pro)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Mauro Carvalho Chehab");
    MODULE_DESCRIPTION("DigitalNow DNTV Live DVB-T Remote controller keytable");
