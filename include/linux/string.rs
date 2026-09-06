//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/string.h
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
// memdup_array_user - duplicate array from user space
// @src: source address in user space
// @n: number of array members to copy
// @size: size of one array member
//
// Return: an ERR_PTR() on failure. Result is physically
// contiguous, to be freed by kfree().
//
extern "C" {
    pub fn ERR_PTR(_arg: -EOVERFLOW) -> return;
}
extern "C" {
    pub fn memdup_user(_arg: src, _arg: nbytes) -> return;
}
//
// vmemdup_array_user - duplicate array from user space
// @src: source address in user space
// @n: number of array members to copy
// @size: size of one array member
//
// Return: an ERR_PTR() on failure. Result may be not
// physically contiguous. Use kvfree() to free.
//
extern "C" {
    pub fn ERR_PTR(_arg: -EOVERFLOW) -> return;
}
extern "C" {
    pub fn vmemdup_user(_arg: src, _arg: nbytes) -> return;
}
//
// Include machine specific inline routines
//

extern "C" {
    pub fn strcpy(: *mut c_char, : *const c_char) -> *mut c_char;
}

extern "C" {
    pub fn sized_strscpy(: *mut c_char, : *const c_char, _arg: usize) -> isize;
}
//
// The 2 argument style can only be used when dst is an array with a
// known size.
//

//
// strscpy - Copy a C-string into a sized buffer
// @dst: Where to copy the string to
// @src: Where to copy the string from
// @...: Size of destination buffer (optional)
//
// Copy the source string @src, or as much of it as fits, into the
// destination @dst buffer. The behavior is undefined if the string
// buffers overlap. The destination @dst buffer is always NUL terminated,
// unless it's zero-sized.
//
// The size argument @... is only required when @dst is not an array, or
// when the copy needs to be smaller than sizeof(@dst).
//
// Preferred to strncpy() since it always returns a valid string, and
// doesn't unnecessarily force the tail of the destination buffer to be
// zero padded. If padding is desired please use strscpy_pad().
//
// Returns the number of characters copied in @dst (not including the
// trailing %NUL) or -E2BIG if @size is 0 or the copy from @src was
// truncated.
//

//
// strscpy_pad() - Copy a C-string into a sized buffer
// @dst: Where to copy the string to
// @src: Where to copy the string from
// @...: Size of destination buffer
//
// Copy the string, or as much of it as fits, into the dest buffer. The
// behavior is undefined if the string buffers overlap. The destination
// buffer is always %NUL terminated, unless it's zero-sized.
//
// If the source string is shorter than the destination buffer, the
// remaining bytes in the buffer will be filled with %NUL bytes.
//
// For full explanation of why you may want to consider using the
// 'strscpy' functions please see the function docstring for strscpy().
//
// Returns:
// * The number of characters copied (not including the trailing %NULs)
// * -E2BIG if count is 0 or @src was truncated.
//

extern "C" {
    pub fn strcat(: *mut c_char, : *const c_char) -> *mut c_char;
}

extern "C" {
    pub fn strncat(: *mut c_char, : *const c_char, _arg: __kernel_size_t) -> *mut c_char;
}

extern "C" {
    pub fn strlcat(: *mut c_char, : *const c_char, _arg: __kernel_size_t) -> usize;
}

extern "C" {
    pub fn strcmp(: *const c_char, : *const c_char) -> c_int;
}

extern "C" {
    pub fn strncmp(: *const c_char, : *const c_char, _arg: __kernel_size_t) -> c_int;
}

extern "C" {
    pub fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
}

extern "C" {
    pub fn strncasecmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
}

extern "C" {
    pub fn strchr(: *const c_char, _arg: c_int) -> *mut c_char;
}

extern "C" {
    pub fn strchrnul(: *const c_char, _arg: c_int) -> *mut c_char;
}

extern "C" {
    pub fn strnchrnul(: *const c_char, _arg: usize, _arg: c_int) -> *mut c_char;
}
extern "C" {
    pub fn strnchr(: *const c_char, _arg: usize, _arg: c_int) -> *mut c_char;
}

extern "C" {
    pub fn strrchr(: *const c_char, _arg: c_int) -> *mut c_char;
}

extern "C" {
    pub fn skip_spaces(: *const c_char) -> *mut char  __must_check;
}
extern "C" {
    pub fn strim(_arg: str) -> return;
}
extern "C" {
    pub fn strstr(: *const c_char, : *const c_char) -> *mut c_char;
}

extern "C" {
    pub fn strnstr(: *const c_char, : *const c_char, _arg: usize) -> *mut c_char;
}

extern "C" {
    pub fn strlen(: *const c_char) -> __kernel_size_t;
}

extern "C" {
    pub fn strnlen(: *const c_char, _arg: __kernel_size_t) -> __kernel_size_t;
}

extern "C" {
    pub fn strpbrk(: *const c_char, : *const c_char) -> *mut c_char;
}

extern "C" {
    pub fn strsep(: *mut c_char, : *const c_char) -> *mut c_char;
}

extern "C" {
    pub fn strspn(: *const c_char, : *const c_char) -> __kernel_size_t;
}

extern "C" {
    pub fn strcspn(: *const c_char, : *const c_char) -> __kernel_size_t;
}

extern "C" {
    pub fn memset(: *mut c_void, _arg: c_int, _arg: __kernel_size_t) -> *mut c_void;
}

extern "C" {
    pub fn memset32()p: *mut (uint32_t, _arg: v, _arg: n) -> return;
}
extern "C" {
    pub fn memset64()p: *mut (uint64_t, _arg: v, _arg: n) -> return;
}
extern "C" {
    pub fn memset32()p: *mut (uint32_t, _arg: (uintptr_t)v, _arg: n) -> return;
}
extern "C" {
    pub fn memset64()p: *mut (uint64_t, _arg: (uintptr_t)v, _arg: n) -> return;
}

extern "C" {
    pub fn memcpy(: *mut c_void, : *const c_void, _arg: __kernel_size_t) -> *mut c_void;
}

extern "C" {
    pub fn memmove(: *mut c_void, : *const c_void, _arg: __kernel_size_t) -> *mut c_void;
}

extern "C" {
    pub fn memscan(: *mut c_void, _arg: c_int, _arg: __kernel_size_t) -> *mut c_void;
}

extern "C" {
    pub fn memcmp(: *const c_void, : *const c_void, _arg: __kernel_size_t) -> c_int;
}

extern "C" {
    pub fn bcmp(: *const c_void, : *const c_void, _arg: __kernel_size_t) -> c_int;
}

extern "C" {
    pub fn memchr(: *const c_void, _arg: c_int, _arg: __kernel_size_t) -> *mut c_void;
}

//
// mem_is_zero - Check if an area of memory is all 0's.
// @s: The memory area
// @n: The size of the area
//
// Return: True if the area of memory is all 0's.
//
extern "C" {
    pub fn kfree_const(x: *const c_void);
}

// lib/argv_split.c
extern "C" {
    pub fn argv_free(argv: *mut c_char);
}
// lib/cmdline.c
extern "C" {
    pub fn get_option(str: *mut c_char, pint: *mut c_int) -> c_int;
}
extern "C" {
    pub fn memparse(ptr: *const c_char, retptr: *mut c_char) -> c_ulonglong;
}
extern "C" {
    pub fn parse_option_str(str: *const c_char, option: *const c_char) -> bool;
}
extern "C" {
    pub fn sysfs_streq(s1: *const c_char, s2: *const c_char) -> bool;
}
extern "C" {
    pub fn match_string(array: *const *const c_char, n: usize, string: *const c_char) -> c_int;
}
extern "C" {
    pub fn __sysfs_match_string(array: *const *const c_char, n: usize, s: *const c_char) -> c_int;
}
//
// sysfs_match_string - matches given string in an array
// @_a: array of strings
// @_s: string to match with
//
// Helper for __sysfs_match_string(). Calculates the size of @a automatically.
//

extern "C" {
    pub fn vbin_printf(bin_buf: *mut u32, size: usize, fmt: *const c_char, args: va_list) -> c_int;
}
extern "C" {
    pub fn bstr_printf(buf: *mut c_char, size: usize, fmt: *const c_char, bin_buf: *const u32) -> c_int;
}

extern "C" {
    pub fn ptr_to_hashval(ptr: *const c_void, hashval_out: *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn memweight(ptr: *const c_void, bytes: usize) -> usize;
}
//
// memzero_explicit - Fill a region of memory (e.g. sensitive
// keying data) with 0s.
// @s: Pointer to the start of the area.
// @count: The size of the area.
//
// Note: usually using memset() is just fine (!), but in cases
// where clearing out _local_ data at the end of a scope is
// necessary, memzero_explicit() should be used instead in
// order to prevent the compiler from optimising away zeroing.
//
// memzero_explicit() doesn't need an arch-specific version as
// it just invokes the one of memset() implicitly.
//
// kbasename - return the last part of a pathname.
//
// @path: path to extract the filename from.
//
// Returns:
// Pointer to the filename portion inside @path. If no '/' exists,
// returns @path unchanged.
//

//
// strtomem_pad - Copy NUL-terminated string to non-NUL-terminated buffer
//
// @dest: Pointer of destination character array (marked as __nonstring)
// @src: Pointer to NUL-terminated string
// @pad: Padding character to fill any remaining bytes of @dest after copy
//
// This is a replacement for strncpy() uses where the destination is not
// a NUL-terminated string, but with bounds checking on the source size, and
// an explicit padding character. If padding is not required, use strtomem().
//
// Note that the size of @dest is not an argument, as the length of @dest
// must be discoverable by the compiler.
//

//
// strtomem - Copy NUL-terminated string to non-NUL-terminated buffer
//
// @dest: Pointer of destination character array (marked as __nonstring)
// @src: Pointer to NUL-terminated string
//
// This is a replacement for strncpy() uses where the destination is not
// a NUL-terminated string, but with bounds checking on the source size, and
// without trailing padding. If padding is required, use strtomem_pad().
//
// Note that the size of @dest is not an argument, as the length of @dest
// must be discoverable by the compiler.
//

//
// memtostr - Copy a possibly non-NUL-term string to a NUL-term string
// @dest: Pointer to destination NUL-terminates string
// @src: Pointer to character array (likely marked as __nonstring)
//
// This is a replacement for strncpy() uses where the source is not
// a NUL-terminated string.
//
// Note that sizes of @dest and @src must be known at compile-time.
//

//
// memtostr_pad - Copy a possibly non-NUL-term string to a NUL-term string
// with NUL padding in the destination
// @dest: Pointer to destination NUL-terminates string
// @src: Pointer to character array (likely marked as __nonstring)
//
// This is a replacement for strncpy() uses where the source is not
// a NUL-terminated string.
//
// Note that sizes of @dest and @src must be known at compile-time.
//

//
// memset_after - Set a value after a struct member to the end of a struct
//
// @obj: Address of target struct instance
// @v: Byte value to repeatedly write
// @member: after which struct member to start writing bytes
//
// This is good for clearing padding following the given member.
//

//
// memset_startat - Set a value starting at a member to the end of a struct
//
// @obj: Address of target struct instance
// @v: Byte value to repeatedly write
// @member: struct member to start writing at
//
// Note that if there is padding between the prior member and the target
// member, memset_after() should be used to clear the prior padding.
//

//
// str_has_prefix - Test if a string has a given prefix
// @str: The string to test
// @prefix: The string to see if @str starts with
//
// A common way to test a prefix of a string is to do:
// strncmp(str, prefix, sizeof(prefix) - 1)
//
// But this can lead to bugs due to typos, or if prefix is a pointer
// and not a constant. Instead use str_has_prefix().
//
// Returns:
// * strlen(@prefix) if @str starts with @prefix
// * 0 if @str does not start with @prefix
//
// strstarts - does @str start with @prefix?
// @str: string to examine
// @prefix: prefix to look for.
//
// Returns:
// True if @str begins with @prefix. False in all other cases.
//
// strends - Check if a string ends with another string.
// @str: NULL-terminated string to check against @suffix
// @suffix: NULL-terminated string defining the suffix to look for in @str
//
// Returns:
// True if @str ends with @suffix. False in all other cases.
//
