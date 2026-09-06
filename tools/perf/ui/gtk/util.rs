//! Automatically rewritten from C to Rust
//! Source: tools/perf/ui/gtk/util.c
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

    struct perf_gtk_context *pgctx;
    struct perf_gtk_context *perf_gtk__activate_context(GtkWidget *window)
    {
    struct perf_gtk_context *ctx;
    ctx = malloc(sizeof(*pgctx));
    if (ctx)
    ctx.main_window = window;
    return ctx;
    }
#[no_mangle]
pub unsafe extern "C" fn perf_gtk__deactivate_context(ctx: *mut perf_gtk_context) -> c_int {
    int perf_gtk__deactivate_context(struct perf_gtk_context **ctx)
    {
    if (!perf_gtk__is_active_context(*ctx))
    return -1;
    zfree(ctx);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn perf_gtk__error(format: *const c_char, args: va_list) -> c_int {
    static int perf_gtk__error(const char *format, va_list args)
    {
    char *msg;
    GtkWidget *dialog;
    if (!perf_gtk__is_active_context(pgctx) ||
    vasprintf(&msg, format, args) < 0) {
    fprintf(stderr, "Error:\n");
    vfprintf(stderr, format, args);
    fprintf(stderr, "\n");
    return -1;
    }
    dialog = gtk_message_dialog_new_with_markup(GTK_WINDOW(pgctx.main_window),
    GTK_DIALOG_DESTROY_WITH_PARENT,
    GTK_MESSAGE_ERROR,
    GTK_BUTTONS_CLOSE,
    "<b>Error</b>\n\n%s", msg);
    gtk_dialog_run(GTK_DIALOG(dialog));
    gtk_widget_destroy(dialog);
    free(msg);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn perf_gtk__warning_info_bar(format: *const c_char, args: va_list) -> c_int {
    static int perf_gtk__warning_info_bar(const char *format, va_list args)
    {
    char *msg;
    if (!perf_gtk__is_active_context(pgctx) ||
    vasprintf(&msg, format, args) < 0) {
    fprintf(stderr, "Warning:\n");
    vfprintf(stderr, format, args);
    fprintf(stderr, "\n");
    return -1;
    }
    gtk_label_set_text(GTK_LABEL(pgctx.message_label), msg);
    gtk_info_bar_set_message_type(GTK_INFO_BAR(pgctx.info_bar),
    GTK_MESSAGE_WARNING);
    gtk_widget_show(pgctx.info_bar);
    free(msg);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn perf_gtk__warning_statusbar(format: *const c_char, args: va_list) -> c_int {
    static int perf_gtk__warning_statusbar(const char *format, va_list args)
    {
    char *msg, *p;
    if (!perf_gtk__is_active_context(pgctx) ||
    vasprintf(&msg, format, args) < 0) {
    fprintf(stderr, "Warning:\n");
    vfprintf(stderr, format, args);
    fprintf(stderr, "\n");
    return -1;
    }
    gtk_statusbar_pop(GTK_STATUSBAR(pgctx.statbar),
    pgctx.statbar_ctx_id);
// Only first line can be displayed
    p = strchr(msg, '\n');
    if (p)
// p = '\0';
    gtk_statusbar_push(GTK_STATUSBAR(pgctx.statbar),
    pgctx.statbar_ctx_id, msg);
    free(msg);
    return 0;
    }

    struct perf_error_ops perf_gtk_eops = {
    .error		= perf_gtk__error,

    .warning	= perf_gtk__warning_info_bar,

    .warning	= perf_gtk__warning_statusbar,

    };
