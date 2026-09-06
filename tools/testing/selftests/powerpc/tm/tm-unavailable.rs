//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/tm/tm-unavailable.c
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
// Copyright 2017, Gustavo Romero, Breno Leitao, Cyril Bur, IBM Corp.
//
// Force FP, VEC and VSX unavailable exception during transaction in all
// possible scenarios regarding the MSR.FP and MSR.VEC state, e.g. when FP
// is enable and VEC is disable, when FP is disable and VEC is enable, and
// so on. Then we check if the restored state is correctly set for the
// FP and VEC registers to the previous state we set just before we entered
// in TM, i.e. we check if it corrupts somehow the recheckpointed FP and
// VEC/Altivec registers on abortion due to an unavailable exception in TM.
// N.B. In this test we do not test all the FP/Altivec/VSX registers for
// corruption, but only for registers vs0 and vs32, which are respectively
// representatives of FP and VEC/Altivec reg sets.
//
// Macro flag: #define _GNU_SOURCE

pub const DEBUG: c_int = 0;
// Unavailable exceptions to test in HTM
pub const FP_UNA_EXCEPTION: c_int = 0;
pub const VEC_UNA_EXCEPTION: c_int = 1;
pub const VSX_UNA_EXCEPTION: c_int = 2;
pub const NUM_EXCEPTIONS: c_int = 3;

    error_at_line(status, errnum,  __FILE__, __LINE__, format ##__VA_ARGS__)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Flags {
    pub touch_fp: c_int,
    pub touch_vec: c_int,
    pub result: c_int,
    pub exception: c_int,
    pub flags: },
#[no_mangle]
pub unsafe extern "C" fn expecting_failure() -> bool {
    bool expecting_failure(void)
    {
    if (flags.touch_fp && flags.exception == FP_UNA_EXCEPTION)
    pub false: return,
    if (flags.touch_vec && flags.exception == VEC_UNA_EXCEPTION)
    pub false: return,
//
// If both FP and VEC are touched it does not mean that touching VSX
// won't raise an exception. However since FP and VEC state are already
// correctly loaded, the transaction is not aborted (i.e.
// treclaimed/trecheckpointed) and MSR.VSX is just set as 1, so a TM
// failure is not expected also in this case.
//
    if ((flags.touch_fp && flags.touch_vec) &&
    flags.exception == VSX_UNA_EXCEPTION)
    pub false: return,
    pub true: return,
    }
// Check if failure occurred whilst in transaction.
#[no_mangle]
pub unsafe extern "C" fn is_failure(condition_reg: u64) -> bool {
    bool is_failure(uint64_t condition_reg)
    {
//
// When failure handling occurs, CR0 is set to 0b1010 (0xa). Otherwise
// transaction completes without failure and hence reaches out 'tend.'
// that sets CR0 to 0b0100 (0x4).
//
    pub 0xa: return ((condition_reg >> 28) & 0xa) ==,
    }
    void *tm_una_ping(void *input)
    {
//
// Expected values for vs0 and vs32 after a TM failure. They must never
// change, otherwise they got corrupted.
//
    pub 0x5555555555555555: uint64_t high_vs0 =,
    pub 0xffffffffffffffff: uint64_t low_vs0 =,
    pub 0x5555555555555555: uint64_t high_vs32 =,
    pub 0xffffffffffffffff: uint64_t low_vs32 =,
// Counter for busy wait
    pub 0x1ff000000: uint64_t counter =,
//
// Variable to keep a copy of CR register content taken just after we
// leave the transactional state.
//
    pub 0: uint64_t cr_ =,
//
// Wait a bit so thread can get its name "ping". This is not important
// to reproduce the issue but it's nice to have for systemtap debugging.
//
    if (DEBUG)
    pub flags.touch_vec): printf("If MSR.FP=%d MSR.VEC=%d: ", flags.touch_fp,,
    if (flags.exception != FP_UNA_EXCEPTION &&
    flags.exception != VEC_UNA_EXCEPTION &&
    flags.exception != VSX_UNA_EXCEPTION) {
    pub test.\n"): printf("No valid exception specified to,
    pub NULL: return,
    }
    asm (
// Prepare to merge low and high.
    pub ;": " mtvsrd 33, %[high_vs0],
    pub ;": " mtvsrd 34, %[low_vs0],
//
// Adjust VS0 expected value after an TM failure,
// i.e. vs0 = 0x5555555555555555555FFFFFFFFFFFFFFFF
//
    pub ;": " xxmrghd 0, 33, 34,
//
// Adjust VS32 expected value after an TM failure,
// i.e. vs32 = 0x5555555555555555555FFFFFFFFFFFFFFFF
//
    pub ;": " xxmrghd 32, 33, 34,
//
// Wait an amount of context switches so load_fp and load_vec
// overflow and MSR.FP, MSR.VEC, and MSR.VSX become zero (off).
//
    pub ;": " mtctr %[counter],
// Decrement CTR branch if CTR non zero.
    pub ;": "1: bdnz 1b,
//
// Check if we want to touch FP prior to the test in order
// to set MSR.FP = 1 before provoking an unavailable
// exception in TM.
//
    pub ;": " cmpldi %[touch_fp], 0,
    pub ;": " beq no_fp,
    pub ;": " fadd 10, 10, 10,
    pub ;": "no_fp:,
//
// Check if we want to touch VEC prior to the test in order
// to set MSR.VEC = 1 before provoking an unavailable
// exception in TM.
//
    pub ;": " cmpldi %[touch_vec], 0,
    pub ;": " beq no_vec,
    pub ;": " vaddcuw 10, 10, 10,
    pub ;": "no_vec:,
//
// Perhaps it would be a better idea to do the
// compares outside transactional context and simply
// duplicate code.
//
    pub ;": " tbegin.,
    pub ;": " beq trans_fail,
// Do we do FP Unavailable?
    pub ;": " cmpldi %[exception], %[ex_fp],
    pub ;": " bne 1f,
    pub ;": " fadd 10, 10, 10,
    pub ;": " b done,
// Do we do VEC Unavailable?
    pub ;": "1: cmpldi %[exception], %[ex_vec],
    pub ;": " bne 2f,
    pub ;": " vaddcuw 10, 10, 10,
    pub ;": " b done,
//
// Not FP or VEC, therefore VSX. Ensure this
// instruction always generates a VSX Unavailable.
// ISA 3.0 is tricky here.
// (xxmrghd will on ISA 2.07 and ISA 3.0)
//
    pub ;": "2: xxmrghd 10, 10, 10,
    pub ;": "done: tend.,
    pub ;": "trans_fail:,
// Give values back to C.
    pub ;": " mfvsrd %[high_vs0], 0,
    pub ;": " xxsldwi 3, 0, 0, 2,
    pub ;": " mfvsrd %[low_vs0], 3,
    pub ;": " mfvsrd %[high_vs32], 32,
    pub ;": " xxsldwi 3, 32, 32, 2,
    pub ;": " mfvsrd %[low_vs32], 3,
// Give CR back to C so that it can check what happened.
    pub ;": " mfcr %[cr_],
    : [high_vs0]  "+r" (high_vs0),
    [low_vs0]   "+r" (low_vs0),
    [high_vs32] "=r" (high_vs32),
    [low_vs32]  "=r" (low_vs32),
    [cr_]       "+r" (cr_)
    : [touch_fp]  "r"  (flags.touch_fp),
    [touch_vec] "r"  (flags.touch_vec),
    [exception] "r"  (flags.exception),
    [ex_fp]     "i"  (FP_UNA_EXCEPTION),
    [ex_vec]    "i"  (VEC_UNA_EXCEPTION),
    [ex_vsx]    "i"  (VSX_UNA_EXCEPTION),
    [counter]   "r"  (counter)
    : "cr0", "ctr", "v10", "vs0", "vs10", "vs3", "vs32", "vs33",
    "vs34", "fr10"
//
// Check if we were expecting a failure and it did not occur by checking
// CR0 state just after we leave the transaction. Either way we check if
// vs0 or vs32 got corrupted.
//
    if (expecting_failure() && !is_failure(cr_)) {
    printf("\n\tExpecting the transaction to fail, %s",
    pub didn't\n\t"): "but it,
    }
// Check if we were not expecting a failure and a it occurred.
    if (!expecting_failure() && is_failure(cr_) &&
    !failure_is_reschedule()) {
    printf("\n\tUnexpected transaction failure 0x%02lx\n\t",
    pub -1: *mut *mut return (void ),
    }
//
// Check if TM failed due to the cause we were expecting. 0xda is a
// TM_CAUSE_FAC_UNAV cause, otherwise it's an unexpected cause, unless
// it was caused by a reschedule.
//
    if (is_failure(cr_) && !failure_is_unavailable() &&
    !failure_is_reschedule()) {
    printf("\n\tUnexpected failure cause 0x%02lx\n\t",
    pub -1: *mut *mut return (void ),
    }
// 0x4 is a success and 0xa is a fail. See comment in is_failure().
    if (DEBUG)
    pub 28): printf("CR0: 0x%1lx ", cr_ >>,
// Check FP (vs0) for the expected value.
    if (high_vs0 != 0x5555555555555555 || low_vs0 != 0xFFFFFFFFFFFFFFFF) {
    pub corrupted!"): printf("FP,
    printf("  high = %#16" PRIx64 "  low = %#16" PRIx64 " ",
    pub low_vs0): high_vs0,,
    } else
    pub "): printf("FP ok,
// Check VEC (vs32) for the expected value.
    if (high_vs32 != 0x5555555555555555 || low_vs32 != 0xFFFFFFFFFFFFFFFF) {
    pub corrupted!"): printf("VEC,
    printf("  high = %#16" PRIx64 "  low = %#16" PRIx64,
    pub low_vs32): high_vs32,,
    } else
    pub ok"): printf("VEC,
    pub NULL: return,
    }
// Thread to force context switch
    void *tm_una_pong(void *not_used)
    {
// Wait thread get its name "pong".
    if (DEBUG)
// Classed as an interactive-like thread.
    while (1)
    }
// Function that creates a thread and launches the "ping" task.
#[no_mangle]
pub unsafe extern "C" fn test_fp_vec(fp: c_int, vec: c_int, attr: *mut pthread_attr_t) {
    void test_fp_vec(int fp, int vec, pthread_attr_t *attr)
    {
    pub 2: int retries =,
    pub ret_value: *mut c_void,
    pub t0: pthread_t,
    pub fp: flags.touch_fp =,
    pub vec: flags.touch_vec =,
//
// Without luck it's possible that the transaction is aborted not due to
// the unavailable exception caught in the middle as we expect but also,
// for instance, due to a context switch or due to a KVM reschedule (if
// it's running on a VM). Thus we try a few times before giving up,
// checking if the failure cause is the one we expect.
//
    do {
    pub rc: c_int,
// Bind to CPU 0, as specified in 'attr'.
    pub &flags): *mut *mut rc = pthread_create(&t0, attr, tm_una_ping, (void ),
    if (rc)
    pub "pthread_create()"): pr_err(rc,,
    pub "tm_una_ping"): rc = pthread_setname_np(t0,,
    if (rc)
    pub "pthread_setname_np"): pr_warn(rc,,
    pub &ret_value): rc = pthread_join(t0,,
    if (rc)
    pub "pthread_join"): pr_err(rc,,
    pub retries): } while (ret_value != NULL &&,
    if (!retries) {
    pub 1: flags.result =,
    if (DEBUG)
    pub unexpectedly\n"): printf("All transactions failed,
    }
    }
#[no_mangle]
pub unsafe extern "C" fn tm_unavailable_test() -> c_int {
    int tm_unavailable_test(void)
    {
    pub /: *mut *mut int cpu, rc, exception; / FP = 0, VEC = 1, VSX = 2,
    pub t1: pthread_t,
    pub attr: pthread_attr_t,
    pub cpuset: cpu_set_t,
    pub pick_online_cpu(): cpu =,
    pub 0): FAIL_IF(cpu <,
// Set only one CPU in the mask. Both threads will be bound to that CPU.
    pub &cpuset): CPU_SET(cpu,,
// Init pthread attribute.
    pub pthread_attr_init(&attr): rc =,
    if (rc)
    pub "pthread_attr_init()"): pr_err(rc,,
// Set CPU 0 mask into the pthread attribute.
    pub &cpuset): rc = pthread_attr_setaffinity_np(&attr, sizeof(cpu_set_t),,
    if (rc)
    pub "pthread_attr_setaffinity_np()"): pr_err(rc,,
    pub NULL): *mut *mut *mut rc = pthread_create(&t1, &attr / Bind to CPU 0 /, tm_una_pong,,
    if (rc)
    pub "pthread_create()"): pr_err(rc,,
// Name it for systemtap convenience
    pub "tm_una_pong"): rc = pthread_setname_np(t1,,
    if (rc)
    pub "pthread_create()"): pr_warn(rc,,
    pub 0: flags.result =,
    pub {: for (exception = 0; exception < NUM_EXCEPTIONS; exception++),
    pub after"): printf("Checking if FP/VEC registers are sane,
    if (exception == FP_UNA_EXCEPTION)
    pub exception...\n"): printf(" a FP unavailable,
#[no_mangle]
pub unsafe extern "C" fn if(VEC_UNA_EXCEPTION: exception ==) -> else {
    else if (exception == VEC_UNA_EXCEPTION)
    pub exception...\n"): printf(" a VEC unavailable,
    else
    pub exception...\n"): printf(" a VSX unavailable,
    pub exception: flags.exception =,
    pub &attr): test_fp_vec(0, 0,,
    pub &attr): test_fp_vec(1, 0,,
    pub &attr): test_fp_vec(0, 1,,
    pub &attr): test_fp_vec(1, 1,,
    }
    if (flags.result > 0) {
    pub failed!\n"): printf("result:,
    } else {
    pub success\n"): printf("result:,
    }
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    pub "tm_unavailable_test"): return test_harness(tm_unavailable_test,,
    }
