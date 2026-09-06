//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/x86/fsgsbase_restore.c
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
//
// fsgsbase_restore.c, test ptrace vs fsgsbase
// Copyright (c) 2020 Andy Lutomirski
//
// This test case simulates a tracer redirecting tracee execution to
// a function and then restoring tracee state using PTRACE_GETREGS and
// PTRACE_SETREGS.  This is similar to what gdb does when doing
// 'p func()'.  The catch is that this test has the called function
// modify a segment register.  This makes sure that ptrace correctly
// restores segment state when using PTRACE_SETREGS.
//
// This is not part of fsgsbase.c, because that test is 64-bit only.
//
// Macro flag: #define _GNU_SOURCE

pub const EXPECTED_VALUE: c_uint = 0x1337f00d;

//
// Defined in clang_helpers_[32|64].S, because unlike gcc, clang inline asm does
// not support segmentation prefixes.
//
    unsigned int dereference_seg_base(void);
#[no_mangle]
unsafe extern "C" fn init_seg() {
    static void init_seg(void)
    {
    unsigned int *target = mmap(
    core::ptr::null_mut(), sizeof(unsigned int),
    PROT_READ | PROT_WRITE,
    MAP_PRIVATE | MAP_ANONYMOUS | MAP_32BIT, -1, 0);
    if (target == MAP_FAILED)
    err(1, "mmap");
// target = EXPECTED_VALUE;
    printf("\tsegment base address = 0x%lx\n", (unsigned long)target);
    struct user_desc desc = {
    .entry_number    = 0,
    .base_addr       = (unsigned int)(uintptr_t)target,
    .limit           = sizeof(unsigned int) - 1,
    .seg_32bit       = 1,
    .contents        = 0, /* Data, grow-up */
    .read_exec_only  = 0,
    .limit_in_pages  = 0,
    .seg_not_present = 0,
    .useable         = 0
    };
    if (syscall(SYS_modify_ldt, 1, &desc, sizeof(desc)) == 0) {
    printf("\tusing LDT slot 0\n");
    asm volatile ("mov %0, %" SEG :: "rm" ((unsigned short)0x7));
    } else {
// No modify_ldt for us (configured out, perhaps)
    struct user_desc *low_desc = mmap(
    core::ptr::null_mut(), sizeof(desc),
    PROT_READ | PROT_WRITE,
    MAP_PRIVATE | MAP_ANONYMOUS | MAP_32BIT, -1, 0);
    memcpy(low_desc, &desc, sizeof(desc));
    low_desc.entry_number = -1;
// 32-bit set_thread_area
    long ret;
    asm volatile ("int $0x80"
    : "=a" (ret), "+m" (*low_desc)
    : "a" (243), "b" (low_desc)

    : "r8", "r9", "r10", "r11"

    );
    memcpy(&desc, low_desc, sizeof(desc));
    munmap(low_desc, sizeof(desc));
    if (ret != 0) {
    printf("[NOTE]\tcould not create a segment -- can't test anything\n");
    exit(0);
    }
    printf("\tusing GDT slot %d\n", desc.entry_number);
    let mut sel: c_ushort = (unsigned short)((desc.entry_number << 3) | 0x3);
    asm volatile ("mov %0, %" SEG :: "rm" (sel));
    }
    }
#[no_mangle]
unsafe extern "C" fn tracee_zap_segment() {
    static void tracee_zap_segment(void)
    {
//
// The tracer will redirect execution here.  This is meant to
// work like gdb's 'p func()' feature.  The tricky bit is that
// we modify a segment register in order to make sure that ptrace
// can correctly restore segment registers.
//
    printf("\tTracee: in tracee_zap_segment()\n");
//
// Write a nonzero selector with base zero to the segment register.
// Using a null selector would defeat the test on AMD pre-Zen2
// CPUs, as such CPUs don't clear the base when loading a null
// selector.
//
    unsigned short sel;
    asm volatile ("mov %%ss, %0\n\t"
    "mov %0, %" SEG
    : "=rm" (sel));
    let mut pid: pid_t = getpid(), tid = syscall(SYS_gettid);
    printf("\tTracee is going back to sleep\n");
    syscall(SYS_tgkill, pid, tid, SIGSTOP);
// Should not get here.
    while (true) {
    printf("[FAIL]\tTracee hit unreachable code\n");
    pause();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main()
    {
    printf("\tSetting up a segment\n");
    init_seg();
    let mut val: c_uint = dereference_seg_base();
    if (val != EXPECTED_VALUE) {
    printf("[FAIL]\tseg[0] == %x; should be %x\n", val, EXPECTED_VALUE);
    return 1;
    }
    printf("[OK]\tThe segment points to the right place.\n");
    let mut chld: pid_t = fork();
    if (chld < 0)
    err(1, "fork");
    if (chld == 0) {
    prctl(PR_SET_PDEATHSIG, SIGKILL, 0, 0, 0, 0);
    if (ptrace(PTRACE_TRACEME, 0, 0, 0) != 0)
    err(1, "PTRACE_TRACEME");
    let mut pid: pid_t = getpid(), tid = syscall(SYS_gettid);
    printf("\tTracee will take a nap until signaled\n");
    syscall(SYS_tgkill, pid, tid, SIGSTOP);
    printf("\tTracee was resumed.  Will re-check segment.\n");
    val = dereference_seg_base();
    if (val != EXPECTED_VALUE) {
    printf("[FAIL]\tseg[0] == %x; should be %x\n", val, EXPECTED_VALUE);
    exit(1);
    }
    printf("[OK]\tThe segment points to the right place.\n");
    exit(0);
    }
    int status;
// Wait for SIGSTOP.
    if (waitpid(chld, &status, 0) != chld || !WIFSTOPPED(status))
    err(1, "waitpid");
    struct user_regs_struct regs;
    if (ptrace(PTRACE_GETREGS, chld, core::ptr::null_mut(), &regs) != 0)
    err(1, "PTRACE_GETREGS");

    printf("\tChild GS=0x%lx, GSBASE=0x%lx\n", (unsigned long)regs.gs, (unsigned long)regs.gs_base);

    printf("\tChild FS=0x%lx\n", (unsigned long)regs.xfs);

    let mut regs2: user_regs_struct = regs;

    regs2.rip = (unsigned long)tracee_zap_segment;
    regs2.rsp -= 128;	/* Don't clobber the redzone. */

    regs2.eip = (unsigned long)tracee_zap_segment;

    printf("\tTracer: redirecting tracee to tracee_zap_segment()\n");
    if (ptrace(PTRACE_SETREGS, chld, core::ptr::null_mut(), &regs2) != 0)
    err(1, "PTRACE_GETREGS");
    if (ptrace(PTRACE_CONT, chld, core::ptr::null_mut(), core::ptr::null_mut()) != 0)
    err(1, "PTRACE_GETREGS");
// Wait for SIGSTOP.
    if (waitpid(chld, &status, 0) != chld || !WIFSTOPPED(status))
    err(1, "waitpid");
    printf("\tTracer: restoring tracee state\n");
    if (ptrace(PTRACE_SETREGS, chld, core::ptr::null_mut(), &regs) != 0)
    err(1, "PTRACE_GETREGS");
    if (ptrace(PTRACE_DETACH, chld, core::ptr::null_mut(), core::ptr::null_mut()) != 0)
    err(1, "PTRACE_GETREGS");
// Wait for SIGSTOP.
    if (waitpid(chld, &status, 0) != chld)
    err(1, "waitpid");
    if (WIFSIGNALED(status)) {
    printf("[FAIL]\tTracee crashed\n");
    return 1;
    }
    if (!WIFEXITED(status)) {
    printf("[FAIL]\tTracee stopped for an unexpected reason: %d\n", status);
    return 1;
    }
    let mut exitcode: c_int = WEXITSTATUS(status);
    if (exitcode != 0) {
    printf("[FAIL]\tTracee reported failure\n");
    return 1;
    }
    printf("[OK]\tAll is well.\n");
    return 0;
    }
