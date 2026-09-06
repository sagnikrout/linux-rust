//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_xadd.c
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
// Converted from tools/testing/selftests/bpf/verifier/xadd.c

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1);
    __type(key, long long);
    __type(value, long long);
    } map_hash_8b SEC(".maps");
    SEC("tc")
    __description("xadd/w check unaligned stack")
#[no_mangle]
pub unsafe extern "C" fn __msg(off": "misaligned stack access) -> __failure {
    __failure __msg("misaligned stack access off")
#[no_mangle]
pub unsafe extern "C" fn xadd_w_check_unaligned_stack() -> __naked void {
    __naked void xadd_w_check_unaligned_stack(void)
    {
    asm volatile ("					\
    r0 = 1;						\
// (u64*)(r10 - 8) = r0;				\
    lock *(u32 *)(r10 - 7) += w0;			\
    r0 = *(u64*)(r10 - 8);				\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("tc")
    __description("xadd/w check unaligned map")
#[no_mangle]
pub unsafe extern "C" fn __msg(off": "misaligned value access) -> __failure {
    __failure __msg("misaligned value access off")
#[no_mangle]
pub unsafe extern "C" fn xadd_w_check_unaligned_map() -> __naked void {
    __naked void xadd_w_check_unaligned_map(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_8b] ll;				\
    call %[bpf_map_lookup_elem];			\
    if r0 != 0 goto l0_%=;				\
    exit;						\
    l0_%=:	r1 = 1;						\
    lock *(u32 *)(r0 + 3) += w1;			\
    r0 = *(u32*)(r0 + 3);				\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_8b)
    : __clobber_all);
    }
    SEC("xdp")
    __description("xadd/w check unaligned pkt")
#[no_mangle]
pub unsafe extern "C" fn __msg(allowed": "BPF_ATOMIC stores into R2 pkt is not) -> __failure {
    __failure __msg("BPF_ATOMIC stores into R2 pkt is not allowed")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn xadd_w_check_unaligned_pkt() -> __naked void {
    __naked void xadd_w_check_unaligned_pkt(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r1 < r3 goto l0_%=;				\
    r0 = 99;					\
    goto l1_%=;					\
    l0_%=:	r0 = 1;						\
    r1 = 0;						\
// (u32*)(r2 + 0) = r1;				\
    r1 = 0;						\
// (u32*)(r2 + 3) = r1;				\
    lock *(u32 *)(r2 + 1) += w0;			\
    lock *(u32 *)(r2 + 2) += w0;			\
    r0 = *(u32*)(r2 + 1);				\
    l1_%=:	exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("tc")
    __description("xadd/w check whether src/dst got mangled, 1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 3) -> __success {
    __success __retval(3)
#[no_mangle]
pub unsafe extern "C" fn src_dst_got_mangled_1() -> __naked void {
    __naked void src_dst_got_mangled_1(void)
    {
    asm volatile ("					\
    r0 = 1;						\
    r6 = r0;					\
    r7 = r10;					\
// (u64*)(r10 - 8) = r0;				\
    lock *(u64 *)(r10 - 8) += r0;			\
    lock *(u64 *)(r10 - 8) += r0;			\
    if r6 != r0 goto l0_%=;				\
    if r7 != r10 goto l0_%=;			\
    r0 = *(u64*)(r10 - 8);				\
    exit;						\
    l0_%=:	r0 = 42;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("tc")
    __description("xadd/w check whether src/dst got mangled, 2")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 3) -> __success {
    __success __retval(3)
#[no_mangle]
pub unsafe extern "C" fn src_dst_got_mangled_2() -> __naked void {
    __naked void src_dst_got_mangled_2(void)
    {
    asm volatile ("					\
    r0 = 1;						\
    r6 = r0;					\
    r7 = r10;					\
// (u32*)(r10 - 8) = r0;				\
    lock *(u32 *)(r10 - 8) += w0;			\
    lock *(u32 *)(r10 - 8) += w0;			\
    if r6 != r0 goto l0_%=;				\
    if r7 != r10 goto l0_%=;			\
    r0 = *(u32*)(r10 - 8);				\
    exit;						\
    l0_%=:	r0 = 42;					\
    exit;						\
    "	::: __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
