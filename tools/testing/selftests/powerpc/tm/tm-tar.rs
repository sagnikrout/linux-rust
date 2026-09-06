//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/tm/tm-tar.c
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
// Copyright 2015, Michael Neuling, IBM Corp.
// Original: Michael Neuling 19/7/2013
// Edited: Rashmica Gupta 01/12/2015
//
// Do some transactions, see if the tar is corrupted.
// If the transaction is aborted, the TAR should be rolled back to the
// checkpointed value before the transaction began. The value written to
// TAR in suspended mode should only remain in TAR if the transaction
// completes.
//

    let mut num_loops: c_int = 10000;
#[no_mangle]
pub unsafe extern "C" fn test_tar() -> c_int {
    int test_tar(void)
    {
    int i;
    SKIP_IF(!have_htm());
    SKIP_IF(htm_is_synthetic());
    SKIP_IF(!is_ppc64le());
    for (i = 0; i < num_loops; i++)
    {
    let mut result: u64 = 0;
    asm __volatile__(
    "li	7, 1;"
    "mtspr	%[tar], 7;"	/* tar = 1 */
    "tbegin.;"
    "beq	3f;"
    "li	4, 0x7000;"	/* Loop lots, to use time */
    "2:;"			/* Start loop */
    "li	7, 2;"
    "mtspr	%[tar], 7;"	/* tar = 2 */
    "tsuspend.;"
    "li	7, 3;"
    "mtspr	%[tar], 7;"	/* tar = 3 */
    "tresume.;"
    "subi	4, 4, 1;"
    "cmpdi	4, 0;"
    "bne	2b;"
    "tend.;"
// Transaction sucess! TAR should be 3
    "mfspr  7, %[tar];"
    "ori	%[res], 7, 4;"  // res = 3|4 = 7
    "b	4f;"
// Abort handler. TAR should be rolled back to 1
    "3:;"
    "mfspr  7, %[tar];"
    "ori	%[res], 7, 8;"	// res = 1|8 = 9
    "4:;"
    : [res]"=r"(result)
    : [tar]"i"(SPRN_TAR)
    : "memory", "r0", "r4", "r7");
// If result is anything else other than 7 or 9, the tar
// value must have been corrupted.
    if ((result != 7) && (result != 9))
    return 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
// A low number of iterations (eg 100) can cause a false pass
    if (argc > 1) {
    if (strcmp(argv[1], "-h") == 0) {
    printf("Syntax:\n\t%s [<num loops>]\n",
    argv[0]);
    return 1;
    } else {
    num_loops = atoi(argv[1]);
    }
    }
    printf("Starting, %d loops\n", num_loops);
    return test_harness(test_tar, "tm_tar");
    }
