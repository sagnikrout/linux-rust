//! Automatically rewritten from C to Rust
//! Source: lib/strncpy_from_user.c
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

    (((long) dst | (long) src) & (sizeof(long) - 1))

//
// Do a strncpy, return length of string without final '\0'.
// 'count' is the user-supplied count (return 'count' if we
// hit it), 'max' is the address space maximum (and we return
// -EFAULT if we hit it).
//
    static __always_inline long do_strncpy_from_user(char *dst, const char __user *src,
    unsigned long count, unsigned long max)
    {
    let mut constants: word_at_a_time = WORD_AT_A_TIME_CONSTANTS;
    let mut res: c_ulong = 0;
    if (IS_UNALIGNED(src, dst))
    goto byte_at_a_time;
    while (max >= sizeof(unsigned long)) {
    unsigned long c, data, mask;
// Fall back to byte-at-a-time if we get a page fault
    unsafe_get_user(c, (unsigned long __user *)(src+res), byte_at_a_time);
//
// Note that we mask out the bytes following the NUL. This is
// important to do because string oblivious code may read past
// the NUL. For those routines, we don't want to give them
// potentially random bytes after the NUL in `src`.
//
// One example of such code is BPF map keys. BPF treats map keys
// as an opaque set of bytes. Without the post-NUL mask, any BPF
// maps keyed by strings returned from strncpy_from_user() may
// have multiple entries for semantically identical strings.
//
    if (has_zero(c, &data, &constants)) {
    data = prep_zero_mask(c, data, &constants);
    data = create_zero_mask(data);
    mask = zero_bytemask(data);
// (unsigned long *)(dst+res) = c & mask;
    return res + find_zero(data);
    }
// (unsigned long *)(dst+res) = c;
    res += sizeof(unsigned long);
    max -= sizeof(unsigned long);
    }
    byte_at_a_time:
    while (max) {
    char c;
    unsafe_get_user(c,src+res, efault);
    dst[res] = c;
    if (!c)
    return res;
    res++;
    max--;
    }
//
// Uhhuh. We hit 'max'. But was that the user-specified maximum
// too? If so, that's ok - we got as much as the user asked for.
//
    if (res >= count)
    return res;
//
// Nope: we hit the address space limit, and we still had more
// characters the caller would have wanted. That's an EFAULT.
//
    efault:
    return -EFAULT;
    }
//
// strncpy_from_user: - Copy a NUL terminated string from userspace.
// @dst:   Destination address, in kernel space.  This buffer must be at
// least @count bytes long.
// @src:   Source address, in user space.
// @count: Maximum number of bytes to copy, including the trailing NUL.
//
// Copies a NUL-terminated string from userspace to kernel space.
//
// On success, returns the length of the string (not including the trailing
// NUL).
//
// If access to userspace fails, returns -EFAULT (some data may have been
// copied).
//
// If @count is smaller than the length of the string, copies @count bytes
// and returns @count.
//
#[no_mangle]
pub unsafe extern "C" fn strncpy_from_user(dst: *mut c_char, src: *const char __user, count: c_long) -> c_long {
    long strncpy_from_user(char *dst, const char __user *src, long count)
    {
    unsigned long max_addr, src_addr;
    might_fault();
    if (should_fail_usercopy())
    return -EFAULT;
    if (unlikely(count <= 0))
    return 0;
    kasan_check_write(dst, count);
    check_object_size(dst, count, false);
    if (can_do_masked_user_access()) {
    long retval;
    src = masked_user_read_access_begin(src);
    retval = do_strncpy_from_user(dst, src, count, count);
    user_read_access_end();
    return retval;
    }
    max_addr = TASK_SIZE_MAX;
    src_addr = (unsigned long)untagged_addr(src);
    if (likely(src_addr < max_addr)) {
    let mut max: c_ulong = max_addr - src_addr;
    long retval;
//
// Truncate 'max' to the user-specified limit, so that
// we only have one limit we need to check in the loop
//
    if (max > count)
    max = count;
    if (user_read_access_begin(src, max)) {
    retval = do_strncpy_from_user(dst, src, count, max);
    user_read_access_end();
    return retval;
    }
    }
    return -EFAULT;
    }
    EXPORT_SYMBOL(strncpy_from_user);
