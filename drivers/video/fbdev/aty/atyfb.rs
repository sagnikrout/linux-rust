//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/aty/atyfb.h
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
// ATI Frame Buffer Device Driver Core Definitions
//

//
// Elements of the hardware specific atyfb_par structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crtc {
    pub vxres: u32,
    pub vyres: u32,
    pub xoffset: u32,
    pub yoffset: u32,
    pub bpp: u32,
    pub h_tot_disp: u32,
    pub h_sync_strt_wid: u32,
    pub v_tot_disp: u32,
    pub v_sync_strt_wid: u32,
    pub vline_crnt_vline: u32,
    pub off_pitch: u32,
    pub gen_cntl: u32,
    pub /: *mut *mut u32 dp_pix_width; / acceleration,
    pub /: *mut *mut u32 dp_chain_mask; / acceleration,

    pub horz_stretching: u32,
    pub vert_stretching: u32,
    pub ext_vert_stretch: u32,
    pub shadow_h_tot_disp: u32,
    pub shadow_h_sync_strt_wid: u32,
    pub shadow_v_tot_disp: u32,
    pub shadow_v_sync_strt_wid: u32,
    pub lcd_gen_cntl: u32,
    pub lcd_config_panel: u32,
    pub lcd_index: u32,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aty_interrupt {
    pub wait: wait_queue_head_t,
    pub count: c_uint,
    pub pan_display: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pll_info {
    pub pll_max: c_int,
    pub pll_min: c_int,
    pub xclk: int sclk, mclk, mclk_pm,,
    pub ref_div: c_int,
    pub ref_clk: c_int,
    pub ecp_max: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pll_514 {
    pub m: u8,
    pub n: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pll_18818 {
    pub program_bits: u32,
    pub locationAddr: u32,
    pub period_in_ps: u32,
    pub post_divider: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pll_ct {
    pub pll_ref_div: u8,
    pub pll_gen_cntl: u8,
    pub mclk_fb_div: u8,
    pub /: *mut *mut u8 mclk_fb_mult; / 2 ro 4,
    pub sclk_fb_div: u8,
    pub pll_vclk_cntl: u8,
    pub vclk_post_div: u8,
    pub vclk_fb_div: u8,
    pub pll_ext_cntl: u8,
    pub ext_vpll_cntl: u8,
    pub spll_cntl2: u8,
    pub /: *mut *mut u32 dsp_config; / Mach64 GTB DSP,
    pub /: *mut *mut u32 dsp_on_off; / Mach64 GTB DSP,
    pub dsp_loop_latency: u32,
    pub fifo_size: u32,
    pub xclkpagefaultdelay: u32,
    pub xclkmaxrasdelay: u32,
    pub xclk_ref_div: u8,
    pub xclk_post_div: u8,
    pub mclk_post_div_real: u8,
    pub xclk_post_div_real: u8,
    pub vclk_post_div_real: u8,
    pub features: u8,

    pub /: *mut *mut u32 xres; / use for LCD stretching/scaling,

}

//
pub const DONT_USE_SPLL: c_uint = 0x1;
pub const DONT_USE_XDLL: c_uint = 0x2;
pub const USE_CPUCLK: c_uint = 0x4;
pub const POWERDOWN_PLL: c_uint = 0x8;
#[repr(C)]
#[derive(Copy, Clone)]
pub union aty_pll {
    pub ct: pll_ct,
    pub ibm514: pll_514,
    pub ics2595: pll_18818,
}

//
// The hardware parameters for each card
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atyfb_par {
    pub pseudo_palette: [u32; 16],
    pub palette: [{ u8 red, green, blue; }; 256],
    pub dac_ops: *const aty_dac_ops,
    pub pll_ops: *const aty_pll_ops,
    pub ati_regbase: *mut void __iomem,
    pub /: *mut *mut unsigned long clk_wr_offset; / meaning overloaded, clock id by CT,
    pub crtc: crtc,
    pub pll: aty_pll,
    pub pll_limits: pll_info,
    pub features: u32,
    pub ref_clk_per: u32,
    pub pll_per: u32,
    pub mclk_per: u32,
    pub xclk_per: u32,
    pub bus_type: u8,
    pub ram_type: u8,
    pub mem_refresh_rate: u8,
    pub pci_id: u16,
    pub accel_flags: u32,
    pub blitter_may_be_busy: c_int,
    pub fifo_space: unsigned,
    pub asleep: c_int,
    pub lock_blank: c_int,
    pub res_start: c_ulong,
    pub res_size: c_ulong,
    pub pdev: *mut pci_dev,

    pub mmap_map: *mut pci_mmap_map,
    pub mmaped: u8,

    pub open: c_int,

    pub bios_base_phys: c_ulong,
    pub bios_base: c_ulong,
    pub lcd_table: c_ulong,
    pub lcd_width: u16,
    pub lcd_height: u16,
    pub lcd_pixclock: u32,
    pub lcd_refreshrate: u16,
    pub lcd_htotal: u16,
    pub lcd_hdisp: u16,
    pub lcd_hsync_dly: u16,
    pub lcd_hsync_len: u16,
    pub lcd_vtotal: u16,
    pub lcd_vdisp: u16,
    pub lcd_vsync_len: u16,
    pub lcd_right_margin: u16,
    pub lcd_lower_margin: u16,
    pub lcd_hblank_len: u16,
    pub lcd_vblank_len: u16,

    pub /: *mut *mut unsigned long aux_start; / auxiliary aperture,
    pub aux_size: c_ulong,
    pub vblank: aty_interrupt,
    pub irq_flags: c_ulong,
    pub irq: c_uint,
    pub int_lock: spinlock_t,
    pub wc_cookie: c_int,
    pub mem_cntl: u32,
    pub saved_crtc: crtc,
    pub saved_pll: aty_pll,
}

//
// ATI Mach64 features
//

pub const M64F_RESET_3D: c_uint = 0x00000001;
pub const M64F_MAGIC_FIFO: c_uint = 0x00000002;
pub const M64F_GTB_DSP: c_uint = 0x00000004;
pub const M64F_FIFO_32: c_uint = 0x00000008;
pub const M64F_SDRAM_MAGIC_PLL: c_uint = 0x00000010;
pub const M64F_MAGIC_POSTDIV: c_uint = 0x00000020;
pub const M64F_INTEGRATED: c_uint = 0x00000040;
pub const M64F_CT_BUS: c_uint = 0x00000080;
pub const M64F_VT_BUS: c_uint = 0x00000100;
pub const M64F_MOBIL_BUS: c_uint = 0x00000200;
pub const M64F_GX: c_uint = 0x00000400;
pub const M64F_CT: c_uint = 0x00000800;
pub const M64F_VT: c_uint = 0x00001000;
pub const M64F_GT: c_uint = 0x00002000;
pub const M64F_MAGIC_VRAM_SIZE: c_uint = 0x00004000;
pub const M64F_G3_PB_1_1: c_uint = 0x00008000;
pub const M64F_G3_PB_1024x768: c_uint = 0x00010000;
pub const M64F_EXTRA_BRIGHT: c_uint = 0x00020000;
pub const M64F_LT_LCD_REGS: c_uint = 0x00040000;
pub const M64F_XL_DLL: c_uint = 0x00080000;
pub const M64F_MFB_FORCE_4: c_uint = 0x00100000;
pub const M64F_HW_TRIPLE: c_uint = 0x00200000;
pub const M64F_XL_MEM: c_uint = 0x00400000;
//
// Register access
//
// Hack for bloc 1, should be cleanly optimized by compiler

extern "C" {
    pub fn in_le32(regindex: par->ati_regbase +) -> return;
}

extern "C" {
    pub fn readl(regindex: par->ati_regbase +) -> return;
}

// Hack for bloc 1, should be cleanly optimized by compiler

// Hack for bloc 1, should be cleanly optimized by compiler

// Hack for bloc 1, should be cleanly optimized by compiler

extern "C" {
    pub fn in_8(regindex: par->ati_regbase +) -> return;
}

extern "C" {
    pub fn readb(regindex: par->ati_regbase +) -> return;
}

// Hack for bloc 1, should be cleanly optimized by compiler

extern "C" {
    pub fn aty_st_lcd(index: c_int, val: u32, par: *const atyfb_par);
}
extern "C" {
    pub fn aty_ld_lcd(index: c_int, par: *const atyfb_par) -> u32;
}
//
// DAC operations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aty_dac_ops {
    pub accel): *const *const aty_pll  pll, u32 bpp, u32,
}

//
// Clock operations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aty_pll_ops {
    pub pll): *const *const *const *const int (var_to_pll) (struct fb_info  info, u32 vclk_per, u32 bpp, union aty_pll,
    pub pll): *const *const *const *const u32 (pll_to_var) (struct fb_info  info, union aty_pll,
    pub pll): *const *const *const *const void (set_pll) (struct fb_info  info, union aty_pll,
    pub pll): *const *const *const *const void (get_pll) (struct fb_info info, union aty_pll,
    pub pll): *const *const *const *const int (init_pll) (struct fb_info  info, union aty_pll,
    pub pll): *const *const *const void (resume_pll)(struct fb_info info, union aty_pll,
}

extern "C" {
    pub fn aty_set_pll_ct(info: *const fb_info, pll: *const aty_pll);
}
extern "C" {
    pub fn aty_ld_pll_ct(offset: c_int, par: *const atyfb_par) -> u8;
}
//
// Hardware cursor support
//
extern "C" {
    pub fn aty_init_cursor(info: *mut fb_info, atyfb_ops: *mut fb_ops) -> c_int;
}
//
// Hardware acceleration
//
extern "C" {
    pub fn aty_reset_engine(par: *mut atyfb_par);
}
extern "C" {
    pub fn aty_init_engine(par: *mut atyfb_par, info: *mut fb_info);
}
extern "C" {
    pub fn atyfb_copyarea(info: *mut fb_info, area: *const fb_copyarea);
}
extern "C" {
    pub fn atyfb_fillrect(info: *mut fb_info, rect: *const fb_fillrect);
}
extern "C" {
    pub fn atyfb_imageblit(info: *mut fb_info, image: *const fb_image);
}
