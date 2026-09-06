//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_and.c
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
// Converted from tools/testing/selftests/bpf/verifier/and.c

pub const MAX_ENTRIES: c_int = 11;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_val {
    pub index: c_uint,
    pub foo: [c_int; MAX_ENTRIES],
}

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1);
    __type(key, long long);
    __type(value, struct test_val);
    } map_hash_48b SEC(".maps");
    SEC("socket")
    __description("invalid and of negative number")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "R0 max value is outside of the allowed memory) -> __failure {
    __failure __msg("R0 max value is outside of the allowed memory range")
    __failure_unpriv
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn invalid_and_of_negative_number() -> __naked void {
    __naked void invalid_and_of_negative_number(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = *(u8*)(r0 + 0);				\
    r1 &= -4;					\
    r1 <<= 2;					\
    r0 += r1;					\
    l0_%=:	r1 = %[test_val_foo];				\
// (u64*)(r0 + 0) = r1;				\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_48b),
    __imm_const(test_val_foo, offsetof(struct test_val, foo))
    : __clobber_all);
    }
    SEC("socket")
    __description("invalid range check")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "R0 max value is outside of the allowed memory) -> __failure {
    __failure __msg("R0 max value is outside of the allowed memory range")
    __failure_unpriv
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn invalid_range_check() -> __naked void {
    __naked void invalid_range_check(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = *(u32*)(r0 + 0);				\
    r9 = 1;						\
    w1 %%= 2;					\
    w1 += 1;					\
    w9 &= w1;					\
    w9 += 1;					\
    w9 >>= 1;					\
    w3 = 1;						\
    w3 -= w9;					\
    w3 *= 0x10000000;				\
    r0 += r3;					\
// (u32*)(r0 + 0) = r3;				\
    l0_%=:	r0 = r0;					\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_48b)
    : __clobber_all);
    }
    SEC("socket")
    __description("check known subreg with unknown reg")
    __success __success_unpriv
    __retval(0)

    __xlated_unpriv("if w0 < 0x1 goto pc+2")
    __xlated_unpriv("nospec") /* inserted to prevent `R1 !read_ok'` */
    __xlated_unpriv("goto pc-1") /* `r1 = *(u32*)(r1 + 512)`, sanitized dead code */
    __xlated_unpriv("r0 = 0")

#[no_mangle]
pub unsafe extern "C" fn known_subreg_with_unknown_reg() -> __naked void {
    __naked void known_subreg_with_unknown_reg(void)
    {
    asm volatile ("					\
    call %[bpf_get_prandom_u32];			\
    r0 <<= 32;					\
    r0 += 1;					\
    r0 &= 0xFFFF1234;				\
// Upper bits are unknown but AND above masks out 1 zero'ing lower bits */\
    if w0 < 1 goto l0_%=;				\
    r1 = *(u32*)(r1 + 512);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_get_prandom_u32)
    : __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
