//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/x86/entry_from_vm86.c
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
// entry_from_vm86.c - tests kernel entries from vm86 mode
// Copyright (c) 2014-2015 Andrew Lutomirski
//
// This exercises a few paths that need to special-case vm86 mode.
//
// Macro flag: #define _GNU_SOURCE

    let mut load_addr: static unsigned long = 0x10000;
    let mut nerrs: static int = 0;
    static sig_atomic_t got_signal;
#[no_mangle]
unsafe extern "C" fn sighandler(sig: c_int, info: *mut siginfo_t, ctx_void: *mut c_void) {
    static void sighandler(int sig, siginfo_t *info, void *ctx_void)
    {
    ucontext_t *ctx = (ucontext_t*)ctx_void;
    if (ctx.uc_mcontext.gregs[REG_EFL] & X86_EFLAGS_VM ||
    (ctx.uc_mcontext.gregs[REG_CS] & 3) != 3) {
    printf("[FAIL]\tSignal frame should not reflect vm86 mode\n");
    nerrs++;
    }
    const char *signame;
    if (sig == SIGSEGV)
    signame = "SIGSEGV";
#[no_mangle]
pub unsafe extern "C" fn if(SIGILL: sig ==) -> else {
    else if (sig == SIGILL)
    signame = "SIGILL";
    else
    signame = "unexpected signal";
    printf("[INFO]\t%s: FLAGS = 0x%lx, CS = 0x%hx\n", signame,
    (unsigned long)ctx.uc_mcontext.gregs[REG_EFL],
    (unsigned short)ctx.uc_mcontext.gregs[REG_CS]);
    got_signal = 1;
    }
    asm (
    ".pushsection .rodata\n\t"
    ".type vmcode_bound, @object\n\t"
    "vmcode:\n\t"
    "vmcode_bound:\n\t"
    ".code16\n\t"
    "bound %ax, (2048)\n\t"
    "int3\n\t"
    "vmcode_sysenter:\n\t"
    "sysenter\n\t"
    "vmcode_syscall:\n\t"
    "syscall\n\t"
    "vmcode_sti:\n\t"
    "sti\n\t"
    "vmcode_int3:\n\t"
    "int3\n\t"
    "vmcode_int80:\n\t"
    "int $0x80\n\t"
    "vmcode_popf_hlt:\n\t"
    "push %ax\n\t"
    "popf\n\t"
    "hlt\n\t"
    "vmcode_umip:\n\t"
// addressing via displacements
    "smsw (2052)\n\t"
    "sidt (2054)\n\t"
    "sgdt (2060)\n\t"
// addressing via registers
    "mov $2066, %bx\n\t"
    "smsw (%bx)\n\t"
    "mov $2068, %bx\n\t"
    "sidt (%bx)\n\t"
    "mov $2074, %bx\n\t"
    "sgdt (%bx)\n\t"
// register operands, only for smsw
    "smsw %ax\n\t"
    "mov %ax, (2080)\n\t"
    "int3\n\t"
    "vmcode_umip_str:\n\t"
    "str %eax\n\t"
    "vmcode_umip_sldt:\n\t"
    "sldt %eax\n\t"
    "int3\n\t"
    ".size vmcode, . - vmcode\n\t"
    "end_vmcode:\n\t"
    ".code32\n\t"
    ".popsection"
    );
    extern unsigned char vmcode[], end_vmcode[];
    extern unsigned char vmcode_bound[], vmcode_sysenter[], vmcode_syscall[],
    vmcode_sti[], vmcode_int3[], vmcode_int80[], vmcode_popf_hlt[],
    vmcode_umip[], vmcode_umip_str[], vmcode_umip_sldt[];
// Returns false if the test was skipped.
    static bool do_test(struct vm86plus_struct *v86, unsigned long eip,
    unsigned int rettype, unsigned int retarg,
    const char *text)
    {
    long ret;
    printf("[RUN]\t%s from vm86 mode\n", text);
    v86.regs.eip = eip;
    ret = vm86(VM86_ENTER, v86);
    if (ret == -1 && (errno == ENOSYS || errno == EPERM)) {
    printf("[SKIP]\tvm86 %s\n",
    errno == ENOSYS ? "not supported" : "not allowed");
    return false;
    }
    if (VM86_TYPE(ret) == VM86_INTx) {
    char trapname[32];
    let mut trapno: c_int = VM86_ARG(ret);
    if (trapno == 13)
    strcpy(trapname, "GP");
#[no_mangle]
pub unsafe extern "C" fn if(5: trapno ==) -> else {
    else if (trapno == 5)
    strcpy(trapname, "BR");
#[no_mangle]
pub unsafe extern "C" fn if(14: trapno ==) -> else {
    else if (trapno == 14)
    strcpy(trapname, "PF");
    else
    sprintf(trapname, "%d", trapno);
    printf("[INFO]\tExited vm86 mode due to #%s\n", trapname);
    } else if (VM86_TYPE(ret) == VM86_UNKNOWN) {
    printf("[INFO]\tExited vm86 mode due to unhandled GP fault\n");
    } else if (VM86_TYPE(ret) == VM86_TRAP) {
    printf("[INFO]\tExited vm86 mode due to a trap (arg=%ld)\n",
    VM86_ARG(ret));
    } else if (VM86_TYPE(ret) == VM86_SIGNAL) {
    printf("[INFO]\tExited vm86 mode due to a signal\n");
    } else if (VM86_TYPE(ret) == VM86_STI) {
    printf("[INFO]\tExited vm86 mode due to STI\n");
    } else {
    printf("[INFO]\tExited vm86 mode due to type %ld, arg %ld\n",
    VM86_TYPE(ret), VM86_ARG(ret));
    }
    if (rettype == -1 ||
    (VM86_TYPE(ret) == rettype && VM86_ARG(ret) == retarg)) {
    printf("[OK]\tReturned correctly\n");
    } else {
    printf("[FAIL]\tIncorrect return reason (started at eip = 0x%lx, ended at eip = 0x%lx)\n", eip, v86.regs.eip);
    nerrs++;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn do_umip_tests(vm86: *mut vm86plus_struct, test_mem: *mut c_uchar) {
    void do_umip_tests(struct vm86plus_struct *vm86, unsigned char *test_mem)
    {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct table_desc {
    pub limit: c_ushort,
    pub base: c_ulong,
    pub __attribute__((packed)): },
// Initialize variables with arbitrary values
    pub }: table_desc gdt1 = { .base = 0x3c3c3c3c, .limit = 0x9999,
    pub }: table_desc gdt2 = { .base = 0x1a1a1a1a, .limit = 0xaeae,
    pub }: table_desc idt1 = { .base = 0x7b7b7b7b, .limit = 0xf1f1,
    pub }: table_desc idt2 = { .base = 0x89898989, .limit = 0x1313,
    pub 3737: unsigned short msw1 = 0x1414, msw2 = 0x2525, msw3 =,
// UMIP -- exit with INT3 unless kernel emulation did not trap #GP
    pub tests"): do_test(vm86, vmcode_umip - vmcode, VM86_TRAP, 3, "UMIP,
// Results from displacement-only addressing
    pub 2052): *mut *mut *mut msw1 = (unsigned short )(test_mem +,
    pub sizeof(idt1)): memcpy(&idt1, test_mem + 2054,,
    pub sizeof(gdt1)): memcpy(&gdt1, test_mem + 2060,,
// Results from register-indirect addressing
    pub 2066): *mut *mut *mut msw2 = (unsigned short )(test_mem +,
    pub sizeof(idt2)): memcpy(&idt2, test_mem + 2068,,
    pub sizeof(gdt2)): memcpy(&gdt2, test_mem + 2074,,
// Results when using register operands
    pub 2080): *mut *mut *mut msw3 = (unsigned short )(test_mem +,
    pub msw1): printf("[INFO]\tResult from SMSW:[0x%04x]\n",,
    printf("[INFO]\tResult from SIDT: limit[0x%04x]base[0x%08lx]\n",
    pub idt1.base): idt1.limit,,
    printf("[INFO]\tResult from SGDT: limit[0x%04x]base[0x%08lx]\n",
    pub gdt1.base): gdt1.limit,,
    if (msw1 != msw2 || msw1 != msw3)
    pub same.\n"): printf("[FAIL]\tAll the results of SMSW should be the,
    else
    pub identical.\n"): printf("[PASS]\tAll the results from SMSW are,
    if (memcmp(&gdt1, &gdt2, sizeof(gdt1)))
    pub same.\n"): printf("[FAIL]\tAll the results of SGDT should be the,
    else
    pub identical.\n"): printf("[PASS]\tAll the results from SGDT are,
    if (memcmp(&idt1, &idt2, sizeof(idt1)))
    pub same.\n"): printf("[FAIL]\tAll the results of SIDT should be the,
    else
    pub identical.\n"): printf("[PASS]\tAll the results from SIDT are,
    pub 0): sethandler(SIGILL, sighandler,,
    do_test(vm86, vmcode_umip_str - vmcode, VM86_SIGNAL, 0,
    pub instruction"): "STR,
    pub 0): sethandler(SIGILL, sighandler,,
    do_test(vm86, vmcode_umip_sldt - vmcode, VM86_SIGNAL, 0,
    pub instruction"): "SLDT,
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    pub v86: vm86plus_struct,
    unsigned char *addr = mmap((void *)load_addr, 4096,
    PROT_READ | PROT_WRITE | PROT_EXEC,
    pub -1,0): MAP_ANONYMOUS | MAP_PRIVATE,,
    if (addr != (unsigned char *)load_addr)
    pub "mmap"): err(1,,
    pub vmcode): memcpy(addr, vmcode, end_vmcode -,
    pub 2: addr[2048] =,
    pub 3: addr[2050] =,
    pub sizeof(v86)): memset(&v86, 0,,
    pub 16: v86.regs.cs = load_addr /,
    pub 16: v86.regs.ss = load_addr /,
    pub 16: v86.regs.ds = load_addr /,
    pub 16: v86.regs.es = load_addr /,
// Use the end of the page as our stack.
    pub 4096: v86.regs.esp =,
    pub /: *mut *mut assert((v86.regs.cs & 3) == 0); / Looks like RPL = 0,
// #BR -- should deliver SIG???
    pub "#BR"): do_test(&v86, vmcode_bound - vmcode, VM86_INTx, 5,,
//
// SYSENTER -- should cause #GP or #UD depending on CPU.
// Expected return type -1 means that we shouldn't validate
// the vm86 return value.  This will avoid problems on non-SEP
// CPUs.
//
    pub 0): sethandler(SIGILL, sighandler,,
    pub "SYSENTER"): do_test(&v86, vmcode_sysenter - vmcode, -1, 0,,
//
// SYSCALL would be a disaster in VM86 mode.  Fortunately,
// there is no kernel that both enables SYSCALL and sets
// EFER.SCE, so it's #UD on all systems.  But vm86 is
// buggy (or has a "feature"), so the SIGILL will actually
// be delivered.
//
    pub 0): sethandler(SIGILL, sighandler,,
    pub "SYSCALL"): do_test(&v86, vmcode_syscall - vmcode, VM86_SIGNAL, 0,,
// STI with VIP set
    pub X86_EFLAGS_VIP: v86.regs.eflags |=,
    pub ~X86_EFLAGS_IF: v86.regs.eflags &=,
    pub set"): do_test(&v86, vmcode_sti - vmcode, VM86_STI, 0, "STI with VIP,
// POPF with VIP set but IF clear: should not trap
    pub X86_EFLAGS_VIP: v86.regs.eflags =,
    pub 0: v86.regs.eax =,
    pub clear"): do_test(&v86, vmcode_popf_hlt - vmcode, VM86_UNKNOWN, 0, "POPF with VIP set and IF,
// POPF with VIP set and IF set: should trap
    pub X86_EFLAGS_VIP: v86.regs.eflags =,
    pub X86_EFLAGS_IF: v86.regs.eax =,
    pub set"): do_test(&v86, vmcode_popf_hlt - vmcode, VM86_STI, 0, "POPF with VIP and IF,
// POPF with VIP clear and IF set: should not trap
    pub 0: v86.regs.eflags =,
    pub X86_EFLAGS_IF: v86.regs.eax =,
    pub set"): do_test(&v86, vmcode_popf_hlt - vmcode, VM86_UNKNOWN, 0, "POPF with VIP clear and IF,
    pub 0: v86.regs.eflags =,
// INT3 -- should cause #BP
    pub "INT3"): do_test(&v86, vmcode_int3 - vmcode, VM86_TRAP, 3,,
// INT80 -- should exit with "INTx 0x80"
    pub int)-1: v86.regs.eax = (unsigned,
    pub "int80"): do_test(&v86, vmcode_int80 - vmcode, VM86_INTx, 0x80,,
// UMIP -- should exit with INTx 0x80 unless UMIP was not disabled
    pub addr): do_umip_tests(&v86,,
// Execute a null pointer
    pub 0: v86.regs.cs =,
    pub 0: v86.regs.ss =,
    pub 0): sethandler(SIGSEGV, sighandler,,
    pub 0: got_signal =,
    if (do_test(&v86, 0, VM86_SIGNAL, 0, "Execute null pointer") &&
    !got_signal) {
    pub SIGSEGV\n"): printf("[FAIL]\tDid not receive,
    }
// Make sure nothing explodes if we fork.
    if (fork() == 0)
    pub 0: return,
    pub 1): return (nerrs == 0 ? 0 :,
    }
