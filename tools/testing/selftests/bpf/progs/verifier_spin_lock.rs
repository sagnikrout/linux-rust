//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_spin_lock.c
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
// Converted from tools/testing/selftests/bpf/verifier/spin_lock.c

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
    SEC("cgroup/skb")
    __description("spin_lock: test1 success")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(_arg: "") -> __success __failure_unpriv {
    __success __failure_unpriv __msg_unpriv("")
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn spin_lock_test1_success() -> __naked void {
    __naked void spin_lock_test1_success(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u32*)(r10 - 4) = r1;				\
    r2 = r10;					\
    r2 += -4;					\
    r1 = %[map_spin_lock] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 != 0 goto l0_%=;				\
    exit;						\
    l0_%=:	r6 = r0;					\
    r1 = r0;					\
    r1 += 4;					\
    call %[bpf_spin_lock];				\
    r1 = r6;					\
    r1 += 4;					\
    r0 = *(u32*)(r6 + 0);				\
    call %[bpf_spin_unlock];			\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_spin_lock),
    __imm(bpf_spin_unlock),
    __imm_addr(map_spin_lock)
    : __clobber_all);
    }
    SEC("cgroup/skb")
    __description("spin_lock: test2 direct ld/st")
#[no_mangle]
pub unsafe extern "C" fn __msg(directly": "cannot be accessed) -> __failure {
    __failure __msg("cannot be accessed directly")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(_arg: "") -> __failure_unpriv {
    __failure_unpriv __msg_unpriv("")
#[no_mangle]
pub unsafe extern "C" fn lock_test2_direct_ld_st() -> __naked void {
    __naked void lock_test2_direct_ld_st(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u32*)(r10 - 4) = r1;				\
    r2 = r10;					\
    r2 += -4;					\
    r1 = %[map_spin_lock] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 != 0 goto l0_%=;				\
    exit;						\
    l0_%=:	r6 = r0;					\
    r1 = r0;					\
    r1 += 4;					\
    call %[bpf_spin_lock];				\
    r1 = r6;					\
    r1 += 4;					\
    r0 = *(u32*)(r1 + 0);				\
    call %[bpf_spin_unlock];			\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_spin_lock),
    __imm(bpf_spin_unlock),
    __imm_addr(map_spin_lock)
    : __clobber_all);
    }
    SEC("cgroup/skb")
    __description("spin_lock: test3 direct ld/st")
#[no_mangle]
pub unsafe extern "C" fn __msg(directly": "cannot be accessed) -> __failure {
    __failure __msg("cannot be accessed directly")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(_arg: "") -> __failure_unpriv {
    __failure_unpriv __msg_unpriv("")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn lock_test3_direct_ld_st() -> __naked void {
    __naked void lock_test3_direct_ld_st(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u32*)(r10 - 4) = r1;				\
    r2 = r10;					\
    r2 += -4;					\
    r1 = %[map_spin_lock] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 != 0 goto l0_%=;				\
    exit;						\
    l0_%=:	r6 = r0;					\
    r1 = r0;					\
    r1 += 4;					\
    call %[bpf_spin_lock];				\
    r1 = r6;					\
    r1 += 4;					\
    r0 = *(u32*)(r6 + 1);				\
    call %[bpf_spin_unlock];			\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_spin_lock),
    __imm(bpf_spin_unlock),
    __imm_addr(map_spin_lock)
    : __clobber_all);
    }
    SEC("cgroup/skb")
    __description("spin_lock: test4 direct ld/st")
#[no_mangle]
pub unsafe extern "C" fn __msg(directly": "cannot be accessed) -> __failure {
    __failure __msg("cannot be accessed directly")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(_arg: "") -> __failure_unpriv {
    __failure_unpriv __msg_unpriv("")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn lock_test4_direct_ld_st() -> __naked void {
    __naked void lock_test4_direct_ld_st(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u32*)(r10 - 4) = r1;				\
    r2 = r10;					\
    r2 += -4;					\
    r1 = %[map_spin_lock] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 != 0 goto l0_%=;				\
    exit;						\
    l0_%=:	r6 = r0;					\
    r1 = r0;					\
    r1 += 4;					\
    call %[bpf_spin_lock];				\
    r1 = r6;					\
    r1 += 4;					\
    r0 = *(u16*)(r6 + 3);				\
    call %[bpf_spin_unlock];			\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_spin_lock),
    __imm(bpf_spin_unlock),
    __imm_addr(map_spin_lock)
    : __clobber_all);
    }
    SEC("cgroup/skb")
    __description("spin_lock: test5 call within a locked region")
#[no_mangle]
pub unsafe extern "C" fn __msg(allowed": "calls are not) -> __failure {
    __failure __msg("calls are not allowed")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(_arg: "") -> __failure_unpriv {
    __failure_unpriv __msg_unpriv("")
#[no_mangle]
pub unsafe extern "C" fn call_within_a_locked_region() -> __naked void {
    __naked void call_within_a_locked_region(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u32*)(r10 - 4) = r1;				\
    r2 = r10;					\
    r2 += -4;					\
    r1 = %[map_spin_lock] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 != 0 goto l0_%=;				\
    exit;						\
    l0_%=:	r6 = r0;					\
    r1 = r0;					\
    r1 += 4;					\
    call %[bpf_spin_lock];				\
    call %[bpf_get_prandom_u32];			\
    r1 = r6;					\
    r1 += 4;					\
    call %[bpf_spin_unlock];			\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_get_prandom_u32),
    __imm(bpf_map_lookup_elem),
    __imm(bpf_spin_lock),
    __imm(bpf_spin_unlock),
    __imm_addr(map_spin_lock)
    : __clobber_all);
    }
    SEC("cgroup/skb")
    __description("spin_lock: test6 missing unlock")
#[no_mangle]
pub unsafe extern "C" fn __msg(region": "BPF_EXIT instruction in main prog cannot be used inside bpf_spin_lock-ed) -> __failure {
    __failure __msg("BPF_EXIT instruction in main prog cannot be used inside bpf_spin_lock-ed region")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(_arg: "") -> __failure_unpriv {
    __failure_unpriv __msg_unpriv("")
#[no_mangle]
pub unsafe extern "C" fn spin_lock_test6_missing_unlock() -> __naked void {
    __naked void spin_lock_test6_missing_unlock(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u32*)(r10 - 4) = r1;				\
    r2 = r10;					\
    r2 += -4;					\
    r1 = %[map_spin_lock] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 != 0 goto l0_%=;				\
    exit;						\
    l0_%=:	r6 = r0;					\
    r1 = r0;					\
    r1 += 4;					\
    call %[bpf_spin_lock];				\
    r1 = r6;					\
    r1 += 4;					\
    r0 = *(u32*)(r6 + 0);				\
    if r0 != 0 goto l1_%=;				\
    call %[bpf_spin_unlock];			\
    l1_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_spin_lock),
    __imm(bpf_spin_unlock),
    __imm_addr(map_spin_lock)
    : __clobber_all);
    }
    SEC("cgroup/skb")
    __description("spin_lock: test7 unlock without lock")
#[no_mangle]
pub unsafe extern "C" fn __msg(lock": "without taking a) -> __failure {
    __failure __msg("without taking a lock")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(_arg: "") -> __failure_unpriv {
    __failure_unpriv __msg_unpriv("")
#[no_mangle]
pub unsafe extern "C" fn lock_test7_unlock_without_lock() -> __naked void {
    __naked void lock_test7_unlock_without_lock(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u32*)(r10 - 4) = r1;				\
    r2 = r10;					\
    r2 += -4;					\
    r1 = %[map_spin_lock] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 != 0 goto l0_%=;				\
    exit;						\
    l0_%=:	r6 = r0;					\
    r1 = r0;					\
    r1 += 4;					\
    if r1 != 0 goto l1_%=;				\
    call %[bpf_spin_lock];				\
    l1_%=:	r1 = r6;					\
    r1 += 4;					\
    r0 = *(u32*)(r6 + 0);				\
    call %[bpf_spin_unlock];			\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_spin_lock),
    __imm(bpf_spin_unlock),
    __imm_addr(map_spin_lock)
    : __clobber_all);
    }
    SEC("cgroup/skb")
    __description("spin_lock: test8 double lock")
#[no_mangle]
pub unsafe extern "C" fn __msg(allowed": "calls are not) -> __failure {
    __failure __msg("calls are not allowed")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(_arg: "") -> __failure_unpriv {
    __failure_unpriv __msg_unpriv("")
#[no_mangle]
pub unsafe extern "C" fn spin_lock_test8_double_lock() -> __naked void {
    __naked void spin_lock_test8_double_lock(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u32*)(r10 - 4) = r1;				\
    r2 = r10;					\
    r2 += -4;					\
    r1 = %[map_spin_lock] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 != 0 goto l0_%=;				\
    exit;						\
    l0_%=:	r6 = r0;					\
    r1 = r0;					\
    r1 += 4;					\
    call %[bpf_spin_lock];				\
    r1 = r6;					\
    r1 += 4;					\
    call %[bpf_spin_lock];				\
    r1 = r6;					\
    r1 += 4;					\
    r0 = *(u32*)(r6 + 0);				\
    call %[bpf_spin_unlock];			\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_spin_lock),
    __imm(bpf_spin_unlock),
    __imm_addr(map_spin_lock)
    : __clobber_all);
    }
    SEC("cgroup/skb")
    __description("spin_lock: test9 different lock")
#[no_mangle]
pub unsafe extern "C" fn __msg(lock": "unlock of different) -> __failure {
    __failure __msg("unlock of different lock")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(_arg: "") -> __failure_unpriv {
    __failure_unpriv __msg_unpriv("")
#[no_mangle]
pub unsafe extern "C" fn spin_lock_test9_different_lock() -> __naked void {
    __naked void spin_lock_test9_different_lock(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u32*)(r10 - 4) = r1;				\
    r2 = r10;					\
    r2 += -4;					\
    r1 = %[map_spin_lock] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 != 0 goto l0_%=;				\
    exit;						\
    l0_%=:	r6 = r0;					\
    r2 = r10;					\
    r2 += -4;					\
    r1 = %[map_spin_lock] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 != 0 goto l1_%=;				\
    exit;						\
    l1_%=:	r7 = r0;					\
    r1 = r6;					\
    r1 += 4;					\
    call %[bpf_spin_lock];				\
    r1 = r7;					\
    r1 += 4;					\
    call %[bpf_spin_unlock];			\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_spin_lock),
    __imm(bpf_spin_unlock),
    __imm_addr(map_spin_lock)
    : __clobber_all);
    }
    SEC("cgroup/skb")
    __description("spin_lock: test10 lock in subprog without unlock")
    __success
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(_arg: "") -> __failure_unpriv {
    __failure_unpriv __msg_unpriv("")
#[no_mangle]
pub unsafe extern "C" fn lock_in_subprog_without_unlock() -> __naked void {
    __naked void lock_in_subprog_without_unlock(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u32*)(r10 - 4) = r1;				\
    r2 = r10;					\
    r2 += -4;					\
    r1 = %[map_spin_lock] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 != 0 goto l0_%=;				\
    exit;						\
    l0_%=:	r6 = r0;					\
    r1 = r0;					\
    r1 += 4;					\
    call lock_in_subprog_without_unlock__1;		\
    r1 = r6;					\
    r1 += 4;					\
    call %[bpf_spin_unlock];			\
    r0 = 1;						\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_spin_unlock),
    __imm_addr(map_spin_lock)
    : __clobber_all);
    }
#[no_mangle]
unsafe extern "C" fn __attribute__(_arg: (used)) -> __naked __noinline {
    static __naked __noinline __attribute__((used))
#[no_mangle]
pub unsafe extern "C" fn lock_in_subprog_without_unlock__1() {
    void lock_in_subprog_without_unlock__1(void)
    {
    asm volatile ("					\
    call %[bpf_spin_lock];				\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_spin_lock)
    : __clobber_all);
    }
    SEC("tc")
    __description("spin_lock: test11 ld_abs under lock")
#[no_mangle]
pub unsafe extern "C" fn __msg(bpf_spin_lock": "inside) -> __failure {
    __failure __msg("inside bpf_spin_lock")
#[no_mangle]
pub unsafe extern "C" fn test11_ld_abs_under_lock() -> __naked void {
    __naked void test11_ld_abs_under_lock(void)
    {
    asm volatile ("					\
    r6 = r1;					\
    r1 = 0;						\
// (u32*)(r10 - 4) = r1;				\
    r2 = r10;					\
    r2 += -4;					\
    r1 = %[map_spin_lock] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 != 0 goto l0_%=;				\
    exit;						\
    l0_%=:	r7 = r0;					\
    r1 = r0;					\
    r1 += 4;					\
    call %[bpf_spin_lock];				\
    r0 = *(u8*)skb[0];				\
    r1 = r7;					\
    r1 += 4;					\
    call %[bpf_spin_unlock];			\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_spin_lock),
    __imm(bpf_spin_unlock),
    __imm_addr(map_spin_lock)
    : __clobber_all);
    }
    SEC("tc")
    __description("spin_lock: regsafe compare reg.id for map value")
#[no_mangle]
pub unsafe extern "C" fn __msg(lock": "bpf_spin_unlock of different) -> __failure {
    __failure __msg("bpf_spin_unlock of different lock")
    __flag(BPF_F_TEST_STATE_FREQ)
#[no_mangle]
pub unsafe extern "C" fn reg_id_for_map_value() -> __naked void {
    __naked void reg_id_for_map_value(void)
    {
    asm volatile ("					\
    r6 = r1;					\
    r6 = *(u32*)(r6 + %[__sk_buff_mark]);		\
    r1 = %[map_spin_lock] ll;			\
    r9 = r1;					\
    r2 = 0;						\
// (u32*)(r10 - 4) = r2;				\
    r2 = r10;					\
    r2 += -4;					\
    call %[bpf_map_lookup_elem];			\
    if r0 != 0 goto l0_%=;				\
    exit;						\
    l0_%=:	r7 = r0;					\
    r1 = r9;					\
    r2 = r10;					\
    r2 += -4;					\
    call %[bpf_map_lookup_elem];			\
    if r0 != 0 goto l1_%=;				\
    exit;						\
    l1_%=:	r8 = r0;					\
    r1 = r7;					\
    r1 += 4;					\
    call %[bpf_spin_lock];				\
    if r6 == 0 goto l2_%=;				\
    goto l3_%=;					\
    l2_%=:	r7 = r8;					\
    l3_%=:	r1 = r7;					\
    r1 += 4;					\
    call %[bpf_spin_unlock];			\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_spin_lock),
    __imm(bpf_spin_unlock),
    __imm_addr(map_spin_lock),
    __imm_const(__sk_buff_mark, offsetof(struct __sk_buff, mark))
    : __clobber_all);
    }
// Make sure that regsafe() compares ids for spin lock records using
// check_ids():
// 1: r9 = map_lookup_elem(...)  ; r9.id == 1
// 2: r8 = map_lookup_elem(...)  ; r8.id == 2
// 3: r7 = ktime_get_ns()
// 4: r6 = ktime_get_ns()
// 5: if r6 > r7 goto <9>
// 6: spin_lock(r8)
// 7: r9 = r8
// 8: goto <10>
// 9: spin_lock(r9)
// 10: spin_unlock(r9)             ; r9.id == 1 || r9.id == 2 and lock is active,
// ; second visit to (10) should be considered safe
// ; if check_ids() is used.
// 11: exit(0)
//
    SEC("cgroup/skb")
    __description("spin_lock: regsafe() check_ids() similar id mappings")
#[no_mangle]
pub unsafe extern "C" fn __msg(safe": "29:) -> __success {
    __success __msg("29: safe")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(_arg: "") -> __failure_unpriv {
    __failure_unpriv __msg_unpriv("")
    __log_level(2) __retval(0) __flag(BPF_F_TEST_STATE_FREQ)
#[no_mangle]
pub unsafe extern "C" fn check_ids_similar_id_mappings() -> __naked void {
    __naked void check_ids_similar_id_mappings(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u32*)(r10 - 4) = r1;				\
// r9 = map_lookup_elem(...) */			\
    r2 = r10;					\
    r2 += -4;					\
    r1 = %[map_spin_lock] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r9 = r0;					\
// r8 = map_lookup_elem(...) */			\
    r2 = r10;					\
    r2 += -4;					\
    r1 = %[map_spin_lock] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l1_%=;				\
    r8 = r0;					\
// r7 = ktime_get_ns() */			\
    call %[bpf_ktime_get_ns];			\
    r7 = r0;					\
// r6 = ktime_get_ns() */			\
    call %[bpf_ktime_get_ns];			\
    r6 = r0;					\
// if r6 > r7 goto +5      ; no new information about the state is derived from\
// ; this check, thus produced verifier states differ\
// ; only in 'insn_idx'	\
// spin_lock(r8)				\
// r9 = r8					\
// goto unlock					\
// \
    if r6 > r7 goto l2_%=;				\
    r1 = r8;					\
    r1 += 4;					\
    call %[bpf_spin_lock];				\
    r9 = r8;					\
    goto l3_%=;					\
    l2_%=:	/* spin_lock(r9) */				\
    r1 = r9;					\
    r1 += 4;					\
    call %[bpf_spin_lock];				\
    l3_%=:	/* spin_unlock(r9) */				\
    r1 = r9;					\
    r1 += 4;					\
    call %[bpf_spin_unlock];			\
    l0_%=:	/* exit(0) */					\
    r0 = 0;						\
    l1_%=:	exit;						\
    "	:
    : __imm(bpf_ktime_get_ns),
    __imm(bpf_map_lookup_elem),
    __imm(bpf_spin_lock),
    __imm(bpf_spin_unlock),
    __imm_addr(map_spin_lock)
    : __clobber_all);
    }
    SEC("tc")
    __description("spin_lock: loop within a locked region")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(_arg: "") -> __success __failure_unpriv {
    __success __failure_unpriv __msg_unpriv("")
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn bpf_loop_inside_locked_region() -> c_int {
    int bpf_loop_inside_locked_region(void)
    {
    let mut zero: c_int = 0;
    struct val *val;
    int i, j = 0;
    val = bpf_map_lookup_elem(&map_spin_lock, &zero);
    if (!val)
    return -1;
    bpf_spin_lock(&val.l);
    bpf_for(i, 0, 10) {
    j++;
// Silence "unused variable" warnings.
    if (j == 10)
    break;
    }
    bpf_spin_unlock(&val.l);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
