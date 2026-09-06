//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_lsm.c
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

    SEC("lsm/file_permission")
    __description("lsm bpf prog with -4095~0 retval. test 1")
    __success
#[no_mangle]
pub unsafe extern "C" fn errno_zero_retval_test1(ctx: *mut c_void) -> __naked int {
    __naked int errno_zero_retval_test1(void *ctx)
    {
    asm volatile (
    "r0 = 0;"
    "exit;"
    ::: __clobber_all);
    }
    SEC("lsm/file_permission")
    __description("lsm bpf prog with -4095~0 retval. test 2")
    __success
#[no_mangle]
pub unsafe extern "C" fn errno_zero_retval_test2(ctx: *mut c_void) -> __naked int {
    __naked int errno_zero_retval_test2(void *ctx)
    {
    asm volatile (
    "r0 = -4095;"
    "exit;"
    ::: __clobber_all);
    }
    SEC("lsm/file_mprotect")
    __description("lsm bpf prog with -4095~0 retval. test 4")
#[no_mangle]
pub unsafe extern "C" fn __msg([-4095: "R0 has smin=-4096 smax=-4096 should have been in, _arg: 0]") -> __failure {
    __failure __msg("R0 has smin=-4096 smax=-4096 should have been in [-4095, 0]")
#[no_mangle]
pub unsafe extern "C" fn errno_zero_retval_test4(ctx: *mut c_void) -> __naked int {
    __naked int errno_zero_retval_test4(void *ctx)
    {
    asm volatile (
    "r0 = -4096;"
    "exit;"
    ::: __clobber_all);
    }
    SEC("lsm/file_mprotect")
    __description("lsm bpf prog with -4095~0 retval. test 5")
#[no_mangle]
pub unsafe extern "C" fn __msg([-4095: "R0 has smin=4096 smax=4096 should have been in, _arg: 0]") -> __failure {
    __failure __msg("R0 has smin=4096 smax=4096 should have been in [-4095, 0]")
#[no_mangle]
pub unsafe extern "C" fn errno_zero_retval_test5(ctx: *mut c_void) -> __naked int {
    __naked int errno_zero_retval_test5(void *ctx)
    {
    asm volatile (
    "r0 = 4096;"
    "exit;"
    ::: __clobber_all);
    }
    SEC("lsm/file_mprotect")
    __description("lsm bpf prog with -4095~0 retval. test 6")
#[no_mangle]
pub unsafe extern "C" fn __msg([-4095: "R0 has smin=1 smax=1 should have been in, _arg: 0]") -> __failure {
    __failure __msg("R0 has smin=1 smax=1 should have been in [-4095, 0]")
#[no_mangle]
pub unsafe extern "C" fn errno_zero_retval_test6(ctx: *mut c_void) -> __naked int {
    __naked int errno_zero_retval_test6(void *ctx)
    {
    asm volatile (
    "r0 = 1;"
    "exit;"
    ::: __clobber_all);
    }
    SEC("lsm/audit_rule_known")
    __description("lsm bpf prog with bool retval. test 1")
    __success
#[no_mangle]
pub unsafe extern "C" fn bool_retval_test1(ctx: *mut c_void) -> __naked int {
    __naked int bool_retval_test1(void *ctx)
    {
    asm volatile (
    "r0 = 1;"
    "exit;"
    ::: __clobber_all);
    }
    SEC("lsm/audit_rule_known")
    __description("lsm bpf prog with bool retval. test 2")
    __success
    __success
#[no_mangle]
pub unsafe extern "C" fn bool_retval_test2(ctx: *mut c_void) -> __naked int {
    __naked int bool_retval_test2(void *ctx)
    {
    asm volatile (
    "r0 = 0;"
    "exit;"
    ::: __clobber_all);
    }
    SEC("lsm/audit_rule_known")
    __description("lsm bpf prog with bool retval. test 3")
#[no_mangle]
pub unsafe extern "C" fn __msg([0: "R0 has smin=-1 smax=-1 should have been in, _arg: 1]") -> __failure {
    __failure __msg("R0 has smin=-1 smax=-1 should have been in [0, 1]")
#[no_mangle]
pub unsafe extern "C" fn bool_retval_test3(ctx: *mut c_void) -> __naked int {
    __naked int bool_retval_test3(void *ctx)
    {
    asm volatile (
    "r0 = -1;"
    "exit;"
    ::: __clobber_all);
    }
    SEC("lsm/audit_rule_known")
    __description("lsm bpf prog with bool retval. test 4")
#[no_mangle]
pub unsafe extern "C" fn __msg([0: "R0 has smin=2 smax=2 should have been in, _arg: 1]") -> __failure {
    __failure __msg("R0 has smin=2 smax=2 should have been in [0, 1]")
#[no_mangle]
pub unsafe extern "C" fn bool_retval_test4(ctx: *mut c_void) -> __naked int {
    __naked int bool_retval_test4(void *ctx)
    {
    asm volatile (
    "r0 = 2;"
    "exit;"
    ::: __clobber_all);
    }
    SEC("lsm/file_free_security")
    __success
    __description("lsm bpf prog with void retval. test 1")
#[no_mangle]
pub unsafe extern "C" fn void_retval_test1(ctx: *mut c_void) -> __naked int {
    __naked int void_retval_test1(void *ctx)
    {
    asm volatile (
    "r0 = -4096;"
    "exit;"
    ::: __clobber_all);
    }
    SEC("lsm/file_free_security")
    __success
    __description("lsm bpf prog with void retval. test 2")
#[no_mangle]
pub unsafe extern "C" fn void_retval_test2(ctx: *mut c_void) -> __naked int {
    __naked int void_retval_test2(void *ctx)
    {
    asm volatile (
    "r0 = 4096;"
    "exit;"
    ::: __clobber_all);
    }
    SEC("lsm/getprocattr")
    __description("lsm disabled hook: getprocattr")
#[no_mangle]
pub unsafe extern "C" fn __msg(hook": "points to disabled) -> __failure {
    __failure __msg("points to disabled hook")
#[no_mangle]
pub unsafe extern "C" fn disabled_hook_test1(ctx: *mut c_void) -> __naked int {
    __naked int disabled_hook_test1(void *ctx)
    {
    asm volatile (
    "r0 = 0;"
    "exit;"
    ::: __clobber_all);
    }
    SEC("lsm/setprocattr")
    __description("lsm disabled hook: setprocattr")
#[no_mangle]
pub unsafe extern "C" fn __msg(hook": "points to disabled) -> __failure {
    __failure __msg("points to disabled hook")
#[no_mangle]
pub unsafe extern "C" fn disabled_hook_test2(ctx: *mut c_void) -> __naked int {
    __naked int disabled_hook_test2(void *ctx)
    {
    asm volatile (
    "r0 = 0;"
    "exit;"
    ::: __clobber_all);
    }
    SEC("lsm/ismaclabel")
    __description("lsm disabled hook: ismaclabel")
#[no_mangle]
pub unsafe extern "C" fn __msg(hook": "points to disabled) -> __failure {
    __failure __msg("points to disabled hook")
#[no_mangle]
pub unsafe extern "C" fn disabled_hook_test3(ctx: *mut c_void) -> __naked int {
    __naked int disabled_hook_test3(void *ctx)
    {
    asm volatile (
    "r0 = 0;"
    "exit;"
    ::: __clobber_all);
    }
    SEC("lsm/mmap_file")
    __description("not null checking nullable pointer in bpf_lsm_mmap_file")
#[no_mangle]
pub unsafe extern "C" fn __msg('trusted_ptr_or_null_'": "R1 invalid mem access) -> __failure {
    __failure __msg("R1 invalid mem access 'trusted_ptr_or_null_'")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: no_null_check, file: *mut file) -> c_int {
    int BPF_PROG(no_null_check, struct file *file)
    {
    struct inode *inode;
    inode = file.f_inode;
    __sink(inode);
    return 0;
    }
    SEC("lsm/mmap_file")
    __description("null checking nullable pointer in bpf_lsm_mmap_file")
    __success
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: null_check, file: *mut file) -> c_int {
    int BPF_PROG(null_check, struct file *file)
    {
    struct inode *inode;
    if (file) {
    inode = file.f_inode;
    __sink(inode);
    }
    return 0;
    }
    SEC("lsm_cgroup/file_open")
    __description("sleepable lsm_cgroup program is rejected")
#[no_mangle]
pub unsafe extern "C" fn __msg(sleepable": "Program of this type cannot be) -> __failure {
    __failure __msg("Program of this type cannot be sleepable")
    __flag(BPF_F_SLEEPABLE)
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: sleepable_lsm_cgroup) -> c_int {
    int BPF_PROG(sleepable_lsm_cgroup)
    {
    return 0;
    }
    SEC("lsm/file_mprotect")
    __description("lsm retval load must reset stale register bounds")
#[no_mangle]
pub unsafe extern "C" fn __msg(zero": "div by) -> __failure {
    __failure __msg("div by zero")
#[no_mangle]
pub unsafe extern "C" fn retval_load_resets_bounds(ctx: *mut c_void) -> __naked int {
    __naked int retval_load_resets_bounds(void *ctx)
    {
    asm volatile (
    "r6 = 0;"
    "r6 = *(u64 *)(r1 + 24);"
    "if r6 == 0 goto +1;"
    "r6 /= 0;"
    "r0 = 0;"
    "exit;"
    ::: __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
