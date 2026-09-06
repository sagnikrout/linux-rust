//! Automatically rewritten from C Header to Rust Module
//! Source: include/video/imx-ipu-v3.h
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
// Copyright 2005-2009 Freescale Semiconductor, Inc.
//
// The code contained herein is licensed under the GNU Lesser General
// Public License.  You may obtain a copy of the GNU Lesser General
// Public License Version 2.1 or later at the following locations:
//
// http://www.opensource.org/licenses/lgpl-license.html
// http://www.gnu.org/copyleft/lgpl.html
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipuv3_type {
    IPUV3EX,
    IPUV3M,
    IPUV3H,
}

//
// Bitfield of Display Interface signal polarities.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu_di_signal_cfg {
    pub /: *mut *mut unsigned data_pol:1; / true = inverted,
    pub /: *mut *mut unsigned clk_pol:1; / true = rising edge,
    pub enable_pol:1: unsigned,
    pub mode: videomode,
    pub bus_format: u32,
    pub v_to_h_sync: u32,

    pub clkflags: c_ulong,
    pub hsync_pin: u8,
    pub vsync_pin: u8,
}

//
// Enumeration of CSI destinations
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipu_csi_dest {
    IPU_CSI_DEST_IDMAC, /* to memory via SMFC */
    IPU_CSI_DEST_IC,	/* to Image Converter */
    IPU_CSI_DEST_VDIC,  /* to VDIC */
}

//
// Enumeration of IPU rotation modes
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipu_rotate_mode {
    IPU_ROTATE_NONE = 0,
    IPU_ROTATE_VERT_FLIP = IPU_ROT_BIT_VFLIP,
    IPU_ROTATE_HORIZ_FLIP = IPU_ROT_BIT_HFLIP,
    IPU_ROTATE_180 = (IPU_ROT_BIT_VFLIP | IPU_ROT_BIT_HFLIP),
    IPU_ROTATE_90_RIGHT = IPU_ROT_BIT_90,
    IPU_ROTATE_90_RIGHT_VFLIP = (IPU_ROT_BIT_90 | IPU_ROT_BIT_VFLIP),
    IPU_ROTATE_90_RIGHT_HFLIP = (IPU_ROT_BIT_90 | IPU_ROT_BIT_HFLIP),
    IPU_ROTATE_90_LEFT = (IPU_ROT_BIT_90 |
    IPU_ROT_BIT_VFLIP | IPU_ROT_BIT_HFLIP),
}

// 90-degree rotations require the IRT unit

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipu_color_space {
    IPUV3_COLORSPACE_RGB,
    IPUV3_COLORSPACE_YUV,
    IPUV3_COLORSPACE_UNKNOWN,
}

//
// Enumeration of VDI MOTION select
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipu_motion_sel {
    MOTION_NONE = 0,
    LOW_MOTION,
    MED_MOTION,
    HIGH_MOTION,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipu_channel_irq {
    IPU_IRQ_EOF = 0,
    IPU_IRQ_NFACK = 64,
    IPU_IRQ_NFB4EOF = 128,
    IPU_IRQ_EOS = 192,
}

//
// Enumeration of IDMAC channels
//
pub const IPUV3_CHANNEL_CSI0: c_int = 0;
pub const IPUV3_CHANNEL_CSI1: c_int = 1;
pub const IPUV3_CHANNEL_CSI2: c_int = 2;
pub const IPUV3_CHANNEL_CSI3: c_int = 3;
pub const IPUV3_CHANNEL_VDI_MEM_IC_VF: c_int = 5;
//
// NOTE: channels 6,7 are unused in the IPU and are not IDMAC channels,
// but the direct CSI->VDI linking is handled the same way as IDMAC
// channel linking in the FSU via the IPU_FS_PROC_FLOW registers, so
// these channel names are used to support the direct CSI->VDI link.
//
pub const IPUV3_CHANNEL_CSI_DIRECT: c_int = 6;
pub const IPUV3_CHANNEL_CSI_VDI_PREV: c_int = 7;
pub const IPUV3_CHANNEL_MEM_VDI_PREV: c_int = 8;
pub const IPUV3_CHANNEL_MEM_VDI_CUR: c_int = 9;
pub const IPUV3_CHANNEL_MEM_VDI_NEXT: c_int = 10;
pub const IPUV3_CHANNEL_MEM_IC_PP: c_int = 11;
pub const IPUV3_CHANNEL_MEM_IC_PRP_VF: c_int = 12;
pub const IPUV3_CHANNEL_VDI_MEM_RECENT: c_int = 13;
pub const IPUV3_CHANNEL_G_MEM_IC_PRP_VF: c_int = 14;
pub const IPUV3_CHANNEL_G_MEM_IC_PP: c_int = 15;
pub const IPUV3_CHANNEL_G_MEM_IC_PRP_VF_ALPHA: c_int = 17;
pub const IPUV3_CHANNEL_G_MEM_IC_PP_ALPHA: c_int = 18;
pub const IPUV3_CHANNEL_MEM_VDI_PLANE1_COMB_ALPHA: c_int = 19;
pub const IPUV3_CHANNEL_IC_PRP_ENC_MEM: c_int = 20;
pub const IPUV3_CHANNEL_IC_PRP_VF_MEM: c_int = 21;
pub const IPUV3_CHANNEL_IC_PP_MEM: c_int = 22;
pub const IPUV3_CHANNEL_MEM_BG_SYNC: c_int = 23;
pub const IPUV3_CHANNEL_MEM_BG_ASYNC: c_int = 24;
pub const IPUV3_CHANNEL_MEM_VDI_PLANE1_COMB: c_int = 25;
pub const IPUV3_CHANNEL_MEM_VDI_PLANE3_COMB: c_int = 26;
pub const IPUV3_CHANNEL_MEM_FG_SYNC: c_int = 27;
pub const IPUV3_CHANNEL_MEM_DC_SYNC: c_int = 28;
pub const IPUV3_CHANNEL_MEM_FG_ASYNC: c_int = 29;
pub const IPUV3_CHANNEL_MEM_FG_SYNC_ALPHA: c_int = 31;
pub const IPUV3_CHANNEL_MEM_FG_ASYNC_ALPHA: c_int = 33;
pub const IPUV3_CHANNEL_DC_MEM_READ: c_int = 40;
pub const IPUV3_CHANNEL_MEM_DC_ASYNC: c_int = 41;
pub const IPUV3_CHANNEL_MEM_DC_COMMAND: c_int = 42;
pub const IPUV3_CHANNEL_MEM_DC_COMMAND2: c_int = 43;
pub const IPUV3_CHANNEL_MEM_DC_OUTPUT_MASK: c_int = 44;
pub const IPUV3_CHANNEL_MEM_ROT_ENC: c_int = 45;
pub const IPUV3_CHANNEL_MEM_ROT_VF: c_int = 46;
pub const IPUV3_CHANNEL_MEM_ROT_PP: c_int = 47;
pub const IPUV3_CHANNEL_ROT_ENC_MEM: c_int = 48;
pub const IPUV3_CHANNEL_ROT_VF_MEM: c_int = 49;
pub const IPUV3_CHANNEL_ROT_PP_MEM: c_int = 50;
pub const IPUV3_CHANNEL_MEM_BG_SYNC_ALPHA: c_int = 51;
pub const IPUV3_CHANNEL_MEM_BG_ASYNC_ALPHA: c_int = 52;
pub const IPUV3_NUM_CHANNELS: c_int = 64;
extern "C" {
    pub fn ipu_map_irq(ipu: *mut ipu_soc, irq: c_int) -> c_int;
}

//
// IPU Common functions
//
extern "C" {
    pub fn ipu_get_num(ipu: *mut ipu_soc) -> c_int;
}
extern "C" {
    pub fn ipu_set_csi_src_mux(ipu: *mut ipu_soc, csi_id: c_int, mipi_csi2: bool);
}
extern "C" {
    pub fn ipu_set_ic_src_mux(ipu: *mut ipu_soc, csi_id: c_int, vdi: bool);
}
extern "C" {
    pub fn ipu_dump(ipu: *mut ipu_soc);
}
//
// IPU Image DMA Controller (idmac) functions
//
extern "C" {
    pub fn ipu_idmac_put(: *mut ipuv3_channel);
}
extern "C" {
    pub fn ipu_idmac_enable_channel(channel: *mut ipuv3_channel) -> c_int;
}
extern "C" {
    pub fn ipu_idmac_disable_channel(channel: *mut ipuv3_channel) -> c_int;
}
extern "C" {
    pub fn ipu_idmac_enable_watermark(channel: *mut ipuv3_channel, enable: bool);
}
extern "C" {
    pub fn ipu_idmac_lock_enable(channel: *mut ipuv3_channel, num_bursts: c_int) -> c_int;
}
extern "C" {
    pub fn ipu_idmac_wait_busy(channel: *mut ipuv3_channel, ms: c_int) -> c_int;
}
extern "C" {
    pub fn ipu_idmac_get_current_buffer(channel: *mut ipuv3_channel) -> c_int;
}
extern "C" {
    pub fn ipu_idmac_buffer_is_ready(channel: *mut ipuv3_channel, buf_num: u32) -> bool;
}
extern "C" {
    pub fn ipu_idmac_select_buffer(channel: *mut ipuv3_channel, buf_num: u32);
}
extern "C" {
    pub fn ipu_idmac_clear_buffer(channel: *mut ipuv3_channel, buf_num: u32);
}
extern "C" {
    pub fn ipu_fsu_link(ipu: *mut ipu_soc, src_ch: c_int, sink_ch: c_int) -> c_int;
}
extern "C" {
    pub fn ipu_fsu_unlink(ipu: *mut ipu_soc, src_ch: c_int, sink_ch: c_int) -> c_int;
}
extern "C" {
    pub fn ipu_idmac_link(src: *mut ipuv3_channel, sink: *mut ipuv3_channel) -> c_int;
}
extern "C" {
    pub fn ipu_idmac_unlink(src: *mut ipuv3_channel, sink: *mut ipuv3_channel) -> c_int;
}
//
// IPU Channel Parameter Memory (cpmem) functions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu_rgb {
    pub red: fb_bitfield,
    pub green: fb_bitfield,
    pub blue: fb_bitfield,
    pub transp: fb_bitfield,
    pub bits_per_pixel: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu_image {
    pub pix: v4l2_pix_format,
    pub rect: v4l2_rect,
    pub phys0: dma_addr_t,
    pub phys1: dma_addr_t,
// chroma plane offset overrides
    pub u_offset: u32,
    pub v_offset: u32,
}

extern "C" {
    pub fn ipu_cpmem_zero(ch: *mut ipuv3_channel);
}
extern "C" {
    pub fn ipu_cpmem_set_resolution(ch: *mut ipuv3_channel, xres: c_int, yres: c_int);
}
extern "C" {
    pub fn ipu_cpmem_skip_odd_chroma_rows(ch: *mut ipuv3_channel);
}
extern "C" {
    pub fn ipu_cpmem_set_stride(ch: *mut ipuv3_channel, stride: c_int);
}
extern "C" {
    pub fn ipu_cpmem_set_high_priority(ch: *mut ipuv3_channel);
}
extern "C" {
    pub fn ipu_cpmem_set_buffer(ch: *mut ipuv3_channel, bufnum: c_int, buf: dma_addr_t);
}
extern "C" {
    pub fn ipu_cpmem_set_uv_offset(ch: *mut ipuv3_channel, u_off: u32, v_off: u32);
}
extern "C" {
    pub fn ipu_cpmem_set_axi_id(ch: *mut ipuv3_channel, id: u32);
}
extern "C" {
    pub fn ipu_cpmem_set_burstsize(ch: *mut ipuv3_channel, burstsize: c_int);
}
extern "C" {
    pub fn ipu_cpmem_set_block_mode(ch: *mut ipuv3_channel);
}
extern "C" {
    pub fn ipu_cpmem_set_format_passthrough(ch: *mut ipuv3_channel, width: c_int) -> c_int;
}
extern "C" {
    pub fn ipu_cpmem_set_fmt(ch: *mut ipuv3_channel, drm_fourcc: u32) -> c_int;
}
extern "C" {
    pub fn ipu_cpmem_set_image(ch: *mut ipuv3_channel, image: *mut ipu_image) -> c_int;
}
extern "C" {
    pub fn ipu_cpmem_dump(ch: *mut ipuv3_channel);
}
//
// IPU Display Controller (dc) functions
//
extern "C" {
    pub fn ipu_dc_put(dc: *mut ipu_dc);
}
extern "C" {
    pub fn ipu_dc_enable(ipu: *mut ipu_soc);
}
extern "C" {
    pub fn ipu_dc_enable_channel(dc: *mut ipu_dc);
}
extern "C" {
    pub fn ipu_dc_disable_channel(dc: *mut ipu_dc);
}
extern "C" {
    pub fn ipu_dc_disable(ipu: *mut ipu_soc);
}
//
// IPU Display Interface (di) functions
//
extern "C" {
    pub fn ipu_di_put(: *mut ipu_di);
}
extern "C" {
    pub fn ipu_di_disable(: *mut ipu_di) -> c_int;
}
extern "C" {
    pub fn ipu_di_enable(: *mut ipu_di) -> c_int;
}
extern "C" {
    pub fn ipu_di_get_num(: *mut ipu_di) -> c_int;
}
extern "C" {
    pub fn ipu_di_adjust_videomode(di: *mut ipu_di, mode: *mut videomode) -> c_int;
}
extern "C" {
    pub fn ipu_di_init_sync_panel(: *mut ipu_di, sig: *mut ipu_di_signal_cfg) -> c_int;
}
//
// IPU Display Multi FIFO Controller (dmfc) functions
//
extern "C" {
    pub fn ipu_dmfc_enable_channel(dmfc: *mut dmfc_channel) -> c_int;
}
extern "C" {
    pub fn ipu_dmfc_disable_channel(dmfc: *mut dmfc_channel);
}
extern "C" {
    pub fn ipu_dmfc_config_wait4eot(dmfc: *mut dmfc_channel, width: c_int);
}
extern "C" {
    pub fn ipu_dmfc_put(dmfc: *mut dmfc_channel);
}
//
// IPU Display Processor (dp) functions
//
pub const IPU_DP_FLOW_SYNC_BG: c_int = 0;
pub const IPU_DP_FLOW_SYNC_FG: c_int = 1;
pub const IPU_DP_FLOW_ASYNC0_BG: c_int = 2;
pub const IPU_DP_FLOW_ASYNC0_FG: c_int = 3;
pub const IPU_DP_FLOW_ASYNC1_BG: c_int = 4;
pub const IPU_DP_FLOW_ASYNC1_FG: c_int = 5;
extern "C" {
    pub fn ipu_dp_put(: *mut ipu_dp);
}
extern "C" {
    pub fn ipu_dp_enable(ipu: *mut ipu_soc) -> c_int;
}
extern "C" {
    pub fn ipu_dp_enable_channel(dp: *mut ipu_dp) -> c_int;
}
extern "C" {
    pub fn ipu_dp_disable_channel(dp: *mut ipu_dp, sync: bool);
}
extern "C" {
    pub fn ipu_dp_disable(ipu: *mut ipu_soc);
}
extern "C" {
    pub fn ipu_dp_set_window_pos(: *mut ipu_dp, x_pos: u16, y_pos: u16) -> c_int;
}
//
// IPU Prefetch Resolve Gasket (prg) functions
//
extern "C" {
    pub fn ipu_prg_max_active_channels() -> c_int;
}
extern "C" {
    pub fn ipu_prg_present(ipu: *mut ipu_soc) -> bool;
}
extern "C" {
    pub fn ipu_prg_enable(ipu: *mut ipu_soc) -> c_int;
}
extern "C" {
    pub fn ipu_prg_disable(ipu: *mut ipu_soc);
}
extern "C" {
    pub fn ipu_prg_channel_disable(ipu_chan: *mut ipuv3_channel);
}
extern "C" {
    pub fn ipu_prg_channel_configure_pending(ipu_chan: *mut ipuv3_channel) -> bool;
}
//
// IPU CMOS Sensor Interface (csi) functions
//
extern "C" {
    pub fn ipu_csi_set_window(csi: *mut ipu_csi, w: *mut v4l2_rect);
}
extern "C" {
    pub fn ipu_csi_set_downsize(csi: *mut ipu_csi, horiz: bool, vert: bool);
}
extern "C" {
    pub fn ipu_csi_set_dest(csi: *mut ipu_csi, csi_dest: ipu_csi_dest) -> c_int;
}
extern "C" {
    pub fn ipu_csi_enable(csi: *mut ipu_csi) -> c_int;
}
extern "C" {
    pub fn ipu_csi_disable(csi: *mut ipu_csi) -> c_int;
}
extern "C" {
    pub fn ipu_csi_put(csi: *mut ipu_csi);
}
extern "C" {
    pub fn ipu_csi_dump(csi: *mut ipu_csi);
}
//
// IPU Image Converter (ic) functions
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipu_ic_task {
    IC_TASK_ENCODER,
    IC_TASK_VIEWFINDER,
    IC_TASK_POST_PROCESSOR,
    IC_NUM_TASKS,
}

//
// The parameters that describe a colorspace according to the
// Image Converter:
// - Y'CbCr encoding
// - quantization
// - "colorspace" (RGB or YUV).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu_ic_colorspace {
    pub enc: v4l2_ycbcr_encoding,
    pub quant: v4l2_quantization,
    pub cs: ipu_color_space,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu_ic_csc_params {
    pub /: *mut *mut s16 coeff[3][3]; / signed 9-bit integer coefficients,
    pub /: *mut *mut s16 offset[3]; / signed 11+2-bit fixed point offset,
    pub /: *mut *mut *mut u8 scale:2; / scale coefficients  2^(scale-1),
    pub /: *mut *mut bool sat:1; / saturate to (16, 235(Y) / 240(U, V)),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu_ic_csc {
    pub in_cs: ipu_ic_colorspace,
    pub out_cs: ipu_ic_colorspace,
    pub params: ipu_ic_csc_params,
}

extern "C" {
    pub fn __ipu_ic_calc_csc(csc: *mut ipu_ic_csc) -> c_int;
}
extern "C" {
    pub fn ipu_ic_task_enable(ic: *mut ipu_ic);
}
extern "C" {
    pub fn ipu_ic_task_disable(ic: *mut ipu_ic);
}
extern "C" {
    pub fn ipu_ic_enable(ic: *mut ipu_ic) -> c_int;
}
extern "C" {
    pub fn ipu_ic_disable(ic: *mut ipu_ic) -> c_int;
}
extern "C" {
    pub fn ipu_ic_put(ic: *mut ipu_ic);
}
extern "C" {
    pub fn ipu_ic_dump(ic: *mut ipu_ic);
}
//
// IPU Video De-Interlacer (vdi) functions
//
extern "C" {
    pub fn ipu_vdi_set_field_order(vdi: *mut ipu_vdi, std: v4l2_std_id, field: u32);
}
extern "C" {
    pub fn ipu_vdi_set_motion(vdi: *mut ipu_vdi, motion_sel: ipu_motion_sel);
}
extern "C" {
    pub fn ipu_vdi_setup(vdi: *mut ipu_vdi, code: u32, xres: c_int, yres: c_int);
}
extern "C" {
    pub fn ipu_vdi_enable(vdi: *mut ipu_vdi) -> c_int;
}
extern "C" {
    pub fn ipu_vdi_disable(vdi: *mut ipu_vdi) -> c_int;
}
extern "C" {
    pub fn ipu_vdi_put(vdi: *mut ipu_vdi);
}
//
// IPU Sensor Multiple FIFO Controller (SMFC) functions
//
extern "C" {
    pub fn ipu_smfc_put(smfc: *mut ipu_smfc);
}
extern "C" {
    pub fn ipu_smfc_enable(smfc: *mut ipu_smfc) -> c_int;
}
extern "C" {
    pub fn ipu_smfc_disable(smfc: *mut ipu_smfc) -> c_int;
}
extern "C" {
    pub fn ipu_smfc_map_channel(smfc: *mut ipu_smfc, csi_id: c_int, mipi_id: c_int) -> c_int;
}
extern "C" {
    pub fn ipu_smfc_set_burstsize(smfc: *mut ipu_smfc, burstsize: c_int) -> c_int;
}
extern "C" {
    pub fn ipu_smfc_set_watermark(smfc: *mut ipu_smfc, set_level: u32, clr_level: u32) -> c_int;
}
extern "C" {
    pub fn ipu_drm_fourcc_to_colorspace(drm_fourcc: u32) -> ipu_color_space;
}
extern "C" {
    pub fn ipu_pixelformat_to_colorspace(pixelformat: u32) -> ipu_color_space;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu_client_platformdata {
    pub csi: c_int,
    pub di: c_int,
    pub dc: c_int,
    pub dp: c_int,
    pub dma: [c_int; 2],
    pub of_node: *mut device_node,
}
