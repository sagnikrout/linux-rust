//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/arm/malidp_hw.h
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
// (C) COPYRIGHT 2013-2016 ARM Limited. All rights reserved.
//
// ARM Mali DP hardware manipulation routines.
//

// Mali DP IP blocks
// Mali DP layer IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rotation_features {
    ROTATE_NONE,		/* does not support rotation at all */
    ROTATE_ANY,		/* supports rotation on any buffers */
    ROTATE_COMPRESSED,	/* supports rotation only on compressed buffers */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct malidp_format_id {
    pub /: *mut *mut u32 format; / DRM fourcc,
    pub /: *mut *mut u8 layer; / bitmask of layers supporting it,
    pub /: *mut *mut u8 id; / used internally,
}

pub const MALIDP_INVALID_FORMAT_ID: c_uint = 0xff;
//
// hide the differences between register maps
// by using a common structure to hold the
// base register offsets
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct malidp_irq_map {
    pub /: *mut *mut u32 irq_mask; / mask of IRQs that can be enabled in the block,
    pub /: *mut *mut u32 vsync_irq; / IRQ bit used for signaling during VSYNC,
    pub /: *mut *mut u32 err_mask; / mask of bits that represent errors,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct malidp_layer {
    pub /: *mut *mut u16 id; / layer ID,
    pub /: *mut *mut u16 base; / address offset for the register bank,
    pub /: *mut *mut u16 ptr; / address offset for the pointer register,
    pub /: *mut *mut u16 stride_offset; / offset to the first stride register.,
    pub /: *mut *mut s16 yuv2rgb_offset; / offset to the YUV->RGB matrix entries,
    pub /: *mut *mut u16 mmu_ctrl_offset; / offset to the MMU control register,
    pub /: *mut *mut rotation_features rot; / type of rotation supported,
// address offset for the AFBC decoder registers
    pub afbc_decoder_offset: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum malidp_scaling_coeff_set {
    MALIDP_UPSCALING_COEFFS = 1,
    MALIDP_DOWNSCALING_1_5_COEFFS = 2,
    MALIDP_DOWNSCALING_2_COEFFS = 3,
    MALIDP_DOWNSCALING_2_75_COEFFS = 4,
    MALIDP_DOWNSCALING_4_COEFFS = 5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct malidp_se_config {
    pub 1: u8 scale_enable :,
    pub 1: u8 enhancer_enable :,
    pub 3: u8 hcoeff :,
    pub 3: u8 vcoeff :,
    pub plane_src_id: u8,
    pub input_h: u16 input_w,,
    pub output_h: u16 output_w,,
    pub h_delta_phase: u32 h_init_phase,,
    pub v_delta_phase: u32 v_init_phase,,
}

// regmap features

#[repr(C)]
#[derive(Copy, Clone)]
pub struct malidp_hw_regmap {
// address offset of the DE register bank
// is always 0x0000
// address offset of the DE coefficients registers
    pub coeffs_base: u16,
// address offset of the SE registers bank
    pub se_base: u16,
// address offset of the DC registers bank
    pub dc_base: u16,
// address offset for the output depth register
    pub out_depth_base: u16,
// bitmap with register map features
    pub features: u8,
// list of supported layers
    pub n_layers: u8,
    pub layers: *const malidp_layer,
    pub de_irq_map: malidp_irq_map,
    pub se_irq_map: malidp_irq_map,
    pub dc_irq_map: malidp_irq_map,
// list of supported pixel formats for each layer
    pub pixel_formats: *const malidp_format_id,
    pub n_pixel_formats: u8,
// pitch alignment requirement in bytes
    pub bus_align_bytes: u8,
}

// device features
// Unlike DP550/650, DP500 has 3 stride registers in its video layer.

//
// Static structure containing hardware specific data and pointers to
// functions that behave differently between various versions of the IP.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct malidp_hw {
    pub map: malidp_hw_regmap,
//
// Validate the driver instance against the hardware bits
//
    pub hwdev): *mut *mut int (query_hw)(struct malidp_hw_device,
//
// Set the hardware into config mode, ready to accept mode changes
//
    pub hwdev): *mut *mut void (enter_config_mode)(struct malidp_hw_device,
//
// Tell hardware to exit configuration mode
//
    pub hwdev): *mut *mut void (leave_config_mode)(struct malidp_hw_device,
//
// Query if hardware is in configuration mode
//
    pub hwdev): *mut *mut bool (in_config_mode)(struct malidp_hw_device,
//
// Set/clear configuration valid flag for hardware parameters that can
// be changed outside the configuration mode to the given value.
// Hardware will use the new settings when config valid is set,
// after the end of the current buffer scanout, and will ignore
// any new values for those parameters if config valid flag is cleared
//
    pub value): *mut *mut *mut void (set_config_valid)(struct malidp_hw_device hwdev, u8,
//
// Set a new mode in hardware. Requires the hardware to be in
// configuration mode before this function is called.
//
    pub m): *mut *mut *mut void (modeset)(struct malidp_hw_device hwdev, struct videomode,
//
// Calculate the required rotation memory given the active area
// and the buffer format.
//
    pub has_modifier): u32 fmt, bool,
    pub old_config): *mut malidp_se_config,
    pub vm): *mut videomode,
//
// Enable writing to memory the content of the next frame
// @param hwdev - malidp_hw_device structure containing the HW description
// @param addrs - array of addresses for each plane
// @param pitches - array of pitches for each plane
// @param num_planes - number of planes to be written
// @param w - width of the output frame
// @param h - height of the output frame
// @param fmt_id - internal format ID of output buffer
//
    pub rgb2yuv_coeffs): *const i16,
//
// Disable the writing to memory of the next frame's content.
//
    pub hwdev): *mut *mut void (disable_memwrite)(struct malidp_hw_device,
    pub features: u8,
}

// Supported variants of the hardware
// keep the next entry last
//
// Structure used by the driver during runtime operation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct malidp_hw_device {
    pub hw: *mut malidp_hw,
    pub regs: *mut void __iomem,
// APB clock
    pub pclk: *mut clk,
// AXI clock
    pub aclk: *mut clk,
// main clock for display core
    pub mclk: *mut clk,
// pixel clock for display core
    pub pxlclk: *mut clk,
    pub min_line_size: u8,
    pub max_line_size: u16,
    pub output_color_depth: u32,
// track the device PM state
    pub pm_suspended: bool,
// track the SE memory writeback state
    pub mw_state: u8,
// size of memory used for rotating layers, up to two banks available
    pub rotation_memory: [u32; 2],
// priority level of RQOS register used for driven the ARQOS signal
    pub arqos_value: u32,
}

extern "C" {
    pub fn readl(reg: hwdev->regs +) -> return;
}
extern "C" {
    pub fn malidp_de_irq_init(drm: *mut drm_device, irq: c_int) -> c_int;
}
extern "C" {
    pub fn malidp_se_irq_hw_init(hwdev: *mut malidp_hw_device);
}
extern "C" {
    pub fn malidp_de_irq_hw_init(hwdev: *mut malidp_hw_device);
}
extern "C" {
    pub fn malidp_de_irq_fini(hwdev: *mut malidp_hw_device);
}
extern "C" {
    pub fn malidp_se_irq_init(drm: *mut drm_device, irq: c_int) -> c_int;
}
extern "C" {
    pub fn malidp_se_irq_fini(hwdev: *mut malidp_hw_device);
}
extern "C" {
    pub fn malidp_format_get_bpp(fmt: u32) -> c_int;
}
//
// only hardware that cannot do 8 bytes bus alignments have further
// constraints on rotated planes
//
// U16.16
pub const FP_1_00000: c_uint = 0x00010000	/* 1.0 */;
pub const FP_0_66667: c_uint = 0x0000AAAA	/* 0.6667 = 1/1.5 */;
pub const FP_0_50000: c_uint = 0x00008000	/* 0.5 = 1/2 */;
pub const FP_0_36363: c_uint = 0x00005D17	/* 0.36363 = 1/2.75 */;
pub const FP_0_25000: c_uint = 0x00004000	/* 0.25 = 1/4 */;

//
// background color components are defined as 12bits values,
// they will be shifted right when stored on hardware that
// supports only 8bits per channel
//
pub const MALIDP_BGND_COLOR_R: c_uint = 0x000;
pub const MALIDP_BGND_COLOR_G: c_uint = 0x000;
pub const MALIDP_BGND_COLOR_B: c_uint = 0x000;
pub const MALIDP_COLORADJ_NUM_COEFFS: c_int = 12;
pub const MALIDP_COEFFTAB_NUM_COEFFS: c_int = 64;
pub const MALIDP_GAMMA_LUT_SIZE: c_int = 4096;

