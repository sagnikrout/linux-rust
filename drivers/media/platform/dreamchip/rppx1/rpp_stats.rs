//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/dreamchip/rppx1/rpp_stats.c
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
// Copyright (C) 2026 Renesas Electronics Corp.
// Copyright (C) 2026 Ideas on Board Oy
// Copyright (C) 2026 Ragnatech AB
//

    [RPPX1_STATS_BLOCK_TYPE_ ## type] = { \
    .size = sizeof(struct rppx1_ ## block ## _stats), \
    }
    static const struct v4l2_isp_stats_block_type_info
    rppx1_stats_blocks_info[] = {
    RPPX1_STATS_BLOCK_INFO(HIST_POST, hist),
    RPPX1_STATS_BLOCK_INFO(EXM_PRE1, exm),
    RPPX1_STATS_BLOCK_INFO(WBMEAS_POST, wbmeas),
    };

    ((union rppx1_stats_block *)					\
    v4l2_isp_stats_init_block((rpp).dev, (buf),			\
    rppx1_stats_blocks_info,		\
    ARRAY_SIZE(rppx1_stats_blocks_info),	\
    (type), RPPX1_STATS_MAX_SIZE))	\
#[no_mangle]
pub unsafe extern "C" fn rppx1_stats_fill_isr(rpp: *mut rppx1, isc: u32, buf: *mut c_void) {
    void rppx1_stats_fill_isr(struct rppx1 *rpp, u32 isc, void *buf)
    {
    struct v4l2_isp_buffer *stats = buf;
    union rppx1_stats_block *block;
    v4l2_isp_stats_init_buffer(stats, V4L2_ISP_VERSION_V1);
    if (isc & RPPX1_IRQ_ID_POST_HIST_MEAS) {
    block = rppx1_init_stats_block(rpp, stats,
    RPPX1_STATS_BLOCK_TYPE_HIST_POST);
    if (IS_ERR(block))
    return;
    rpp_module_call(&rpp.post.hist, fill_stats, block);
    }
    if (isc & RPPX1_IRQ_ID_PRE1_EXM) {
    block = rppx1_init_stats_block(rpp, stats,
    RPPX1_STATS_BLOCK_TYPE_EXM_PRE1);
    if (IS_ERR(block))
    return;
    rpp_module_call(&rpp.pre1.exm, fill_stats, block);
    }
    if (isc & RPPX1_IRQ_ID_POST_AWB_MEAS) {
    block = rppx1_init_stats_block(rpp, stats,
    RPPX1_STATS_BLOCK_TYPE_WBMEAS_POST);
    if (IS_ERR(block))
    return;
    rpp_module_call(&rpp.post.wbmeas, fill_stats, block);
    }
    }
    EXPORT_SYMBOL_GPL(rppx1_stats_fill_isr);
