//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_netfilter_ctx.c
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

    SEC("netfilter")
    __description("netfilter invalid context access, size too short")
#[no_mangle]
pub unsafe extern "C" fn __msg(access": "invalid bpf_context) -> __failure {
    __failure __msg("invalid bpf_context access")
#[no_mangle]
pub unsafe extern "C" fn with_invalid_ctx_access_test1() -> __naked void {
    __naked void with_invalid_ctx_access_test1(void)
    {
    asm volatile ("					\
    r2 = *(u8*)(r1 + %[__bpf_nf_ctx_state]);	\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(__bpf_nf_ctx_state, offsetof(struct bpf_nf_ctx, state))
    : __clobber_all);
    }
    SEC("netfilter")
    __description("netfilter invalid context access, size too short")
#[no_mangle]
pub unsafe extern "C" fn __msg(access": "invalid bpf_context) -> __failure {
    __failure __msg("invalid bpf_context access")
#[no_mangle]
pub unsafe extern "C" fn with_invalid_ctx_access_test2() -> __naked void {
    __naked void with_invalid_ctx_access_test2(void)
    {
    asm volatile ("					\
    r2 = *(u16*)(r1 + %[__bpf_nf_ctx_skb]);	\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(__bpf_nf_ctx_skb, offsetof(struct bpf_nf_ctx, skb))
    : __clobber_all);
    }
    SEC("netfilter")
    __description("netfilter invalid context access, past end of ctx")
#[no_mangle]
pub unsafe extern "C" fn __msg(access": "invalid bpf_context) -> __failure {
    __failure __msg("invalid bpf_context access")
#[no_mangle]
pub unsafe extern "C" fn with_invalid_ctx_access_test3() -> __naked void {
    __naked void with_invalid_ctx_access_test3(void)
    {
    asm volatile ("					\
    r2 = *(u64*)(r1 + %[__bpf_nf_ctx_size]);	\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(__bpf_nf_ctx_size, sizeof(struct bpf_nf_ctx))
    : __clobber_all);
    }
    SEC("netfilter")
    __description("netfilter invalid context, write")
#[no_mangle]
pub unsafe extern "C" fn __msg(access": "invalid bpf_context) -> __failure {
    __failure __msg("invalid bpf_context access")
#[no_mangle]
pub unsafe extern "C" fn with_invalid_ctx_access_test4() -> __naked void {
    __naked void with_invalid_ctx_access_test4(void)
    {
    asm volatile ("					\
    r2 = r1;					\
// (u64*)(r2 + 0) = r1;				\
    r0 = 1;						\
    exit;						\
    "	:
    : __imm_const(__bpf_nf_ctx_skb, offsetof(struct bpf_nf_ctx, skb))
    : __clobber_all);
    }
pub const NF_DROP: c_int = 0;
pub const NF_ACCEPT: c_int = 1;
    SEC("netfilter")
    __description("netfilter valid context read and invalid write")
#[no_mangle]
pub unsafe extern "C" fn __msg(supported": "only read is) -> __failure {
    __failure __msg("only read is supported")
#[no_mangle]
pub unsafe extern "C" fn with_invalid_ctx_access_test5(ctx: *mut bpf_nf_ctx) -> c_int {
    int with_invalid_ctx_access_test5(struct bpf_nf_ctx *ctx)
    {
    struct nf_hook_state *state = (void *)ctx.state;
    state.sk = core::ptr::null_mut();
    return NF_ACCEPT;
    }
    SEC("netfilter")
    __description("netfilter test prog with skb and state read access")
    __success __failure_unpriv
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn with_valid_ctx_access_test6(ctx: *mut bpf_nf_ctx) -> c_int {
    int with_valid_ctx_access_test6(struct bpf_nf_ctx *ctx)
    {
    struct __sk_buff *skb = (struct __sk_buff *)ctx.skb;
    const struct nf_hook_state *state = ctx.state;
    const struct iphdr *iph;
    const struct tcphdr *th;
    u8 buffer_iph[20] = {};
    u8 buffer_th[40] = {};
    struct bpf_dynptr ptr;
    uint8_t ihl;
    if (ctx.skb.len <= 20 || bpf_dynptr_from_skb(skb, 0, &ptr))
    return NF_ACCEPT;
    iph = bpf_dynptr_slice(&ptr, 0, buffer_iph, sizeof(buffer_iph));
    if (!iph)
    return NF_ACCEPT;
    if (state.pf != 2)
    return NF_ACCEPT;
    ihl = iph.ihl << 2;
    th = bpf_dynptr_slice(&ptr, ihl, buffer_th, sizeof(buffer_th));
    if (!th)
    return NF_ACCEPT;
    return th.dest == bpf_htons(22) ? NF_ACCEPT : NF_DROP;
    }
    char _license[] SEC("license") = "GPL";
