//! Automatically rewritten from C to Rust
//! Source: lib/decompress_inflate.c
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

// Macro flag: #define PREBOOT
// Pre-boot environment: included
// prevent inclusion of _LINUX_KERNEL_H in pre-boot environment: lots
// errors about console_printk etc... on ARM
// Macro flag: #define _LINUX_KERNEL_H

// initramfs et al: linked

#[no_mangle]
unsafe extern "C" fn nofill(buffer: *mut c_void, len: c_ulong) -> long INIT {
    static long INIT nofill(void *buffer, unsigned long len)
    {
    return -1;
    }
// Included from initramfs et al code
    static int INIT __gunzip(unsigned char *buf, long len,
    long (*fill)(void*, unsigned long),
    long (*flush)(void*, unsigned long),
    unsigned char *out_buf, long out_len,
    long *pos,
    void(*error)(char *x)) {
    u8 *zbuf;
    struct z_stream_s *strm;
    int rc;
    rc = -1;
    if (flush) {
    out_len = 0x8000; /* 32 K */
    out_buf = malloc(out_len);
    } else {
    if (!out_len)
    out_len = ((size_t)~0) - (size_t)out_buf; /* no limit */
    }
    if (!out_buf) {
    error("Out of memory while allocating output buffer");
    goto gunzip_nomem1;
    }
    if (buf)
    zbuf = buf;
    else {
    zbuf = malloc(GZIP_IOBUF_SIZE);
    len = 0;
    }
    if (!zbuf) {
    error("Out of memory while allocating input buffer");
    goto gunzip_nomem2;
    }
    strm = malloc(sizeof(*strm));
    if (strm == core::ptr::null_mut()) {
    error("Out of memory while allocating z_stream");
    goto gunzip_nomem3;
    }
    strm.workspace = malloc(flush ? zlib_inflate_workspacesize() :

// Always allocate the full workspace for DFLTCC
    zlib_inflate_workspacesize());

    sizeof(struct inflate_state));

    if (strm.workspace == core::ptr::null_mut()) {
    error("Out of memory while allocating workspace");
    goto gunzip_nomem4;
    }
    if (!fill)
    fill = nofill;
    if (len == 0)
    len = fill(zbuf, GZIP_IOBUF_SIZE);
// verify the gzip header
    if (len < 10 ||
    zbuf[0] != 0x1f || zbuf[1] != 0x8b || zbuf[2] != 0x08) {
    if (pos)
// pos = 0;
    error("Not a gzip file");
    goto gunzip_5;
    }
// skip over gzip header (1f,8b,08... 10 bytes total +
// possible asciz filename)
//
    strm.next_in = zbuf + 10;
    strm.avail_in = len - 10;
// skip over asciz filename
    if (zbuf[3] & 0x8) {
    do {
//
// If the filename doesn't fit into the buffer,
// the file is very probably corrupt. Don't try
// to read more data.
//
    if (strm.avail_in == 0) {
    error("header error");
    goto gunzip_5;
    }
    --strm.avail_in;
    } while (*strm.next_in++);
    }
    strm.next_out = out_buf;
    strm.avail_out = out_len;
    rc = zlib_inflateInit2(strm, -MAX_WBITS);

// Always keep the window for DFLTCC

    if (!flush) {
    WS(strm).inflate_state.wsize = 0;
    WS(strm).inflate_state.window = core::ptr::null_mut();
    }

    while (rc == Z_OK) {
    if (strm.avail_in == 0) {
// TODO: handle case where both pos and fill are set
    len = fill(zbuf, GZIP_IOBUF_SIZE);
    if (len < 0) {
    rc = -1;
    error("read error");
    break;
    }
    strm.next_in = zbuf;
    strm.avail_in = len;
    }
    rc = zlib_inflate(strm, 0);
// Write any data generated
    if (flush && strm.next_out > out_buf) {
    let mut l: c_long = strm.next_out - out_buf;
    if (l != flush(out_buf, l)) {
    rc = -1;
    error("write error");
    break;
    }
    strm.next_out = out_buf;
    strm.avail_out = out_len;
    }
// after Z_FINISH, only Z_STREAM_END is "we unpacked it all"
    if (rc == Z_STREAM_END) {
    rc = 0;
    break;
    } else if (rc != Z_OK) {
    error("uncompression error");
    rc = -1;
    }
    }
    zlib_inflateEnd(strm);
    if (pos)
// add + 8 to skip over trailer
// pos = strm->next_in - zbuf+8;
    gunzip_5:
    free(strm.workspace);
    gunzip_nomem4:
    free(strm);
    gunzip_nomem3:
    if (!buf)
    free(zbuf);
    gunzip_nomem2:
    if (flush)
    free(out_buf);
    gunzip_nomem1:
    return rc; /* returns Z_OK (0) if successful */
    }

    STATIC int INIT gunzip(unsigned char *buf, long len,
    long (*fill)(void*, unsigned long),
    long (*flush)(void*, unsigned long),
    unsigned char *out_buf,
    long *pos,
    void (*error)(char *x))
    {
    return __gunzip(buf, len, fill, flush, out_buf, 0, pos, error);
    }

    STATIC int INIT __decompress(unsigned char *buf, long len,
    long (*fill)(void*, unsigned long),
    long (*flush)(void*, unsigned long),
    unsigned char *out_buf, long out_len,
    long *pos,
    void (*error)(char *x))
    {
    return __gunzip(buf, len, fill, flush, out_buf, out_len, pos, error);
    }
