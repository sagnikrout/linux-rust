//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/rseq/syscall_errors_test.c
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


// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2024 Michael Jeanson <mjeanson@efficios.com>

// Macro flag: #define _GNU_SOURCE

    static int sys_rseq(void *rseq_abi, uint32_t rseq_len,
    int flags, uint32_t sig)
    {
    return syscall(__NR_rseq, rseq_abi, rseq_len, flags, sig);
    }
//
// Check the value of errno on some expected failures of the rseq syscall.
//
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    struct rseq_abi *global_rseq = rseq_get_abi();
    int ret;
    int errno_copy;
    if (!rseq_available()) {
    fprintf(stderr, "rseq syscall unavailable");
    goto error;
    }
// The current thread is NOT registered.
// EINVAL
    errno = 0;
    ret = sys_rseq(global_rseq, 32, -1, RSEQ_SIG);
    errno_copy = errno;
    fprintf(stderr, "Registration with invalid flag fails with errno set to EINVAL (ret = %d, errno = %s)\n", ret, strerrorname_np(errno_copy));
    if (ret == 0 || errno_copy != EINVAL)
    goto error;
    errno = 0;
    ret = sys_rseq((char *) global_rseq + 1, 32, 0, RSEQ_SIG);
    errno_copy = errno;
    fprintf(stderr, "Registration with unaligned rseq_abi fails with errno set to EINVAL (ret = %d, errno = %s)\n", ret, strerrorname_np(errno_copy));
    if (ret == 0 || errno_copy != EINVAL)
    goto error;
    errno = 0;
    ret = sys_rseq(global_rseq, 31, 0, RSEQ_SIG);
    errno_copy = errno;
    fprintf(stderr, "Registration with invalid size fails with errno set to EINVAL (ret = %d, errno = %s)\n", ret, strerrorname_np(errno_copy));
    if (ret == 0 || errno_copy != EINVAL)
    goto error;

//
// We haven't found a reliable way to find an invalid address when
// running a 32bit userspace on a 64bit kernel, so only run this test
// on 64bit builds for the moment.
//
// Also exclude architectures that select
// CONFIG_ALTERNATE_USER_ADDRESS_SPACE where the kernel and userspace
// have their own address space and this failure can't happen.
//
// EFAULT
    errno = 0;
    ret = sys_rseq((void *) -4096UL, 32, 0, RSEQ_SIG);
    errno_copy = errno;
    fprintf(stderr, "Registration with invalid address fails with errno set to EFAULT (ret = %d, errno = %s)\n", ret, strerrorname_np(errno_copy));
    if (ret == 0 || errno_copy != EFAULT)
    goto error;

    errno = 0;
    ret = sys_rseq(global_rseq, 32, 0, RSEQ_SIG);
    errno_copy = errno;
    fprintf(stderr, "Registration succeeds for the current thread (ret = %d, errno = %s)\n", ret, strerrorname_np(errno_copy));
    if (ret != 0 && errno != 0)
    goto error;
// The current thread is registered.
// EBUSY
    errno = 0;
    ret = sys_rseq(global_rseq, 32, 0, RSEQ_SIG);
    errno_copy = errno;
    fprintf(stderr, "Double registration fails with errno set to EBUSY (ret = %d, errno = %s)\n", ret, strerrorname_np(errno_copy));
    if (ret == 0 || errno_copy != EBUSY)
    goto error;
// EPERM
    errno = 0;
    ret = sys_rseq(global_rseq, 32, RSEQ_ABI_FLAG_UNREGISTER, RSEQ_SIG + 1);
    errno_copy = errno;
    fprintf(stderr, "Unregistration with wrong RSEQ_SIG fails with errno to EPERM (ret = %d, errno = %s)\n", ret, strerrorname_np(errno_copy));
    if (ret == 0 || errno_copy != EPERM)
    goto error;
    errno = 0;
    ret = sys_rseq(global_rseq, 32, RSEQ_ABI_FLAG_UNREGISTER, RSEQ_SIG);
    errno_copy = errno;
    fprintf(stderr, "Unregistration succeeds for the current thread (ret = %d, errno = %s)\n", ret, strerrorname_np(errno_copy));
    if (ret != 0)
    goto error;
    errno = 0;
    ret = sys_rseq(global_rseq, 32, RSEQ_ABI_FLAG_UNREGISTER, RSEQ_SIG);
    errno_copy = errno;
    fprintf(stderr, "Double unregistration fails with errno set to EINVAL (ret = %d, errno = %s)\n", ret, strerrorname_np(errno_copy));
    if (ret == 0 || errno_copy != EINVAL)
    goto error;
    return 0;
    error:
    return -1;
    }
