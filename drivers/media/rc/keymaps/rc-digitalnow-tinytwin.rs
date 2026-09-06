//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/keymaps/rc-digitalnow-tinytwin.c
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
// DigitalNow TinyTwin remote controller keytable
//
// Copyright (C) 2010 Antti Palosaari <crope@iki.fi>
//

    static struct rc_map_table digitalnow_tinytwin[] = {
    { 0x0000, KEY_MUTE },            /* [symbol speaker] */
    { 0x0001, KEY_VOLUMEUP },
    { 0x0002, KEY_POWER2 },          /* TV [power button] */
    { 0x0003, KEY_NUMERIC_2 },
    { 0x0004, KEY_NUMERIC_3 },
    { 0x0005, KEY_NUMERIC_4 },
    { 0x0006, KEY_NUMERIC_6 },
    { 0x0007, KEY_NUMERIC_7 },
    { 0x0008, KEY_NUMERIC_8 },
    { 0x0009, KEY_NUMERIC_STAR },    /* [*] */
    { 0x000a, KEY_NUMERIC_0 },
    { 0x000b, KEY_NUMERIC_POUND },   /* [#] */
    { 0x000c, KEY_RIGHT },           /* [right arrow] */
    { 0x000d, KEY_HOMEPAGE },        /* [symbol home] Start */
    { 0x000e, KEY_RED },             /* [red] Videos */
    { 0x0010, KEY_POWER },           /* PC [power button] */
    { 0x0011, KEY_YELLOW },          /* [yellow] Pictures */
    { 0x0012, KEY_DOWN },            /* [down arrow] */
    { 0x0013, KEY_GREEN },           /* [green] Music */
    { 0x0014, KEY_CYCLEWINDOWS },    /* BACK */
    { 0x0015, KEY_FAVORITES },       /* MORE */
    { 0x0016, KEY_UP },              /* [up arrow] */
    { 0x0017, KEY_LEFT },            /* [left arrow] */
    { 0x0018, KEY_OK },              /* OK */
    { 0x0019, KEY_BLUE },            /* [blue] MyTV */
    { 0x001a, KEY_REWIND },          /* REW [<<] */
    { 0x001b, KEY_PLAY },            /* PLAY */
    { 0x001c, KEY_NUMERIC_5 },
    { 0x001d, KEY_NUMERIC_9 },
    { 0x001e, KEY_VOLUMEDOWN },
    { 0x001f, KEY_NUMERIC_1 },
    { 0x0040, KEY_STOP },            /* STOP */
    { 0x0042, KEY_PAUSE },           /* PAUSE */
    { 0x0043, KEY_SCREEN },          /* Aspect */
    { 0x0044, KEY_FORWARD },         /* FWD [>>] */
    { 0x0045, KEY_NEXT },            /* SKIP */
    { 0x0048, KEY_RECORD },          /* RECORD */
    { 0x0049, KEY_VIDEO },           /* RTV */
    { 0x004a, KEY_EPG },             /* Guide */
    { 0x004b, KEY_CHANNELUP },
    { 0x004c, KEY_HELP },            /* Help */
    { 0x004d, KEY_RADIO },           /* Radio */
    { 0x004f, KEY_CHANNELDOWN },
    { 0x0050, KEY_DVD },             /* DVD */
    { 0x0051, KEY_AUDIO },           /* Audio */
    { 0x0052, KEY_TITLE },           /* Title */
    { 0x0053, KEY_NEW },             /* [symbol PIP?] */
    { 0x0057, KEY_MENU },            /* Mouse */
    { 0x005a, KEY_PREVIOUS },        /* REPLAY */
    };
    static struct rc_map_list digitalnow_tinytwin_map = {
    .map = {
    .scan     = digitalnow_tinytwin,
    .size     = ARRAY_SIZE(digitalnow_tinytwin),
    .rc_proto = RC_PROTO_NEC,
    .name     = RC_MAP_DIGITALNOW_TINYTWIN,
    }
    };
#[no_mangle]
unsafe extern "C" fn init_rc_map_digitalnow_tinytwin() -> int __init {
    static int __init init_rc_map_digitalnow_tinytwin(void)
    {
    return rc_map_register(&digitalnow_tinytwin_map);
    }
#[no_mangle]
unsafe extern "C" fn exit_rc_map_digitalnow_tinytwin() -> void __exit {
    static void __exit exit_rc_map_digitalnow_tinytwin(void)
    {
    rc_map_unregister(&digitalnow_tinytwin_map);
    }
    module_init(init_rc_map_digitalnow_tinytwin)
    module_exit(exit_rc_map_digitalnow_tinytwin)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Antti Palosaari <crope@iki.fi>");
    MODULE_DESCRIPTION("DigitalNow TinyTwin remote controller keytable");
