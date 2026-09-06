//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_gotox.c
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
// Copyright (c) 2025 Isovalent

    \
    SEC("socket")							\
    OUTCOME								\
    __naked void jump_table_ ## NAME(void)				\
    {								\
    asm volatile ("						\
    .pushsection .jumptables,\"\",@progbits;		\
    jt0_%=:								\
    .quad ret0_%= - socket;					\
    .quad ret1_%= - socket;					\
    .size jt0_%=, 16;					\
    .global jt0_%=;						\
    .popsection;						\
    \
    r0 = jt0_%= ll;						\
    r0 += 8;						\
    r0 = *(u64 *)(r0 + 0);					\
    .8byte %[gotox_r0];					\
    ret0_%=:						\
    r0 = 0;							\
    exit;							\
    ret1_%=:						\
    r0 = 1;							\
    exit;							\
    "	:							\
    : __imm_insn(gotox_r0, BPF_RAW_INSN(BPF_JMP | BPF_JA | BPF_X, BPF_REG_0, (SRC_REG), (OFF) , (IMM))) \
    : __clobber_all);					\
    }
//
// The first program which doesn't use reserved fields
// loads and works properly. The rest fail to load.
//
    DEFINE_SIMPLE_JUMP_TABLE_PROG(ok,                          BPF_REG_0, 0, 0, __success __retval(1))
    DEFINE_SIMPLE_JUMP_TABLE_PROG(reserved_field_src_reg,      BPF_REG_1, 0, 0, __failure __msg("BPF_JA|BPF_X uses reserved fields"))
    DEFINE_SIMPLE_JUMP_TABLE_PROG(reserved_field_non_zero_off, BPF_REG_0, 1, 0, __failure __msg("BPF_JA|BPF_X uses reserved fields"))
    DEFINE_SIMPLE_JUMP_TABLE_PROG(reserved_field_non_zero_imm, BPF_REG_0, 0, 1, __failure __msg("BPF_JA|BPF_X uses reserved fields"))
//
// Gotox is forbidden when there is no jump table loaded
// which points to the sub-function where the gotox is used
//
    SEC("socket")
#[no_mangle]
pub unsafe extern "C" fn __msg(0": "no jump tables found for subprog starting at) -> __failure {
    __failure __msg("no jump tables found for subprog starting at 0")
#[no_mangle]
pub unsafe extern "C" fn jump_table_no_jump_table() -> __naked void {
    __naked void jump_table_no_jump_table(void)
    {
    asm volatile ("						\
    .8byte %[gotox_r0];					\
    r0 = 1;							\
    exit;							\
    "	:							\
    : __imm_insn(gotox_r0, BPF_RAW_INSN(BPF_JMP | BPF_JA | BPF_X, BPF_REG_0, 0, 0 , 0))
    : __clobber_all);
    }
//
// Incorrect type of the target register, only PTR_TO_INSN allowed
//
    SEC("socket")
#[no_mangle]
pub unsafe extern "C" fn __msg(scalar: "R1 has type, PTR_TO_INSN": expected) -> __failure {
    __failure __msg("R1 has type scalar, expected PTR_TO_INSN")
#[no_mangle]
pub unsafe extern "C" fn jump_table_incorrect_dst_reg_type() -> __naked void {
    __naked void jump_table_incorrect_dst_reg_type(void)
    {
    asm volatile ("						\
    .pushsection .jumptables,\"\",@progbits;		\
    jt0_%=:								\
    .quad ret0_%= - socket;					\
    .quad ret1_%= - socket;					\
    .size jt0_%=, 16;					\
    .global jt0_%=;						\
    .popsection;						\
    \
    r0 = jt0_%= ll;						\
    r0 += 8;						\
    r0 = *(u64 *)(r0 + 0);					\
    r1 = 42;						\
    .8byte %[gotox_r1];					\
    ret0_%=:						\
    r0 = 0;							\
    exit;							\
    ret1_%=:						\
    r0 = 1;							\
    exit;							\
    "	:							\
    : __imm_insn(gotox_r1, BPF_RAW_INSN(BPF_JMP | BPF_JA | BPF_X, BPF_REG_1, 0, 0 , 0))
    : __clobber_all);
    }

    \
    SEC("socket")							\
    OUTCOME								\
    __naked void jump_table_invalid_read_size_ ## READ_SIZE(void)	\
    {								\
    asm volatile ("						\
    .pushsection .jumptables,\"\",@progbits;		\
    jt0_%=:								\
    .quad ret0_%= - socket;					\
    .quad ret1_%= - socket;					\
    .size jt0_%=, 16;					\
    .global jt0_%=;						\
    .popsection;						\
    \
    r0 = jt0_%= ll;						\
    r0 += 8;						\
    r0 = *(" #READ_SIZE " *)(r0 + 0);			\
    .8byte %[gotox_r0];					\
    ret0_%=:						\
    r0 = 0;							\
    exit;							\
    ret1_%=:						\
    r0 = 1;							\
    exit;							\
    "	:							\
    : __imm_insn(gotox_r0, BPF_RAW_INSN(BPF_JMP | BPF_JA | BPF_X, BPF_REG_0, 0, 0 , 0)) \
    : __clobber_all);					\
    }
    DEFINE_INVALID_SIZE_PROG(u32, __failure __msg("Invalid read of 4 bytes from insn_array"))
    DEFINE_INVALID_SIZE_PROG(u16, __failure __msg("Invalid read of 2 bytes from insn_array"))
    DEFINE_INVALID_SIZE_PROG(u8,  __failure __msg("Invalid read of 1 bytes from insn_array"))
    SEC("socket")
#[no_mangle]
pub unsafe extern "C" fn __msg(8": "misaligned value access off 1+0 size) -> __failure {
    __failure __msg("misaligned value access off 1+0 size 8")
#[no_mangle]
pub unsafe extern "C" fn jump_table_misaligned_access() -> __naked void {
    __naked void jump_table_misaligned_access(void)
    {
    asm volatile ("						\
    .pushsection .jumptables,\"\",@progbits;		\
    jt0_%=:								\
    .quad ret0_%= - socket;					\
    .quad ret1_%= - socket;					\
    .size jt0_%=, 16;					\
    .global jt0_%=;						\
    .popsection;						\
    \
    r0 = jt0_%= ll;						\
    r0 += 1;						\
    r0 = *(u64 *)(r0 + 0);					\
    .8byte %[gotox_r0];					\
    ret0_%=:						\
    r0 = 0;							\
    exit;							\
    ret1_%=:						\
    r0 = 1;							\
    exit;							\
    "	:							\
    : __imm_insn(gotox_r0, BPF_RAW_INSN(BPF_JMP | BPF_JA | BPF_X, BPF_REG_0, 0, 0 , 0))
    : __clobber_all);
    }
    SEC("socket")
#[no_mangle]
pub unsafe extern "C" fn __msg(value: "invalid access to map, size=8": value_size=16 off=24) -> __failure {
    __failure __msg("invalid access to map value, value_size=16 off=24 size=8")
#[no_mangle]
pub unsafe extern "C" fn jump_table_invalid_mem_acceess_pos() -> __naked void {
    __naked void jump_table_invalid_mem_acceess_pos(void)
    {
    asm volatile ("						\
    .pushsection .jumptables,\"\",@progbits;		\
    jt0_%=:								\
    .quad ret0_%= - socket;					\
    .quad ret1_%= - socket;					\
    .size jt0_%=, 16;					\
    .global jt0_%=;						\
    .popsection;						\
    \
    r0 = jt0_%= ll;						\
    r0 += 24;						\
    r0 = *(u64 *)(r0 + 0);					\
    .8byte %[gotox_r0];					\
    ret0_%=:						\
    r0 = 0;							\
    exit;							\
    ret1_%=:						\
    r0 = 1;							\
    exit;							\
    "	:							\
    : __imm_insn(gotox_r0, BPF_RAW_INSN(BPF_JMP | BPF_JA | BPF_X, BPF_REG_0, 0, 0 , 0))
    : __clobber_all);
    }
    SEC("socket")
#[no_mangle]
pub unsafe extern "C" fn __msg(negative": "R0 min value is) -> __failure {
    __failure __msg("R0 min value is negative")
#[no_mangle]
pub unsafe extern "C" fn jump_table_invalid_mem_acceess_neg() -> __naked void {
    __naked void jump_table_invalid_mem_acceess_neg(void)
    {
    asm volatile ("						\
    .pushsection .jumptables,\"\",@progbits;		\
    jt0_%=:								\
    .quad ret0_%= - socket;					\
    .quad ret1_%= - socket;					\
    .size jt0_%=, 16;					\
    .global jt0_%=;						\
    .popsection;						\
    \
    r0 = jt0_%= ll;						\
    r0 -= 24;						\
    r0 = *(u64 *)(r0 + 0);					\
    .8byte %[gotox_r0];					\
    ret0_%=:						\
    r0 = 0;							\
    exit;							\
    ret1_%=:						\
    r0 = 1;							\
    exit;							\
    "	:							\
    : __imm_insn(gotox_r0, BPF_RAW_INSN(BPF_JMP | BPF_JA | BPF_X, BPF_REG_0, 0, 0 , 0))
    : __clobber_all);
    }
    SEC("socket")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 1) -> __success {
    __success __retval(1)
#[no_mangle]
pub unsafe extern "C" fn jump_table_add_sub_ok() -> __naked void {
    __naked void jump_table_add_sub_ok(void)
    {
    asm volatile ("						\
    .pushsection .jumptables,\"\",@progbits;		\
    jt0_%=:								\
    .quad ret0_%= - socket;					\
    .quad ret1_%= - socket;					\
    .size jt0_%=, 16;					\
    .global jt0_%=;						\
    .popsection;						\
    \
    r0 = jt0_%= ll;						\
    r0 -= 24;						\
    r0 += 32;						\
    r0 = *(u64 *)(r0 + 0);					\
    .8byte %[gotox_r0];					\
    ret0_%=:						\
    r0 = 0;							\
    exit;							\
    ret1_%=:						\
    r0 = 1;							\
    exit;							\
    "	:							\
    : __imm_insn(gotox_r0, BPF_RAW_INSN(BPF_JMP | BPF_JA | BPF_X, BPF_REG_0, 0, 0 , 0))
    : __clobber_all);
    }
    SEC("socket")
#[no_mangle]
pub unsafe extern "C" fn __msg(forbidden: "write into map, size=8": value_size=16 off=8) -> __failure {
    __failure __msg("write into map forbidden, value_size=16 off=8 size=8")
#[no_mangle]
pub unsafe extern "C" fn jump_table_no_writes() -> __naked void {
    __naked void jump_table_no_writes(void)
    {
    asm volatile ("						\
    .pushsection .jumptables,\"\",@progbits;		\
    jt0_%=:								\
    .quad ret0_%= - socket;					\
    .quad ret1_%= - socket;					\
    .size jt0_%=, 16;					\
    .global jt0_%=;						\
    .popsection;						\
    \
    r0 = jt0_%= ll;						\
    r0 += 8;						\
    r1 = 0xbeef;						\
// (u64 *)(r0 + 0) = r1;					\
    .8byte %[gotox_r0];					\
    ret0_%=:						\
    r0 = 0;							\
    exit;							\
    ret1_%=:						\
    r0 = 1;							\
    exit;							\
    "	:							\
    : __imm_insn(gotox_r0, BPF_RAW_INSN(BPF_JMP | BPF_JA | BPF_X, BPF_REG_0, 0, 0 , 0))
    : __clobber_all);
    }

    SEC("socket")							\
    __success __retval(1)						\
    __naked void jump_table_use_reg_r ## REG(void)			\
    {								\
    asm volatile ("						\
    .pushsection .jumptables,\"\",@progbits;		\
    jt0_%=:								\
    .quad ret0_%= - socket;					\
    .quad ret1_%= - socket;					\
    .size jt0_%=, 16;					\
    .global jt0_%=;						\
    .popsection;						\
    \
    r0 = jt0_%= ll;						\
    r0 += 8;						\
    r" #REG " = *(u64 *)(r0 + 0);				\
    .8byte %[gotox_rX];					\
    ret0_%=:						\
    r0 = 0;							\
    exit;							\
    ret1_%=:						\
    r0 = 1;							\
    exit;							\
    "	:							\
    : __imm_insn(gotox_rX, BPF_RAW_INSN(BPF_JMP | BPF_JA | BPF_X, BPF_REG_ ## REG, 0, 0 , 0)) \
    : __clobber_all);					\
    }
    DEFINE_JUMP_TABLE_USE_REG(0)
    DEFINE_JUMP_TABLE_USE_REG(1)
    DEFINE_JUMP_TABLE_USE_REG(2)
    DEFINE_JUMP_TABLE_USE_REG(3)
    DEFINE_JUMP_TABLE_USE_REG(4)
    DEFINE_JUMP_TABLE_USE_REG(5)
    DEFINE_JUMP_TABLE_USE_REG(6)
    DEFINE_JUMP_TABLE_USE_REG(7)
    DEFINE_JUMP_TABLE_USE_REG(8)
    DEFINE_JUMP_TABLE_USE_REG(9)
#[no_mangle]
pub unsafe extern "C" fn test_subprog() -> __used static int {
    __used static int test_subprog(void)
    {
    return 0;
    }
    SEC("socket")
#[no_mangle]
pub unsafe extern "C" fn __msg([0: "jump table for insn 4 points outside of the subprog, _arg: 10]") -> __failure {
    __failure __msg("jump table for insn 4 points outside of the subprog [0,10]")
#[no_mangle]
pub unsafe extern "C" fn jump_table_outside_subprog() -> __naked void {
    __naked void jump_table_outside_subprog(void)
    {
    asm volatile ("						\
    .pushsection .jumptables,\"\",@progbits;		\
    jt0_%=:								\
    .quad ret0_%= - socket;					\
    .quad ret1_%= - socket;					\
    .quad ret_out_%= - socket;				\
    .size jt0_%=, 24;					\
    .global jt0_%=;						\
    .popsection;						\
    \
    r0 = jt0_%= ll;						\
    r0 += 8;						\
    r0 = *(u64 *)(r0 + 0);					\
    .8byte %[gotox_r0];					\
    ret0_%=:						\
    r0 = 0;							\
    exit;							\
    ret1_%=:						\
    r0 = 1;							\
    call test_subprog;					\
    exit;							\
    ret_out_%=:						\
    "	:							\
    : __imm_insn(gotox_r0, BPF_RAW_INSN(BPF_JMP | BPF_JA | BPF_X, BPF_REG_0, 0, 0 , 0))
    : __clobber_all);
    }
    SEC("socket")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 1) -> __success {
    __success __retval(1)
#[no_mangle]
pub unsafe extern "C" fn jump_table_contains_non_unique_values() -> __naked void {
    __naked void jump_table_contains_non_unique_values(void)
    {
    asm volatile ("						\
    .pushsection .jumptables,\"\",@progbits;		\
    jt0_%=:								\
    .quad ret0_%= - socket;					\
    .quad ret1_%= - socket;					\
    .quad ret0_%= - socket;					\
    .quad ret1_%= - socket;					\
    .quad ret0_%= - socket;					\
    .quad ret1_%= - socket;					\
    .quad ret0_%= - socket;					\
    .quad ret1_%= - socket;					\
    .quad ret0_%= - socket;					\
    .quad ret1_%= - socket;					\
    .size jt0_%=, 80;					\
    .global jt0_%=;						\
    .popsection;						\
    \
    r0 = jt0_%= ll;						\
    r0 += 8;						\
    r0 = *(u64 *)(r0 + 0);					\
    .8byte %[gotox_r0];					\
    ret0_%=:						\
    r0 = 0;							\
    exit;							\
    ret1_%=:						\
    r0 = 1;							\
    exit;							\
    "	:							\
    : __imm_insn(gotox_r0, BPF_RAW_INSN(BPF_JMP | BPF_JA | BPF_X, BPF_REG_0, 0, 0 , 0))
    : __clobber_all);
    }
// check valid spill/fill, ptr to insn
    SEC("socket")
    __success
#[no_mangle]
pub unsafe extern "C" fn spill_fill_ptr_to_insn() -> __naked void {
    __naked void spill_fill_ptr_to_insn(void)
    {
    asm volatile (
    ".pushsection .jumptables,\"\",@progbits;"
    "jt0_%=:"
    ".quad ret0_%= - socket;"
    ".size jt0_%=, 8;"
    ".global jt0_%=;"
    ".popsection;"
    "r0 = jt0_%= ll;"
    "r0 = *(u64 *)(r0 + 0);"
    "*(u64 *)(r10 - 8) = r0;"
    "r0 = *(u64 *)(r10 - 8);"
    ".8byte %[gotox_r0];"
    "ret0_%=:"
    "r0 = 0;"
    "exit;"
    :
    : __imm_insn(gotox_r0, BPF_RAW_INSN(BPF_JMP | BPF_JA | BPF_X, BPF_REG_0, 0, 0, 0))
    : __clobber_all);
    }

    char _license[] SEC("license") = "GPL";
