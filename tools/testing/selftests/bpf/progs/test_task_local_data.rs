//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_task_local_data.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tld_keys {
    pub value0: tld_key_t,
    pub value1: tld_key_t,
    pub value2: tld_key_t,
    pub value_not_exist: tld_key_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_tld_struct {
    pub a: __u64,
    pub b: __u64,
    pub c: __u64,
    pub d: __u64,
}

    int test_value0;
    int test_value1;
    struct test_tld_struct test_value2;
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn task_main(ctx: *mut c_void) -> c_int {
    int task_main(void *ctx)
    {
    struct tld_object tld_obj;
    struct test_tld_struct *struct_p;
    struct task_struct *task;
    int err, *int_p;
    task = bpf_get_current_task_btf();
    err = tld_object_init(task, &tld_obj);
    if (err)
    return 1;
    int_p = tld_get_data(&tld_obj, value0, "value0", sizeof(int));
    if (int_p)
    test_value0 = *int_p;
    else
    return 2;
    int_p = tld_get_data(&tld_obj, value1, "value1", sizeof(int));
    if (int_p)
    test_value1 = *int_p;
    else
    return 3;
    struct_p = tld_get_data(&tld_obj, value2, "value2", sizeof(struct test_tld_struct));
    if (struct_p)
    test_value2 = *struct_p;
    else
    return 4;
    int_p = tld_get_data(&tld_obj, value_not_exist, "value_not_exist", sizeof(int));
    if (int_p)
    return 5;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
