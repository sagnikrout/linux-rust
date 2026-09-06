//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/x86/nx_stack.c
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


//
// Copyright (c) 2023 Alexey Dobriyan <adobriyan@gmail.com>
//
// Permission to use, copy, modify, and distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//
// Test that userspace stack is NX. Requires linking with -Wl,-z,noexecstack
// because I don't want to bother with PT_GNU_STACK detection.
//
// Fill the stack with INT3's and then try to execute some of them:
// SIGSEGV -- good, SIGTRAP -- bad.
//
// Regular stack is completely overwritten before testing.
// Test doesn't exit SIGSEGV handler after first fault at INT3.
//

// Macro flag: #define _GNU_SOURCE

pub const PAGE_SIZE: c_int = 4096;
//
// This is memset(rsp, 0xcc, -1); but down.
// It will SIGSEGV when bottom of the stack is reached.
// Byte-size access is important! (see rdi tweak in the signal handler).
//
    void make_stack1(void);
    asm(
    ".pushsection .text\n"
    ".globl make_stack1\n"
    ".align 16\n"
    "make_stack1:\n"
    "mov $0xcc, %al\n"

    "mov %rsp, %rdi\n"
    "mov $-1, %rcx\n"

    "mov %esp, %edi\n"
    "mov $-1, %ecx\n"

    "std\n"
    "rep stosb\n"
// unreachable
    "hlt\n"
    ".type make_stack1,@function\n"
    ".size make_stack1,.-make_stack1\n"
    ".popsection\n"
    );
//
// memset(p, 0xcc, -1);
// It will SIGSEGV when top of the stack is reached.
//
    void make_stack2(uint64_t p);
    asm(
    ".pushsection .text\n"
    ".globl make_stack2\n"
    ".align 16\n"
    "make_stack2:\n"
    "mov $0xcc, %al\n"

    "mov $-1, %rcx\n"

    "mov $-1, %ecx\n"

    "cld\n"
    "rep stosb\n"
// unreachable
    "hlt\n"
    ".type make_stack2,@function\n"
    ".size make_stack2,.-make_stack2\n"
    ".popsection\n"
    );
    let mut test_state: static volatile int = 0;
    static volatile unsigned long stack_min_addr;

#[no_mangle]
unsafe extern "C" fn sigsegv(_: c_int, __: *mut siginfo_t, uc_: *mut c_void) {
    static void sigsegv(int _, siginfo_t *__, void *uc_)
    {
//
// Some Linux versions didn't clear DF before entering signal
// handler. make_stack1() doesn't have a chance to clear DF
// either so we clear it by hand here.
//
    asm volatile ("cld" ::: "memory");
    ucontext_t *uc = uc_;
    if (test_state == 0) {
// Stack is faulted and cleared from RSP to the lowest address.
    stack_min_addr = ++uc.uc_mcontext.gregs[RDI];
    if (1) {
    printf("stack min %lx\n", stack_min_addr);
    }
    uc.uc_mcontext.gregs[RIP] = (uintptr_t)&make_stack2;
    test_state = 1;
    } else if (test_state == 1) {
// Stack has been cleared from top to bottom.
    let mut stack_max_addr: c_ulong = uc.uc_mcontext.gregs[RDI];
    if (1) {
    printf("stack max %lx\n", stack_max_addr);
    }
// Start faulting pages on stack and see what happens.
    uc.uc_mcontext.gregs[RIP] = stack_max_addr - PAGE_SIZE;
    test_state = 2;
    } else if (test_state == 2) {
// Stack page is NX -- good, test next page.
    uc.uc_mcontext.gregs[RIP] -= PAGE_SIZE;
    if (uc.uc_mcontext.gregs[RIP] == stack_min_addr) {
// One more SIGSEGV and test ends.
    test_state = 3;
    }
    } else {
    printf("PASS\tAll stack pages are NX\n");
    _exit(EXIT_SUCCESS);
    }
    }
#[no_mangle]
unsafe extern "C" fn sigtrap(_: c_int, __: *mut siginfo_t, uc_: *mut c_void) {
    static void sigtrap(int _, siginfo_t *__, void *uc_)
    {
    const ucontext_t *uc = uc_;
    let mut rip: c_ulong = uc.uc_mcontext.gregs[RIP];
    printf("FAIL\texecutable page on the stack: " RIP_STRING " %lx\n", rip);
    _exit(EXIT_FAILURE);
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    {
    let mut act: sigaction = {};
    sigemptyset(&act.sa_mask);
    act.sa_flags = SA_SIGINFO;
    act.sa_sigaction = &sigsegv;
    let mut rv: c_int = sigaction(SIGSEGV, &act, core::ptr::null_mut());
    assert(rv == 0);
    }
    {
    let mut act: sigaction = {};
    sigemptyset(&act.sa_mask);
    act.sa_flags = SA_SIGINFO;
    act.sa_sigaction = &sigtrap;
    let mut rv: c_int = sigaction(SIGTRAP, &act, core::ptr::null_mut());
    assert(rv == 0);
    }
    {
    struct rlimit rlim;
    let mut rv: c_int = getrlimit(RLIMIT_STACK, &rlim);
    assert(rv == 0);
// Cap stack at time-honored 8 MiB value.
    rlim.rlim_max = rlim.rlim_cur;
    if (rlim.rlim_max > 8 * 1024 * 1024) {
    rlim.rlim_max = 8 * 1024 * 1024;
    }
    rv = setrlimit(RLIMIT_STACK, &rlim);
    assert(rv == 0);
    }
    {
//
// We don't know now much stack SIGSEGV handler uses.
// Bump this by 1 page every time someone complains,
// or rewrite it in assembly.
//
    let mut len: usize = SIGSTKSZ;
    void *p = mmap(core::ptr::null_mut(), len, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0);
    assert(p != MAP_FAILED);
    let mut ss: stack_t = {};
    ss.ss_sp = p;
    ss.ss_size = len;
    let mut rv: c_int = sigaltstack(&ss, core::ptr::null_mut());
    assert(rv == 0);
    }
    make_stack1();
//
// Unreachable, but if _this_ INT3 is ever reached, it's a bug somewhere.
// Fold it into main SIGTRAP pathway.
//
    __builtin_trap();
    }
