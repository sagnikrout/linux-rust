//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_bounds_mix_sign_unsign.c
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
// Converted from tools/testing/selftests/bpf/verifier/bounds_mix_sign_unsign.c

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1);
    __type(key, long long);
    __type(value, long long);
    } map_hash_8b SEC(".maps");
    SEC("socket")
    __description("bounds checks mixing signed and unsigned, positive bounds")
#[no_mangle]
pub unsafe extern "C" fn __msg(value": "unbounded min) -> __failure {
    __failure __msg("unbounded min value")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn signed_and_unsigned_positive_bounds() -> __naked void {
    __naked void signed_and_unsigned_positive_bounds(void)
    {
    asm volatile ("					\
    call %[bpf_ktime_get_ns];			\
// (u64*)(r10 - 16) = r0;				\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_8b] ll;				\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = *(u64*)(r10 - 16);				\
    r2 = 2;						\
    if r2 >= r1 goto l0_%=;				\
    if r1 s> 4 goto l0_%=;				\
    r0 += r1;					\
    r1 = 0;						\
// (u8*)(r0 + 0) = r1;				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_ktime_get_ns),
    __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_8b)
    : __clobber_all);
    }
    SEC("socket")
    __description("bounds checks mixing signed and unsigned")
#[no_mangle]
pub unsafe extern "C" fn __msg(value": "unbounded min) -> __failure {
    __failure __msg("unbounded min value")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn checks_mixing_signed_and_unsigned() -> __naked void {
    __naked void checks_mixing_signed_and_unsigned(void)
    {
    asm volatile ("					\
    call %[bpf_ktime_get_ns];			\
// (u64*)(r10 - 16) = r0;				\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_8b] ll;				\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = *(u64*)(r10 - 16);				\
    r2 = -1;					\
    if r1 > r2 goto l0_%=;				\
    if r1 s> 1 goto l0_%=;				\
    r0 += r1;					\
    r1 = 0;						\
// (u8*)(r0 + 0) = r1;				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_ktime_get_ns),
    __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_8b)
    : __clobber_all);
    }
    SEC("socket")
    __description("bounds checks mixing signed and unsigned, variant 2")
#[no_mangle]
pub unsafe extern "C" fn __msg(value": "unbounded min) -> __failure {
    __failure __msg("unbounded min value")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn signed_and_unsigned_variant_2() -> __naked void {
    __naked void signed_and_unsigned_variant_2(void)
    {
    asm volatile ("					\
    call %[bpf_ktime_get_ns];			\
// (u64*)(r10 - 16) = r0;				\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_8b] ll;				\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = *(u64*)(r10 - 16);				\
    r2 = -1;					\
    if r1 > r2 goto l0_%=;				\
    r8 = 0;						\
    r8 += r1;					\
    if r8 s> 1 goto l0_%=;				\
    r0 += r8;					\
    r0 = 0;						\
// (u8*)(r8 + 0) = r0;				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_ktime_get_ns),
    __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_8b)
    : __clobber_all);
    }
    SEC("socket")
    __description("bounds checks mixing signed and unsigned, variant 3")
#[no_mangle]
pub unsafe extern "C" fn __msg(value": "unbounded min) -> __failure {
    __failure __msg("unbounded min value")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn signed_and_unsigned_variant_3() -> __naked void {
    __naked void signed_and_unsigned_variant_3(void)
    {
    asm volatile ("					\
    call %[bpf_ktime_get_ns];			\
// (u64*)(r10 - 16) = r0;				\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_8b] ll;				\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = *(u64*)(r10 - 16);				\
    r2 = -1;					\
    if r1 > r2 goto l0_%=;				\
    r8 = r1;					\
    if r8 s> 1 goto l0_%=;				\
    r0 += r8;					\
    r0 = 0;						\
// (u8*)(r8 + 0) = r0;				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_ktime_get_ns),
    __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_8b)
    : __clobber_all);
    }
    SEC("socket")
    __description("bounds checks mixing signed and unsigned, variant 4")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn signed_and_unsigned_variant_4() -> __naked void {
    __naked void signed_and_unsigned_variant_4(void)
    {
    asm volatile ("					\
    call %[bpf_ktime_get_ns];			\
// (u64*)(r10 - 16) = r0;				\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_8b] ll;				\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = *(u64*)(r10 - 16);				\
    r2 = 1;						\
    r1 &= r2;					\
    if r1 s> 1 goto l0_%=;				\
    r0 += r1;					\
    r1 = 0;						\
// (u8*)(r0 + 0) = r1;				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_ktime_get_ns),
    __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_8b)
    : __clobber_all);
    }
    SEC("socket")
    __description("bounds checks mixing signed and unsigned, variant 5")
#[no_mangle]
pub unsafe extern "C" fn __msg(value": "unbounded min) -> __failure {
    __failure __msg("unbounded min value")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn signed_and_unsigned_variant_5() -> __naked void {
    __naked void signed_and_unsigned_variant_5(void)
    {
    asm volatile ("					\
    call %[bpf_ktime_get_ns];			\
// (u64*)(r10 - 16) = r0;				\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_8b] ll;				\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = *(u64*)(r10 - 16);				\
    r2 = -1;					\
    if r1 > r2 goto l0_%=;				\
    if r1 s> 1 goto l0_%=;				\
    r0 += 4;					\
    r0 -= r1;					\
    r1 = 0;						\
// (u8*)(r0 + 0) = r1;				\
    r0 = 0;						\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_ktime_get_ns),
    __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_8b)
    : __clobber_all);
    }
    SEC("socket")
    __description("bounds checks mixing signed and unsigned, variant 6")
#[no_mangle]
pub unsafe extern "C" fn __msg(negative: "R4 min value is, unsigned": either use) -> __failure {
    __failure __msg("R4 min value is negative, either use unsigned")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn signed_and_unsigned_variant_6() -> __naked void {
    __naked void signed_and_unsigned_variant_6(void)
    {
    asm volatile ("					\
    r9 = r1;					\
    call %[bpf_ktime_get_ns];			\
// (u64*)(r10 - 16) = r0;				\
    r1 = r9;					\
    r2 = 0;						\
    r3 = r10;					\
    r3 += -512;					\
    r4 = *(u64*)(r10 - 16);				\
    r6 = -1;					\
    if r4 > r6 goto l0_%=;				\
    if r4 s> 1 goto l0_%=;				\
    r4 += 1;					\
    r5 = 0;						\
    r6 = 0;						\
// (u16*)(r10 - 512) = r6;			\
    call %[bpf_skb_load_bytes];			\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_ktime_get_ns),
    __imm(bpf_skb_load_bytes)
    : __clobber_all);
    }
    SEC("socket")
    __description("bounds checks mixing signed and unsigned, variant 7")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn signed_and_unsigned_variant_7() -> __naked void {
    __naked void signed_and_unsigned_variant_7(void)
    {
    asm volatile ("					\
    call %[bpf_ktime_get_ns];			\
// (u64*)(r10 - 16) = r0;				\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_8b] ll;				\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = *(u64*)(r10 - 16);				\
    r2 = %[__imm_0];				\
    if r1 > r2 goto l0_%=;				\
    if r1 s> 1 goto l0_%=;				\
    r0 += r1;					\
    r1 = 0;						\
// (u8*)(r0 + 0) = r1;				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_ktime_get_ns),
    __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_8b),
    __imm_const(__imm_0, 1024 * 1024 * 1024)
    : __clobber_all);
    }
    SEC("socket")
    __description("bounds checks mixing signed and unsigned, variant 8")
#[no_mangle]
pub unsafe extern "C" fn __msg(value": "unbounded min) -> __failure {
    __failure __msg("unbounded min value")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn signed_and_unsigned_variant_8() -> __naked void {
    __naked void signed_and_unsigned_variant_8(void)
    {
    asm volatile ("					\
    call %[bpf_ktime_get_ns];			\
// (u64*)(r10 - 16) = r0;				\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_8b] ll;				\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = *(u64*)(r10 - 16);				\
    r2 = -1;					\
    if r2 > r1 goto l1_%=;				\
    r0 = 0;						\
    exit;						\
    l1_%=:	if r1 s> 1 goto l0_%=;				\
    r0 += r1;					\
    r1 = 0;						\
// (u8*)(r0 + 0) = r1;				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_ktime_get_ns),
    __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_8b)
    : __clobber_all);
    }
    SEC("socket")
    __description("bounds checks mixing signed and unsigned, variant 9")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn signed_and_unsigned_variant_9() -> __naked void {
    __naked void signed_and_unsigned_variant_9(void)
    {
    asm volatile ("					\
    call %[bpf_ktime_get_ns];			\
// (u64*)(r10 - 16) = r0;				\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_8b] ll;				\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = *(u64*)(r10 - 16);				\
    r2 = -9223372036854775808ULL ll;		\
    if r2 > r1 goto l1_%=;				\
    r0 = 0;						\
    exit;						\
    l1_%=:	if r1 s> 1 goto l0_%=;				\
    r0 += r1;					\
    r1 = 0;						\
// (u8*)(r0 + 0) = r1;				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_ktime_get_ns),
    __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_8b)
    : __clobber_all);
    }
    SEC("socket")
    __description("bounds checks mixing signed and unsigned, variant 10")
#[no_mangle]
pub unsafe extern "C" fn __msg(value": "unbounded min) -> __failure {
    __failure __msg("unbounded min value")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn signed_and_unsigned_variant_10() -> __naked void {
    __naked void signed_and_unsigned_variant_10(void)
    {
    asm volatile ("					\
    call %[bpf_ktime_get_ns];			\
// (u64*)(r10 - 16) = r0;				\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_8b] ll;				\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = *(u64*)(r10 - 16);				\
    r2 = -1;						\
    if r2 > r1 goto l1_%=;				\
    r0 = 0;						\
    exit;						\
    l1_%=:	if r1 s> 1 goto l0_%=;				\
    r0 += r1;					\
    r1 = 0;						\
// (u8*)(r0 + 0) = r1;				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_ktime_get_ns),
    __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_8b)
    : __clobber_all);
    }
    SEC("socket")
    __description("bounds checks mixing signed and unsigned, variant 11")
#[no_mangle]
pub unsafe extern "C" fn __msg(value": "unbounded min) -> __failure {
    __failure __msg("unbounded min value")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn signed_and_unsigned_variant_11() -> __naked void {
    __naked void signed_and_unsigned_variant_11(void)
    {
    asm volatile ("					\
    call %[bpf_ktime_get_ns];			\
// (u64*)(r10 - 16) = r0;				\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_8b] ll;				\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = *(u64*)(r10 - 16);				\
    r2 = -1;					\
    if r2 >= r1 goto l1_%=;				\
// Dead branch. */				\
    r0 = 0;						\
    exit;						\
    l1_%=:	if r1 s> 1 goto l0_%=;				\
    r0 += r1;					\
    r1 = 0;						\
// (u8*)(r0 + 0) = r1;				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_ktime_get_ns),
    __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_8b)
    : __clobber_all);
    }
    SEC("socket")
    __description("bounds checks mixing signed and unsigned, variant 12")
#[no_mangle]
pub unsafe extern "C" fn __msg(value": "unbounded min) -> __failure {
    __failure __msg("unbounded min value")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn signed_and_unsigned_variant_12() -> __naked void {
    __naked void signed_and_unsigned_variant_12(void)
    {
    asm volatile ("					\
    call %[bpf_ktime_get_ns];			\
// (u64*)(r10 - 16) = r0;				\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_8b] ll;				\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = *(u64*)(r10 - 16);				\
    r2 = -6;					\
    if r2 >= r1 goto l1_%=;				\
    r0 = 0;						\
    exit;						\
    l1_%=:	if r1 s> 1 goto l0_%=;				\
    r0 += r1;					\
    r1 = 0;						\
// (u8*)(r0 + 0) = r1;				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_ktime_get_ns),
    __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_8b)
    : __clobber_all);
    }
    SEC("socket")
    __description("bounds checks mixing signed and unsigned, variant 13")
#[no_mangle]
pub unsafe extern "C" fn __msg(value": "unbounded min) -> __failure {
    __failure __msg("unbounded min value")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn signed_and_unsigned_variant_13() -> __naked void {
    __naked void signed_and_unsigned_variant_13(void)
    {
    asm volatile ("					\
    call %[bpf_ktime_get_ns];			\
// (u64*)(r10 - 16) = r0;				\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_8b] ll;				\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = *(u64*)(r10 - 16);				\
    r2 = 2;						\
    if r2 >= r1 goto l0_%=;				\
    r7 = 1;						\
    if r7 s> 0 goto l1_%=;				\
    l0_%=:	r0 = 0;						\
    exit;						\
    l1_%=:	r7 += r1;					\
    if r7 s> 4 goto l2_%=;				\
    r0 += r7;					\
    r1 = 0;						\
// (u8*)(r0 + 0) = r1;				\
    l2_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_ktime_get_ns),
    __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_8b)
    : __clobber_all);
    }
    SEC("socket")
    __description("bounds checks mixing signed and unsigned, variant 14")
#[no_mangle]
pub unsafe extern "C" fn __msg(value": "unbounded min) -> __failure {
    __failure __msg("unbounded min value")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn signed_and_unsigned_variant_14() -> __naked void {
    __naked void signed_and_unsigned_variant_14(void)
    {
    asm volatile ("					\
    r9 = *(u32*)(r1 + %[__sk_buff_mark]);		\
    call %[bpf_ktime_get_ns];			\
// (u64*)(r10 - 16) = r0;				\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_8b] ll;				\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = *(u64*)(r10 - 16);				\
    r2 = -1;					\
    r8 = 2;						\
    if r9 == 42 goto l1_%=;				\
    if r8 s> r1 goto l2_%=;				\
    l3_%=:	if r1 s> 1 goto l2_%=;				\
    r0 += r1;					\
    l0_%=:	r1 = 0;						\
// (u8*)(r0 + 0) = r1;				\
    l2_%=:	r0 = 0;						\
    exit;						\
    l1_%=:	if r1 > r2 goto l2_%=;				\
    goto l3_%=;					\
    "	:
    : __imm(bpf_ktime_get_ns),
    __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_8b),
    __imm_const(__sk_buff_mark, offsetof(struct __sk_buff, mark))
    : __clobber_all);
    }
    SEC("socket")
    __description("bounds checks mixing signed and unsigned, variant 15")
#[no_mangle]
pub unsafe extern "C" fn __msg(value": "unbounded min) -> __failure {
    __failure __msg("unbounded min value")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn signed_and_unsigned_variant_15() -> __naked void {
    __naked void signed_and_unsigned_variant_15(void)
    {
    asm volatile ("					\
    call %[bpf_ktime_get_ns];			\
// (u64*)(r10 - 16) = r0;				\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_8b] ll;				\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = *(u64*)(r10 - 16);				\
    r2 = -6;					\
    if r2 >= r1 goto l1_%=;				\
    l0_%=:	r0 = 0;						\
    exit;						\
    l1_%=:	r0 += r1;					\
    if r0 > 1 goto l2_%=;				\
    r0 = 0;						\
    exit;						\
    l2_%=:	r1 = 0;						\
// (u8*)(r0 + 0) = r1;				\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_ktime_get_ns),
    __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_8b)
    : __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
