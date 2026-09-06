//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/mm/stack_expansion_signal.c
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
// Test that signal delivery is able to expand the stack segment without
// triggering a SEGV.
//
// Based on test code by Tom Lane.
//

    static char *stack_base_ptr;
    static char *stack_top_ptr;
    let mut sig_occurred: static volatile sig_atomic_t = 0;
#[no_mangle]
unsafe extern "C" fn sigusr1_handler(signal_arg: c_int) {
    static void sigusr1_handler(int signal_arg)
    {
    sig_occurred = 1;
    }
#[no_mangle]
unsafe extern "C" fn consume_stack(stack_size: c_uint, write_pipe: union pipe) -> c_int {
    static int consume_stack(unsigned int stack_size, union pipe write_pipe)
    {
    char stack_cur;
    if ((stack_base_ptr - &stack_cur) < stack_size)
    return consume_stack(stack_size, write_pipe);
    else {
    stack_top_ptr = &stack_cur;
    FAIL_IF(notify_parent(write_pipe));
    while (!sig_occurred)
    barrier();
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn child(stack_size: c_uint, write_pipe: union pipe) -> c_int {
    static int child(unsigned int stack_size, union pipe write_pipe)
    {
    struct sigaction act;
    char stack_base;
    act.sa_handler = sigusr1_handler;
    sigemptyset(&act.sa_mask);
    act.sa_flags = 0;
    if (sigaction(SIGUSR1, &act, core::ptr::null_mut()) < 0)
    err(1, "sigaction");
    stack_base_ptr = (char *) (((size_t) &stack_base + 65535) & ~65535UL);
    FAIL_IF(consume_stack(stack_size, write_pipe));
    printf("size 0x%06x: OK, stack base %p top %p (%zx used)\n",
    stack_size, stack_base_ptr, stack_top_ptr,
    stack_base_ptr - stack_top_ptr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_one_size(stack_size: c_uint) -> c_int {
    static int test_one_size(unsigned int stack_size)
    {
    union pipe read_pipe, write_pipe;
    pid_t pid;
    FAIL_IF(pipe(read_pipe.fds) == -1);
    FAIL_IF(pipe(write_pipe.fds) == -1);
    pid = fork();
    if (pid == 0) {
    close(read_pipe.read_fd);
    close(write_pipe.write_fd);
    exit(child(stack_size, read_pipe));
    }
    close(read_pipe.write_fd);
    close(write_pipe.read_fd);
    FAIL_IF(sync_with_child(read_pipe, write_pipe));
    kill(pid, SIGUSR1);
    FAIL_IF(wait_for_child(pid));
    close(read_pipe.read_fd);
    close(write_pipe.write_fd);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn test() -> c_int {
    int test(void)
    {
    unsigned int i, size;
// Test with used stack from 1MB - 64K to 1MB + 64K
// Increment by 64 to get more coverage of odd sizes
    for (i = 0; i < (128 * _KB); i += 64) {
    size = i + (1 * _MB) - (64 * _KB);
    FAIL_IF(test_one_size(size));
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(test, "stack_expansion_signal");
    }
