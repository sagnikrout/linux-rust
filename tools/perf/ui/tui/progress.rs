//! Automatically rewritten from C to Rust
//! Source: tools/perf/ui/tui/progress.c
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
unsafe extern "C" fn __tui_progress__init(p: *mut ui_progress) {
    static void __tui_progress__init(struct ui_progress *p)
    {
    p.next = p.step = p.total / (SLtt_Screen_Cols - 2) ?: 1;
    }
#[no_mangle]
unsafe extern "C" fn get_title(p: *mut ui_progress, buf: *mut c_char, size: usize) -> c_int {
    static int get_title(struct ui_progress *p, char *buf, size_t size)
    {
    char buf_cur[20];
    char buf_tot[20];
    int ret;
    ret  = unit_number__scnprintf(buf_cur, sizeof(buf_cur), p.curr);
    ret += unit_number__scnprintf(buf_tot, sizeof(buf_tot), p.total);
    return ret + scnprintf(buf, size, "%s [%s/%s]",
    p.title, buf_cur, buf_tot);
    }
#[no_mangle]
unsafe extern "C" fn tui_progress__update(p: *mut ui_progress) {
    static void tui_progress__update(struct ui_progress *p)
    {
    char buf[100], *title = (char *) p.title;
    int bar, y;
//
// FIXME: We should have a per UI backend way of showing progress,
// stdio will just show a percentage as NN%, etc.
//
    if (use_browser <= 0)
    return;
    if (p.total == 0)
    return;
    if (p.size) {
    get_title(p, buf, sizeof(buf));
    title = buf;
    }
    ui__refresh_dimensions(false);
    mutex_lock(&ui__lock);
    y = SLtt_Screen_Rows / 2 - 2;
    SLsmg_set_color(0);
    SLsmg_draw_box(y, 0, 3, SLtt_Screen_Cols);
    SLsmg_gotorc(y++, 1);
    SLsmg_write_string(title);
    SLsmg_fill_region(y, 1, 1, SLtt_Screen_Cols - 2, ' ');
    SLsmg_set_color(HE_COLORSET_SELECTED);
    bar = ((SLtt_Screen_Cols - 2) * p.curr) / p.total;
    SLsmg_fill_region(y, 1, 1, bar, ' ');
    SLsmg_refresh();
    mutex_unlock(&ui__lock);
    }
#[no_mangle]
unsafe extern "C" fn tui_progress__finish() {
    static void tui_progress__finish(void)
    {
    int y;
    if (use_browser <= 0)
    return;
    ui__refresh_dimensions(false);
    mutex_lock(&ui__lock);
    y = SLtt_Screen_Rows / 2 - 2;
    SLsmg_set_color(0);
    SLsmg_fill_region(y, 0, 3, SLtt_Screen_Cols, ' ');
    SLsmg_refresh();
    mutex_unlock(&ui__lock);
    }
    static struct ui_progress_ops tui_progress__ops = {
    .init   = __tui_progress__init,
    .update = tui_progress__update,
    .finish = tui_progress__finish,
    };
#[no_mangle]
pub unsafe extern "C" fn tui_progress__init() {
    void tui_progress__init(void)
    {
    ui_progress__ops = &tui_progress__ops;
    }
