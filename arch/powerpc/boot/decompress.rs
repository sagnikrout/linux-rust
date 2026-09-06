//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/boot/decompress.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Wrapper around the kernel's pre-boot decompression library.
//
// Copyright (C) IBM Corporation 2016.
//

//
// The decompressor_*.c files play #ifdef games so they can be used in both
// pre-boot and regular kernel code. We need these definitions to make the
// includes work.
//

// Macro flag: #define INIT
//
// The build process will copy the required zlib source files and headers
// out of lib/ and "fix" the includes so they do not pull in other kernel
// headers.
//

// globals for tracking the state of the decompression
    static unsigned long decompressed_bytes;
    static unsigned long limit;
    static unsigned long skip;
    static char *output_buffer;
//
// flush() is called by __decompress() when the decompressor's scratch buffer is
// full.
//
#[no_mangle]
unsafe extern "C" fn flush(v: *mut c_void, buffer_size: c_ulong) -> c_long {
    static long flush(void *v, unsigned long buffer_size)
    {
    let mut end: c_ulong = decompressed_bytes + buffer_size;
    let mut size: c_ulong = buffer_size;
    let mut offset: c_ulong = 0;
    char *in = v;
    char *out;
//
// if we hit our decompression limit, we need to fake an error to abort
// the in-progress decompression.
//
    if (decompressed_bytes >= limit)
    return -1;
// skip this entire block
    if (end <= skip) {
    decompressed_bytes += buffer_size;
    return buffer_size;
    }
// skip some data at the start, but keep the rest of the block
    if (decompressed_bytes < skip && end > skip) {
    offset = skip - decompressed_bytes;
    in += offset;
    size -= offset;
    decompressed_bytes += offset;
    }
    out = &output_buffer[decompressed_bytes - skip];
    size = min(decompressed_bytes + size, limit) - decompressed_bytes;
    memcpy(out, in, size);
    decompressed_bytes += size;
    return buffer_size;
    }
#[no_mangle]
unsafe extern "C" fn print_err(s: *mut c_char) {
    static void print_err(char *s)
    {
// suppress the "error" when we terminate the decompressor
    if (decompressed_bytes >= limit)
    return;
    printf("Decompression error: '%s'\n\r", s);
    }
//
// partial_decompress - decompresses part or all of a compressed buffer
// @inbuf:       input buffer
// @input_size:  length of the input buffer
// @outbuf:      output buffer
// @output_size: length of the output buffer
// @_skip:       number of output bytes to ignore
//
// This function takes compressed data from inbuf, decompresses and write it to
// outbuf. Once output_size bytes are written to the output buffer, or the
// stream is exhausted the function will return the number of bytes that were
// decompressed. Otherwise it will return whatever error code the decompressor
// reported (NB: This is specific to each decompressor type).
//
// The skip functionality is mainly there so the program and discover
// the size of the compressed image so that it can ask firmware (if present)
// for an appropriately sized buffer.
//
    long partial_decompress(void *inbuf, unsigned long input_size,
    void *outbuf, unsigned long output_size, unsigned long _skip)
    {
    int ret;
//
// The skipped bytes needs to be included in the size of data we want
// to decompress.
//
    output_size += _skip;
    decompressed_bytes = 0;
    output_buffer = outbuf;
    limit = output_size;
    skip = _skip;
    ret = __decompress(inbuf, input_size, core::ptr::null_mut(), flush, outbuf,
    output_size, core::ptr::null_mut(), print_err);
//
// If decompression was aborted due to an actual error rather than
// a fake error that we used to abort, then we should report it.
//
    if (decompressed_bytes < limit)
    return ret;
    return decompressed_bytes - skip;
    }
