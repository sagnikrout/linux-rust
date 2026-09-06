//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_int_ptr.c
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
// Converted from tools/testing/selftests/bpf/verifier/int_ptr.c

    SEC("socket")
    __description("arg pointer to long uninitialized")
    __success
#[no_mangle]
pub unsafe extern "C" fn arg_ptr_to_long_uninitialized() -> __naked void {
    __naked void arg_ptr_to_long_uninitialized(void)
    {
    asm volatile ("					\
// bpf_strtoul arg1 (buf) */			\
    r7 = r10;					\
    r7 += -8;					\
    r0 = 0x00303036;				\
// (u64*)(r7 + 0) = r0;				\
    r1 = r7;					\
// bpf_strtoul arg2 (buf_len) */		\
    r2 = 4;						\
// bpf_strtoul arg3 (flags) */			\
    r3 = 0;						\
// bpf_strtoul arg4 (res) */			\
    r7 += -8;					\
    r4 = r7;					\
// bpf_strtoul() */				\
    call %[bpf_strtoul];				\
    r0 = 1;						\
    exit;						\
    "	:
    : __imm(bpf_strtoul)
    : __clobber_all);
    }
    SEC("socket")
    __description("arg pointer to long half-uninitialized")
    __success
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn ptr_to_long_half_uninitialized() -> __naked void {
    __naked void ptr_to_long_half_uninitialized(void)
    {
    asm volatile ("					\
// bpf_strtoul arg1 (buf) */			\
    r7 = r10;					\
    r7 += -8;					\
    r0 = 0x00303036;				\
// (u64*)(r7 + 0) = r0;				\
    r1 = r7;					\
// bpf_strtoul arg2 (buf_len) */		\
    r2 = 4;						\
// bpf_strtoul arg3 (flags) */			\
    r3 = 0;						\
// bpf_strtoul arg4 (res) */			\
    r7 += -8;					\
// (u32*)(r7 + 0) = r0;				\
    r4 = r7;					\
// bpf_strtoul() */				\
    call %[bpf_strtoul];				\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_strtoul)
    : __clobber_all);
    }
    SEC("cgroup/sysctl")
    __description("arg pointer to long misaligned")
#[no_mangle]
pub unsafe extern "C" fn __msg(8": "misaligned stack access off -20+0 size) -> __failure {
    __failure __msg("misaligned stack access off -20+0 size 8")
#[no_mangle]
pub unsafe extern "C" fn arg_ptr_to_long_misaligned() -> __naked void {
    __naked void arg_ptr_to_long_misaligned(void)
    {
    asm volatile ("					\
// bpf_strtoul arg1 (buf) */			\
    r7 = r10;					\
    r7 += -8;					\
    r0 = 0x00303036;				\
// (u64*)(r7 + 0) = r0;				\
    r1 = r7;					\
// bpf_strtoul arg2 (buf_len) */		\
    r2 = 4;						\
// bpf_strtoul arg3 (flags) */			\
    r3 = 0;						\
// bpf_strtoul arg4 (res) */			\
    r7 += -12;					\
    r0 = 0;						\
// (u32*)(r7 + 0) = r0;				\
// (u64*)(r7 + 4) = r0;				\
    r4 = r7;					\
// bpf_strtoul() */				\
    call %[bpf_strtoul];				\
    r0 = 1;						\
    exit;						\
    "	:
    : __imm(bpf_strtoul)
    : __clobber_all);
    }
    SEC("cgroup/sysctl")
    __description("arg pointer to long size < sizeof(long)")
#[no_mangle]
pub unsafe extern "C" fn __msg(size=8": "invalid write to stack R4 off=-4) -> __failure {
    __failure __msg("invalid write to stack R4 off=-4 size=8")
#[no_mangle]
pub unsafe extern "C" fn to_long_size_sizeof_long() -> __naked void {
    __naked void to_long_size_sizeof_long(void)
    {
    asm volatile ("					\
// bpf_strtoul arg1 (buf) */			\
    r7 = r10;					\
    r7 += -16;					\
    r0 = 0x00303036;				\
// (u64*)(r7 + 0) = r0;				\
    r1 = r7;					\
// bpf_strtoul arg2 (buf_len) */		\
    r2 = 4;						\
// bpf_strtoul arg3 (flags) */			\
    r3 = 0;						\
// bpf_strtoul arg4 (res) */			\
    r7 += 12;					\
// (u32*)(r7 + 0) = r0;				\
    r4 = r7;					\
// bpf_strtoul() */				\
    call %[bpf_strtoul];				\
    r0 = 1;						\
    exit;						\
    "	:
    : __imm(bpf_strtoul)
    : __clobber_all);
    }
    SEC("cgroup/sysctl")
    __description("arg pointer to long initialized")
    __success
#[no_mangle]
pub unsafe extern "C" fn arg_ptr_to_long_initialized() -> __naked void {
    __naked void arg_ptr_to_long_initialized(void)
    {
    asm volatile ("					\
// bpf_strtoul arg1 (buf) */			\
    r7 = r10;					\
    r7 += -8;					\
    r0 = 0x00303036;				\
// (u64*)(r7 + 0) = r0;				\
    r1 = r7;					\
// bpf_strtoul arg2 (buf_len) */		\
    r2 = 4;						\
// bpf_strtoul arg3 (flags) */			\
    r3 = 0;						\
// bpf_strtoul arg4 (res) */			\
    r7 += -8;					\
// (u64*)(r7 + 0) = r0;				\
    r4 = r7;					\
// bpf_strtoul() */				\
    call %[bpf_strtoul];				\
    r0 = 1;						\
    exit;						\
    "	:
    : __imm(bpf_strtoul)
    : __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
