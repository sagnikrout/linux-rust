//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/vcodec/common/mtk_vcodec_dbgfs.h
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
//
// Copyright (c) 2023 MediaTek Inc.
// Author: Yunfei Dong <yunfei.dong@mediatek.com>
//
// enum mtk_vdec_dbgfs_log_index  - used to get different debug information
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_vdec_dbgfs_log_index {
    MTK_VDEC_DBGFS_PICINFO,
    MTK_VDEC_DBGFS_FORMAT,
    MTK_VDEC_DBGFS_MAX,
}

//
// struct mtk_vcodec_dbgfs_inst  - debugfs information for each inst
// @node:       list node for each inst
// @vcodec_ctx: struct mtk_vcodec_dec_ctx
// @inst_id:    index of the context that the same with ctx->id
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_vcodec_dbgfs_inst {
    pub node: list_head,
    pub vcodec_ctx: *mut mtk_vcodec_dec_ctx,
    pub inst_id: c_int,
}

//
// struct mtk_vcodec_dbgfs  - dbgfs information
// @dbgfs_head:  list head used to link each instance
// @vcodec_root: vcodec dbgfs entry
// @dbgfs_lock:  dbgfs lock used to protect dbgfs_buf
// @dbgfs_buf:   dbgfs buf used to store dbgfs cmd
// @buf_size:    buffer size of dbgfs
// @inst_count:  the count of total instance
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_vcodec_dbgfs {
    pub dbgfs_head: list_head,
    pub vcodec_root: *mut dentry,
    pub dbgfs_lock: mutex,
    pub dbgfs_buf: [c_char; 1024],
    pub buf_size: c_int,
    pub inst_count: c_int,
}

extern "C" {
    pub fn mtk_vcodec_dbgfs_create(ctx: *mut mtk_vcodec_dec_ctx);
}
extern "C" {
    pub fn mtk_vcodec_dbgfs_remove(vcodec_dev: *mut mtk_vcodec_dec_dev, ctx_id: c_int);
}
extern "C" {
    pub fn mtk_vcodec_dbgfs_init(vcodec_dev: *mut c_void, is_encode: bool);
}
extern "C" {
    pub fn mtk_vcodec_dbgfs_deinit(dbgfs: *mut mtk_vcodec_dbgfs);
}

