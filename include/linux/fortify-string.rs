//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fortify-string.h
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

// Overridden by KUnit tests.

pub const FORTIFY_READ: c_int = 0;
pub const FORTIFY_WRITE: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fortify_func {
    EACH_FORTIFY_FUNC(MAKE_FORTIFY_FUNC)
}

extern "C" {
    pub fn __fortify_report(reason: u8, avail: usize, size: usize);
}
extern "C" {
    pub fn __read_overflow(parameter)": void) __compiletime_error("detected read beyond size of object (1st);
}
extern "C" {
    pub fn __read_overflow2(parameter)": void) __compiletime_error("detected read beyond size of object (2nd);
}
extern "C" {
    pub fn __read_overflow2_field(avail: usize, struct_group()?": size_t wanted) __compiletime_warning("detected read beyond size of field (2nd parameter); maybe use);
}
extern "C" {
    pub fn __write_overflow(parameter)": void) __compiletime_error("detected write beyond size of object (1st);
}
extern "C" {
    pub fn __write_overflow_field(avail: usize, struct_group()?": size_t wanted) __compiletime_warning("detected write beyond size of field (1st parameter); maybe use);
}

extern "C" {
    pub fn __underlying_memcmp(p: *const c_void, q: *const c_void, __RENAME(memcmp: __kernel_size_t size)) -> c_int;
}
extern "C" {
    pub fn __underlying_strlen(__RENAME(strlen: *const *const char p)) -> __kernel_size_t;
}

//
// For KMSAN builds all memcpy/memset/memmove calls should be replaced by the
// corresponding __msan_XXX functions.
//

//
// unsafe_memcpy - memcpy implementation with no FORTIFY bounds checking
//
// @dst: Destination memory address to write to
// @src: Source memory address to read from
// @bytes: How many bytes to write to @dst from @src
// @justification: Free-form text or comment describing why the use is needed
//
// This should be used for corner cases where the compiler cannot do the
// right thing, or during transitions between APIs, etc. It should be used
// very rarely, and includes a place for justification detailing where bounds
// checking has happened, and why existing solutions cannot be employed.
//

//
// Clang's use of __builtin_*object_size() within inlines needs hinting via
// __pass_*object_size(). The preference is to only ever use type 1 (member
// size, rather than struct size), but there remain some stragglers using
// type 0 that will be converted in the future.
//

extern "C" {
    pub fn __real_strnlen(: *const c_char, __RENAME(strnlen: __kernel_size_t)) -> __kernel_size_t;
}
//
// strnlen - Return bounded count of characters in a NUL-terminated string
//
// @p: pointer to NUL-terminated string to count.
// @maxlen: maximum number of characters to count.
//
// Returns number of characters in @p (NOT including the final NUL), or
// @maxlen, if no NUL has been found up to there.
//
// We can take compile-time actions when maxlen is const.
// If p is const, we can use its compile-time-known len.
// Do not check characters beyond the end of p.
//
// Defined after fortified strnlen to reuse it. However, it must still be
// possible for strlen() to be used on compile-time strings for use in
// static initializers (i.e. as a constant expression).
//
// strlen - Return count of characters in a NUL-terminated string
//
// @p: pointer to NUL-terminated string to count.
//
// Do not use this function unless the string length is known at
// compile-time. When @p is unterminated, this function may crash
// or return unexpected counts that could lead to memory content
// exposures. Prefer strnlen().
//
// Returns number of characters in @p (NOT including the final NUL).
//

// Give up if we don't know how large p is.
extern "C" {
    pub fn __underlying_strlen(_arg: p) -> return;
}
// Defined after fortified strnlen() to reuse it.
extern "C" {
    pub fn __real_strscpy(: *mut c_char, : *const c_char, __RENAME(sized_strscpy: size_t)) -> isize;
}
// Use string size rather than possible enclosing struct size.
// If we cannot get size of p and q default to call strscpy.
extern "C" {
    pub fn __real_strscpy(_arg: p, _arg: q, _arg: size) -> return;
}
//
// If size can be known at compile time and is greater than
// p_size, generate a compile time write overflow error.
//
// Short-circuit for compile-time known-safe lengths.
//
// This call protects from read overflow, because len will default to q
// length if it smaller than size.
//
// If len equals size, we will copy only size bytes which leads to
// -E2BIG being returned.
// Otherwise we will copy len + 1 because of the final '\O'.
//
// Generate a runtime write overflow error if len is greater than
// p_size.
//
// We can now safely call vanilla strscpy because we are protected from:
// 1. Read overflow thanks to call to strnlen().
// 2. Write overflow thanks to above ifs.
//
extern "C" {
    pub fn __real_strscpy(_arg: p, _arg: q, _arg: len) -> return;
}
// Defined after fortified strlen() to reuse it.
extern "C" {
    pub fn __real_strlcat(p: *mut c_char, q: *const c_char, __RENAME(strlcat: size_t avail)) -> usize;
}
//
// strlcat - Append a string to an existing string
//
// @p: pointer to %NUL-terminated string to append to
// @q: pointer to %NUL-terminated string to append from
// @avail: Maximum bytes available in @p
//
// Appends %NUL-terminated string @q after the %NUL-terminated
// string at @p, but will not write beyond @avail bytes total,
// potentially truncating the copy from @q. @p will stay
// %NUL-terminated only if a %NUL already existed within
// the @avail bytes of @p. If so, the resulting number of
// bytes copied from @q will be at most "@avail - strlen(@p) - 1".
//
// Do not use this function. While FORTIFY_SOURCE tries to avoid
// read and write overflows, this is only possible when the sizes
// of @p and @q are known to the compiler. Prefer building the
// string with formatting, via scnprintf(), seq_buf, or similar.
//
// Returns total bytes that _would_ have been contained by @p
// regardless of truncation, similar to snprintf(). If return
// value is >= @avail, the string has been truncated.
//
// Give up immediately if both buffer sizes are unknown.
extern "C" {
    pub fn __real_strlcat(_arg: p, _arg: q, _arg: avail) -> return;
}
// Cannot append any more: report truncation.
// Give up if string is already overflowed.
// Give up if copy will overflow.
// Defined after fortified strlcat() to reuse it.
//
// strcat - Append a string to an existing string
//
// @p: pointer to NUL-terminated string to append to
// @q: pointer to NUL-terminated source string to append from
//
// Do not use this function. While FORTIFY_SOURCE tries to avoid
// read and write overflows, this is only possible when the
// destination buffer size is known to the compiler. Prefer
// building the string with formatting, via scnprintf() or similar.
// At the very least, use strncat().
//
// Returns @p.
//
// strncat - Append a string to an existing string
//
// @p: pointer to NUL-terminated string to append to
// @q: pointer to source string to append from
// @count: Maximum bytes to read from @q
//
// Appends at most @count bytes from @q (stopping at the first
// NUL byte) after the NUL-terminated string at @p. @p will be
// NUL-terminated.
//
// Do not use this function. While FORTIFY_SOURCE tries to avoid
// read and write overflows, this is only possible when the sizes
// of @p and @q are known to the compiler. Prefer building the
// string with formatting, via scnprintf() or similar.
//
// Returns @p.
//
// Defined after fortified strlen() and strnlen() to reuse them.
extern "C" {
    pub fn __underlying_strncat(_arg: p, _arg: q, _arg: count) -> return;
}
//
// Length argument is a constant expression, so we
// can perform compile-time bounds checking where
// buffer sizes are also known at compile time.
//
// Error when size is larger than enclosing struct.
// Warn when write size is larger than dest field.
//
// At this point, length argument may not be a constant expression,
// so run-time bounds checking can be done where buffer sizes are
// known. (This is not an "else" because the above checks may only
// be compile-time warnings, and we want to still warn for run-time
// overflows.)
//
// Always stop accesses beyond the struct that contains the
// field, when the buffer's remaining size is known.
// (The SIZE_MAX test is to optimize away checks where the buffer
// lengths are unknown.)
//

//
// __struct_size() vs __member_size() must be captured here to avoid
// evaluating argument side-effects further into the macro layers.
//

//
// To make sure the compiler can enforce protection against buffer overflows,
// memcpy(), memmove(), and memset() must not be used beyond individual
// struct members. If you need to copy across multiple members, please use
// struct_group() to create a named mirror of an anonymous struct union.
// (e.g. see struct sk_buff.) Read overflow checking is currently only
// done when a write overflow is also present, or when building with W=1.
//
// Mitigation coverage matrix
// Bounds checking at:
// +-------+-------+-------+-------+
// | Compile time  |   Run time    |
// memcpy() argument sizes:		| write | read  | write | read  |
// dest     source   length      +-------+-------+-------+-------+
// memcpy(known,   known,   constant)	|   y   |   y   |  n/a  |  n/a  |
// memcpy(known,   unknown, constant)	|   y   |   n   |  n/a  |   V   |
// memcpy(known,   known,   dynamic)	|   n   |   n   |   B   |   B   |
// memcpy(known,   unknown, dynamic)	|   n   |   n   |   B   |   V   |
// memcpy(unknown, known,   constant)	|   n   |   y   |   V   |  n/a  |
// memcpy(unknown, unknown, constant)	|   n   |   n   |   V   |   V   |
// memcpy(unknown, known,   dynamic)	|   n   |   n   |   V   |   B   |
// memcpy(unknown, unknown, dynamic)	|   n   |   n   |   V   |   V   |
// +-------+-------+-------+-------+
//
// y = perform deterministic compile-time bounds checking
// n = cannot perform deterministic compile-time bounds checking
// n/a = no run-time bounds checking needed since compile-time deterministic
// B = can perform run-time bounds checking (currently unimplemented)
// V = vulnerable to run-time overflow (will need refactoring to solve)
//
// Length argument is a constant expression, so we
// can perform compile-time bounds checking where
// buffer sizes are also known at compile time.
//
// Error when size is larger than enclosing struct.
// Warn when write size argument larger than dest field.
//
// Warn for source field over-read when building with W=1
// or when an over-write happened, so both can be fixed at
// the same time.
//
// At this point, length argument may not be a constant expression,
// so run-time bounds checking can be done where buffer sizes are
// known. (This is not an "else" because the above checks may only
// be compile-time warnings, and we want to still warn for run-time
// overflows.)
//
// Always stop accesses beyond the struct that contains the
// field, when the buffer's remaining size is known.
// (The SIZE_MAX test is to optimize away checks where the buffer
// lengths are unknown.)
//
// Warn when writing beyond destination field size.
//
// Note the implementation of __builtin_*object_size() behaves
// like sizeof() when not directly referencing a flexible
// array member, which means there will be many bounds checks
// that will appear at run-time, without a way for them to be
// detected at compile-time (as can be done when the destination
// is specifically the flexible array member).
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=101832
//
// To work around what seems to be an optimizer bug, the macro arguments
// need to have const copies or the values end up changed by the time they
// reach fortify_warn_once(). See commit 6f7630b1b5bc ("fortify: Capture
// __bos() results in const temp vars") for more details.
//

// Keep a mutable version of the size for the final copy. */	\

// Hide only the run-time size from value range tracking to */	\
// silence compile-time false positive bounds warnings. */	\
//
// Notes about compile-time buffer size detection:
//
// With these types...
//
// struct middle {
// u16 a;
// u8 middle_buf[16];
// int b;
// };
// struct end {
// u16 a;
// u8 end_buf[16];
// };
// struct flex {
// int a;
// u8 flex_buf[];
// };
//
// void func(TYPE *ptr) { ... }
//
// Cases where destination size cannot be currently detected:
// - the size of ptr's object (seemingly by design, gcc & clang fail):
// __builtin_object_size(ptr, 1) == SIZE_MAX
// - the size of flexible arrays in ptr's obj (by design, dynamic size):
// __builtin_object_size(ptr->flex_buf, 1) == SIZE_MAX
// - the size of ANY array at the end of ptr's obj (gcc and clang bug):
// __builtin_object_size(ptr->end_buf, 1) == SIZE_MAX
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=101836
//
// Cases where destination size is currently detected:
// - the size of non-array members within ptr's object:
// __builtin_object_size(ptr->a, 1) == 2
// - the size of non-flexible-array in the middle of ptr's obj:
// __builtin_object_size(ptr->middle_buf, 1) == 16
//
// __struct_size() vs __member_size() must be captured here to avoid
// evaluating argument side-effects further into the macro layers.
//

extern "C" {
    pub fn __real_memscan(_arg: p, _arg: c, _arg: size) -> return;
}
extern "C" {
    pub fn __underlying_memcmp(_arg: p, _arg: q, _arg: size) -> return;
}
extern "C" {
    pub fn __underlying_memchr(_arg: p, _arg: c, _arg: size) -> return;
}
extern "C" {
    pub fn __real_memchr_inv(_arg: p, _arg: c, _arg: size) -> return;
}
extern "C" {
    pub fn __real_kmemdup(_arg: p, _arg: size, _arg: gfp) -> return;
}

//
// strcpy - Copy a string into another string buffer
//
// @p: pointer to destination of copy
// @q: pointer to NUL-terminated source string to copy
//
// Do not use this function. While FORTIFY_SOURCE tries to avoid
// overflows, this is only possible when the sizes of @q and @p are
// known to the compiler. Prefer strscpy(), though note its different
// return values for detecting truncation.
//
// Returns @p.
//
// Defined after fortified strlen to reuse it.
// If neither buffer size is known, immediately give up.
extern "C" {
    pub fn __underlying_strcpy(_arg: p, _arg: q) -> return;
}
// Compile-time check for const size overflow.
// Run-time check for dynamic size overflow.
// Don't use these outside the FORITFY_SOURCE implementation

