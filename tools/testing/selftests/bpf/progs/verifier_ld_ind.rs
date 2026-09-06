//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_ld_ind.c
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
// Converted from tools/testing/selftests/bpf/verifier/ld_ind.c

    SEC("socket")
    __description("ld_ind: check calling conv, r1")
#[no_mangle]
pub unsafe extern "C" fn __msg(!read_ok": "R1) -> __failure {
    __failure __msg("R1 !read_ok")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn ind_check_calling_conv_r1() -> __naked void {
    __naked void ind_check_calling_conv_r1(void)
    {
    asm volatile ("					\
    r6 = r1;					\
    r1 = 1;						\
    .8byte %[ld_ind];				\
    r0 = r1;					\
    exit;						\
    "	:
    : __imm_insn(ld_ind, BPF_LD_IND(BPF_W, BPF_REG_1, -0x200000))
    : __clobber_all);
    }
    SEC("socket")
    __description("ld_ind: check calling conv, r2")
#[no_mangle]
pub unsafe extern "C" fn __msg(!read_ok": "R2) -> __failure {
    __failure __msg("R2 !read_ok")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn ind_check_calling_conv_r2() -> __naked void {
    __naked void ind_check_calling_conv_r2(void)
    {
    asm volatile ("					\
    r6 = r1;					\
    r2 = 1;						\
    .8byte %[ld_ind];				\
    r0 = r2;					\
    exit;						\
    "	:
    : __imm_insn(ld_ind, BPF_LD_IND(BPF_W, BPF_REG_2, -0x200000))
    : __clobber_all);
    }
    SEC("socket")
    __description("ld_ind: check calling conv, r3")
#[no_mangle]
pub unsafe extern "C" fn __msg(!read_ok": "R3) -> __failure {
    __failure __msg("R3 !read_ok")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn ind_check_calling_conv_r3() -> __naked void {
    __naked void ind_check_calling_conv_r3(void)
    {
    asm volatile ("					\
    r6 = r1;					\
    r3 = 1;						\
    .8byte %[ld_ind];				\
    r0 = r3;					\
    exit;						\
    "	:
    : __imm_insn(ld_ind, BPF_LD_IND(BPF_W, BPF_REG_3, -0x200000))
    : __clobber_all);
    }
    SEC("socket")
    __description("ld_ind: check calling conv, r4")
#[no_mangle]
pub unsafe extern "C" fn __msg(!read_ok": "R4) -> __failure {
    __failure __msg("R4 !read_ok")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn ind_check_calling_conv_r4() -> __naked void {
    __naked void ind_check_calling_conv_r4(void)
    {
    asm volatile ("					\
    r6 = r1;					\
    r4 = 1;						\
    .8byte %[ld_ind];				\
    r0 = r4;					\
    exit;						\
    "	:
    : __imm_insn(ld_ind, BPF_LD_IND(BPF_W, BPF_REG_4, -0x200000))
    : __clobber_all);
    }
    SEC("socket")
    __description("ld_ind: check calling conv, r5")
#[no_mangle]
pub unsafe extern "C" fn __msg(!read_ok": "R5) -> __failure {
    __failure __msg("R5 !read_ok")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn ind_check_calling_conv_r5() -> __naked void {
    __naked void ind_check_calling_conv_r5(void)
    {
    asm volatile ("					\
    r6 = r1;					\
    r5 = 1;						\
    .8byte %[ld_ind];				\
    r0 = r5;					\
    exit;						\
    "	:
    : __imm_insn(ld_ind, BPF_LD_IND(BPF_W, BPF_REG_5, -0x200000))
    : __clobber_all);
    }
    SEC("socket")
    __description("ld_ind: check calling conv, r7")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 1) -> __success __success_unpriv {
    __success __success_unpriv __retval(1)
#[no_mangle]
pub unsafe extern "C" fn ind_check_calling_conv_r7() -> __naked void {
    __naked void ind_check_calling_conv_r7(void)
    {
    asm volatile ("					\
    r6 = r1;					\
    r7 = 1;						\
    .8byte %[ld_ind];				\
    r0 = r7;					\
    exit;						\
    "	:
    : __imm_insn(ld_ind, BPF_LD_IND(BPF_W, BPF_REG_7, -0x200000))
    : __clobber_all);
    }
//
// ld_{abs,ind} subprog that always sets r0=1 on the success path.
// bpf_gen_ld_abs() emits a hidden exit with r0=0 when the load helper
// fails. The verifier must model this failure return so that callers
// account for r0=0 as a possible return value.
//
    __naked __noinline __used
#[no_mangle]
unsafe extern "C" fn ldabs_subprog() -> c_int {
    static int ldabs_subprog(void)
    {
    asm volatile (
    "r6 = r1;"
    ".8byte %[ld_abs];"
    "r0 = 1;"
    "exit;"
    :
    : __imm_insn(ld_abs, BPF_LD_ABS(BPF_W, 0))
    : __clobber_all);
    }
    __naked __noinline __used
#[no_mangle]
unsafe extern "C" fn ldind_subprog() -> c_int {
    static int ldind_subprog(void)
    {
    asm volatile (
    "r6 = r1;"
    "r7 = 0;"
    ".8byte %[ld_ind];"
    "r0 = 1;"
    "exit;"
    :
    : __imm_insn(ld_ind, BPF_LD_IND(BPF_W, BPF_REG_7, 0))
    : __clobber_all);
    }
    SEC("socket")
    __description("ld_abs: subprog early exit on ld_abs failure")
#[no_mangle]
pub unsafe extern "C" fn __msg(!read_ok": "R9) -> __failure {
    __failure __msg("R9 !read_ok")
#[no_mangle]
pub unsafe extern "C" fn ld_abs_subprog_early_exit() -> __naked void {
    __naked void ld_abs_subprog_early_exit(void)
    {
    asm volatile (
    "call ldabs_subprog;"
    "if r0 != 0 goto l_exit_%=;"
    "r0 = r9;"
    "l_exit_%=:"
    "r0 = 0;"
    "exit;"
    ::: __clobber_all);
    }
    SEC("socket")
    __description("ld_ind: subprog early exit on ld_ind failure")
#[no_mangle]
pub unsafe extern "C" fn __msg(!read_ok": "R9) -> __failure {
    __failure __msg("R9 !read_ok")
#[no_mangle]
pub unsafe extern "C" fn ld_ind_subprog_early_exit() -> __naked void {
    __naked void ld_ind_subprog_early_exit(void)
    {
    asm volatile (
    "call ldind_subprog;"
    "if r0 != 0 goto l_exit_%=;"
    "r0 = r9;"
    "l_exit_%=:"
    "r0 = 0;"
    "exit;"
    ::: __clobber_all);
    }
    SEC("socket")
    __description("ld_abs: subprog with both paths safe")
    __success
#[no_mangle]
pub unsafe extern "C" fn ld_abs_subprog_both_paths_safe() -> __naked void {
    __naked void ld_abs_subprog_both_paths_safe(void)
    {
    asm volatile (
    "call ldabs_subprog;"
    "r0 = 0;"
    "exit;"
    ::: __clobber_all);
    }
    SEC("socket")
    __description("ld_ind: subprog with both paths safe")
    __success
#[no_mangle]
pub unsafe extern "C" fn ld_ind_subprog_both_paths_safe() -> __naked void {
    __naked void ld_ind_subprog_both_paths_safe(void)
    {
    asm volatile (
    "call ldind_subprog;"
    "r0 = 0;"
    "exit;"
    ::: __clobber_all);
    }
//
// ld_{abs,ind} in subprogs require scalar (int) return type in BTF.
// A test with void return must be rejected.
//
    __naked __noinline __used
#[no_mangle]
unsafe extern "C" fn ldabs_void_subprog() {
    static void ldabs_void_subprog(void)
    {
    asm volatile (
    "r6 = r1;"
    ".8byte %[ld_abs];"
    "r0 = 1;"
    "exit;"
    :
    : __imm_insn(ld_abs, BPF_LD_ABS(BPF_W, 0))
    : __clobber_all);
    }
    SEC("socket")
    __description("ld_abs: reject void return subprog")
#[no_mangle]
pub unsafe extern "C" fn __msg('int'": "LD_ABS is only allowed in functions that return) -> __failure {
    __failure __msg("LD_ABS is only allowed in functions that return 'int'")
#[no_mangle]
pub unsafe extern "C" fn ld_abs_void_subprog_reject() -> __naked void {
    __naked void ld_abs_void_subprog_reject(void)
    {
    asm volatile (
    "call ldabs_void_subprog;"
    "r0 = 0;"
    "exit;"
    ::: __clobber_all);
    }
    __naked __noinline __used
#[no_mangle]
unsafe extern "C" fn ldind_void_subprog() {
    static void ldind_void_subprog(void)
    {
    asm volatile (
    "r6 = r1;"
    "r7 = 0;"
    ".8byte %[ld_ind];"
    "r0 = 1;"
    "exit;"
    :
    : __imm_insn(ld_ind, BPF_LD_IND(BPF_W, BPF_REG_7, 0))
    : __clobber_all);
    }
    SEC("socket")
    __description("ld_ind: reject void return subprog")
#[no_mangle]
pub unsafe extern "C" fn __msg('int'": "LD_ABS is only allowed in functions that return) -> __failure {
    __failure __msg("LD_ABS is only allowed in functions that return 'int'")
#[no_mangle]
pub unsafe extern "C" fn ld_ind_void_subprog_reject() -> __naked void {
    __naked void ld_ind_void_subprog_reject(void)
    {
    asm volatile (
    "call ldind_void_subprog;"
    "r0 = 0;"
    "exit;"
    ::: __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
