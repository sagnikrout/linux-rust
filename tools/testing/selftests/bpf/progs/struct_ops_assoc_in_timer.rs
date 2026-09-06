//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/struct_ops_assoc_in_timer.c
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
pub struct elem {
    pub timer: bpf_timer,
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, struct elem);
    } array_map SEC(".maps");
pub const MAP_MAGIC: c_int = 1234;
    int recur;
    int test_err;
    int timer_ns;
    int timer_test_1_ret;
    int timer_cb_run;
#[no_mangle]
pub unsafe extern "C" fn timer_cb(map: *mut c_void, key: *mut c_int, timer: *mut bpf_timer) -> __noinline static int {
    __noinline static int timer_cb(void *map, int *key, struct bpf_timer *timer)
    {
    let mut args: st_ops_args = {};
    recur++;
    timer_test_1_ret = bpf_kfunc_multi_st_ops_test_1_assoc(&args);
    recur--;
    timer_cb_run++;
    return 0;
    }
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_1, args: *mut st_ops_args) -> c_int {
    int BPF_PROG(test_1, struct st_ops_args *args)
    {
    struct bpf_timer *timer;
    let mut key: c_int = 0;
    if (!recur) {
    timer = bpf_map_lookup_elem(&array_map, &key);
    if (!timer)
    return 0;
    bpf_timer_init(timer, &array_map, 1);
    bpf_timer_set_callback(timer, timer_cb);
    bpf_timer_start(timer, timer_ns, 0);
    }
    return MAP_MAGIC;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn syscall_prog(ctx: *mut c_void) -> c_int {
    int syscall_prog(void *ctx)
    {
    let mut args: st_ops_args = {};
    int ret;
    ret = bpf_kfunc_multi_st_ops_test_1_assoc(&args);
    if (ret != MAP_MAGIC)
    test_err++;
    return 0;
    }
    SEC(".struct_ops.link")
    struct bpf_testmod_multi_st_ops st_ops_map = {
    .test_1 = (void *)test_1,
    };
