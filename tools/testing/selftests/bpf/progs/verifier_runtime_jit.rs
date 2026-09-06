//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_runtime_jit.c
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
// Converted from tools/testing/selftests/bpf/verifier/runtime_jit.c

    void dummy_prog_42_socket(void);
    void dummy_prog_24_socket(void);
    void dummy_prog_loop1_socket(void);
    void dummy_prog_loop2_socket(void);
    struct {
    __uint(type, BPF_MAP_TYPE_PROG_ARRAY);
    __uint(max_entries, 4);
    __uint(key_size, sizeof(int));
    __array(values, void (void));
    } map_prog1_socket SEC(".maps") = {
    .values = {
    [0] = (void *)&dummy_prog_42_socket,
    [1] = (void *)&dummy_prog_loop1_socket,
    [2] = (void *)&dummy_prog_24_socket,
    },
    };
    struct {
    __uint(type, BPF_MAP_TYPE_PROG_ARRAY);
    __uint(max_entries, 8);
    __uint(key_size, sizeof(int));
    __array(values, void (void));
    } map_prog2_socket SEC(".maps") = {
    .values = {
    [1] = (void *)&dummy_prog_loop2_socket,
    [2] = (void *)&dummy_prog_24_socket,
    [7] = (void *)&dummy_prog_42_socket,
    },
    };
    SEC("socket")
    __auxiliary __auxiliary_unpriv
#[no_mangle]
pub unsafe extern "C" fn dummy_prog_42_socket() -> __naked void {
    __naked void dummy_prog_42_socket(void)
    {
    asm volatile ("r0 = 42; exit;");
    }
    SEC("socket")
    __auxiliary __auxiliary_unpriv
#[no_mangle]
pub unsafe extern "C" fn dummy_prog_24_socket() -> __naked void {
    __naked void dummy_prog_24_socket(void)
    {
    asm volatile ("r0 = 24; exit;");
    }
    SEC("socket")
    __auxiliary __auxiliary_unpriv
#[no_mangle]
pub unsafe extern "C" fn dummy_prog_loop1_socket() -> __naked void {
    __naked void dummy_prog_loop1_socket(void)
    {
    asm volatile ("			\
    r3 = 1;				\
    r2 = %[map_prog1_socket] ll;	\
    call %[bpf_tail_call];		\
    r0 = 41;			\
    exit;				\
    "	:
    : __imm(bpf_tail_call),
    __imm_addr(map_prog1_socket)
    : __clobber_all);
    }
    SEC("socket")
    __auxiliary __auxiliary_unpriv
#[no_mangle]
pub unsafe extern "C" fn dummy_prog_loop2_socket() -> __naked void {
    __naked void dummy_prog_loop2_socket(void)
    {
    asm volatile ("			\
    r3 = 1;				\
    r2 = %[map_prog2_socket] ll;	\
    call %[bpf_tail_call];		\
    r0 = 41;			\
    exit;				\
    "	:
    : __imm(bpf_tail_call),
    __imm_addr(map_prog2_socket)
    : __clobber_all);
    }
    SEC("socket")
    __description("runtime/jit: tail_call within bounds, prog once")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 42) -> __success __success_unpriv {
    __success __success_unpriv __retval(42)
#[no_mangle]
pub unsafe extern "C" fn call_within_bounds_prog_once() -> __naked void {
    __naked void call_within_bounds_prog_once(void)
    {
    asm volatile ("					\
    r3 = 0;						\
    r2 = %[map_prog1_socket] ll;			\
    call %[bpf_tail_call];				\
    r0 = 1;						\
    exit;						\
    "	:
    : __imm(bpf_tail_call),
    __imm_addr(map_prog1_socket)
    : __clobber_all);
    }
    SEC("socket")
    __description("runtime/jit: tail_call within bounds, prog loop")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 41) -> __success __success_unpriv {
    __success __success_unpriv __retval(41)
#[no_mangle]
pub unsafe extern "C" fn call_within_bounds_prog_loop() -> __naked void {
    __naked void call_within_bounds_prog_loop(void)
    {
    asm volatile ("					\
    r3 = 1;						\
    r2 = %[map_prog1_socket] ll;			\
    call %[bpf_tail_call];				\
    r0 = 1;						\
    exit;						\
    "	:
    : __imm(bpf_tail_call),
    __imm_addr(map_prog1_socket)
    : __clobber_all);
    }
    SEC("socket")
    __description("runtime/jit: tail_call within bounds, no prog")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 1) -> __success __success_unpriv {
    __success __success_unpriv __retval(1)
#[no_mangle]
pub unsafe extern "C" fn call_within_bounds_no_prog() -> __naked void {
    __naked void call_within_bounds_no_prog(void)
    {
    asm volatile ("					\
    r3 = 3;						\
    r2 = %[map_prog1_socket] ll;			\
    call %[bpf_tail_call];				\
    r0 = 1;						\
    exit;						\
    "	:
    : __imm(bpf_tail_call),
    __imm_addr(map_prog1_socket)
    : __clobber_all);
    }
    SEC("socket")
    __description("runtime/jit: tail_call within bounds, key 2")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 24) -> __success __success_unpriv {
    __success __success_unpriv __retval(24)
#[no_mangle]
pub unsafe extern "C" fn call_within_bounds_key_2() -> __naked void {
    __naked void call_within_bounds_key_2(void)
    {
    asm volatile ("					\
    r3 = 2;						\
    r2 = %[map_prog1_socket] ll;			\
    call %[bpf_tail_call];				\
    r0 = 1;						\
    exit;						\
    "	:
    : __imm(bpf_tail_call),
    __imm_addr(map_prog1_socket)
    : __clobber_all);
    }
    SEC("socket")
    __description("runtime/jit: tail_call within bounds, key 2 / key 2, first branch")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 24) -> __success __success_unpriv {
    __success __success_unpriv __retval(24)
#[no_mangle]
pub unsafe extern "C" fn _2_key_2_first_branch() -> __naked void {
    __naked void _2_key_2_first_branch(void)
    {
    asm volatile ("					\
    r0 = 13;					\
// (u8*)(r1 + %[__sk_buff_cb_0]) = r0;		\
    r0 = *(u8*)(r1 + %[__sk_buff_cb_0]);		\
    if r0 == 13 goto l0_%=;				\
    r3 = 2;						\
    r2 = %[map_prog1_socket] ll;			\
    goto l1_%=;					\
    l0_%=:	r3 = 2;						\
    r2 = %[map_prog1_socket] ll;			\
    l1_%=:	call %[bpf_tail_call];				\
    r0 = 1;						\
    exit;						\
    "	:
    : __imm(bpf_tail_call),
    __imm_addr(map_prog1_socket),
    __imm_const(__sk_buff_cb_0, offsetof(struct __sk_buff, cb[0]))
    : __clobber_all);
    }
    SEC("socket")
    __description("runtime/jit: tail_call within bounds, key 2 / key 2, second branch")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 24) -> __success __success_unpriv {
    __success __success_unpriv __retval(24)
#[no_mangle]
pub unsafe extern "C" fn _2_key_2_second_branch() -> __naked void {
    __naked void _2_key_2_second_branch(void)
    {
    asm volatile ("					\
    r0 = 14;					\
// (u8*)(r1 + %[__sk_buff_cb_0]) = r0;		\
    r0 = *(u8*)(r1 + %[__sk_buff_cb_0]);		\
    if r0 == 13 goto l0_%=;				\
    r3 = 2;						\
    r2 = %[map_prog1_socket] ll;			\
    goto l1_%=;					\
    l0_%=:	r3 = 2;						\
    r2 = %[map_prog1_socket] ll;			\
    l1_%=:	call %[bpf_tail_call];				\
    r0 = 1;						\
    exit;						\
    "	:
    : __imm(bpf_tail_call),
    __imm_addr(map_prog1_socket),
    __imm_const(__sk_buff_cb_0, offsetof(struct __sk_buff, cb[0]))
    : __clobber_all);
    }
    SEC("socket")
    __description("runtime/jit: tail_call within bounds, key 0 / key 2, first branch")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 24) -> __success __success_unpriv {
    __success __success_unpriv __retval(24)
#[no_mangle]
pub unsafe extern "C" fn _0_key_2_first_branch() -> __naked void {
    __naked void _0_key_2_first_branch(void)
    {
    asm volatile ("					\
    r0 = 13;					\
// (u8*)(r1 + %[__sk_buff_cb_0]) = r0;		\
    r0 = *(u8*)(r1 + %[__sk_buff_cb_0]);		\
    if r0 == 13 goto l0_%=;				\
    r3 = 0;						\
    r2 = %[map_prog1_socket] ll;			\
    goto l1_%=;					\
    l0_%=:	r3 = 2;						\
    r2 = %[map_prog1_socket] ll;			\
    l1_%=:	call %[bpf_tail_call];				\
    r0 = 1;						\
    exit;						\
    "	:
    : __imm(bpf_tail_call),
    __imm_addr(map_prog1_socket),
    __imm_const(__sk_buff_cb_0, offsetof(struct __sk_buff, cb[0]))
    : __clobber_all);
    }
    SEC("socket")
    __description("runtime/jit: tail_call within bounds, key 0 / key 2, second branch")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 42) -> __success __success_unpriv {
    __success __success_unpriv __retval(42)
#[no_mangle]
pub unsafe extern "C" fn _0_key_2_second_branch() -> __naked void {
    __naked void _0_key_2_second_branch(void)
    {
    asm volatile ("					\
    r0 = 14;					\
// (u8*)(r1 + %[__sk_buff_cb_0]) = r0;		\
    r0 = *(u8*)(r1 + %[__sk_buff_cb_0]);		\
    if r0 == 13 goto l0_%=;				\
    r3 = 0;						\
    r2 = %[map_prog1_socket] ll;			\
    goto l1_%=;					\
    l0_%=:	r3 = 2;						\
    r2 = %[map_prog1_socket] ll;			\
    l1_%=:	call %[bpf_tail_call];				\
    r0 = 1;						\
    exit;						\
    "	:
    : __imm(bpf_tail_call),
    __imm_addr(map_prog1_socket),
    __imm_const(__sk_buff_cb_0, offsetof(struct __sk_buff, cb[0]))
    : __clobber_all);
    }
    SEC("socket")
    __description("runtime/jit: tail_call within bounds, different maps, first branch")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(map_ptr": "tail_call abusing) -> __success __failure_unpriv {
    __success __failure_unpriv __msg_unpriv("tail_call abusing map_ptr")
    __retval(1)
#[no_mangle]
pub unsafe extern "C" fn bounds_different_maps_first_branch() -> __naked void {
    __naked void bounds_different_maps_first_branch(void)
    {
    asm volatile ("					\
    r0 = 13;					\
// (u8*)(r1 + %[__sk_buff_cb_0]) = r0;		\
    r0 = *(u8*)(r1 + %[__sk_buff_cb_0]);		\
    if r0 == 13 goto l0_%=;				\
    r3 = 0;						\
    r2 = %[map_prog1_socket] ll;			\
    goto l1_%=;					\
    l0_%=:	r3 = 0;						\
    r2 = %[map_prog2_socket] ll;			\
    l1_%=:	call %[bpf_tail_call];				\
    r0 = 1;						\
    exit;						\
    "	:
    : __imm(bpf_tail_call),
    __imm_addr(map_prog1_socket),
    __imm_addr(map_prog2_socket),
    __imm_const(__sk_buff_cb_0, offsetof(struct __sk_buff, cb[0]))
    : __clobber_all);
    }
    SEC("socket")
    __description("runtime/jit: tail_call within bounds, different maps, second branch")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(map_ptr": "tail_call abusing) -> __success __failure_unpriv {
    __success __failure_unpriv __msg_unpriv("tail_call abusing map_ptr")
    __retval(42)
#[no_mangle]
pub unsafe extern "C" fn bounds_different_maps_second_branch() -> __naked void {
    __naked void bounds_different_maps_second_branch(void)
    {
    asm volatile ("					\
    r0 = 14;					\
// (u8*)(r1 + %[__sk_buff_cb_0]) = r0;		\
    r0 = *(u8*)(r1 + %[__sk_buff_cb_0]);		\
    if r0 == 13 goto l0_%=;				\
    r3 = 0;						\
    r2 = %[map_prog1_socket] ll;			\
    goto l1_%=;					\
    l0_%=:	r3 = 0;						\
    r2 = %[map_prog2_socket] ll;			\
    l1_%=:	call %[bpf_tail_call];				\
    r0 = 1;						\
    exit;						\
    "	:
    : __imm(bpf_tail_call),
    __imm_addr(map_prog1_socket),
    __imm_addr(map_prog2_socket),
    __imm_const(__sk_buff_cb_0, offsetof(struct __sk_buff, cb[0]))
    : __clobber_all);
    }
    SEC("socket")
    __description("runtime/jit: tail_call out of bounds")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 2) -> __success __success_unpriv {
    __success __success_unpriv __retval(2)
#[no_mangle]
pub unsafe extern "C" fn tail_call_out_of_bounds() -> __naked void {
    __naked void tail_call_out_of_bounds(void)
    {
    asm volatile ("					\
    r3 = 256;					\
    r2 = %[map_prog1_socket] ll;			\
    call %[bpf_tail_call];				\
    r0 = 2;						\
    exit;						\
    "	:
    : __imm(bpf_tail_call),
    __imm_addr(map_prog1_socket)
    : __clobber_all);
    }
    SEC("socket")
    __description("runtime/jit: pass negative index to tail_call")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 2) -> __success __success_unpriv {
    __success __success_unpriv __retval(2)
#[no_mangle]
pub unsafe extern "C" fn negative_index_to_tail_call() -> __naked void {
    __naked void negative_index_to_tail_call(void)
    {
    asm volatile ("					\
    r3 = -1;					\
    r2 = %[map_prog1_socket] ll;			\
    call %[bpf_tail_call];				\
    r0 = 2;						\
    exit;						\
    "	:
    : __imm(bpf_tail_call),
    __imm_addr(map_prog1_socket)
    : __clobber_all);
    }
    SEC("socket")
    __description("runtime/jit: pass > 32bit index to tail_call")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 42) -> __success __success_unpriv {
    __success __success_unpriv __retval(42)
// Verifier rewrite for unpriv skips tail call here.
    __retval_unpriv(2)
#[no_mangle]
pub unsafe extern "C" fn _32bit_index_to_tail_call() -> __naked void {
    __naked void _32bit_index_to_tail_call(void)
    {
    asm volatile ("					\
    r3 = 0x100000000 ll;				\
    r2 = %[map_prog1_socket] ll;			\
    call %[bpf_tail_call];				\
    r0 = 2;						\
    exit;						\
    "	:
    : __imm(bpf_tail_call),
    __imm_addr(map_prog1_socket)
    : __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
