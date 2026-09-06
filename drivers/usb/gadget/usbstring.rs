//! Automatically rewritten from C to Rust
//! Source: drivers/usb/gadget/usbstring.c
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


// SPDX-License-Identifier: LGPL-2.1+
//
// Copyright (C) 2003 David Brownell
//

//
// usb_gadget_get_string - fill out a string descriptor
// @table: of c strings encoded using UTF-8
// @id: string id, from low byte of wValue in get string descriptor
// @buf: at least 256 bytes, must be 16-bit aligned
//
// Finds the UTF-8 string matching the ID, and converts it into a
// string descriptor in utf16-le.
// Returns length of descriptor (always even) or negative errno
//
// If your driver needs stings in multiple languages, you'll probably
// "switch (wIndex) { ... }"  in your ep0 string descriptor logic,
// using this routine after choosing which set of UTF-8 strings to use.
// Note that US-ASCII is a strict subset of UTF-8; any string bytes with
// the eighth bit set will be multibyte UTF-8 characters, not ISO-8859/1
// characters (which are also widely used in C strings).
//
    int
    usb_gadget_get_string (const struct usb_gadget_strings *table, int id, u8 *buf)
    {
    struct usb_string	*s;
    int			len;
// descriptor 0 has the language id
    if (id == 0) {
    buf [0] = 4;
    buf [1] = USB_DT_STRING;
    buf [2] = (u8) table.language;
    buf [3] = (u8) (table.language >> 8);
    return 4;
    }
    for (s = table.strings; s && s.s; s++)
    if (s.id == id)
    break;
// unrecognized: stall.
    if (!s || !s.s)
    return -EINVAL;
// string descriptors have length, tag, then UTF16-LE text
    len = min_t(size_t, USB_MAX_STRING_LEN, strlen(s.s));
    len = utf8s_to_utf16s(s.s, len, UTF16_LITTLE_ENDIAN,
    (wchar_t *) &buf[2], USB_MAX_STRING_LEN);
    if (len < 0)
    return -EINVAL;
    buf [0] = (len + 1) * 2;
    buf [1] = USB_DT_STRING;
    return buf [0];
    }
    EXPORT_SYMBOL_GPL(usb_gadget_get_string);
//
// usb_validate_langid - validate usb language identifiers
// @langid: usb language identifier
//
// Returns true for valid language identifier, otherwise false.
//
#[no_mangle]
pub unsafe extern "C" fn usb_validate_langid(langid: u16) -> bool {
    bool usb_validate_langid(u16 langid)
    {
    u16 primary_lang = langid & 0x3ff;	/* bit [9:0] */
    u16 sub_lang = langid >> 10;		/* bit [15:10] */
    switch (primary_lang) {
    case 0:
    case 0x62 ... 0xfe:
    case 0x100 ... 0x3ff:
    return false;
    }
    if (!sub_lang)
    return false;
    return true;
    }
    EXPORT_SYMBOL_GPL(usb_validate_langid);
