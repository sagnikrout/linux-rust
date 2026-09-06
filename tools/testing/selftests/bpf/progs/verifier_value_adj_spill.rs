//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_value_adj_spill.c
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
// Converted from tools/testing/selftests/bpf/verifier/value_adj_spill.c

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
    __description("map element value is preserved across register spilling")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(addr": "R0 leaks) -> __success __failure_unpriv {
    __success __failure_unpriv __msg_unpriv("R0 leaks addr")
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn is_preserved_across_register_spilling() -> __naked void {
    __naked void is_preserved_across_register_spilling(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = 42;					\
// (u64*)(r0 + 0) = r1;				\
    r1 = r10;					\
    r1 += -184;					\
// (u64*)(r1 + 0) = r0;				\
    r3 = *(u64*)(r1 + 0);				\
    r1 = 42;					\
// (u64*)(r3 + 0) = r1;				\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_48b)
    : __clobber_all);
    }
    SEC("socket")
    __description("map element value or null is marked on register spilling")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(addr": "R0 leaks) -> __success __failure_unpriv {
    __success __failure_unpriv __msg_unpriv("R0 leaks addr")
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn is_marked_on_register_spilling() -> __naked void {
    __naked void is_marked_on_register_spilling(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    r1 = r10;					\
    r1 += -152;					\
// (u64*)(r1 + 0) = r0;				\
    if r0 == 0 goto l0_%=;				\
    r3 = *(u64*)(r1 + 0);				\
    r1 = 42;					\
// (u64*)(r3 + 0) = r1;				\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_48b)
    : __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
