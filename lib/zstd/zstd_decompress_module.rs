//! Automatically rewritten from C to Rust
//! Source: lib/zstd/zstd_decompress_module.c
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


// SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause
//
// Copyright (c) Meta Platforms, Inc. and affiliates.
// All rights reserved.
//
// This source code is licensed under both the BSD-style license (found in the
// LICENSE file in the root directory of this source tree) and the GPLv2 (found
// in the COPYING file in the root directory of this source tree).
// You may select, at your option, one of the above-listed licenses.
//

// Common symbols. zstd_compress must depend on zstd_decompress.
#[no_mangle]
pub unsafe extern "C" fn zstd_is_error(code: usize) -> c_uint {
    unsigned int zstd_is_error(size_t code)
    {
    return ZSTD_isError(code);
    }
    EXPORT_SYMBOL(zstd_is_error);
#[no_mangle]
pub unsafe extern "C" fn zstd_get_error_code(code: usize) -> zstd_error_code {
    zstd_error_code zstd_get_error_code(size_t code)
    {
    return ZSTD_getErrorCode(code);
    }
    EXPORT_SYMBOL(zstd_get_error_code);
    const char *zstd_get_error_name(size_t code)
    {
    return ZSTD_getErrorName(code);
    }
    EXPORT_SYMBOL(zstd_get_error_name);
// Decompression symbols.
#[no_mangle]
pub unsafe extern "C" fn zstd_dctx_workspace_bound() -> usize {
    size_t zstd_dctx_workspace_bound(void)
    {
    return ZSTD_estimateDCtxSize();
    }
    EXPORT_SYMBOL(zstd_dctx_workspace_bound);
    zstd_dctx *zstd_create_dctx_advanced(zstd_custom_mem custom_mem)
    {
    return ZSTD_createDCtx_advanced(custom_mem);
    }
    EXPORT_SYMBOL(zstd_create_dctx_advanced);
#[no_mangle]
pub unsafe extern "C" fn zstd_free_dctx(dctx: *mut zstd_dctx) -> usize {
    size_t zstd_free_dctx(zstd_dctx *dctx)
    {
    return ZSTD_freeDCtx(dctx);
    }
    EXPORT_SYMBOL(zstd_free_dctx);
    zstd_ddict *zstd_create_ddict_byreference(const void *dict, size_t dict_size,
    zstd_custom_mem custom_mem)
    {
    return ZSTD_createDDict_advanced(dict, dict_size, ZSTD_dlm_byRef,
    ZSTD_dct_auto, custom_mem);
    }
    EXPORT_SYMBOL(zstd_create_ddict_byreference);
#[no_mangle]
pub unsafe extern "C" fn zstd_free_ddict(ddict: *mut zstd_ddict) -> usize {
    size_t zstd_free_ddict(zstd_ddict *ddict)
    {
    return ZSTD_freeDDict(ddict);
    }
    EXPORT_SYMBOL(zstd_free_ddict);
    zstd_dctx *zstd_init_dctx(void *workspace, size_t workspace_size)
    {
    if (workspace == core::ptr::null_mut())
    return core::ptr::null_mut();
    return ZSTD_initStaticDCtx(workspace, workspace_size);
    }
    EXPORT_SYMBOL(zstd_init_dctx);
    size_t zstd_decompress_dctx(zstd_dctx *dctx, void *dst, size_t dst_capacity,
    const void *src, size_t src_size)
    {
    return ZSTD_decompressDCtx(dctx, dst, dst_capacity, src, src_size);
    }
    EXPORT_SYMBOL(zstd_decompress_dctx);
    size_t zstd_decompress_using_ddict(zstd_dctx *dctx,
    void *dst, size_t dst_capacity, const void* src, size_t src_size,
    const zstd_ddict* ddict)
    {
    return ZSTD_decompress_usingDDict(dctx, dst, dst_capacity, src,
    src_size, ddict);
    }
    EXPORT_SYMBOL(zstd_decompress_using_ddict);
#[no_mangle]
pub unsafe extern "C" fn zstd_dstream_workspace_bound(max_window_size: usize) -> usize {
    size_t zstd_dstream_workspace_bound(size_t max_window_size)
    {
    return ZSTD_estimateDStreamSize(max_window_size);
    }
    EXPORT_SYMBOL(zstd_dstream_workspace_bound);
    zstd_dstream *zstd_init_dstream(size_t max_window_size, void *workspace,
    size_t workspace_size)
    {
    if (workspace == core::ptr::null_mut())
    return core::ptr::null_mut();
    (void)max_window_size;
    return ZSTD_initStaticDStream(workspace, workspace_size);
    }
    EXPORT_SYMBOL(zstd_init_dstream);
#[no_mangle]
pub unsafe extern "C" fn zstd_reset_dstream(dstream: *mut zstd_dstream) -> usize {
    size_t zstd_reset_dstream(zstd_dstream *dstream)
    {
    return ZSTD_DCtx_reset(dstream, ZSTD_reset_session_only);
    }
    EXPORT_SYMBOL(zstd_reset_dstream);
    size_t zstd_decompress_stream(zstd_dstream *dstream, zstd_out_buffer *output,
    zstd_in_buffer *input)
    {
    return ZSTD_decompressStream(dstream, output, input);
    }
    EXPORT_SYMBOL(zstd_decompress_stream);
#[no_mangle]
pub unsafe extern "C" fn zstd_find_frame_compressed_size(src: *const c_void, src_size: usize) -> usize {
    size_t zstd_find_frame_compressed_size(const void *src, size_t src_size)
    {
    return ZSTD_findFrameCompressedSize(src, src_size);
    }
    EXPORT_SYMBOL(zstd_find_frame_compressed_size);
    size_t zstd_get_frame_header(zstd_frame_header *header, const void *src,
    size_t src_size)
    {
    return ZSTD_getFrameHeader(header, src, src_size);
    }
    EXPORT_SYMBOL(zstd_get_frame_header);
    MODULE_LICENSE("Dual BSD/GPL");
    MODULE_DESCRIPTION("Zstd Decompressor");
