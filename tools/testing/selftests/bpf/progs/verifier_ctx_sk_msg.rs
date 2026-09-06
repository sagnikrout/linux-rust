//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_ctx_sk_msg.c
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
// Converted from tools/testing/selftests/bpf/verifier/ctx_sk_msg.c

    SEC("sk_msg")
    __description("valid access family in SK_MSG")
    __success
#[no_mangle]
pub unsafe extern "C" fn access_family_in_sk_msg() -> __naked void {
    __naked void access_family_in_sk_msg(void)
    {
    asm volatile ("					\
    r0 = *(u32*)(r1 + %[sk_msg_md_family]);		\
    exit;						\
    "	:
    : __imm_const(sk_msg_md_family, offsetof(struct sk_msg_md, family))
    : __clobber_all);
    }
    SEC("sk_msg")
    __description("valid access remote_ip4 in SK_MSG")
    __success
#[no_mangle]
pub unsafe extern "C" fn remote_ip4_in_sk_msg() -> __naked void {
    __naked void remote_ip4_in_sk_msg(void)
    {
    asm volatile ("					\
    r0 = *(u32*)(r1 + %[sk_msg_md_remote_ip4]);	\
    exit;						\
    "	:
    : __imm_const(sk_msg_md_remote_ip4, offsetof(struct sk_msg_md, remote_ip4))
    : __clobber_all);
    }
    SEC("sk_msg")
    __description("valid access local_ip4 in SK_MSG")
    __success
#[no_mangle]
pub unsafe extern "C" fn local_ip4_in_sk_msg() -> __naked void {
    __naked void local_ip4_in_sk_msg(void)
    {
    asm volatile ("					\
    r0 = *(u32*)(r1 + %[sk_msg_md_local_ip4]);	\
    exit;						\
    "	:
    : __imm_const(sk_msg_md_local_ip4, offsetof(struct sk_msg_md, local_ip4))
    : __clobber_all);
    }
    SEC("sk_msg")
    __description("valid access remote_port in SK_MSG")
    __success
#[no_mangle]
pub unsafe extern "C" fn remote_port_in_sk_msg() -> __naked void {
    __naked void remote_port_in_sk_msg(void)
    {
    asm volatile ("					\
    r0 = *(u32*)(r1 + %[sk_msg_md_remote_port]);	\
    exit;						\
    "	:
    : __imm_const(sk_msg_md_remote_port, offsetof(struct sk_msg_md, remote_port))
    : __clobber_all);
    }
    SEC("sk_msg")
    __description("valid access local_port in SK_MSG")
    __success
#[no_mangle]
pub unsafe extern "C" fn local_port_in_sk_msg() -> __naked void {
    __naked void local_port_in_sk_msg(void)
    {
    asm volatile ("					\
    r0 = *(u32*)(r1 + %[sk_msg_md_local_port]);	\
    exit;						\
    "	:
    : __imm_const(sk_msg_md_local_port, offsetof(struct sk_msg_md, local_port))
    : __clobber_all);
    }
    SEC("sk_skb")
    __description("valid access remote_ip6 in SK_MSG")
    __success
#[no_mangle]
pub unsafe extern "C" fn remote_ip6_in_sk_msg() -> __naked void {
    __naked void remote_ip6_in_sk_msg(void)
    {
    asm volatile ("					\
    r0 = *(u32*)(r1 + %[sk_msg_md_remote_ip6_0]);	\
    r0 = *(u32*)(r1 + %[sk_msg_md_remote_ip6_1]);	\
    r0 = *(u32*)(r1 + %[sk_msg_md_remote_ip6_2]);	\
    r0 = *(u32*)(r1 + %[sk_msg_md_remote_ip6_3]);	\
    exit;						\
    "	:
    : __imm_const(sk_msg_md_remote_ip6_0, offsetof(struct sk_msg_md, remote_ip6[0])),
    __imm_const(sk_msg_md_remote_ip6_1, offsetof(struct sk_msg_md, remote_ip6[1])),
    __imm_const(sk_msg_md_remote_ip6_2, offsetof(struct sk_msg_md, remote_ip6[2])),
    __imm_const(sk_msg_md_remote_ip6_3, offsetof(struct sk_msg_md, remote_ip6[3]))
    : __clobber_all);
    }
    SEC("sk_skb")
    __description("valid access local_ip6 in SK_MSG")
    __success
#[no_mangle]
pub unsafe extern "C" fn local_ip6_in_sk_msg() -> __naked void {
    __naked void local_ip6_in_sk_msg(void)
    {
    asm volatile ("					\
    r0 = *(u32*)(r1 + %[sk_msg_md_local_ip6_0]);	\
    r0 = *(u32*)(r1 + %[sk_msg_md_local_ip6_1]);	\
    r0 = *(u32*)(r1 + %[sk_msg_md_local_ip6_2]);	\
    r0 = *(u32*)(r1 + %[sk_msg_md_local_ip6_3]);	\
    exit;						\
    "	:
    : __imm_const(sk_msg_md_local_ip6_0, offsetof(struct sk_msg_md, local_ip6[0])),
    __imm_const(sk_msg_md_local_ip6_1, offsetof(struct sk_msg_md, local_ip6[1])),
    __imm_const(sk_msg_md_local_ip6_2, offsetof(struct sk_msg_md, local_ip6[2])),
    __imm_const(sk_msg_md_local_ip6_3, offsetof(struct sk_msg_md, local_ip6[3]))
    : __clobber_all);
    }
    SEC("sk_msg")
    __description("valid access size in SK_MSG")
    __success
#[no_mangle]
pub unsafe extern "C" fn access_size_in_sk_msg() -> __naked void {
    __naked void access_size_in_sk_msg(void)
    {
    asm volatile ("					\
    r0 = *(u32*)(r1 + %[sk_msg_md_size]);		\
    exit;						\
    "	:
    : __imm_const(sk_msg_md_size, offsetof(struct sk_msg_md, size))
    : __clobber_all);
    }
    SEC("sk_msg")
    __description("invalid 64B read of size in SK_MSG")
#[no_mangle]
pub unsafe extern "C" fn __msg(access": "invalid bpf_context) -> __failure {
    __failure __msg("invalid bpf_context access")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn of_size_in_sk_msg() -> __naked void {
    __naked void of_size_in_sk_msg(void)
    {
    asm volatile ("					\
    r2 = *(u64*)(r1 + %[sk_msg_md_size]);		\
    exit;						\
    "	:
    : __imm_const(sk_msg_md_size, offsetof(struct sk_msg_md, size))
    : __clobber_all);
    }
    SEC("sk_msg")
    __description("invalid read past end of SK_MSG")
#[no_mangle]
pub unsafe extern "C" fn __msg(access": "invalid bpf_context) -> __failure {
    __failure __msg("invalid bpf_context access")
#[no_mangle]
pub unsafe extern "C" fn past_end_of_sk_msg() -> __naked void {
    __naked void past_end_of_sk_msg(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[__imm_0]);			\
    exit;						\
    "	:
    : __imm_const(__imm_0, offsetof(struct sk_msg_md, size) + 4)
    : __clobber_all);
    }
    SEC("sk_msg")
    __description("invalid read offset in SK_MSG")
#[no_mangle]
pub unsafe extern "C" fn __msg(access": "invalid bpf_context) -> __failure {
    __failure __msg("invalid bpf_context access")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn read_offset_in_sk_msg() -> __naked void {
    __naked void read_offset_in_sk_msg(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[__imm_0]);			\
    exit;						\
    "	:
    : __imm_const(__imm_0, offsetof(struct sk_msg_md, family) + 1)
    : __clobber_all);
    }
    SEC("sk_msg")
    __description("direct packet read for SK_MSG")
    __success
#[no_mangle]
pub unsafe extern "C" fn packet_read_for_sk_msg() -> __naked void {
    __naked void packet_read_for_sk_msg(void)
    {
    asm volatile ("					\
    r2 = *(u64*)(r1 + %[sk_msg_md_data]);		\
    r3 = *(u64*)(r1 + %[sk_msg_md_data_end]);	\
    r0 = r2;					\
    r0 += 8;					\
    if r0 > r3 goto l0_%=;				\
    r0 = *(u8*)(r2 + 0);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(sk_msg_md_data, offsetof(struct sk_msg_md, data)),
    __imm_const(sk_msg_md_data_end, offsetof(struct sk_msg_md, data_end))
    : __clobber_all);
    }
    SEC("sk_msg")
    __description("direct packet write for SK_MSG")
    __success
#[no_mangle]
pub unsafe extern "C" fn packet_write_for_sk_msg() -> __naked void {
    __naked void packet_write_for_sk_msg(void)
    {
    asm volatile ("					\
    r2 = *(u64*)(r1 + %[sk_msg_md_data]);		\
    r3 = *(u64*)(r1 + %[sk_msg_md_data_end]);	\
    r0 = r2;					\
    r0 += 8;					\
    if r0 > r3 goto l0_%=;				\
// (u8*)(r2 + 0) = r2;				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(sk_msg_md_data, offsetof(struct sk_msg_md, data)),
    __imm_const(sk_msg_md_data_end, offsetof(struct sk_msg_md, data_end))
    : __clobber_all);
    }
    SEC("sk_msg")
    __description("overlapping checks for direct packet access SK_MSG")
    __success
#[no_mangle]
pub unsafe extern "C" fn direct_packet_access_sk_msg() -> __naked void {
    __naked void direct_packet_access_sk_msg(void)
    {
    asm volatile ("					\
    r2 = *(u64*)(r1 + %[sk_msg_md_data]);		\
    r3 = *(u64*)(r1 + %[sk_msg_md_data_end]);	\
    r0 = r2;					\
    r0 += 8;					\
    if r0 > r3 goto l0_%=;				\
    r1 = r2;					\
    r1 += 6;					\
    if r1 > r3 goto l0_%=;				\
    r0 = *(u16*)(r2 + 6);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(sk_msg_md_data, offsetof(struct sk_msg_md, data)),
    __imm_const(sk_msg_md_data_end, offsetof(struct sk_msg_md, data_end))
    : __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
