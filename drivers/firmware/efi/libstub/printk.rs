//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/efi/libstub/printk.c
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

    let mut efi_loglevel: c_int = LOGLEVEL_NOTICE;
//
// efi_char16_puts() - Write a UCS-2 encoded string to the console
// @str:	UCS-2 encoded string
//
#[no_mangle]
pub unsafe extern "C" fn efi_char16_puts(str: *mut efi_char16_t) {
    void efi_char16_puts(efi_char16_t *str)
    {
    efi_call_proto(efi_table_attr(efi_system_table, con_out),
    output_string, str);
    }
    static
#[no_mangle]
pub unsafe extern "C" fn utf8_to_utf32(s8: *const u8) -> u32 {
    u32 utf8_to_utf32(const u8 **s8)
    {
    u32 c32;
    u8 c0, cx;
    size_t clen, i;
    c0 = cx = *(*s8)++;
//
// The position of the most-significant 0 bit gives us the length of
// a multi-octet encoding.
//
    for (clen = 0; cx & 0x80; ++clen)
    cx <<= 1;
//
// If the 0 bit is in position 8, this is a valid single-octet
// encoding. If the 0 bit is in position 7 or positions 1-3, the
// encoding is invalid.
// In either case, we just return the first octet.
//
    if (clen < 2 || clen > 4)
    return c0;
// Get the bits from the first octet.
    c32 = cx >> clen--;
    for (i = 0; i < clen; ++i) {
// Trailing octets must have 10 in most significant bits.
    cx = (*s8)[i] ^ 0x80;
    if (cx & 0xc0)
    return c0;
    c32 = (c32 << 6) | cx;
    }
//
// Check for validity:
// - The character must be in the Unicode range.
// - It must not be a surrogate.
// - It must be encoded using the correct number of octets.
//
    if (c32 > 0x10ffff ||
    (c32 & 0xf800) == 0xd800 ||
    clen != (c32 >= 0x80) + (c32 >= 0x800) + (c32 >= 0x10000))
    return c0;
// s8 += clen;
    return c32;
    }
//
// efi_puts() - Write a UTF-8 encoded string to the console
// @str:	UTF-8 encoded string
//
#[no_mangle]
pub unsafe extern "C" fn efi_puts(str: *const c_char) {
    void efi_puts(const char *str)
    {
    efi_char16_t buf[128];
    let mut pos: usize = 0, lim = ARRAY_SIZE(buf);
    const u8 *s8 = (const u8 *)str;
    u32 c32;
    while (*s8) {
    if (*s8 == '\n')
    buf[pos++] = L'\r';
    c32 = utf8_to_utf32(&s8);
    if (c32 < 0x10000) {
// Characters in plane 0 use a single word.
    buf[pos++] = c32;
    } else {
//
// Characters in other planes encode into a surrogate
// pair.
//
    buf[pos++] = (0xd800 - (0x10000 >> 10)) + (c32 >> 10);
    buf[pos++] = 0xdc00 + (c32 & 0x3ff);
    }
    if (*s8 == '\0' || pos >= lim - 2) {
    buf[pos] = L'\0';
    efi_char16_puts(buf);
    pos = 0;
    }
    }
    }
//
// efi_printk() - Print a kernel message
// @fmt:	format string
//
// The first letter of the format string is used to determine the logging level
// of the message. If the level is less then the current EFI logging level, the
// message is suppressed. The message will be truncated to 255 bytes.
//
// Return:	number of printed characters
//
#[no_mangle]
pub unsafe extern "C" fn efi_printk(fmt: *const c_char, ...) -> c_int {
    int efi_printk(const char *fmt, ...)
    {
    char printf_buf[256];
    va_list args;
    int printed;
    let mut loglevel: c_int = printk_get_level(fmt);
    switch (loglevel) {
    case '0' ... '9':
    loglevel -= '0';
    break;
    default:
//
// Use loglevel -1 for cases where we just want to print to
// the screen.
//
    loglevel = -1;
    break;
    }
    if (loglevel >= efi_loglevel)
    return 0;
    if (loglevel >= 0)
    efi_puts("EFI stub: ");
    fmt = printk_skip_level(fmt);
    va_start(args, fmt);
    printed = vsnprintf(printf_buf, sizeof(printf_buf), fmt, args);
    va_end(args);
    efi_puts(printf_buf);
    if (printed >= sizeof(printf_buf)) {
    efi_puts("[Message truncated]\n");
    return -1;
    }
    return printed;
    }
