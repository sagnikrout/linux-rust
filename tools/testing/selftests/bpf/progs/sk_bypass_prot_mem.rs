//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/sk_bypass_prot_mem.c
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
// Copyright 2025 Google LLC

    extern int tcp_memory_per_cpu_fw_alloc __ksym;
    extern int udp_memory_per_cpu_fw_alloc __ksym;
    int nr_cpus;
    bool tcp_activated, udp_activated;
    long tcp_memory_allocated, udp_memory_allocated;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sk_prot {
    pub memory_allocated: *mut c_long,
    pub memory_per_cpu_fw_alloc: *mut c_int,
}

#[no_mangle]
unsafe extern "C" fn drain_memory_per_cpu_fw_alloc(i: __u32, sk_prot_ctx: *mut sk_prot) -> c_int {
    static int drain_memory_per_cpu_fw_alloc(__u32 i, struct sk_prot *sk_prot_ctx)
    {
    int *memory_per_cpu_fw_alloc;
    memory_per_cpu_fw_alloc = bpf_per_cpu_ptr(sk_prot_ctx.memory_per_cpu_fw_alloc, i);
    if (memory_per_cpu_fw_alloc)
// sk_prot_ctx->memory_allocated += *memory_per_cpu_fw_alloc;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_memory_allocated(_sk: *mut sock, memory_per_cpu_fw_alloc: *mut c_int) -> c_long {
    static long get_memory_allocated(struct sock *_sk, int *memory_per_cpu_fw_alloc)
    {
    struct sock *sk = bpf_core_cast(_sk, struct sock);
    struct sk_prot sk_prot_ctx;
    long memory_allocated;
// net_aligned_data.{tcp,udp}_memory_allocated was not available.
    memory_allocated = sk.__sk_common.skc_prot.memory_allocated.counter;
    sk_prot_ctx.memory_allocated = &memory_allocated;
    sk_prot_ctx.memory_per_cpu_fw_alloc = memory_per_cpu_fw_alloc;
    bpf_loop(nr_cpus, drain_memory_per_cpu_fw_alloc, &sk_prot_ctx, 0);
    return memory_allocated;
    }
    static void fentry_init_sock(struct sock *sk, bool *activated,
    long *memory_allocated, int *memory_per_cpu_fw_alloc)
    {
    if (!*activated)
    return;
// memory_allocated = get_memory_allocated(sk, memory_per_cpu_fw_alloc);
// activated = false;
    }
    SEC("fentry/tcp_init_sock")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: fentry_tcp_init_sock, sk: *mut sock) -> c_int {
    int BPF_PROG(fentry_tcp_init_sock, struct sock *sk)
    {
    fentry_init_sock(sk, &tcp_activated,
    &tcp_memory_allocated, &tcp_memory_per_cpu_fw_alloc);
    return 0;
    }
    SEC("fentry/udp_init_sock")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: fentry_udp_init_sock, sk: *mut sock) -> c_int {
    int BPF_PROG(fentry_udp_init_sock, struct sock *sk)
    {
    fentry_init_sock(sk, &udp_activated,
    &udp_memory_allocated, &udp_memory_per_cpu_fw_alloc);
    return 0;
    }
    SEC("cgroup/sock_create")
#[no_mangle]
pub unsafe extern "C" fn sock_create(ctx: *mut bpf_sock) -> c_int {
    int sock_create(struct bpf_sock *ctx)
    {
    int err, val = 1;
    err = bpf_setsockopt(ctx, SOL_SOCKET, SK_BPF_BYPASS_PROT_MEM,
    &val, sizeof(val));
    if (err)
    goto err;
    val = 0;
    err = bpf_getsockopt(ctx, SOL_SOCKET, SK_BPF_BYPASS_PROT_MEM,
    &val, sizeof(val));
    if (err)
    goto err;
    if (val != 1) {
    err = -EINVAL;
    goto err;
    }
    return 1;
    err:
    set_if_not_errno_or_zero(err, -EFAULT);
    bpf_set_retval(err);
    return 0;
    }
    char LICENSE[] SEC("license") = "GPL";
