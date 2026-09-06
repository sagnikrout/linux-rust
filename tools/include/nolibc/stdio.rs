//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/nolibc/stdio.h
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


// SPDX-License-Identifier: LGPL-2.1 OR MIT
//
// minimal stdio function definitions for NOLIBC
// Copyright (C) 2017-2021 Willy Tarreau <w@1wt.eu>
//
// make sure to include all global symbols

// Buffering mode used by setvbuf.

// just define FILE as a non-empty type. The value of the pointer gives
// the FD: FILE=~fd for fd>=0 or NULL for fd<0. This way positive FILE
// are immediately identified as abnormal entries (i.e. possible copies
// of valid pointers to something else).
//
// provides a FILE* equivalent of fd. The mode is ignored.
extern "C" {
    pub fn fdopen(_arg: fd, _arg: mode) -> return;
}
// provides the fd of stream.
// flush a stream.
// NULL is valid here.
// Don't do anything, nolibc does not support buffering.
// flush a stream.
// getc(), fgetc(), getchar()

extern "C" {
    pub fn fgetc(_arg: stdin) -> return;
}
// putc(), fputc(), putchar()

extern "C" {
    pub fn fputc(_arg: c, _arg: stdout) -> return;
}
// fwrite(), fread(), puts(), fputs(). Note that puts() emits '\n' but not fputs().
// internal fwrite()-like function which only takes a size and returns 0 on
// success or EOF on error. It automatically retries on short writes.
//
// internal fread()-like function which only takes a size and returns 0 on
// success or EOF on error. It automatically retries on short reads.
//
extern "C" {
    pub fn _fwrite(_arg: s, _arg: strlen(s), _arg: stream) -> return;
}
extern "C" {
    pub fn putchar(_arg: '\n') -> return;
}
// fgets()
// fseek
// lseek() and fseek() differ in that lseek returns the new
// position or -1, fseek() returns either 0 or -1.
//
// printf(). Supports most of the normal integer and string formats.
// - %[#0-+ ][width|*[.precision|*}][{l,t,z,ll,L,j,q}]{c,d,i,u,o,x,X,p,s,m,%}
// - %% generates a single %
// - %m outputs strerror(errno).
// - %X outputs a..f the same as %x.
// - No support for floating point or wide characters.
// - Invalid formats are copied to the output buffer.
//
// Called by vfprintf() and snprintf() to do the actual formatting.
// The callers provide a callback function to save the formatted data.
// The callback function is called multiple times:
// - for each group of literal characters in the format string.
// - for field padding.
// - for each conversion specifier.
// - with (NULL, 0) at the end of the __nolibc_printf.
// If the callback returns non-zero __nolibc_printf() immediately returns -1.
//
extern "C" {
    pub fn int(state: *mut *mut __nolibc_printf_cb)(void, buf: *const c_char, size: usize) -> typedef;
}
// This code uses 'flag' variables that are indexed by the low 6 bits
// of characters to optimise checks for multiple characters.
//
// _NOLIBC_PF_FLAGS_CONTAIN(flags, 'a', 'b'. ...)
// returns non-zero if the bit for any of the specified characters is set.
//
// _NOLIBC_PF_CHAR_IS_ONE_OF(ch, 'a', 'b'. ...)
// returns the flag bit for ch if it is one of the specified characters.
// All the characters must be in the same 32 character block (non-alphabetic,
// upper case, or lower case) of the ASCII character set.
//

// Output characters from the format string.
// we're in a format sequence
// Conversion flag characters
// Width and precision
// Default precision for strings
// A negative width (e.g. from "%*s") requests left justify.
// Length modifier.
// They miss the conversion flags characters " #+-0" so can go into flags.
// Change both L and ll to j (all always 64bit).
//
// Conversion specifiers.
// Numeric and pointer conversion specifiers.
//
// Use an explicit bound check (rather than _NOLIBC_PF_CHAR_IS_ONE_OF())
// so that 'X' can be allowed through.
// 'X' gets treated and 'x' because _NOLIBC_PF_FLAG() returns the same
// value for both.
//
// We need to check for "%p" or "%#x" later, merging here gives better code.
// But '#' collides with 'c' so shift right.
//
// 'long' is needed for pointer/string conversions and ltz lengths.
// A single test can be used provided 'p' (the same bit as '0')
// is masked from flags.
//
// "%c" - single character.
// "%s" - character string.
// Match glibc, nothing output if precision too small
// The 'sign_prefix' can be zero, one or two ("0x") characters.
// Prepended least significant byte first stopping on a zero byte.
//
// "%d" and "%i" - signed decimal numbers.
// "#o" requires that the output always starts with a '0'.
// This needs another check after any zero padding to avoid
// adding an extra leading '0'.
//
// The value is converted offset into the buffer so that
// 31 zero pad characters and the sign/prefix can be added in front.
// The longest digit string is 22 + 1 for octal conversions.
//
// There are special rules for zero.
// "%p" match glibc, precision is ignored
// Explicit %nn.0d, no digits output (except for %#.0o)
// All other formats (including "%#x") just output "0".
// Convert the number to ascii in the required base.
// "%p" and "%#x" need "0x" prepending.
// Add zero padding
// No explicit precision (or negative from "%.*s").
// Left justify overrides zero pad
// eg "%05d", Zero pad to field width less sign.
// Note that precision can end up negative so all
// the variables have to be 'signed int'.
//
// Don't run off the start of outbuf[], arbitrary limit
// longer than the longest number field.
// Stop gcc generating horrid code and memset().
// --out = '0';
// %#o has set sign_prefix to '0', but we don't want so add an extra
// leading zero here.
// Since the only other byte values of sign_prefix are ' ', '+' and '-'
// it is enough to check that out[] doesn't already start with sign_prefix.
//
// Add the 0, 1 or 2 ("0x") sign/prefix characters at the front.
// Force gcc to increment len inside the loop.
// --out = sign_prefix;

// Invalid format: back up to output the format characters
// and output a '%' now.
// %% is documented as a 'conversion specifier'.
// Any flags, precision or length modifier are ignored.
//
// Open coded strnlen() (slightly smaller).
// Stop gcc back-merging this code into one of the conditionals above.
// Output the characters on the required side of any padding.
// Output pad in 16 byte blocks with the small block first.
// Request a final '\0' be added to the snprintf() output.
// This may be the only call of the cb() function.
//
extern "C" {
    pub fn _fwrite(_arg: buf, _arg: size, _arg: stream) -> return;
}
extern "C" {
    pub fn __nolibc_printf(_arg: __nolibc_fprintf_cb, _arg: stream, _arg: fmt, _arg: args) -> return;
}
extern "C" {
    pub fn vfprintf(_arg: stdout, _arg: fmt, _arg: args) -> return;
}
// Technically 'stream' is leaked, but as it's only a wrapper around 'fd' that is fine
extern "C" {
    pub fn vfprintf(_arg: stream, _arg: fmt, _arg: args) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __nolibc_sprintf_cb_state {
    pub buf: *mut c_char,
    pub space: usize,
}

// Truncate the request to fit in the output buffer space.
// The last byte is reserved for the terminating '\0'.
// state->space can only be zero for snprintf(NULL, 0, fmt, args)
// so this normally lets through calls with 'size == 0'.
//
// __nolibc_printf() ends with cb(state, NULL, 0) to request the output
// buffer be '\0' terminated.
// That will be the only cb() call for, eg, snprintf(buf, sz, "").
// Zero lengths can occur at other times (eg "%s" for an empty string).
// Unconditionally write the '\0' byte to reduce code size, it is
// normally overwritten by the data being output.
// There is no point adding a '\0' after copied data - there is always
// another call.
//
// tgt = '\0';
extern "C" {
    pub fn __nolibc_printf(_arg: __nolibc_sprintf_cb, _arg: &state, _arg: fmt, _arg: args) -> return;
}
extern "C" {
    pub fn vsnprintf(_arg: buf, _arg: SIZE_MAX, _arg: fmt, _arg: args) -> return;
}
// strp = buf;
// start of pattern
// same as in printf()
// literal %
// va_arg(args, int *) = ival;
// va_arg(args, long *) = ival;
// va_arg(args, long long *) = ival;
// va_arg(args, unsigned int *) = uval;
// va_arg(args, unsigned long *) = uval;
// va_arg(args, unsigned long long *) = uval;
// va_arg(args, void **) = (void *)strtoul(str, &endptr, 16);
// skip spaces in format and str
// literal match

//
// nolibc does not support buffering so this is a nop. Just check mode
// is valid as required by the spec.
//
// Force gcc to use 'register offset' to access buf[].
// Use strerror_r() to avoid having the only .data in small programs.
