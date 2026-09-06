//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/mdp3/mtk-mdp3-comp.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2022 MediaTek Inc.
// Author: Ping-Hsun Wu <ping-hsun.wu@mediatek.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_mdp_comp_id {
    MDP_COMP_NONE = -1,	/* Invalid engine */

// ISP
    MDP_COMP_WPEI = 0,
    MDP_COMP_WPEO,		/* 1 */
    MDP_COMP_WPEI2,		/* 2 */
    MDP_COMP_WPEO2,		/* 3 */
    MDP_COMP_ISP_IMGI,	/* 4 */
    MDP_COMP_ISP_IMGO,	/* 5 */
    MDP_COMP_ISP_IMG2O,	/* 6 */

// IPU
    MDP_COMP_IPUI,		/* 7 */
    MDP_COMP_IPUO,		/* 8 */

// MDP
    MDP_COMP_CAMIN,		/* 9 */
    MDP_COMP_CAMIN2,	/* 10 */
    MDP_COMP_RDMA0,		/* 11 */
    MDP_COMP_RDMA1,		/* 12 */
    MDP_COMP_RDMA2,		/* 13 */
    MDP_COMP_RDMA3,		/* 14 */
    MDP_COMP_AAL0,		/* 15 */
    MDP_COMP_AAL1,		/* 16 */
    MDP_COMP_AAL2,		/* 17 */
    MDP_COMP_AAL3,		/* 18 */
    MDP_COMP_CCORR0,	/* 19 */
    MDP_COMP_RSZ0,		/* 20 */
    MDP_COMP_RSZ1,		/* 21 */
    MDP_COMP_RSZ2,		/* 22 */
    MDP_COMP_RSZ3,		/* 23 */
    MDP_COMP_TDSHP0,	/* 24 */
    MDP_COMP_TDSHP1,	/* 25 */
    MDP_COMP_TDSHP2,	/* 26 */
    MDP_COMP_TDSHP3,	/* 27 */
    MDP_COMP_COLOR0,	/* 28 */
    MDP_COMP_COLOR1,	/* 29 */
    MDP_COMP_COLOR2,	/* 30 */
    MDP_COMP_COLOR3,	/* 31 */
    MDP_COMP_PATH0_SOUT,	/* 32 */
    MDP_COMP_PATH1_SOUT,	/* 33 */
    MDP_COMP_WROT0,		/* 34 */
    MDP_COMP_WROT1,		/* 35 */
    MDP_COMP_WROT2,		/* 36 */
    MDP_COMP_WROT3,		/* 37 */
    MDP_COMP_WDMA,		/* 38 */
    MDP_COMP_SPLIT,		/* 39 */
    MDP_COMP_SPLIT2,	/* 40 */
    MDP_COMP_STITCH,	/* 41 */
    MDP_COMP_FG0,		/* 42 */
    MDP_COMP_FG1,		/* 43 */
    MDP_COMP_FG2,		/* 44 */
    MDP_COMP_FG3,		/* 45 */
    MDP_COMP_TO_SVPP2MOUT,	/* 46 */
    MDP_COMP_TO_SVPP3MOUT,	/* 47 */
    MDP_COMP_TO_WARP0MOUT,	/* 48 */
    MDP_COMP_TO_WARP1MOUT,	/* 49 */
    MDP_COMP_VPP0_SOUT,	/* 50 */
    MDP_COMP_VPP1_SOUT,	/* 51 */
    MDP_COMP_PQ0_SOUT,	/* 52 */
    MDP_COMP_PQ1_SOUT,	/* 53 */
    MDP_COMP_HDR0,		/* 54 */
    MDP_COMP_HDR1,		/* 55 */
    MDP_COMP_HDR2,		/* 56 */
    MDP_COMP_HDR3,		/* 57 */
    MDP_COMP_OVL0,		/* 58 */
    MDP_COMP_OVL1,		/* 59 */
    MDP_COMP_PAD0,		/* 60 */
    MDP_COMP_PAD1,		/* 61 */
    MDP_COMP_PAD2,		/* 62 */
    MDP_COMP_PAD3,		/* 63 */
    MDP_COMP_TCC0,		/* 64 */
    MDP_COMP_TCC1,		/* 65 */
    MDP_COMP_MERGE2,	/* 66 */
    MDP_COMP_MERGE3,	/* 67 */
    MDP_COMP_VDO0DL0,	/* 68 */
    MDP_COMP_VDO1DL0,	/* 69 */
    MDP_COMP_VDO0DL1,	/* 70 */
    MDP_COMP_VDO1DL1,	/* 71 */

    MDP_MAX_COMP_COUNT	/* ALWAYS keep at the end */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mdp_comp_type {
    MDP_COMP_TYPE_INVALID = 0,

    MDP_COMP_TYPE_RDMA,
    MDP_COMP_TYPE_RSZ,
    MDP_COMP_TYPE_WROT,
    MDP_COMP_TYPE_WDMA,
    MDP_COMP_TYPE_PATH,

    MDP_COMP_TYPE_TDSHP,
    MDP_COMP_TYPE_COLOR,
    MDP_COMP_TYPE_DRE,
    MDP_COMP_TYPE_CCORR,
    MDP_COMP_TYPE_AAL,
    MDP_COMP_TYPE_TCC,
    MDP_COMP_TYPE_HDR,
    MDP_COMP_TYPE_SPLIT,
    MDP_COMP_TYPE_STITCH,
    MDP_COMP_TYPE_FG,
    MDP_COMP_TYPE_OVL,
    MDP_COMP_TYPE_PAD,
    MDP_COMP_TYPE_MERGE,

    MDP_COMP_TYPE_IMGI,
    MDP_COMP_TYPE_WPEI,
    MDP_COMP_TYPE_EXTO,	/* External path */
    MDP_COMP_TYPE_DL_PATH,	/* Direct-link path */
    MDP_COMP_TYPE_DUMMY,

    MDP_COMP_TYPE_COUNT	/* ALWAYS keep at the end */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_comp_match {
    pub type: mdp_comp_type,
    pub alias_id: u32,
    pub inner_id: i32,
    pub subsys_id: i32,
}

// Used to describe the item order in MDP property
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_comp_info {
    pub clk_num: u32,
    pub clk_ofst: u32,
    pub dts_reg_ofst: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_comp_blend {
    pub b_id: mtk_mdp_comp_id,
    pub aid_mod: bool,
    pub aid_clk: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_comp_data {
    pub match: mdp_comp_match,
    pub info: mdp_comp_info,
    pub blend: mdp_comp_blend,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_comp {
    pub mdp_dev: *mut mdp_dev,
    pub regs: *mut void __iomem,
    pub reg_base: phys_addr_t,
    pub subsys_id: u8,
    pub clk_num: u8,
    pub clks: *mut clk,
    pub comp_dev: *mut device,
    pub type: mdp_comp_type,
    pub public_id: mtk_mdp_comp_id,
    pub inner_id: i32,
    pub alias_id: u32,
    pub gce_event: [i32; MDP_GCE_EVENT_MAX],
    pub ops: *const mdp_comp_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_comp_ctx {
    pub comp: *mut mdp_comp,
    pub param: *const img_compparam,
    pub input: *const img_input,
    pub outputs: [*const img_output; IMG_MAX_HW_OUTPUTS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_comp_ops {
    pub ctx): *const *const s64 (get_comp_flag)(struct mdp_comp_ctx,
    pub cmd): *mut *mut *mut int (init_comp)(struct mdp_comp_ctx ctx, struct mdp_cmdq_cmd,
    pub compose): *const v4l2_rect,
    pub index): *mut *mut mdp_cmdq_cmd cmd, u32,
    pub cmd): *mut mdp_cmdq_cmd,
    pub index): *mut *mut mdp_cmdq_cmd cmd, u32,
    pub cmd): *mut *mut *mut int (post_process)(struct mdp_comp_ctx ctx, struct mdp_cmdq_cmd,
}

extern "C" {
    pub fn mdp_comp_config(mdp: *mut mdp_dev) -> c_int;
}
extern "C" {
    pub fn mdp_comp_destroy(mdp: *mut mdp_dev);
}
extern "C" {
    pub fn mdp_comp_clock_on(dev: *mut device, comp: *mut mdp_comp) -> c_int;
}
extern "C" {
    pub fn mdp_comp_clock_off(dev: *mut device, comp: *mut mdp_comp);
}
extern "C" {
    pub fn mdp_comp_clocks_on(dev: *mut device, comps: *mut mdp_comp, num: c_int) -> c_int;
}
extern "C" {
    pub fn mdp_comp_clocks_off(dev: *mut device, comps: *mut mdp_comp, num: c_int);
}
