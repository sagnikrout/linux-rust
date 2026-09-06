//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/tm/tm-vmxcopy.c
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
//
// Original: Michael Neuling 4/12/2013
// Edited: Rashmica Gupta 4/12/2015
//
// See if the altivec state is leaked out of an aborted transaction due to
// kernel vmx copy loops.
//
// When the transaction aborts, VSR values should rollback to the values
// they held before the transaction commenced. Using VSRs while transaction
// is suspended should not affect the checkpointed values.
//
// (1) write A to a VSR
// (2) start transaction
// (3) suspend transaction
// (4) change the VSR to B
// (5) trigger kernel vmx copy loop
// (6) abort transaction
// (7) check that the VSR value is A
//

#[no_mangle]
pub unsafe extern "C" fn test_vmxcopy() -> c_int {
    int test_vmxcopy()
    {
    let mut vecin: long double = 1.3;
    long double vecout;
    let mut pgsize: c_ulong = getpagesize();
    int i;
    int fd;
    let mut size: c_int = pgsize*16;
    char tmpfile[] = "/tmp/page_faultXXXXXX";
    char buf[pgsize];
    char *a;
    let mut aborted: u64 = 0;
    SKIP_IF(!have_htm());
    SKIP_IF(htm_is_synthetic());
    SKIP_IF(!is_ppc64le());
    fd = mkstemp(tmpfile);
    assert(fd >= 0);
    memset(buf, 0, pgsize);
    for (i = 0; i < size; i += pgsize)
    assert(write(fd, buf, pgsize) == pgsize);
    unlink(tmpfile);
    a = mmap(core::ptr::null_mut(), size, PROT_READ|PROT_WRITE, MAP_PRIVATE, fd, 0);
    assert(a != MAP_FAILED);
    asm __volatile__(
    "lxvd2x 40,0,%[vecinptr];"	/* set 40 to initial value*/
    "tbegin.;"
    "beq	3f;"
    "tsuspend.;"
    "xxlxor 40,40,40;"		/* set 40 to 0 */
    "std	5, 0(%[map]);"		/* cause kernel vmx copy page */
    "tabort. 0;"
    "tresume.;"
    "tend.;"
    "li	%[res], 0;"
    "b	5f;"
// Abort handler
    "3:;"
    "li	%[res], 1;"
    "5:;"
    "stxvd2x 40,0,%[vecoutptr];"
    : [res]"=&r"(aborted)
    : [vecinptr]"r"(&vecin),
    [vecoutptr]"r"(&vecout),
    [map]"r"(a)
    : "memory", "r0", "r3", "r4", "r5", "r6", "r7");
    if (aborted && (vecin != vecout)){
    printf("FAILED: vector state leaked on abort %f != %f\n",
    (double)vecin, (double)vecout);
    return 1;
    }
    munmap(a, size);
    close(fd);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(test_vmxcopy, "tm_vmxcopy");
    }
