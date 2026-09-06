//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/soc/mediatek/mtk-mmsys.h
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
pub const DISP_REG_CONFIG_DISP_OVL0_MOUT_EN: c_uint = 0x040;
pub const DISP_REG_CONFIG_DISP_OVL1_MOUT_EN: c_uint = 0x044;
pub const DISP_REG_CONFIG_DISP_OD_MOUT_EN: c_uint = 0x048;
pub const DISP_REG_CONFIG_DISP_GAMMA_MOUT_EN: c_uint = 0x04c;
pub const DISP_REG_CONFIG_DISP_UFOE_MOUT_EN: c_uint = 0x050;
pub const DISP_REG_CONFIG_DISP_COLOR0_SEL_IN: c_uint = 0x084;
pub const DISP_REG_CONFIG_DISP_COLOR1_SEL_IN: c_uint = 0x088;
pub const DISP_REG_CONFIG_DSIE_SEL_IN: c_uint = 0x0a4;
pub const DISP_REG_CONFIG_DSIO_SEL_IN: c_uint = 0x0a8;
pub const DISP_REG_CONFIG_DPI_SEL_IN: c_uint = 0x0ac;
pub const DISP_REG_CONFIG_DISP_RDMA2_SOUT: c_uint = 0x0b8;
pub const DISP_REG_CONFIG_DISP_RDMA0_SOUT_EN: c_uint = 0x0c4;
pub const DISP_REG_CONFIG_DISP_RDMA1_SOUT_EN: c_uint = 0x0c8;
pub const DISP_REG_CONFIG_MMSYS_CG_CON0: c_uint = 0x100;
pub const DISP_REG_CONFIG_DISP_OVL_MOUT_EN: c_uint = 0x030;
pub const DISP_REG_CONFIG_OUT_SEL: c_uint = 0x04c;
pub const DISP_REG_CONFIG_DSI_SEL: c_uint = 0x050;
pub const DISP_REG_CONFIG_DPI_SEL: c_uint = 0x064;
pub const OVL0_MOUT_EN_COLOR0: c_uint = 0x1;
pub const OD_MOUT_EN_RDMA0: c_uint = 0x1;

pub const UFOE_MOUT_EN_DSI0: c_uint = 0x1;
pub const COLOR0_SEL_IN_OVL0: c_uint = 0x1;
pub const OVL1_MOUT_EN_COLOR1: c_uint = 0x1;
pub const GAMMA_MOUT_EN_RDMA1: c_uint = 0x1;
pub const RDMA0_SOUT_DPI0: c_uint = 0x2;
pub const RDMA0_SOUT_DPI1: c_uint = 0x3;
pub const RDMA0_SOUT_DSI1: c_uint = 0x1;
pub const RDMA0_SOUT_DSI2: c_uint = 0x4;
pub const RDMA0_SOUT_DSI3: c_uint = 0x5;
pub const RDMA0_SOUT_MASK: c_uint = 0x7;
pub const RDMA1_SOUT_DPI0: c_uint = 0x2;
pub const RDMA1_SOUT_DPI1: c_uint = 0x3;
pub const RDMA1_SOUT_DSI1: c_uint = 0x1;
pub const RDMA1_SOUT_DSI2: c_uint = 0x4;
pub const RDMA1_SOUT_DSI3: c_uint = 0x5;
pub const RDMA1_SOUT_MASK: c_uint = 0x7;
pub const RDMA2_SOUT_DPI0: c_uint = 0x2;
pub const RDMA2_SOUT_DPI1: c_uint = 0x3;
pub const RDMA2_SOUT_DSI1: c_uint = 0x1;
pub const RDMA2_SOUT_DSI2: c_uint = 0x4;
pub const RDMA2_SOUT_DSI3: c_uint = 0x5;
pub const RDMA2_SOUT_MASK: c_uint = 0x7;
pub const DPI0_SEL_IN_RDMA1: c_uint = 0x1;
pub const DPI0_SEL_IN_RDMA2: c_uint = 0x3;
pub const DPI0_SEL_IN_MASK: c_uint = 0x3;

pub const DSI0_SEL_IN_RDMA1: c_uint = 0x1;
pub const DSI0_SEL_IN_RDMA2: c_uint = 0x4;
pub const DSI0_SEL_IN_MASK: c_uint = 0x7;
pub const DSI1_SEL_IN_RDMA1: c_uint = 0x1;
pub const DSI1_SEL_IN_RDMA2: c_uint = 0x4;
pub const DSI1_SEL_IN_MASK: c_uint = 0x7;

pub const COLOR1_SEL_IN_OVL1: c_uint = 0x1;
pub const OVL_MOUT_EN_RDMA: c_uint = 0x1;
pub const BLS_TO_DSI_RDMA1_TO_DPI1: c_uint = 0x8;
pub const BLS_TO_DPI_RDMA1_TO_DSI: c_uint = 0x2;
pub const BLS_RDMA1_DSI_DPI_MASK: c_uint = 0xf;
pub const DSI_SEL_IN_BLS: c_uint = 0x0;
pub const DPI_SEL_IN_BLS: c_uint = 0x0;
pub const DPI_SEL_IN_MASK: c_uint = 0x1;
pub const DSI_SEL_IN_RDMA: c_uint = 0x1;
pub const DSI_SEL_IN_MASK: c_uint = 0x1;

// Temporary compatibility definitions

//
// This macro adds a compile time check to make sure that the in/out
// selection bit(s) fit in the register mask, similar to bitfield
// macros, but this does not transform the value.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_mmsys_routes {
    pub from_comp: u32,
    pub to_comp: u32,
    pub addr: u32,
    pub mask: u32,
    pub val: u32,
}

//
// struct mtk_mmsys_driver_data - Settings of the mmsys
// @clk_driver: Clock driver name that the mmsys is using
// (defined in drivers/clk/mediatek/clk-*.c).
// @routes: Routing table of the mmsys.
// It provides mux settings from one module to another.
// @num_routes: Array size of the routes.
// @sw0_rst_offset: Register offset for the reset control.
// @num_resets: Number of reset bits that are defined
// @is_vppsys: Whether the mmsys is VPPSYS (Video Processing Pipe)
// or VDOSYS (Video). Only VDOSYS needs to be added to drm driver.
// @vsync_len: VSYNC length of the MIXER.
// VSYNC is usually triggered by the connector, so its length is a
// fixed value when the frame rate is decided, but ETHDR and
// MIXER generate their own VSYNC due to hardware design, therefore
// MIXER has to sync with ETHDR by adjusting VSYNC length.
// On MT8195, there is no such setting so we use the gap between
// falling edge and rising edge of SOF (Start of Frame) signal to
// do the job, but since MT8188, VSYNC_LEN setting is introduced to
// solve the problem and is given 0x40 (ticks) as the default value.
// Please notice that this value has to be set to 1 (minimum) if
// ETHDR is bypassed, otherwise MIXER could wait too long and causing
// underflow.
//
// Each MMSYS (multi-media system) may have different settings, they may use
// different clock sources, mux settings, reset control ...etc., and these
// differences are all stored here.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_mmsys_driver_data {
    pub clk_driver: *const c_char,
    pub routes: *const mtk_mmsys_routes,
    pub num_routes: c_uint,
    pub sw0_rst_offset: u16,
    pub rst_tb: *const u8,
    pub num_resets: u32,
    pub is_vppsys: bool,
    pub vsync_len: u8,
}

//
// Routes in mt2701 and mt2712 are different. That means
// in the same register address, it controls different input/output
// selection for each SoC. But, right now, they use the same table as
// default routes meet their requirements. But we don't have the complete
// route information for these three SoC, so just keep them in the same
// table. After we've more information, we could separate mt2701, mt2712
// to an independent table.
//
