//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/arm64/abi/tpidr2.c
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


// SPDX-License-Identifier: GPL-2.0-only

pub const EXPECTED_TESTS: c_int = 5;
#[no_mangle]
unsafe extern "C" fn set_tpidr2(val: u64) {
    static void set_tpidr2(uint64_t val)
    {
    asm volatile (
    "msr	" SYS_TPIDR2 ", %0\n"
    :
    : "r"(val)
    : "cc");
    }
#[no_mangle]
unsafe extern "C" fn get_tpidr2() -> u64 {
    static uint64_t get_tpidr2(void)
    {
    uint64_t val;
    asm volatile (
    "mrs	%0, " SYS_TPIDR2 "\n"
    : "=r"(val)
    :
    : "cc");
    return val;
    }
// Processes should start with TPIDR2 == 0
#[no_mangle]
unsafe extern "C" fn default_value() -> c_int {
    static int default_value(void)
    {
    return get_tpidr2() == 0;
    }
// If we set TPIDR2 we should read that value
#[no_mangle]
unsafe extern "C" fn write_read() -> c_int {
    static int write_read(void)
    {
    set_tpidr2(getpid());
    return getpid() == get_tpidr2();
    }
// If we set a value we should read the same value after scheduling out
#[no_mangle]
unsafe extern "C" fn write_sleep_read() -> c_int {
    static int write_sleep_read(void)
    {
    set_tpidr2(getpid());
    msleep(100);
    return getpid() == get_tpidr2();
    }
//
// If we fork the value in the parent should be unchanged and the
// child should start with the same value and be able to set its own
// value.
//
#[no_mangle]
unsafe extern "C" fn write_fork_read() -> c_int {
    static int write_fork_read(void)
    {
    pid_t newpid, waiting, oldpid;
    int status;
    set_tpidr2(getpid());
    oldpid = getpid();
    newpid = fork();
    if (newpid == 0) {
// In child
    if (get_tpidr2() != oldpid) {
    ksft_print_msg("TPIDR2 changed in child: %llx\n",
    get_tpidr2());
    exit(0);
    }
    set_tpidr2(getpid());
    if (get_tpidr2() == getpid()) {
    exit(1);
    } else {
    ksft_print_msg("Failed to set TPIDR2 in child\n");
    exit(0);
    }
    }
    if (newpid < 0) {
    ksft_print_msg("fork() failed: %d\n", newpid);
    return 0;
    }
    for (;;) {
    waiting = waitpid(newpid, &status, 0);
    if (waiting < 0) {
    if (errno == EINTR)
    continue;
    ksft_print_msg("waitpid() failed: %d\n", errno);
    return 0;
    }
    if (waiting != newpid) {
    ksft_print_msg("waitpid() returned wrong PID: %d != %d\n",
    waiting, newpid);
    return 0;
    }
    if (!WIFEXITED(status)) {
    ksft_print_msg("child did not exit\n");
    return 0;
    }
    if (getpid() != get_tpidr2()) {
    ksft_print_msg("TPIDR2 corrupted in parent\n");
    return 0;
    }
    return WEXITSTATUS(status);
    }
    }
//
// sys_clone() has a lot of per architecture variation so just define
// it here rather than adding it to nolibc, plus the raw API is a
// little more convenient for this test.
//
    static int sys_clone(unsigned long clone_flags, unsigned long newsp,
    int *parent_tidptr, unsigned long tls,
    int *child_tidptr)
    {
    return syscall(__NR_clone, clone_flags, newsp, parent_tidptr, tls, child_tidptr);
    }

//
// If we clone with CLONE_VM then the value in the parent should
// be unchanged and the child should start with zero and be able to
// set its own value.
//
#[no_mangle]
unsafe extern "C" fn write_clone_read() -> c_int {
    static int write_clone_read(void)
    {
    int parent_tid, child_tid;
    pid_t parent, waiting;
    int ret, status;
    void *stack;
    parent = getpid();
    set_tpidr2(parent);
    stack = malloc(__STACK_SIZE);
    if (!stack) {
    ksft_print_msg("malloc() failed\n");
    return 0;
    }
    ret = sys_clone(CLONE_VM, (unsigned long)stack + __STACK_SIZE,
    &parent_tid, 0, &child_tid);
    if (ret == -1) {
    ksft_print_msg("clone() failed: %d\n", errno);
    return 0;
    }
    if (ret == 0) {
// In child
    if (get_tpidr2() != 0) {
    ksft_print_msg("TPIDR2 non-zero in child: %llx\n",
    get_tpidr2());
    exit(0);
    }
    if (gettid() == 0)
    ksft_print_msg("Child TID==0\n");
    set_tpidr2(gettid());
    if (get_tpidr2() == gettid()) {
    exit(1);
    } else {
    ksft_print_msg("Failed to set TPIDR2 in child\n");
    exit(0);
    }
    }
    for (;;) {
    waiting = waitpid(ret, &status, __WCLONE);
    if (waiting < 0) {
    if (errno == EINTR)
    continue;
    ksft_print_msg("waitpid() failed: %d\n", errno);
    return 0;
    }
    if (waiting != ret) {
    ksft_print_msg("waitpid() returned wrong PID %d\n",
    waiting);
    return 0;
    }
    if (!WIFEXITED(status)) {
    ksft_print_msg("child did not exit\n");
    return 0;
    }
    if (parent != get_tpidr2()) {
    ksft_print_msg("TPIDR2 corrupted in parent\n");
    return 0;
    }
    return WEXITSTATUS(status);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    int ret;
    ksft_print_header();
    ksft_set_plan(5);
    ksft_print_msg("PID: %d\n", getpid());
//
// This test is run with nolibc which doesn't support hwcap and
// it's probably disproportionate to implement so instead check
// for the default vector length configuration in /proc.
//
    ret = open("/proc/sys/abi/sme_default_vector_length", O_RDONLY, 0);
    if (ret >= 0) {
    ksft_test_result(default_value(), "default_value\n");
    ksft_test_result(write_read(), "write_read\n");
    ksft_test_result(write_sleep_read(), "write_sleep_read\n");
    ksft_test_result(write_fork_read(), "write_fork_read\n");
    ksft_test_result(write_clone_read(), "write_clone_read\n");
    } else {
    ksft_print_msg("SME support not present\n");
    ksft_test_result_skip("default_value\n");
    ksft_test_result_skip("write_read\n");
    ksft_test_result_skip("write_sleep_read\n");
    ksft_test_result_skip("write_fork_read\n");
    ksft_test_result_skip("write_clone_read\n");
    }
    ksft_finished();
    }
