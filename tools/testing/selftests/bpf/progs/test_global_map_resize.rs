//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_global_map_resize.c
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
// Copyright (c) 2023 Meta Platforms, Inc. and affiliates.

    char _license[] SEC("license") = "GPL";
// rodata section
    const volatile pid_t pid;
    const volatile size_t bss_array_len;
    const volatile size_t data_array_len;
// bss section
    let mut sum: c_int = 0;
    int array[1];
// custom data section
    int my_array[1] SEC(".data.custom");
// custom data section which should NOT be resizable,
// since it contains a single var which is not an array
//
    int my_int SEC(".data.non_array");
// custom data section which should NOT be resizable,
// since its last var is not an array
//
    int my_array_first[1] SEC(".data.array_not_last");
    int my_int_last SEC(".data.array_not_last");
    int percpu_arr[1] SEC(".data.percpu_arr");
// at least one extern is included, to ensure that a specific
// regression is tested whereby resizing resulted in a free-after-use
// bug after type information is invalidated by the resize operation.
//
// There isn't a particularly good API to test for this specific condition,
// but by having externs for the resizing tests it will cover this path.
//
    extern int LINUX_KERNEL_VERSION __kconfig;
    long version_sink;
    SEC("tp/syscalls/sys_enter_getpid")
#[no_mangle]
pub unsafe extern "C" fn bss_array_sum(ctx: *mut c_void) -> c_int {
    int bss_array_sum(void *ctx)
    {
    if (pid != (bpf_get_current_pid_tgid() >> 32))
    return 0;
// this will be zero, we just rely on verifier not rejecting this
    sum = percpu_arr[bpf_get_smp_processor_id()];
    for (size_t i = 0; i < bss_array_len; ++i)
    sum += array[i];
// see above; ensure this is not optimized out
    version_sink = LINUX_KERNEL_VERSION;
    return 0;
    }
    SEC("tp/syscalls/sys_enter_getuid")
#[no_mangle]
pub unsafe extern "C" fn data_array_sum(ctx: *mut c_void) -> c_int {
    int data_array_sum(void *ctx)
    {
    if (pid != (bpf_get_current_pid_tgid() >> 32))
    return 0;
// this will be zero, we just rely on verifier not rejecting this
    sum = percpu_arr[bpf_get_smp_processor_id()];
    for (size_t i = 0; i < data_array_len; ++i)
    sum += my_array[i];
// see above; ensure this is not optimized out
    version_sink = LINUX_KERNEL_VERSION;
    return 0;
    }
    SEC("struct_ops/test_1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_1) -> c_int {
    int BPF_PROG(test_1)
    {
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_testmod_ops {
    pub (*test_1)(void): *mut c_int,
}

    SEC(".struct_ops.link")
    struct bpf_testmod_ops st_ops_resize = {
    .test_1 = (void *)test_1
    };
