//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_raw_tp_writable.c
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
// Converted from tools/testing/selftests/bpf/verifier/raw_tp_writable.c

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1);
    __type(key, long long);
    __type(value, long long);
    } map_hash_8b SEC(".maps");
    SEC("raw_tracepoint.w")
    __description("raw_tracepoint_writable: reject variable offset")
    __failure
    __msg("R6 invalid variable buffer offset: off=0, var_off=(0x0; 0xffffffff)")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn tracepoint_writable_reject_variable_offset() -> __naked void {
    __naked void tracepoint_writable_reject_variable_offset(void)
    {
    asm volatile ("					\
// r6 is our tp buffer */			\
    r6 = *(u64*)(r1 + 0);				\
    r1 = %[map_hash_8b] ll;				\
// move the key (== 0) to r10-8 */		\
    w0 = 0;						\
    r2 = r10;					\
    r2 += -8;					\
// (u64*)(r2 + 0) = r0;				\
// lookup in the map */				\
    call %[bpf_map_lookup_elem];			\
// exit clean if null */			\
    if r0 != 0 goto l0_%=;				\
    exit;						\
    l0_%=:	/* shift the buffer pointer to a variable location */\
    r0 = *(u32*)(r0 + 0);				\
    r6 += r0;					\
// clobber whatever's there */			\
    r7 = 4242;					\
// (u64*)(r6 + 0) = r7;				\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_8b)
    : __clobber_all);
    }
    SEC("raw_tracepoint.w")
    __description("raw_tracepoint_writable: reject negative const offset")
    __failure
    __msg("invalid negative tracepoint buffer offset")
#[no_mangle]
pub unsafe extern "C" fn tracepoint_writable_reject_negative_const_offset() -> __naked void {
    __naked void tracepoint_writable_reject_negative_const_offset(void)
    {
    asm volatile ("					\
    r6 = *(u64 *)(r1 + 0);				\
    r6 += -8;					\
    r0 = *(u64 *)(r6 + 0);				\
    exit;						\
    "	:
    :
    : __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
