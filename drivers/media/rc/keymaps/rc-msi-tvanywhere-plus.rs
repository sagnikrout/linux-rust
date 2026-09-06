//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-msi-tvanywhere-plus.c
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
// msi-tvanywhere-plus.h - Keytable for msi_tvanywhere_plus Remote Controller
//
// keymap imported from ir-keymaps.c
//
// Copyright (c) 2010 by Mauro Carvalho Chehab

//
    Keycodes for remote on the MSI TV@nywhere Plus. The controller IC on the card
    is marked "KS003". The controller is I2C at address 0x30, but does not seem
    to respond to probes until a read is performed from a valid device.
    I don't know why...
    Note: This remote may be of similar or identical design to the
    Pixelview remote (?).  The raw codes and duplicate button codes
    appear to be the same.
    Henry Wong <henry@stuffedcow.net>
    Some changes to formatting and keycodes by Mark Schultz <n9xmj@yahoo.com>
//
    static struct rc_map_table msi_tvanywhere_plus[] = {
// ---- Remote Button Layout ----
    POWER   SOURCE  SCAN    MUTE
    TV/FM   1       2       3
    |>      4       5       6
    <|      7       8       9
    ^^UP    0       +       RECALL
    vvDN    RECORD  STOP    PLAY
    MINIMIZE          ZOOM
    CH+
    VOL-                   VOL+
    CH-
    SNAPSHOT           MTS
    <<      FUNC    >>     RESET
//
    { 0x01, KEY_NUMERIC_1 },	/* 1 */
    { 0x0b, KEY_NUMERIC_2 },	/* 2 */
    { 0x1b, KEY_NUMERIC_3 },	/* 3 */
    { 0x05, KEY_NUMERIC_4 },	/* 4 */
    { 0x09, KEY_NUMERIC_5 },	/* 5 */
    { 0x15, KEY_NUMERIC_6 },	/* 6 */
    { 0x06, KEY_NUMERIC_7 },	/* 7 */
    { 0x0a, KEY_NUMERIC_8 },	/* 8 */
    { 0x12, KEY_NUMERIC_9 },	/* 9 */
    { 0x02, KEY_NUMERIC_0 },	/* 0 */
    { 0x10, KEY_KPPLUS },		/* + */
    { 0x13, KEY_AGAIN },		/* Recall */
    { 0x1e, KEY_POWER },		/* Power */
    { 0x07, KEY_VIDEO },		/* Source */
    { 0x1c, KEY_SEARCH },		/* Scan */
    { 0x18, KEY_MUTE },		/* Mute */
    { 0x03, KEY_RADIO },		/* TV/FM */
// The next four keys are duplicates that appear to send the
    same IR code as Ch+, Ch-, >>, and << .  The raw code assigned
    to them is the actual code + 0x20 - they will never be
    detected as such unless some way is discovered to distinguish
    these buttons from those that have the same code. */
    { 0x3f, KEY_RIGHT },		/* |> and Ch+ */
    { 0x37, KEY_LEFT },		/* <| and Ch- */
    { 0x2c, KEY_UP },		/* ^^Up and >> */
    { 0x24, KEY_DOWN },		/* vvDn and << */
    { 0x00, KEY_RECORD },		/* Record */
    { 0x08, KEY_STOP },		/* Stop */
    { 0x11, KEY_PLAY },		/* Play */
    { 0x0f, KEY_CLOSE },		/* Minimize */
    { 0x19, KEY_ZOOM },		/* Zoom */
    { 0x1a, KEY_CAMERA },		/* Snapshot */
    { 0x0d, KEY_LANGUAGE },		/* MTS */
    { 0x14, KEY_VOLUMEDOWN },	/* Vol- */
    { 0x16, KEY_VOLUMEUP },		/* Vol+ */
    { 0x17, KEY_CHANNELDOWN },	/* Ch- */
    { 0x1f, KEY_CHANNELUP },	/* Ch+ */
    { 0x04, KEY_REWIND },		/* << */
    { 0x0e, KEY_MENU },		/* Function */
    { 0x0c, KEY_FASTFORWARD },	/* >> */
    { 0x1d, KEY_RESTART },		/* Reset */
    };
    static struct rc_map_list msi_tvanywhere_plus_map = {
    .map = {
    .scan     = msi_tvanywhere_plus,
    .size     = ARRAY_SIZE(msi_tvanywhere_plus),
    .rc_proto = RC_PROTO_UNKNOWN,	/* Legacy IR type */
    .name     = RC_MAP_MSI_TVANYWHERE_PLUS,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_msi_tvanywhere_plus() -> int __init {
    static int __init init_rc_map_msi_tvanywhere_plus(void)
    {
    return rc_map_register(&msi_tvanywhere_plus_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_msi_tvanywhere_plus() -> void __exit {
    static void __exit exit_rc_map_msi_tvanywhere_plus(void)
    {
    rc_map_unregister(&msi_tvanywhere_plus_map);
    }
    module_init(init_rc_map_msi_tvanywhere_plus)
    module_exit(exit_rc_map_msi_tvanywhere_plus)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Mauro Carvalho Chehab");
    MODULE_DESCRIPTION("MSI TV@nywhere Plus remote controller keytable");
