//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/tm/tm-signal-context-chk-vmx.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 2016, Cyril Bur, IBM Corp.
//
// Test the kernel's signal frame code.
//
// The kernel sets up two sets of ucontexts if the signal was to be
// delivered while the thread was in a transaction (referred too as
// first and second contexts).
// Expected behaviour is that the checkpointed state is in the user
// context passed to the signal handler (first context). The speculated
// state can be accessed with the uc_link pointer (second context).
//
// The rationale for this is that if TM unaware code (which linked
// against TM libs) installs a signal handler it will not know of the
// speculative nature of the 'live' registers and may infer the wrong
// thing.
//

pub const MAX_ATTEMPT: c_int = 500000;

    long tm_signal_self_context_load(pid_t pid, long *gprs, double *fps, vector int *vms, vector int *vss);
    static sig_atomic_t fail, broken;
// Test only non-volatile registers, i.e. 12 vmx registers from vr20 to vr31
    vector int vms[] = {
// First context will be set with these values, i.e. non-speculative
// VMX20     ,  VMX21      , ...
    { 1, 2, 3, 4},{ 5, 6, 7, 8},{ 9,10,11,12},
    {13,14,15,16},{17,18,19,20},{21,22,23,24},
    {25,26,27,28},{29,30,31,32},{33,34,35,36},
    {37,38,39,40},{41,42,43,44},{45,46,47,48},
// Second context will be set with these values, i.e. speculative
// VMX20        , VMX21            , ...
    { -1, -2, -3, -4},{ -5, -6, -7, -8},{ -9,-10,-11,-12},
    {-13,-14,-15,-16},{-17,-18,-19,-20},{-21,-22,-23,-24},
    {-25,-26,-27,-28},{-29,-30,-31,-32},{-33,-34,-35,-36},
    {-37,-38,-39,-40},{-41,-42,-43,-44},{-45,-46,-47,-48}
    };
#[no_mangle]
unsafe extern "C" fn signal_usr1(signum: c_int, info: *mut siginfo_t, uc: *mut c_void) {
    static void signal_usr1(int signum, siginfo_t *info, void *uc)
    {
    int i, j;
    ucontext_t *ucp = uc;
    ucontext_t *tm_ucp = ucp.uc_link;
    for (i = 0; i < NV_VMX_REGS; i++) {
// Check first context. Print all mismatches.
    fail = memcmp(ucp.uc_mcontext.v_regs.vrregs[VMX20 + i],
    &vms[i], sizeof(vector int));
    if (fail) {
    broken = 1;
    printf("VMX%d (1st context) == 0x", VMX20 + i);
// Print actual value in first context.
    for (j = 0; j < 4; j++)
    printf("%08x", ucp.uc_mcontext.v_regs.vrregs[VMX20 + i][j]);
    printf(" instead of 0x");
// Print expected value.
    for (j = 0; j < 4; j++)
    printf("%08x", vms[i][j]);
    printf(" (expected)\n");
    }
    }
    for (i = 0; i < NV_VMX_REGS; i++)  {
// Check second context. Print all mismatches.
    fail = memcmp(tm_ucp.uc_mcontext.v_regs.vrregs[VMX20 + i],
    &vms[NV_VMX_REGS + i], sizeof (vector int));
    if (fail) {
    broken = 1;
    printf("VMX%d (2nd context) == 0x", NV_VMX_REGS + i);
// Print actual value in second context.
    for (j = 0; j < 4; j++)
    printf("%08x", tm_ucp.uc_mcontext.v_regs.vrregs[VMX20 + i][j]);
    printf(" instead of 0x");
// Print expected value.
    for (j = 0; j < 4; j++)
    printf("%08x", vms[NV_VMX_REGS + i][j]);
    printf(" (expected)\n");
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn tm_signal_context_chk() -> c_int {
    static int tm_signal_context_chk()
    {
    struct sigaction act;
    int i;
    long rc;
    let mut pid: pid_t = getpid();
    SKIP_IF(!have_htm());
    SKIP_IF(htm_is_synthetic());
    act.sa_sigaction = signal_usr1;
    sigemptyset(&act.sa_mask);
    act.sa_flags = SA_SIGINFO;
    if (sigaction(SIGUSR1, &act, core::ptr::null_mut()) < 0) {
    perror("sigaction sigusr1");
    exit(1);
    }
    i = 0;
    while (i < MAX_ATTEMPT && !broken) {
//
// tm_signal_self_context_load will set both first and second
// contexts accordingly to the values passed through non-NULL
// array pointers to it, in that case 'vms', and invoke the
// signal handler installed for SIGUSR1.
//
    rc = tm_signal_self_context_load(pid, core::ptr::null_mut(), core::ptr::null_mut(), vms, core::ptr::null_mut());
    FAIL_IF(rc != pid);
    i++;
    }
    return (broken);
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(tm_signal_context_chk, "tm_signal_context_chk_vmx");
    }
