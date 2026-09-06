//! Automatically rewritten from C to Rust
//! Source: fs/jffs2/compr_zlib.c
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


//
// JFFS2 -- Journalling Flash File System, Version 2.
//
// Copyright © 2001-2007 Red Hat, Inc.
// Copyright © 2004-2010 David Woodhouse <dwmw2@infradead.org>
//
// Created by David Woodhouse <dwmw2@infradead.org>
//
// For licensing information, see the file 'LICENCE' in this directory.
//

// Plan: call deflate() with avail_in == *sourcelen,
    avail_out = *dstlen - 12 and flush == Z_FINISH.
    If it doesn't manage to finish,	call it again with
    avail_in == 0 and avail_out set to the remaining 12
    bytes for it to clean up.
    Q: Is 12 bytes sufficient?
//
pub const STREAM_END_SPACE: c_int = 12;
    static DEFINE_MUTEX(deflate_mutex);
    static DEFINE_MUTEX(inflate_mutex);
    static z_stream inf_strm, def_strm;

#[no_mangle]
unsafe extern "C" fn alloc_workspaces() -> int __init {
    static int __init alloc_workspaces(void)
    {
    def_strm.workspace = vmalloc(zlib_deflate_workspacesize(MAX_WBITS,
    MAX_MEM_LEVEL));
    if (!def_strm.workspace)
    return -ENOMEM;
    jffs2_dbg(1, "Allocated %d bytes for deflate workspace\n",
    zlib_deflate_workspacesize(MAX_WBITS, MAX_MEM_LEVEL));
    inf_strm.workspace = vmalloc(zlib_inflate_workspacesize());
    if (!inf_strm.workspace) {
    vfree(def_strm.workspace);
    return -ENOMEM;
    }
    jffs2_dbg(1, "Allocated %d bytes for inflate workspace\n",
    zlib_inflate_workspacesize());
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn free_workspaces() {
    static void free_workspaces(void)
    {
    vfree(def_strm.workspace);
    vfree(inf_strm.workspace);
    }

    static int jffs2_zlib_compress(unsigned char *data_in,
    unsigned char *cpage_out,
    uint32_t *sourcelen, uint32_t *dstlen)
    {
    int ret;
    if (*dstlen <= STREAM_END_SPACE)
    return -1;
    mutex_lock(&deflate_mutex);
    if (Z_OK != zlib_deflateInit(&def_strm, 3)) {
    pr_warn("deflateInit failed\n");
    mutex_unlock(&deflate_mutex);
    return -1;
    }
    def_strm.next_in = data_in;
    def_strm.total_in = 0;
    def_strm.next_out = cpage_out;
    def_strm.total_out = 0;
    while (def_strm.total_out < *dstlen - STREAM_END_SPACE && def_strm.total_in < *sourcelen) {
    def_strm.avail_out = *dstlen - (def_strm.total_out + STREAM_END_SPACE);
    def_strm.avail_in = min_t(unsigned long,
    (*sourcelen-def_strm.total_in), def_strm.avail_out);
    jffs2_dbg(1, "calling deflate with avail_in %ld, avail_out %ld\n",
    def_strm.avail_in, def_strm.avail_out);
    ret = zlib_deflate(&def_strm, Z_PARTIAL_FLUSH);
    jffs2_dbg(1, "deflate returned with avail_in %ld, avail_out %ld, total_in %ld, total_out %ld\n",
    def_strm.avail_in, def_strm.avail_out,
    def_strm.total_in, def_strm.total_out);
    if (ret != Z_OK) {
    jffs2_dbg(1, "deflate in loop returned %d\n", ret);
    zlib_deflateEnd(&def_strm);
    mutex_unlock(&deflate_mutex);
    return -1;
    }
    }
    def_strm.avail_out += STREAM_END_SPACE;
    def_strm.avail_in = 0;
    ret = zlib_deflate(&def_strm, Z_FINISH);
    zlib_deflateEnd(&def_strm);
    if (ret != Z_STREAM_END) {
    jffs2_dbg(1, "final deflate returned %d\n", ret);
    ret = -1;
    goto out;
    }
    if (def_strm.total_out >= def_strm.total_in) {
    jffs2_dbg(1, "zlib compressed %ld bytes into %ld; failing\n",
    def_strm.total_in, def_strm.total_out);
    ret = -1;
    goto out;
    }
    jffs2_dbg(1, "zlib compressed %ld bytes into %ld\n",
    def_strm.total_in, def_strm.total_out);
// dstlen = def_strm.total_out;
// sourcelen = def_strm.total_in;
    ret = 0;
    out:
    mutex_unlock(&deflate_mutex);
    return ret;
    }
    static int jffs2_zlib_decompress(unsigned char *data_in,
    unsigned char *cpage_out,
    uint32_t srclen, uint32_t destlen)
    {
    int ret;
    let mut wbits: c_int = MAX_WBITS;
    mutex_lock(&inflate_mutex);
    inf_strm.next_in = data_in;
    inf_strm.avail_in = srclen;
    inf_strm.total_in = 0;
    inf_strm.next_out = cpage_out;
    inf_strm.avail_out = destlen;
    inf_strm.total_out = 0;
// If it's deflate, and it's got no preset dictionary, then
    we can tell zlib to skip the adler32 check. */
    if (srclen > 2 && !(data_in[1] & PRESET_DICT) &&
    ((data_in[0] & 0x0f) == Z_DEFLATED) &&
    !(((data_in[0]<<8) + data_in[1]) % 31)) {
    jffs2_dbg(2, "inflate skipping adler32\n");
    wbits = -((data_in[0] >> 4) + 8);
    inf_strm.next_in += 2;
    inf_strm.avail_in -= 2;
    } else {
// Let this remain D1 for now -- it should never happen
    jffs2_dbg(1, "inflate not skipping adler32\n");
    }
    if (Z_OK != zlib_inflateInit2(&inf_strm, wbits)) {
    pr_warn("inflateInit failed\n");
    mutex_unlock(&inflate_mutex);
    return 1;
    }
    while((ret = zlib_inflate(&inf_strm, Z_FINISH)) == Z_OK)
    ;
    if (ret != Z_STREAM_END) {
    pr_notice("inflate returned %d\n", ret);
    }
    zlib_inflateEnd(&inf_strm);
    mutex_unlock(&inflate_mutex);
    return 0;
    }
    static struct jffs2_compressor jffs2_zlib_comp = {
    .priority = JFFS2_ZLIB_PRIORITY,
    .name = "zlib",
    .compr = JFFS2_COMPR_ZLIB,
    .compress = &jffs2_zlib_compress,
    .decompress = &jffs2_zlib_decompress,

    .disabled = 1,

    .disabled = 0,

    };
#[no_mangle]
pub unsafe extern "C" fn jffs2_zlib_init() -> int __init {
    int __init jffs2_zlib_init(void)
    {
    int ret;
    ret = alloc_workspaces();
    if (ret)
    return ret;
    ret = jffs2_register_compressor(&jffs2_zlib_comp);
    if (ret)
    free_workspaces();
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn jffs2_zlib_exit() {
    void jffs2_zlib_exit(void)
    {
    jffs2_unregister_compressor(&jffs2_zlib_comp);
    free_workspaces();
    }
