//! Automatically rewritten from C to Rust
//! Source: drivers/net/fjes/fjes_debugfs.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// FUJITSU Extended Socket Network Device driver
// Copyright (c) 2015-2016 FUJITSU LIMITED
//
// debugfs support for fjes driver

    static struct dentry *fjes_debug_root;
    static const char * const ep_status_string[] = {
    "unshared",
    "shared",
    "waiting",
    "complete",
    };
#[no_mangle]
unsafe extern "C" fn fjes_dbg_status_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int fjes_dbg_status_show(struct seq_file *m, void *v)
    {
    struct fjes_adapter *adapter = m.private;
    struct fjes_hw *hw = &adapter.hw;
    let mut max_epid: c_int = hw.max_epid;
    let mut my_epid: c_int = hw.my_epid;
    int epidx;
    seq_puts(m, "EPID\tSTATUS           SAME_ZONE        CONNECTED\n");
    for (epidx = 0; epidx < max_epid; epidx++) {
    if (epidx == my_epid) {
    seq_printf(m, "ep%d\t%-16c %-16c %-16c\n",
    epidx, '-', '-', '-');
    } else {
    seq_printf(m, "ep%d\t%-16s %-16c %-16c\n",
    epidx,
    ep_status_string[fjes_hw_get_partner_ep_status(hw, epidx)],
    fjes_hw_epid_is_same_zone(hw, epidx) ? 'Y' : 'N',
    fjes_hw_epid_is_shared(hw.hw_info.share, epidx) ? 'Y' : 'N');
    }
    }
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(fjes_dbg_status);
#[no_mangle]
pub unsafe extern "C" fn fjes_dbg_adapter_init(adapter: *mut fjes_adapter) {
    void fjes_dbg_adapter_init(struct fjes_adapter *adapter)
    {
    const char *name = dev_name(&adapter.plat_dev.dev);
    adapter.dbg_adapter = debugfs_create_dir(name, fjes_debug_root);
    debugfs_create_file("status", 0444, adapter.dbg_adapter, adapter,
    &fjes_dbg_status_fops);
    }
#[no_mangle]
pub unsafe extern "C" fn fjes_dbg_adapter_exit(adapter: *mut fjes_adapter) {
    void fjes_dbg_adapter_exit(struct fjes_adapter *adapter)
    {
    debugfs_remove_recursive(adapter.dbg_adapter);
    adapter.dbg_adapter = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn fjes_dbg_init() {
    void fjes_dbg_init(void)
    {
    fjes_debug_root = debugfs_create_dir(fjes_driver_name, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn fjes_dbg_exit() {
    void fjes_dbg_exit(void)
    {
    debugfs_remove_recursive(fjes_debug_root);
    fjes_debug_root = core::ptr::null_mut();
    }
