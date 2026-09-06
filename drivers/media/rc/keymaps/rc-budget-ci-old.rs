//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-budget-ci-old.c
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
// budget-ci-old.h - Keytable for budget_ci_old Remote Controller
//
// keymap imported from ir-keymaps.c
//
// Copyright (c) 2010 by Mauro Carvalho Chehab

//
// From reading the following remotes:
// Zenith Universal 7 / TV Mode 807 / VCR Mode 837
// Hauppauge (from NOVA-CI-s box product)
// This is a "middle of the road" approach, differences are noted
//
    static struct rc_map_table budget_ci_old[] = {
    { 0x00, KEY_NUMERIC_0 },
    { 0x01, KEY_NUMERIC_1 },
    { 0x02, KEY_NUMERIC_2 },
    { 0x03, KEY_NUMERIC_3 },
    { 0x04, KEY_NUMERIC_4 },
    { 0x05, KEY_NUMERIC_5 },
    { 0x06, KEY_NUMERIC_6 },
    { 0x07, KEY_NUMERIC_7 },
    { 0x08, KEY_NUMERIC_8 },
    { 0x09, KEY_NUMERIC_9 },
    { 0x0a, KEY_ENTER },
    { 0x0b, KEY_RED },
    { 0x0c, KEY_POWER },		/* RADIO on Hauppauge */
    { 0x0d, KEY_MUTE },
    { 0x0f, KEY_A },		/* TV on Hauppauge */
    { 0x10, KEY_VOLUMEUP },
    { 0x11, KEY_VOLUMEDOWN },
    { 0x14, KEY_B },
    { 0x1c, KEY_UP },
    { 0x1d, KEY_DOWN },
    { 0x1e, KEY_OPTION },		/* RESERVED on Hauppauge */
    { 0x1f, KEY_BREAK },
    { 0x20, KEY_CHANNELUP },
    { 0x21, KEY_CHANNELDOWN },
    { 0x22, KEY_PREVIOUS },		/* Prev Ch on Zenith, SOURCE on Hauppauge */
    { 0x24, KEY_RESTART },
    { 0x25, KEY_OK },
    { 0x26, KEY_CYCLEWINDOWS },	/* MINIMIZE on Hauppauge */
    { 0x28, KEY_ENTER },		/* VCR mode on Zenith */
    { 0x29, KEY_PAUSE },
    { 0x2b, KEY_RIGHT },
    { 0x2c, KEY_LEFT },
    { 0x2e, KEY_MENU },		/* FULL SCREEN on Hauppauge */
    { 0x30, KEY_SLOW },
    { 0x31, KEY_PREVIOUS },		/* VCR mode on Zenith */
    { 0x32, KEY_REWIND },
    { 0x34, KEY_FASTFORWARD },
    { 0x35, KEY_PLAY },
    { 0x36, KEY_STOP },
    { 0x37, KEY_RECORD },
    { 0x38, KEY_TUNER },		/* TV/VCR on Zenith */
    { 0x3a, KEY_C },
    { 0x3c, KEY_EXIT },
    { 0x3d, KEY_POWER2 },
    { 0x3e, KEY_TUNER },
    };
    static struct rc_map_list budget_ci_old_map = {
    .map = {
    .scan     = budget_ci_old,
    .size     = ARRAY_SIZE(budget_ci_old),
    .rc_proto = RC_PROTO_UNKNOWN,	/* Legacy IR type */
    .name     = RC_MAP_BUDGET_CI_OLD,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_budget_ci_old() -> int __init {
    static int __init init_rc_map_budget_ci_old(void)
    {
    return rc_map_register(&budget_ci_old_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_budget_ci_old() -> void __exit {
    static void __exit exit_rc_map_budget_ci_old(void)
    {
    rc_map_unregister(&budget_ci_old_map);
    }
    module_init(init_rc_map_budget_ci_old)
    module_exit(exit_rc_map_budget_ci_old)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Mauro Carvalho Chehab");
    MODULE_DESCRIPTION("budget-ci-old remote controller keytable");
