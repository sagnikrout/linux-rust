//! Automatically rewritten from C to Rust
//! Source: tools/perf/ui/gtk/browser.c
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
pub unsafe extern "C" fn perf_gtk__signal(sig: c_int) {
    void perf_gtk__signal(int sig)
    {
    perf_gtk__exit(false);
    psignal(sig, "perf");
    }
#[no_mangle]
pub unsafe extern "C" fn perf_gtk__resize_window(window: *mut GtkWidget) {
    void perf_gtk__resize_window(GtkWidget *window)
    {
    GdkRectangle rect;
    GdkScreen *screen;
    int monitor;
    int height;
    int width;
    screen = gtk_widget_get_screen(window);
    monitor = gdk_screen_get_monitor_at_window(screen, window.window);
    gdk_screen_get_monitor_geometry(screen, monitor, &rect);
    width	= rect.width * 3 / 4;
    height	= rect.height * 3 / 4;
    gtk_window_resize(GTK_WINDOW(window), width, height);
    }
    const char *perf_gtk__get_percent_color(double percent)
    {
    if (percent >= MIN_RED)
    return "<span fgcolor='red'>";
    if (percent >= MIN_GREEN)
    return "<span fgcolor='dark green'>";
    return core::ptr::null_mut();
    }

    GtkWidget *perf_gtk__setup_info_bar(void)
    {
    GtkWidget *info_bar;
    GtkWidget *label;
    GtkWidget *content_area;
    info_bar = gtk_info_bar_new();
    gtk_widget_set_no_show_all(info_bar, TRUE);
    label = gtk_label_new("");
    gtk_widget_show(label);
    content_area = gtk_info_bar_get_content_area(GTK_INFO_BAR(info_bar));
    gtk_container_add(GTK_CONTAINER(content_area), label);
    gtk_info_bar_add_button(GTK_INFO_BAR(info_bar), GTK_STOCK_OK,
    GTK_RESPONSE_OK);
    g_signal_connect(info_bar, "response",
    G_CALLBACK(gtk_widget_hide), core::ptr::null_mut());
    pgctx.info_bar = info_bar;
    pgctx.message_label = label;
    return info_bar;
    }

    GtkWidget *perf_gtk__setup_statusbar(void)
    {
    GtkWidget *stbar;
    unsigned ctxid;
    stbar = gtk_statusbar_new();
    ctxid = gtk_statusbar_get_context_id(GTK_STATUSBAR(stbar),
    "perf report");
    pgctx.statbar = stbar;
    pgctx.statbar_ctx_id = ctxid;
    return stbar;
    }
