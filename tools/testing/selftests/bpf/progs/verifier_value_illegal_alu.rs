//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_value_illegal_alu.c
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
// Converted from tools/testing/selftests/bpf/verifier/value_illegal_alu.c

pub const MAX_ENTRIES: c_int = 11;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_val {
    pub index: c_uint,
    pub foo: [c_int; MAX_ENTRIES],
}

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1);
    __type(key, long long);
    __type(value, struct test_val);
    } map_hash_48b SEC(".maps");
    SEC("socket")
    __description("map element value illegal alu op, 1")
#[no_mangle]
pub unsafe extern "C" fn __msg(pointer": "R0 bitwise operator &= on) -> __failure {
    __failure __msg("R0 bitwise operator &= on pointer")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn value_illegal_alu_op_1() -> __naked void {
    __naked void value_illegal_alu_op_1(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r0 &= 8;					\
    r1 = 22;					\
// (u64*)(r0 + 0) = r1;				\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_48b)
    : __clobber_all);
    }
    SEC("socket")
    __description("map element value illegal alu op, 2")
#[no_mangle]
pub unsafe extern "C" fn __msg(prohibited": "R0 32-bit pointer arithmetic) -> __failure {
    __failure __msg("R0 32-bit pointer arithmetic prohibited")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn value_illegal_alu_op_2() -> __naked void {
    __naked void value_illegal_alu_op_2(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    w0 += 0;					\
    r1 = 22;					\
// (u64*)(r0 + 0) = r1;				\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_48b)
    : __clobber_all);
    }
    SEC("socket")
    __description("map element value illegal alu op, 3")
#[no_mangle]
pub unsafe extern "C" fn __msg(operator": "R0 pointer arithmetic with /=) -> __failure {
    __failure __msg("R0 pointer arithmetic with /= operator")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn value_illegal_alu_op_3() -> __naked void {
    __naked void value_illegal_alu_op_3(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r0 /= 42;					\
    r1 = 22;					\
// (u64*)(r0 + 0) = r1;				\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_48b)
    : __clobber_all);
    }
    SEC("socket")
    __description("map element value illegal alu op, 4")
#[no_mangle]
pub unsafe extern "C" fn __msg('scalar'": "invalid mem access) -> __failure {
    __failure __msg("invalid mem access 'scalar'")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(prohibited": "R0 pointer arithmetic) -> __failure_unpriv {
    __failure_unpriv __msg_unpriv("R0 pointer arithmetic prohibited")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn value_illegal_alu_op_4() -> __naked void {
    __naked void value_illegal_alu_op_4(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r0 = be64 r0;					\
    r1 = 22;					\
// (u64*)(r0 + 0) = r1;				\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_48b)
    : __clobber_all);
    }
    SEC("socket")
    __description("map element value illegal alu op, 5")
#[no_mangle]
pub unsafe extern "C" fn __msg('scalar'": "R0 invalid mem access) -> __failure {
    __failure __msg("R0 invalid mem access 'scalar'")
    __msg_unpriv("leaking pointer from stack off -8")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn value_illegal_alu_op_5() -> __naked void {
    __naked void value_illegal_alu_op_5(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r3 = 4096;					\
    r2 = r10;					\
    r2 += -8;					\
// (u64*)(r2 + 0) = r0;				\
    lock *(u64 *)(r2 + 0) += r3;			\
    r0 = *(u64*)(r2 + 0);				\
    r1 = 22;					\
// (u64*)(r0 + 0) = r1;				\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_48b)
    : __clobber_all);
    }
    SEC("socket")
    __description("map_ptr illegal alu op, map_ptr = -map_ptr")
#[no_mangle]
pub unsafe extern "C" fn __msg('scalar'": "R0 invalid mem access) -> __failure {
    __failure __msg("R0 invalid mem access 'scalar'")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(prohibited": "R0 pointer arithmetic) -> __failure_unpriv {
    __failure_unpriv __msg_unpriv("R0 pointer arithmetic prohibited")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn map_ptr_illegal_alu_op() -> __naked void {
    __naked void map_ptr_illegal_alu_op(void)
    {
    asm volatile ("					\
    r0 = %[map_hash_48b] ll;			\
    r0 = -r0;					\
    r1 = 22;					\
// (u64*)(r0 + 0) = r1;				\
    exit;						\
    "	:
    : __imm_addr(map_hash_48b)
    : __clobber_all);
    }
    SEC("flow_dissector")
    __description("flow_keys illegal alu op with variable offset")
#[no_mangle]
pub unsafe extern "C" fn __msg(prohibited": "R7 pointer arithmetic on flow_keys) -> __failure {
    __failure __msg("R7 pointer arithmetic on flow_keys prohibited")
#[no_mangle]
pub unsafe extern "C" fn flow_keys_illegal_variable_offset_alu() -> __naked void {
    __naked void flow_keys_illegal_variable_offset_alu(void)
    {
    asm volatile("					\
    r6 = r1;					\
    r7 = *(u64*)(r6 + %[flow_keys_off]);		\
    call %[bpf_get_prandom_u32];			\
    r8 = r0;					\
    r8 &= 8;					\
    r7 += r8;					\
    r0 = *(u64*)(r7 + 0);				\
    exit;						\
    "	:
    : __imm_const(flow_keys_off, offsetof(struct __sk_buff, flow_keys)),
    __imm(bpf_get_prandom_u32)
    : __clobber_all);
    }

    SEC("socket")					\
    __failure __msg("BPF_ALU uses reserved fields") \
    __naked void name(void)				\
    {						\
    asm volatile(				\
    "r0 = 1;"			\
    ".8byte %[insn];"		\
    "r0 = 0;"			\
    "exit;"				\
    :					\
    : __imm_insn(insn, BPF_RAW_INSN((op), 0, 0, (off), (imm))) \
    : __clobber_all);			\
    }
//
// Offset fields of 0 and 1 are legal for BPF_{DIV,MOD} instructions.
// Offset fields of 0 are legal for the rest of ALU instructions.
// Test that error is reported for illegal offsets, assuming that tests
// for legal offsets exist.
//
    DEFINE_BAD_OFFSET_TEST(bad_offset_divx, BPF_ALU64 | BPF_DIV | BPF_X, -1, 0)
    DEFINE_BAD_OFFSET_TEST(bad_offset_modk, BPF_ALU64 | BPF_MOD | BPF_K, -1, 1)
    DEFINE_BAD_OFFSET_TEST(bad_offset_addx, BPF_ALU64 | BPF_ADD | BPF_X, -1, 0)
    DEFINE_BAD_OFFSET_TEST(bad_offset_divx2, BPF_ALU64 | BPF_DIV | BPF_X, 2, 0)
    DEFINE_BAD_OFFSET_TEST(bad_offset_modk2, BPF_ALU64 | BPF_MOD | BPF_K, 2, 1)
    DEFINE_BAD_OFFSET_TEST(bad_offset_addx2, BPF_ALU64 | BPF_ADD | BPF_X, 1, 0)
    char _license[] SEC("license") = "GPL";
