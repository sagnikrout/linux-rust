//! Automatically rewritten from C to Rust
//! Source: tools/perf/ui/tui/helpline.c
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

    char ui_helpline__last_msg[1024];
    bool tui_helpline__set;
#[no_mangle]
unsafe extern "C" fn tui_helpline__pop() {
    static void tui_helpline__pop(void)
    {
    }
#[no_mangle]
unsafe extern "C" fn tui_helpline__push(msg: *const c_char) {
    static void tui_helpline__push(const char *msg)
    {
    let mut sz: usize = sizeof(ui_helpline__current);
    SLsmg_gotorc(SLtt_Screen_Rows - 1, 0);
    SLsmg_set_color(0);
    SLsmg_write_nstring(msg, SLtt_Screen_Cols);
    SLsmg_refresh();
    strlcpy(ui_helpline__current, msg, sz);
    }
#[no_mangle]
unsafe extern "C" fn tui_helpline__show(format: *const c_char, ap: va_list) -> c_int {
    static int tui_helpline__show(const char *format, va_list ap)
    {
    int ret;
    static int backlog;
    mutex_lock(&ui__lock);
    ret = vscnprintf(ui_helpline__last_msg + backlog,
    sizeof(ui_helpline__last_msg) - backlog, format, ap);
    backlog += ret;
    tui_helpline__set = true;
    if (ui_helpline__last_msg[backlog - 1] == '\n') {
    ui_helpline__puts(ui_helpline__last_msg);
    SLsmg_refresh();
    backlog = 0;
    }
    mutex_unlock(&ui__lock);
    return ret;
    }
    struct ui_helpline tui_helpline_fns = {
    .pop	= tui_helpline__pop,
    .push	= tui_helpline__push,
    .show	= tui_helpline__show,
    };
#[no_mangle]
pub unsafe extern "C" fn ui_helpline__init() {
    void ui_helpline__init(void)
    {
    helpline_fns = &tui_helpline_fns;
    ui_helpline__puts(" ");
    }
