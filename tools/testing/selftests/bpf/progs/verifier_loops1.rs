//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_loops1.c
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
// Converted from tools/testing/selftests/bpf/verifier/loops1.c

    SEC("xdp")
    __description("bounded loop, count to 4")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 4) -> __success {
    __success __retval(4)
#[no_mangle]
pub unsafe extern "C" fn bounded_loop_count_to_4() -> __naked void {
    __naked void bounded_loop_count_to_4(void)
    {
    asm volatile ("					\
    r0 = 0;						\
    l0_%=:	r0 += 1;					\
    if r0 < 4 goto l0_%=;				\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("tracepoint")
    __description("bounded loop, count to 20")
    __success
#[no_mangle]
pub unsafe extern "C" fn bounded_loop_count_to_20() -> __naked void {
    __naked void bounded_loop_count_to_20(void)
    {
    asm volatile ("					\
    r0 = 0;						\
    l0_%=:	r0 += 3;					\
    if r0 < 20 goto l0_%=;				\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("tracepoint")
    __description("bounded loop, count from positive unknown to 4")
    __success
#[no_mangle]
pub unsafe extern "C" fn from_positive_unknown_to_4() -> __naked void {
    __naked void from_positive_unknown_to_4(void)
    {
    asm volatile ("					\
    call %[bpf_get_prandom_u32];			\
    if r0 s< 0 goto l0_%=;				\
    l1_%=:	r0 += 1;					\
    if r0 < 4 goto l1_%=;				\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_get_prandom_u32)
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("bounded loop, count from totally unknown to 4")
    __success
#[no_mangle]
pub unsafe extern "C" fn from_totally_unknown_to_4() -> __naked void {
    __naked void from_totally_unknown_to_4(void)
    {
    asm volatile ("					\
    call %[bpf_get_prandom_u32];			\
    l0_%=:	r0 += 1;					\
    if r0 < 4 goto l0_%=;				\
    exit;						\
    "	:
    : __imm(bpf_get_prandom_u32)
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("bounded loop, count to 4 with equality")
    __success
#[no_mangle]
pub unsafe extern "C" fn count_to_4_with_equality() -> __naked void {
    __naked void count_to_4_with_equality(void)
    {
    asm volatile ("					\
    r0 = 0;						\
    l0_%=:	r0 += 1;					\
    if r0 != 4 goto l0_%=;				\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("bounded loop, start in the middle")
    __success
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(_arg: "back-edge") -> __failure_unpriv {
    __failure_unpriv __msg_unpriv("back-edge")
#[no_mangle]
pub unsafe extern "C" fn loop_start_in_the_middle() -> __naked void {
    __naked void loop_start_in_the_middle(void)
    {
    asm volatile ("					\
    r0 = 0;						\
    goto l0_%=;					\
    l1_%=:	r0 += 1;					\
    l0_%=:	if r0 < 4 goto l1_%=;				\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("xdp")
    __description("bounded loop containing a forward jump")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 4) -> __success {
    __success __retval(4)
#[no_mangle]
pub unsafe extern "C" fn loop_containing_a_forward_jump() -> __naked void {
    __naked void loop_containing_a_forward_jump(void)
    {
    asm volatile ("					\
    r0 = 0;						\
    l1_%=:	r0 += 1;					\
    if r0 == r0 goto l0_%=;				\
    l0_%=:	if r0 < 4 goto l1_%=;				\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("tracepoint")
    __description("bounded loop that jumps out rather than in")
    __success
#[no_mangle]
pub unsafe extern "C" fn jumps_out_rather_than_in() -> __naked void {
    __naked void jumps_out_rather_than_in(void)
    {
    asm volatile ("					\
    r6 = 0;						\
    l1_%=:	r6 += 1;					\
    if r6 > 10000 goto l0_%=;			\
    call %[bpf_get_prandom_u32];			\
    goto l1_%=;					\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_get_prandom_u32)
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("infinite loop after a conditional jump")
#[no_mangle]
pub unsafe extern "C" fn __msg(large": "program is too) -> __failure {
    __failure __msg("program is too large")
#[no_mangle]
pub unsafe extern "C" fn loop_after_a_conditional_jump() -> __naked void {
    __naked void loop_after_a_conditional_jump(void)
    {
    asm volatile ("					\
    r0 = 5;						\
    if r0 < 4 goto l0_%=;				\
    l1_%=:	r0 += 1;					\
    goto l1_%=;					\
    l0_%=:	exit;						\
    "	::: __clobber_all);
    }
    SEC("tracepoint")
    __description("bounded recursion")
    __failure
    __msg("recursive call from")
#[no_mangle]
pub unsafe extern "C" fn bounded_recursion() -> __naked void {
    __naked void bounded_recursion(void)
    {
    asm volatile ("					\
    r1 = 0;						\
    call bounded_recursion__1;			\
    exit;						\
    "	::: __clobber_all);
    }
#[no_mangle]
unsafe extern "C" fn __attribute__(_arg: (used)) -> __naked __noinline {
    static __naked __noinline __attribute__((used))
#[no_mangle]
pub unsafe extern "C" fn bounded_recursion__1() {
    void bounded_recursion__1(void)
    {
    asm volatile ("					\
    r1 += 1;					\
    r0 = r1;					\
    if r1 < 4 goto l0_%=;				\
    exit;						\
    l0_%=:	call bounded_recursion__1;			\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("tracepoint")
    __description("infinite loop in two jumps")
#[no_mangle]
pub unsafe extern "C" fn __msg(detected": "loop) -> __failure {
    __failure __msg("loop detected")
#[no_mangle]
pub unsafe extern "C" fn infinite_loop_in_two_jumps() -> __naked void {
    __naked void infinite_loop_in_two_jumps(void)
    {
    asm volatile ("					\
    r0 = 0;						\
    l1_%=:	goto l0_%=;					\
    l0_%=:	if r0 < 4 goto l1_%=;				\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("tracepoint")
    __description("infinite loop: three-jump trick")
#[no_mangle]
pub unsafe extern "C" fn __msg(detected": "loop) -> __failure {
    __failure __msg("loop detected")
#[no_mangle]
pub unsafe extern "C" fn infinite_loop_three_jump_trick() -> __naked void {
    __naked void infinite_loop_three_jump_trick(void)
    {
    asm volatile ("					\
    r0 = 0;						\
    l2_%=:	r0 += 1;					\
    r0 &= 1;					\
    if r0 < 2 goto l0_%=;				\
    exit;						\
    l0_%=:	r0 += 1;					\
    r0 &= 1;					\
    if r0 < 2 goto l1_%=;				\
    exit;						\
    l1_%=:	r0 += 1;					\
    r0 &= 1;					\
    if r0 < 2 goto l2_%=;				\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("xdp")
    __description("not-taken loop with back jump to 1st insn")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 123) -> __success {
    __success __retval(123)
#[no_mangle]
pub unsafe extern "C" fn back_jump_to_1st_insn_1() -> __naked void {
    __naked void back_jump_to_1st_insn_1(void)
    {
    asm volatile ("					\
    l0_%=:	r0 = 123;					\
    if r0 == 4 goto l0_%=;				\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("xdp")
    __description("taken loop with back jump to 1st insn")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 55) -> __success {
    __success __retval(55)
#[no_mangle]
pub unsafe extern "C" fn back_jump_to_1st_insn_2() -> __naked void {
    __naked void back_jump_to_1st_insn_2(void)
    {
    asm volatile ("					\
    r1 = 10;					\
    r2 = 0;						\
    call back_jump_to_1st_insn_2__1;		\
    exit;						\
    "	::: __clobber_all);
    }
#[no_mangle]
unsafe extern "C" fn __attribute__(_arg: (used)) -> __naked __noinline {
    static __naked __noinline __attribute__((used))
#[no_mangle]
pub unsafe extern "C" fn back_jump_to_1st_insn_2__1() {
    void back_jump_to_1st_insn_2__1(void)
    {
    asm volatile ("					\
    l0_%=:	r2 += r1;					\
    r1 -= 1;					\
    if r1 != 0 goto l0_%=;				\
    r0 = r2;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("xdp")
    __description("taken loop with back jump to 1st insn, 2")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 55) -> __success {
    __success __retval(55)
#[no_mangle]
pub unsafe extern "C" fn jump_to_1st_insn_2() -> __naked void {
    __naked void jump_to_1st_insn_2(void)
    {
    asm volatile ("					\
    r1 = 10;					\
    r2 = 0;						\
    call jump_to_1st_insn_2__1;			\
    exit;						\
    "	::: __clobber_all);
    }
#[no_mangle]
unsafe extern "C" fn __attribute__(_arg: (used)) -> __naked __noinline {
    static __naked __noinline __attribute__((used))
#[no_mangle]
pub unsafe extern "C" fn jump_to_1st_insn_2__1() {
    void jump_to_1st_insn_2__1(void)
    {
    asm volatile ("					\
    l0_%=:	r2 += r1;					\
    r1 -= 1;					\
    if w1 != 0 goto l0_%=;				\
    r0 = r2;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("xdp")
    __success
#[no_mangle]
pub unsafe extern "C" fn not_an_inifinite_loop() -> __naked void {
    __naked void not_an_inifinite_loop(void)
    {
    asm volatile ("					\
    call %[bpf_get_prandom_u32];			\
    r0 &= 0xff;					\
// (u64 *)(r10 - 8) = r0;				\
    r0 = 0;						\
    loop_%=:						\
    r0 = *(u64 *)(r10 - 8);				\
    if r0 > 10 goto exit_%=;			\
    r0 += 1;					\
// (u64 *)(r10 - 8) = r0;				\
    r0 = 0;						\
    goto loop_%=;					\
    exit_%=:						\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_get_prandom_u32)
    : __clobber_all);
    }
//
// This test case triggered a bug in verifier.c:maybe_exit_scc().
// Speculative execution path reaches stack access instruction,
// stops and triggers maybe_exit_scc() w/o accompanying maybe_enter_scc() call.
//
    SEC("socket")
    __arch_x86_64
    __caps_unpriv(CAP_BPF)
#[no_mangle]
pub unsafe extern "C" fn maybe_exit_scc_bug1() -> __naked void {
    __naked void maybe_exit_scc_bug1(void)
    {
    asm volatile (
    "r0 = 100;"
    "1:"
// Speculative execution path reaches and stops here.
    "*(u64 *)(r10 - 512) = r0;"
// Condition is always false, but verifier speculatively executes the true branch.
    "if r0 <= 0x0 goto 1b;"
    "exit;"
    ::: __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
