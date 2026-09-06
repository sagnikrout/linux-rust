//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/copyloops/validate.c
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

pub const MAX_LEN: c_int = 8192;
pub const MAX_OFFSET: c_int = 16;
pub const MIN_REDZONE: c_int = 128;

pub const POISON: c_uint = 0xa5;

pub const VMX_COPY_THRESHOLD: c_int = 3328;

    unsigned long COPY_LOOP(void *to, const void *from, unsigned long size);
    static void do_one(char *src, char *dst, unsigned long src_off,
    unsigned long dst_off, unsigned long len, void *redzone,
    void *fill)
    {
    char *srcp, *dstp;
    unsigned long ret;
    unsigned long i;
    srcp = src + MIN_REDZONE + src_off;
    dstp = dst + MIN_REDZONE + dst_off;
    memset(src, POISON, BUFLEN);
    memset(dst, POISON, BUFLEN);
    memcpy(srcp, fill, len);
    ret = COPY_LOOP(dstp, srcp, len);
    if (ret && ret != (unsigned long)dstp) {
    printf("(%p,%p,%ld) returned %ld\n", dstp, srcp, len, ret);
    abort();
    }
    if (memcmp(dstp, srcp, len)) {
    printf("(%p,%p,%ld) miscompare\n", dstp, srcp, len);
    printf("src: ");
    for (i = 0; i < len; i++)
    printf("%02x ", srcp[i]);
    printf("\ndst: ");
    for (i = 0; i < len; i++)
    printf("%02x ", dstp[i]);
    printf("\n");
    abort();
    }
    if (memcmp(dst, redzone, dstp - dst)) {
    printf("(%p,%p,%ld) redzone before corrupted\n",
    dstp, srcp, len);
    abort();
    }
    if (memcmp(dstp+len, redzone, dst+BUFLEN-(dstp+len))) {
    printf("(%p,%p,%ld) redzone after corrupted\n",
    dstp, srcp, len);
    abort();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn test_copy_loop() -> c_int {
    int test_copy_loop(void)
    {
    char *src, *dst, *redzone, *fill;
    unsigned long len, src_off, dst_off;
    unsigned long i;
    src = memalign(BUFLEN, BUFLEN);
    dst = memalign(BUFLEN, BUFLEN);
    redzone = malloc(BUFLEN);
    fill = malloc(BUFLEN);
    if (!src || !dst || !redzone || !fill) {
    fprintf(stderr, "malloc failed\n");
    exit(1);
    }
    memset(redzone, POISON, BUFLEN);
// Fill with sequential bytes
    for (i = 0; i < BUFLEN; i++)
    fill[i] = i & 0xff;

// Force sizes above kernel VMX threshold (3328)
    for (len = VMX_COPY_THRESHOLD + 1; len < MAX_LEN; len++) {

    for (len = 1; len < MAX_LEN; len++) {

    for (src_off = 0; src_off < MAX_OFFSET; src_off++) {
    for (dst_off = 0; dst_off < MAX_OFFSET; dst_off++) {
    do_one(src, dst, src_off, dst_off, len,
    redzone, fill);
    }
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {

// Skip if Altivec not present
    SKIP_IF_MSG(!have_hwcap(PPC_FEATURE_HAS_ALTIVEC), "ALTIVEC not supported");

    return test_harness(test_copy_loop, str(COPY_LOOP));
    }
