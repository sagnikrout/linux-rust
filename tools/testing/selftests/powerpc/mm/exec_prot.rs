//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/mm/exec_prot.c
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
// Copyright 2022, Nicholas Miehlbradt, IBM Corporation
// based on pkey_exec_prot.c
//
// Test if applying execute protection on pages works as expected.
//
// Macro flag: #define _GNU_SOURCE

pub const PPC_INST_NOP: c_uint = 0x60000000;
pub const PPC_INST_TRAP: c_uint = 0x7fe00008;
pub const PPC_INST_BLR: c_uint = 0x4e800020;
    static volatile sig_atomic_t fault_code;
    static volatile sig_atomic_t remaining_faults;
    static volatile unsigned int *fault_addr;
    static unsigned long pgsize, numinsns;
    static unsigned int *insns;
    static bool pkeys_supported;
#[no_mangle]
unsafe extern "C" fn is_fault_expected(fault_code: c_int) -> bool {
    static bool is_fault_expected(int fault_code)
    {
    if (fault_code == SEGV_ACCERR)
    return true;
// Assume any pkey error is fine since pkey_exec_prot test covers them
    if (fault_code == SEGV_PKUERR && pkeys_supported)
    return true;
    return false;
    }
#[no_mangle]
unsafe extern "C" fn trap_handler(signum: c_int, sinfo: *mut siginfo_t, ctx: *mut c_void) {
    static void trap_handler(int signum, siginfo_t *sinfo, void *ctx)
    {
// Check if this fault originated from the expected address
    if (sinfo.si_addr != (void *)fault_addr)
    sigsafe_err("got a fault for an unexpected address\n");
    _exit(1);
    }
#[no_mangle]
unsafe extern "C" fn segv_handler(signum: c_int, sinfo: *mut siginfo_t, ctx: *mut c_void) {
    static void segv_handler(int signum, siginfo_t *sinfo, void *ctx)
    {
    fault_code = sinfo.si_code;
// Check if this fault originated from the expected address
    if (sinfo.si_addr != (void *)fault_addr) {
    sigsafe_err("got a fault for an unexpected address\n");
    _exit(1);
    }
// Check if too many faults have occurred for a single test case
    if (!remaining_faults) {
    sigsafe_err("got too many faults for the same address\n");
    _exit(1);
    }
// Restore permissions in order to continue
    if (is_fault_expected(fault_code)) {
    if (mprotect(insns, pgsize, PROT_READ | PROT_WRITE | PROT_EXEC)) {
    sigsafe_err("failed to set access permissions\n");
    _exit(1);
    }
    } else {
    sigsafe_err("got a fault with an unexpected code\n");
    _exit(1);
    }
    remaining_faults--;
    }
#[no_mangle]
unsafe extern "C" fn check_exec_fault(rights: c_int) -> c_int {
    static int check_exec_fault(int rights)
    {
//
// Jump to the executable region.
//
// The first iteration also checks if the overwrite of the
// first instruction word from a trap to a no-op succeeded.
//
    fault_code = -1;
    remaining_faults = 0;
    if (!(rights & PROT_EXEC))
    remaining_faults = 1;
    FAIL_IF(mprotect(insns, pgsize, rights) != 0);
    asm volatile("mtctr	%0; bctrl" : : "r"(insns));
    FAIL_IF(remaining_faults != 0);
    if (!(rights & PROT_EXEC))
    FAIL_IF(!is_fault_expected(fault_code));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test() -> c_int {
    static int test(void)
    {
    struct sigaction segv_act, trap_act;
    int i;
// Skip the test if the CPU doesn't support Radix
    SKIP_IF(!have_hwcap2(PPC_FEATURE2_ARCH_3_00));
// Check if pkeys are supported
    pkeys_supported = pkeys_unsupported() == 0;
// Setup SIGSEGV handler
    segv_act.sa_handler = 0;
    segv_act.sa_sigaction = segv_handler;
    FAIL_IF(sigprocmask(SIG_SETMASK, 0, &segv_act.sa_mask) != 0);
    segv_act.sa_flags = SA_SIGINFO;
    segv_act.sa_restorer = 0;
    FAIL_IF(sigaction(SIGSEGV, &segv_act, core::ptr::null_mut()) != 0);
// Setup SIGTRAP handler
    trap_act.sa_handler = 0;
    trap_act.sa_sigaction = trap_handler;
    FAIL_IF(sigprocmask(SIG_SETMASK, 0, &trap_act.sa_mask) != 0);
    trap_act.sa_flags = SA_SIGINFO;
    trap_act.sa_restorer = 0;
    FAIL_IF(sigaction(SIGTRAP, &trap_act, core::ptr::null_mut()) != 0);
// Setup executable region
    pgsize = getpagesize();
    numinsns = pgsize / sizeof(unsigned int);
    insns = (unsigned int *)mmap(core::ptr::null_mut(), pgsize, PROT_READ | PROT_WRITE,
    MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    FAIL_IF(insns == MAP_FAILED);
// Write the instruction words
    for (i = 1; i < numinsns - 1; i++)
    insns[i] = PPC_INST_NOP;
//
// Set the first instruction as an unconditional trap. If
// the last write to this address succeeds, this should
// get overwritten by a no-op.
//
    insns[0] = PPC_INST_TRAP;
//
// Later, to jump to the executable region, we use a branch
// and link instruction (bctrl) which sets the return address
// automatically in LR. Use that to return back.
//
    insns[numinsns - 1] = PPC_INST_BLR;
//
// Pick the first instruction's address from the executable
// region.
//
    fault_addr = insns;
//
// Read an instruction word from the address when the page
// is execute only. This should generate an access fault.
//
    fault_code = -1;
    remaining_faults = 1;
    printf("Testing read on --x, should fault...");
    FAIL_IF(mprotect(insns, pgsize, PROT_EXEC) != 0);
    i = *fault_addr;
    FAIL_IF(remaining_faults != 0 || !is_fault_expected(fault_code));
    printf("ok!\n");
//
// Write an instruction word to the address when the page
// execute only. This should also generate an access fault.
//
    fault_code = -1;
    remaining_faults = 1;
    printf("Testing write on --x, should fault...");
    FAIL_IF(mprotect(insns, pgsize, PROT_EXEC) != 0);
// fault_addr = PPC_INST_NOP;
    FAIL_IF(remaining_faults != 0 || !is_fault_expected(fault_code));
    printf("ok!\n");
    printf("Testing exec on ---, should fault...");
    FAIL_IF(check_exec_fault(PROT_NONE));
    printf("ok!\n");
    printf("Testing exec on r--, should fault...");
    FAIL_IF(check_exec_fault(PROT_READ));
    printf("ok!\n");
    printf("Testing exec on -w-, should fault...");
    FAIL_IF(check_exec_fault(PROT_WRITE));
    printf("ok!\n");
    printf("Testing exec on rw-, should fault...");
    FAIL_IF(check_exec_fault(PROT_READ | PROT_WRITE));
    printf("ok!\n");
    printf("Testing exec on --x, should succeed...");
    FAIL_IF(check_exec_fault(PROT_EXEC));
    printf("ok!\n");
    printf("Testing exec on r-x, should succeed...");
    FAIL_IF(check_exec_fault(PROT_READ | PROT_EXEC));
    printf("ok!\n");
    printf("Testing exec on -wx, should succeed...");
    FAIL_IF(check_exec_fault(PROT_WRITE | PROT_EXEC));
    printf("ok!\n");
    printf("Testing exec on rwx, should succeed...");
    FAIL_IF(check_exec_fault(PROT_READ | PROT_WRITE | PROT_EXEC));
    printf("ok!\n");
// Cleanup
    FAIL_IF(munmap((void *)insns, pgsize));
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(test, "exec_prot");
    }
