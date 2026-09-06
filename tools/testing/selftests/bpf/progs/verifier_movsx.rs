//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_movsx.c
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

    (defined(__TARGET_ARCH_riscv) && __riscv_xlen == 64) || \
    defined(__TARGET_ARCH_arm) || defined(__TARGET_ARCH_s390) || \
    defined(__TARGET_ARCH_loongarch)) && \
    __clang_major__ >= 18
    SEC("socket")
    __description("MOV32SX, S8")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0x23) -> __success __success_unpriv {
    __success __success_unpriv __retval(0x23)
#[no_mangle]
pub unsafe extern "C" fn mov32sx_s8() -> __naked void {
    __naked void mov32sx_s8(void)
    {
    asm volatile ("					\
    w0 = 0xff23;					\
    w0 = (s8)w0;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("MOV32SX, S16")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0xFFFFff23) -> __success __success_unpriv {
    __success __success_unpriv __retval(0xFFFFff23)
#[no_mangle]
pub unsafe extern "C" fn mov32sx_s16() -> __naked void {
    __naked void mov32sx_s16(void)
    {
    asm volatile ("					\
    w0 = 0xff23;					\
    w0 = (s16)w0;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("MOV64SX, S8")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -2) -> __success __success_unpriv {
    __success __success_unpriv __retval(-2)
#[no_mangle]
pub unsafe extern "C" fn mov64sx_s8() -> __naked void {
    __naked void mov64sx_s8(void)
    {
    asm volatile ("					\
    r0 = 0x1fe;					\
    r0 = (s8)r0;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("MOV64SX, S16")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0xf23) -> __success __success_unpriv {
    __success __success_unpriv __retval(0xf23)
#[no_mangle]
pub unsafe extern "C" fn mov64sx_s16() -> __naked void {
    __naked void mov64sx_s16(void)
    {
    asm volatile ("					\
    r0 = 0xf0f23;					\
    r0 = (s16)r0;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("MOV64SX, S32")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -1) -> __success __success_unpriv {
    __success __success_unpriv __retval(-1)
#[no_mangle]
pub unsafe extern "C" fn mov64sx_s32() -> __naked void {
    __naked void mov64sx_s32(void)
    {
    asm volatile ("					\
    r0 = 0xfffffffe;				\
    r0 = (s32)r0;					\
    r0 >>= 1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("MOV32SX, S8, range_check")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 1) -> __success __success_unpriv {
    __success __success_unpriv __retval(1)
#[no_mangle]
pub unsafe extern "C" fn mov32sx_s8_range() -> __naked void {
    __naked void mov32sx_s8_range(void)
    {
    asm volatile ("					\
    call %[bpf_get_prandom_u32];			\
    w1 = (s8)w0;					\
// w1 with s8 range */				\
    if w1 s> 0x7f goto l0_%=;			\
    if w1 s< -0x80 goto l0_%=;			\
    r0 = 1;						\
    l1_%=:							\
    exit;						\
    l0_%=:							\
    r0 = 2;						\
    goto l1_%=;					\
    "	:
    : __imm(bpf_get_prandom_u32)
    : __clobber_all);
    }
    SEC("socket")
    __description("MOV32SX, S16, range_check")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 1) -> __success __success_unpriv {
    __success __success_unpriv __retval(1)
#[no_mangle]
pub unsafe extern "C" fn mov32sx_s16_range() -> __naked void {
    __naked void mov32sx_s16_range(void)
    {
    asm volatile ("					\
    call %[bpf_get_prandom_u32];			\
    w1 = (s16)w0;					\
// w1 with s16 range */				\
    if w1 s> 0x7fff goto l0_%=;			\
    if w1 s< -0x80ff goto l0_%=;			\
    r0 = 1;						\
    l1_%=:							\
    exit;						\
    l0_%=:							\
    r0 = 2;						\
    goto l1_%=;					\
    "	:
    : __imm(bpf_get_prandom_u32)
    : __clobber_all);
    }
    SEC("socket")
    __description("MOV32SX, S16, range_check 2")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 1) -> __success __success_unpriv {
    __success __success_unpriv __retval(1)
#[no_mangle]
pub unsafe extern "C" fn mov32sx_s16_range_2() -> __naked void {
    __naked void mov32sx_s16_range_2(void)
    {
    asm volatile ("					\
    r1 = 65535;					\
    w2 = (s16)w1;					\
    r2 >>= 1;					\
    if r2 != 0x7fffFFFF goto l0_%=;			\
    r0 = 1;						\
    l1_%=:							\
    exit;						\
    l0_%=:							\
    r0 = 0;						\
    goto l1_%=;					\
    "	:
    : __imm(bpf_get_prandom_u32)
    : __clobber_all);
    }
    SEC("socket")
    __description("MOV64SX, S8, range_check")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 1) -> __success __success_unpriv {
    __success __success_unpriv __retval(1)
#[no_mangle]
pub unsafe extern "C" fn mov64sx_s8_range() -> __naked void {
    __naked void mov64sx_s8_range(void)
    {
    asm volatile ("					\
    call %[bpf_get_prandom_u32];			\
    r1 = (s8)r0;					\
// r1 with s8 range */				\
    if r1 s> 0x7f goto l0_%=;			\
    if r1 s< -0x80 goto l0_%=;			\
    r0 = 1;						\
    l1_%=:							\
    exit;						\
    l0_%=:							\
    r0 = 2;						\
    goto l1_%=;					\
    "	:
    : __imm(bpf_get_prandom_u32)
    : __clobber_all);
    }
    SEC("socket")
    __description("MOV64SX, S16, range_check")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 1) -> __success __success_unpriv {
    __success __success_unpriv __retval(1)
#[no_mangle]
pub unsafe extern "C" fn mov64sx_s16_range() -> __naked void {
    __naked void mov64sx_s16_range(void)
    {
    asm volatile ("					\
    call %[bpf_get_prandom_u32];			\
    r1 = (s16)r0;					\
// r1 with s16 range */				\
    if r1 s> 0x7fff goto l0_%=;			\
    if r1 s< -0x8000 goto l0_%=;			\
    r0 = 1;						\
    l1_%=:							\
    exit;						\
    l0_%=:							\
    r0 = 2;						\
    goto l1_%=;					\
    "	:
    : __imm(bpf_get_prandom_u32)
    : __clobber_all);
    }
    SEC("socket")
    __description("MOV64SX, S32, range_check")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 1) -> __success __success_unpriv {
    __success __success_unpriv __retval(1)
#[no_mangle]
pub unsafe extern "C" fn mov64sx_s32_range() -> __naked void {
    __naked void mov64sx_s32_range(void)
    {
    asm volatile ("					\
    call %[bpf_get_prandom_u32];			\
    r1 = (s32)r0;					\
// r1 with s32 range */				\
    if r1 s> 0x7fffffff goto l0_%=;			\
    if r1 s< -0x80000000 goto l0_%=;		\
    r0 = 1;						\
    l1_%=:							\
    exit;						\
    l0_%=:							\
    r0 = 2;						\
    goto l1_%=;					\
    "	:
    : __imm(bpf_get_prandom_u32)
    : __clobber_all);
    }
    SEC("socket")
    __description("MOV64SX, S16, R10 Sign Extension")
#[no_mangle]
pub unsafe extern "C" fn __msg(expected=fp: "R1 type=scalar, _arg: pkt, _arg: pkt_meta, _arg: map_key, _arg: map_value, _arg: mem, _arg: ringbuf_mem, _arg: buf, _arg: trusted_ptr_") -> __failure {
    __failure __msg("R1 type=scalar expected=fp, pkt, pkt_meta, map_key, map_value, mem, ringbuf_mem, buf, trusted_ptr_")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(pointer": "R10 sign-extension part of) -> __failure_unpriv {
    __failure_unpriv __msg_unpriv("R10 sign-extension part of pointer")
#[no_mangle]
pub unsafe extern "C" fn mov64sx_s16_r10() -> __naked void {
    __naked void mov64sx_s16_r10(void)
    {
    asm volatile ("					\
    r1 = 553656332;					\
// (u32 *)(r10 - 8) = r1; 			\
    r1 = (s16)r10;					\
    r1 += -8;					\
    r2 = 3;						\
    if r2 <= r1 goto l0_%=;				\
    l0_%=:							\
    call %[bpf_trace_printk];			\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_trace_printk)
    : __clobber_all);
    }
    SEC("socket")
    __description("MOV32SX, S8, var_off u32_max")
#[no_mangle]
pub unsafe extern "C" fn __msg(detected": "infinite loop) -> __failure {
    __failure __msg("infinite loop detected")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(0": "back-edge from insn 2 to) -> __failure_unpriv {
    __failure_unpriv __msg_unpriv("back-edge from insn 2 to 0")
#[no_mangle]
pub unsafe extern "C" fn mov64sx_s32_varoff_1() -> __naked void {
    __naked void mov64sx_s32_varoff_1(void)
    {
    asm volatile ("					\
    l0_%=:							\
    r3 = *(u8 *)(r10 -387);				\
    w7 = (s8)w3;					\
    if w7 >= 0x2533823b goto l0_%=;			\
    w0 = 0;						\
    exit;						\
    "	:
    :
    : __clobber_all);
    }
    SEC("socket")
    __description("MOV32SX, S8, var_off not u32_max, positive after s8 extension")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
    __success_unpriv

    __xlated_unpriv("w0 = 0")
    __xlated_unpriv("exit")
    __xlated_unpriv("nospec") /* inserted to prevent `frame pointer is read only` */
    __xlated_unpriv("goto pc-1")

#[no_mangle]
pub unsafe extern "C" fn mov64sx_s32_varoff_2() -> __naked void {
    __naked void mov64sx_s32_varoff_2(void)
    {
    asm volatile ("					\
    call %[bpf_get_prandom_u32];			\
    r3 = r0;					\
    r3 &= 0xf;					\
    w7 = (s8)w3;					\
    if w7 s>= 16 goto l0_%=;			\
    w0 = 0;						\
    exit;						\
    l0_%=:							\
    r10 = 1;					\
    exit;						\
    "	:
    : __imm(bpf_get_prandom_u32)
    : __clobber_all);
    }
    SEC("socket")
    __description("MOV32SX, S8, var_off not u32_max, negative after s8 extension")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
    __success_unpriv

    __xlated_unpriv("w0 = 0")
    __xlated_unpriv("exit")
    __xlated_unpriv("nospec") /* inserted to prevent `frame pointer is read only` */
    __xlated_unpriv("goto pc-1")

#[no_mangle]
pub unsafe extern "C" fn mov64sx_s32_varoff_3() -> __naked void {
    __naked void mov64sx_s32_varoff_3(void)
    {
    asm volatile ("					\
    call %[bpf_get_prandom_u32];			\
    r3 = r0;					\
    r3 &= 0xf;					\
    r3 |= 0x80;					\
    w7 = (s8)w3;					\
    if w7 s>= -5 goto l0_%=;			\
    w0 = 0;						\
    exit;						\
    l0_%=:							\
    r10 = 1;					\
    exit;						\
    "	:
    : __imm(bpf_get_prandom_u32)
    : __clobber_all);
    }
    SEC("socket")
    __description("MOV64SX, S8, unsigned range_check")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn mov64sx_s8_range_check() -> __naked void {
    __naked void mov64sx_s8_range_check(void)
    {
    asm volatile ("					\
    call %[bpf_get_prandom_u32];			\
    r0 &= 0x1;					\
    r0 += 0xfe;					\
    r0 = (s8)r0;					\
    if r0 < 0xfffffffffffffffe goto label_%=;	\
    r0 = 0;						\
    exit;						\
    label_%=:						\
    exit;						\
    "	:
    : __imm(bpf_get_prandom_u32)
    : __clobber_all);
    }
    SEC("socket")
    __description("MOV32SX, S8, unsigned range_check")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn mov32sx_s8_range_check() -> __naked void {
    __naked void mov32sx_s8_range_check(void)
    {
    asm volatile ("                                 \
    call %[bpf_get_prandom_u32];                    \
    w0 &= 0x1;                                      \
    w0 += 0xfe;                                     \
    w0 = (s8)w0;                                    \
    if w0 < 0xfffffffe goto label_%=;               \
    r0 = 0;                                         \
    exit;                                           \
    label_%=: 	                                        \
    exit;                                           \
    "      :
    : __imm(bpf_get_prandom_u32)
    : __clobber_all);
    }

    SEC("socket")
    __description("cpuv4 is not supported by compiler or jit, use a dummy test")
    __success
#[no_mangle]
pub unsafe extern "C" fn dummy_test() -> c_int {
    int dummy_test(void)
    {
    return 0;
    }

    char _license[] SEC("license") = "GPL";
