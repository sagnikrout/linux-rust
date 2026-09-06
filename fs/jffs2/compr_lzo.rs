//! Automatically rewritten from C to Rust
//! Source: fs/jffs2/compr_lzo.c
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
// Copyright © 2007 Nokia Corporation. All rights reserved.
// Copyright © 2004-2010 David Woodhouse <dwmw2@infradead.org>
//
// Created by Richard Purdie <rpurdie@openedhand.com>
//
// For licensing information, see the file 'LICENCE' in this directory.
//

    static void *lzo_mem;
    static void *lzo_compress_buf;
    static DEFINE_MUTEX(deflate_mutex);	/* for lzo_mem and lzo_compress_buf */
#[no_mangle]
unsafe extern "C" fn free_workspace() {
    static void free_workspace(void)
    {
    vfree(lzo_mem);
    vfree(lzo_compress_buf);
    }
#[no_mangle]
unsafe extern "C" fn alloc_workspace() -> int __init {
    static int __init alloc_workspace(void)
    {
    lzo_mem = vmalloc(LZO1X_MEM_COMPRESS);
    lzo_compress_buf = vmalloc(lzo1x_worst_compress(PAGE_SIZE));
    if (!lzo_mem || !lzo_compress_buf) {
    free_workspace();
    return -ENOMEM;
    }
    return 0;
    }
    static int jffs2_lzo_compress(unsigned char *data_in, unsigned char *cpage_out,
    uint32_t *sourcelen, uint32_t *dstlen)
    {
    size_t compress_size;
    int ret;
    mutex_lock(&deflate_mutex);
    ret = lzo1x_1_compress(data_in, *sourcelen, lzo_compress_buf, &compress_size, lzo_mem);
    if (ret != LZO_E_OK)
    goto fail;
    if (compress_size > *dstlen)
    goto fail;
    memcpy(cpage_out, lzo_compress_buf, compress_size);
    mutex_unlock(&deflate_mutex);
// dstlen = compress_size;
    return 0;
    fail:
    mutex_unlock(&deflate_mutex);
    return -1;
    }
    static int jffs2_lzo_decompress(unsigned char *data_in, unsigned char *cpage_out,
    uint32_t srclen, uint32_t destlen)
    {
    let mut dl: usize = destlen;
    int ret;
    ret = lzo1x_decompress_safe(data_in, srclen, cpage_out, &dl);
    if (ret != LZO_E_OK || dl != destlen)
    return -1;
    return 0;
    }
    static struct jffs2_compressor jffs2_lzo_comp = {
    .priority = JFFS2_LZO_PRIORITY,
    .name = "lzo",
    .compr = JFFS2_COMPR_LZO,
    .compress = &jffs2_lzo_compress,
    .decompress = &jffs2_lzo_decompress,
    .disabled = 0,
    };
#[no_mangle]
pub unsafe extern "C" fn jffs2_lzo_init() -> int __init {
    int __init jffs2_lzo_init(void)
    {
    int ret;
    ret = alloc_workspace();
    if (ret < 0)
    return ret;
    ret = jffs2_register_compressor(&jffs2_lzo_comp);
    if (ret)
    free_workspace();
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn jffs2_lzo_exit() {
    void jffs2_lzo_exit(void)
    {
    jffs2_unregister_compressor(&jffs2_lzo_comp);
    free_workspace();
    }
