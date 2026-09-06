//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_helper_restricted.c
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
// Converted from tools/testing/selftests/bpf/verifier/helper_restricted.c

#[repr(C)]
#[derive(Copy, Clone)]
pub struct val {
    pub cnt: c_int,
    pub l: bpf_spin_lock,
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, struct val);
    } map_spin_lock SEC(".maps");
    SEC("kprobe")
    __description("bpf_ktime_get_coarse_ns is forbidden in BPF_PROG_TYPE_KPROBE")
#[no_mangle]
pub unsafe extern "C" fn __msg(bpf_ktime_get_coarse_ns": "program of this type cannot use helper) -> __failure {
    __failure __msg("program of this type cannot use helper bpf_ktime_get_coarse_ns")
#[no_mangle]
pub unsafe extern "C" fn in_bpf_prog_type_kprobe_1() -> __naked void {
    __naked void in_bpf_prog_type_kprobe_1(void)
    {
    asm volatile ("					\
    call %[bpf_ktime_get_coarse_ns];		\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_ktime_get_coarse_ns)
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("bpf_ktime_get_coarse_ns is forbidden in BPF_PROG_TYPE_TRACEPOINT")
#[no_mangle]
pub unsafe extern "C" fn __msg(bpf_ktime_get_coarse_ns": "program of this type cannot use helper) -> __failure {
    __failure __msg("program of this type cannot use helper bpf_ktime_get_coarse_ns")
#[no_mangle]
pub unsafe extern "C" fn in_bpf_prog_type_tracepoint_1() -> __naked void {
    __naked void in_bpf_prog_type_tracepoint_1(void)
    {
    asm volatile ("					\
    call %[bpf_ktime_get_coarse_ns];		\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_ktime_get_coarse_ns)
    : __clobber_all);
    }
    SEC("perf_event")
    __description("bpf_ktime_get_coarse_ns is forbidden in BPF_PROG_TYPE_PERF_EVENT")
#[no_mangle]
pub unsafe extern "C" fn __msg(bpf_ktime_get_coarse_ns": "program of this type cannot use helper) -> __failure {
    __failure __msg("program of this type cannot use helper bpf_ktime_get_coarse_ns")
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_type_perf_event_1() -> __naked void {
    __naked void bpf_prog_type_perf_event_1(void)
    {
    asm volatile ("					\
    call %[bpf_ktime_get_coarse_ns];		\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_ktime_get_coarse_ns)
    : __clobber_all);
    }
    SEC("raw_tracepoint")
    __description("bpf_ktime_get_coarse_ns is forbidden in BPF_PROG_TYPE_RAW_TRACEPOINT")
#[no_mangle]
pub unsafe extern "C" fn __msg(bpf_ktime_get_coarse_ns": "program of this type cannot use helper) -> __failure {
    __failure __msg("program of this type cannot use helper bpf_ktime_get_coarse_ns")
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_type_raw_tracepoint_1() -> __naked void {
    __naked void bpf_prog_type_raw_tracepoint_1(void)
    {
    asm volatile ("					\
    call %[bpf_ktime_get_coarse_ns];		\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_ktime_get_coarse_ns)
    : __clobber_all);
    }
    SEC("kprobe")
    __description("bpf_spin_lock is forbidden in BPF_PROG_TYPE_KPROBE")
#[no_mangle]
pub unsafe extern "C" fn __msg(yet": "tracing progs cannot use bpf_spin_lock) -> __failure {
    __failure __msg("tracing progs cannot use bpf_spin_lock yet")
#[no_mangle]
pub unsafe extern "C" fn in_bpf_prog_type_kprobe_3() -> __naked void {
    __naked void in_bpf_prog_type_kprobe_3(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_spin_lock] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r0;					\
    call %[bpf_spin_lock];				\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_spin_lock),
    __imm_addr(map_spin_lock)
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("bpf_spin_lock is forbidden in BPF_PROG_TYPE_TRACEPOINT")
#[no_mangle]
pub unsafe extern "C" fn __msg(yet": "tracing progs cannot use bpf_spin_lock) -> __failure {
    __failure __msg("tracing progs cannot use bpf_spin_lock yet")
#[no_mangle]
pub unsafe extern "C" fn in_bpf_prog_type_tracepoint_3() -> __naked void {
    __naked void in_bpf_prog_type_tracepoint_3(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_spin_lock] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r0;					\
    call %[bpf_spin_lock];				\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_spin_lock),
    __imm_addr(map_spin_lock)
    : __clobber_all);
    }
    SEC("perf_event")
    __description("bpf_spin_lock is forbidden in BPF_PROG_TYPE_PERF_EVENT")
#[no_mangle]
pub unsafe extern "C" fn __msg(yet": "tracing progs cannot use bpf_spin_lock) -> __failure {
    __failure __msg("tracing progs cannot use bpf_spin_lock yet")
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_type_perf_event_3() -> __naked void {
    __naked void bpf_prog_type_perf_event_3(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_spin_lock] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r0;					\
    call %[bpf_spin_lock];				\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_spin_lock),
    __imm_addr(map_spin_lock)
    : __clobber_all);
    }
    SEC("raw_tracepoint")
    __description("bpf_spin_lock is forbidden in BPF_PROG_TYPE_RAW_TRACEPOINT")
#[no_mangle]
pub unsafe extern "C" fn __msg(yet": "tracing progs cannot use bpf_spin_lock) -> __failure {
    __failure __msg("tracing progs cannot use bpf_spin_lock yet")
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_type_raw_tracepoint_3() -> __naked void {
    __naked void bpf_prog_type_raw_tracepoint_3(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_spin_lock] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r0;					\
    call %[bpf_spin_lock];				\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_spin_lock),
    __imm_addr(map_spin_lock)
    : __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
