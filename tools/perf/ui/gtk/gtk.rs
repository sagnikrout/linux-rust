//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/ui/gtk/gtk.h
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
pub const _PERF_GTK_H_: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_gtk_context {
    pub main_window: *mut GtkWidget,
    pub notebook: *mut GtkWidget,

    pub info_bar: *mut GtkWidget,
    pub message_label: *mut GtkWidget,

    pub statbar: *mut GtkWidget,
    pub statbar_ctx_id: guint,
}

extern "C" {
    pub fn perf_gtk__init() -> c_int;
}
extern "C" {
    pub fn perf_gtk__exit(wait_for_ok: bool);
}
extern "C" {
    pub fn perf_gtk__deactivate_context(ctx: *mut perf_gtk_context) -> c_int;
}
extern "C" {
    pub fn perf_gtk__init_helpline();
}
extern "C" {
    pub fn gtk_ui_progress__init();
}
extern "C" {
    pub fn perf_gtk__init_hpp();
}
extern "C" {
    pub fn perf_gtk__signal(sig: c_int);
}
extern "C" {
    pub fn perf_gtk__resize_window(window: *mut GtkWidget);
}

extern "C" {
    pub fn perf_gtk__show_annotations();
}
