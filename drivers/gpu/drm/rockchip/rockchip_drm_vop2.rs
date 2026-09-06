//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/rockchip/rockchip_drm_vop2.h
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
// Copyright (C) Rockchip Electronics Co., Ltd.
// Author:Mark Yao <mark.yao@rock-chips.com>
//

// The VOP version of new SoC is bigger than the old

//
// the delay number of a window in different mode.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum win_dly_mode {
    VOP2_DLY_MODE_DEFAULT,   /**< default mode */
    VOP2_DLY_MODE_HISO_S,    /** HDR in SDR out mode, as a SDR window */
    VOP2_DLY_MODE_HIHO_H,    /** HDR in HDR out mode, as a HDR window */
    VOP2_DLY_MODE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vop2_dly_module {
    VOP2_DLY_WIN,           /** Win delay cycle for this VP */
    VOP2_DLY_LAYER_MIX,     /** Layer Mix delay cycle for this VP */
    VOP2_DLY_HDR_MIX,       /** HDR delay cycle for this VP */
    VOP2_DLY_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vop2_scale_up_mode {
    VOP2_SCALE_UP_NRST_NBOR,
    VOP2_SCALE_UP_BIL,
    VOP2_SCALE_UP_BIC,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vop2_scale_down_mode {
    VOP2_SCALE_DOWN_NRST_NBOR,
    VOP2_SCALE_DOWN_BIL,
    VOP2_SCALE_DOWN_AVG,
}

//
// vop2 internal power domain id,
// should be all none zero, 0 will be treat as invalid;
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vop2_win_regs {
    VOP2_WIN_ENABLE,
    VOP2_WIN_FORMAT,
    VOP2_WIN_CSC_MODE,
    VOP2_WIN_XMIRROR,
    VOP2_WIN_YMIRROR,
    VOP2_WIN_RB_SWAP,
    VOP2_WIN_UV_SWAP,
    VOP2_WIN_ACT_INFO,
    VOP2_WIN_DSP_INFO,
    VOP2_WIN_DSP_ST,
    VOP2_WIN_YRGB_MST,
    VOP2_WIN_UV_MST,
    VOP2_WIN_YRGB_VIR,
    VOP2_WIN_UV_VIR,
    VOP2_WIN_YUV_CLIP,
    VOP2_WIN_Y2R_EN,
    VOP2_WIN_R2Y_EN,
    VOP2_WIN_COLOR_KEY,
    VOP2_WIN_COLOR_KEY_EN,
    VOP2_WIN_DITHER_UP,
    VOP2_WIN_AXI_BUS_ID,
    VOP2_WIN_AXI_YRGB_R_ID,
    VOP2_WIN_AXI_UV_R_ID,

// scale regs
    VOP2_WIN_SCALE_YRGB_X,
    VOP2_WIN_SCALE_YRGB_Y,
    VOP2_WIN_SCALE_CBCR_X,
    VOP2_WIN_SCALE_CBCR_Y,
    VOP2_WIN_YRGB_HOR_SCL_MODE,
    VOP2_WIN_YRGB_HSCL_FILTER_MODE,
    VOP2_WIN_YRGB_VER_SCL_MODE,
    VOP2_WIN_YRGB_VSCL_FILTER_MODE,
    VOP2_WIN_CBCR_VER_SCL_MODE,
    VOP2_WIN_CBCR_HSCL_FILTER_MODE,
    VOP2_WIN_CBCR_HOR_SCL_MODE,
    VOP2_WIN_CBCR_VSCL_FILTER_MODE,
    VOP2_WIN_VSD_CBCR_GT2,
    VOP2_WIN_VSD_CBCR_GT4,
    VOP2_WIN_VSD_YRGB_GT2,
    VOP2_WIN_VSD_YRGB_GT4,
    VOP2_WIN_BIC_COE_SEL,

// cluster regs
    VOP2_WIN_CLUSTER_ENABLE,
    VOP2_WIN_AFBC_ENABLE,
    VOP2_WIN_CLUSTER_LB_MODE,

// afbc regs
    VOP2_WIN_AFBC_FORMAT,
    VOP2_WIN_AFBC_RB_SWAP,
    VOP2_WIN_AFBC_UV_SWAP,
    VOP2_WIN_AFBC_AUTO_GATING_EN,
    VOP2_WIN_AFBC_BLOCK_SPLIT_EN,
    VOP2_WIN_AFBC_PLD_OFFSET_EN,
    VOP2_WIN_AFBC_PIC_VIR_WIDTH,
    VOP2_WIN_AFBC_TILE_NUM,
    VOP2_WIN_AFBC_PIC_OFFSET,
    VOP2_WIN_AFBC_PIC_SIZE,
    VOP2_WIN_AFBC_DSP_OFFSET,
    VOP2_WIN_AFBC_PLD_OFFSET,
    VOP2_WIN_TRANSFORM_OFFSET,
    VOP2_WIN_AFBC_HDR_PTR,
    VOP2_WIN_AFBC_HALF_BLOCK_EN,
    VOP2_WIN_AFBC_ROTATE_270,
    VOP2_WIN_AFBC_ROTATE_90,

    VOP2_WIN_VP_SEL,
    VOP2_WIN_DLY_NUM,

    VOP2_WIN_MAX_REG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vop2_regs_dump {
    pub name: *const c_char,
    pub base: u32,
    pub size: u32,
    pub en_reg: u32,
    pub en_val: u32,
    pub en_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vop2_win_data {
    pub name: *const c_char,
    pub phys_id: c_uint,
    pub base: u32,
    pub possible_vp_mask: u32,
    pub type: drm_plane_type,
    pub nformats: u32,
    pub formats: *const u32,
    pub format_modifiers: *const u64,
    pub supported_rotations: c_uint,
//
// @layer_sel_id: defined by register OVERLAY_LAYER_SEL or PORTn_LAYER_SEL
//
    pub layer_sel_id: [c_uint; ROCKCHIP_MAX_CRTC],
    pub feature: u64,
    pub axi_bus_id: u8,
    pub axi_yrgb_r_id: u8,
    pub axi_uv_r_id: u8,
    pub max_upscale_factor: c_uint,
    pub max_downscale_factor: c_uint,
    pub dly: [u8; VOP2_DLY_MODE_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vop2_win {
    pub vop2: *mut vop2,
    pub base: drm_plane,
    pub data: *const vop2_win_data,
    pub reg: [*mut regmap_field; VOP2_WIN_MAX_REG],
//
// @win_id: graphic window id, a cluster may be split into two
// graphics windows.
//
    pub win_id: u8,
    pub delay: u8,
    pub offset: u32,
    pub type: drm_plane_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vop2_video_port_data {
    pub id: c_uint,
    pub feature: u32,
    pub gamma_lut_len: u16,
    pub cubic_lut_len: u16,
    pub max_output: vop_rect,
    pub pre_scan_max_dly: [u8; 4],
    pub offset: c_uint,
//
// @pixel_rate: pixel per cycle
//
    pub pixel_rate: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vop2_video_port {
    pub crtc: drm_crtc,
    pub vop2: *mut vop2,
    pub dclk: *mut clk,
    pub dclk_src: *mut clk,
    pub id: c_uint,
    pub data: *const vop2_video_port_data,
    pub dsp_hold_completion: completion,
//
// @win_mask: Bitmask of windows attached to the video port;
//
    pub win_mask: u32,
    pub primary_plane: *mut vop2_win,
    pub event: *mut drm_pending_vblank_event,
    pub nlayers: c_uint,
}

//
// struct vop2_ops - helper operations for vop2 hardware
//
// These hooks are used by the common part of the vop2 driver to
// implement the proper behaviour of different variants.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vop2_ops {
    pub polflags): *mut *mut *mut unsigned long (setup_intf_mux)(struct vop2_video_port vp, int ep_id, u32,
    pub vp): *mut *mut void (setup_bg_dly)(struct vop2_video_port,
    pub vp): *mut *mut void (setup_overlay)(struct vop2_video_port,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vop2_data {
    pub nr_vps: u8,
    pub feature: u64,
    pub version: u32,
    pub ops: *const vop2_ops,
    pub win: *const vop2_win_data,
    pub vp: *const vop2_video_port_data,
    pub cluster_reg: *const reg_field,
    pub smart_reg: *const reg_field,
    pub regs_dump: *const vop2_regs_dump,
    pub max_input: vop_rect,
    pub max_output: vop_rect,
    pub nr_cluster_regs: c_uint,
    pub nr_smart_regs: c_uint,
    pub win_size: c_uint,
    pub regs_dump_size: c_uint,
    pub soc_id: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vop2 {
    pub version: u32,
    pub dev: *mut device,
    pub drm: *mut drm_device,
    pub vps: [vop2_video_port; ROCKCHIP_MAX_CRTC],
    pub data: *const vop2_data,
    pub ops: *const vop2_ops,
//
// Number of windows that are registered as plane, may be less than the
// total number of hardware windows.
//
    pub registered_num_wins: u32,
    pub res: *mut resource,
    pub regs: *mut void __iomem,
    pub map: *mut regmap,
    pub sys_grf: *mut regmap,
    pub vop_grf: *mut regmap,
    pub vo1_grf: *mut regmap,
    pub sys_pmu: *mut regmap,
// physical map length of vop2 register
    pub len: u32,
    pub lut_regs: *mut void __iomem,
// protects crtc enable/disable
    pub vop2_lock: mutex,
    pub irq: c_int,
//
// Some global resources are shared between all video ports(crtcs), so
// we need a ref counter here.
//
    pub enable_count: c_uint,
    pub hclk: *mut clk,
    pub aclk: *mut clk,
    pub pclk: *mut clk,
    pub pll_hdmiphy0: *mut clk,
    pub pll_hdmiphy1: *mut clk,
// optional internal rgb encoder
    pub rgb: *mut rockchip_rgb,
//
// Used to record layer selection configuration on rk356x/rk3588
// as register RK3568_OVL_LAYER_SEL and RK3568_OVL_PORT_SEL are
// shared for all the Video Ports.
//
    pub old_layer_sel: u32,
    pub old_port_sel: u32,
//
// Ensure that the updates to these two registers(RKK3568_OVL_LAYER_SEL/RK3568_OVL_PORT_SEL)
// take effect in sequence.
//
    pub ovl_lock: mutex,
// must be put at the end of the struct
    pub win: [vop2_win; ],
}

// interrupt define

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vop_csc_format {
    CSC_BT601L,
    CSC_BT709L,
    CSC_BT601F,
    CSC_BT2020L,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum src_factor_mode {
    SRC_FAC_ALPHA_ZERO,
    SRC_FAC_ALPHA_ONE,
    SRC_FAC_ALPHA_DST,
    SRC_FAC_ALPHA_DST_INVERSE,
    SRC_FAC_ALPHA_SRC,
    SRC_FAC_ALPHA_SRC_GLOBAL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dst_factor_mode {
    DST_FAC_ALPHA_ZERO,
    DST_FAC_ALPHA_ONE,
    DST_FAC_ALPHA_SRC,
    DST_FAC_ALPHA_SRC_INVERSE,
    DST_FAC_ALPHA_DST,
    DST_FAC_ALPHA_DST_GLOBAL,
}

pub const RK3568_GRF_VO_CON1: c_uint = 0x0364;
pub const RK3588_GRF_SOC_CON1: c_uint = 0x0304;
pub const RK3588_GRF_VOP_CON2: c_uint = 0x08;
pub const RK3588_GRF_VO1_CON0: c_uint = 0x00;
// System registers definition
pub const RK3568_REG_CFG_DONE: c_uint = 0x000;
pub const RK3568_VERSION_INFO: c_uint = 0x004;
pub const RK3568_SYS_AUTO_GATING_CTRL: c_uint = 0x008;
pub const RK3576_SYS_MMU_CTRL_IMD: c_uint = 0x020;
pub const RK3568_SYS_AXI_LUT_CTRL: c_uint = 0x024;
pub const RK3568_DSP_IF_EN: c_uint = 0x028;
pub const RK3576_SYS_PORT_CTRL_IMD: c_uint = 0x028;
pub const RK3568_DSP_IF_CTRL: c_uint = 0x02c;
pub const RK3568_DSP_IF_POL: c_uint = 0x030;
pub const RK3576_SYS_CLUSTER_PD_CTRL_IMD: c_uint = 0x030;
pub const RK3588_SYS_PD_CTRL: c_uint = 0x034;
pub const RK3568_WB_CTRL: c_uint = 0x40;
pub const RK3568_WB_XSCAL_FACTOR: c_uint = 0x44;
pub const RK3568_WB_YRGB_MST: c_uint = 0x48;
pub const RK3568_WB_CBR_MST: c_uint = 0x4C;
pub const RK3568_OTP_WIN_EN: c_uint = 0x050;
pub const RK3568_LUT_PORT_SEL: c_uint = 0x058;
pub const RK3568_SYS_STATUS0: c_uint = 0x060;

pub const RK3568_SYS0_INT_EN: c_uint = 0x80;
pub const RK3568_SYS0_INT_CLR: c_uint = 0x84;
pub const RK3568_SYS0_INT_STATUS: c_uint = 0x88;
pub const RK3568_SYS1_INT_EN: c_uint = 0x90;
pub const RK3568_SYS1_INT_CLR: c_uint = 0x94;
pub const RK3568_SYS1_INT_STATUS: c_uint = 0x98;

pub const RK3576_WB_CTRL: c_uint = 0x100;
pub const RK3576_WB_XSCAL_FACTOR: c_uint = 0x104;
pub const RK3576_WB_YRGB_MST: c_uint = 0x108;
pub const RK3576_WB_CBR_MST: c_uint = 0x10C;
pub const RK3576_WB_VIR_STRIDE: c_uint = 0x110;
pub const RK3576_WB_TIMEOUT_CTRL: c_uint = 0x114;
pub const RK3576_MIPI0_IF_CTRL: c_uint = 0x180;
pub const RK3576_HDMI0_IF_CTRL: c_uint = 0x184;
pub const RK3576_EDP0_IF_CTRL: c_uint = 0x188;
pub const RK3576_DP0_IF_CTRL: c_uint = 0x18C;
pub const RK3576_RGB_IF_CTRL: c_uint = 0x194;
pub const RK3576_DP1_IF_CTRL: c_uint = 0x1A4;
pub const RK3576_DP2_IF_CTRL: c_uint = 0x1B0;
// Extra OVL register definition
pub const RK3576_SYS_EXTRA_ALPHA_CTRL: c_uint = 0x500;
pub const RK3576_CLUSTER0_MIX_SRC_COLOR_CTRL: c_uint = 0x530;
pub const RK3576_CLUSTER0_MIX_DST_COLOR_CTRL: c_uint = 0x534;
pub const RK3576_CLUSTER0_MIX_SRC_ALPHA_CTRL: c_uint = 0x538;
pub const RK3576_CLUSTER0_MIX_DST_ALPHA_CTRL: c_uint = 0x53c;
pub const RK3576_CLUSTER1_MIX_SRC_COLOR_CTRL: c_uint = 0x540;
pub const RK3576_CLUSTER1_MIX_DST_COLOR_CTRL: c_uint = 0x544;
pub const RK3576_CLUSTER1_MIX_SRC_ALPHA_CTRL: c_uint = 0x548;
pub const RK3576_CLUSTER1_MIX_DST_ALPHA_CTRL: c_uint = 0x54c;
// OVL registers for Video Port definition

// Video Port registers definition
pub const RK3568_VP0_CTRL_BASE: c_uint = 0x0C00;
pub const RK3568_VP1_CTRL_BASE: c_uint = 0x0D00;
pub const RK3568_VP2_CTRL_BASE: c_uint = 0x0E00;
pub const RK3588_VP3_CTRL_BASE: c_uint = 0x0F00;
pub const RK3568_VP_DSP_CTRL: c_uint = 0x00;
pub const RK3568_VP_MIPI_CTRL: c_uint = 0x04;
pub const RK3568_VP_COLOR_BAR_CTRL: c_uint = 0x08;
pub const RK3588_VP_CLK_CTRL: c_uint = 0x0C;
pub const RK3568_VP_3D_LUT_CTRL: c_uint = 0x10;
pub const RK3568_VP_3D_LUT_MST: c_uint = 0x20;
pub const RK3568_VP_DSP_BG: c_uint = 0x2C;
pub const RK3568_VP_PRE_SCAN_HTIMING: c_uint = 0x30;
pub const RK3568_VP_POST_DSP_HACT_INFO: c_uint = 0x34;
pub const RK3568_VP_POST_DSP_VACT_INFO: c_uint = 0x38;
pub const RK3568_VP_POST_SCL_FACTOR_YRGB: c_uint = 0x3C;
pub const RK3568_VP_POST_SCL_CTRL: c_uint = 0x40;
pub const RK3568_VP_POST_DSP_VACT_INFO_F1: c_uint = 0x44;
pub const RK3568_VP_DSP_HTOTAL_HS_END: c_uint = 0x48;
pub const RK3568_VP_DSP_HACT_ST_END: c_uint = 0x4C;
pub const RK3568_VP_DSP_VTOTAL_VS_END: c_uint = 0x50;
pub const RK3568_VP_DSP_VACT_ST_END: c_uint = 0x54;
pub const RK3568_VP_DSP_VS_ST_END_F1: c_uint = 0x58;
pub const RK3568_VP_DSP_VACT_ST_END_F1: c_uint = 0x5C;
pub const RK3568_VP_BCSH_CTRL: c_uint = 0x60;
pub const RK3568_VP_BCSH_BCS: c_uint = 0x64;
pub const RK3568_VP_BCSH_H: c_uint = 0x68;
pub const RK3568_VP_BCSH_COLOR_BAR: c_uint = 0x6C;
// Overlay registers definition
pub const RK3568_OVL_CTRL: c_uint = 0x600;
pub const RK3568_OVL_LAYER_SEL: c_uint = 0x604;
pub const RK3568_OVL_PORT_SEL: c_uint = 0x608;
pub const RK3568_CLUSTER0_MIX_SRC_COLOR_CTRL: c_uint = 0x610;
pub const RK3568_CLUSTER0_MIX_DST_COLOR_CTRL: c_uint = 0x614;
pub const RK3568_CLUSTER0_MIX_SRC_ALPHA_CTRL: c_uint = 0x618;
pub const RK3568_CLUSTER0_MIX_DST_ALPHA_CTRL: c_uint = 0x61C;
pub const RK3568_MIX0_SRC_COLOR_CTRL: c_uint = 0x650;
pub const RK3568_MIX0_DST_COLOR_CTRL: c_uint = 0x654;
pub const RK3568_MIX0_SRC_ALPHA_CTRL: c_uint = 0x658;
pub const RK3568_MIX0_DST_ALPHA_CTRL: c_uint = 0x65C;
pub const RK3568_HDR0_SRC_COLOR_CTRL: c_uint = 0x6C0;
pub const RK3568_HDR0_DST_COLOR_CTRL: c_uint = 0x6C4;
pub const RK3568_HDR0_SRC_ALPHA_CTRL: c_uint = 0x6C8;
pub const RK3568_HDR0_DST_ALPHA_CTRL: c_uint = 0x6CC;

pub const RK3568_CLUSTER_DLY_NUM: c_uint = 0x6F0;
pub const RK3568_SMART_DLY_NUM: c_uint = 0x6F8;
// Cluster register definition, offset relative to window base
pub const RK3568_CLUSTER0_CTRL_BASE: c_uint = 0x1000;
pub const RK3568_CLUSTER1_CTRL_BASE: c_uint = 0x1200;
pub const RK3588_CLUSTER2_CTRL_BASE: c_uint = 0x1400;
pub const RK3588_CLUSTER3_CTRL_BASE: c_uint = 0x1600;
pub const RK3568_ESMART0_CTRL_BASE: c_uint = 0x1800;
pub const RK3568_ESMART1_CTRL_BASE: c_uint = 0x1A00;
pub const RK3568_SMART0_CTRL_BASE: c_uint = 0x1C00;
pub const RK3568_SMART1_CTRL_BASE: c_uint = 0x1E00;
pub const RK3588_ESMART2_CTRL_BASE: c_uint = 0x1C00;
pub const RK3588_ESMART3_CTRL_BASE: c_uint = 0x1E00;
pub const RK3568_CLUSTER_WIN_CTRL0: c_uint = 0x00;
pub const RK3568_CLUSTER_WIN_CTRL1: c_uint = 0x04;
pub const RK3568_CLUSTER_WIN_CTRL2: c_uint = 0x08;
pub const RK3568_CLUSTER_WIN_YRGB_MST: c_uint = 0x10;
pub const RK3568_CLUSTER_WIN_CBR_MST: c_uint = 0x14;
pub const RK3568_CLUSTER_WIN_VIR: c_uint = 0x18;
pub const RK3568_CLUSTER_WIN_ACT_INFO: c_uint = 0x20;
pub const RK3568_CLUSTER_WIN_DSP_INFO: c_uint = 0x24;
pub const RK3568_CLUSTER_WIN_DSP_ST: c_uint = 0x28;
pub const RK3568_CLUSTER_WIN_SCL_FACTOR_YRGB: c_uint = 0x30;
pub const RK3568_CLUSTER_WIN_TRANSFORM_OFFSET: c_uint = 0x3C;
pub const RK3568_CLUSTER_WIN_AFBCD_OUTPUT_CTRL: c_uint = 0x50;
pub const RK3568_CLUSTER_WIN_AFBCD_ROTATE_MODE: c_uint = 0x54;
pub const RK3568_CLUSTER_WIN_AFBCD_HDR_PTR: c_uint = 0x58;
pub const RK3568_CLUSTER_WIN_AFBCD_VIR_WIDTH: c_uint = 0x5C;
pub const RK3568_CLUSTER_WIN_AFBCD_PIC_SIZE: c_uint = 0x60;
pub const RK3568_CLUSTER_WIN_AFBCD_PIC_OFFSET: c_uint = 0x64;
pub const RK3568_CLUSTER_WIN_AFBCD_DSP_OFFSET: c_uint = 0x68;
pub const RK3568_CLUSTER_WIN_AFBCD_CTRL: c_uint = 0x6C;
pub const RK3576_CLUSTER_WIN_AFBCD_PLD_PTR_OFFSET: c_uint = 0x78;
pub const RK3568_CLUSTER_CTRL: c_uint = 0x100;
pub const RK3576_CLUSTER_PORT_SEL_IMD: c_uint = 0x1F4;
pub const RK3576_CLUSTER_DLY_NUM: c_uint = 0x1F8;
// (E)smart register definition, offset relative to window base
pub const RK3568_SMART_CTRL0: c_uint = 0x00;
pub const RK3568_SMART_CTRL1: c_uint = 0x04;
pub const RK3588_SMART_AXI_CTRL: c_uint = 0x08;
pub const RK3568_SMART_REGION0_CTRL: c_uint = 0x10;
pub const RK3568_SMART_REGION0_YRGB_MST: c_uint = 0x14;
pub const RK3568_SMART_REGION0_CBR_MST: c_uint = 0x18;
pub const RK3568_SMART_REGION0_VIR: c_uint = 0x1C;
pub const RK3568_SMART_REGION0_ACT_INFO: c_uint = 0x20;
pub const RK3568_SMART_REGION0_DSP_INFO: c_uint = 0x24;
pub const RK3568_SMART_REGION0_DSP_ST: c_uint = 0x28;
pub const RK3568_SMART_REGION0_SCL_CTRL: c_uint = 0x30;
pub const RK3568_SMART_REGION0_SCL_FACTOR_YRGB: c_uint = 0x34;
pub const RK3568_SMART_REGION0_SCL_FACTOR_CBR: c_uint = 0x38;
pub const RK3568_SMART_REGION0_SCL_OFFSET: c_uint = 0x3C;
pub const RK3568_SMART_REGION1_CTRL: c_uint = 0x40;
pub const RK3568_SMART_REGION1_YRGB_MST: c_uint = 0x44;
pub const RK3568_SMART_REGION1_CBR_MST: c_uint = 0x48;
pub const RK3568_SMART_REGION1_VIR: c_uint = 0x4C;
pub const RK3568_SMART_REGION1_ACT_INFO: c_uint = 0x50;
pub const RK3568_SMART_REGION1_DSP_INFO: c_uint = 0x54;
pub const RK3568_SMART_REGION1_DSP_ST: c_uint = 0x58;
pub const RK3568_SMART_REGION1_SCL_CTRL: c_uint = 0x60;
pub const RK3568_SMART_REGION1_SCL_FACTOR_YRGB: c_uint = 0x64;
pub const RK3568_SMART_REGION1_SCL_FACTOR_CBR: c_uint = 0x68;
pub const RK3568_SMART_REGION1_SCL_OFFSET: c_uint = 0x6C;
pub const RK3568_SMART_REGION2_CTRL: c_uint = 0x70;
pub const RK3568_SMART_REGION2_YRGB_MST: c_uint = 0x74;
pub const RK3568_SMART_REGION2_CBR_MST: c_uint = 0x78;
pub const RK3568_SMART_REGION2_VIR: c_uint = 0x7C;
pub const RK3568_SMART_REGION2_ACT_INFO: c_uint = 0x80;
pub const RK3568_SMART_REGION2_DSP_INFO: c_uint = 0x84;
pub const RK3568_SMART_REGION2_DSP_ST: c_uint = 0x88;
pub const RK3568_SMART_REGION2_SCL_CTRL: c_uint = 0x90;
pub const RK3568_SMART_REGION2_SCL_FACTOR_YRGB: c_uint = 0x94;
pub const RK3568_SMART_REGION2_SCL_FACTOR_CBR: c_uint = 0x98;
pub const RK3568_SMART_REGION2_SCL_OFFSET: c_uint = 0x9C;
pub const RK3568_SMART_REGION3_CTRL: c_uint = 0xA0;
pub const RK3568_SMART_REGION3_YRGB_MST: c_uint = 0xA4;
pub const RK3568_SMART_REGION3_CBR_MST: c_uint = 0xA8;
pub const RK3568_SMART_REGION3_VIR: c_uint = 0xAC;
pub const RK3568_SMART_REGION3_ACT_INFO: c_uint = 0xB0;
pub const RK3568_SMART_REGION3_DSP_INFO: c_uint = 0xB4;
pub const RK3568_SMART_REGION3_DSP_ST: c_uint = 0xB8;
pub const RK3568_SMART_REGION3_SCL_CTRL: c_uint = 0xC0;
pub const RK3568_SMART_REGION3_SCL_FACTOR_YRGB: c_uint = 0xC4;
pub const RK3568_SMART_REGION3_SCL_FACTOR_CBR: c_uint = 0xC8;
pub const RK3568_SMART_REGION3_SCL_OFFSET: c_uint = 0xCC;
pub const RK3568_SMART_COLOR_KEY_CTRL: c_uint = 0xD0;
pub const RK3576_SMART_ALPHA_MAP: c_uint = 0xD8;
pub const RK3576_SMART_PORT_SEL_IMD: c_uint = 0xF4;
pub const RK3576_SMART_DLY_NUM: c_uint = 0xF8;
// HDR register definition
pub const RK3568_HDR_LUT_CTRL: c_uint = 0x2000;
pub const RK3568_HDR_LUT_MST: c_uint = 0x2004;
pub const RK3568_SDR2HDR_CTRL: c_uint = 0x2010;
pub const RK3568_HDR2SDR_CTRL: c_uint = 0x2020;
pub const RK3568_HDR2SDR_SRC_RANGE: c_uint = 0x2024;
pub const RK3568_HDR2SDR_NORMFACEETF: c_uint = 0x2028;
pub const RK3568_HDR2SDR_DST_RANGE: c_uint = 0x202C;
pub const RK3568_HDR2SDR_NORMFACCGAMMA: c_uint = 0x2030;
pub const RK3568_HDR_EETF_OETF_Y0: c_uint = 0x203C;
pub const RK3568_HDR_SAT_Y0: c_uint = 0x20C0;
pub const RK3568_HDR_EOTF_OETF_Y0: c_uint = 0x20F0;
pub const RK3568_HDR_OETF_DX_POW1: c_uint = 0x2200;
pub const RK3568_HDR_OETF_XN1: c_uint = 0x2300;

pub const VOP2_SYS_AXI_BUS_NUM: c_int = 2;
pub const VOP2_CLUSTER_YUV444_10: c_uint = 0x12;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vop2_layer_phy_id {
    ROCKCHIP_VOP2_CLUSTER0 = 0,
    ROCKCHIP_VOP2_CLUSTER1,
    ROCKCHIP_VOP2_ESMART0,
    ROCKCHIP_VOP2_ESMART1,
    ROCKCHIP_VOP2_SMART0,
    ROCKCHIP_VOP2_SMART1,
    ROCKCHIP_VOP2_CLUSTER2,
    ROCKCHIP_VOP2_CLUSTER3,
    ROCKCHIP_VOP2_ESMART2,
    ROCKCHIP_VOP2_ESMART3,
    ROCKCHIP_VOP2_PHY_ID_INVALID = -1,
}

extern "C" {
    pub fn container_of(_arg: crtc, vop2_video_port: struct, _arg: crtc) -> return;
}
extern "C" {
    pub fn container_of(_arg: p, vop2_win: struct, _arg: base) -> return;
}
//
// Note:
// The write mask function is documented but missing on rk3566/8, writes
// to these bits have no effect. For newer soc(rk3588 and following) the
// write mask is needed for register writes.
//
// GLB_CFG_DONE_EN has no write mask bit.
//
