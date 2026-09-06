//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_flow_keys.c
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
// Bounds checks for PTR_TO_FLOW_KEYS pointer arithmetic.

// sizeof(struct bpf_flow_keys) is well under 4096, so +0x1000 is OOB.
    SEC("flow_dissector")
    __description("flow_keys: in-bounds constant pointer arithmetic accepted")
    __success
#[no_mangle]
pub unsafe extern "C" fn flow_keys_const_inbounds() -> __naked void {
    __naked void flow_keys_const_inbounds(void)
    {
    asm volatile ("					\
    r1 = *(u64 *)(r1 + %[flow_keys]);		\
    r1 += 8;					\
    r0 = *(u64 *)(r1 + 0);				\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(flow_keys, offsetof(struct __sk_buff, flow_keys))
    : __clobber_all);
    }
    SEC("flow_dissector")
    __description("flow_keys: OOB via constant pointer arithmetic rejected")
#[no_mangle]
pub unsafe extern "C" fn __msg(size=8": "invalid access to flow keys off=4096) -> __failure {
    __failure __msg("invalid access to flow keys off=4096 size=8")
#[no_mangle]
pub unsafe extern "C" fn flow_keys_const_oob_read() -> __naked void {
    __naked void flow_keys_const_oob_read(void)
    {
    asm volatile ("					\
    r1 = *(u64 *)(r1 + %[flow_keys]);		\
    r1 += 4096;					\
    r0 = *(u64 *)(r1 + 0);				\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(flow_keys, offsetof(struct __sk_buff, flow_keys))
    : __clobber_all);
    }
    SEC("flow_dissector")
    __description("flow_keys: OOB write via constant pointer arithmetic rejected")
#[no_mangle]
pub unsafe extern "C" fn __msg(size=8": "invalid access to flow keys off=4096) -> __failure {
    __failure __msg("invalid access to flow keys off=4096 size=8")
#[no_mangle]
pub unsafe extern "C" fn flow_keys_const_oob_write() -> __naked void {
    __naked void flow_keys_const_oob_write(void)
    {
    asm volatile ("					\
    r1 = *(u64 *)(r1 + %[flow_keys]);		\
    r1 += 4096;					\
    r2 = 0;						\
// (u64 *)(r1 + 0) = r2;				\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(flow_keys, offsetof(struct __sk_buff, flow_keys))
    : __clobber_all);
    }
// Equivalent OOB expressed directly in insn->off; this form was always
// rejected and is kept to show both forms now share one diagnostic.
//
    SEC("flow_dissector")
    __description("flow_keys: OOB via insn.off rejected")
#[no_mangle]
pub unsafe extern "C" fn __msg(size=8": "invalid access to flow keys off=4096) -> __failure {
    __failure __msg("invalid access to flow keys off=4096 size=8")
#[no_mangle]
pub unsafe extern "C" fn flow_keys_insn_off_oob() -> __naked void {
    __naked void flow_keys_insn_off_oob(void)
    {
    asm volatile ("					\
    r1 = *(u64 *)(r1 + %[flow_keys]);		\
    r0 = *(u64 *)(r1 + 4096);			\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(flow_keys, offsetof(struct __sk_buff, flow_keys))
    : __clobber_all);
    }
    SEC("flow_dissector")
    __description("flow_keys: variable pointer arithmetic rejected")
#[no_mangle]
pub unsafe extern "C" fn __msg(prohibited": "R1 pointer arithmetic on flow_keys) -> __failure {
    __failure __msg("R1 pointer arithmetic on flow_keys prohibited")
#[no_mangle]
pub unsafe extern "C" fn flow_keys_var_read() -> __naked void {
    __naked void flow_keys_var_read(void)
    {
    asm volatile ("					\
    r6 = r1;					\
    call %[bpf_get_prandom_u32];			\
    r0 &= 0xFFFF;					\
    r1 = *(u64 *)(r6 + %[flow_keys]);		\
    r1 += r0;					\
    r0 = *(u64 *)(r1 + 0);				\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(flow_keys, offsetof(struct __sk_buff, flow_keys)),
    __imm(bpf_get_prandom_u32)
    : __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
