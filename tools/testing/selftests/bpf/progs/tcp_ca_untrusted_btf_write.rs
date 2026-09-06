//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/tcp_ca_untrusted_btf_write.c
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
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: untrusted_btf_write_init, sk: *mut sock) {
    void BPF_PROG(untrusted_btf_write_init, struct sock *sk)
    {
    struct tcp_sock *tp;
    let mut v: c_int = 1;
    void *p;
    p = bpf_rdonly_cast(&v, 0);
    tp = bpf_rdonly_cast(p, bpf_core_type_id_kernel(struct tcp_sock));
    tp.snd_cwnd = 1;
    }
    SEC(".struct_ops")
    struct tcp_congestion_ops untrusted_btf_write = {
    .init = (void *)untrusted_btf_write_init,
    .name = "bpf_ro_btf",
    };
