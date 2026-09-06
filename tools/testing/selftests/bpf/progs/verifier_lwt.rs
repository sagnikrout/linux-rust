//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_lwt.c
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
// Converted from tools/testing/selftests/bpf/verifier/lwt.c

    SEC("lwt_in")
    __description("invalid direct packet write for LWT_IN")
#[no_mangle]
pub unsafe extern "C" fn __msg(packet": "cannot write into) -> __failure {
    __failure __msg("cannot write into packet")
#[no_mangle]
pub unsafe extern "C" fn packet_write_for_lwt_in() -> __naked void {
    __naked void packet_write_for_lwt_in(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[__sk_buff_data]);		\
    r3 = *(u32*)(r1 + %[__sk_buff_data_end]);	\
    r0 = r2;					\
    r0 += 8;					\
    if r0 > r3 goto l0_%=;				\
// (u8*)(r2 + 0) = r2;				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(__sk_buff_data, offsetof(struct __sk_buff, data)),
    __imm_const(__sk_buff_data_end, offsetof(struct __sk_buff, data_end))
    : __clobber_all);
    }
    SEC("lwt_out")
    __description("invalid direct packet write for LWT_OUT")
#[no_mangle]
pub unsafe extern "C" fn __msg(packet": "cannot write into) -> __failure {
    __failure __msg("cannot write into packet")
#[no_mangle]
pub unsafe extern "C" fn packet_write_for_lwt_out() -> __naked void {
    __naked void packet_write_for_lwt_out(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[__sk_buff_data]);		\
    r3 = *(u32*)(r1 + %[__sk_buff_data_end]);	\
    r0 = r2;					\
    r0 += 8;					\
    if r0 > r3 goto l0_%=;				\
// (u8*)(r2 + 0) = r2;				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(__sk_buff_data, offsetof(struct __sk_buff, data)),
    __imm_const(__sk_buff_data_end, offsetof(struct __sk_buff, data_end))
    : __clobber_all);
    }
    SEC("lwt_xmit")
    __description("direct packet write for LWT_XMIT")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn packet_write_for_lwt_xmit() -> __naked void {
    __naked void packet_write_for_lwt_xmit(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[__sk_buff_data]);		\
    r3 = *(u32*)(r1 + %[__sk_buff_data_end]);	\
    r0 = r2;					\
    r0 += 8;					\
    if r0 > r3 goto l0_%=;				\
// (u8*)(r2 + 0) = r2;				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(__sk_buff_data, offsetof(struct __sk_buff, data)),
    __imm_const(__sk_buff_data_end, offsetof(struct __sk_buff, data_end))
    : __clobber_all);
    }
    SEC("lwt_in")
    __description("direct packet read for LWT_IN")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn packet_read_for_lwt_in() -> __naked void {
    __naked void packet_read_for_lwt_in(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[__sk_buff_data]);		\
    r3 = *(u32*)(r1 + %[__sk_buff_data_end]);	\
    r0 = r2;					\
    r0 += 8;					\
    if r0 > r3 goto l0_%=;				\
    r0 = *(u8*)(r2 + 0);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(__sk_buff_data, offsetof(struct __sk_buff, data)),
    __imm_const(__sk_buff_data_end, offsetof(struct __sk_buff, data_end))
    : __clobber_all);
    }
    SEC("lwt_out")
    __description("direct packet read for LWT_OUT")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn packet_read_for_lwt_out() -> __naked void {
    __naked void packet_read_for_lwt_out(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[__sk_buff_data]);		\
    r3 = *(u32*)(r1 + %[__sk_buff_data_end]);	\
    r0 = r2;					\
    r0 += 8;					\
    if r0 > r3 goto l0_%=;				\
    r0 = *(u8*)(r2 + 0);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(__sk_buff_data, offsetof(struct __sk_buff, data)),
    __imm_const(__sk_buff_data_end, offsetof(struct __sk_buff, data_end))
    : __clobber_all);
    }
    SEC("lwt_xmit")
    __description("direct packet read for LWT_XMIT")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn packet_read_for_lwt_xmit() -> __naked void {
    __naked void packet_read_for_lwt_xmit(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[__sk_buff_data]);		\
    r3 = *(u32*)(r1 + %[__sk_buff_data_end]);	\
    r0 = r2;					\
    r0 += 8;					\
    if r0 > r3 goto l0_%=;				\
    r0 = *(u8*)(r2 + 0);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(__sk_buff_data, offsetof(struct __sk_buff, data)),
    __imm_const(__sk_buff_data_end, offsetof(struct __sk_buff, data_end))
    : __clobber_all);
    }
    SEC("lwt_xmit")
    __description("overlapping checks for direct packet access")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn checks_for_direct_packet_access() -> __naked void {
    __naked void checks_for_direct_packet_access(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[__sk_buff_data]);		\
    r3 = *(u32*)(r1 + %[__sk_buff_data_end]);	\
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
    : __imm_const(__sk_buff_data, offsetof(struct __sk_buff, data)),
    __imm_const(__sk_buff_data_end, offsetof(struct __sk_buff, data_end))
    : __clobber_all);
    }
    SEC("lwt_xmit")
    __description("make headroom for LWT_XMIT")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn make_headroom_for_lwt_xmit() -> __naked void {
    __naked void make_headroom_for_lwt_xmit(void)
    {
    asm volatile ("					\
    r6 = r1;					\
    r2 = 34;					\
    r3 = 0;						\
    call %[bpf_skb_change_head];			\
// split for s390 to succeed */			\
    r1 = r6;					\
    r2 = 42;					\
    r3 = 0;						\
    call %[bpf_skb_change_head];			\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_skb_change_head)
    : __clobber_all);
    }
    SEC("socket")
    __description("invalid access of tc_classid for LWT_IN")
#[no_mangle]
pub unsafe extern "C" fn __msg(access": "invalid bpf_context) -> __failure {
    __failure __msg("invalid bpf_context access")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn tc_classid_for_lwt_in() -> __naked void {
    __naked void tc_classid_for_lwt_in(void)
    {
    asm volatile ("					\
    r0 = *(u32*)(r1 + %[__sk_buff_tc_classid]);	\
    exit;						\
    "	:
    : __imm_const(__sk_buff_tc_classid, offsetof(struct __sk_buff, tc_classid))
    : __clobber_all);
    }
    SEC("socket")
    __description("invalid access of tc_classid for LWT_OUT")
#[no_mangle]
pub unsafe extern "C" fn __msg(access": "invalid bpf_context) -> __failure {
    __failure __msg("invalid bpf_context access")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn tc_classid_for_lwt_out() -> __naked void {
    __naked void tc_classid_for_lwt_out(void)
    {
    asm volatile ("					\
    r0 = *(u32*)(r1 + %[__sk_buff_tc_classid]);	\
    exit;						\
    "	:
    : __imm_const(__sk_buff_tc_classid, offsetof(struct __sk_buff, tc_classid))
    : __clobber_all);
    }
    SEC("socket")
    __description("invalid access of tc_classid for LWT_XMIT")
#[no_mangle]
pub unsafe extern "C" fn __msg(access": "invalid bpf_context) -> __failure {
    __failure __msg("invalid bpf_context access")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn tc_classid_for_lwt_xmit() -> __naked void {
    __naked void tc_classid_for_lwt_xmit(void)
    {
    asm volatile ("					\
    r0 = *(u32*)(r1 + %[__sk_buff_tc_classid]);	\
    exit;						\
    "	:
    : __imm_const(__sk_buff_tc_classid, offsetof(struct __sk_buff, tc_classid))
    : __clobber_all);
    }
    SEC("lwt_in")
    __description("check skb.tc_classid half load not permitted for lwt prog")
#[no_mangle]
pub unsafe extern "C" fn __msg(access": "invalid bpf_context) -> __failure {
    __failure __msg("invalid bpf_context access")
#[no_mangle]
pub unsafe extern "C" fn not_permitted_for_lwt_prog() -> __naked void {
    __naked void not_permitted_for_lwt_prog(void)
    {
    asm volatile (
    "r0 = 0;"

    "r0 = *(u16*)(r1 + %[__sk_buff_tc_classid]);"

    "r0 = *(u16*)(r1 + %[__imm_0]);"

    "exit;"
    :
    : __imm_const(__imm_0, offsetof(struct __sk_buff, tc_classid) + 2),
    __imm_const(__sk_buff_tc_classid, offsetof(struct __sk_buff, tc_classid))
    : __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
