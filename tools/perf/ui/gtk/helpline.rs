//! Automatically rewritten from C to Rust
//! Source: tools/perf/ui/gtk/helpline.c
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

#[no_mangle]
unsafe extern "C" fn gtk_helpline_pop() {
    static void gtk_helpline_pop(void)
    {
    if (!perf_gtk__is_active_context(pgctx))
    return;
    gtk_statusbar_pop(GTK_STATUSBAR(pgctx.statbar),
    pgctx.statbar_ctx_id);
    }
#[no_mangle]
unsafe extern "C" fn gtk_helpline_push(msg: *const c_char) {
    static void gtk_helpline_push(const char *msg)
    {
    if (!perf_gtk__is_active_context(pgctx))
    return;
    gtk_statusbar_push(GTK_STATUSBAR(pgctx.statbar),
    pgctx.statbar_ctx_id, msg);
    }
#[no_mangle]
unsafe extern "C" fn gtk_helpline_show(fmt: *const c_char, ap: va_list) -> c_int {
    static int gtk_helpline_show(const char *fmt, va_list ap)
    {
    int ret;
    char *ptr;
    static int backlog;
    ret = vscnprintf(ui_helpline__current + backlog,
    sizeof(ui_helpline__current) - backlog, fmt, ap);
    backlog += ret;
// only first line can be displayed
    ptr = strchr(ui_helpline__current, '\n');
    if (ptr && (ptr - ui_helpline__current) <= backlog) {
// ptr = '\0';
    ui_helpline__puts(ui_helpline__current);
    backlog = 0;
    }
    return ret;
    }
    static struct ui_helpline gtk_helpline_fns = {
    .pop	= gtk_helpline_pop,
    .push	= gtk_helpline_push,
    .show	= gtk_helpline_show,
    };
#[no_mangle]
pub unsafe extern "C" fn perf_gtk__init_helpline() {
    void perf_gtk__init_helpline(void)
    {
    helpline_fns = &gtk_helpline_fns;
    }
