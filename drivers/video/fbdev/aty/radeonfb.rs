//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/aty/radeonfb.h
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

pub const DEBUG: c_int = 1;

//
// Most of the definitions here are adapted right from XFree86
//
// Chip families. Must fit in the low 16 bits of a long word
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum radeon_family {
    CHIP_FAMILY_UNKNOW,
    CHIP_FAMILY_LEGACY,
    CHIP_FAMILY_RADEON,
    CHIP_FAMILY_RV100,
    CHIP_FAMILY_RS100,    /* U1 (IGP320M) or A3 (IGP320)*/
    CHIP_FAMILY_RV200,
    CHIP_FAMILY_RS200,    /* U2 (IGP330M/340M/350M) or A4 (IGP330/340/345/350),
    RS250 (IGP 7000) */
    CHIP_FAMILY_R200,
    CHIP_FAMILY_RV250,
    CHIP_FAMILY_RS300,    /* Radeon 9000 IGP */
    CHIP_FAMILY_RV280,
    CHIP_FAMILY_R300,
    CHIP_FAMILY_R350,
    CHIP_FAMILY_RV350,
    CHIP_FAMILY_RV380,    /* RV370/RV380/M22/M24 */
    CHIP_FAMILY_R420,     /* R420/R423/M18 */
    CHIP_FAMILY_RC410,
    CHIP_FAMILY_RS400,
    CHIP_FAMILY_RS480,
    CHIP_FAMILY_LAST,
}

//
// Chip flags
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum radeon_chip_flags {
    CHIP_FAMILY_MASK	= 0x0000ffffUL,
    CHIP_FLAGS_MASK		= 0xffff0000UL,
    CHIP_IS_MOBILITY	= 0x00010000UL,
    CHIP_IS_IGP		= 0x00020000UL,
    CHIP_HAS_CRTC2		= 0x00040000UL,
}

//
// Errata workarounds
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum radeon_errata {
    CHIP_ERRATA_R300_CG		= 0x00000001,
    CHIP_ERRATA_PLL_DUMMYREADS	= 0x00000002,
    CHIP_ERRATA_PLL_DELAY		= 0x00000004,
}

//
// Monitor types
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum radeon_montype {
    MT_NONE = 0,
    MT_CRT,		/* CRT */
    MT_LCD,		/* LCD */
    MT_DFP,		/* DVI */
    MT_CTV,		/* composite TV */
    MT_STV		/* S-Video out */
}

//
// DDC i2c ports
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ddc_type {
    ddc_none,
    ddc_monid,
    ddc_dvi,
    ddc_vga,
    ddc_crt2,
}

//
// Connector types
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum conn_type {
    conn_none,
    conn_proprietary,
    conn_crt,
    conn_DVI_I,
    conn_DVI_D,
}

//
// PLL infos
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pll_info {
    pub ppll_max: c_int,
    pub ppll_min: c_int,
    pub mclk: int sclk,,
    pub ref_div: c_int,
    pub ref_clk: c_int,
}

//
// This structure contains the various registers manipulated by this
// driver for setting or restoring a mode. It's mostly copied from
// XFree's RADEONSaveRec structure. A few chip settings might still be
// tweaked without beeing reflected or saved in these registers though
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_regs {
// Common registers
    pub ovr_clr: u32,
    pub ovr_wid_left_right: u32,
    pub ovr_wid_top_bottom: u32,
    pub ov0_scale_cntl: u32,
    pub mpp_tb_config: u32,
    pub mpp_gp_config: u32,
    pub subpic_cntl: u32,
    pub viph_control: u32,
    pub i2c_cntl_1: u32,
    pub gen_int_cntl: u32,
    pub cap0_trig_cntl: u32,
    pub cap1_trig_cntl: u32,
    pub bus_cntl: u32,
    pub surface_cntl: u32,
    pub bios_5_scratch: u32,
// Other registers to save for VT switches or driver load/unload
    pub dp_datatype: u32,
    pub rbbm_soft_reset: u32,
    pub clock_cntl_index: u32,
    pub amcgpio_en_reg: u32,
    pub amcgpio_mask: u32,
// Surface/tiling registers
    pub surf_lower_bound: [u32; 8],
    pub surf_upper_bound: [u32; 8],
    pub surf_info: [u32; 8],
// CRTC registers
    pub crtc_gen_cntl: u32,
    pub crtc_ext_cntl: u32,
    pub dac_cntl: u32,
    pub crtc_h_total_disp: u32,
    pub crtc_h_sync_strt_wid: u32,
    pub crtc_v_total_disp: u32,
    pub crtc_v_sync_strt_wid: u32,
    pub crtc_offset: u32,
    pub crtc_offset_cntl: u32,
    pub crtc_pitch: u32,
    pub disp_merge_cntl: u32,
    pub grph_buffer_cntl: u32,
    pub crtc_more_cntl: u32,
// CRTC2 registers
    pub crtc2_gen_cntl: u32,
    pub dac2_cntl: u32,
    pub disp_output_cntl: u32,
    pub disp_hw_debug: u32,
    pub disp2_merge_cntl: u32,
    pub grph2_buffer_cntl: u32,
    pub crtc2_h_total_disp: u32,
    pub crtc2_h_sync_strt_wid: u32,
    pub crtc2_v_total_disp: u32,
    pub crtc2_v_sync_strt_wid: u32,
    pub crtc2_offset: u32,
    pub crtc2_offset_cntl: u32,
    pub crtc2_pitch: u32,
// Flat panel regs
    pub fp_crtc_h_total_disp: u32,
    pub fp_crtc_v_total_disp: u32,
    pub fp_gen_cntl: u32,
    pub fp2_gen_cntl: u32,
    pub fp_h_sync_strt_wid: u32,
    pub fp2_h_sync_strt_wid: u32,
    pub fp_horz_stretch: u32,
    pub fp_panel_cntl: u32,
    pub fp_v_sync_strt_wid: u32,
    pub fp2_v_sync_strt_wid: u32,
    pub fp_vert_stretch: u32,
    pub lvds_gen_cntl: u32,
    pub lvds_pll_cntl: u32,
    pub tmds_crc: u32,
    pub tmds_transmitter_cntl: u32,
// Computed values for PLL
    pub dot_clock_freq: u32,
    pub feedback_div: c_int,
    pub post_div: c_int,
// PLL registers
    pub ppll_div_3: u32,
    pub ppll_ref_div: u32,
    pub vclk_ecp_cntl: u32,
    pub clk_cntl_index: u32,
// Computed values for PLL2
    pub dot_clock_freq_2: u32,
    pub feedback_div_2: c_int,
    pub post_div_2: c_int,
// PLL2 registers
    pub p2pll_ref_div: u32,
    pub p2pll_div_0: u32,
    pub htotal_cntl2: u32,
// Palette
    pub palette_valid: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct panel_info {
    pub yres: int xres,,
    pub valid: c_int,
    pub clock: c_int,
    pub hblank: int hOver_plus, hSync_width,,
    pub vblank: int vOver_plus, vSync_width,,
    pub interlaced: int hAct_high, vAct_high,,
    pub pwr_delay: c_int,
    pub use_bios_dividers: c_int,
    pub ref_divider: c_int,
    pub post_divider: c_int,
    pub fbk_divider: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_i2c_chan {
    pub rinfo: *mut radeonfb_info,
    pub ddc_reg: u32,
    pub adapter: i2c_adapter,
    pub algo: i2c_algo_bit_data,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum radeon_pm_mode {
    radeon_pm_none	= 0,		/* Nothing supported */
    radeon_pm_d2	= 0x00000001,	/* Can do D2 state */
    radeon_pm_off	= 0x00000002,	/* Can resume from D3 cold */
}

extern "C" {
    pub fn void(rinfo: *mut *mut reinit_function_ptr)(struct radeonfb_info) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeonfb_info {
    pub info: *mut fb_info,
    pub state: radeon_regs,
    pub init_state: radeon_regs,
    pub name: [c_char; 50],
    pub mmio_base_phys: c_ulong,
    pub fb_base_phys: c_ulong,
    pub mmio_base: *mut void __iomem,
    pub fb_base: *mut void __iomem,
    pub fb_local_base: c_ulong,
    pub pdev: *mut pci_dev,

    pub of_node: *mut device_node,

    pub bios_seg: *mut void __iomem,
    pub fp_bios_start: c_int,
    pub pseudo_palette: [u32; 16],
    pub }: { u8 red, green, blue, pad;,
    pub chipset: c_int,
    pub family: u8,
    pub rev: u8,
    pub errata: c_uint,
    pub video_ram: c_ulong,
    pub mapped_vram: c_ulong,
    pub vram_width: c_int,
    pub vram_ddr: c_int,
    pub depth: int pitch, bpp,,
    pub has_CRTC2: c_int,
    pub is_mobility: c_int,
    pub is_IGP: c_int,
    pub reversed_DAC: c_int,
    pub reversed_TMDS: c_int,
    pub panel_info: panel_info,
    pub mon1_type: c_int,
    pub mon1_EDID: *mut u8,
    pub mon1_modedb: *mut fb_videomode,
    pub mon1_dbsize: c_int,
    pub mon2_type: c_int,
    pub mon2_EDID: *mut u8,
    pub dp_gui_master_cntl: u32,
    pub pll: pll_info,
    pub wc_cookie: c_int,
    pub save_regs: [u32; 100],
    pub asleep: c_int,
    pub lock_blank: c_int,
    pub dynclk: c_int,
    pub no_schedule: c_int,
    pub pm_mode: radeon_pm_mode,
    pub reinit_func: reinit_function_ptr,
// Lock on register access
    pub reg_lock: spinlock_t,
// Timer used for delayed LVDS operations
    pub lvds_timer: timer_list,
    pub pending_lvds_gen_cntl: u32,
    pub i2c: [radeon_i2c_chan; 4],
}

//
// IO macros
//
extern "C" {
    pub fn _radeon_msleep(rinfo: *mut radeonfb_info, ms: c_ulong);
}

extern "C" {
    pub fn _OUTREGP(rinfo: *mut radeonfb_info, addr: u32, val: u32, mask: u32);
}

//
// Note about PLL register accesses:
//
// I have removed the spinlock on them on purpose. The driver now
// expects that it will only manipulate the PLL registers in normal
// task environment, where radeon_msleep() will be called, protected
// by a semaphore (currently the console semaphore) so that no conflict
// will happen on the PLL register index.
//
// With the latest changes to the VT layer, this is guaranteed for all
// calls except the actual drawing/blits which aren't supposed to use
// the PLL registers anyway
//
// This is very important for the workarounds to work properly. The only
// possible exception to this rule is the call to unblank(), which may
// be done at irq time if an oops is in progress.
//
extern "C" {
    pub fn radeon_pll_errata_after_index_slow(rinfo: *mut radeonfb_info);
}
extern "C" {
    pub fn radeon_pll_errata_after_data_slow(rinfo: *mut radeonfb_info);
}
extern "C" {
    pub fn __INPLL(rinfo: *mut radeonfb_info, addr: u32) -> u32;
}
extern "C" {
    pub fn __OUTPLL(rinfo: *mut radeonfb_info, index: c_uint, val: u32);
}

//
// Inline utilities
//
// 2D Engine helper routines
//
extern "C" {
    pub fn _radeon_fifo_wait(rinfo: *mut radeonfb_info, entries: c_int);
}
extern "C" {
    pub fn radeon_engine_flush(rinfo: *mut radeonfb_info);
}
extern "C" {
    pub fn _radeon_engine_idle(rinfo: *mut radeonfb_info);
}

// I2C Functions
extern "C" {
    pub fn radeon_create_i2c_busses(rinfo: *mut radeonfb_info);
}
extern "C" {
    pub fn radeon_delete_i2c_busses(rinfo: *mut radeonfb_info);
}
extern "C" {
    pub fn radeon_probe_i2c_connector(rinfo: *mut radeonfb_info, conn: c_int, out_edid: *mut u8) -> c_int;
}
// PM Functions
extern "C" {
    pub fn radeonfb_pm_init(rinfo: *mut radeonfb_info, dynclk: c_int, ignore_devlist: c_int, force_sleep: c_int);
}
extern "C" {
    pub fn radeonfb_pm_exit(rinfo: *mut radeonfb_info);
}
// Monitor probe functions
extern "C" {
    pub fn radeon_check_modes(rinfo: *mut radeonfb_info, mode_option: *const c_char);
}
// Accel functions
extern "C" {
    pub fn radeonfb_fillrect(info: *mut fb_info, region: *const fb_fillrect);
}
extern "C" {
    pub fn radeonfb_copyarea(info: *mut fb_info, area: *const fb_copyarea);
}
extern "C" {
    pub fn radeonfb_imageblit(p: *mut fb_info, image: *const fb_image);
}
extern "C" {
    pub fn radeonfb_sync(info: *mut fb_info) -> c_int;
}
extern "C" {
    pub fn radeonfb_engine_init(rinfo: *mut radeonfb_info);
}
extern "C" {
    pub fn radeonfb_engine_reset(rinfo: *mut radeonfb_info);
}
// Other functions
extern "C" {
    pub fn radeon_screen_blank(rinfo: *mut radeonfb_info, blank: c_int, mode_switch: c_int) -> c_int;
}
// Backlight functions

extern "C" {
    pub fn radeonfb_bl_init(rinfo: *mut radeonfb_info);
}
extern "C" {
    pub fn radeonfb_bl_exit(rinfo: *mut radeonfb_info);
}

