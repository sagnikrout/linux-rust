//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/x86/sigreturn.c
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
// sigreturn.c - tests for x86 sigreturn(2) and exit-to-userspace
// Copyright (c) 2014-2015 Andrew Lutomirski
//
// This is a series of tests that exercises the sigreturn(2) syscall and
// the IRET / SYSRET paths in the kernel.
//
// For now, this focuses on the effects of unusual CS and SS values,
// and it has a bunch of tests to make sure that ESP/RSP is restored
// properly.
//
// The basic idea behind these tests is to raise(SIGUSR1) to create a
// sigcontext frame, plug in the values to be tested, and then return,
// which implicitly invokes sigreturn(2) and programs the user context
// as desired.
//
// For tests for which we expect sigreturn and the subsequent return to
// user mode to succeed, we return to a short trampoline that generates
// SIGTRAP so that the meat of the tests can be ordinary C code in a
// SIGTRAP handler.
//
// The inner workings of each test is documented below.
//
// Do not run on outdated, unpatched kernels at risk of nasty crashes.
//
// Macro flag: #define _GNU_SOURCE

// Pull in AR_xyz defines.
    typedef unsigned int u32;
    typedef unsigned short u16;

//
// Copied from asm/ucontext.h, as asm/ucontext.h conflicts badly with the glibc
// headers.
//

//
// UC_SIGCONTEXT_SS will be set when delivering 64-bit or x32 signals on
// kernels that save SS in the sigcontext.  All kernels that set
// UC_SIGCONTEXT_SS will correctly restore at least the low 32 bits of esp
// regardless of SS (i.e. they implement espfix).
//
// Kernels that set UC_SIGCONTEXT_SS will also set UC_STRICT_RESTORE_SS
// when delivering a signal that came from 64-bit code.
//
// Sigreturn restores SS as follows:
//
// if (saved SS is valid || UC_STRICT_RESTORE_SS is set ||
// saved CS is not 64-bit)
// new SS = saved SS  (will fail IRET and signal if invalid)
// else
// new SS = a flat 32-bit data segment
//
pub const UC_SIGCONTEXT_SS: c_uint = 0x2;
pub const UC_STRICT_RESTORE_SS: c_uint = 0x4;

//
// In principle, this test can run on Linux emulation layers (e.g.
// Illumos "LX branded zones").  Solaris-based kernels reserve LDT
// entries 0-5 for their own internal purposes, so start our LDT
// allocations above that reservation.  (The tests don't pass on LX
// branded zones, but at least this lets them run.)
//
pub const LDT_OFFSET: c_int = 6;
// An aligned stack accessible through some of our segments.
    static unsigned char stack16[65536] __attribute__((aligned(4096)));
//
// An aligned int3 instruction used as a trampoline.  Some of the tests
// want to fish out their ss values, so this trampoline copies ss to eax
// before the int3.
//
    asm (".pushsection .text\n\t"
    ".type int3, @function\n\t"
    ".align 4096\n\t"
    "int3:\n\t"
    "mov %ss,%ecx\n\t"
    "int3\n\t"
    ".size int3, . - int3\n\t"
    ".align 4096, 0xcc\n\t"
    ".popsection");
    extern char int3[4096];
//
// At startup, we prepapre:
//
// - ldt_nonexistent_sel: An LDT entry that doesn't exist (all-zero
// descriptor or out of bounds).
// - code16_sel: A 16-bit LDT code segment pointing to int3.
// - data16_sel: A 16-bit LDT data segment pointing to stack16.
// - npcode32_sel: A 32-bit not-present LDT code segment pointing to int3.
// - npdata32_sel: A 32-bit not-present LDT data segment pointing to stack16.
// - gdt_data16_idx: A 16-bit GDT data segment pointing to stack16.
// - gdt_npdata32_idx: A 32-bit not-present GDT data segment pointing to
// stack16.
//
// For no particularly good reason, xyz_sel is a selector value with the
// RPL and LDT bits filled in, whereas xyz_idx is just an index into the
// descriptor table.  These variables will be zero if their respective
// segments could not be allocated.
//
    static unsigned short ldt_nonexistent_sel;
    static unsigned short code16_sel, data16_sel, npcode32_sel, npdata32_sel;
    static unsigned short gdt_data16_idx, gdt_npdata32_idx;
#[no_mangle]
unsafe extern "C" fn GDT3(idx: c_int) -> c_ushort {
    static unsigned short GDT3(int idx)
    {
    return (idx << 3) | 3;
    }
#[no_mangle]
unsafe extern "C" fn LDT3(idx: c_int) -> c_ushort {
    static unsigned short LDT3(int idx)
    {
    return (idx << 3) | 7;
    }
    static void add_ldt(const struct user_desc *desc, unsigned short *var,
    const char *name)
    {
    if (syscall(SYS_modify_ldt, 1, desc, sizeof(*desc)) == 0) {
// var = LDT3(desc->entry_number);
    } else {
    printf("[NOTE]\tFailed to create %s segment\n", name);
// var = 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn setup_ldt() {
    static void setup_ldt(void)
    {
    if ((unsigned long)stack16 > (1ULL << 32) - sizeof(stack16))
    errx(1, "stack16 is too high\n");
    if ((unsigned long)int3 > (1ULL << 32) - sizeof(int3))
    errx(1, "int3 is too high\n");
    ldt_nonexistent_sel = LDT3(LDT_OFFSET + 2);
    const struct user_desc code16_desc = {
    .entry_number    = LDT_OFFSET + 0,
    .base_addr       = (unsigned long)int3,
    .limit           = 4095,
    .seg_32bit       = 0,
    .contents        = 2, /* Code, not conforming */
    .read_exec_only  = 0,
    .limit_in_pages  = 0,
    .seg_not_present = 0,
    .useable         = 0
    };
    add_ldt(&code16_desc, &code16_sel, "code16");
    const struct user_desc data16_desc = {
    .entry_number    = LDT_OFFSET + 1,
    .base_addr       = (unsigned long)stack16,
    .limit           = 0xffff,
    .seg_32bit       = 0,
    .contents        = 0, /* Data, grow-up */
    .read_exec_only  = 0,
    .limit_in_pages  = 0,
    .seg_not_present = 0,
    .useable         = 0
    };
    add_ldt(&data16_desc, &data16_sel, "data16");
    const struct user_desc npcode32_desc = {
    .entry_number    = LDT_OFFSET + 3,
    .base_addr       = (unsigned long)int3,
    .limit           = 4095,
    .seg_32bit       = 1,
    .contents        = 2, /* Code, not conforming */
    .read_exec_only  = 0,
    .limit_in_pages  = 0,
    .seg_not_present = 1,
    .useable         = 0
    };
    add_ldt(&npcode32_desc, &npcode32_sel, "npcode32");
    const struct user_desc npdata32_desc = {
    .entry_number    = LDT_OFFSET + 4,
    .base_addr       = (unsigned long)stack16,
    .limit           = 0xffff,
    .seg_32bit       = 1,
    .contents        = 0, /* Data, grow-up */
    .read_exec_only  = 0,
    .limit_in_pages  = 0,
    .seg_not_present = 1,
    .useable         = 0
    };
    add_ldt(&npdata32_desc, &npdata32_sel, "npdata32");
    struct user_desc gdt_data16_desc = {
    .entry_number    = -1,
    .base_addr       = (unsigned long)stack16,
    .limit           = 0xffff,
    .seg_32bit       = 0,
    .contents        = 0, /* Data, grow-up */
    .read_exec_only  = 0,
    .limit_in_pages  = 0,
    .seg_not_present = 0,
    .useable         = 0
    };
    if (syscall(SYS_set_thread_area, &gdt_data16_desc) == 0) {
//
// This probably indicates vulnerability to CVE-2014-8133.
// Merely getting here isn't definitive, though, and we'll
// diagnose the problem for real later on.
//
    printf("[WARN]\tset_thread_area allocated data16 at index %d\n",
    gdt_data16_desc.entry_number);
    gdt_data16_idx = gdt_data16_desc.entry_number;
    } else {
    printf("[OK]\tset_thread_area refused 16-bit data\n");
    }
    struct user_desc gdt_npdata32_desc = {
    .entry_number    = -1,
    .base_addr       = (unsigned long)stack16,
    .limit           = 0xffff,
    .seg_32bit       = 1,
    .contents        = 0, /* Data, grow-up */
    .read_exec_only  = 0,
    .limit_in_pages  = 0,
    .seg_not_present = 1,
    .useable         = 0
    };
    if (syscall(SYS_set_thread_area, &gdt_npdata32_desc) == 0) {
//
// As a hardening measure, newer kernels don't allow this.
//
    printf("[WARN]\tset_thread_area allocated npdata32 at index %d\n",
    gdt_npdata32_desc.entry_number);
    gdt_npdata32_idx = gdt_npdata32_desc.entry_number;
    } else {
    printf("[OK]\tset_thread_area refused 16-bit data\n");
    }
    }
// State used by our signal handlers.
    static gregset_t initial_regs, requested_regs, resulting_regs;
// Instructions for the SIGUSR1 handler.
    static volatile unsigned short sig_cs, sig_ss;
    static volatile sig_atomic_t sig_trapped, sig_err, sig_trapno;

    static volatile sig_atomic_t sig_corrupt_final_ss;

// Abstractions for some 32-bit vs 64-bit differences.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct selectors {
    pub ss: unsigned short cs, gs, fs,,
}

    static unsigned short *ssptr(ucontext_t *ctx)
    {
    struct selectors *sels = (void *)&ctx.uc_mcontext.gregs[REG_CSGSFS];
    return &sels.ss;
    }
    static unsigned short *csptr(ucontext_t *ctx)
    {
    struct selectors *sels = (void *)&ctx.uc_mcontext.gregs[REG_CSGSFS];
    return &sels.cs;
    }

    static greg_t *ssptr(ucontext_t *ctx)
    {
    return &ctx.uc_mcontext.gregs[REG_SS];
    }
    static greg_t *csptr(ucontext_t *ctx)
    {
    return &ctx.uc_mcontext.gregs[REG_CS];
    }

//
// Checks a given selector for its code bitness or returns -1 if it's not
// a usable code segment selector.
//
#[no_mangle]
pub unsafe extern "C" fn cs_bitness(cs: c_ushort) -> c_int {
    int cs_bitness(unsigned short cs)
    {
    let mut valid: u32 = 0, ar;
    asm ("lar %[cs], %[ar]\n\t"
    "jnz 1f\n\t"
    "mov $1, %[valid]\n\t"
    "1:"
    : [ar] "=r" (ar), [valid] "+rm" (valid)
    : [cs] "r" (cs));
    if (!valid)
    return -1;
    let mut db: bool = (ar & (1 << 22));
    let mut l: bool = (ar & (1 << 21));
    if (!(ar & (1<<11)))
    return -1;	/* Not code. */
    if (l && !db)
    return 64;
#[no_mangle]
pub unsafe extern "C" fn if(db: !l &&) -> else {
    else if (!l && db)
    return 32;
#[no_mangle]
pub unsafe extern "C" fn if(!db: !l &&) -> else {
    else if (!l && !db)
    return 16;
    else
    return -1;	/* Unknown bitness. */
    }
//
// Checks a given selector for its code bitness or returns -1 if it's not
// a usable code segment selector.
//
#[no_mangle]
pub unsafe extern "C" fn is_valid_ss(cs: c_ushort) -> bool {
    bool is_valid_ss(unsigned short cs)
    {
    let mut valid: u32 = 0, ar;
    asm ("lar %[cs], %[ar]\n\t"
    "jnz 1f\n\t"
    "mov $1, %[valid]\n\t"
    "1:"
    : [ar] "=r" (ar), [valid] "+rm" (valid)
    : [cs] "r" (cs));
    if (!valid)
    return false;
    if ((ar & AR_TYPE_MASK) != AR_TYPE_RWDATA &&
    (ar & AR_TYPE_MASK) != AR_TYPE_RWDATA_EXPDOWN)
    return false;
    return (ar & AR_P);
    }
// Number of errors in the current test case.
    static volatile sig_atomic_t nerrs;
#[no_mangle]
unsafe extern "C" fn validate_signal_ss(sig: c_int, ctx: *mut ucontext_t) {
    static void validate_signal_ss(int sig, ucontext_t *ctx)
    {

    let mut was_64bit: bool = (cs_bitness(*csptr(ctx)) == 64);
    if (!(ctx.uc_flags & UC_SIGCONTEXT_SS)) {
    printf("[FAIL]\tUC_SIGCONTEXT_SS was not set\n");
    nerrs++;
//
// This happens on Linux 4.1.  The rest will fail, too, so
// return now to reduce the noise.
//
    return;
    }
// UC_STRICT_RESTORE_SS is set iff we came from 64-bit mode.
    if (!!(ctx.uc_flags & UC_STRICT_RESTORE_SS) != was_64bit) {
    printf("[FAIL]\tUC_STRICT_RESTORE_SS was wrong in signal %d\n",
    sig);
    nerrs++;
    }
    if (is_valid_ss(*ssptr(ctx))) {
//
// DOSEMU was written before 64-bit sigcontext had SS, and
// it tries to figure out the signal source SS by looking at
// the physical register.  Make sure that keeps working.
//
    unsigned short hw_ss;
    asm ("mov %%ss, %0" : "=rm" (hw_ss));
    if (hw_ss != *ssptr(ctx)) {
    printf("[FAIL]\tHW SS didn't match saved SS\n");
    nerrs++;
    }
    }

    }
//
// SIGUSR1 handler.  Sets CS and SS as requested and points IP to the
// int3 trampoline.  Sets SP to a large known value so that we can see
// whether the value round-trips back to user mode correctly.
//
#[no_mangle]
unsafe extern "C" fn sigusr1(sig: c_int, info: *mut siginfo_t, ctx_void: *mut c_void) {
    static void sigusr1(int sig, siginfo_t *info, void *ctx_void)
    {
    ucontext_t *ctx = (ucontext_t*)ctx_void;
    validate_signal_ss(sig, ctx);
    memcpy(&initial_regs, &ctx.uc_mcontext.gregs, sizeof(gregset_t));
// csptr(ctx) = sig_cs;
// ssptr(ctx) = sig_ss;
    ctx.uc_mcontext.gregs[REG_IP] =
    sig_cs == code16_sel ? 0 : (unsigned long)&int3;
    ctx.uc_mcontext.gregs[REG_SP] = (unsigned long)0x8badf00d5aadc0deULL;
    ctx.uc_mcontext.gregs[REG_CX] = 0;

//
// Make sure the kernel doesn't inadvertently use DS or ES-relative
// accesses in a region where user DS or ES is loaded.
//
// Skip this for 64-bit builds because long mode doesn't care about
// DS and ES and skipping it increases test coverage a little bit,
// since 64-bit kernels can still run the 32-bit build.
//
    ctx.uc_mcontext.gregs[REG_DS] = 0;
    ctx.uc_mcontext.gregs[REG_ES] = 0;

    memcpy(&requested_regs, &ctx.uc_mcontext.gregs, sizeof(gregset_t));
    requested_regs[REG_CX] = *ssptr(ctx);	/* The asm code does this. */
    return;
    }
//
// Called after a successful sigreturn (via int3) or from a failed
// sigreturn (directly by kernel).  Restores our state so that the
// original raise(SIGUSR1) returns.
//
#[no_mangle]
unsafe extern "C" fn sigtrap(sig: c_int, info: *mut siginfo_t, ctx_void: *mut c_void) {
    static void sigtrap(int sig, siginfo_t *info, void *ctx_void)
    {
    ucontext_t *ctx = (ucontext_t*)ctx_void;
    validate_signal_ss(sig, ctx);
    sig_err = ctx.uc_mcontext.gregs[REG_ERR];
    sig_trapno = ctx.uc_mcontext.gregs[REG_TRAPNO];
    unsigned short ss;
    asm ("mov %%ss,%0" : "=r" (ss));
    let mut asm_ss: greg_t = ctx.uc_mcontext.gregs[REG_CX];
    if (asm_ss != sig_ss && sig == SIGTRAP) {
// Sanity check failure.
    printf("[FAIL]\tSIGTRAP: ss = %hx, frame ss = %x, ax = %llx\n",
    ss, *ssptr(ctx), (unsigned long long)asm_ss);
    nerrs++;
    }
    memcpy(&resulting_regs, &ctx.uc_mcontext.gregs, sizeof(gregset_t));
    memcpy(&ctx.uc_mcontext.gregs, &initial_regs, sizeof(gregset_t));

    if (sig_corrupt_final_ss) {
    if (ctx.uc_flags & UC_STRICT_RESTORE_SS) {
    printf("[FAIL]\tUC_STRICT_RESTORE_SS was set inappropriately\n");
    nerrs++;
    } else {
//
// DOSEMU transitions from 32-bit to 64-bit mode by
// adjusting sigcontext, and it requires that this work
// even if the saved SS is bogus.
//
    printf("\tCorrupting SS on return to 64-bit mode\n");
// ssptr(ctx) = 0;
    }
    }

    sig_trapped = sig;
    }

// Tests recovery if !UC_STRICT_RESTORE_SS
#[no_mangle]
unsafe extern "C" fn sigusr2(sig: c_int, info: *mut siginfo_t, ctx_void: *mut c_void) {
    static void sigusr2(int sig, siginfo_t *info, void *ctx_void)
    {
    ucontext_t *ctx = (ucontext_t*)ctx_void;
    if (!(ctx.uc_flags & UC_STRICT_RESTORE_SS)) {
    printf("[FAIL]\traise(2) didn't set UC_STRICT_RESTORE_SS\n");
    nerrs++;
    return;  /* We can't do the rest. */
    }
    ctx.uc_flags &= ~UC_STRICT_RESTORE_SS;
// ssptr(ctx) = 0;
// Return.  The kernel should recover without sending another signal.
    }
#[no_mangle]
unsafe extern "C" fn test_nonstrict_ss() -> c_int {
    static int test_nonstrict_ss(void)
    {
    clearhandler(SIGUSR1);
    clearhandler(SIGTRAP);
    clearhandler(SIGSEGV);
    clearhandler(SIGILL);
    sethandler(SIGUSR2, sigusr2, 0);
    nerrs = 0;
    printf("[RUN]\tClear UC_STRICT_RESTORE_SS and corrupt SS\n");
    raise(SIGUSR2);
    if (!nerrs)
    printf("[OK]\tIt worked\n");
    return nerrs;
    }

// Finds a usable code segment of the requested bitness.
#[no_mangle]
pub unsafe extern "C" fn find_cs(bitness: c_int) -> c_int {
    int find_cs(int bitness)
    {
    unsigned short my_cs;
    asm ("mov %%cs,%0" :  "=r" (my_cs));
    if (cs_bitness(my_cs) == bitness)
    return my_cs;
    if (cs_bitness(my_cs + (2 << 3)) == bitness)
    return my_cs + (2 << 3);
    if (my_cs > (2<<3) && cs_bitness(my_cs - (2 << 3)) == bitness)
    return my_cs - (2 << 3);
    if (cs_bitness(code16_sel) == bitness)
    return code16_sel;
    printf("[WARN]\tCould not find %d-bit CS\n", bitness);
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn test_valid_sigreturn(cs_bits: c_int, use_16bit_ss: bool, force_ss: c_int) -> c_int {
    static int test_valid_sigreturn(int cs_bits, bool use_16bit_ss, int force_ss)
    {
    let mut cs: c_int = find_cs(cs_bits);
    if (cs == -1) {
    printf("[SKIP]\tCode segment unavailable for %d-bit CS, %d-bit SS\n",
    cs_bits, use_16bit_ss ? 16 : 32);
    return 0;
    }
    if (force_ss != -1) {
    sig_ss = force_ss;
    } else {
    if (use_16bit_ss) {
    if (!data16_sel) {
    printf("[SKIP]\tData segment unavailable for %d-bit CS, 16-bit SS\n",
    cs_bits);
    return 0;
    }
    sig_ss = data16_sel;
    } else {
    asm volatile ("mov %%ss,%0" : "=r" (sig_ss));
    }
    }
    sig_cs = cs;
    printf("[RUN]\tValid sigreturn: %d-bit CS (%hx), %d-bit SS (%hx%s)\n",
    cs_bits, sig_cs, use_16bit_ss ? 16 : 32, sig_ss,
    (sig_ss & 4) ? "" : ", GDT");
    raise(SIGUSR1);
    nerrs = 0;
//
// Check that each register had an acceptable value when the
// int3 trampoline was invoked.
//
    for (int i = 0; i < NGREG; i++) {
    let mut req: greg_t = requested_regs[i], res = resulting_regs[i];
    if (i == REG_TRAPNO || i == REG_IP)
    continue;	/* don't care */
    if (i == REG_SP) {
//
// If we were using a 16-bit stack segment, then
// the kernel is a bit stuck: IRET only restores
// the low 16 bits of ESP/RSP if SS is 16-bit.
// The kernel uses a hack to restore bits 31:16,
// but that hack doesn't help with bits 63:32.
// On Intel CPUs, bits 63:32 end up zeroed, and, on
// AMD CPUs, they leak the high bits of the kernel
// espfix64 stack pointer.  There's very little that
// the kernel can do about it.
//
// Similarly, if we are returning to a 32-bit context,
// the CPU will often lose the high 32 bits of RSP.
//
    if (res == req)
    continue;
    if (cs_bits != 64 && ((res ^ req) & 0xFFFFFFFF) == 0) {
    printf("[NOTE]\tSP: %llx . %llx\n",
    (unsigned long long)req,
    (unsigned long long)res);
    continue;
    }
    printf("[FAIL]\tSP mismatch: requested 0x%llx; got 0x%llx\n",
    (unsigned long long)requested_regs[i],
    (unsigned long long)resulting_regs[i]);
    nerrs++;
    continue;
    }
    let mut ignore_reg: bool = false;

    if (i == REG_UESP)
    ignore_reg = true;

    if (i == REG_CSGSFS) {
    struct selectors *req_sels =
    (void *)&requested_regs[REG_CSGSFS];
    struct selectors *res_sels =
    (void *)&resulting_regs[REG_CSGSFS];
    if (req_sels.cs != res_sels.cs) {
    printf("[FAIL]\tCS mismatch: requested 0x%hx; got 0x%hx\n",
    req_sels.cs, res_sels.cs);
    nerrs++;
    }
    if (req_sels.ss != res_sels.ss) {
    printf("[FAIL]\tSS mismatch: requested 0x%hx; got 0x%hx\n",
    req_sels.ss, res_sels.ss);
    nerrs++;
    }
    continue;
    }

// Sanity check on the kernel
    if (i == REG_CX && req != res) {
    printf("[FAIL]\tCX (saved SP) mismatch: requested 0x%llx; got 0x%llx\n",
    (unsigned long long)req,
    (unsigned long long)res);
    nerrs++;
    continue;
    }
    if (req != res && !ignore_reg) {
    printf("[FAIL]\tReg %d mismatch: requested 0x%llx; got 0x%llx\n",
    i, (unsigned long long)req,
    (unsigned long long)res);
    nerrs++;
    }
    }
    if (nerrs == 0)
    printf("[OK]\tall registers okay\n");
    return nerrs;
    }
#[no_mangle]
unsafe extern "C" fn test_bad_iret(cs_bits: c_int, ss: c_ushort, force_cs: c_int) -> c_int {
    static int test_bad_iret(int cs_bits, unsigned short ss, int force_cs)
    {
    let mut cs: c_int = force_cs == -1 ? find_cs(cs_bits) : force_cs;
    if (cs == -1)
    return 0;
    sig_cs = cs;
    sig_ss = ss;
    printf("[RUN]\t%d-bit CS (%hx), bogus SS (%hx)\n",
    cs_bits, sig_cs, sig_ss);
    sig_trapped = 0;
    raise(SIGUSR1);
    if (sig_trapped) {
    char errdesc[32] = "";
    if (sig_err) {
    const char *src = (sig_err & 1) ? " EXT" : "";
    const char *table;
    if ((sig_err & 0x6) == 0x0)
    table = "GDT";
#[no_mangle]
pub unsafe extern "C" fn if(0x4: (sig_err & 0x6) ==) -> else {
    else if ((sig_err & 0x6) == 0x4)
    table = "LDT";
#[no_mangle]
pub unsafe extern "C" fn if(0x2: (sig_err & 0x6) ==) -> else {
    else if ((sig_err & 0x6) == 0x2)
    table = "IDT";
    else
    table = "???";
    sprintf(errdesc, "%s%s index %d, ",
    table, src, sig_err >> 3);
    }
    char trapname[32];
    if (sig_trapno == 13)
    strcpy(trapname, "GP");
#[no_mangle]
pub unsafe extern "C" fn if(11: sig_trapno ==) -> else {
    else if (sig_trapno == 11)
    strcpy(trapname, "NP");
#[no_mangle]
pub unsafe extern "C" fn if(12: sig_trapno ==) -> else {
    else if (sig_trapno == 12)
    strcpy(trapname, "SS");
#[no_mangle]
pub unsafe extern "C" fn if(32: sig_trapno ==) -> else {
    else if (sig_trapno == 32)
    strcpy(trapname, "IRET");  /* X86_TRAP_IRET */
    else
    sprintf(trapname, "%d", sig_trapno);
    printf("[OK]\tGot #%s(0x%lx) (i.e. %s%s)\n",
    trapname, (unsigned long)sig_err,
    errdesc, strsignal(sig_trapped));
    return 0;
    } else {
//
// This also implicitly tests UC_STRICT_RESTORE_SS:
// We check that these signals set UC_STRICT_RESTORE_SS and,
// if UC_STRICT_RESTORE_SS doesn't cause strict behavior,
// then we won't get SIGSEGV.
//
    printf("[FAIL]\tDid not get SIGSEGV\n");
    return 1;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main()
    {
    let mut total_nerrs: c_int = 0;
    unsigned short my_cs, my_ss;
    asm volatile ("mov %%cs,%0" : "=r" (my_cs));
    asm volatile ("mov %%ss,%0" : "=r" (my_ss));
    setup_ldt();
    stack_t stack = {
// Our sigaltstack scratch space.
    .ss_sp = malloc(sizeof(char) * SIGSTKSZ),
    .ss_size = SIGSTKSZ,
    };
    if (sigaltstack(&stack, core::ptr::null_mut()) != 0)
    err(1, "sigaltstack");
    sethandler(SIGUSR1, sigusr1, 0);
    sethandler(SIGTRAP, sigtrap, SA_ONSTACK);
// Easy cases: return to a 32-bit SS in each possible CS bitness.
    total_nerrs += test_valid_sigreturn(64, false, -1);
    total_nerrs += test_valid_sigreturn(32, false, -1);
    total_nerrs += test_valid_sigreturn(16, false, -1);
//
// Test easy espfix cases: return to a 16-bit LDT SS in each possible
// CS bitness.  NB: with a long mode CS, the SS bitness is irrelevant.
//
// This catches the original missing-espfix-on-64-bit-kernels issue
// as well as CVE-2014-8134.
//
    total_nerrs += test_valid_sigreturn(64, true, -1);
    total_nerrs += test_valid_sigreturn(32, true, -1);
    total_nerrs += test_valid_sigreturn(16, true, -1);
    if (gdt_data16_idx) {
//
// For performance reasons, Linux skips espfix if SS points
// to the GDT.  If we were able to allocate a 16-bit SS in
// the GDT, see if it leaks parts of the kernel stack pointer.
//
// This tests for CVE-2014-8133.
//
    total_nerrs += test_valid_sigreturn(64, true,
    GDT3(gdt_data16_idx));
    total_nerrs += test_valid_sigreturn(32, true,
    GDT3(gdt_data16_idx));
    total_nerrs += test_valid_sigreturn(16, true,
    GDT3(gdt_data16_idx));
    }

// Nasty ABI case: check SS corruption handling.
    sig_corrupt_final_ss = 1;
    total_nerrs += test_valid_sigreturn(32, false, -1);
    total_nerrs += test_valid_sigreturn(32, true, -1);
    sig_corrupt_final_ss = 0;

//
// We're done testing valid sigreturn cases.  Now we test states
// for which sigreturn itself will succeed but the subsequent
// entry to user mode will fail.
//
// Depending on the failure mode and the kernel bitness, these
// entry failures can generate SIGSEGV, SIGBUS, or SIGILL.
//
    clearhandler(SIGTRAP);
    sethandler(SIGSEGV, sigtrap, SA_ONSTACK);
    sethandler(SIGBUS, sigtrap, SA_ONSTACK);
    sethandler(SIGILL, sigtrap, SA_ONSTACK);  /* 32-bit kernels do this */
// Easy failures: invalid SS, resulting in #GP(0)
    test_bad_iret(64, ldt_nonexistent_sel, -1);
    test_bad_iret(32, ldt_nonexistent_sel, -1);
    test_bad_iret(16, ldt_nonexistent_sel, -1);
// These fail because SS isn't a data segment, resulting in #GP(SS)
    test_bad_iret(64, my_cs, -1);
    test_bad_iret(32, my_cs, -1);
    test_bad_iret(16, my_cs, -1);
// Try to return to a not-present code segment, triggering #NP(SS).
    test_bad_iret(32, my_ss, npcode32_sel);
//
// Try to return to a not-present but otherwise valid data segment.
// This will cause IRET to fail with #SS on the espfix stack.  This
// exercises CVE-2014-9322.
//
// Note that, if espfix is enabled, 64-bit Linux will lose track
// of the actual cause of failure and report #GP(0) instead.
// This would be very difficult for Linux to avoid, because
// espfix64 causes IRET failures to be promoted to #DF, so the
// original exception frame is never pushed onto the stack.
//
    test_bad_iret(32, npdata32_sel, -1);
//
// Try to return to a not-present but otherwise valid data
// segment without invoking espfix.  Newer kernels don't allow
// this to happen in the first place.  On older kernels, though,
// this can trigger CVE-2014-9322.
//
    if (gdt_npdata32_idx)
    test_bad_iret(32, GDT3(gdt_npdata32_idx), -1);

    total_nerrs += test_nonstrict_ss();

    free(stack.ss_sp);
    return total_nerrs ? 1 : 0;
    }
