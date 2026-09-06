//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/tm/tm-resched-dscr.c
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
// Test context switching to see if the DSCR SPR is correctly preserved
// when within a transaction.
//
// Note: We assume that the DSCR has been left at the default value (0)
// for all CPUs.
//
// Method:
//
// Set a value into the DSCR.
//
// Start a transaction, and suspend it (*).
//
// Hard loop checking to see if the transaction has become doomed.
//
// Now that we *may* have been preempted, record the DSCR and TEXASR SPRS.
//
// If the abort was because of a context switch, check the DSCR value.
// Otherwise, try again.
//
// (*) If the transaction is not suspended we can't see the problem because
// the transaction abort handler will restore the DSCR to it's checkpointed
// value before we regain control.
//

pub const SPRN_DSCR: c_uint = 0x03;
#[no_mangle]
pub unsafe extern "C" fn test_body() -> c_int {
    int test_body(void)
    {
    uint64_t rv, dscr1 = 1, dscr2, texasr;
    SKIP_IF(!have_htm());
    SKIP_IF(htm_is_synthetic());
    printf("Check DSCR TM context switch: ");
    fflush(stdout);
    for (;;) {
    asm __volatile__ (
// set a known value into the DSCR
    "ld      3, %[dscr1];"
    "mtspr   %[sprn_dscr], 3;"
    "li      %[rv], 1;"
// start and suspend a transaction
    "tbegin.;"
    "beq     1f;"
    "tsuspend.;"
// hard loop until the transaction becomes doomed
    "2: ;"
    "tcheck 0;"
    "bc      4, 0, 2b;"
// record DSCR and TEXASR
    "mfspr   3, %[sprn_dscr];"
    "std     3, %[dscr2];"
    "mfspr   3, %[sprn_texasr];"
    "std     3, %[texasr];"
    "tresume.;"
    "tend.;"
    "li      %[rv], 0;"
    "1: ;"
    : [rv]"=r"(rv), [dscr2]"=m"(dscr2), [texasr]"=m"(texasr)
    : [dscr1]"m"(dscr1)
    , [sprn_dscr]"i"(SPRN_DSCR), [sprn_texasr]"i"(SPRN_TEXASR)
    : "memory", "r3"
    );
    assert(rv); /* make sure the transaction aborted */
    if ((texasr >> 56) != TM_CAUSE_RESCHED) {
    continue;
    }
    if (dscr2 != dscr1) {
    printf(" FAIL\n");
    return 1;
    } else {
    printf(" OK\n");
    return 0;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn tm_resched_dscr() -> c_int {
    static int tm_resched_dscr(void)
    {
    return eat_cpu(test_body);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *const c_char) -> c_int {
    int main(int argc, const char *argv[])
    {
    return test_harness(tm_resched_dscr, "tm_resched_dscr");
    }
