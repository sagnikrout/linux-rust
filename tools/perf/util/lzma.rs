//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/lzma.c
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

pub const BUFSIZE: c_int = 8192;
    static const char *lzma_strerror(lzma_ret ret)
    {
    switch ((int) ret) {
    case LZMA_MEM_ERROR:
    return "Memory allocation failed";
    case LZMA_OPTIONS_ERROR:
    return "Unsupported decompressor flags";
    case LZMA_FORMAT_ERROR:
    return "The input is not in the .xz format";
    case LZMA_DATA_ERROR:
    return "Compressed file is corrupt";
    case LZMA_BUF_ERROR:
    return "Compressed file is truncated or otherwise corrupt";
    default:
    return "Unknown error, possibly a bug";
    }
    }
#[no_mangle]
pub unsafe extern "C" fn lzma_decompress_stream_to_file(infile: *mut FILE, output_fd: c_int) -> c_int {
    int lzma_decompress_stream_to_file(FILE *infile, int output_fd)
    {
    let mut action: lzma_action = LZMA_RUN;
    let mut strm: lzma_stream = LZMA_STREAM_INIT;
    lzma_ret ret;
    let mut err: c_int = -1;
    u8 buf_in[BUFSIZE];
    u8 buf_out[BUFSIZE];
    ret = lzma_stream_decoder(&strm, UINT64_MAX, LZMA_CONCATENATED);
    if (ret != LZMA_OK) {
    pr_debug("lzma: lzma_stream_decoder failed %s (%d)\n", lzma_strerror(ret), ret);
    return err;
    }
    strm.next_in   = core::ptr::null_mut();
    strm.avail_in  = 0;
    strm.next_out  = buf_out;
    strm.avail_out = sizeof(buf_out);
    while (1) {
    if (strm.avail_in == 0 && !feof(infile)) {
    strm.next_in  = buf_in;
    strm.avail_in = fread(buf_in, 1, sizeof(buf_in), infile);
    if (ferror(infile)) {
    pr_debug("lzma: read error: %m\n");
    goto err_lzma_end;
    }
    if (feof(infile))
    action = LZMA_FINISH;
    }
    ret = lzma_code(&strm, action);
    if (strm.avail_out == 0 || ret == LZMA_STREAM_END) {
    let mut write_size: isize = sizeof(buf_out) - strm.avail_out;
    if (writen(output_fd, buf_out, write_size) != write_size) {
    pr_debug("lzma: write error: %m\n");
    goto err_lzma_end;
    }
    strm.next_out  = buf_out;
    strm.avail_out = sizeof(buf_out);
    }
    if (ret != LZMA_OK) {
    if (ret == LZMA_STREAM_END)
    break;
    pr_debug("lzma: failed %s\n", lzma_strerror(ret));
    goto err_lzma_end;
    }
    }
    err = 0;
    err_lzma_end:
    lzma_end(&strm);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn lzma_decompress_to_file(input: *const c_char, output_fd: c_int) -> c_int {
    int lzma_decompress_to_file(const char *input, int output_fd)
    {
    FILE *infile;
    int ret;
    infile = fopen(input, "rb");
    if (!infile) {
    pr_debug("lzma: fopen failed on %s: '%m'\n", input);
    return -1;
    }
    ret = lzma_decompress_stream_to_file(infile, output_fd);
    fclose(infile);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn lzma_is_compressed(input: *const c_char) -> bool {
    bool lzma_is_compressed(const char *input)
    {
    let mut fd: c_int = open(input, O_RDONLY);
    const uint8_t magic[6] = { 0xFD, '7', 'z', 'X', 'Z', 0x00 };
    char buf[6] = { 0 };
    ssize_t rc;
    if (fd < 0)
    return false;
    rc = read(fd, buf, sizeof(buf));
    close(fd);
    return rc == sizeof(buf) ?
    memcmp(buf, magic, sizeof(buf)) == 0 : false;
    }
