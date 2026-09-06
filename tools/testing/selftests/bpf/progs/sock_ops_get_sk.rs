//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/sock_ops_get_sk.c
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

//
// Test the SOCK_OPS_GET_SK() and SOCK_OPS_GET_FIELD() macros in
// sock_ops_convert_ctx_access() when dst_reg == src_reg.
//
// When dst_reg == src_reg, the macros borrow a temporary register to load
// is_fullsock / is_locked_tcp_sock, because dst_reg holds the ctx pointer
// and cannot be clobbered before ctx->sk / ctx->field is read. If
// is_fullsock == 0 (e.g., TCP_NEW_SYN_RECV with a request_sock), the macro
// must still zero dst_reg so the verifier's PTR_TO_SOCKET_OR_NULL
// SCALAR_VALUE type is correct at runtime. A missing clear leaves a stale
// ctx pointer in dst_reg that passes NULL checks (GET_SK) or leaks a kernel
// address as a scalar (GET_FIELD).
//
// When dst_reg != src_reg, dst_reg itself is used to load is_fullsock, so
// the JEQ (dst_reg == 0) naturally leaves it zeroed on the !fullsock path.
//
    int bug_detected;
    int null_seen;
    SEC("sockops")
#[no_mangle]
pub unsafe extern "C" fn sock_ops_get_sk_same_reg() -> __naked void {
    __naked void sock_ops_get_sk_same_reg(void)
    {
    asm volatile (
    "r7 = *(u32 *)(r1 + %[is_fullsock_off]);"
    "r1 = *(u64 *)(r1 + %[sk_off]);"
    "if r7 != 0 goto 2f;"
    "if r1 == 0 goto 1f;"
    "r1 = %[bug_detected] ll;"
    "r2 = 1;"
    "*(u32 *)(r1 + 0) = r2;"
    "goto 2f;"
    "1:"
    "r1 = %[null_seen] ll;"
    "r2 = 1;"
    "*(u32 *)(r1 + 0) = r2;"
    "2:"
    "r0 = 1;"
    "exit;"
    :
    : __imm_const(is_fullsock_off, offsetof(struct bpf_sock_ops, is_fullsock)),
    __imm_const(sk_off, offsetof(struct bpf_sock_ops, sk)),
    __imm_addr(bug_detected),
    __imm_addr(null_seen)
    : __clobber_all);
    }
// SOCK_OPS_GET_FIELD: same-register, is_locked_tcp_sock == 0 path.
    int field_bug_detected;
    int field_null_seen;
    SEC("sockops")
#[no_mangle]
pub unsafe extern "C" fn sock_ops_get_field_same_reg() -> __naked void {
    __naked void sock_ops_get_field_same_reg(void)
    {
    asm volatile (
    "r7 = *(u32 *)(r1 + %[is_fullsock_off]);"
    "r1 = *(u32 *)(r1 + %[snd_cwnd_off]);"
    "if r7 != 0 goto 2f;"
    "if r1 == 0 goto 1f;"
    "r1 = %[field_bug_detected] ll;"
    "r2 = 1;"
    "*(u32 *)(r1 + 0) = r2;"
    "goto 2f;"
    "1:"
    "r1 = %[field_null_seen] ll;"
    "r2 = 1;"
    "*(u32 *)(r1 + 0) = r2;"
    "2:"
    "r0 = 1;"
    "exit;"
    :
    : __imm_const(is_fullsock_off, offsetof(struct bpf_sock_ops, is_fullsock)),
    __imm_const(snd_cwnd_off, offsetof(struct bpf_sock_ops, snd_cwnd)),
    __imm_addr(field_bug_detected),
    __imm_addr(field_null_seen)
    : __clobber_all);
    }
// SOCK_OPS_GET_SK: different-register, is_fullsock == 0 path.
    int diff_reg_bug_detected;
    int diff_reg_null_seen;
    SEC("sockops")
#[no_mangle]
pub unsafe extern "C" fn sock_ops_get_sk_diff_reg() -> __naked void {
    __naked void sock_ops_get_sk_diff_reg(void)
    {
    asm volatile (
    "r7 = r1;"
    "r6 = *(u32 *)(r7 + %[is_fullsock_off]);"
    "r2 = *(u64 *)(r7 + %[sk_off]);"
    "if r6 != 0 goto 2f;"
    "if r2 == 0 goto 1f;"
    "r1 = %[diff_reg_bug_detected] ll;"
    "r3 = 1;"
    "*(u32 *)(r1 + 0) = r3;"
    "goto 2f;"
    "1:"
    "r1 = %[diff_reg_null_seen] ll;"
    "r3 = 1;"
    "*(u32 *)(r1 + 0) = r3;"
    "2:"
    "r0 = 1;"
    "exit;"
    :
    : __imm_const(is_fullsock_off, offsetof(struct bpf_sock_ops, is_fullsock)),
    __imm_const(sk_off, offsetof(struct bpf_sock_ops, sk)),
    __imm_addr(diff_reg_bug_detected),
    __imm_addr(diff_reg_null_seen)
    : __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
