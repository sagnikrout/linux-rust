//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/zlib.c
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

pub const CHUNK_SIZE: c_int = 16384;
#[no_mangle]
pub unsafe extern "C" fn gzip_decompress_to_file(input: *const c_char, output_fd: c_int) -> c_int {
    int gzip_decompress_to_file(const char *input, int output_fd)
    {
    let mut ret: c_int = Z_STREAM_ERROR;
    int input_fd;
    void *ptr;
    int len;
    struct stat stbuf;
    unsigned char buf[CHUNK_SIZE];
    z_stream zs = {
    .zalloc		= Z_NULL,
    .zfree		= Z_NULL,
    .opaque		= Z_NULL,
    .avail_in	= 0,
    .next_in	= Z_NULL,
    };
    input_fd = open(input, O_RDONLY);
    if (input_fd < 0)
    return -1;
    if (fstat(input_fd, &stbuf) < 0)
    goto out_close;
    ptr = mmap(core::ptr::null_mut(), stbuf.st_size, PROT_READ, MAP_PRIVATE, input_fd, 0);
    if (ptr == MAP_FAILED)
    goto out_close;
    if (inflateInit2(&zs, 16 + MAX_WBITS) != Z_OK)
    goto out_unmap;
    zs.next_in = ptr;
    zs.avail_in = stbuf.st_size;
    do {
    zs.next_out = buf;
    zs.avail_out = CHUNK_SIZE;
    ret = inflate(&zs, Z_NO_FLUSH);
    switch (ret) {
    case Z_NEED_DICT:
    ret = Z_DATA_ERROR;
// fall through
    case Z_DATA_ERROR:
    case Z_MEM_ERROR:
    goto out;
    default:
    break;
    }
    len = CHUNK_SIZE - zs.avail_out;
    if (writen(output_fd, buf, len) != len) {
    ret = Z_DATA_ERROR;
    goto out;
    }
    } while (ret != Z_STREAM_END);
    out:
    inflateEnd(&zs);
    out_unmap:
    munmap(ptr, stbuf.st_size);
    out_close:
    close(input_fd);
    let mut ret: return = = Z_STREAM_END ? 0 : -1;
    }
#[no_mangle]
pub unsafe extern "C" fn gzip_is_compressed(input: *const c_char) -> bool {
    bool gzip_is_compressed(const char *input)
    {
    let mut fd: c_int = open(input, O_RDONLY);
    const uint8_t magic[2] = { 0x1f, 0x8b };
    char buf[2] = { 0 };
    ssize_t rc;
    if (fd < 0)
    return false;
    rc = read(fd, buf, sizeof(buf));
    close(fd);
    return rc == sizeof(buf) ?
    memcmp(buf, magic, sizeof(buf)) == 0 : false;
    }
