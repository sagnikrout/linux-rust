//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_prevent_map_lookup.c
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
// Converted from tools/testing/selftests/bpf/verifier/prevent_map_lookup.c

    struct {
    __uint(type, BPF_MAP_TYPE_STACK_TRACE);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u64);
    } map_stacktrace SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_PROG_ARRAY);
    __uint(max_entries, 8);
    __uint(key_size, sizeof(int));
    __array(values, void (void));
    } map_prog2_socket SEC(".maps");
    SEC("perf_event")
    __description("prevent map lookup in stack trace")
#[no_mangle]
pub unsafe extern "C" fn __msg(bpf_map_lookup_elem": "cannot pass map_type 7 into func) -> __failure {
    __failure __msg("cannot pass map_type 7 into func bpf_map_lookup_elem")
#[no_mangle]
pub unsafe extern "C" fn map_lookup_in_stack_trace() -> __naked void {
    __naked void map_lookup_in_stack_trace(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_stacktrace] ll;			\
    call %[bpf_map_lookup_elem];			\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_stacktrace)
    : __clobber_all);
    }
    SEC("socket")
    __description("prevent map lookup in prog array")
#[no_mangle]
pub unsafe extern "C" fn __msg(bpf_map_lookup_elem": "cannot pass map_type 3 into func) -> __failure {
    __failure __msg("cannot pass map_type 3 into func bpf_map_lookup_elem")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn map_lookup_in_prog_array() -> __naked void {
    __naked void map_lookup_in_prog_array(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_prog2_socket] ll;			\
    call %[bpf_map_lookup_elem];			\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_prog2_socket)
    : __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
