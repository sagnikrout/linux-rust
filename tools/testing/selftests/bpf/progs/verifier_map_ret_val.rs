//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_map_ret_val.c
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
// Converted from tools/testing/selftests/bpf/verifier/map_ret_val.c

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1);
    __type(key, long long);
    __type(value, long long);
    } map_hash_8b SEC(".maps");
    SEC("socket")
    __description("invalid map_fd for function call")
#[no_mangle]
pub unsafe extern "C" fn __msg(bpf_map": "fd 0 is not pointing to valid) -> __failure {
    __failure __msg("fd 0 is not pointing to valid bpf_map")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn map_fd_for_function_call() -> __naked void {
    __naked void map_fd_for_function_call(void)
    {
    asm volatile ("					\
    r2 = 0;						\
// (u64*)(r10 - 8) = r2;				\
    r2 = r10;					\
    r2 += -8;					\
    .8byte %[ld_map_fd];				\
    .8byte 0;					\
    call %[bpf_map_delete_elem];			\
    exit;						\
    "	:
    : __imm(bpf_map_delete_elem),
    __imm_insn(ld_map_fd, BPF_RAW_INSN(BPF_LD | BPF_DW | BPF_IMM, BPF_REG_1, BPF_PSEUDO_MAP_FD, 0, 0))
    : __clobber_all);
    }
    SEC("socket")
    __description("don't check return value before access")
#[no_mangle]
pub unsafe extern "C" fn __msg('map_value_or_null'": "R0 invalid mem access) -> __failure {
    __failure __msg("R0 invalid mem access 'map_value_or_null'")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn check_return_value_before_access() -> __naked void {
    __naked void check_return_value_before_access(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_8b] ll;				\
    call %[bpf_map_lookup_elem];			\
    r1 = 0;						\
// (u64*)(r0 + 0) = r1;				\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_8b)
    : __clobber_all);
    }
    SEC("socket")
    __description("access memory with incorrect alignment")
#[no_mangle]
pub unsafe extern "C" fn __msg(access": "misaligned value) -> __failure {
    __failure __msg("misaligned value access")
    __failure_unpriv
    __flag(BPF_F_STRICT_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn access_memory_with_incorrect_alignment_1() -> __naked void {
    __naked void access_memory_with_incorrect_alignment_1(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_8b] ll;				\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = 0;						\
// (u64*)(r0 + 4) = r1;				\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_8b)
    : __clobber_all);
    }
    SEC("socket")
    __description("sometimes access memory with incorrect alignment")
#[no_mangle]
pub unsafe extern "C" fn __msg(access": "R0 invalid mem) -> __failure {
    __failure __msg("R0 invalid mem access")
    __msg_unpriv("R0 leaks addr")
    __flag(BPF_F_STRICT_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn access_memory_with_incorrect_alignment_2() -> __naked void {
    __naked void access_memory_with_incorrect_alignment_2(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_8b] ll;				\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = 0;						\
// (u64*)(r0 + 0) = r1;				\
    exit;						\
    l0_%=:	r1 = 1;						\
// (u64*)(r0 + 0) = r1;				\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_8b)
    : __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
