//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_jeq_infer_not_null.c
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
// Converted from tools/testing/selftests/bpf/verifier/jeq_infer_not_null.c

    struct {
    __uint(type, BPF_MAP_TYPE_XSKMAP);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, int);
    } map_xskmap SEC(".maps");
// This is equivalent to the following program:
//
// r6 = skb->sk;
// r7 = sk_fullsock(r6);
// r0 = sk_fullsock(r6);
// if (r0 == 0) return 0;    (a)
// if (r0 != r7) return 0;   (b)
// *r7->type;                (c)
// return 0;
//
// It is safe to dereference r7 at point (c), because of (a) and (b).
// The test verifies that relation r0 == r7 is propagated from (b) to (c).
//
    SEC("cgroup/skb")
    __description("jne/jeq infer not null, PTR_TO_SOCKET_OR_NULL . PTR_TO_SOCKET for JNE false branch")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(comparison": "R7 pointer) -> __success __failure_unpriv {
    __success __failure_unpriv __msg_unpriv("R7 pointer comparison")
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn socket_for_jne_false_branch() -> __naked void {
    __naked void socket_for_jne_false_branch(void)
    {
    asm volatile ("					\
// r6 = skb->sk; */				\
    r6 = *(u64*)(r1 + %[__sk_buff_sk]);		\
// if (r6 == 0) return 0; */			\
    if r6 == 0 goto l0_%=;				\
// r7 = sk_fullsock(skb); */			\
    r1 = r6;					\
    call %[bpf_sk_fullsock];			\
    r7 = r0;					\
// r0 = sk_fullsock(skb); */			\
    r1 = r6;					\
    call %[bpf_sk_fullsock];			\
// if (r0 == null) return 0; */			\
    if r0 == 0 goto l0_%=;				\
// if (r0 == r7) r0 = *(r7->type); */		\
    if r0 != r7 goto l0_%=;		/* Use ! JNE ! */\
    r0 = *(u32*)(r7 + %[bpf_sock_type]);		\
    l0_%=:	/* return 0 */					\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_sk_fullsock),
    __imm_const(__sk_buff_sk, offsetof(struct __sk_buff, sk)),
    __imm_const(bpf_sock_type, offsetof(struct bpf_sock, type))
    : __clobber_all);
    }
// Same as above, but verify that another branch of JNE still
// prohibits access to PTR_MAYBE_NULL.
//
    SEC("cgroup/skb")
    __description("jne/jeq infer not null, PTR_TO_SOCKET_OR_NULL unchanged for JNE true branch")
#[no_mangle]
pub unsafe extern "C" fn __msg('sock_or_null'": "R7 invalid mem access) -> __failure {
    __failure __msg("R7 invalid mem access 'sock_or_null'")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(comparison": "R7 pointer) -> __failure_unpriv {
    __failure_unpriv __msg_unpriv("R7 pointer comparison")
#[no_mangle]
pub unsafe extern "C" fn unchanged_for_jne_true_branch() -> __naked void {
    __naked void unchanged_for_jne_true_branch(void)
    {
    asm volatile ("					\
// r6 = skb->sk */				\
    r6 = *(u64*)(r1 + %[__sk_buff_sk]);		\
// if (r6 == 0) return 0; */			\
    if r6 == 0 goto l0_%=;				\
// r7 = sk_fullsock(skb); */			\
    r1 = r6;					\
    call %[bpf_sk_fullsock];			\
    r7 = r0;					\
// r0 = sk_fullsock(skb); */			\
    r1 = r6;					\
    call %[bpf_sk_fullsock];			\
// if (r0 == null) return 0; */			\
    if r0 != 0 goto l0_%=;				\
// if (r0 == r7) return 0; */			\
    if r0 != r7 goto l1_%=;		/* Use ! JNE ! */\
    goto l0_%=;					\
    l1_%=:	/* r0 = *(r7.type); */				\
    r0 = *(u32*)(r7 + %[bpf_sock_type]);		\
    l0_%=:	/* return 0 */					\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_sk_fullsock),
    __imm_const(__sk_buff_sk, offsetof(struct __sk_buff, sk)),
    __imm_const(bpf_sock_type, offsetof(struct bpf_sock, type))
    : __clobber_all);
    }
// Same as a first test, but not null should be inferred for JEQ branch
    SEC("cgroup/skb")
    __description("jne/jeq infer not null, PTR_TO_SOCKET_OR_NULL . PTR_TO_SOCKET for JEQ true branch")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(comparison": "R7 pointer) -> __success __failure_unpriv {
    __success __failure_unpriv __msg_unpriv("R7 pointer comparison")
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn socket_for_jeq_true_branch() -> __naked void {
    __naked void socket_for_jeq_true_branch(void)
    {
    asm volatile ("					\
// r6 = skb->sk; */				\
    r6 = *(u64*)(r1 + %[__sk_buff_sk]);		\
// if (r6 == null) return 0; */			\
    if r6 == 0 goto l0_%=;				\
// r7 = sk_fullsock(skb); */			\
    r1 = r6;					\
    call %[bpf_sk_fullsock];			\
    r7 = r0;					\
// r0 = sk_fullsock(skb); */			\
    r1 = r6;					\
    call %[bpf_sk_fullsock];			\
// if (r0 == null) return 0; */			\
    if r0 == 0 goto l0_%=;				\
// if (r0 != r7) return 0; */			\
    if r0 == r7 goto l1_%=;		/* Use ! JEQ ! */\
    goto l0_%=;					\
    l1_%=:	/* r0 = *(r7.type); */				\
    r0 = *(u32*)(r7 + %[bpf_sock_type]);		\
    l0_%=:	/* return 0; */					\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_sk_fullsock),
    __imm_const(__sk_buff_sk, offsetof(struct __sk_buff, sk)),
    __imm_const(bpf_sock_type, offsetof(struct bpf_sock, type))
    : __clobber_all);
    }
// Same as above, but verify that another branch of JNE still
// prohibits access to PTR_MAYBE_NULL.
//
    SEC("cgroup/skb")
    __description("jne/jeq infer not null, PTR_TO_SOCKET_OR_NULL unchanged for JEQ false branch")
#[no_mangle]
pub unsafe extern "C" fn __msg('sock_or_null'": "R7 invalid mem access) -> __failure {
    __failure __msg("R7 invalid mem access 'sock_or_null'")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(comparison": "R7 pointer) -> __failure_unpriv {
    __failure_unpriv __msg_unpriv("R7 pointer comparison")
#[no_mangle]
pub unsafe extern "C" fn unchanged_for_jeq_false_branch() -> __naked void {
    __naked void unchanged_for_jeq_false_branch(void)
    {
    asm volatile ("					\
// r6 = skb->sk; */				\
    r6 = *(u64*)(r1 + %[__sk_buff_sk]);		\
// if (r6 == null) return 0; */			\
    if r6 == 0 goto l0_%=;				\
// r7 = sk_fullsock(skb); */			\
    r1 = r6;					\
    call %[bpf_sk_fullsock];			\
    r7 = r0;					\
// r0 = sk_fullsock(skb); */			\
    r1 = r6;					\
    call %[bpf_sk_fullsock];			\
// if (r0 == null) return 0; */			\
    if r0 == 0 goto l0_%=;				\
// if (r0 != r7) r0 = *(r7->type); */		\
    if r0 == r7 goto l0_%=;		/* Use ! JEQ ! */\
    r0 = *(u32*)(r7 + %[bpf_sock_type]);		\
    l0_%=:	/* return 0; */					\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_sk_fullsock),
    __imm_const(__sk_buff_sk, offsetof(struct __sk_buff, sk)),
    __imm_const(bpf_sock_type, offsetof(struct bpf_sock, type))
    : __clobber_all);
    }
// Maps are treated in a different branch of `mark_ptr_not_null_reg`,
// so separate test for maps case.
//
    SEC("xdp")
    __description("jne/jeq infer not null, PTR_TO_MAP_VALUE_OR_NULL . PTR_TO_MAP_VALUE")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn null_ptr_to_map_value() -> __naked void {
    __naked void null_ptr_to_map_value(void)
    {
    asm volatile ("					\
// r9 = &some stack to use as key */		\
    r1 = 0;						\
// (u32*)(r10 - 8) = r1;				\
    r9 = r10;					\
    r9 += -8;					\
// r8 = process local map */			\
    r8 = %[map_xskmap] ll;				\
// r6 = map_lookup_elem(r8, r9); */		\
    r1 = r8;					\
    r2 = r9;					\
    call %[bpf_map_lookup_elem];			\
    r6 = r0;					\
// r7 = map_lookup_elem(r8, r9); */		\
    r1 = r8;					\
    r2 = r9;					\
    call %[bpf_map_lookup_elem];			\
    r7 = r0;					\
// if (r6 == 0) return 0; */			\
    if r6 == 0 goto l0_%=;				\
// if (r6 != r7) return 0; */			\
    if r6 != r7 goto l0_%=;				\
// read *r7; */					\
    r0 = *(u32*)(r7 + %[bpf_xdp_sock_queue_id]);	\
    l0_%=:	/* return 0; */					\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_xskmap),
    __imm_const(bpf_xdp_sock_queue_id, offsetof(struct bpf_xdp_sock, queue_id))
    : __clobber_all);
    }
// Verified that we can detect the pointer as non_null when comparing with
// register with value 0. JEQ test case.
//
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
// to make sure the branch is not falsely predicted
    __msg("r0 = *(u32 *)(r0 +0)")
    __msg("from 7 to 9")
#[no_mangle]
pub unsafe extern "C" fn jeq_reg_reg_null_check() -> __naked void {
    __naked void jeq_reg_reg_null_check(void)
    {
    asm volatile ("                                 \
// (u32*)(r10 - 8) = 0;                           \
    r1 = %[map_xskmap] ll;                          \
    r2 = r10;                                       \
    r2 += -8;                                       \
    call %[bpf_map_lookup_elem];                    \
    r1 = 0;                                         \
    if r0 == r1 goto 1f;                            \
    r0 = *(u32*)(r0 +0);                            \
    1:      r0 = 0;                                         \
    exit;                                           \
    "       :
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_xskmap)
    : __clobber_all);
    }
// Same as above but for JNE.
//
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
// to make sure the branch is not falsely predicted
    __msg("r0 = *(u32 *)(r0 +0)")
    __msg("from 7 to 9")
#[no_mangle]
pub unsafe extern "C" fn jne_reg_reg_null_check() -> __naked void {
    __naked void jne_reg_reg_null_check(void)
    {
    asm volatile ("                                 \
// (u32*)(r10 - 8) = 0;                           \
    r1 = %[map_xskmap] ll;                          \
    r2 = r10;                                       \
    r2 += -8;                                       \
    call %[bpf_map_lookup_elem];                    \
    r1 = 0;                                         \
    if r0 != r1 goto 1f;                            \
    goto 2f;                                        \
    1:      r0 = *(u32*)(r0 +0);                            \
    2:      r0 = 0;                                         \
    exit;                                           \
    "       :
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_xskmap)
    : __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
