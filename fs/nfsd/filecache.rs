//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nfsd/filecache.h
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


//
// Limit the time that the list_lru_one lock is held during
// an LRU scan.
//

//
// This is the fsnotify_mark container that nfsd attaches to the files that it
// is holding open. Note that we have a separate refcount here aside from the
// one in the fsnotify_mark. We only want a single fsnotify_mark attached to
// the inode, and for each nfsd_file to hold a reference to it.
//
// The fsnotify_mark is itself refcounted, but that's not sufficient to tell us
// how to put that reference. If there are still outstanding nfsd_files that
// reference the mark, then we would want to call fsnotify_put_mark on it.
// If there were not, then we'd need to call fsnotify_destroy_mark. Since we
// can't really tell the difference, we use the nfm_mark to keep track of how
// many nfsd_files hold references to the mark. When that counter goes to zero
// then we know to call fsnotify_destroy_mark on it.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd_file_mark {
    pub nfm_mark: fsnotify_mark,
    pub nfm_ref: refcount_t,
// serializes nfsd_fsnotify_recalc_mask() against itself
    pub nfm_recalc_mutex: mutex,
}

//
// A representation of a file that has been opened by knfsd. These are hashed
// in the hashtable by inode pointer value. Note that this object doesn't
// hold a reference to the inode by itself, so the nf_inode pointer should
// never be dereferenced, only used for comparison.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd_file {
    pub nf_rlist: rhlist_head,
    pub nf_inode: *mut c_void,
    pub nf_file: *mut file,
    pub nf_cred: *const cred,
    pub nf_net: *mut net,

    pub nf_flags: c_ulong,
    pub nf_ref: refcount_t,
    pub nf_may: c_uchar,
    pub nf_mark: *mut nfsd_file_mark,
    pub nf_lru: list_head,
    pub nf_gc: list_head,
    pub nf_rcu: rcu_head,
    pub nf_birthtime: ktime_t,
    pub nf_dio_mem_align: u32,
    pub nf_dio_offset_align: u32,
    pub nf_dio_read_offset_align: u32,
}

extern "C" {
    pub fn nfsd_file_cache_init() -> c_int;
}
extern "C" {
    pub fn nfsd_file_cache_purge(: *mut net);
}
extern "C" {
    pub fn nfsd_file_cache_shutdown();
}
extern "C" {
    pub fn nfsd_file_cache_start_net(net: *mut net) -> c_int;
}
extern "C" {
    pub fn nfsd_file_cache_shutdown_net(net: *mut net);
}
extern "C" {
    pub fn nfsd_file_put(nf: *mut nfsd_file);
}
extern "C" {
    pub fn nfsd_file_close_inode_sync(inode: *mut inode);
}
extern "C" {
    pub fn nfsd_file_close_export(net: *mut net, path: *const path);
}
extern "C" {
    pub fn nfsd_file_net_dispose(nn: *mut nfsd_net);
}
extern "C" {
    pub fn nfsd_file_is_cached(inode: *mut inode) -> bool;
}
extern "C" {
    pub fn nfsd_file_cache_stats_show(m: *mut seq_file, v: *mut c_void) -> c_int;
}
extern "C" {
    pub fn nfsd_fsnotify_recalc_mask(nf: *mut nfsd_file);
}
