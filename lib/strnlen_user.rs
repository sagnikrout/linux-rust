//! Automatically rewritten from C to Rust
//! Source: lib/strnlen_user.c
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

//
// Do a strnlen, return length of string *with* final '\0'.
// 'count' is the user-supplied count, while 'max' is the
// address space maximum.
//
// Return 0 for exceptions (which includes hitting the address
// space maximum), or 'count+1' if hitting the user-supplied
// maximum count.
//
// NOTE! We can sometimes overshoot the user-supplied maximum
// if it fits in a aligned 'long'. The caller needs to check
// the return value against "> max".
//
#[no_mangle]
unsafe extern "C" fn do_strnlen_user(src: *const char __user, count: c_ulong, max: c_ulong) -> __always_inline long {
    static __always_inline long do_strnlen_user(const char __user *src, unsigned long count, unsigned long max)
    {
    let mut constants: word_at_a_time = WORD_AT_A_TIME_CONSTANTS;
    unsigned long align, res = 0;
    unsigned long c;
//
// Do everything aligned. But that means that we
// need to also expand the maximum..
//
    align = (sizeof(unsigned long) - 1) & (unsigned long)src;
    src -= align;
    max += align;
    unsafe_get_user(c, (unsigned long __user *)src, efault);
    c |= aligned_byte_mask(align);
    for (;;) {
    unsigned long data;
    if (has_zero(c, &data, &constants)) {
    data = prep_zero_mask(c, data, &constants);
    data = create_zero_mask(data);
    return res + find_zero(data) + 1 - align;
    }
    res += sizeof(unsigned long);
// We already handled 'unsigned long' bytes. Did we do it all ?
    if (unlikely(max <= sizeof(unsigned long)))
    break;
    max -= sizeof(unsigned long);
    unsafe_get_user(c, (unsigned long __user *)(src+res), efault);
    }
    res -= align;
//
// Uhhuh. We hit 'max'. But was that the user-specified maximum
// too? If so, return the marker for "too long".
//
    if (res >= count)
    return count+1;
//
// Nope: we hit the address space limit, and we still had more
// characters the caller would have wanted. That's 0.
//
    efault:
    return 0;
    }
//
// strnlen_user: - Get the size of a user string INCLUDING final NUL.
// @str: The string to measure.
// @count: Maximum count (including NUL character)
//
// Context: User context only. This function may sleep if pagefaults are
// enabled.
//
// Get the size of a NUL-terminated string in user space.
//
// Returns the size of the string INCLUDING the terminating NUL.
// If the string is too long, returns a number larger than @count. User
// has to check the return value against "> count".
// On exception (or invalid count), returns 0.
//
// NOTE! You should basically never use this function. There is
// almost never any valid case for using the length of a user space
// string, since the string can be changed at any time by other
// threads. Use "strncpy_from_user()" instead to get a stable copy
// of the string.
//
#[no_mangle]
pub unsafe extern "C" fn strnlen_user(str: *const char __user, count: c_long) -> c_long {
    long strnlen_user(const char __user *str, long count)
    {
    unsigned long max_addr, src_addr;
    if (unlikely(count <= 0))
    return 0;
    if (can_do_masked_user_access()) {
    long retval;
    str = masked_user_read_access_begin(str);
    retval = do_strnlen_user(str, count, count);
    user_read_access_end();
    return retval;
    }
    max_addr = TASK_SIZE_MAX;
    src_addr = (unsigned long)untagged_addr(str);
    if (likely(src_addr < max_addr)) {
    let mut max: c_ulong = max_addr - src_addr;
    long retval;
//
// Truncate 'max' to the user-specified limit, so that
// we only have one limit we need to check in the loop
//
    if (max > count)
    max = count;
    if (user_read_access_begin(str, max)) {
    retval = do_strnlen_user(str, count, max);
    user_read_access_end();
    return retval;
    }
    }
    return 0;
    }
    EXPORT_SYMBOL(strnlen_user);
