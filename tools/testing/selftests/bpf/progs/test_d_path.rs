//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_d_path.c
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

pub const MAX_PATH_LEN: c_int = 128;
pub const MAX_FILES: c_int = 7;
    let mut my_pid: pid_t = 0;
    let mut cnt_stat: __u32 = 0;
    let mut cnt_close: __u32 = 0;
    char paths_stat[MAX_FILES][MAX_PATH_LEN] = {};
    char paths_close[MAX_FILES][MAX_PATH_LEN] = {};
    int rets_stat[MAX_FILES] = {};
    int rets_close[MAX_FILES] = {};
    let mut called_stat: c_int = 0;
    let mut called_close: c_int = 0;
    let mut path_match_fallocate: c_int = 0;
    SEC("fentry/security_inode_getattr")
    int BPF_PROG(prog_stat, struct path *path, struct kstat *stat,
    __u32 request_mask, unsigned int query_flags)
    {
    let mut pid: pid_t = bpf_get_current_pid_tgid() >> 32;
    let mut cnt: __u32 = cnt_stat;
    int ret;
    called_stat = 1;
    if (pid != my_pid)
    return 0;
    if (cnt >= MAX_FILES)
    return 0;
    ret = bpf_d_path(path, paths_stat[cnt], MAX_PATH_LEN);
    rets_stat[cnt] = ret;
    cnt_stat++;
    return 0;
    }
    SEC("fentry/filp_close")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: prog_close, file: *mut file, id: *mut c_void) -> c_int {
    int BPF_PROG(prog_close, struct file *file, void *id)
    {
    let mut pid: pid_t = bpf_get_current_pid_tgid() >> 32;
    let mut cnt: __u32 = cnt_close;
    int ret;
    called_close = 1;
    if (pid != my_pid)
    return 0;
    if (cnt >= MAX_FILES)
    return 0;
    ret = bpf_d_path(&file.f_path,
    paths_close[cnt], MAX_PATH_LEN);
    rets_close[cnt] = ret;
    cnt_close++;
    return 0;
    }
    SEC("fentry/vfs_fallocate")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: prog_fallocate, file: *mut file, mode: c_int, offset: loff_t, len: loff_t) -> c_int {
    int BPF_PROG(prog_fallocate, struct file *file, int mode, loff_t offset, loff_t len)
    {
    let mut pid: pid_t = bpf_get_current_pid_tgid() >> 32;
    let mut ret: c_int = 0;
    char path_fallocate[MAX_PATH_LEN] = {};
    if (pid != my_pid)
    return 0;
    ret = bpf_d_path(&file.f_path,
    path_fallocate, MAX_PATH_LEN);
    if (ret < 0)
    return 0;
    if (!path_fallocate[0])
    return 0;
    path_match_fallocate = 1;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
