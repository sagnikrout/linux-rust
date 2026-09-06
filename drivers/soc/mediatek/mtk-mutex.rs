//! Automatically rewritten from C to Rust
//! Source: drivers/soc/mediatek/mtk-mutex.c
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
// Copyright (c) 2015 MediaTek Inc.
//

pub const MTK_MUTEX_MAX_HANDLES: c_int = 10;
pub const MT2701_MUTEX0_MOD0: c_uint = 0x2c;
pub const MT2701_MUTEX0_SOF0: c_uint = 0x30;
pub const MT2701_MUTEX0_MOD1: c_uint = 0x34;
pub const MT8183_MUTEX0_MOD0: c_uint = 0x30;
pub const MT8183_MUTEX0_MOD1: c_uint = 0x34;
pub const MT8183_MUTEX0_SOF0: c_uint = 0x2c;

//
// Some SoCs may have multiple MUTEX_MOD registers as more than 32 mods
// are present, hence requiring multiple 32-bits registers.
//
// The mutex_table_mod fully represents that by defining the number of
// the mod sequentially, later used as a bit number, which can be more
// than 0..31.
//
// In order to retain compatibility with older SoCs, we perform R/W on
// the single 32 bits registers, but this requires us to translate the
// mutex ID bit accordingly.
//

    const typeof(mutex) _mutex = (mutex); \
    u32 _offset = (id) < 32 ? \
    _mutex.data.mutex_mod_reg : \
    _mutex.data.mutex_mod1_reg; \
    _offset + 0x20 * (n); \
    })

pub const MT8186_MUTEX_MOD_DISP_OVL0: c_int = 0;
pub const MT8186_MUTEX_MOD_DISP_OVL0_2L: c_int = 1;
pub const MT8186_MUTEX_MOD_DISP_RDMA0: c_int = 2;
pub const MT8186_MUTEX_MOD_DISP_COLOR0: c_int = 4;
pub const MT8186_MUTEX_MOD_DISP_CCORR0: c_int = 5;
pub const MT8186_MUTEX_MOD_DISP_AAL0: c_int = 7;
pub const MT8186_MUTEX_MOD_DISP_GAMMA0: c_int = 8;
pub const MT8186_MUTEX_MOD_DISP_POSTMASK0: c_int = 9;
pub const MT8186_MUTEX_MOD_DISP_DITHER0: c_int = 10;
pub const MT8186_MUTEX_MOD_DISP_RDMA1: c_int = 17;
pub const MT8186_MUTEX_SOF_SINGLE_MODE: c_int = 0;
pub const MT8186_MUTEX_SOF_DSI0: c_int = 1;
pub const MT8186_MUTEX_SOF_DPI0: c_int = 2;

pub const MT8167_MUTEX_MOD_DISP_PWM: c_int = 1;
pub const MT8167_MUTEX_MOD_DISP_OVL0: c_int = 6;
pub const MT8167_MUTEX_MOD_DISP_OVL1: c_int = 7;
pub const MT8167_MUTEX_MOD_DISP_RDMA0: c_int = 8;
pub const MT8167_MUTEX_MOD_DISP_RDMA1: c_int = 9;
pub const MT8167_MUTEX_MOD_DISP_WDMA0: c_int = 10;
pub const MT8167_MUTEX_MOD_DISP_CCORR: c_int = 11;
pub const MT8167_MUTEX_MOD_DISP_COLOR: c_int = 12;
pub const MT8167_MUTEX_MOD_DISP_AAL: c_int = 13;
pub const MT8167_MUTEX_MOD_DISP_GAMMA: c_int = 14;
pub const MT8167_MUTEX_MOD_DISP_DITHER: c_int = 15;
pub const MT8167_MUTEX_MOD_DISP_UFOE: c_int = 16;
pub const MT8192_MUTEX_MOD_DISP_OVL0: c_int = 0;
pub const MT8192_MUTEX_MOD_DISP_OVL0_2L: c_int = 1;
pub const MT8192_MUTEX_MOD_DISP_RDMA0: c_int = 2;
pub const MT8192_MUTEX_MOD_DISP_COLOR0: c_int = 4;
pub const MT8192_MUTEX_MOD_DISP_CCORR0: c_int = 5;
pub const MT8192_MUTEX_MOD_DISP_AAL0: c_int = 6;
pub const MT8192_MUTEX_MOD_DISP_GAMMA0: c_int = 7;
pub const MT8192_MUTEX_MOD_DISP_POSTMASK0: c_int = 8;
pub const MT8192_MUTEX_MOD_DISP_DITHER0: c_int = 9;
pub const MT8192_MUTEX_MOD_DISP_OVL2_2L: c_int = 16;
pub const MT8192_MUTEX_MOD_DISP_RDMA4: c_int = 17;
pub const MT8183_MUTEX_MOD_DISP_RDMA0: c_int = 0;
pub const MT8183_MUTEX_MOD_DISP_RDMA1: c_int = 1;
pub const MT8183_MUTEX_MOD_DISP_OVL0: c_int = 9;
pub const MT8183_MUTEX_MOD_DISP_OVL0_2L: c_int = 10;
pub const MT8183_MUTEX_MOD_DISP_OVL1_2L: c_int = 11;
pub const MT8183_MUTEX_MOD_DISP_WDMA0: c_int = 12;
pub const MT8183_MUTEX_MOD_DISP_COLOR0: c_int = 13;
pub const MT8183_MUTEX_MOD_DISP_CCORR0: c_int = 14;
pub const MT8183_MUTEX_MOD_DISP_AAL0: c_int = 15;
pub const MT8183_MUTEX_MOD_DISP_GAMMA0: c_int = 16;
pub const MT8183_MUTEX_MOD_DISP_DITHER0: c_int = 17;
pub const MT8183_MUTEX_MOD_MDP_RDMA0: c_int = 2;
pub const MT8183_MUTEX_MOD_MDP_RSZ0: c_int = 4;
pub const MT8183_MUTEX_MOD_MDP_RSZ1: c_int = 5;
pub const MT8183_MUTEX_MOD_MDP_TDSHP0: c_int = 6;
pub const MT8183_MUTEX_MOD_MDP_WROT0: c_int = 7;
pub const MT8183_MUTEX_MOD_MDP_WDMA: c_int = 8;
pub const MT8183_MUTEX_MOD_MDP_AAL0: c_int = 23;
pub const MT8183_MUTEX_MOD_MDP_CCORR0: c_int = 24;
pub const MT8186_MUTEX_MOD_MDP_RDMA0: c_int = 0;
pub const MT8186_MUTEX_MOD_MDP_AAL0: c_int = 2;
pub const MT8186_MUTEX_MOD_MDP_HDR0: c_int = 4;
pub const MT8186_MUTEX_MOD_MDP_RSZ0: c_int = 5;
pub const MT8186_MUTEX_MOD_MDP_RSZ1: c_int = 6;
pub const MT8186_MUTEX_MOD_MDP_WROT0: c_int = 7;
pub const MT8186_MUTEX_MOD_MDP_TDSHP0: c_int = 9;
pub const MT8186_MUTEX_MOD_MDP_COLOR0: c_int = 14;
pub const MT8173_MUTEX_MOD_DISP_OVL0: c_int = 11;
pub const MT8173_MUTEX_MOD_DISP_OVL1: c_int = 12;
pub const MT8173_MUTEX_MOD_DISP_RDMA0: c_int = 13;
pub const MT8173_MUTEX_MOD_DISP_RDMA1: c_int = 14;
pub const MT8173_MUTEX_MOD_DISP_RDMA2: c_int = 15;
pub const MT8173_MUTEX_MOD_DISP_WDMA0: c_int = 16;
pub const MT8173_MUTEX_MOD_DISP_WDMA1: c_int = 17;
pub const MT8173_MUTEX_MOD_DISP_COLOR0: c_int = 18;
pub const MT8173_MUTEX_MOD_DISP_COLOR1: c_int = 19;
pub const MT8173_MUTEX_MOD_DISP_AAL: c_int = 20;
pub const MT8173_MUTEX_MOD_DISP_GAMMA: c_int = 21;
pub const MT8173_MUTEX_MOD_DISP_UFOE: c_int = 22;
pub const MT8173_MUTEX_MOD_DISP_PWM0: c_int = 23;
pub const MT8173_MUTEX_MOD_DISP_PWM1: c_int = 24;
pub const MT8173_MUTEX_MOD_DISP_OD: c_int = 25;
pub const MT8188_MUTEX_MOD_DISP_OVL0: c_int = 0;
pub const MT8188_MUTEX_MOD_DISP_WDMA0: c_int = 1;
pub const MT8188_MUTEX_MOD_DISP_RDMA0: c_int = 2;
pub const MT8188_MUTEX_MOD_DISP_COLOR0: c_int = 3;
pub const MT8188_MUTEX_MOD_DISP_CCORR0: c_int = 4;
pub const MT8188_MUTEX_MOD_DISP_AAL0: c_int = 5;
pub const MT8188_MUTEX_MOD_DISP_GAMMA0: c_int = 6;
pub const MT8188_MUTEX_MOD_DISP_DITHER0: c_int = 7;
pub const MT8188_MUTEX_MOD_DISP_DSI0: c_int = 8;
pub const MT8188_MUTEX_MOD_DISP_DSC_WRAP0_CORE0: c_int = 9;
pub const MT8188_MUTEX_MOD_DISP_VPP_MERGE: c_int = 20;
pub const MT8188_MUTEX_MOD_DISP_DP_INTF0: c_int = 21;
pub const MT8188_MUTEX_MOD_DISP_POSTMASK0: c_int = 24;
pub const MT8188_MUTEX_MOD2_DISP_PWM0: c_int = 33;
pub const MT8188_MUTEX_MOD_DISP1_MDP_RDMA0: c_int = 0;
pub const MT8188_MUTEX_MOD_DISP1_MDP_RDMA1: c_int = 1;
pub const MT8188_MUTEX_MOD_DISP1_MDP_RDMA2: c_int = 2;
pub const MT8188_MUTEX_MOD_DISP1_MDP_RDMA3: c_int = 3;
pub const MT8188_MUTEX_MOD_DISP1_MDP_RDMA4: c_int = 4;
pub const MT8188_MUTEX_MOD_DISP1_MDP_RDMA5: c_int = 5;
pub const MT8188_MUTEX_MOD_DISP1_MDP_RDMA6: c_int = 6;
pub const MT8188_MUTEX_MOD_DISP1_MDP_RDMA7: c_int = 7;
pub const MT8188_MUTEX_MOD_DISP1_PADDING0: c_int = 8;
pub const MT8188_MUTEX_MOD_DISP1_PADDING1: c_int = 9;
pub const MT8188_MUTEX_MOD_DISP1_PADDING2: c_int = 10;
pub const MT8188_MUTEX_MOD_DISP1_PADDING3: c_int = 11;
pub const MT8188_MUTEX_MOD_DISP1_PADDING4: c_int = 12;
pub const MT8188_MUTEX_MOD_DISP1_PADDING5: c_int = 13;
pub const MT8188_MUTEX_MOD_DISP1_PADDING6: c_int = 14;
pub const MT8188_MUTEX_MOD_DISP1_PADDING7: c_int = 15;
pub const MT8188_MUTEX_MOD_DISP1_VPP_MERGE0: c_int = 20;
pub const MT8188_MUTEX_MOD_DISP1_VPP_MERGE1: c_int = 21;
pub const MT8188_MUTEX_MOD_DISP1_VPP_MERGE2: c_int = 22;
pub const MT8188_MUTEX_MOD_DISP1_VPP_MERGE3: c_int = 23;
pub const MT8188_MUTEX_MOD_DISP1_VPP_MERGE4: c_int = 24;
pub const MT8188_MUTEX_MOD_DISP1_DISP_MIXER: c_int = 30;
pub const MT8188_MUTEX_MOD_DISP1_DPI1: c_int = 38;
pub const MT8188_MUTEX_MOD_DISP1_DP_INTF1: c_int = 39;
pub const MT8195_MUTEX_MOD_DISP_OVL0: c_int = 0;
pub const MT8195_MUTEX_MOD_DISP_WDMA0: c_int = 1;
pub const MT8195_MUTEX_MOD_DISP_RDMA0: c_int = 2;
pub const MT8195_MUTEX_MOD_DISP_COLOR0: c_int = 3;
pub const MT8195_MUTEX_MOD_DISP_CCORR0: c_int = 4;
pub const MT8195_MUTEX_MOD_DISP_AAL0: c_int = 5;
pub const MT8195_MUTEX_MOD_DISP_GAMMA0: c_int = 6;
pub const MT8195_MUTEX_MOD_DISP_DITHER0: c_int = 7;
pub const MT8195_MUTEX_MOD_DISP_DSI0: c_int = 8;
pub const MT8195_MUTEX_MOD_DISP_DSC_WRAP0_CORE0: c_int = 9;
pub const MT8195_MUTEX_MOD_DISP_VPP_MERGE: c_int = 20;
pub const MT8195_MUTEX_MOD_DISP_DP_INTF0: c_int = 21;
pub const MT8195_MUTEX_MOD_DISP_PWM0: c_int = 27;
pub const MT8195_MUTEX_MOD_DISP1_MDP_RDMA0: c_int = 0;
pub const MT8195_MUTEX_MOD_DISP1_MDP_RDMA1: c_int = 1;
pub const MT8195_MUTEX_MOD_DISP1_MDP_RDMA2: c_int = 2;
pub const MT8195_MUTEX_MOD_DISP1_MDP_RDMA3: c_int = 3;
pub const MT8195_MUTEX_MOD_DISP1_MDP_RDMA4: c_int = 4;
pub const MT8195_MUTEX_MOD_DISP1_MDP_RDMA5: c_int = 5;
pub const MT8195_MUTEX_MOD_DISP1_MDP_RDMA6: c_int = 6;
pub const MT8195_MUTEX_MOD_DISP1_MDP_RDMA7: c_int = 7;
pub const MT8195_MUTEX_MOD_DISP1_VPP_MERGE0: c_int = 8;
pub const MT8195_MUTEX_MOD_DISP1_VPP_MERGE1: c_int = 9;
pub const MT8195_MUTEX_MOD_DISP1_VPP_MERGE2: c_int = 10;
pub const MT8195_MUTEX_MOD_DISP1_VPP_MERGE3: c_int = 11;
pub const MT8195_MUTEX_MOD_DISP1_VPP_MERGE4: c_int = 12;
pub const MT8195_MUTEX_MOD_DISP1_DISP_MIXER: c_int = 18;
pub const MT8195_MUTEX_MOD_DISP1_DPI0: c_int = 25;
pub const MT8195_MUTEX_MOD_DISP1_DPI1: c_int = 26;
pub const MT8195_MUTEX_MOD_DISP1_DP_INTF0: c_int = 27;
// VPPSYS0
pub const MT8195_MUTEX_MOD_MDP_RDMA0: c_int = 0;
pub const MT8195_MUTEX_MOD_MDP_FG0: c_int = 1;
pub const MT8195_MUTEX_MOD_MDP_STITCH0: c_int = 2;
pub const MT8195_MUTEX_MOD_MDP_HDR0: c_int = 3;
pub const MT8195_MUTEX_MOD_MDP_AAL0: c_int = 4;
pub const MT8195_MUTEX_MOD_MDP_RSZ0: c_int = 5;
pub const MT8195_MUTEX_MOD_MDP_TDSHP0: c_int = 6;
pub const MT8195_MUTEX_MOD_MDP_COLOR0: c_int = 7;
pub const MT8195_MUTEX_MOD_MDP_OVL0: c_int = 8;
pub const MT8195_MUTEX_MOD_MDP_PAD0: c_int = 9;
pub const MT8195_MUTEX_MOD_MDP_TCC0: c_int = 10;
pub const MT8195_MUTEX_MOD_MDP_WROT0: c_int = 11;
// VPPSYS1
pub const MT8195_MUTEX_MOD_MDP_TCC1: c_int = 3;
pub const MT8195_MUTEX_MOD_MDP_RDMA1: c_int = 4;
pub const MT8195_MUTEX_MOD_MDP_RDMA2: c_int = 5;
pub const MT8195_MUTEX_MOD_MDP_RDMA3: c_int = 6;
pub const MT8195_MUTEX_MOD_MDP_FG1: c_int = 7;
pub const MT8195_MUTEX_MOD_MDP_FG2: c_int = 8;
pub const MT8195_MUTEX_MOD_MDP_FG3: c_int = 9;
pub const MT8195_MUTEX_MOD_MDP_HDR1: c_int = 10;
pub const MT8195_MUTEX_MOD_MDP_HDR2: c_int = 11;
pub const MT8195_MUTEX_MOD_MDP_HDR3: c_int = 12;
pub const MT8195_MUTEX_MOD_MDP_AAL1: c_int = 13;
pub const MT8195_MUTEX_MOD_MDP_AAL2: c_int = 14;
pub const MT8195_MUTEX_MOD_MDP_AAL3: c_int = 15;
pub const MT8195_MUTEX_MOD_MDP_RSZ1: c_int = 16;
pub const MT8195_MUTEX_MOD_MDP_RSZ2: c_int = 17;
pub const MT8195_MUTEX_MOD_MDP_RSZ3: c_int = 18;
pub const MT8195_MUTEX_MOD_MDP_TDSHP1: c_int = 19;
pub const MT8195_MUTEX_MOD_MDP_TDSHP2: c_int = 20;
pub const MT8195_MUTEX_MOD_MDP_TDSHP3: c_int = 21;
pub const MT8195_MUTEX_MOD_MDP_MERGE2: c_int = 22;
pub const MT8195_MUTEX_MOD_MDP_MERGE3: c_int = 23;
pub const MT8195_MUTEX_MOD_MDP_COLOR1: c_int = 24;
pub const MT8195_MUTEX_MOD_MDP_COLOR2: c_int = 25;
pub const MT8195_MUTEX_MOD_MDP_COLOR3: c_int = 26;
pub const MT8195_MUTEX_MOD_MDP_OVL1: c_int = 27;
pub const MT8195_MUTEX_MOD_MDP_PAD1: c_int = 28;
pub const MT8195_MUTEX_MOD_MDP_PAD2: c_int = 29;
pub const MT8195_MUTEX_MOD_MDP_PAD3: c_int = 30;
pub const MT8195_MUTEX_MOD_MDP_WROT1: c_int = 31;
pub const MT8195_MUTEX_MOD_MDP_WROT2: c_int = 32;
pub const MT8195_MUTEX_MOD_MDP_WROT3: c_int = 33;
pub const MT8365_MUTEX_MOD_DISP_OVL0: c_int = 7;
pub const MT8365_MUTEX_MOD_DISP_OVL0_2L: c_int = 8;
pub const MT8365_MUTEX_MOD_DISP_RDMA0: c_int = 9;
pub const MT8365_MUTEX_MOD_DISP_RDMA1: c_int = 10;
pub const MT8365_MUTEX_MOD_DISP_WDMA0: c_int = 11;
pub const MT8365_MUTEX_MOD_DISP_COLOR0: c_int = 12;
pub const MT8365_MUTEX_MOD_DISP_CCORR: c_int = 13;
pub const MT8365_MUTEX_MOD_DISP_AAL: c_int = 14;
pub const MT8365_MUTEX_MOD_DISP_GAMMA: c_int = 15;
pub const MT8365_MUTEX_MOD_DISP_DITHER: c_int = 16;
pub const MT8365_MUTEX_MOD_DISP_DSI0: c_int = 17;
pub const MT8365_MUTEX_MOD_DISP_PWM0: c_int = 20;
pub const MT8365_MUTEX_MOD_DISP_DPI0: c_int = 22;
pub const MT2712_MUTEX_MOD_DISP_PWM2: c_int = 10;
pub const MT2712_MUTEX_MOD_DISP_OVL0: c_int = 11;
pub const MT2712_MUTEX_MOD_DISP_OVL1: c_int = 12;
pub const MT2712_MUTEX_MOD_DISP_RDMA0: c_int = 13;
pub const MT2712_MUTEX_MOD_DISP_RDMA1: c_int = 14;
pub const MT2712_MUTEX_MOD_DISP_RDMA2: c_int = 15;
pub const MT2712_MUTEX_MOD_DISP_WDMA0: c_int = 16;
pub const MT2712_MUTEX_MOD_DISP_WDMA1: c_int = 17;
pub const MT2712_MUTEX_MOD_DISP_COLOR0: c_int = 18;
pub const MT2712_MUTEX_MOD_DISP_COLOR1: c_int = 19;
pub const MT2712_MUTEX_MOD_DISP_AAL0: c_int = 20;
pub const MT2712_MUTEX_MOD_DISP_UFOE: c_int = 22;
pub const MT2712_MUTEX_MOD_DISP_PWM0: c_int = 23;
pub const MT2712_MUTEX_MOD_DISP_PWM1: c_int = 24;
pub const MT2712_MUTEX_MOD_DISP_OD0: c_int = 25;
pub const MT2712_MUTEX_MOD2_DISP_AAL1: c_int = 33;
pub const MT2712_MUTEX_MOD2_DISP_OD1: c_int = 34;
pub const MT2701_MUTEX_MOD_DISP_OVL: c_int = 3;
pub const MT2701_MUTEX_MOD_DISP_WDMA: c_int = 6;
pub const MT2701_MUTEX_MOD_DISP_COLOR: c_int = 7;
pub const MT2701_MUTEX_MOD_DISP_BLS: c_int = 9;
pub const MT2701_MUTEX_MOD_DISP_RDMA0: c_int = 10;
pub const MT2701_MUTEX_MOD_DISP_RDMA1: c_int = 12;
pub const MT2712_MUTEX_SOF_SINGLE_MODE: c_int = 0;
pub const MT2712_MUTEX_SOF_DSI0: c_int = 1;
pub const MT2712_MUTEX_SOF_DSI1: c_int = 2;
pub const MT2712_MUTEX_SOF_DPI0: c_int = 3;
pub const MT2712_MUTEX_SOF_DPI1: c_int = 4;
pub const MT2712_MUTEX_SOF_DSI2: c_int = 5;
pub const MT2712_MUTEX_SOF_DSI3: c_int = 6;
pub const MT8167_MUTEX_SOF_DPI0: c_int = 2;
pub const MT8167_MUTEX_SOF_DPI1: c_int = 3;
pub const MT8183_MUTEX_SOF_DSI0: c_int = 1;
pub const MT8183_MUTEX_SOF_DPI0: c_int = 2;
pub const MT8188_MUTEX_SOF_DSI0: c_int = 1;
pub const MT8188_MUTEX_SOF_DP_INTF0: c_int = 3;
pub const MT8188_MUTEX_SOF_DP_INTF1: c_int = 4;
pub const MT8188_MUTEX_SOF_DPI1: c_int = 5;
pub const MT8195_MUTEX_SOF_DSI0: c_int = 1;
pub const MT8195_MUTEX_SOF_DSI1: c_int = 2;
pub const MT8195_MUTEX_SOF_DP_INTF0: c_int = 3;
pub const MT8195_MUTEX_SOF_DP_INTF1: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_mutex {
    pub id: u8,
    pub claimed: bool,
}

    enum mtk_mutex_sof_id {
    MUTEX_SOF_SINGLE_MODE,
    MUTEX_SOF_DSI0,
    MUTEX_SOF_DSI1,
    MUTEX_SOF_DPI0,
    MUTEX_SOF_DPI1,
    MUTEX_SOF_DSI2,
    MUTEX_SOF_DSI3,
    MUTEX_SOF_DP_INTF0,
    MUTEX_SOF_DP_INTF1,
    DDP_MUTEX_SOF_MAX,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_mutex_data {
    pub mutex_mod: *const u8,
    pub mutex_table_mod: *const u8,
    pub mutex_sof: *const u16,
    pub mutex_mod_reg: u16,
    pub mutex_mod1_reg: u16,
    pub mutex_sof_reg: u16,
    pub no_clk: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_mutex_ctx {
    pub dev: *mut device,
    pub clk: *mut clk,
    pub regs: *mut void __iomem,
    pub mutex: [mtk_mutex; MTK_MUTEX_MAX_HANDLES],
    pub data: *const mtk_mutex_data,
    pub addr: phys_addr_t,
    pub cmdq_reg: cmdq_client_reg,
}

    static const u8 mt2701_mutex_mod[DDP_COMPONENT_ID_MAX] = {
    [DDP_COMPONENT_BLS] = MT2701_MUTEX_MOD_DISP_BLS,
    [DDP_COMPONENT_COLOR0] = MT2701_MUTEX_MOD_DISP_COLOR,
    [DDP_COMPONENT_OVL0] = MT2701_MUTEX_MOD_DISP_OVL,
    [DDP_COMPONENT_RDMA0] = MT2701_MUTEX_MOD_DISP_RDMA0,
    [DDP_COMPONENT_RDMA1] = MT2701_MUTEX_MOD_DISP_RDMA1,
    [DDP_COMPONENT_WDMA0] = MT2701_MUTEX_MOD_DISP_WDMA,
    };
    static const u8 mt2712_mutex_mod[DDP_COMPONENT_ID_MAX] = {
    [DDP_COMPONENT_AAL0] = MT2712_MUTEX_MOD_DISP_AAL0,
    [DDP_COMPONENT_AAL1] = MT2712_MUTEX_MOD2_DISP_AAL1,
    [DDP_COMPONENT_COLOR0] = MT2712_MUTEX_MOD_DISP_COLOR0,
    [DDP_COMPONENT_COLOR1] = MT2712_MUTEX_MOD_DISP_COLOR1,
    [DDP_COMPONENT_OD0] = MT2712_MUTEX_MOD_DISP_OD0,
    [DDP_COMPONENT_OD1] = MT2712_MUTEX_MOD2_DISP_OD1,
    [DDP_COMPONENT_OVL0] = MT2712_MUTEX_MOD_DISP_OVL0,
    [DDP_COMPONENT_OVL1] = MT2712_MUTEX_MOD_DISP_OVL1,
    [DDP_COMPONENT_PWM0] = MT2712_MUTEX_MOD_DISP_PWM0,
    [DDP_COMPONENT_PWM1] = MT2712_MUTEX_MOD_DISP_PWM1,
    [DDP_COMPONENT_PWM2] = MT2712_MUTEX_MOD_DISP_PWM2,
    [DDP_COMPONENT_RDMA0] = MT2712_MUTEX_MOD_DISP_RDMA0,
    [DDP_COMPONENT_RDMA1] = MT2712_MUTEX_MOD_DISP_RDMA1,
    [DDP_COMPONENT_RDMA2] = MT2712_MUTEX_MOD_DISP_RDMA2,
    [DDP_COMPONENT_UFOE] = MT2712_MUTEX_MOD_DISP_UFOE,
    [DDP_COMPONENT_WDMA0] = MT2712_MUTEX_MOD_DISP_WDMA0,
    [DDP_COMPONENT_WDMA1] = MT2712_MUTEX_MOD_DISP_WDMA1,
    };
    static const u8 mt8167_mutex_mod[DDP_COMPONENT_ID_MAX] = {
    [DDP_COMPONENT_AAL0] = MT8167_MUTEX_MOD_DISP_AAL,
    [DDP_COMPONENT_CCORR] = MT8167_MUTEX_MOD_DISP_CCORR,
    [DDP_COMPONENT_COLOR0] = MT8167_MUTEX_MOD_DISP_COLOR,
    [DDP_COMPONENT_DITHER0] = MT8167_MUTEX_MOD_DISP_DITHER,
    [DDP_COMPONENT_GAMMA] = MT8167_MUTEX_MOD_DISP_GAMMA,
    [DDP_COMPONENT_OVL0] = MT8167_MUTEX_MOD_DISP_OVL0,
    [DDP_COMPONENT_OVL1] = MT8167_MUTEX_MOD_DISP_OVL1,
    [DDP_COMPONENT_PWM0] = MT8167_MUTEX_MOD_DISP_PWM,
    [DDP_COMPONENT_RDMA0] = MT8167_MUTEX_MOD_DISP_RDMA0,
    [DDP_COMPONENT_RDMA1] = MT8167_MUTEX_MOD_DISP_RDMA1,
    [DDP_COMPONENT_UFOE] = MT8167_MUTEX_MOD_DISP_UFOE,
    [DDP_COMPONENT_WDMA0] = MT8167_MUTEX_MOD_DISP_WDMA0,
    };
    static const u8 mt8173_mutex_mod[DDP_COMPONENT_ID_MAX] = {
    [DDP_COMPONENT_AAL0] = MT8173_MUTEX_MOD_DISP_AAL,
    [DDP_COMPONENT_COLOR0] = MT8173_MUTEX_MOD_DISP_COLOR0,
    [DDP_COMPONENT_COLOR1] = MT8173_MUTEX_MOD_DISP_COLOR1,
    [DDP_COMPONENT_GAMMA] = MT8173_MUTEX_MOD_DISP_GAMMA,
    [DDP_COMPONENT_OD0] = MT8173_MUTEX_MOD_DISP_OD,
    [DDP_COMPONENT_OVL0] = MT8173_MUTEX_MOD_DISP_OVL0,
    [DDP_COMPONENT_OVL1] = MT8173_MUTEX_MOD_DISP_OVL1,
    [DDP_COMPONENT_PWM0] = MT8173_MUTEX_MOD_DISP_PWM0,
    [DDP_COMPONENT_PWM1] = MT8173_MUTEX_MOD_DISP_PWM1,
    [DDP_COMPONENT_RDMA0] = MT8173_MUTEX_MOD_DISP_RDMA0,
    [DDP_COMPONENT_RDMA1] = MT8173_MUTEX_MOD_DISP_RDMA1,
    [DDP_COMPONENT_RDMA2] = MT8173_MUTEX_MOD_DISP_RDMA2,
    [DDP_COMPONENT_UFOE] = MT8173_MUTEX_MOD_DISP_UFOE,
    [DDP_COMPONENT_WDMA0] = MT8173_MUTEX_MOD_DISP_WDMA0,
    [DDP_COMPONENT_WDMA1] = MT8173_MUTEX_MOD_DISP_WDMA1,
    };
    static const u8 mt8183_mutex_mod[DDP_COMPONENT_ID_MAX] = {
    [DDP_COMPONENT_AAL0] = MT8183_MUTEX_MOD_DISP_AAL0,
    [DDP_COMPONENT_CCORR] = MT8183_MUTEX_MOD_DISP_CCORR0,
    [DDP_COMPONENT_COLOR0] = MT8183_MUTEX_MOD_DISP_COLOR0,
    [DDP_COMPONENT_DITHER0] = MT8183_MUTEX_MOD_DISP_DITHER0,
    [DDP_COMPONENT_GAMMA] = MT8183_MUTEX_MOD_DISP_GAMMA0,
    [DDP_COMPONENT_OVL0] = MT8183_MUTEX_MOD_DISP_OVL0,
    [DDP_COMPONENT_OVL_2L0] = MT8183_MUTEX_MOD_DISP_OVL0_2L,
    [DDP_COMPONENT_OVL_2L1] = MT8183_MUTEX_MOD_DISP_OVL1_2L,
    [DDP_COMPONENT_RDMA0] = MT8183_MUTEX_MOD_DISP_RDMA0,
    [DDP_COMPONENT_RDMA1] = MT8183_MUTEX_MOD_DISP_RDMA1,
    [DDP_COMPONENT_WDMA0] = MT8183_MUTEX_MOD_DISP_WDMA0,
    };
    static const u8 mt8183_mutex_table_mod[MUTEX_MOD_IDX_MAX] = {
    [MUTEX_MOD_IDX_MDP_RDMA0] = MT8183_MUTEX_MOD_MDP_RDMA0,
    [MUTEX_MOD_IDX_MDP_RSZ0] = MT8183_MUTEX_MOD_MDP_RSZ0,
    [MUTEX_MOD_IDX_MDP_RSZ1] = MT8183_MUTEX_MOD_MDP_RSZ1,
    [MUTEX_MOD_IDX_MDP_TDSHP0] = MT8183_MUTEX_MOD_MDP_TDSHP0,
    [MUTEX_MOD_IDX_MDP_WROT0] = MT8183_MUTEX_MOD_MDP_WROT0,
    [MUTEX_MOD_IDX_MDP_WDMA] = MT8183_MUTEX_MOD_MDP_WDMA,
    [MUTEX_MOD_IDX_MDP_AAL0] = MT8183_MUTEX_MOD_MDP_AAL0,
    [MUTEX_MOD_IDX_MDP_CCORR0] = MT8183_MUTEX_MOD_MDP_CCORR0,
    };
    static const u8 mt8186_mutex_mod[DDP_COMPONENT_ID_MAX] = {
    [DDP_COMPONENT_AAL0] = MT8186_MUTEX_MOD_DISP_AAL0,
    [DDP_COMPONENT_CCORR] = MT8186_MUTEX_MOD_DISP_CCORR0,
    [DDP_COMPONENT_COLOR0] = MT8186_MUTEX_MOD_DISP_COLOR0,
    [DDP_COMPONENT_DITHER0] = MT8186_MUTEX_MOD_DISP_DITHER0,
    [DDP_COMPONENT_GAMMA] = MT8186_MUTEX_MOD_DISP_GAMMA0,
    [DDP_COMPONENT_OVL0] = MT8186_MUTEX_MOD_DISP_OVL0,
    [DDP_COMPONENT_OVL_2L0] = MT8186_MUTEX_MOD_DISP_OVL0_2L,
    [DDP_COMPONENT_POSTMASK0] = MT8186_MUTEX_MOD_DISP_POSTMASK0,
    [DDP_COMPONENT_RDMA0] = MT8186_MUTEX_MOD_DISP_RDMA0,
    [DDP_COMPONENT_RDMA1] = MT8186_MUTEX_MOD_DISP_RDMA1,
    };
    static const u8 mt8186_mdp_mutex_table_mod[MUTEX_MOD_IDX_MAX] = {
    [MUTEX_MOD_IDX_MDP_RDMA0] = MT8186_MUTEX_MOD_MDP_RDMA0,
    [MUTEX_MOD_IDX_MDP_RSZ0] = MT8186_MUTEX_MOD_MDP_RSZ0,
    [MUTEX_MOD_IDX_MDP_RSZ1] = MT8186_MUTEX_MOD_MDP_RSZ1,
    [MUTEX_MOD_IDX_MDP_TDSHP0] = MT8186_MUTEX_MOD_MDP_TDSHP0,
    [MUTEX_MOD_IDX_MDP_WROT0] = MT8186_MUTEX_MOD_MDP_WROT0,
    [MUTEX_MOD_IDX_MDP_HDR0] = MT8186_MUTEX_MOD_MDP_HDR0,
    [MUTEX_MOD_IDX_MDP_AAL0] = MT8186_MUTEX_MOD_MDP_AAL0,
    [MUTEX_MOD_IDX_MDP_COLOR0] = MT8186_MUTEX_MOD_MDP_COLOR0,
    };
    static const u8 mt8188_mutex_mod[DDP_COMPONENT_ID_MAX] = {
    [DDP_COMPONENT_OVL0] = MT8188_MUTEX_MOD_DISP_OVL0,
    [DDP_COMPONENT_WDMA0] = MT8188_MUTEX_MOD_DISP_WDMA0,
    [DDP_COMPONENT_RDMA0] = MT8188_MUTEX_MOD_DISP_RDMA0,
    [DDP_COMPONENT_COLOR0] = MT8188_MUTEX_MOD_DISP_COLOR0,
    [DDP_COMPONENT_CCORR] = MT8188_MUTEX_MOD_DISP_CCORR0,
    [DDP_COMPONENT_AAL0] = MT8188_MUTEX_MOD_DISP_AAL0,
    [DDP_COMPONENT_GAMMA] = MT8188_MUTEX_MOD_DISP_GAMMA0,
    [DDP_COMPONENT_POSTMASK0] = MT8188_MUTEX_MOD_DISP_POSTMASK0,
    [DDP_COMPONENT_DITHER0] = MT8188_MUTEX_MOD_DISP_DITHER0,
    [DDP_COMPONENT_MERGE0] = MT8188_MUTEX_MOD_DISP_VPP_MERGE,
    [DDP_COMPONENT_DSC0] = MT8188_MUTEX_MOD_DISP_DSC_WRAP0_CORE0,
    [DDP_COMPONENT_DSI0] = MT8188_MUTEX_MOD_DISP_DSI0,
    [DDP_COMPONENT_PWM0] = MT8188_MUTEX_MOD2_DISP_PWM0,
    [DDP_COMPONENT_DP_INTF0] = MT8188_MUTEX_MOD_DISP_DP_INTF0,
    [DDP_COMPONENT_DP_INTF1] = MT8188_MUTEX_MOD_DISP1_DP_INTF1,
    [DDP_COMPONENT_DPI1] = MT8188_MUTEX_MOD_DISP1_DPI1,
    [DDP_COMPONENT_ETHDR_MIXER] = MT8188_MUTEX_MOD_DISP1_DISP_MIXER,
    [DDP_COMPONENT_MDP_RDMA0] = MT8188_MUTEX_MOD_DISP1_MDP_RDMA0,
    [DDP_COMPONENT_MDP_RDMA1] = MT8188_MUTEX_MOD_DISP1_MDP_RDMA1,
    [DDP_COMPONENT_MDP_RDMA2] = MT8188_MUTEX_MOD_DISP1_MDP_RDMA2,
    [DDP_COMPONENT_MDP_RDMA3] = MT8188_MUTEX_MOD_DISP1_MDP_RDMA3,
    [DDP_COMPONENT_MDP_RDMA4] = MT8188_MUTEX_MOD_DISP1_MDP_RDMA4,
    [DDP_COMPONENT_MDP_RDMA5] = MT8188_MUTEX_MOD_DISP1_MDP_RDMA5,
    [DDP_COMPONENT_MDP_RDMA6] = MT8188_MUTEX_MOD_DISP1_MDP_RDMA6,
    [DDP_COMPONENT_MDP_RDMA7] = MT8188_MUTEX_MOD_DISP1_MDP_RDMA7,
    [DDP_COMPONENT_PADDING0] = MT8188_MUTEX_MOD_DISP1_PADDING0,
    [DDP_COMPONENT_PADDING1] = MT8188_MUTEX_MOD_DISP1_PADDING1,
    [DDP_COMPONENT_PADDING2] = MT8188_MUTEX_MOD_DISP1_PADDING2,
    [DDP_COMPONENT_PADDING3] = MT8188_MUTEX_MOD_DISP1_PADDING3,
    [DDP_COMPONENT_PADDING4] = MT8188_MUTEX_MOD_DISP1_PADDING4,
    [DDP_COMPONENT_PADDING5] = MT8188_MUTEX_MOD_DISP1_PADDING5,
    [DDP_COMPONENT_PADDING6] = MT8188_MUTEX_MOD_DISP1_PADDING6,
    [DDP_COMPONENT_PADDING7] = MT8188_MUTEX_MOD_DISP1_PADDING7,
    [DDP_COMPONENT_MERGE1] = MT8188_MUTEX_MOD_DISP1_VPP_MERGE0,
    [DDP_COMPONENT_MERGE2] = MT8188_MUTEX_MOD_DISP1_VPP_MERGE1,
    [DDP_COMPONENT_MERGE3] = MT8188_MUTEX_MOD_DISP1_VPP_MERGE2,
    [DDP_COMPONENT_MERGE4] = MT8188_MUTEX_MOD_DISP1_VPP_MERGE3,
    [DDP_COMPONENT_MERGE5] = MT8188_MUTEX_MOD_DISP1_VPP_MERGE4,
    };
    static const u8 mt8188_mdp_mutex_table_mod[MUTEX_MOD_IDX_MAX] = {
    [MUTEX_MOD_IDX_MDP_RDMA0] = MT8195_MUTEX_MOD_MDP_RDMA0,
    [MUTEX_MOD_IDX_MDP_RDMA2] = MT8195_MUTEX_MOD_MDP_RDMA2,
    [MUTEX_MOD_IDX_MDP_RDMA3] = MT8195_MUTEX_MOD_MDP_RDMA3,
    [MUTEX_MOD_IDX_MDP_FG0] = MT8195_MUTEX_MOD_MDP_FG0,
    [MUTEX_MOD_IDX_MDP_FG2] = MT8195_MUTEX_MOD_MDP_FG2,
    [MUTEX_MOD_IDX_MDP_FG3] = MT8195_MUTEX_MOD_MDP_FG3,
    [MUTEX_MOD_IDX_MDP_HDR0] = MT8195_MUTEX_MOD_MDP_HDR0,
    [MUTEX_MOD_IDX_MDP_HDR2] = MT8195_MUTEX_MOD_MDP_HDR2,
    [MUTEX_MOD_IDX_MDP_HDR3] = MT8195_MUTEX_MOD_MDP_HDR3,
    [MUTEX_MOD_IDX_MDP_AAL0] = MT8195_MUTEX_MOD_MDP_AAL0,
    [MUTEX_MOD_IDX_MDP_AAL2] = MT8195_MUTEX_MOD_MDP_AAL2,
    [MUTEX_MOD_IDX_MDP_AAL3] = MT8195_MUTEX_MOD_MDP_AAL3,
    [MUTEX_MOD_IDX_MDP_RSZ0] = MT8195_MUTEX_MOD_MDP_RSZ0,
    [MUTEX_MOD_IDX_MDP_RSZ2] = MT8195_MUTEX_MOD_MDP_RSZ2,
    [MUTEX_MOD_IDX_MDP_RSZ3] = MT8195_MUTEX_MOD_MDP_RSZ3,
    [MUTEX_MOD_IDX_MDP_MERGE2] = MT8195_MUTEX_MOD_MDP_MERGE2,
    [MUTEX_MOD_IDX_MDP_MERGE3] = MT8195_MUTEX_MOD_MDP_MERGE3,
    [MUTEX_MOD_IDX_MDP_TDSHP0] = MT8195_MUTEX_MOD_MDP_TDSHP0,
    [MUTEX_MOD_IDX_MDP_TDSHP2] = MT8195_MUTEX_MOD_MDP_TDSHP2,
    [MUTEX_MOD_IDX_MDP_TDSHP3] = MT8195_MUTEX_MOD_MDP_TDSHP3,
    [MUTEX_MOD_IDX_MDP_COLOR0] = MT8195_MUTEX_MOD_MDP_COLOR0,
    [MUTEX_MOD_IDX_MDP_COLOR2] = MT8195_MUTEX_MOD_MDP_COLOR2,
    [MUTEX_MOD_IDX_MDP_COLOR3] = MT8195_MUTEX_MOD_MDP_COLOR3,
    [MUTEX_MOD_IDX_MDP_OVL0] = MT8195_MUTEX_MOD_MDP_OVL0,
    [MUTEX_MOD_IDX_MDP_PAD0] = MT8195_MUTEX_MOD_MDP_PAD0,
    [MUTEX_MOD_IDX_MDP_PAD2] = MT8195_MUTEX_MOD_MDP_PAD2,
    [MUTEX_MOD_IDX_MDP_PAD3] = MT8195_MUTEX_MOD_MDP_PAD3,
    [MUTEX_MOD_IDX_MDP_TCC0] = MT8195_MUTEX_MOD_MDP_TCC0,
    [MUTEX_MOD_IDX_MDP_WROT0] = MT8195_MUTEX_MOD_MDP_WROT0,
    [MUTEX_MOD_IDX_MDP_WROT2] = MT8195_MUTEX_MOD_MDP_WROT2,
    [MUTEX_MOD_IDX_MDP_WROT3] = MT8195_MUTEX_MOD_MDP_WROT3,
    };
    static const u8 mt8192_mutex_mod[DDP_COMPONENT_ID_MAX] = {
    [DDP_COMPONENT_AAL0] = MT8192_MUTEX_MOD_DISP_AAL0,
    [DDP_COMPONENT_CCORR] = MT8192_MUTEX_MOD_DISP_CCORR0,
    [DDP_COMPONENT_COLOR0] = MT8192_MUTEX_MOD_DISP_COLOR0,
    [DDP_COMPONENT_DITHER0] = MT8192_MUTEX_MOD_DISP_DITHER0,
    [DDP_COMPONENT_GAMMA] = MT8192_MUTEX_MOD_DISP_GAMMA0,
    [DDP_COMPONENT_POSTMASK0] = MT8192_MUTEX_MOD_DISP_POSTMASK0,
    [DDP_COMPONENT_OVL0] = MT8192_MUTEX_MOD_DISP_OVL0,
    [DDP_COMPONENT_OVL_2L0] = MT8192_MUTEX_MOD_DISP_OVL0_2L,
    [DDP_COMPONENT_OVL_2L2] = MT8192_MUTEX_MOD_DISP_OVL2_2L,
    [DDP_COMPONENT_RDMA0] = MT8192_MUTEX_MOD_DISP_RDMA0,
    [DDP_COMPONENT_RDMA4] = MT8192_MUTEX_MOD_DISP_RDMA4,
    };
    static const u8 mt8195_mutex_mod[DDP_COMPONENT_ID_MAX] = {
    [DDP_COMPONENT_OVL0] = MT8195_MUTEX_MOD_DISP_OVL0,
    [DDP_COMPONENT_WDMA0] = MT8195_MUTEX_MOD_DISP_WDMA0,
    [DDP_COMPONENT_RDMA0] = MT8195_MUTEX_MOD_DISP_RDMA0,
    [DDP_COMPONENT_COLOR0] = MT8195_MUTEX_MOD_DISP_COLOR0,
    [DDP_COMPONENT_CCORR] = MT8195_MUTEX_MOD_DISP_CCORR0,
    [DDP_COMPONENT_AAL0] = MT8195_MUTEX_MOD_DISP_AAL0,
    [DDP_COMPONENT_GAMMA] = MT8195_MUTEX_MOD_DISP_GAMMA0,
    [DDP_COMPONENT_DITHER0] = MT8195_MUTEX_MOD_DISP_DITHER0,
    [DDP_COMPONENT_MERGE0] = MT8195_MUTEX_MOD_DISP_VPP_MERGE,
    [DDP_COMPONENT_DSC0] = MT8195_MUTEX_MOD_DISP_DSC_WRAP0_CORE0,
    [DDP_COMPONENT_DSI0] = MT8195_MUTEX_MOD_DISP_DSI0,
    [DDP_COMPONENT_PWM0] = MT8195_MUTEX_MOD_DISP_PWM0,
    [DDP_COMPONENT_DP_INTF0] = MT8195_MUTEX_MOD_DISP_DP_INTF0,
    [DDP_COMPONENT_MDP_RDMA0] = MT8195_MUTEX_MOD_DISP1_MDP_RDMA0,
    [DDP_COMPONENT_MDP_RDMA1] = MT8195_MUTEX_MOD_DISP1_MDP_RDMA1,
    [DDP_COMPONENT_MDP_RDMA2] = MT8195_MUTEX_MOD_DISP1_MDP_RDMA2,
    [DDP_COMPONENT_MDP_RDMA3] = MT8195_MUTEX_MOD_DISP1_MDP_RDMA3,
    [DDP_COMPONENT_MDP_RDMA4] = MT8195_MUTEX_MOD_DISP1_MDP_RDMA4,
    [DDP_COMPONENT_MDP_RDMA5] = MT8195_MUTEX_MOD_DISP1_MDP_RDMA5,
    [DDP_COMPONENT_MDP_RDMA6] = MT8195_MUTEX_MOD_DISP1_MDP_RDMA6,
    [DDP_COMPONENT_MDP_RDMA7] = MT8195_MUTEX_MOD_DISP1_MDP_RDMA7,
    [DDP_COMPONENT_MERGE1] = MT8195_MUTEX_MOD_DISP1_VPP_MERGE0,
    [DDP_COMPONENT_MERGE2] = MT8195_MUTEX_MOD_DISP1_VPP_MERGE1,
    [DDP_COMPONENT_MERGE3] = MT8195_MUTEX_MOD_DISP1_VPP_MERGE2,
    [DDP_COMPONENT_MERGE4] = MT8195_MUTEX_MOD_DISP1_VPP_MERGE3,
    [DDP_COMPONENT_ETHDR_MIXER] = MT8195_MUTEX_MOD_DISP1_DISP_MIXER,
    [DDP_COMPONENT_MERGE5] = MT8195_MUTEX_MOD_DISP1_VPP_MERGE4,
    [DDP_COMPONENT_DP_INTF1] = MT8195_MUTEX_MOD_DISP1_DP_INTF0,
    };
    static const u8 mt8195_mutex_table_mod[MUTEX_MOD_IDX_MAX] = {
    [MUTEX_MOD_IDX_MDP_RDMA0] = MT8195_MUTEX_MOD_MDP_RDMA0,
    [MUTEX_MOD_IDX_MDP_RDMA1] = MT8195_MUTEX_MOD_MDP_RDMA1,
    [MUTEX_MOD_IDX_MDP_RDMA2] = MT8195_MUTEX_MOD_MDP_RDMA2,
    [MUTEX_MOD_IDX_MDP_RDMA3] = MT8195_MUTEX_MOD_MDP_RDMA3,
    [MUTEX_MOD_IDX_MDP_STITCH0] = MT8195_MUTEX_MOD_MDP_STITCH0,
    [MUTEX_MOD_IDX_MDP_FG0] = MT8195_MUTEX_MOD_MDP_FG0,
    [MUTEX_MOD_IDX_MDP_FG1] = MT8195_MUTEX_MOD_MDP_FG1,
    [MUTEX_MOD_IDX_MDP_FG2] = MT8195_MUTEX_MOD_MDP_FG2,
    [MUTEX_MOD_IDX_MDP_FG3] = MT8195_MUTEX_MOD_MDP_FG3,
    [MUTEX_MOD_IDX_MDP_HDR0] = MT8195_MUTEX_MOD_MDP_HDR0,
    [MUTEX_MOD_IDX_MDP_HDR1] = MT8195_MUTEX_MOD_MDP_HDR1,
    [MUTEX_MOD_IDX_MDP_HDR2] = MT8195_MUTEX_MOD_MDP_HDR2,
    [MUTEX_MOD_IDX_MDP_HDR3] = MT8195_MUTEX_MOD_MDP_HDR3,
    [MUTEX_MOD_IDX_MDP_AAL0] = MT8195_MUTEX_MOD_MDP_AAL0,
    [MUTEX_MOD_IDX_MDP_AAL1] = MT8195_MUTEX_MOD_MDP_AAL1,
    [MUTEX_MOD_IDX_MDP_AAL2] = MT8195_MUTEX_MOD_MDP_AAL2,
    [MUTEX_MOD_IDX_MDP_AAL3] = MT8195_MUTEX_MOD_MDP_AAL3,
    [MUTEX_MOD_IDX_MDP_RSZ0] = MT8195_MUTEX_MOD_MDP_RSZ0,
    [MUTEX_MOD_IDX_MDP_RSZ1] = MT8195_MUTEX_MOD_MDP_RSZ1,
    [MUTEX_MOD_IDX_MDP_RSZ2] = MT8195_MUTEX_MOD_MDP_RSZ2,
    [MUTEX_MOD_IDX_MDP_RSZ3] = MT8195_MUTEX_MOD_MDP_RSZ3,
    [MUTEX_MOD_IDX_MDP_MERGE2] = MT8195_MUTEX_MOD_MDP_MERGE2,
    [MUTEX_MOD_IDX_MDP_MERGE3] = MT8195_MUTEX_MOD_MDP_MERGE3,
    [MUTEX_MOD_IDX_MDP_TDSHP0] = MT8195_MUTEX_MOD_MDP_TDSHP0,
    [MUTEX_MOD_IDX_MDP_TDSHP1] = MT8195_MUTEX_MOD_MDP_TDSHP1,
    [MUTEX_MOD_IDX_MDP_TDSHP2] = MT8195_MUTEX_MOD_MDP_TDSHP2,
    [MUTEX_MOD_IDX_MDP_TDSHP3] = MT8195_MUTEX_MOD_MDP_TDSHP3,
    [MUTEX_MOD_IDX_MDP_COLOR0] = MT8195_MUTEX_MOD_MDP_COLOR0,
    [MUTEX_MOD_IDX_MDP_COLOR1] = MT8195_MUTEX_MOD_MDP_COLOR1,
    [MUTEX_MOD_IDX_MDP_COLOR2] = MT8195_MUTEX_MOD_MDP_COLOR2,
    [MUTEX_MOD_IDX_MDP_COLOR3] = MT8195_MUTEX_MOD_MDP_COLOR3,
    [MUTEX_MOD_IDX_MDP_OVL0] = MT8195_MUTEX_MOD_MDP_OVL0,
    [MUTEX_MOD_IDX_MDP_OVL1] = MT8195_MUTEX_MOD_MDP_OVL1,
    [MUTEX_MOD_IDX_MDP_PAD0] = MT8195_MUTEX_MOD_MDP_PAD0,
    [MUTEX_MOD_IDX_MDP_PAD1] = MT8195_MUTEX_MOD_MDP_PAD1,
    [MUTEX_MOD_IDX_MDP_PAD2] = MT8195_MUTEX_MOD_MDP_PAD2,
    [MUTEX_MOD_IDX_MDP_PAD3] = MT8195_MUTEX_MOD_MDP_PAD3,
    [MUTEX_MOD_IDX_MDP_TCC0] = MT8195_MUTEX_MOD_MDP_TCC0,
    [MUTEX_MOD_IDX_MDP_TCC1] = MT8195_MUTEX_MOD_MDP_TCC1,
    [MUTEX_MOD_IDX_MDP_WROT0] = MT8195_MUTEX_MOD_MDP_WROT0,
    [MUTEX_MOD_IDX_MDP_WROT1] = MT8195_MUTEX_MOD_MDP_WROT1,
    [MUTEX_MOD_IDX_MDP_WROT2] = MT8195_MUTEX_MOD_MDP_WROT2,
    [MUTEX_MOD_IDX_MDP_WROT3] = MT8195_MUTEX_MOD_MDP_WROT3,
    };
    static const u8 mt8365_mutex_mod[DDP_COMPONENT_ID_MAX] = {
    [DDP_COMPONENT_AAL0] = MT8365_MUTEX_MOD_DISP_AAL,
    [DDP_COMPONENT_CCORR] = MT8365_MUTEX_MOD_DISP_CCORR,
    [DDP_COMPONENT_COLOR0] = MT8365_MUTEX_MOD_DISP_COLOR0,
    [DDP_COMPONENT_DITHER0] = MT8365_MUTEX_MOD_DISP_DITHER,
    [DDP_COMPONENT_DPI0] = MT8365_MUTEX_MOD_DISP_DPI0,
    [DDP_COMPONENT_DSI0] = MT8365_MUTEX_MOD_DISP_DSI0,
    [DDP_COMPONENT_GAMMA] = MT8365_MUTEX_MOD_DISP_GAMMA,
    [DDP_COMPONENT_OVL0] = MT8365_MUTEX_MOD_DISP_OVL0,
    [DDP_COMPONENT_OVL_2L0] = MT8365_MUTEX_MOD_DISP_OVL0_2L,
    [DDP_COMPONENT_PWM0] = MT8365_MUTEX_MOD_DISP_PWM0,
    [DDP_COMPONENT_RDMA0] = MT8365_MUTEX_MOD_DISP_RDMA0,
    [DDP_COMPONENT_RDMA1] = MT8365_MUTEX_MOD_DISP_RDMA1,
    [DDP_COMPONENT_WDMA0] = MT8365_MUTEX_MOD_DISP_WDMA0,
    };
    static const u16 mt2712_mutex_sof[DDP_MUTEX_SOF_MAX] = {
    [MUTEX_SOF_SINGLE_MODE] = MUTEX_SOF_SINGLE_MODE,
    [MUTEX_SOF_DSI0] = MUTEX_SOF_DSI0,
    [MUTEX_SOF_DSI1] = MUTEX_SOF_DSI1,
    [MUTEX_SOF_DPI0] = MUTEX_SOF_DPI0,
    [MUTEX_SOF_DPI1] = MUTEX_SOF_DPI1,
    [MUTEX_SOF_DSI2] = MUTEX_SOF_DSI2,
    [MUTEX_SOF_DSI3] = MUTEX_SOF_DSI3,
    };
    static const u16 mt6795_mutex_sof[DDP_MUTEX_SOF_MAX] = {
    [MUTEX_SOF_SINGLE_MODE] = MUTEX_SOF_SINGLE_MODE,
    [MUTEX_SOF_DSI0] = MUTEX_SOF_DSI0,
    [MUTEX_SOF_DSI1] = MUTEX_SOF_DSI1,
    [MUTEX_SOF_DPI0] = MUTEX_SOF_DPI0,
    };
    static const u16 mt8167_mutex_sof[DDP_MUTEX_SOF_MAX] = {
    [MUTEX_SOF_SINGLE_MODE] = MUTEX_SOF_SINGLE_MODE,
    [MUTEX_SOF_DSI0] = MUTEX_SOF_DSI0,
    [MUTEX_SOF_DPI0] = MT8167_MUTEX_SOF_DPI0,
    [MUTEX_SOF_DPI1] = MT8167_MUTEX_SOF_DPI1,
    };
// Add EOF setting so overlay hardware can receive frame done irq
    static const u16 mt8183_mutex_sof[DDP_MUTEX_SOF_MAX] = {
    [MUTEX_SOF_SINGLE_MODE] = MUTEX_SOF_SINGLE_MODE,
    [MUTEX_SOF_DSI0] = MUTEX_SOF_DSI0 | MT8183_MUTEX_EOF_DSI0,
    [MUTEX_SOF_DPI0] = MT8183_MUTEX_SOF_DPI0 | MT8183_MUTEX_EOF_DPI0,
    };
    static const u16 mt8186_mutex_sof[MUTEX_SOF_DSI3 + 1] = {
    [MUTEX_SOF_SINGLE_MODE] = MUTEX_SOF_SINGLE_MODE,
    [MUTEX_SOF_DSI0] = MT8186_MUTEX_SOF_DSI0 | MT8186_MUTEX_EOF_DSI0,
    [MUTEX_SOF_DPI0] = MT8186_MUTEX_SOF_DPI0 | MT8186_MUTEX_EOF_DPI0,
    };
//
// To support refresh mode(video mode), DISP_REG_MUTEX_SOF should
// select the EOF source and configure the EOF plus timing from the
// module that provides the timing signal.
// So that MUTEX can not only send a STREAM_DONE event to GCE
// but also detect the error at end of frame(EAEOF) when EOF signal
// arrives.
//
    static const u16 mt8188_mutex_sof[DDP_MUTEX_SOF_MAX] = {
    [MUTEX_SOF_SINGLE_MODE] = MUTEX_SOF_SINGLE_MODE,
    [MUTEX_SOF_DSI0] =
    MT8188_MUTEX_SOF_DSI0 | MT8188_MUTEX_EOF_DSI0,
    [MUTEX_SOF_DPI1] =
    MT8188_MUTEX_SOF_DPI1 | MT8188_MUTEX_EOF_DPI1,
    [MUTEX_SOF_DP_INTF0] =
    MT8188_MUTEX_SOF_DP_INTF0 | MT8188_MUTEX_EOF_DP_INTF0,
    [MUTEX_SOF_DP_INTF1] =
    MT8188_MUTEX_SOF_DP_INTF1 | MT8188_MUTEX_EOF_DP_INTF1,
    };
    static const u16 mt8195_mutex_sof[DDP_MUTEX_SOF_MAX] = {
    [MUTEX_SOF_SINGLE_MODE] = MUTEX_SOF_SINGLE_MODE,
    [MUTEX_SOF_DSI0] = MT8195_MUTEX_SOF_DSI0 | MT8195_MUTEX_EOF_DSI0,
    [MUTEX_SOF_DSI1] = MT8195_MUTEX_SOF_DSI1 | MT8195_MUTEX_EOF_DSI1,
    [MUTEX_SOF_DPI0] = MT8195_MUTEX_SOF_DPI0 | MT8195_MUTEX_EOF_DPI0,
    [MUTEX_SOF_DPI1] = MT8195_MUTEX_SOF_DPI1 | MT8195_MUTEX_EOF_DPI1,
    [MUTEX_SOF_DP_INTF0] =
    MT8195_MUTEX_SOF_DP_INTF0 | MT8195_MUTEX_EOF_DP_INTF0,
    [MUTEX_SOF_DP_INTF1] =
    MT8195_MUTEX_SOF_DP_INTF1 | MT8195_MUTEX_EOF_DP_INTF1,
    };
    static const struct mtk_mutex_data mt2701_mutex_driver_data = {
    .mutex_mod = mt2701_mutex_mod,
    .mutex_sof = mt2712_mutex_sof,
    .mutex_mod_reg = MT2701_MUTEX0_MOD0,
    .mutex_mod1_reg = MT2701_MUTEX0_MOD1,
    .mutex_sof_reg = MT2701_MUTEX0_SOF0,
    };
    static const struct mtk_mutex_data mt2712_mutex_driver_data = {
    .mutex_mod = mt2712_mutex_mod,
    .mutex_sof = mt2712_mutex_sof,
    .mutex_mod_reg = MT2701_MUTEX0_MOD0,
    .mutex_mod1_reg = MT2701_MUTEX0_MOD1,
    .mutex_sof_reg = MT2701_MUTEX0_SOF0,
    };
    static const struct mtk_mutex_data mt6795_mutex_driver_data = {
    .mutex_mod = mt8173_mutex_mod,
    .mutex_sof = mt6795_mutex_sof,
    .mutex_mod_reg = MT2701_MUTEX0_MOD0,
    .mutex_mod1_reg = MT2701_MUTEX0_MOD1,
    .mutex_sof_reg = MT2701_MUTEX0_SOF0,
    };
    static const struct mtk_mutex_data mt8167_mutex_driver_data = {
    .mutex_mod = mt8167_mutex_mod,
    .mutex_sof = mt8167_mutex_sof,
    .mutex_mod_reg = MT2701_MUTEX0_MOD0,
    .mutex_mod1_reg = MT2701_MUTEX0_MOD1,
    .mutex_sof_reg = MT2701_MUTEX0_SOF0,
    .no_clk = true,
    };
    static const struct mtk_mutex_data mt8173_mutex_driver_data = {
    .mutex_mod = mt8173_mutex_mod,
    .mutex_sof = mt2712_mutex_sof,
    .mutex_mod_reg = MT2701_MUTEX0_MOD0,
    .mutex_mod1_reg = MT2701_MUTEX0_MOD1,
    .mutex_sof_reg = MT2701_MUTEX0_SOF0,
    };
    static const struct mtk_mutex_data mt8183_mutex_driver_data = {
    .mutex_mod = mt8183_mutex_mod,
    .mutex_sof = mt8183_mutex_sof,
    .mutex_mod_reg = MT8183_MUTEX0_MOD0,
    .mutex_mod1_reg = MT8183_MUTEX0_MOD1,
    .mutex_sof_reg = MT8183_MUTEX0_SOF0,
    .mutex_table_mod = mt8183_mutex_table_mod,
    .no_clk = true,
    };
    static const struct mtk_mutex_data mt8186_mdp_mutex_driver_data = {
    .mutex_mod_reg = MT8183_MUTEX0_MOD0,
    .mutex_mod1_reg = MT8183_MUTEX0_MOD1,
    .mutex_sof_reg = MT8183_MUTEX0_SOF0,
    .mutex_table_mod = mt8186_mdp_mutex_table_mod,
    };
    static const struct mtk_mutex_data mt8186_mutex_driver_data = {
    .mutex_mod = mt8186_mutex_mod,
    .mutex_sof = mt8186_mutex_sof,
    .mutex_mod_reg = MT8183_MUTEX0_MOD0,
    .mutex_mod1_reg = MT8183_MUTEX0_MOD1,
    .mutex_sof_reg = MT8183_MUTEX0_SOF0,
    };
    static const struct mtk_mutex_data mt8188_mutex_driver_data = {
    .mutex_mod = mt8188_mutex_mod,
    .mutex_sof = mt8188_mutex_sof,
    .mutex_mod_reg = MT8183_MUTEX0_MOD0,
    .mutex_mod1_reg = MT8183_MUTEX0_MOD1,
    .mutex_sof_reg = MT8183_MUTEX0_SOF0,
    };
    static const struct mtk_mutex_data mt8188_vpp_mutex_driver_data = {
    .mutex_sof = mt8188_mutex_sof,
    .mutex_mod_reg = MT8183_MUTEX0_MOD0,
    .mutex_mod1_reg = MT8183_MUTEX0_MOD1,
    .mutex_sof_reg = MT8183_MUTEX0_SOF0,
    .mutex_table_mod = mt8188_mdp_mutex_table_mod,
    };
    static const struct mtk_mutex_data mt8192_mutex_driver_data = {
    .mutex_mod = mt8192_mutex_mod,
    .mutex_sof = mt8183_mutex_sof,
    .mutex_mod_reg = MT8183_MUTEX0_MOD0,
    .mutex_mod1_reg = MT8183_MUTEX0_MOD1,
    .mutex_sof_reg = MT8183_MUTEX0_SOF0,
    };
    static const struct mtk_mutex_data mt8195_mutex_driver_data = {
    .mutex_mod = mt8195_mutex_mod,
    .mutex_sof = mt8195_mutex_sof,
    .mutex_mod_reg = MT8183_MUTEX0_MOD0,
    .mutex_mod1_reg = MT8183_MUTEX0_MOD1,
    .mutex_sof_reg = MT8183_MUTEX0_SOF0,
    };
    static const struct mtk_mutex_data mt8195_vpp_mutex_driver_data = {
    .mutex_sof = mt8195_mutex_sof,
    .mutex_mod_reg = MT8183_MUTEX0_MOD0,
    .mutex_mod1_reg = MT8183_MUTEX0_MOD1,
    .mutex_sof_reg = MT8183_MUTEX0_SOF0,
    .mutex_table_mod = mt8195_mutex_table_mod,
    };
    static const struct mtk_mutex_data mt8365_mutex_driver_data = {
    .mutex_mod = mt8365_mutex_mod,
    .mutex_sof = mt8183_mutex_sof,
    .mutex_mod_reg = MT8183_MUTEX0_MOD0,
    .mutex_mod1_reg = MT8183_MUTEX0_MOD1,
    .mutex_sof_reg = MT8183_MUTEX0_SOF0,
    .no_clk = true,
    };
    struct mtk_mutex *mtk_mutex_get(struct device *dev)
    {
    struct mtk_mutex_ctx *mtx = dev_get_drvdata(dev);
    int i;
    for (i = 0; i < MTK_MUTEX_MAX_HANDLES; i++)
    if (!mtx.mutex[i].claimed) {
    mtx.mutex[i].claimed = true;
    return &mtx.mutex[i];
    }
    return ERR_PTR(-EBUSY);
    }
    EXPORT_SYMBOL_GPL(mtk_mutex_get);
#[no_mangle]
pub unsafe extern "C" fn mtk_mutex_put(mutex: *mut mtk_mutex) {
    void mtk_mutex_put(struct mtk_mutex *mutex)
    {
    struct mtk_mutex_ctx *mtx = container_of(mutex, struct mtk_mutex_ctx,
    mutex[mutex.id]);
    WARN_ON(&mtx.mutex[mutex.id] != mutex);
    mutex.claimed = false;
    }
    EXPORT_SYMBOL_GPL(mtk_mutex_put);
#[no_mangle]
pub unsafe extern "C" fn mtk_mutex_prepare(mutex: *mut mtk_mutex) -> c_int {
    int mtk_mutex_prepare(struct mtk_mutex *mutex)
    {
    struct mtk_mutex_ctx *mtx = container_of(mutex, struct mtk_mutex_ctx,
    mutex[mutex.id]);
    return clk_prepare_enable(mtx.clk);
    }
    EXPORT_SYMBOL_GPL(mtk_mutex_prepare);
#[no_mangle]
pub unsafe extern "C" fn mtk_mutex_unprepare(mutex: *mut mtk_mutex) {
    void mtk_mutex_unprepare(struct mtk_mutex *mutex)
    {
    struct mtk_mutex_ctx *mtx = container_of(mutex, struct mtk_mutex_ctx,
    mutex[mutex.id]);
    clk_disable_unprepare(mtx.clk);
    }
    EXPORT_SYMBOL_GPL(mtk_mutex_unprepare);
    void mtk_mutex_add_comp(struct mtk_mutex *mutex,
    enum mtk_ddp_comp_id id)
    {
    struct mtk_mutex_ctx *mtx = container_of(mutex, struct mtk_mutex_ctx,
    mutex[mutex.id]);
    unsigned int reg;
    unsigned int sof_id, mod_id;
    unsigned int offset;
    WARN_ON(&mtx.mutex[mutex.id] != mutex);
    switch (id) {
    case DDP_COMPONENT_DSI0:
    sof_id = MUTEX_SOF_DSI0;
    break;
    case DDP_COMPONENT_DSI1:
    sof_id = MUTEX_SOF_DSI0;
    break;
    case DDP_COMPONENT_DSI2:
    sof_id = MUTEX_SOF_DSI2;
    break;
    case DDP_COMPONENT_DSI3:
    sof_id = MUTEX_SOF_DSI3;
    break;
    case DDP_COMPONENT_DPI0:
    sof_id = MUTEX_SOF_DPI0;
    break;
    case DDP_COMPONENT_DPI1:
    sof_id = MUTEX_SOF_DPI1;
    break;
    case DDP_COMPONENT_DP_INTF0:
    sof_id = MUTEX_SOF_DP_INTF0;
    break;
    case DDP_COMPONENT_DP_INTF1:
    sof_id = MUTEX_SOF_DP_INTF1;
    break;
    default:
    offset = DISP_REG_MUTEX_MOD(mtx, mtx.data.mutex_mod[id], mutex.id);
    mod_id = mtx.data.mutex_mod[id] % 32;
    reg = readl_relaxed(mtx.regs + offset);
    reg |= BIT(mod_id);
    writel_relaxed(reg, mtx.regs + offset);
    return;
    }
    writel_relaxed(mtx.data.mutex_sof[sof_id],
    mtx.regs +
    DISP_REG_MUTEX_SOF(mtx.data.mutex_sof_reg, mutex.id));
    }
    EXPORT_SYMBOL_GPL(mtk_mutex_add_comp);
    void mtk_mutex_remove_comp(struct mtk_mutex *mutex,
    enum mtk_ddp_comp_id id)
    {
    struct mtk_mutex_ctx *mtx = container_of(mutex, struct mtk_mutex_ctx,
    mutex[mutex.id]);
    unsigned int reg;
    unsigned int mod_id;
    unsigned int offset;
    WARN_ON(&mtx.mutex[mutex.id] != mutex);
    switch (id) {
    case DDP_COMPONENT_DSI0:
    case DDP_COMPONENT_DSI1:
    case DDP_COMPONENT_DSI2:
    case DDP_COMPONENT_DSI3:
    case DDP_COMPONENT_DPI0:
    case DDP_COMPONENT_DPI1:
    case DDP_COMPONENT_DP_INTF0:
    case DDP_COMPONENT_DP_INTF1:
    writel_relaxed(MUTEX_SOF_SINGLE_MODE,
    mtx.regs +
    DISP_REG_MUTEX_SOF(mtx.data.mutex_sof_reg,
    mutex.id));
    break;
    default:
    offset = DISP_REG_MUTEX_MOD(mtx, mtx.data.mutex_mod[id], mutex.id);
    mod_id = mtx.data.mutex_mod[id] % 32;
    reg = readl_relaxed(mtx.regs + offset);
    reg &= ~BIT(mod_id);
    writel_relaxed(reg, mtx.regs + offset);
    break;
    }
    }
    EXPORT_SYMBOL_GPL(mtk_mutex_remove_comp);
#[no_mangle]
pub unsafe extern "C" fn mtk_mutex_enable(mutex: *mut mtk_mutex) {
    void mtk_mutex_enable(struct mtk_mutex *mutex)
    {
    struct mtk_mutex_ctx *mtx = container_of(mutex, struct mtk_mutex_ctx,
    mutex[mutex.id]);
    WARN_ON(&mtx.mutex[mutex.id] != mutex);
    writel(1, mtx.regs + DISP_REG_MUTEX_EN(mutex.id));
    }
    EXPORT_SYMBOL_GPL(mtk_mutex_enable);
#[no_mangle]
pub unsafe extern "C" fn mtk_mutex_enable_by_cmdq(mutex: *mut mtk_mutex, pkt: *mut c_void) -> c_int {
    int mtk_mutex_enable_by_cmdq(struct mtk_mutex *mutex, void *pkt)
    {
    struct mtk_mutex_ctx *mtx = container_of(mutex, struct mtk_mutex_ctx,
    mutex[mutex.id]);
    struct cmdq_pkt *cmdq_pkt = (struct cmdq_pkt *)pkt;
    let mut en_addr: dma_addr_t = mtx.addr + DISP_REG_MUTEX_EN(mutex.id);
    WARN_ON(&mtx.mutex[mutex.id] != mutex);
    if (!mtx.cmdq_reg.size) {
    dev_err(mtx.dev, "mediatek,gce-client-reg hasn't been set");
    return -ENODEV;
    }
    mtx.cmdq_reg.pkt_write(cmdq_pkt, mtx.cmdq_reg.subsys, en_addr, en_addr, 1);
    return 0;
    }
    EXPORT_SYMBOL_GPL(mtk_mutex_enable_by_cmdq);
#[no_mangle]
pub unsafe extern "C" fn mtk_mutex_disable(mutex: *mut mtk_mutex) {
    void mtk_mutex_disable(struct mtk_mutex *mutex)
    {
    struct mtk_mutex_ctx *mtx = container_of(mutex, struct mtk_mutex_ctx,
    mutex[mutex.id]);
    WARN_ON(&mtx.mutex[mutex.id] != mutex);
    writel(0, mtx.regs + DISP_REG_MUTEX_EN(mutex.id));
    }
    EXPORT_SYMBOL_GPL(mtk_mutex_disable);
#[no_mangle]
pub unsafe extern "C" fn mtk_mutex_acquire(mutex: *mut mtk_mutex) {
    void mtk_mutex_acquire(struct mtk_mutex *mutex)
    {
    struct mtk_mutex_ctx *mtx = container_of(mutex, struct mtk_mutex_ctx,
    mutex[mutex.id]);
    u32 tmp;
    writel(1, mtx.regs + DISP_REG_MUTEX_EN(mutex.id));
    writel(1, mtx.regs + DISP_REG_MUTEX(mutex.id));
    if (readl_poll_timeout_atomic(mtx.regs + DISP_REG_MUTEX(mutex.id),
    tmp, tmp & INT_MUTEX, 1, 10000))
    pr_err("could not acquire mutex %d\n", mutex.id);
    }
    EXPORT_SYMBOL_GPL(mtk_mutex_acquire);
#[no_mangle]
pub unsafe extern "C" fn mtk_mutex_release(mutex: *mut mtk_mutex) {
    void mtk_mutex_release(struct mtk_mutex *mutex)
    {
    struct mtk_mutex_ctx *mtx = container_of(mutex, struct mtk_mutex_ctx,
    mutex[mutex.id]);
    writel(0, mtx.regs + DISP_REG_MUTEX(mutex.id));
    }
    EXPORT_SYMBOL_GPL(mtk_mutex_release);
    int mtk_mutex_write_mod(struct mtk_mutex *mutex,
    enum mtk_mutex_mod_index idx, bool clear)
    {
    struct mtk_mutex_ctx *mtx = container_of(mutex, struct mtk_mutex_ctx,
    mutex[mutex.id]);
    unsigned int reg;
    u32 offset, mod_id;
    WARN_ON(&mtx.mutex[mutex.id] != mutex);
    if (idx < MUTEX_MOD_IDX_MDP_RDMA0 ||
    idx >= MUTEX_MOD_IDX_MAX) {
    dev_err(mtx.dev, "Not supported MOD table index : %d", idx);
    return -EINVAL;
    }
    offset = DISP_REG_MUTEX_MOD(mtx, mtx.data.mutex_table_mod[idx], mutex.id);
    mod_id = mtx.data.mutex_table_mod[idx] % 32;
    reg = readl_relaxed(mtx.regs + offset);
    if (clear)
    reg &= ~BIT(mod_id);
    else
    reg |= BIT(mod_id);
    writel_relaxed(reg, mtx.regs + offset);
    return 0;
    }
    EXPORT_SYMBOL_GPL(mtk_mutex_write_mod);
    int mtk_mutex_write_sof(struct mtk_mutex *mutex,
    enum mtk_mutex_sof_index idx)
    {
    struct mtk_mutex_ctx *mtx = container_of(mutex, struct mtk_mutex_ctx,
    mutex[mutex.id]);
    WARN_ON(&mtx.mutex[mutex.id] != mutex);
    if (idx < MUTEX_SOF_IDX_SINGLE_MODE ||
    idx >= MUTEX_SOF_IDX_MAX) {
    dev_err(mtx.dev, "Not supported SOF index : %d", idx);
    return -EINVAL;
    }
    writel_relaxed(idx, mtx.regs +
    DISP_REG_MUTEX_SOF(mtx.data.mutex_sof_reg, mutex.id));
    return 0;
    }
    EXPORT_SYMBOL_GPL(mtk_mutex_write_sof);
#[no_mangle]
unsafe extern "C" fn mtk_mutex_probe(pdev: *mut platform_device) -> c_int {
    static int mtk_mutex_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct mtk_mutex_ctx *mtx;
    struct resource *regs;
    int i, ret;
    mtx = devm_kzalloc(dev, sizeof(*mtx), GFP_KERNEL);
    if (!mtx)
    return -ENOMEM;
    for (i = 0; i < MTK_MUTEX_MAX_HANDLES; i++)
    mtx.mutex[i].id = i;
    mtx.data = of_device_get_match_data(dev);
    if (!mtx.data.no_clk) {
    mtx.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(mtx.clk))
    return dev_err_probe(dev, PTR_ERR(mtx.clk), "Failed to get clock\n");
    }
    mtx.regs = devm_platform_get_and_ioremap_resource(pdev, 0, &regs);
    if (IS_ERR(mtx.regs)) {
    dev_err(dev, "Failed to map mutex registers\n");
    return PTR_ERR(mtx.regs);
    }
    mtx.addr = regs.start;
// CMDQ is optional
    ret = cmdq_dev_get_client_reg(dev, &mtx.cmdq_reg, 0);
    if (ret)
    dev_dbg(dev, "No mediatek,gce-client-reg!\n");
    platform_set_drvdata(pdev, mtx);
    return 0;
    }
    static const struct of_device_id mutex_driver_dt_match[] = {
    { .compatible = "mediatek,mt2701-disp-mutex", .data = &mt2701_mutex_driver_data },
    { .compatible = "mediatek,mt2712-disp-mutex", .data = &mt2712_mutex_driver_data },
    { .compatible = "mediatek,mt6795-disp-mutex", .data = &mt6795_mutex_driver_data },
    { .compatible = "mediatek,mt8167-disp-mutex", .data = &mt8167_mutex_driver_data },
    { .compatible = "mediatek,mt8173-disp-mutex", .data = &mt8173_mutex_driver_data },
    { .compatible = "mediatek,mt8183-disp-mutex", .data = &mt8183_mutex_driver_data },
    { .compatible = "mediatek,mt8186-disp-mutex", .data = &mt8186_mutex_driver_data },
    { .compatible = "mediatek,mt8186-mdp3-mutex", .data = &mt8186_mdp_mutex_driver_data },
    { .compatible = "mediatek,mt8188-disp-mutex", .data = &mt8188_mutex_driver_data },
    { .compatible = "mediatek,mt8188-vpp-mutex",  .data = &mt8188_vpp_mutex_driver_data },
    { .compatible = "mediatek,mt8192-disp-mutex", .data = &mt8192_mutex_driver_data },
    { .compatible = "mediatek,mt8195-disp-mutex", .data = &mt8195_mutex_driver_data },
    { .compatible = "mediatek,mt8195-vpp-mutex",  .data = &mt8195_vpp_mutex_driver_data },
    { .compatible = "mediatek,mt8365-disp-mutex", .data = &mt8365_mutex_driver_data },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, mutex_driver_dt_match);
    static struct platform_driver mtk_mutex_driver = {
    .probe		= mtk_mutex_probe,
    .driver		= {
    .name	= "mediatek-mutex",
    .of_match_table = mutex_driver_dt_match,
    },
    };
    module_platform_driver(mtk_mutex_driver);
    MODULE_AUTHOR("Yongqiang Niu <yongqiang.niu@mediatek.com>");
    MODULE_DESCRIPTION("MediaTek SoC MUTEX driver");
    MODULE_LICENSE("GPL");
