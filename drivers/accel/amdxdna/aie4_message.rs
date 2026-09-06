//! Automatically rewritten from C to Rust
//! Source: drivers/accel/amdxdna/aie4_message.c
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
// Copyright (C) 2026, Advanced Micro Devices, Inc.
//

#[no_mangle]
pub unsafe extern "C" fn aie4_suspend_fw(ndev: *mut amdxdna_dev_hdl) -> c_int {
    int aie4_suspend_fw(struct amdxdna_dev_hdl *ndev)
    {
    DECLARE_AIE_MSG(aie4_msg_suspend, AIE4_MSG_OP_SUSPEND);
    int ret;
    ret = aie_send_mgmt_msg_wait(&ndev.aie, &msg);
    if (ret)
    XDNA_ERR(ndev.aie.xdna, "Failed to suspend fw, ret %d", ret);
    return ret;
    }
    int aie4_query_aie_metadata(struct amdxdna_dev_hdl *ndev,
    struct amdxdna_drm_query_aie_metadata *metadata)
    {
    DECLARE_AIE_MSG(aie4_msg_aie4_tile_info, AIE4_MSG_OP_AIE_TILE_INFO);
    int ret;
    ret = aie_send_mgmt_msg_wait(&ndev.aie, &msg);
    if (ret)
    return ret;
    metadata.col_size = resp.info.size;
    metadata.cols = resp.info.cols;
    metadata.rows = resp.info.rows;
    metadata.version.major = resp.info.major;
    metadata.version.minor = resp.info.minor;
    metadata.core.row_count = resp.info.core_rows;
    metadata.core.row_start = resp.info.core_row_start;
    metadata.core.dma_channel_count = resp.info.core_dma_channels;
    metadata.core.lock_count = resp.info.core_locks;
    metadata.core.event_reg_count = resp.info.core_events;
    metadata.mem.row_count = resp.info.mem_rows;
    metadata.mem.row_start = resp.info.mem_row_start;
    metadata.mem.dma_channel_count = resp.info.mem_dma_channels;
    metadata.mem.lock_count = resp.info.mem_locks;
    metadata.mem.event_reg_count = resp.info.mem_events;
    metadata.shim.row_count = resp.info.shim_rows;
    metadata.shim.row_start = resp.info.shim_row_start;
    metadata.shim.dma_channel_count = resp.info.shim_dma_channels;
    metadata.shim.lock_count = resp.info.shim_locks;
    metadata.shim.event_reg_count = resp.info.shim_events;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn aie4_attach_work_buffer(ndev: *mut amdxdna_dev_hdl) -> c_int {
    int aie4_attach_work_buffer(struct amdxdna_dev_hdl *ndev)
    {
    DECLARE_AIE_MSG(aie4_msg_attach_work_buffer, AIE4_MSG_OP_ATTACH_WORK_BUFFER);
    struct amdxdna_dev *xdna = ndev.aie.xdna;
    int ret;
    req.buff_addr = ndev.work_buf_addr;
    req.buff_size = AIE4_WORK_BUFFER_MIN_SIZE;
    ret = aie_send_mgmt_msg_wait(&ndev.aie, &msg);
    if (ret)
    XDNA_ERR(xdna, "Failed to attach work buffer, ret %d", ret);
    else
    XDNA_DBG(xdna, "Attached work buffer");
    return ret;
    }
