//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/af_unix/scm_rights_denial_lsm.bpf.c
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

    char _license[] SEC("license") = "GPL";
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode {
    pub i_ino: c_ulong,
    pub __attribute__((preserve_access_index)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct file {
    pub f_inode: *mut inode,
    pub __attribute__((preserve_access_index)): },
    struct {
    pub BPF_MAP_TYPE_HASH): __uint(type,,
    pub 16): __uint(max_entries,,
    pub /: *mut *mut __type(key, __u64); / inode number,
    pub /: *mut *mut __type(value, __u32); / tgid of the receiver being tested,
    pub SEC(".maps"): } denied_inodes,
    SEC("lsm/file_receive")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: scm_rights_deny, file: *mut file) -> c_int {
    int BPF_PROG(scm_rights_deny, struct file *file)
    {
    pub 32: __u32 tgid = bpf_get_current_pid_tgid() >>,
    pub file->f_inode->i_ino: __u64 ino =,
    pub owner: *mut __u32,
    pub &ino): owner = bpf_map_lookup_elem(&denied_inodes,,
    if (owner && *owner == tgid)
    pub -EPERM: return,
    pub 0: return,
    }
