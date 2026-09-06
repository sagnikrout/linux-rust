//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/powernv/vas-debug.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 2016-17 IBM Corp.
//

    static struct dentry *vas_debugfs;
    static char *cop_to_str(int cop)
    {
    switch (cop) {
    case VAS_COP_TYPE_FAULT:	return "Fault";
    case VAS_COP_TYPE_842:		return "NX-842 Normal Priority";
    case VAS_COP_TYPE_842_HIPRI:	return "NX-842 High Priority";
    case VAS_COP_TYPE_GZIP:		return "NX-GZIP Normal Priority";
    case VAS_COP_TYPE_GZIP_HIPRI:	return "NX-GZIP High Priority";
    case VAS_COP_TYPE_FTW:		return "Fast Thread-wakeup";
    default:			return "Unknown";
    }
    }
#[no_mangle]
unsafe extern "C" fn info_show(s: *mut seq_file, private: *mut c_void) -> c_int {
    static int info_show(struct seq_file *s, void *private)
    {
    struct pnv_vas_window *window = s.private;
    mutex_lock(&vas_mutex);
// ensure window is not unmapped
    if (!window.hvwc_map)
    goto unlock;
    seq_printf(s, "Type: %s, %s\n", cop_to_str(window.vas_win.cop),
    window.tx_win ? "Send" : "Receive");
    seq_printf(s, "Pid : %d\n", vas_window_pid(&window.vas_win));
    unlock:
    mutex_unlock(&vas_mutex);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(info);
    static inline void print_reg(struct seq_file *s, struct pnv_vas_window *win,
    char *name, u32 reg)
    {
    seq_printf(s, "0x%016llx %s\n", read_hvwc_reg(win, name, reg), name);
    }
#[no_mangle]
unsafe extern "C" fn hvwc_show(s: *mut seq_file, private: *mut c_void) -> c_int {
    static int hvwc_show(struct seq_file *s, void *private)
    {
    struct pnv_vas_window *window = s.private;
    mutex_lock(&vas_mutex);
// ensure window is not unmapped
    if (!window.hvwc_map)
    goto unlock;
    print_reg(s, window, VREG(LPID));
    print_reg(s, window, VREG(PID));
    print_reg(s, window, VREG(XLATE_MSR));
    print_reg(s, window, VREG(XLATE_LPCR));
    print_reg(s, window, VREG(XLATE_CTL));
    print_reg(s, window, VREG(AMR));
    print_reg(s, window, VREG(SEIDR));
    print_reg(s, window, VREG(FAULT_TX_WIN));
    print_reg(s, window, VREG(OSU_INTR_SRC_RA));
    print_reg(s, window, VREG(HV_INTR_SRC_RA));
    print_reg(s, window, VREG(PSWID));
    print_reg(s, window, VREG(LFIFO_BAR));
    print_reg(s, window, VREG(LDATA_STAMP_CTL));
    print_reg(s, window, VREG(LDMA_CACHE_CTL));
    print_reg(s, window, VREG(LRFIFO_PUSH));
    print_reg(s, window, VREG(CURR_MSG_COUNT));
    print_reg(s, window, VREG(LNOTIFY_AFTER_COUNT));
    print_reg(s, window, VREG(LRX_WCRED));
    print_reg(s, window, VREG(LRX_WCRED_ADDER));
    print_reg(s, window, VREG(TX_WCRED));
    print_reg(s, window, VREG(TX_WCRED_ADDER));
    print_reg(s, window, VREG(LFIFO_SIZE));
    print_reg(s, window, VREG(WINCTL));
    print_reg(s, window, VREG(WIN_STATUS));
    print_reg(s, window, VREG(WIN_CTX_CACHING_CTL));
    print_reg(s, window, VREG(TX_RSVD_BUF_COUNT));
    print_reg(s, window, VREG(LRFIFO_WIN_PTR));
    print_reg(s, window, VREG(LNOTIFY_CTL));
    print_reg(s, window, VREG(LNOTIFY_PID));
    print_reg(s, window, VREG(LNOTIFY_LPID));
    print_reg(s, window, VREG(LNOTIFY_TID));
    print_reg(s, window, VREG(LNOTIFY_SCOPE));
    print_reg(s, window, VREG(NX_UTIL_ADDER));
    unlock:
    mutex_unlock(&vas_mutex);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(hvwc);
#[no_mangle]
pub unsafe extern "C" fn vas_window_free_dbgdir(pnv_win: *mut pnv_vas_window) {
    void vas_window_free_dbgdir(struct pnv_vas_window *pnv_win)
    {
    struct vas_window *window =  &pnv_win.vas_win;
    if (window.dbgdir) {
    debugfs_remove_recursive(window.dbgdir);
    kfree(window.dbgname);
    window.dbgdir = core::ptr::null_mut();
    window.dbgname = core::ptr::null_mut();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn vas_window_init_dbgdir(window: *mut pnv_vas_window) {
    void vas_window_init_dbgdir(struct pnv_vas_window *window)
    {
    struct dentry *d;
    if (!window.vinst.dbgdir)
    return;
    window.vas_win.dbgname = kzalloc(16, GFP_KERNEL);
    if (!window.vas_win.dbgname)
    return;
    snprintf(window.vas_win.dbgname, 16, "w%d", window.vas_win.winid);
    d = debugfs_create_dir(window.vas_win.dbgname, window.vinst.dbgdir);
    window.vas_win.dbgdir = d;
    debugfs_create_file("info", 0444, d, window, &info_fops);
    debugfs_create_file("hvwc", 0444, d, window, &hvwc_fops);
    }
#[no_mangle]
pub unsafe extern "C" fn vas_instance_init_dbgdir(vinst: *mut vas_instance) {
    void vas_instance_init_dbgdir(struct vas_instance *vinst)
    {
    struct dentry *d;
    vas_init_dbgdir();
    vinst.dbgname = kzalloc(16, GFP_KERNEL);
    if (!vinst.dbgname)
    return;
    snprintf(vinst.dbgname, 16, "v%d", vinst.vas_id);
    d = debugfs_create_dir(vinst.dbgname, vas_debugfs);
    vinst.dbgdir = d;
    }
//
// Set up the "root" VAS debugfs dir. Return if we already set it up
// (or failed to) in an earlier instance of VAS.
//
#[no_mangle]
pub unsafe extern "C" fn vas_init_dbgdir() {
    void vas_init_dbgdir(void)
    {
    let mut first_time: static bool = true;
    if (!first_time)
    return;
    first_time = false;
    vas_debugfs = debugfs_create_dir("vas", core::ptr::null_mut());
    }
