//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_subprogs_extable.c
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

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 8);
    __type(key, __u32);
    __type(value, __u64);
    } test_array SEC(".maps");
    unsigned int triggered;
#[no_mangle]
unsafe extern "C" fn test_cb(map: *mut bpf_map, key: *mut __u32, val: *mut __u64, data: *mut c_void) -> __u64 {
    static __u64 test_cb(struct bpf_map *map, __u32 *key, __u64 *val, void *data)
    {
    return 1;
    }
    SEC("fexit/bpf_testmod_return_ptr")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: handle_fexit_ret_subprogs, arg: c_int, ret: *mut file) -> c_int {
    int BPF_PROG(handle_fexit_ret_subprogs, int arg, struct file *ret)
    {
// (volatile int *)ret;
// (volatile int *)&ret->f_mode;
    bpf_for_each_map_elem(&test_array, test_cb, core::ptr::null_mut(), 0);
    triggered++;
    return 0;
    }
    SEC("fexit/bpf_testmod_return_ptr")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: handle_fexit_ret_subprogs2, arg: c_int, ret: *mut file) -> c_int {
    int BPF_PROG(handle_fexit_ret_subprogs2, int arg, struct file *ret)
    {
// (volatile int *)ret;
// (volatile int *)&ret->f_mode;
    bpf_for_each_map_elem(&test_array, test_cb, core::ptr::null_mut(), 0);
    triggered++;
    return 0;
    }
    SEC("fexit/bpf_testmod_return_ptr")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: handle_fexit_ret_subprogs3, arg: c_int, ret: *mut file) -> c_int {
    int BPF_PROG(handle_fexit_ret_subprogs3, int arg, struct file *ret)
    {
// (volatile int *)ret;
// (volatile int *)&ret->f_mode;
    bpf_for_each_map_elem(&test_array, test_cb, core::ptr::null_mut(), 0);
    triggered++;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
