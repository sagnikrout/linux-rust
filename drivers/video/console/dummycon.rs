//! Automatically rewritten from C to Rust
//! Source: drivers/video/console/dummycon.c
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
// linux/drivers/video/dummycon.c -- A dummy console driver
//
// To be used if there's no other console driver (e.g. for plain VGA text)
// available, usually until fbcon takes console over.
//

//
// Dummy console driver
//

// set by Kconfig. Use 80x25 for 640x480 and 160x64 for 1280x1024

// These are both protected by the console_lock
    static RAW_NOTIFIER_HEAD(dummycon_output_nh);
    static bool dummycon_putc_called;
#[no_mangle]
pub unsafe extern "C" fn dummycon_register_output_notifier(nb: *mut notifier_block) {
    void dummycon_register_output_notifier(struct notifier_block *nb)
    {
    WARN_CONSOLE_UNLOCKED();
    raw_notifier_chain_register(&dummycon_output_nh, nb);
    if (dummycon_putc_called)
    nb.notifier_call(nb, 0, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn dummycon_unregister_output_notifier(nb: *mut notifier_block) {
    void dummycon_unregister_output_notifier(struct notifier_block *nb)
    {
    WARN_CONSOLE_UNLOCKED();
    raw_notifier_chain_unregister(&dummycon_output_nh, nb);
    }
    static void dummycon_putc(struct vc_data *vc, u16 c, unsigned int y,
    unsigned int x)
    {
    WARN_CONSOLE_UNLOCKED();
    dummycon_putc_called = true;
    raw_notifier_call_chain(&dummycon_output_nh, 0, core::ptr::null_mut());
    }
    static void dummycon_putcs(struct vc_data *vc, const u16 *s, unsigned int count,
    unsigned int ypos, unsigned int xpos)
    {
    unsigned int i;
    if (!dummycon_putc_called) {
// Ignore erases
    for (i = 0 ; i < count; i++) {
    if (s[i] != vc.vc_video_erase_char)
    break;
    }
    if (i == count)
    return;
    dummycon_putc_called = true;
    }
    raw_notifier_call_chain(&dummycon_output_nh, 0, core::ptr::null_mut());
    }
    static bool dummycon_blank(struct vc_data *vc, enum vesa_blank_mode blank,
    bool mode_switch)
    {
// Redraw, so that we get putc(s) for output done while blanked
    return true;
    }
#[no_mangle]
unsafe extern "C" fn dummycon_switch(vc: *mut vc_data) -> bool {
    static bool dummycon_switch(struct vc_data *vc)
    {
//
// Redraw, so that we get putc(s) for output done while switched
// away. Informs deferred consoles to take over the display.
//
    return true;
    }

    static void dummycon_putc(struct vc_data *vc, u16 c, unsigned int y,
    unsigned int x) { }
    static void dummycon_putcs(struct vc_data *vc, const u16 *s, unsigned int count,
    unsigned int ypos, unsigned int xpos) { }
    static bool dummycon_blank(struct vc_data *vc, enum vesa_blank_mode blank,
    bool mode_switch)
    {
    return false;
    }
#[no_mangle]
unsafe extern "C" fn dummycon_switch(vc: *mut vc_data) -> bool {
    static bool dummycon_switch(struct vc_data *vc)
    {
    return false;
    }

    static const char *dummycon_startup(void)
    {
    return "dummy device";
    }
#[no_mangle]
unsafe extern "C" fn dummycon_init(vc: *mut vc_data, init: bool) {
    static void dummycon_init(struct vc_data *vc, bool init)
    {
    vc.vc_can_do_color = 1;
    if (init) {
    vc.vc_cols = DUMMY_COLUMNS;
    vc.vc_rows = DUMMY_ROWS;
    } else
    vc_resize(vc, DUMMY_COLUMNS, DUMMY_ROWS);
    }
    static void dummycon_deinit(struct vc_data *vc) { }
    static void dummycon_clear(struct vc_data *vc, unsigned int sy, unsigned int sx,
    unsigned int width) { }
    static void dummycon_cursor(struct vc_data *vc, bool enable) { }
    static bool dummycon_scroll(struct vc_data *vc, unsigned int top,
    unsigned int bottom, enum con_scroll dir,
    unsigned int lines)
    {
    return false;
    }
//
// The console `switch' structure for the dummy console
//
// Most of the operations are dummies.
//
    const struct consw dummy_con = {
    .owner =		THIS_MODULE,
    .con_startup =	dummycon_startup,
    .con_init =		dummycon_init,
    .con_deinit =	dummycon_deinit,
    .con_clear =	dummycon_clear,
    .con_putc =		dummycon_putc,
    .con_putcs =	dummycon_putcs,
    .con_cursor =	dummycon_cursor,
    .con_scroll =	dummycon_scroll,
    .con_switch =	dummycon_switch,
    .con_blank =	dummycon_blank,
    };
    EXPORT_SYMBOL_GPL(dummy_con);
