//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_set_retval.c
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

    SEC("lsm_cgroup/socket_create")
    __description("lsm_cgroup bpf_set_retval success")
    __success
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: lsm_cgroup_set_retval_zero_valid, family: c_int, type: c_int, protocol: c_int, kern: c_int) -> c_int {
    int BPF_PROG(lsm_cgroup_set_retval_zero_valid, int family, int type, int protocol, int kern)
    {
    bpf_set_retval(0);
    return 0;
    }
    SEC("lsm_cgroup/socket_create")
    __description("lsm_cgroup bpf_set_retval valid errno")
    __success
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: lsm_cgroup_set_retval_negative_valid, family: c_int, type: c_int, protocol: c_int, kern: c_int) -> c_int {
    int BPF_PROG(lsm_cgroup_set_retval_negative_valid, int family, int type, int protocol, int kern)
    {
    bpf_set_retval(-12);
    return 0;
    }
    SEC("lsm_cgroup/socket_create")
    __description("lsm_cgroup bpf_set_retval invalid negative value")
#[no_mangle]
pub unsafe extern "C" fn __msg([-4095: "should have been in, _arg: 0]") -> __failure {
    __failure __msg("should have been in [-4095, 0]")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: lsm_cgroup_set_retval_negative_invalid, family: c_int, type: c_int, protocol: c_int, kern: c_int) -> c_int {
    int BPF_PROG(lsm_cgroup_set_retval_negative_invalid, int family, int type, int protocol, int kern)
    {
    bpf_set_retval(-4096);
    return 0;
    }
    SEC("lsm_cgroup/socket_create")
    __description("lsm_cgroup bpf_set_retval invalid positive value")
#[no_mangle]
pub unsafe extern "C" fn __msg([-4095: "should have been in, _arg: 0]") -> __failure {
    __failure __msg("should have been in [-4095, 0]")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: lsm_cgroup_set_retval_positive_invalid, family: c_int, type: c_int, protocol: c_int, kern: c_int) -> c_int {
    int BPF_PROG(lsm_cgroup_set_retval_positive_invalid, int family, int type, int protocol, int kern)
    {
    bpf_set_retval(1);
    return 0;
    }
    SEC("cgroup/dev")
    __description("cgroup_device bpf_set_retval success")
    __success
#[no_mangle]
pub unsafe extern "C" fn cgroup_dev_set_retval_0(ctx: *mut bpf_cgroup_dev_ctx) -> c_int {
    int cgroup_dev_set_retval_0(struct bpf_cgroup_dev_ctx *ctx)
    {
    bpf_set_retval(0);
    return 1;
    }
    SEC("cgroup/dev")
    __description("cgroup_device bpf_set_retval valid errno")
    __success
#[no_mangle]
pub unsafe extern "C" fn cgroup_dev_set_retval_neg_maxerrno(ctx: *mut bpf_cgroup_dev_ctx) -> c_int {
    int cgroup_dev_set_retval_neg_maxerrno(struct bpf_cgroup_dev_ctx *ctx)
    {
    bpf_set_retval(-4095);
    return 1;
    }
    SEC("cgroup/dev")
    __description("cgroup_device bpf_set_retval invalid positive value")
#[no_mangle]
pub unsafe extern "C" fn __msg([-4095: "should have been in, _arg: 0]") -> __failure {
    __failure __msg("should have been in [-4095, 0]")
#[no_mangle]
pub unsafe extern "C" fn cgroup_dev_set_retval_1(ctx: *mut bpf_cgroup_dev_ctx) -> c_int {
    int cgroup_dev_set_retval_1(struct bpf_cgroup_dev_ctx *ctx)
    {
    bpf_set_retval(1);
    return 1;
    }
    SEC("cgroup/dev")
    __description("cgroup_device bpf_set_retval invalid negative value")
#[no_mangle]
pub unsafe extern "C" fn __msg([-4095: "should have been in, _arg: 0]") -> __failure {
    __failure __msg("should have been in [-4095, 0]")
#[no_mangle]
pub unsafe extern "C" fn cgroup_dev_set_retval_neg_4096(ctx: *mut bpf_cgroup_dev_ctx) -> c_int {
    int cgroup_dev_set_retval_neg_4096(struct bpf_cgroup_dev_ctx *ctx)
    {
    bpf_set_retval(-4096);
    return 1;
    }
    SEC("cgroup/dev")
    __description("bpf_set_retval bounds check survives state pruning")
#[no_mangle]
pub unsafe extern "C" fn __msg([-4095: "should have been in, _arg: 0]") -> __failure {
    __failure __msg("should have been in [-4095, 0]")
#[no_mangle]
pub unsafe extern "C" fn cgroup_dev_set_retval_pruning_bypass(ctx: *mut bpf_cgroup_dev_ctx) -> __naked int {
    __naked int cgroup_dev_set_retval_pruning_bypass(struct bpf_cgroup_dev_ctx *ctx)
    {
    asm volatile (
    "call %[bpf_get_prandom_u32];"
    "if r0 != 0 goto 1f;"
    "r0 = r0;"
    "r0 = r0;"
    "r0 = r0;"
    "r0 = r0;"
    "goto 2f;"
    "1:"
    "call %[bpf_get_prandom_u32];"
    "2:"
    "r1 = r0;"
    "call %[bpf_set_retval];"
    "r0 = 1;"
    "exit;"
    :
    : __imm(bpf_get_prandom_u32),
    __imm(bpf_set_retval)
    : __clobber_common
    );
    }
    char _license[] SEC("license") = "GPL";
