//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_map_ptr.c
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
// Converted from tools/testing/selftests/bpf/verifier/map_ptr.c

pub const MAX_ENTRIES: c_int = 11;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_val {
    pub index: c_uint,
    pub foo: [c_int; MAX_ENTRIES],
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, struct test_val);
    } map_array_48b SEC(".maps");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct other_val {
    pub foo: c_longlong,
    pub bar: c_longlong,
}

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1);
    __type(key, long long);
    __type(value, struct other_val);
    } map_hash_16b SEC(".maps");
    SEC("socket")
    __description("bpf_map_ptr: read with negative offset rejected")
#[no_mangle]
pub unsafe extern "C" fn __msg(off=-8": "R1 is bpf_array invalid negative access:) -> __failure {
    __failure __msg("R1 is bpf_array invalid negative access: off=-8")
    __failure_unpriv
    __msg_unpriv("access is allowed only to CAP_PERFMON and CAP_SYS_ADMIN")
#[no_mangle]
pub unsafe extern "C" fn read_with_negative_offset_rejected() -> __naked void {
    __naked void read_with_negative_offset_rejected(void)
    {
    asm volatile ("					\
    r1 = r10;					\
    r1 = %[map_array_48b] ll;			\
    r6 = *(u64*)(r1 - 8);				\
    r0 = 1;						\
    exit;						\
    "	:
    : __imm_addr(map_array_48b)
    : __clobber_all);
    }
    SEC("socket")
    __description("bpf_map_ptr: write rejected")
#[no_mangle]
pub unsafe extern "C" fn __msg(supported": "only read from bpf_array is) -> __failure {
    __failure __msg("only read from bpf_array is supported")
    __failure_unpriv
    __msg_unpriv("access is allowed only to CAP_PERFMON and CAP_SYS_ADMIN")
#[no_mangle]
pub unsafe extern "C" fn bpf_map_ptr_write_rejected() -> __naked void {
    __naked void bpf_map_ptr_write_rejected(void)
    {
    asm volatile ("					\
    r0 = 0;						\
// (u64*)(r10 - 8) = r0;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_array_48b] ll;			\
// (u64*)(r1 + 0) = r2;				\
    r0 = 1;						\
    exit;						\
    "	:
    : __imm_addr(map_array_48b)
    : __clobber_all);
    }
//
// struct bpf_map starts with the SHA256 hash sha[32] at offset 0 (a readable
// byte array), followed by the ops pointer at offset 32 and the inner_map_meta
// pointer at offset 40. Reading a u32 at offset 41 reaches into the middle of
// the inner_map_meta pointer, i.e. a partial pointer access, which is
// rejected.
//
    SEC("socket")
    __description("bpf_map_ptr: read non-existent field rejected")
    __failure
    __msg("cannot access ptr member inner_map_meta with moff 40 in struct bpf_map with off 41 size 4")
    __failure_unpriv
    __msg_unpriv("access is allowed only to CAP_PERFMON and CAP_SYS_ADMIN")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn read_non_existent_field_rejected() -> __naked void {
    __naked void read_non_existent_field_rejected(void)
    {
    asm volatile ("					\
    r6 = 0;						\
    r1 = %[map_array_48b] ll;			\
    r6 = *(u32*)(r1 + 41);				\
    r0 = 1;						\
    exit;						\
    "	:
    : __imm_addr(map_array_48b)
    : __clobber_all);
    }
//
// The sha byte array spans offsets 0..31 (mend 32). Reading a u32 at offset
// 30 starts inside sha but extends past its end, which the verifier rejects
// as an out-of-bounds scalar access.
//
    SEC("socket")
    __description("bpf_map_ptr: read beyond sha field rejected")
    __failure
    __msg("access beyond the end of member sha (mend:32) in struct bpf_map with off 30 size 4")
    __failure_unpriv
    __msg_unpriv("access is allowed only to CAP_PERFMON and CAP_SYS_ADMIN")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn read_beyond_sha_field_rejected() -> __naked void {
    __naked void read_beyond_sha_field_rejected(void)
    {
    asm volatile ("					\
    r6 = 0;						\
    r1 = %[map_array_48b] ll;			\
    r6 = *(u32*)(r1 + 30);				\
    r0 = 1;						\
    exit;						\
    "	:
    : __imm_addr(map_array_48b)
    : __clobber_all);
    }
    SEC("socket")
    __description("bpf_map_ptr: read ops field accepted")
    __success __failure_unpriv
    __msg_unpriv("access is allowed only to CAP_PERFMON and CAP_SYS_ADMIN")
    __retval(1)
#[no_mangle]
pub unsafe extern "C" fn ptr_read_ops_field_accepted() -> __naked void {
    __naked void ptr_read_ops_field_accepted(void)
    {
    asm volatile ("					\
    r6 = 0;						\
    r1 = %[map_array_48b] ll;			\
    r6 = *(u64*)(r1 + 32);				\
    r0 = 1;						\
    exit;						\
    "	:
    : __imm_addr(map_array_48b)
    : __clobber_all);
    }
    SEC("socket")
    __description("bpf_map_ptr: r = 0, map_ptr = map_ptr + r")
    __success __failure_unpriv
    __msg_unpriv("R1 has pointer with unsupported alu operation")
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn map_ptr_map_ptr_r() -> __naked void {
    __naked void map_ptr_map_ptr_r(void)
    {
    asm volatile ("					\
    r0 = 0;						\
// (u64*)(r10 - 8) = r0;				\
    r2 = r10;					\
    r2 += -8;					\
    r0 = 0;						\
    r1 = %[map_hash_16b] ll;			\
    r1 += r0;					\
    call %[bpf_map_lookup_elem];			\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_16b)
    : __clobber_all);
    }
    SEC("socket")
    __description("bpf_map_ptr: r = 0, r = r + map_ptr")
    __success __failure_unpriv
    __msg_unpriv("R0 has pointer with unsupported alu operation")
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn _0_r_r_map_ptr() -> __naked void {
    __naked void _0_r_r_map_ptr(void)
    {
    asm volatile ("					\
    r0 = 0;						\
// (u64*)(r10 - 8) = r0;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
    r0 = %[map_hash_16b] ll;			\
    r1 += r0;					\
    call %[bpf_map_lookup_elem];			\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_16b)
    : __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
