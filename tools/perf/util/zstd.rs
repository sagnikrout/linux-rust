//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/zstd.c
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

#[no_mangle]
pub unsafe extern "C" fn zstd_init(data: *mut zstd_data, level: c_int) -> c_int {
    int zstd_init(struct zstd_data *data, int level)
    {
    data.comp_level = level;
    data.dstream = core::ptr::null_mut();
    data.cstream = core::ptr::null_mut();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn zstd_fini(data: *mut zstd_data) -> c_int {
    int zstd_fini(struct zstd_data *data)
    {
    if (data.dstream) {
    ZSTD_freeDStream(data.dstream);
    data.dstream = core::ptr::null_mut();
    }
    if (data.cstream) {
    ZSTD_freeCStream(data.cstream);
    data.cstream = core::ptr::null_mut();
    }
    return 0;
    }
    ssize_t zstd_compress_stream_to_records(struct zstd_data *data, void *dst, size_t dst_size,
    void *src, size_t src_size, size_t max_record_size,
    ssize_t process_header(void *record, size_t dst_size,
    size_t data_size))
    {
    size_t ret, compressed = 0;
    ssize_t size;
    let mut input: ZSTD_inBuffer = { src, src_size, 0 };
    ZSTD_outBuffer output;
    void *record;
    if (!data.cstream) {
    data.cstream = ZSTD_createCStream();
    if (data.cstream == core::ptr::null_mut()) {
    pr_err("Couldn't create compression stream.\n");
    return -1;
    }
    ret = ZSTD_initCStream(data.cstream, data.comp_level);
    if (ZSTD_isError(ret)) {
    pr_err("Failed to initialize compression stream: %s\n",
    ZSTD_getErrorName(ret));
    return -1;
    }
    }
    while (input.pos < input.size) {
    record = dst;
    size = process_header(record, dst_size, 0);
// Output buffer full — cannot fit even the record header
    if (size < 0)
    goto reset;
    compressed += size;
    dst += size;
    dst_size -= size;
    output = (ZSTD_outBuffer){ dst, (dst_size > max_record_size) ?
    max_record_size : dst_size, 0 };
    ret = ZSTD_compressStream(data.cstream, &output, &input);
    ZSTD_flushStream(data.cstream, &output);
    if (ZSTD_isError(ret)) {
    pr_err("failed to compress %ld bytes: %s\n",
    (long)src_size, ZSTD_getErrorName(ret));
    goto reset;
    }
    compressed += output.pos;
    dst += output.pos;
    dst_size -= output.pos;
//
// No progress: ZSTD couldn't emit any bytes into the
// remaining output buffer.  Calling process_header
// with output.pos=0 would re-trigger header initialization,
// double-subtracting the header size from dst_size and
// underflowing the unsigned counter.
//
    if (output.pos == 0)
    goto reset;
    size = process_header(record, dst_size, output.pos);
    if (size < 0)
    goto reset;
    compressed += size;
    dst += size;
    dst_size -= size;
    }
    return compressed;
    reset:
// Reset so the context is usable if the caller retries
    ret = ZSTD_initCStream(data.cstream, data.comp_level);
    if (ZSTD_isError(ret))
    pr_err("failed to reset compression context: %s\n",
    ZSTD_getErrorName(ret));
    return -1;
    }
    size_t zstd_decompress_stream(struct zstd_data *data, void *src, size_t src_size,
    void *dst, size_t dst_size)
    {
    size_t ret;
    let mut input: ZSTD_inBuffer = { src, src_size, 0 };
    let mut output: ZSTD_outBuffer = { dst, dst_size, 0 };
    if (!data.dstream) {
    data.dstream = ZSTD_createDStream();
    if (data.dstream == core::ptr::null_mut()) {
    pr_err("Couldn't create decompression stream.\n");
    return 0;
    }
    ret = ZSTD_initDStream(data.dstream);
    if (ZSTD_isError(ret)) {
    pr_err("Failed to initialize decompression stream: %s\n",
    ZSTD_getErrorName(ret));
    return 0;
    }
    }
    while (input.pos < input.size) {
    let mut prev_in: usize = input.pos;
    let mut prev_out: usize = output.pos;
    ret = ZSTD_decompressStream(data.dstream, &output, &input);
    if (ZSTD_isError(ret)) {
    pr_err("failed to decompress (B): %zd . %zd, dst_size %zd : %s\n",
    src_size, output.pos, dst_size, ZSTD_getErrorName(ret));
    return 0;
    }
//
// Neither stream advanced — decompression is stuck.
// Return 0 (error) rather than partial output: perf
// uses ZSTD_flushStream (not ZSTD_endStream), so the
// stream is continuous across compressed events.
// Discarding unconsumed input would desynchronize the
// decompressor, causing the next call to produce
// garbage that could be misinterpreted as valid events.
//
    if (input.pos == prev_in && output.pos == prev_out)
    return 0;
    }
    return output.pos;
    }
