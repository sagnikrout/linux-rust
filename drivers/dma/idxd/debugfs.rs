//! Automatically rewritten from C to Rust
//! Source: drivers/dma/idxd/debugfs.c
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
// Copyright(c) 2021 Intel Corporation. All rights rsvd.

    static struct dentry *idxd_debugfs_dir;
    static void dump_event_entry(struct idxd_device *idxd, struct seq_file *s,
    u16 index, int *count, bool processed)
    {
    struct idxd_evl *evl = idxd.evl;
    struct dsa_evl_entry *entry;
    struct dsa_completion_record *cr;
    u64 *raw;
    int i;
    let mut evl_strides: c_int = evl_ent_size(idxd) / sizeof(u64);
    entry = (struct dsa_evl_entry *)evl.log + index;
    if (!entry.e.desc_valid)
    return;
    seq_printf(s, "Event Log entry %d (real index %u) processed: %u\n",
// count, index, processed);
    seq_printf(s, "desc valid %u wq idx valid %u\n"
    "batch %u fault rw %u priv %u error 0x%x\n"
    "wq idx %u op %#x pasid %u batch idx %u\n"
    "fault addr %#llx\n",
    entry.e.desc_valid, entry.e.wq_idx_valid,
    entry.e.batch, entry.e.fault_rw, entry.e.priv,
    entry.e.error, entry.e.wq_idx, entry.e.operation,
    entry.e.pasid, entry.e.batch_idx, entry.e.fault_addr);
    cr = &entry.cr;
    seq_printf(s, "status %#x result %#x fault_info %#x bytes_completed %u\n"
    "fault addr %#llx inv flags %#x\n\n",
    cr.status, cr.result, cr.fault_info, cr.bytes_completed,
    cr.fault_addr, cr.invalid_flags);
    raw = (u64 *)entry;
    for (i = 0; i < evl_strides; i++)
    seq_printf(s, "entry[%d] = %#llx\n", i, raw[i]);
    seq_puts(s, "\n");
// count += 1;
    }
#[no_mangle]
unsafe extern "C" fn debugfs_evl_show(s: *mut seq_file, d: *mut c_void) -> c_int {
    static int debugfs_evl_show(struct seq_file *s, void *d)
    {
    struct idxd_device *idxd = s.private;
    struct idxd_evl *evl = idxd.evl;
    union evl_status_reg evl_status;
    u16 h, t, evl_size, i;
    let mut count: c_int = 0;
    let mut processed: bool = true;
    if (!evl || !evl.log)
    return 0;
    mutex_lock(&evl.lock);
    evl_status.bits = ioread64(idxd.reg_base + IDXD_EVLSTATUS_OFFSET);
    t = evl_status.tail;
    h = evl_status.head;
    evl_size = evl.size;
    seq_printf(s, "Event Log head %u tail %u interrupt pending %u\n\n",
    evl_status.head, evl_status.tail, evl_status.int_pending);
    i = t;
    while (1) {
    i = (i + 1) % evl_size;
    if (i == t)
    break;
    if (processed && i == h)
    processed = false;
    dump_event_entry(idxd, s, i, &count, processed);
    }
    mutex_unlock(&evl.lock);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(debugfs_evl);
#[no_mangle]
pub unsafe extern "C" fn idxd_device_init_debugfs(idxd: *mut idxd_device) -> c_int {
    int idxd_device_init_debugfs(struct idxd_device *idxd)
    {
    if (IS_ERR_OR_NULL(idxd_debugfs_dir))
    return 0;
    idxd.dbgfs_dir = debugfs_create_dir(dev_name(idxd_confdev(idxd)), idxd_debugfs_dir);
    if (IS_ERR(idxd.dbgfs_dir))
    return PTR_ERR(idxd.dbgfs_dir);
    if (idxd.evl) {
    idxd.dbgfs_evl_file = debugfs_create_file("event_log", 0400,
    idxd.dbgfs_dir, idxd,
    &debugfs_evl_fops);
    if (IS_ERR(idxd.dbgfs_evl_file)) {
    debugfs_remove_recursive(idxd.dbgfs_dir);
    idxd.dbgfs_dir = core::ptr::null_mut();
    return PTR_ERR(idxd.dbgfs_evl_file);
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn idxd_device_remove_debugfs(idxd: *mut idxd_device) {
    void idxd_device_remove_debugfs(struct idxd_device *idxd)
    {
    debugfs_remove_recursive(idxd.dbgfs_dir);
    }
#[no_mangle]
pub unsafe extern "C" fn idxd_init_debugfs() -> c_int {
    int idxd_init_debugfs(void)
    {
    if (!debugfs_initialized())
    return 0;
    idxd_debugfs_dir = debugfs_create_dir(KBUILD_MODNAME, core::ptr::null_mut());
    if (IS_ERR(idxd_debugfs_dir))
    return  PTR_ERR(idxd_debugfs_dir);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn idxd_remove_debugfs() {
    void idxd_remove_debugfs(void)
    {
    debugfs_remove_recursive(idxd_debugfs_dir);
    }
