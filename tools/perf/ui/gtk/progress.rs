//! Automatically rewritten from C to Rust
//! Source: tools/perf/ui/gtk/progress.c
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

    static GtkWidget *dialog;
    static GtkWidget *progress;
#[no_mangle]
unsafe extern "C" fn gtk_ui_progress__update(p: *mut ui_progress) {
    static void gtk_ui_progress__update(struct ui_progress *p)
    {
    let mut fraction: double = p.total ? 1.0 * p.curr / p.total : 0.0;
    char buf[1024];
    if (dialog == core::ptr::null_mut()) {
    GtkWidget *vbox = gtk_vbox_new(TRUE, 5);
    GtkWidget *label = gtk_label_new(p.title);
    dialog = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    progress = gtk_progress_bar_new();
    gtk_box_pack_start(GTK_BOX(vbox), label, TRUE, FALSE, 3);
    gtk_box_pack_start(GTK_BOX(vbox), progress, TRUE, TRUE, 3);
    gtk_container_add(GTK_CONTAINER(dialog), vbox);
    gtk_window_set_title(GTK_WINDOW(dialog), "perf");
    gtk_window_resize(GTK_WINDOW(dialog), 300, 80);
    gtk_window_set_position(GTK_WINDOW(dialog), GTK_WIN_POS_CENTER);
    gtk_widget_show_all(dialog);
    }
    gtk_progress_bar_set_fraction(GTK_PROGRESS_BAR(progress), fraction);
    snprintf(buf, sizeof(buf), "%"PRIu64" / %"PRIu64, p.curr, p.total);
    gtk_progress_bar_set_text(GTK_PROGRESS_BAR(progress), buf);
// we didn't call gtk_main yet, so do it manually
    while (gtk_events_pending())
    gtk_main_iteration();
    }
#[no_mangle]
unsafe extern "C" fn gtk_ui_progress__finish() {
    static void gtk_ui_progress__finish(void)
    {
// this will also destroy all of its children
    gtk_widget_destroy(dialog);
    dialog = core::ptr::null_mut();
    }
    static struct ui_progress_ops gtk_ui_progress__ops = {
    .update		= gtk_ui_progress__update,
    .finish		= gtk_ui_progress__finish,
    };
#[no_mangle]
pub unsafe extern "C" fn gtk_ui_progress__init() {
    void gtk_ui_progress__init(void)
    {
    ui_progress__ops = &gtk_ui_progress__ops;
    }
