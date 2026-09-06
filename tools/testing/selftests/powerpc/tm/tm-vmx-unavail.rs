//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/tm/tm-vmx-unavail.c
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
// Copyright 2017, Michael Neuling, IBM Corp.
// Original: Breno Leitao <brenohl@br.ibm.com> &
// Gustavo Bueno Romero <gromero@br.ibm.com>
// Edited: Michael Neuling
//
// Force VMX unavailable during a transaction and see if it corrupts
// the checkpointed VMX register state after the abort.
//

    int passed;
    void *worker(void *unused)
    {
    __int128 vmx0;
    uint64_t texasr;
    asm goto (
    "li       3, 1;"  /* Stick non-zero value in VMX0 */
    "std      3, 0(%[vmx0_ptr]);"
    "lvx      0, 0, %[vmx0_ptr];"
// Wait here a bit so we get scheduled out 255 times
    "lis      3, 0x3fff;"
    "1: ;"
    "addi     3, 3, -1;"
    "cmpdi    3, 0;"
    "bne      1b;"
// Kernel will hopefully turn VMX off now
    "tbegin. ;"
    "beq      failure;"
// Cause VMX unavail. Any VMX instruction
    "vaddcuw  0,0,0;"
    "tend. ;"
    "b        %l[success];"
// Check VMX0 sanity after abort
    "failure: ;"
    "lvx       1,  0, %[vmx0_ptr];"
    "vcmpequb. 2,  0, 1;"
    "bc        4, 24, %l[value_mismatch];"
    "b        %l[value_match];"
    :
    : [vmx0_ptr] "r"(&vmx0)
    : "r3"
    : success, value_match, value_mismatch
    );
// HTM aborted and VMX0 is corrupted
    value_mismatch:
    texasr = __builtin_get_texasr();
    printf("\n\n==============\n\n");
    printf("Failure with error: %lx\n",   _TEXASR_FAILURE_CODE(texasr));
    printf("Summary error     : %lx\n",   _TEXASR_FAILURE_SUMMARY(texasr));
    printf("TFIAR exact       : %lx\n\n", _TEXASR_TFIAR_EXACT(texasr));
    passed = 0;
    return core::ptr::null_mut();
// HTM aborted but VMX0 is correct
    value_match:
// printf("!");
    return core::ptr::null_mut();
    success:
// printf(".");
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn tm_vmx_unavail_test() -> c_int {
    int tm_vmx_unavail_test()
    {
    int threads;
    pthread_t *thread;
    SKIP_IF(!have_htm());
    SKIP_IF(htm_is_synthetic());
    passed = 1;
    threads = sysconf(_SC_NPROCESSORS_ONLN) * 4;
    thread = malloc(sizeof(pthread_t)*threads);
    if (!thread)
    return EXIT_FAILURE;
    for (uint64_t i = 0; i < threads; i++)
    pthread_create(&thread[i], core::ptr::null_mut(), &worker, core::ptr::null_mut());
    for (uint64_t i = 0; i < threads; i++)
    pthread_join(thread[i], core::ptr::null_mut());
    free(thread);
    return passed ? EXIT_SUCCESS : EXIT_FAILURE;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    return test_harness(tm_vmx_unavail_test, "tm_vmx_unavail_test");
    }
