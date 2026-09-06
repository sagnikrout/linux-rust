//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/powermac/udbg_adb.c
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


// SPDX-License-Identifier: GPL-2.0

//
// This implementation is "special", it can "patch" the current
// udbg implementation and work on top of it. It must thus be
// initialized last
//
    static void (*udbg_adb_old_putc)(char c);
    static int (*udbg_adb_old_getc)(void);
    static int (*udbg_adb_old_getc_poll)(void);
    static enum {
    input_adb_none,
    input_adb_pmu,
    input_adb_cuda,
    } input_type = input_adb_none;
    int xmon_wants_key, xmon_adb_keycode;
#[no_mangle]
pub unsafe extern "C" fn udbg_adb_poll() {
    static inline void udbg_adb_poll(void)
    {

    if (input_type == input_adb_pmu)
    pmu_poll_adb();

    if (input_type == input_adb_cuda)
    cuda_poll();

    }

    static int udbg_adb_use_btext;
    static int xmon_adb_shiftstate;
    static unsigned char xmon_keytab[128] =
    "asdfhgzxcv\000bqwer"				/* 0x00 - 0x0f */
    "yt123465=97-80]o"				/* 0x10 - 0x1f */
    "u[ip\rlj'k;\\,/nm."				/* 0x20 - 0x2f */
    "\t `\177\0\033\0\0\0\0\0\0\0\0\0\0"		/* 0x30 - 0x3f */
    "\0.\0*\0+\0\0\0\0\0/\r\0-\0"			/* 0x40 - 0x4f */
    "\0\0000123456789\0\0\0";			/* 0x50 - 0x5f */
    static unsigned char xmon_shift_keytab[128] =
    "ASDFHGZXCV\000BQWER"				/* 0x00 - 0x0f */
    "YT!@#$^%+(&_*)}O"				/* 0x10 - 0x1f */
    "U{IP\rLJ\"K:|<?NM>"				/* 0x20 - 0x2f */
    "\t ~\177\0\033\0\0\0\0\0\0\0\0\0\0"		/* 0x30 - 0x3f */
    "\0.\0*\0+\0\0\0\0\0/\r\0-\0"			/* 0x40 - 0x4f */
    "\0\0000123456789\0\0\0";			/* 0x50 - 0x5f */
#[no_mangle]
unsafe extern "C" fn udbg_adb_local_getc() -> c_int {
    static int udbg_adb_local_getc(void)
    {
    int k, t, on;
    xmon_wants_key = 1;
    for (;;) {
    xmon_adb_keycode = -1;
    t = 0;
    on = 0;
    k = -1;
    do {
    if (--t < 0) {
    on = 1 - on;
    btext_drawchar(on? 0xdb: 0x20);
    btext_drawchar('\b');
    t = 200000;
    }
    udbg_adb_poll();
    if (udbg_adb_old_getc_poll)
    k = udbg_adb_old_getc_poll();
    } while (k == -1 && xmon_adb_keycode == -1);
    if (on)
    btext_drawstring(" \b");
    if (k != -1)
    return k;
    k = xmon_adb_keycode;
// test for shift keys
    if ((k & 0x7f) == 0x38 || (k & 0x7f) == 0x7b) {
    xmon_adb_shiftstate = (k & 0x80) == 0;
    continue;
    }
    if (k >= 0x80)
    continue;	/* ignore up transitions */
    k = (xmon_adb_shiftstate? xmon_shift_keytab: xmon_keytab)[k];
    if (k != 0)
    break;
    }
    xmon_wants_key = 0;
    return k;
    }

#[no_mangle]
unsafe extern "C" fn udbg_adb_getc() -> c_int {
    static int udbg_adb_getc(void)
    {

    if (udbg_adb_use_btext && input_type != input_adb_none)
    return udbg_adb_local_getc();

    if (udbg_adb_old_getc)
    return udbg_adb_old_getc();
    return -1;
    }
// getc_poll() is not really used, unless you have the xmon-over modem
// hack that doesn't quite concern us here, thus we just poll the low level
// ADB driver to prevent it from timing out and call back the original poll
// routine.
//
#[no_mangle]
unsafe extern "C" fn udbg_adb_getc_poll() -> c_int {
    static int udbg_adb_getc_poll(void)
    {
    udbg_adb_poll();
    if (udbg_adb_old_getc_poll)
    return udbg_adb_old_getc_poll();
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn udbg_adb_putc(c: c_char) {
    static void udbg_adb_putc(char c)
    {

    if (udbg_adb_use_btext)
    btext_drawchar(c);

    if (udbg_adb_old_putc)
    return udbg_adb_old_putc(c);
    }
#[no_mangle]
pub unsafe extern "C" fn udbg_adb_init_early() -> void __init {
    void __init udbg_adb_init_early(void)
    {

    if (btext_find_display(1) == 0) {
    udbg_adb_use_btext = 1;
    udbg_putc = udbg_adb_putc;
    }

    }
#[no_mangle]
pub unsafe extern "C" fn udbg_adb_init(force_btext: c_int) -> int __init {
    int __init udbg_adb_init(int force_btext)
    {
    struct device_node *np;
// Capture existing callbacks
    udbg_adb_old_putc = udbg_putc;
    udbg_adb_old_getc = udbg_getc;
    udbg_adb_old_getc_poll = udbg_getc_poll;
// Check if our early init was already called
    if (udbg_adb_old_putc == udbg_adb_putc)
    udbg_adb_old_putc = core::ptr::null_mut();

    if (udbg_adb_old_putc == btext_drawchar)
    udbg_adb_old_putc = core::ptr::null_mut();

// Set ours as output
    udbg_putc = udbg_adb_putc;
    udbg_getc = udbg_adb_getc;
    udbg_getc_poll = udbg_adb_getc_poll;

// Check if we should use btext output
    if (btext_find_display(force_btext) == 0)
    udbg_adb_use_btext = 1;

// See if there is a keyboard in the device tree with a parent
// of type "adb". If not, we return a failure, but we keep the
// bext output set for now
//
    for_each_node_by_name(np, "keyboard") {
    struct device_node *parent = of_get_parent(np);
    let mut found: c_int = of_node_is_type(parent, "adb");
    of_node_put(parent);
    if (found)
    break;
    }
    if (np == core::ptr::null_mut())
    return -ENODEV;
    of_node_put(np);

    if (find_via_pmu())
    input_type = input_adb_pmu;

    if (find_via_cuda())
    input_type = input_adb_cuda;

// Same as above: nothing found, keep btext set for output
    if (input_type == input_adb_none)
    return -ENODEV;
    return 0;
    }
