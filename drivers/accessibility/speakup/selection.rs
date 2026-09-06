//! Automatically rewritten from C to Rust
//! Source: drivers/accessibility/speakup/selection.c
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

    u16 spk_xs, spk_ys, spk_xe, spk_ye; /* our region points */
    struct vc_data *spk_sel_cons;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct speakup_selection_work {
    pub work: work_struct,
    pub sel: tiocl_selection,
    pub tty: *mut tty_struct,
}

#[no_mangle]
unsafe extern "C" fn __speakup_set_selection(work: *mut work_struct) {
    static void __speakup_set_selection(struct work_struct *work)
    {
    struct speakup_selection_work *ssw =
    container_of(work, struct speakup_selection_work, work);
    struct tty_struct *tty;
    struct tiocl_selection sel;
    sel = ssw.sel;
// this ensures we copy sel before releasing the lock below
    rmb();
// release the lock by setting tty of the struct to NULL
    tty = xchg(&ssw.tty, core::ptr::null_mut());
    if (spk_sel_cons != vc_cons[fg_console].d) {
    spk_sel_cons = vc_cons[fg_console].d;
    pr_warn("Selection: mark console not the same as cut\n");
    goto unref;
    }
    console_lock();
    clear_selection();
    console_unlock();
    set_selection_kernel(&sel, tty);
    unref:
    tty_kref_put(tty);
    }
    static struct speakup_selection_work speakup_sel_work = {
    .work = __WORK_INITIALIZER(speakup_sel_work.work,
    __speakup_set_selection)
    };
#[no_mangle]
pub unsafe extern "C" fn speakup_set_selection(tty: *mut tty_struct) -> c_int {
    int speakup_set_selection(struct tty_struct *tty)
    {
// we get kref here first in order to avoid a subtle race when
// cancelling selection work. getting kref first establishes the
// invariant that if speakup_sel_work.tty is not NULL when
// speakup_cancel_selection() is called, it must be the case that a put
// kref is pending.
//
    tty_kref_get(tty);
    if (cmpxchg(&speakup_sel_work.tty, core::ptr::null_mut(), tty)) {
    tty_kref_put(tty);
    return -EBUSY;
    }
// now we have the 'lock' by setting tty member of
// speakup_selection_work. wmb() ensures that writes to
// speakup_sel_work don't happen before cmpxchg() above.
//
    wmb();
    speakup_sel_work.sel.xs = spk_xs + 1;
    speakup_sel_work.sel.ys = spk_ys + 1;
    speakup_sel_work.sel.xe = spk_xe + 1;
    speakup_sel_work.sel.ye = spk_ye + 1;
    speakup_sel_work.sel.sel_mode = TIOCL_SELCHAR;
    schedule_work_on(WORK_CPU_UNBOUND, &speakup_sel_work.work);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn speakup_cancel_selection() {
    void speakup_cancel_selection(void)
    {
    struct tty_struct *tty;
    cancel_work_sync(&speakup_sel_work.work);
// setting to null so that if work fails to run and we cancel it,
// we can run it again without getting EBUSY forever from there on.
// we need to use xchg here to avoid race with speakup_set_selection()
//
    tty = xchg(&speakup_sel_work.tty, core::ptr::null_mut());
    if (tty)
    tty_kref_put(tty);
    }
#[no_mangle]
unsafe extern "C" fn __speakup_paste_selection(work: *mut work_struct) {
    static void __speakup_paste_selection(struct work_struct *work)
    {
    struct speakup_selection_work *ssw =
    container_of(work, struct speakup_selection_work, work);
    struct tty_struct *tty = xchg(&ssw.tty, core::ptr::null_mut());
    paste_selection(tty);
    tty_kref_put(tty);
    }
    static struct speakup_selection_work speakup_paste_work = {
    .work = __WORK_INITIALIZER(speakup_paste_work.work,
    __speakup_paste_selection)
    };
#[no_mangle]
pub unsafe extern "C" fn speakup_paste_selection(tty: *mut tty_struct) -> c_int {
    int speakup_paste_selection(struct tty_struct *tty)
    {
    tty_kref_get(tty);
    if (cmpxchg(&speakup_paste_work.tty, core::ptr::null_mut(), tty)) {
    tty_kref_put(tty);
    return -EBUSY;
    }
    schedule_work_on(WORK_CPU_UNBOUND, &speakup_paste_work.work);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn speakup_cancel_paste() {
    void speakup_cancel_paste(void)
    {
    struct tty_struct *tty;
    cancel_work_sync(&speakup_paste_work.work);
    tty = xchg(&speakup_paste_work.tty, core::ptr::null_mut());
    if (tty)
    tty_kref_put(tty);
    }
