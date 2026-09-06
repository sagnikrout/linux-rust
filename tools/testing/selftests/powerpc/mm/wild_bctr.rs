//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/mm/wild_bctr.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright 2018, Michael Ellerman, IBM Corp.
//
// Test that an out-of-bounds branch to counter behaves as expected.
//

pub const BAD_NIP: c_uint = 0x788c545a18000000ull;
    static struct pt_regs signal_regs;
    static jmp_buf setjmp_env;
#[no_mangle]
unsafe extern "C" fn save_regs(ctxt: *mut ucontext_t) {
    static void save_regs(ucontext_t *ctxt)
    {
    struct pt_regs *regs = ctxt.uc_mcontext.regs;
    memcpy(&signal_regs, regs, sizeof(signal_regs));
    }
#[no_mangle]
unsafe extern "C" fn segv_handler(signum: c_int, info: *mut siginfo_t, ctxt_v: *mut c_void) {
    static void segv_handler(int signum, siginfo_t *info, void *ctxt_v)
    {
    save_regs(ctxt_v);
    longjmp(setjmp_env, 1);
    }
#[no_mangle]
unsafe extern "C" fn usr2_handler(signum: c_int, info: *mut siginfo_t, ctxt_v: *mut c_void) {
    static void usr2_handler(int signum, siginfo_t *info, void *ctxt_v)
    {
    save_regs(ctxt_v);
    }
#[no_mangle]
unsafe extern "C" fn ok() -> c_int {
    static int ok(void)
    {
    printf("Everything is OK in here.\n");
    return 0;
    }
pub const REG_POISON: c_uint = 0x5a5a;

    (((unsigned long)REG_POISON) << 16) | (n))
#[no_mangle]
pub unsafe extern "C" fn poison_regs() {
    static inline void poison_regs(void)
    {

    "lis  " __stringify(n) "," __stringify(REG_POISON) ";" \
    "addi " __stringify(n) "," __stringify(n) "," __stringify(n) ";" \
    "sldi " __stringify(n) "," __stringify(n) ", 32 ;" \
    "oris " __stringify(n) "," __stringify(n) "," __stringify(REG_POISON) ";" \
    "addi " __stringify(n) "," __stringify(n) "," __stringify(n) ";"
    asm (POISON_REG(15)
    POISON_REG(16)
    POISON_REG(17)
    POISON_REG(18)
    POISON_REG(19)
    POISON_REG(20)
    POISON_REG(21)
    POISON_REG(22)
    POISON_REG(23)
    POISON_REG(24)
    POISON_REG(25)
    POISON_REG(26)
    POISON_REG(27)
    POISON_REG(28)
    POISON_REG(29)
    : // inputs
    : // outputs
    : "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25",
    "26", "27", "28", "29"
    );

    }
#[no_mangle]
unsafe extern "C" fn check_regs() -> c_int {
    static int check_regs(void)
    {
    unsigned long i;
    for (i = 15; i <= 29; i++)
    FAIL_IF(signal_regs.gpr[i] != POISONED_REG(i));
    printf("Regs OK\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dump_regs() {
    static void dump_regs(void)
    {
    for (int i = 0; i < 32; i += 4) {
    printf("r%02d 0x%016lx  r%02d 0x%016lx  " \
    "r%02d 0x%016lx  r%02d 0x%016lx\n",
    i, signal_regs.gpr[i],
    i+1, signal_regs.gpr[i+1],
    i+2, signal_regs.gpr[i+2],
    i+3, signal_regs.gpr[i+3]);
    }
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opd {
    pub ip: c_ulong,
    pub toc: c_ulong,
    pub env: c_ulong,
}

    static struct opd bad_opd = {
    .ip = BAD_NIP,
    };

#[no_mangle]
pub unsafe extern "C" fn test_wild_bctr() -> c_int {
    int test_wild_bctr(void)
    {
    int (*func_ptr)(void);
    struct sigaction segv = {
    .sa_sigaction = segv_handler,
    .sa_flags = SA_SIGINFO
    };
    struct sigaction usr2 = {
    .sa_sigaction = usr2_handler,
    .sa_flags = SA_SIGINFO
    };
    FAIL_IF(sigaction(SIGSEGV, &segv, core::ptr::null_mut()));
    FAIL_IF(sigaction(SIGUSR2, &usr2, core::ptr::null_mut()));
    bzero(&signal_regs, sizeof(signal_regs));
    if (setjmp(setjmp_env) == 0) {
    func_ptr = ok;
    func_ptr();
    kill(getpid(), SIGUSR2);
    printf("Regs before:\n");
    dump_regs();
    bzero(&signal_regs, sizeof(signal_regs));
    poison_regs();
    func_ptr = (int (*)(void))BAD_FUNC;
    func_ptr();
    FAIL_IF(1); /* we didn't segv? */
    }
    FAIL_IF(signal_regs.nip != BAD_NIP);
    printf("All good - took SEGV as expected branching to 0x%llx\n", BAD_NIP);
    dump_regs();
    FAIL_IF(check_regs());
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(test_wild_bctr, "wild_bctr");
    }
