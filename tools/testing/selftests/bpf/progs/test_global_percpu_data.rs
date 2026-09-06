//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_global_percpu_data.c
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

// Used for testing map name.
    int loong SEC(".percpu.looooooooong");
    int data3 SEC(".data.percpu");
    int data2 SEC(".percpu.data");
    int run;
// cpu_id as array to verify map value resizing.
    int cpu_id[1] SEC(".percpu");
    int data SEC(".percpu") = -1;
    int nums[7] SEC(".percpu");
    bool set SEC(".percpu") = false;
    struct {
    char set;
    int i;
    int nums[7];
    } struct_data SEC(".percpu") = {
    .set = 0,
    .i = -1,
    };
    SEC("raw_tp/task_rename")
    __auxiliary
#[no_mangle]
pub unsafe extern "C" fn update_percpu_data(ctx: *mut c_void) -> c_int {
    int update_percpu_data(void *ctx)
    {
    struct_data.nums[6] = 0xc0de;
    struct_data.set = 1;
    struct_data.i = 1;
    nums[6] = 0xc0de;
    data = 1;
    run++;
    set = true;
    cpu_id[0] = bpf_get_smp_processor_id();
    return 0;
    }
    static const char fmt[] SEC(".percpu.fmt") = "data %d\n";
    SEC("?kprobe")
#[no_mangle]
pub unsafe extern "C" fn __msg(string": "R{{[0-9]+}} points to percpu_array map which cannot be used as const) -> __failure {
    __failure __msg("R{{[0-9]+}} points to percpu_array map which cannot be used as const string")
#[no_mangle]
pub unsafe extern "C" fn verifier_strncmp(ctx: *mut c_void) -> c_int {
    int verifier_strncmp(void *ctx)
    {
    return bpf_strncmp("test", 5, fmt);
    }
    SEC("?kprobe")
#[no_mangle]
pub unsafe extern "C" fn __msg(string": "R{{[0-9]+}} points to percpu_array map which cannot be used as const) -> __failure {
    __failure __msg("R{{[0-9]+}} points to percpu_array map which cannot be used as const string")
#[no_mangle]
pub unsafe extern "C" fn verifier_snprintf(ctx: *mut c_void) -> c_int {
    int verifier_snprintf(void *ctx)
    {
    u64 args[] = { data };
    char buf[128];
    int len;
    len = bpf_snprintf(buf, sizeof(buf), fmt, args, sizeof(args));
    if (len > 0)
    bpf_printk("snprintf: %s\n", buf);
    return 0;
    }
    let mut num_cpus: volatile __u32 = 0;
    volatile const int num_off;
    volatile const int elem_sz;
    let mut sum: __u32 = 0;
    let mut run_iter: bool = false;
    SEC("iter/bpf_map_elem")
    __auxiliary
#[no_mangle]
pub unsafe extern "C" fn dump_percpu_data(ctx: *mut bpf_iter__bpf_map_elem) -> c_int {
    int dump_percpu_data(struct bpf_iter__bpf_map_elem *ctx)
    {
    void *pptr = ctx.value;
    int i;
    if (!pptr)
    return 0;
    run_iter = true;
    for (i = 0; i < num_cpus; i++) {
    sum += *(int *) (pptr + num_off);
    pptr += elem_sz;
    }
    return 0;
    }
    char _license[] SEC("license") = "GPL";
