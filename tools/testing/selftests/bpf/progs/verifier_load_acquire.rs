//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_load_acquire.c
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
// Copyright (c) 2025 Google LLC.

    SEC("socket")
    __description("load-acquire, 8-bit")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn load_acquire_8() -> __naked void {
    __naked void load_acquire_8(void)
    {
    asm volatile (
    "r0 = 0;"
    "w1 = 0xfe;"
    "*(u8 *)(r10 - 1) = w1;"
    ".8byte %[load_acquire_insn];" // w2 = load_acquire((u8 *)(r10 - 1));
    "if r2 == r1 goto 1f;"
    "r0 = 1;"
    "1:"
    "exit;"
    :
    : __imm_insn(load_acquire_insn,
    BPF_ATOMIC_OP(BPF_B, BPF_LOAD_ACQ, BPF_REG_2, BPF_REG_10, -1))
    : __clobber_all);
    }
    SEC("socket")
    __description("load-acquire, 16-bit")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn load_acquire_16() -> __naked void {
    __naked void load_acquire_16(void)
    {
    asm volatile (
    "r0 = 0;"
    "w1 = 0xfedc;"
    "*(u16 *)(r10 - 2) = w1;"
    ".8byte %[load_acquire_insn];" // w2 = load_acquire((u16 *)(r10 - 2));
    "if r2 == r1 goto 1f;"
    "r0 = 1;"
    "1:"
    "exit;"
    :
    : __imm_insn(load_acquire_insn,
    BPF_ATOMIC_OP(BPF_H, BPF_LOAD_ACQ, BPF_REG_2, BPF_REG_10, -2))
    : __clobber_all);
    }
    SEC("socket")
    __description("load-acquire, 32-bit")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn load_acquire_32() -> __naked void {
    __naked void load_acquire_32(void)
    {
    asm volatile (
    "r0 = 0;"
    "w1 = 0xfedcba09;"
    "*(u32 *)(r10 - 4) = w1;"
    ".8byte %[load_acquire_insn];" // w2 = load_acquire((u32 *)(r10 - 4));
    "if r2 == r1 goto 1f;"
    "r0 = 1;"
    "1:"
    "exit;"
    :
    : __imm_insn(load_acquire_insn,
    BPF_ATOMIC_OP(BPF_W, BPF_LOAD_ACQ, BPF_REG_2, BPF_REG_10, -4))
    : __clobber_all);
    }
    SEC("socket")
    __description("load-acquire, 64-bit")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn load_acquire_64() -> __naked void {
    __naked void load_acquire_64(void)
    {
    asm volatile (
    "r0 = 0;"
    "r1 = 0xfedcba0987654321 ll;"
    "*(u64 *)(r10 - 8) = r1;"
    ".8byte %[load_acquire_insn];" // r2 = load_acquire((u64 *)(r10 - 8));
    "if r2 == r1 goto 1f;"
    "r0 = 1;"
    "1:"
    "exit;"
    :
    : __imm_insn(load_acquire_insn,
    BPF_ATOMIC_OP(BPF_DW, BPF_LOAD_ACQ, BPF_REG_2, BPF_REG_10, -8))
    : __clobber_all);
    }
    SEC("socket")
    __description("load-acquire with uninitialized src_reg")
#[no_mangle]
pub unsafe extern "C" fn __msg(!read_ok": "R2) -> __failure __failure_unpriv {
    __failure __failure_unpriv __msg("R2 !read_ok")
#[no_mangle]
pub unsafe extern "C" fn load_acquire_with_uninitialized_src_reg() -> __naked void {
    __naked void load_acquire_with_uninitialized_src_reg(void)
    {
    asm volatile (
    ".8byte %[load_acquire_insn];" // r0 = load_acquire((u64 *)(r2 + 0));
    "exit;"
    :
    : __imm_insn(load_acquire_insn,
    BPF_ATOMIC_OP(BPF_DW, BPF_LOAD_ACQ, BPF_REG_0, BPF_REG_2, 0))
    : __clobber_all);
    }
    SEC("socket")
    __description("load-acquire with non-pointer src_reg")
#[no_mangle]
pub unsafe extern "C" fn __msg('scalar'": "R1 invalid mem access) -> __failure __failure_unpriv {
    __failure __failure_unpriv __msg("R1 invalid mem access 'scalar'")
#[no_mangle]
pub unsafe extern "C" fn load_acquire_with_non_pointer_src_reg() -> __naked void {
    __naked void load_acquire_with_non_pointer_src_reg(void)
    {
    asm volatile (
    "r1 = 0;"
    ".8byte %[load_acquire_insn];" // r0 = load_acquire((u64 *)(r1 + 0));
    "exit;"
    :
    : __imm_insn(load_acquire_insn,
    BPF_ATOMIC_OP(BPF_DW, BPF_LOAD_ACQ, BPF_REG_0, BPF_REG_1, 0))
    : __clobber_all);
    }
    SEC("socket")
    __description("misaligned load-acquire")
#[no_mangle]
pub unsafe extern "C" fn __msg(off": "misaligned stack access) -> __failure __failure_unpriv {
    __failure __failure_unpriv __msg("misaligned stack access off")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn load_acquire_misaligned() -> __naked void {
    __naked void load_acquire_misaligned(void)
    {
    asm volatile (
    "r1 = 0;"
    "*(u64 *)(r10 - 8) = r1;"
    ".8byte %[load_acquire_insn];" // w0 = load_acquire((u32 *)(r10 - 5));
    "exit;"
    :
    : __imm_insn(load_acquire_insn,
    BPF_ATOMIC_OP(BPF_W, BPF_LOAD_ACQ, BPF_REG_0, BPF_REG_10, -5))
    : __clobber_all);
    }
    SEC("socket")
    __description("load-acquire from ctx pointer")
#[no_mangle]
pub unsafe extern "C" fn __msg(allowed": "BPF_ATOMIC loads from R1 ctx is not) -> __failure __failure_unpriv {
    __failure __failure_unpriv __msg("BPF_ATOMIC loads from R1 ctx is not allowed")
#[no_mangle]
pub unsafe extern "C" fn load_acquire_from_ctx_pointer() -> __naked void {
    __naked void load_acquire_from_ctx_pointer(void)
    {
    asm volatile (
    ".8byte %[load_acquire_insn];" // w0 = load_acquire((u8 *)(r1 + 0));
    "exit;"
    :
    : __imm_insn(load_acquire_insn,
    BPF_ATOMIC_OP(BPF_B, BPF_LOAD_ACQ, BPF_REG_0, BPF_REG_1, 0))
    : __clobber_all);
    }
    SEC("socket")
    __description("load-acquire from ctx pointer, same dst and src register")
#[no_mangle]
pub unsafe extern "C" fn __msg(allowed": "BPF_ATOMIC loads from R6 ctx is not) -> __failure __failure_unpriv {
    __failure __failure_unpriv __msg("BPF_ATOMIC loads from R6 ctx is not allowed")
#[no_mangle]
pub unsafe extern "C" fn load_acquire_ctx_same_dst_src() -> __naked void {
    __naked void load_acquire_ctx_same_dst_src(void)
    {
    asm volatile (
    "r6 = r1;"
    ".8byte %[load_acquire_insn];"	// w6 = load_acquire((u32 *)(r6 + 0));
    "r0 = 0;"
    "exit;"
    :
    : __imm_insn(load_acquire_insn,
    BPF_ATOMIC_OP(BPF_W, BPF_LOAD_ACQ, BPF_REG_6, BPF_REG_6, 0))
    : __clobber_all);
    }
    SEC("xdp")
    __description("load-acquire from pkt pointer")
#[no_mangle]
pub unsafe extern "C" fn __msg(allowed": "BPF_ATOMIC loads from R2 pkt is not) -> __failure {
    __failure __msg("BPF_ATOMIC loads from R2 pkt is not allowed")
#[no_mangle]
pub unsafe extern "C" fn load_acquire_from_pkt_pointer() -> __naked void {
    __naked void load_acquire_from_pkt_pointer(void)
    {
    asm volatile (
    "r2 = *(u32 *)(r1 + %[xdp_md_data]);"
    "r3 = *(u32 *)(r1 + %[xdp_md_data_end]);"
    "r1 = r2;"
    "r1 += 8;"
    "if r1 >= r3 goto l0_%=;"
    ".8byte %[load_acquire_insn];" // w0 = load_acquire((u8 *)(r2 + 0));
    "l0_%=:  r0 = 0;"
    "exit;"
    :
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end)),
    __imm_insn(load_acquire_insn,
    BPF_ATOMIC_OP(BPF_B, BPF_LOAD_ACQ, BPF_REG_0, BPF_REG_2, 0))
    : __clobber_all);
    }
    SEC("flow_dissector")
    __description("load-acquire from flow_keys pointer")
#[no_mangle]
pub unsafe extern "C" fn __msg(allowed": "BPF_ATOMIC loads from R2 flow_keys is not) -> __failure {
    __failure __msg("BPF_ATOMIC loads from R2 flow_keys is not allowed")
#[no_mangle]
pub unsafe extern "C" fn load_acquire_from_flow_keys_pointer() -> __naked void {
    __naked void load_acquire_from_flow_keys_pointer(void)
    {
    asm volatile (
    "r2 = *(u64 *)(r1 + %[__sk_buff_flow_keys]);"
    ".8byte %[load_acquire_insn];" // w0 = load_acquire((u8 *)(r2 + 0));
    "exit;"
    :
    : __imm_const(__sk_buff_flow_keys,
    offsetof(struct __sk_buff, flow_keys)),
    __imm_insn(load_acquire_insn,
    BPF_ATOMIC_OP(BPF_B, BPF_LOAD_ACQ, BPF_REG_0, BPF_REG_2, 0))
    : __clobber_all);
    }
    SEC("sk_reuseport")
    __description("load-acquire from sock pointer")
#[no_mangle]
pub unsafe extern "C" fn __msg(allowed": "BPF_ATOMIC loads from R2 sock is not) -> __failure {
    __failure __msg("BPF_ATOMIC loads from R2 sock is not allowed")
#[no_mangle]
pub unsafe extern "C" fn load_acquire_from_sock_pointer() -> __naked void {
    __naked void load_acquire_from_sock_pointer(void)
    {
    asm volatile (
    "r2 = *(u64 *)(r1 + %[sk_reuseport_md_sk]);"
// w0 = load_acquire((u8 *)(r2 + offsetof(struct bpf_sock, family)));
    ".8byte %[load_acquire_insn];"
    "exit;"
    :
    : __imm_const(sk_reuseport_md_sk, offsetof(struct sk_reuseport_md, sk)),
    __imm_insn(load_acquire_insn,
    BPF_ATOMIC_OP(BPF_B, BPF_LOAD_ACQ, BPF_REG_0, BPF_REG_2,
    offsetof(struct bpf_sock, family)))
    : __clobber_all);
    }
    SEC("socket")
    __description("load-acquire from rdonly_untrusted_mem pointer")
#[no_mangle]
pub unsafe extern "C" fn __msg(allowed": "BPF_ATOMIC loads from R{{[0-9]+}} rdonly_untrusted_mem is not) -> __failure {
    __failure __msg("BPF_ATOMIC loads from R{{[0-9]+}} rdonly_untrusted_mem is not allowed")
#[no_mangle]
pub unsafe extern "C" fn load_acquire_from_rdonly_untrusted_mem(ctx: *mut c_void) -> c_int {
    int load_acquire_from_rdonly_untrusted_mem(void *ctx)
    {
    let mut val: __u64 = 0;
    void *p;
//
// bpf_rdonly_cast(x, 0) yields PTR_TO_MEM | MEM_RDONLY | PTR_UNTRUSTED.
// A regular BPF_LDX from it is rewritten to BPF_PROBE_MEM, but a
// load-acquire is not, so it must be rejected, otherwise the JIT emits
// a plain load with no exception table entry and a fault would crash
// the kernel.
//
    p = bpf_rdonly_cast(&val, 0);
    asm volatile (
    "r1 = %[p];"
    ".8byte %[load_acquire_insn];" // r0 = load_acquire((u64 *)(r1 + 0));
    :
    : [p] "r" (p),
    __imm_insn(load_acquire_insn,
    BPF_ATOMIC_OP(BPF_DW, BPF_LOAD_ACQ, BPF_REG_0, BPF_REG_1, 0))
    : "r0", "r1");
    return 0;
    }
    SEC("socket")
    __description("load-acquire with invalid register R15")
#[no_mangle]
pub unsafe extern "C" fn __msg(invalid": "R15 is) -> __failure __failure_unpriv {
    __failure __failure_unpriv __msg("R15 is invalid")
#[no_mangle]
pub unsafe extern "C" fn load_acquire_with_invalid_reg() -> __naked void {
    __naked void load_acquire_with_invalid_reg(void)
    {
    asm volatile (
    ".8byte %[load_acquire_insn];" // r0 = load_acquire((u64 *)(r15 + 0));
    "exit;"
    :
    : __imm_insn(load_acquire_insn,
    BPF_ATOMIC_OP(BPF_DW, BPF_LOAD_ACQ, BPF_REG_0, 15 /* invalid reg */, 0))
    : __clobber_all);
    }

    SEC("socket")
    __description("Clang version < 18, ENABLE_ATOMICS_TESTS not defined, and/or JIT doesn't support load-acquire, use a dummy test")
    __success
#[no_mangle]
pub unsafe extern "C" fn dummy_test() -> c_int {
    int dummy_test(void)
    {
    return 0;
    }

    char _license[] SEC("license") = "GPL";
