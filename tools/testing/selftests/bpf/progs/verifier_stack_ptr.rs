//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_stack_ptr.c
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
// Converted from tools/testing/selftests/bpf/verifier/stack_ptr.c

pub const MAX_ENTRIES: c_int = 11;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_val {
    pub index: c_uint,
    pub foo: [c_int; MAX_ENTRIES],
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, struct test_val);
    } map_array_48b SEC(".maps");
    SEC("socket")
    __description("PTR_TO_STACK store/load")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0xfaceb00c) -> __success __success_unpriv {
    __success __success_unpriv __retval(0xfaceb00c)
#[no_mangle]
pub unsafe extern "C" fn ptr_to_stack_store_load() -> __naked void {
    __naked void ptr_to_stack_store_load(void)
    {
    asm volatile ("					\
    r1 = r10;					\
    r1 += -10;					\
    r0 = 0xfaceb00c;				\
// (u64*)(r1 + 2) = r0;				\
    r0 = *(u64*)(r1 + 2);				\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("PTR_TO_STACK store/load - bad alignment on off")
#[no_mangle]
pub unsafe extern "C" fn __msg(8": "misaligned stack access off -8+2 size) -> __failure {
    __failure __msg("misaligned stack access off -8+2 size 8")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn load_bad_alignment_on_off() -> __naked void {
    __naked void load_bad_alignment_on_off(void)
    {
    asm volatile ("					\
    r1 = r10;					\
    r1 += -8;					\
    r0 = 0xfaceb00c;				\
// (u64*)(r1 + 2) = r0;				\
    r0 = *(u64*)(r1 + 2);				\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("PTR_TO_STACK store/load - bad alignment on reg")
#[no_mangle]
pub unsafe extern "C" fn __msg(8": "misaligned stack access off -10+8 size) -> __failure {
    __failure __msg("misaligned stack access off -10+8 size 8")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn load_bad_alignment_on_reg() -> __naked void {
    __naked void load_bad_alignment_on_reg(void)
    {
    asm volatile ("					\
    r1 = r10;					\
    r1 += -10;					\
    r0 = 0xfaceb00c;				\
// (u64*)(r1 + 8) = r0;				\
    r0 = *(u64*)(r1 + 8);				\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("PTR_TO_STACK store/load - out of bounds low")
#[no_mangle]
pub unsafe extern "C" fn __msg(size=8": "invalid write to stack R1 off=-79992) -> __failure {
    __failure __msg("invalid write to stack R1 off=-79992 size=8")
    __msg_unpriv("R1 stack pointer arithmetic goes out of range")
#[no_mangle]
pub unsafe extern "C" fn load_out_of_bounds_low() -> __naked void {
    __naked void load_out_of_bounds_low(void)
    {
    asm volatile ("					\
    r1 = r10;					\
    r1 += -80000;					\
    r0 = 0xfaceb00c;				\
// (u64*)(r1 + 8) = r0;				\
    r0 = *(u64*)(r1 + 8);				\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("PTR_TO_STACK store/load - out of bounds high")
#[no_mangle]
pub unsafe extern "C" fn __msg(size=8": "invalid write to stack R1 off=0) -> __failure {
    __failure __msg("invalid write to stack R1 off=0 size=8")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn load_out_of_bounds_high() -> __naked void {
    __naked void load_out_of_bounds_high(void)
    {
    asm volatile ("					\
    r1 = r10;					\
    r1 += -8;					\
    r0 = 0xfaceb00c;				\
// (u64*)(r1 + 8) = r0;				\
    r0 = *(u64*)(r1 + 8);				\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("PTR_TO_STACK check high 1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 42) -> __success __success_unpriv {
    __success __success_unpriv __retval(42)
#[no_mangle]
pub unsafe extern "C" fn to_stack_check_high_1() -> __naked void {
    __naked void to_stack_check_high_1(void)
    {
    asm volatile ("					\
    r1 = r10;					\
    r1 += -1;					\
    r0 = 42;					\
// (u8*)(r1 + 0) = r0;				\
    r0 = *(u8*)(r1 + 0);				\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("PTR_TO_STACK check high 2")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 42) -> __success __success_unpriv {
    __success __success_unpriv __retval(42)
#[no_mangle]
pub unsafe extern "C" fn to_stack_check_high_2() -> __naked void {
    __naked void to_stack_check_high_2(void)
    {
    asm volatile ("					\
    r1 = r10;					\
    r0 = 42;					\
// (u8*)(r1 - 1) = r0;				\
    r0 = *(u8*)(r1 - 1);				\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("PTR_TO_STACK check high 3")
    __success __failure_unpriv
    __msg_unpriv("R1 stack pointer arithmetic goes out of range")
    __retval(42)
#[no_mangle]
pub unsafe extern "C" fn to_stack_check_high_3() -> __naked void {
    __naked void to_stack_check_high_3(void)
    {
    asm volatile ("					\
    r1 = r10;					\
    r1 += 0;					\
    r0 = 42;					\
// (u8*)(r1 - 1) = r0;				\
    r0 = *(u8*)(r1 - 1);				\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("PTR_TO_STACK check high 4")
#[no_mangle]
pub unsafe extern "C" fn __msg(size=1": "invalid write to stack R1 off=0) -> __failure {
    __failure __msg("invalid write to stack R1 off=0 size=1")
    __msg_unpriv("R1 stack pointer arithmetic goes out of range")
#[no_mangle]
pub unsafe extern "C" fn to_stack_check_high_4() -> __naked void {
    __naked void to_stack_check_high_4(void)
    {
    asm volatile ("					\
    r1 = r10;					\
    r1 += 0;					\
    r0 = 42;					\
// (u8*)(r1 + 0) = r0;				\
    r0 = *(u8*)(r1 + 0);				\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("PTR_TO_STACK check high 5")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "invalid write to stack) -> __failure {
    __failure __msg("invalid write to stack R1")
    __msg_unpriv("R1 stack pointer arithmetic goes out of range")
#[no_mangle]
pub unsafe extern "C" fn to_stack_check_high_5() -> __naked void {
    __naked void to_stack_check_high_5(void)
    {
    asm volatile ("					\
    r1 = r10;					\
    r1 += %[__imm_0];				\
    r0 = 42;					\
// (u8*)(r1 + 0) = r0;				\
    r0 = *(u8*)(r1 + 0);				\
    exit;						\
    "	:
    : __imm_const(__imm_0, (1 << 29) - 1)
    : __clobber_all);
    }
    SEC("socket")
    __description("PTR_TO_STACK check high 6")
#[no_mangle]
pub unsafe extern "C" fn __msg(stack": "invalid write to) -> __failure {
    __failure __msg("invalid write to stack")
    __msg_unpriv("R1 stack pointer arithmetic goes out of range")
#[no_mangle]
pub unsafe extern "C" fn to_stack_check_high_6() -> __naked void {
    __naked void to_stack_check_high_6(void)
    {
    asm volatile ("					\
    r1 = r10;					\
    r1 += %[__imm_0];				\
    r0 = 42;					\
// (u8*)(r1 + %[shrt_max]) = r0;			\
    r0 = *(u8*)(r1 + %[shrt_max]);			\
    exit;						\
    "	:
    : __imm_const(__imm_0, (1 << 29) - 1),
    __imm_const(shrt_max, SHRT_MAX)
    : __clobber_all);
    }
    SEC("socket")
    __description("PTR_TO_STACK check high 7")
#[no_mangle]
pub unsafe extern "C" fn __msg(offset": "fp pointer) -> __failure {
    __failure __msg("fp pointer offset")
    __msg_unpriv("R1 stack pointer arithmetic goes out of range")
#[no_mangle]
pub unsafe extern "C" fn to_stack_check_high_7() -> __naked void {
    __naked void to_stack_check_high_7(void)
    {
    asm volatile ("					\
    r1 = r10;					\
    r1 += %[__imm_0];				\
    r1 += %[__imm_0];				\
    r0 = 42;					\
// (u8*)(r1 + %[shrt_max]) = r0;			\
    r0 = *(u8*)(r1 + %[shrt_max]);			\
    exit;						\
    "	:
    : __imm_const(__imm_0, (1 << 29) - 1),
    __imm_const(shrt_max, SHRT_MAX)
    : __clobber_all);
    }
    SEC("socket")
    __description("PTR_TO_STACK check low 1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 42) -> __success __success_unpriv {
    __success __success_unpriv __retval(42)
#[no_mangle]
pub unsafe extern "C" fn to_stack_check_low_1() -> __naked void {
    __naked void to_stack_check_low_1(void)
    {
    asm volatile ("					\
    r1 = r10;					\
    r1 += -512;					\
    r0 = 42;					\
// (u8*)(r1 + 0) = r0;				\
    r0 = *(u8*)(r1 + 0);				\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("PTR_TO_STACK check low 2")
    __success __failure_unpriv
    __msg_unpriv("R1 stack pointer arithmetic goes out of range")
    __retval(42)
#[no_mangle]
pub unsafe extern "C" fn to_stack_check_low_2() -> __naked void {
    __naked void to_stack_check_low_2(void)
    {
    asm volatile ("					\
    r1 = r10;					\
    r1 += -513;					\
    r0 = 42;					\
// (u8*)(r1 + 1) = r0;				\
    r0 = *(u8*)(r1 + 1);				\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("PTR_TO_STACK check low 3")
#[no_mangle]
pub unsafe extern "C" fn __msg(size=1": "invalid write to stack R1 off=-513) -> __failure {
    __failure __msg("invalid write to stack R1 off=-513 size=1")
    __msg_unpriv("R1 stack pointer arithmetic goes out of range")
#[no_mangle]
pub unsafe extern "C" fn to_stack_check_low_3() -> __naked void {
    __naked void to_stack_check_low_3(void)
    {
    asm volatile ("					\
    r1 = r10;					\
    r1 += -513;					\
    r0 = 42;					\
// (u8*)(r1 + 0) = r0;				\
    r0 = *(u8*)(r1 + 0);				\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("PTR_TO_STACK check low 4")
#[no_mangle]
pub unsafe extern "C" fn __msg(pointer": "math between fp) -> __failure {
    __failure __msg("math between fp pointer")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn to_stack_check_low_4() -> __naked void {
    __naked void to_stack_check_low_4(void)
    {
    asm volatile ("					\
    r1 = r10;					\
    r1 += %[int_min];				\
    r0 = 42;					\
// (u8*)(r1 + 0) = r0;				\
    r0 = *(u8*)(r1 + 0);				\
    exit;						\
    "	:
    : __imm_const(int_min, INT_MIN)
    : __clobber_all);
    }
    SEC("socket")
    __description("PTR_TO_STACK check low 5")
#[no_mangle]
pub unsafe extern "C" fn __msg(stack": "invalid write to) -> __failure {
    __failure __msg("invalid write to stack")
    __msg_unpriv("R1 stack pointer arithmetic goes out of range")
#[no_mangle]
pub unsafe extern "C" fn to_stack_check_low_5() -> __naked void {
    __naked void to_stack_check_low_5(void)
    {
    asm volatile ("					\
    r1 = r10;					\
    r1 += %[__imm_0];				\
    r0 = 42;					\
// (u8*)(r1 + 0) = r0;				\
    r0 = *(u8*)(r1 + 0);				\
    exit;						\
    "	:
    : __imm_const(__imm_0, -((1 << 29) - 1))
    : __clobber_all);
    }
    SEC("socket")
    __description("PTR_TO_STACK check low 6")
#[no_mangle]
pub unsafe extern "C" fn __msg(stack": "invalid write to) -> __failure {
    __failure __msg("invalid write to stack")
    __msg_unpriv("R1 stack pointer arithmetic goes out of range")
#[no_mangle]
pub unsafe extern "C" fn to_stack_check_low_6() -> __naked void {
    __naked void to_stack_check_low_6(void)
    {
    asm volatile ("					\
    r1 = r10;					\
    r1 += %[__imm_0];				\
    r0 = 42;					\
// (u8*)(r1  %[shrt_min]) = r0;			\
    r0 = *(u8*)(r1  %[shrt_min]);			\
    exit;						\
    "	:
    : __imm_const(__imm_0, -((1 << 29) - 1)),
    __imm_const(shrt_min, SHRT_MIN)
    : __clobber_all);
    }
    SEC("socket")
    __description("PTR_TO_STACK check low 7")
#[no_mangle]
pub unsafe extern "C" fn __msg(offset": "fp pointer) -> __failure {
    __failure __msg("fp pointer offset")
    __msg_unpriv("R1 stack pointer arithmetic goes out of range")
#[no_mangle]
pub unsafe extern "C" fn to_stack_check_low_7() -> __naked void {
    __naked void to_stack_check_low_7(void)
    {
    asm volatile ("					\
    r1 = r10;					\
    r1 += %[__imm_0];				\
    r1 += %[__imm_0];				\
    r0 = 42;					\
// (u8*)(r1  %[shrt_min]) = r0;			\
    r0 = *(u8*)(r1  %[shrt_min]);			\
    exit;						\
    "	:
    : __imm_const(__imm_0, -((1 << 29) - 1)),
    __imm_const(shrt_min, SHRT_MIN)
    : __clobber_all);
    }
    SEC("socket")
    __description("PTR_TO_STACK mixed reg/k, 1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 42) -> __success __success_unpriv {
    __success __success_unpriv __retval(42)
#[no_mangle]
pub unsafe extern "C" fn stack_mixed_reg_k_1() -> __naked void {
    __naked void stack_mixed_reg_k_1(void)
    {
    asm volatile ("					\
    r1 = r10;					\
    r1 += -3;					\
    r2 = -3;					\
    r1 += r2;					\
    r0 = 42;					\
// (u8*)(r1 + 0) = r0;				\
    r0 = *(u8*)(r1 + 0);				\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("PTR_TO_STACK mixed reg/k, 2")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 42) -> __success __success_unpriv {
    __success __success_unpriv __retval(42)
#[no_mangle]
pub unsafe extern "C" fn stack_mixed_reg_k_2() -> __naked void {
    __naked void stack_mixed_reg_k_2(void)
    {
    asm volatile ("					\
    r0 = 0;						\
// (u64*)(r10 - 8) = r0;				\
    r0 = 0;						\
// (u64*)(r10 - 16) = r0;				\
    r1 = r10;					\
    r1 += -3;					\
    r2 = -3;					\
    r1 += r2;					\
    r0 = 42;					\
// (u8*)(r1 + 0) = r0;				\
    r5 = r10;					\
    r0 = *(u8*)(r5 - 6);				\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("PTR_TO_STACK mixed reg/k, 3")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -3) -> __success __success_unpriv {
    __success __success_unpriv __retval(-3)
#[no_mangle]
pub unsafe extern "C" fn stack_mixed_reg_k_3() -> __naked void {
    __naked void stack_mixed_reg_k_3(void)
    {
    asm volatile ("					\
    r1 = r10;					\
    r1 += -3;					\
    r2 = -3;					\
    r1 += r2;					\
    r0 = 42;					\
// (u8*)(r1 + 0) = r0;				\
    r0 = r2;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("PTR_TO_STACK reg")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 42) -> __success __success_unpriv {
    __success __success_unpriv __retval(42)
#[no_mangle]
pub unsafe extern "C" fn ptr_to_stack_reg() -> __naked void {
    __naked void ptr_to_stack_reg(void)
    {
    asm volatile ("					\
    r1 = r10;					\
    r2 = -3;					\
    r1 += r2;					\
    r0 = 42;					\
// (u8*)(r1 + 0) = r0;				\
    r0 = *(u8*)(r1 + 0);				\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("stack pointer arithmetic")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn stack_pointer_arithmetic() -> __naked void {
    __naked void stack_pointer_arithmetic(void)
    {
    asm volatile ("					\
    r1 = 4;						\
    goto l0_%=;					\
    l0_%=:	r7 = r10;					\
    r7 += -10;					\
    r7 += -10;					\
    r2 = r7;					\
    r2 += r1;					\
    r0 = 0;						\
// (u32*)(r2 + 4) = r0;				\
    r2 = r7;					\
    r2 += 8;					\
    r0 = 0;						\
// (u32*)(r2 + 4) = r0;				\
    r0 = 0;						\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("tc")
    __description("store PTR_TO_STACK in R10 to array map using BPF_B")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 42) -> __success {
    __success __retval(42)
#[no_mangle]
pub unsafe extern "C" fn array_map_using_bpf_b() -> __naked void {
    __naked void array_map_using_bpf_b(void)
    {
    asm volatile ("					\
// Load pointer to map. */			\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_array_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 != 0 goto l0_%=;				\
    r0 = 2;						\
    exit;						\
    l0_%=:	r1 = r0;					\
// Copy R10 to R9. */				\
    r9 = r10;					\
// Pollute other registers with unaligned values. */\
    r2 = -1;					\
    r3 = -1;					\
    r4 = -1;					\
    r5 = -1;					\
    r6 = -1;					\
    r7 = -1;					\
    r8 = -1;					\
// Store both R9 and R10 with BPF_B and read back. */\
// (u8*)(r1 + 0) = r10;				\
    r2 = *(u8*)(r1 + 0);				\
// (u8*)(r1 + 0) = r9;				\
    r3 = *(u8*)(r1 + 0);				\
// Should read back as same value. */		\
    if r2 == r3 goto l1_%=;				\
    r0 = 1;						\
    exit;						\
    l1_%=:	r0 = 42;					\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_array_48b)
    : __clobber_all);
    }
    SEC("socket")
    __description("PTR_TO_STACK stack size > 512")
#[no_mangle]
pub unsafe extern "C" fn __msg(size=8": "invalid write to stack R1 off=-520) -> __failure {
    __failure __msg("invalid write to stack R1 off=-520 size=8")
#[no_mangle]
pub unsafe extern "C" fn stack_check_size_gt_512() -> __naked void {
    __naked void stack_check_size_gt_512(void)
    {
    asm volatile ("					\
    r1 = r10;					\
    r1 += -520;					\
    r0 = 42;					\
// (u64*)(r1 + 0) = r0;				\
    exit;						\
    "	::: __clobber_all);
    }

    SEC("socket")
    __description("PTR_TO_STACK stack size 512 with may_goto with jit")
    __load_if_JITed()
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 42) -> __success {
    __success __retval(42)
#[no_mangle]
pub unsafe extern "C" fn stack_check_size_512_with_may_goto_jit() -> __naked void {
    __naked void stack_check_size_512_with_may_goto_jit(void)
    {
    asm volatile ("					\
    r1 = r10;					\
    r1 += -512;					\
    r0 = 42;					\
// (u32*)(r1 + 0) = r0;				\
    may_goto l0_%=;					\
    r2 = 100;					\
    l0_%=:						\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("PTR_TO_STACK stack size 512 with may_goto without jit")
    __load_if_no_JITed()
#[no_mangle]
pub unsafe extern "C" fn __msg(large": "stack size 520(extra 8) is too) -> __failure {
    __failure __msg("stack size 520(extra 8) is too large")
#[no_mangle]
pub unsafe extern "C" fn stack_check_size_512_with_may_goto() -> __naked void {
    __naked void stack_check_size_512_with_may_goto(void)
    {
    asm volatile ("					\
    r1 = r10;					\
    r1 += -512;					\
    r0 = 42;					\
// (u32*)(r1 + 0) = r0;				\
    may_goto l0_%=;					\
    r2 = 100;					\
    l0_%=:						\
    exit;						\
    "	::: __clobber_all);
    }

    char _license[] SEC("license") = "GPL";
