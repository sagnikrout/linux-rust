//! Automatically rewritten from C to Rust
//! Source: fs/nfsd/debugfs.c
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

    static struct dentry *nfsd_top_dir __read_mostly;
//
// /sys/kernel/debug/nfsd/disable-splice-read
//
// Contents:
// %0: NFS READ is allowed to use page splicing
// %1: NFS READ uses only iov iter read
//
// The default value of this setting is zero (page splicing is
// allowed). This setting takes immediate effect for all NFS
// versions, all exports, and in all NFSD net namespaces.
//
#[no_mangle]
unsafe extern "C" fn nfsd_dsr_get(data: *mut c_void, val: *mut u64) -> c_int {
    static int nfsd_dsr_get(void *data, u64 *val)
    {
// val = nfsd_disable_splice_read ? 1 : 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nfsd_dsr_set(data: *mut c_void, val: u64) -> c_int {
    static int nfsd_dsr_set(void *data, u64 val)
    {
    nfsd_disable_splice_read = (val > 0);
    if (!nfsd_disable_splice_read) {
//
// Must use buffered I/O if splice_read is enabled.
//
    nfsd_io_cache_read = NFSD_IO_BUFFERED;
    }
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(nfsd_dsr_fops, nfsd_dsr_get, nfsd_dsr_set, "%llu\n");
//
// /sys/kernel/debug/nfsd/io_cache_read
//
// Contents:
// %0: NFS READ will use buffered IO
// %1: NFS READ will use dontcache (buffered IO w/ dropbehind)
// %2: NFS READ will use direct IO
//
// This setting takes immediate effect for all NFS versions,
// all exports, and in all NFSD net namespaces.
//
#[no_mangle]
unsafe extern "C" fn nfsd_io_cache_read_get(data: *mut c_void, val: *mut u64) -> c_int {
    static int nfsd_io_cache_read_get(void *data, u64 *val)
    {
// val = nfsd_io_cache_read;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nfsd_io_cache_read_set(data: *mut c_void, val: u64) -> c_int {
    static int nfsd_io_cache_read_set(void *data, u64 val)
    {
    let mut ret: c_int = 0;
    switch (val) {
    case NFSD_IO_BUFFERED:
    nfsd_io_cache_read = NFSD_IO_BUFFERED;
    break;
    case NFSD_IO_DONTCACHE:
    case NFSD_IO_DIRECT:
//
// Must disable splice_read when enabling
// NFSD_IO_DONTCACHE.
//
    nfsd_disable_splice_read = true;
    nfsd_io_cache_read = val;
    break;
    default:
    ret = -EINVAL;
    break;
    }
    return ret;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(nfsd_io_cache_read_fops, nfsd_io_cache_read_get,
    nfsd_io_cache_read_set, "%llu\n");
//
// /sys/kernel/debug/nfsd/io_cache_write
//
// Contents:
// %0: NFS WRITE will use buffered IO
// %1: NFS WRITE will use dontcache (buffered IO w/ dropbehind)
//
// This setting takes immediate effect for all NFS versions,
// all exports, and in all NFSD net namespaces.
//
#[no_mangle]
unsafe extern "C" fn nfsd_io_cache_write_get(data: *mut c_void, val: *mut u64) -> c_int {
    static int nfsd_io_cache_write_get(void *data, u64 *val)
    {
// val = nfsd_io_cache_write;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nfsd_io_cache_write_set(data: *mut c_void, val: u64) -> c_int {
    static int nfsd_io_cache_write_set(void *data, u64 val)
    {
    let mut ret: c_int = 0;
    switch (val) {
    case NFSD_IO_BUFFERED:
    case NFSD_IO_DONTCACHE:
    case NFSD_IO_DIRECT:
    nfsd_io_cache_write = val;
    break;
    default:
    ret = -EINVAL;
    break;
    }
    return ret;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(nfsd_io_cache_write_fops, nfsd_io_cache_write_get,
    nfsd_io_cache_write_set, "%llu\n");
#[no_mangle]
pub unsafe extern "C" fn nfsd_debugfs_exit() {
    void nfsd_debugfs_exit(void)
    {
    debugfs_remove_recursive(nfsd_top_dir);
    nfsd_top_dir = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn nfsd_debugfs_init() {
    void nfsd_debugfs_init(void)
    {
    nfsd_top_dir = debugfs_create_dir("nfsd", core::ptr::null_mut());
    debugfs_create_file("disable-splice-read", S_IWUSR | S_IRUGO,
    nfsd_top_dir, core::ptr::null_mut(), &nfsd_dsr_fops);
    debugfs_create_file("io_cache_read", 0644, nfsd_top_dir, core::ptr::null_mut(),
    &nfsd_io_cache_read_fops);
    debugfs_create_file("io_cache_write", 0644, nfsd_top_dir, core::ptr::null_mut(),
    &nfsd_io_cache_write_fops);

    debugfs_create_bool("delegated_timestamps", 0644, nfsd_top_dir,
    &nfsd_delegts_enabled);

    }
