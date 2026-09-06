//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_raw_stack.c
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
// Converted from tools/testing/selftests/bpf/verifier/raw_stack.c

    SEC("socket")
    __description("raw_stack: no skb_load_bytes")
    __success
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(size=8": "invalid read from stack R6 off=-8) -> __failure_unpriv {
    __failure_unpriv __msg_unpriv("invalid read from stack R6 off=-8 size=8")
#[no_mangle]
pub unsafe extern "C" fn stack_no_skb_load_bytes() -> __naked void {
    __naked void stack_no_skb_load_bytes(void)
    {
    asm volatile ("					\
    r2 = 4;						\
    r6 = r10;					\
    r6 += -8;					\
    r3 = r6;					\
    r4 = 8;						\
// Call to skb_load_bytes() omitted. */		\
    r0 = *(u64*)(r6 + 0);				\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("tc")
    __description("raw_stack: skb_load_bytes, negative len")
#[no_mangle]
pub unsafe extern "C" fn __msg(negative": "R4 min value is) -> __failure {
    __failure __msg("R4 min value is negative")
#[no_mangle]
pub unsafe extern "C" fn skb_load_bytes_negative_len() -> __naked void {
    __naked void skb_load_bytes_negative_len(void)
    {
    asm volatile ("					\
    r2 = 4;						\
    r6 = r10;					\
    r6 += -8;					\
    r3 = r6;					\
    r4 = -8;					\
    call %[bpf_skb_load_bytes];			\
    r0 = *(u64*)(r6 + 0);				\
    exit;						\
    "	:
    : __imm(bpf_skb_load_bytes)
    : __clobber_all);
    }
    SEC("tc")
    __description("raw_stack: skb_load_bytes, negative len 2")
#[no_mangle]
pub unsafe extern "C" fn __msg(negative": "R4 min value is) -> __failure {
    __failure __msg("R4 min value is negative")
#[no_mangle]
pub unsafe extern "C" fn load_bytes_negative_len_2() -> __naked void {
    __naked void load_bytes_negative_len_2(void)
    {
    asm volatile ("					\
    r2 = 4;						\
    r6 = r10;					\
    r6 += -8;					\
    r3 = r6;					\
    r4 = %[__imm_0];				\
    call %[bpf_skb_load_bytes];			\
    r0 = *(u64*)(r6 + 0);				\
    exit;						\
    "	:
    : __imm(bpf_skb_load_bytes),
    __imm_const(__imm_0, ~0)
    : __clobber_all);
    }
    SEC("tc")
    __description("raw_stack: skb_load_bytes, zero len")
#[no_mangle]
pub unsafe extern "C" fn __msg(u64=[0: "R4 invalid zero-sized read:, _arg: 0]") -> __failure {
    __failure __msg("R4 invalid zero-sized read: u64=[0,0]")
#[no_mangle]
pub unsafe extern "C" fn skb_load_bytes_zero_len() -> __naked void {
    __naked void skb_load_bytes_zero_len(void)
    {
    asm volatile ("					\
    r2 = 4;						\
    r6 = r10;					\
    r6 += -8;					\
    r3 = r6;					\
    r4 = 0;						\
    call %[bpf_skb_load_bytes];			\
    r0 = *(u64*)(r6 + 0);				\
    exit;						\
    "	:
    : __imm(bpf_skb_load_bytes)
    : __clobber_all);
    }
    SEC("tc")
    __description("raw_stack: skb_load_bytes, no init")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn skb_load_bytes_no_init() -> __naked void {
    __naked void skb_load_bytes_no_init(void)
    {
    asm volatile ("					\
    r2 = 4;						\
    r6 = r10;					\
    r6 += -8;					\
    r3 = r6;					\
    r4 = 8;						\
    call %[bpf_skb_load_bytes];			\
    r0 = *(u64*)(r6 + 0);				\
    exit;						\
    "	:
    : __imm(bpf_skb_load_bytes)
    : __clobber_all);
    }
    SEC("tc")
    __description("raw_stack: skb_load_bytes, init")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn stack_skb_load_bytes_init() -> __naked void {
    __naked void stack_skb_load_bytes_init(void)
    {
    asm volatile ("					\
    r2 = 4;						\
    r6 = r10;					\
    r6 += -8;					\
    r3 = 0xcafe;					\
// (u64*)(r6 + 0) = r3;				\
    r3 = r6;					\
    r4 = 8;						\
    call %[bpf_skb_load_bytes];			\
    r0 = *(u64*)(r6 + 0);				\
    exit;						\
    "	:
    : __imm(bpf_skb_load_bytes)
    : __clobber_all);
    }
    SEC("tc")
    __description("raw_stack: skb_load_bytes, spilled regs around bounds")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn bytes_spilled_regs_around_bounds() -> __naked void {
    __naked void bytes_spilled_regs_around_bounds(void)
    {
    asm volatile ("					\
    r2 = 4;						\
    r6 = r10;					\
    r6 += -16;					\
// (u64*)(r6 - 8) = r1;				\
// (u64*)(r6 + 8) = r1;				\
    r3 = r6;					\
    r4 = 8;						\
    call %[bpf_skb_load_bytes];			\
    r0 = *(u64*)(r6 - 8);				\
    r2 = *(u64*)(r6 + 8);				\
    r0 = *(u32*)(r0 + %[__sk_buff_mark]);		\
    r2 = *(u32*)(r2 + %[__sk_buff_priority]);	\
    r0 += r2;					\
    exit;						\
    "	:
    : __imm(bpf_skb_load_bytes),
    __imm_const(__sk_buff_mark, offsetof(struct __sk_buff, mark)),
    __imm_const(__sk_buff_priority, offsetof(struct __sk_buff, priority))
    : __clobber_all);
    }
    SEC("tc")
    __description("raw_stack: skb_load_bytes, spilled regs corruption")
#[no_mangle]
pub unsafe extern "C" fn __msg('scalar'": "R0 invalid mem access) -> __failure {
    __failure __msg("R0 invalid mem access 'scalar'")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn load_bytes_spilled_regs_corruption() -> __naked void {
    __naked void load_bytes_spilled_regs_corruption(void)
    {
    asm volatile ("					\
    r2 = 4;						\
    r6 = r10;					\
    r6 += -8;					\
// (u64*)(r6 + 0) = r1;				\
    r3 = r6;					\
    r4 = 8;						\
    call %[bpf_skb_load_bytes];			\
    r0 = *(u64*)(r6 + 0);				\
    r0 = *(u32*)(r0 + %[__sk_buff_mark]);		\
    exit;						\
    "	:
    : __imm(bpf_skb_load_bytes),
    __imm_const(__sk_buff_mark, offsetof(struct __sk_buff, mark))
    : __clobber_all);
    }
    SEC("tc")
    __description("raw_stack: skb_load_bytes, spilled regs corruption 2")
#[no_mangle]
pub unsafe extern "C" fn __msg('scalar'": "R3 invalid mem access) -> __failure {
    __failure __msg("R3 invalid mem access 'scalar'")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn bytes_spilled_regs_corruption_2() -> __naked void {
    __naked void bytes_spilled_regs_corruption_2(void)
    {
    asm volatile ("					\
    r2 = 4;						\
    r6 = r10;					\
    r6 += -16;					\
// (u64*)(r6 - 8) = r1;				\
// (u64*)(r6 + 0) = r1;				\
// (u64*)(r6 + 8) = r1;				\
    r3 = r6;					\
    r4 = 8;						\
    call %[bpf_skb_load_bytes];			\
    r0 = *(u64*)(r6 - 8);				\
    r2 = *(u64*)(r6 + 8);				\
    r3 = *(u64*)(r6 + 0);				\
    r0 = *(u32*)(r0 + %[__sk_buff_mark]);		\
    r2 = *(u32*)(r2 + %[__sk_buff_priority]);	\
    r0 += r2;					\
    r3 = *(u32*)(r3 + %[__sk_buff_pkt_type]);	\
    r0 += r3;					\
    exit;						\
    "	:
    : __imm(bpf_skb_load_bytes),
    __imm_const(__sk_buff_mark, offsetof(struct __sk_buff, mark)),
    __imm_const(__sk_buff_pkt_type, offsetof(struct __sk_buff, pkt_type)),
    __imm_const(__sk_buff_priority, offsetof(struct __sk_buff, priority))
    : __clobber_all);
    }
    SEC("tc")
    __description("raw_stack: skb_load_bytes, spilled regs + data")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn load_bytes_spilled_regs_data() -> __naked void {
    __naked void load_bytes_spilled_regs_data(void)
    {
    asm volatile ("					\
    r2 = 4;						\
    r6 = r10;					\
    r6 += -16;					\
// (u64*)(r6 - 8) = r1;				\
// (u64*)(r6 + 0) = r1;				\
// (u64*)(r6 + 8) = r1;				\
    r3 = r6;					\
    r4 = 8;						\
    call %[bpf_skb_load_bytes];			\
    r0 = *(u64*)(r6 - 8);				\
    r2 = *(u64*)(r6 + 8);				\
    r3 = *(u64*)(r6 + 0);				\
    r0 = *(u32*)(r0 + %[__sk_buff_mark]);		\
    r2 = *(u32*)(r2 + %[__sk_buff_priority]);	\
    r0 += r2;					\
    r0 += r3;					\
    exit;						\
    "	:
    : __imm(bpf_skb_load_bytes),
    __imm_const(__sk_buff_mark, offsetof(struct __sk_buff, mark)),
    __imm_const(__sk_buff_priority, offsetof(struct __sk_buff, priority))
    : __clobber_all);
    }
    SEC("tc")
    __description("raw_stack: skb_load_bytes, invalid access 1")
#[no_mangle]
pub unsafe extern "C" fn __msg(size=8": "invalid write to stack R3 off=-513) -> __failure {
    __failure __msg("invalid write to stack R3 off=-513 size=8")
#[no_mangle]
pub unsafe extern "C" fn load_bytes_invalid_access_1() -> __naked void {
    __naked void load_bytes_invalid_access_1(void)
    {
    asm volatile ("					\
    r2 = 4;						\
    r6 = r10;					\
    r6 += -513;					\
    r3 = r6;					\
    r4 = 8;						\
    call %[bpf_skb_load_bytes];			\
    r0 = *(u64*)(r6 + 0);				\
    exit;						\
    "	:
    : __imm(bpf_skb_load_bytes)
    : __clobber_all);
    }
    SEC("tc")
    __description("raw_stack: skb_load_bytes, invalid access 2")
#[no_mangle]
pub unsafe extern "C" fn __msg(size=8": "invalid write to stack R3 off=-1) -> __failure {
    __failure __msg("invalid write to stack R3 off=-1 size=8")
#[no_mangle]
pub unsafe extern "C" fn load_bytes_invalid_access_2() -> __naked void {
    __naked void load_bytes_invalid_access_2(void)
    {
    asm volatile ("					\
    r2 = 4;						\
    r6 = r10;					\
    r6 += -1;					\
    r3 = r6;					\
    r4 = 8;						\
    call %[bpf_skb_load_bytes];			\
    r0 = *(u64*)(r6 + 0);				\
    exit;						\
    "	:
    : __imm(bpf_skb_load_bytes)
    : __clobber_all);
    }
    SEC("tc")
    __description("raw_stack: skb_load_bytes, invalid access 3")
#[no_mangle]
pub unsafe extern "C" fn __msg(negative": "R4 min value is) -> __failure {
    __failure __msg("R4 min value is negative")
#[no_mangle]
pub unsafe extern "C" fn load_bytes_invalid_access_3() -> __naked void {
    __naked void load_bytes_invalid_access_3(void)
    {
    asm volatile ("					\
    r2 = 4;						\
    r6 = r10;					\
    r6 += 0xffffffff;				\
    r3 = r6;					\
    r4 = 0xffffffff;				\
    call %[bpf_skb_load_bytes];			\
    r0 = *(u64*)(r6 + 0);				\
    exit;						\
    "	:
    : __imm(bpf_skb_load_bytes)
    : __clobber_all);
    }
    SEC("tc")
    __description("raw_stack: skb_load_bytes, invalid access 4")
    __failure
    __msg("R4 unbounded memory access, use 'var &= const' or 'if (var < const)'")
#[no_mangle]
pub unsafe extern "C" fn load_bytes_invalid_access_4() -> __naked void {
    __naked void load_bytes_invalid_access_4(void)
    {
    asm volatile ("					\
    r2 = 4;						\
    r6 = r10;					\
    r6 += -1;					\
    r3 = r6;					\
    r4 = 0x7fffffff;				\
    call %[bpf_skb_load_bytes];			\
    r0 = *(u64*)(r6 + 0);				\
    exit;						\
    "	:
    : __imm(bpf_skb_load_bytes)
    : __clobber_all);
    }
    SEC("tc")
    __description("raw_stack: skb_load_bytes, invalid access 5")
    __failure
    __msg("R4 unbounded memory access, use 'var &= const' or 'if (var < const)'")
#[no_mangle]
pub unsafe extern "C" fn load_bytes_invalid_access_5() -> __naked void {
    __naked void load_bytes_invalid_access_5(void)
    {
    asm volatile ("					\
    r2 = 4;						\
    r6 = r10;					\
    r6 += -512;					\
    r3 = r6;					\
    r4 = 0x7fffffff;				\
    call %[bpf_skb_load_bytes];			\
    r0 = *(u64*)(r6 + 0);				\
    exit;						\
    "	:
    : __imm(bpf_skb_load_bytes)
    : __clobber_all);
    }
    SEC("tc")
    __description("raw_stack: skb_load_bytes, invalid access 6")
#[no_mangle]
pub unsafe extern "C" fn __msg(read": "invalid zero-sized) -> __failure {
    __failure __msg("invalid zero-sized read")
#[no_mangle]
pub unsafe extern "C" fn load_bytes_invalid_access_6() -> __naked void {
    __naked void load_bytes_invalid_access_6(void)
    {
    asm volatile ("					\
    r2 = 4;						\
    r6 = r10;					\
    r6 += -512;					\
    r3 = r6;					\
    r4 = 0;						\
    call %[bpf_skb_load_bytes];			\
    r0 = *(u64*)(r6 + 0);				\
    exit;						\
    "	:
    : __imm(bpf_skb_load_bytes)
    : __clobber_all);
    }
    SEC("tc")
    __description("raw_stack: skb_load_bytes, large access")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn skb_load_bytes_large_access() -> __naked void {
    __naked void skb_load_bytes_large_access(void)
    {
    asm volatile ("					\
    r2 = 4;						\
    r6 = r10;					\
    r6 += -512;					\
    r3 = r6;					\
    r4 = 512;					\
    call %[bpf_skb_load_bytes];			\
    r0 = *(u64*)(r6 + 0);				\
    exit;						\
    "	:
    : __imm(bpf_skb_load_bytes)
    : __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
