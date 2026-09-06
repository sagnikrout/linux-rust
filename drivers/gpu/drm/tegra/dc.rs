//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/tegra/dc.h
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
// Copyright (C) 2012 Avionic Design GmbH
// Copyright (C) 2012 NVIDIA CORPORATION.  All rights reserved.
//
pub const TEGRA_DC_H: c_int = 1;

pub const TEGRA_DC_LEGACY_PLANES_NUM: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_dc_state {
    pub base: drm_crtc_state,
    pub clk: *mut clk,
    pub pclk: c_ulong,
    pub div: c_uint,
    pub planes: u32,
}

extern "C" {
    pub fn container_of(_arg: state, tegra_dc_state: struct, _arg: base) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_dc_stats {
    pub frames: c_ulong,
    pub vblank: c_ulong,
    pub underflow: c_ulong,
    pub overflow: c_ulong,
    pub frames_total: c_ulong,
    pub vblank_total: c_ulong,
    pub underflow_total: c_ulong,
    pub overflow_total: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_windowgroup_soc {
    pub index: c_uint,
    pub dc: c_uint,
    pub windows: *const c_uint,
    pub num_windows: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_dc_soc_info {
    pub supports_background_color: bool,
    pub supports_interlacing: bool,
    pub supports_cursor: bool,
    pub supports_block_linear: bool,
    pub supports_sector_layout: bool,
    pub has_legacy_blending: bool,
    pub pitch_align: c_uint,
    pub has_powergate: bool,
    pub coupled_pm: bool,
    pub has_nvdisplay: bool,
    pub wgrps: *const tegra_windowgroup_soc,
    pub num_wgrps: c_uint,
    pub primary_formats: *const u32,
    pub num_primary_formats: c_uint,
    pub overlay_formats: *const u32,
    pub num_overlay_formats: c_uint,
    pub modifiers: *const u64,
    pub has_win_a_without_filters: bool,
    pub has_win_b_vfilter_mem_client: bool,
    pub has_win_c_without_vert_filter: bool,
    pub plane_tiled_memory_bandwidth_x2: bool,
    pub has_pll_d2_out0: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_dc {
    pub client: host1x_client,
    pub syncpt: *mut host1x_syncpt,
    pub dev: *mut device,
    pub base: drm_crtc,
    pub powergate: c_uint,
    pub pipe: c_int,
    pub clk: *mut clk,
    pub rst: *mut reset_control,
    pub regs: *mut void __iomem,
    pub irq: c_int,
    pub rgb: *mut tegra_output,
    pub pmc: *mut tegra_pmc,
    pub stats: tegra_dc_stats,
    pub list: list_head,
    pub debugfs_files: *mut drm_info_list,
    pub soc: *const tegra_dc_soc_info,
    pub has_opp_table: bool,
    pub cmu_output_lut: *mut u64,
    pub cmu_output_lut_phys: dma_addr_t,
}

extern "C" {
    pub fn container_of(_arg: client, tegra_dc: struct, _arg: client) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_dc_window {
    pub x: c_uint,
    pub y: c_uint,
    pub w: c_uint,
    pub h: c_uint,
    pub src: },
    pub x: c_uint,
    pub y: c_uint,
    pub w: c_uint,
    pub h: c_uint,
    pub dst: },
    pub bits_per_pixel: c_uint,
    pub stride: [c_uint; 2],
    pub base: [c_ulong; 3],
    pub zpos: c_uint,
    pub reflect_x: bool,
    pub reflect_y: bool,
    pub tiling: tegra_bo_tiling,
    pub format: u32,
    pub swap: u32,
}

// from dc.c
extern "C" {
    pub fn tegra_dc_has_output(dc: *mut tegra_dc, dev: *mut device) -> bool;
}
extern "C" {
    pub fn tegra_dc_commit(dc: *mut tegra_dc);
}
// from rgb.c
extern "C" {
    pub fn tegra_dc_rgb_probe(dc: *mut tegra_dc) -> c_int;
}
extern "C" {
    pub fn tegra_dc_rgb_remove(dc: *mut tegra_dc);
}
extern "C" {
    pub fn tegra_dc_rgb_init(drm: *mut drm_device, dc: *mut tegra_dc) -> c_int;
}
extern "C" {
    pub fn tegra_dc_rgb_exit(dc: *mut tegra_dc) -> c_int;
}
pub const DC_CMD_GENERAL_INCR_SYNCPT: c_uint = 0x000;
pub const DC_CMD_GENERAL_INCR_SYNCPT_CNTRL: c_uint = 0x001;

pub const DC_CMD_GENERAL_INCR_SYNCPT_ERROR: c_uint = 0x002;
pub const DC_CMD_WIN_A_INCR_SYNCPT: c_uint = 0x008;
pub const DC_CMD_WIN_A_INCR_SYNCPT_CNTRL: c_uint = 0x009;
pub const DC_CMD_WIN_A_INCR_SYNCPT_ERROR: c_uint = 0x00a;
pub const DC_CMD_WIN_B_INCR_SYNCPT: c_uint = 0x010;
pub const DC_CMD_WIN_B_INCR_SYNCPT_CNTRL: c_uint = 0x011;
pub const DC_CMD_WIN_B_INCR_SYNCPT_ERROR: c_uint = 0x012;
pub const DC_CMD_WIN_C_INCR_SYNCPT: c_uint = 0x018;
pub const DC_CMD_WIN_C_INCR_SYNCPT_CNTRL: c_uint = 0x019;
pub const DC_CMD_WIN_C_INCR_SYNCPT_ERROR: c_uint = 0x01a;
pub const DC_CMD_CONT_SYNCPT_VSYNC: c_uint = 0x028;

pub const DC_CMD_DISPLAY_COMMAND_OPTION0: c_uint = 0x031;
pub const DC_CMD_DISPLAY_COMMAND: c_uint = 0x032;

pub const DC_CMD_SIGNAL_RAISE: c_uint = 0x033;
pub const DC_CMD_DISPLAY_POWER_CONTROL: c_uint = 0x036;

pub const DC_CMD_INT_STATUS: c_uint = 0x037;
pub const DC_CMD_INT_MASK: c_uint = 0x038;
pub const DC_CMD_INT_ENABLE: c_uint = 0x039;
pub const DC_CMD_INT_TYPE: c_uint = 0x03a;
pub const DC_CMD_INT_POLARITY: c_uint = 0x03b;

pub const DC_CMD_SIGNAL_RAISE1: c_uint = 0x03c;
pub const DC_CMD_SIGNAL_RAISE2: c_uint = 0x03d;
pub const DC_CMD_SIGNAL_RAISE3: c_uint = 0x03e;
pub const DC_CMD_STATE_ACCESS: c_uint = 0x040;

pub const DC_CMD_STATE_CONTROL: c_uint = 0x041;

pub const DC_CMD_DISPLAY_WINDOW_HEADER: c_uint = 0x042;

pub const DC_CMD_REG_ACT_CONTROL: c_uint = 0x043;
pub const DC_COM_CRC_CONTROL: c_uint = 0x300;

pub const DC_COM_CRC_CHECKSUM: c_uint = 0x301;

pub const DC_COM_PIN_MISC_CONTROL: c_uint = 0x31b;
pub const DC_COM_PIN_PM0_CONTROL: c_uint = 0x31c;
pub const DC_COM_PIN_PM0_DUTY_CYCLE: c_uint = 0x31d;
pub const DC_COM_PIN_PM1_CONTROL: c_uint = 0x31e;
pub const DC_COM_PIN_PM1_DUTY_CYCLE: c_uint = 0x31f;
pub const DC_COM_SPI_CONTROL: c_uint = 0x320;
pub const DC_COM_SPI_START_BYTE: c_uint = 0x321;
pub const DC_COM_HSPI_WRITE_DATA_AB: c_uint = 0x322;
pub const DC_COM_HSPI_WRITE_DATA_CD: c_uint = 0x323;
pub const DC_COM_HSPI_CS_DC: c_uint = 0x324;
pub const DC_COM_SCRATCH_REGISTER_A: c_uint = 0x325;
pub const DC_COM_SCRATCH_REGISTER_B: c_uint = 0x326;
pub const DC_COM_GPIO_CTRL: c_uint = 0x327;
pub const DC_COM_GPIO_DEBOUNCE_COUNTER: c_uint = 0x328;
pub const DC_COM_CRC_CHECKSUM_LATCHED: c_uint = 0x329;
pub const DC_COM_RG_UNDERFLOW: c_uint = 0x365;

pub const DC_DISP_DISP_SIGNAL_OPTIONS0: c_uint = 0x400;

pub const DC_DISP_DISP_SIGNAL_OPTIONS1: c_uint = 0x401;
pub const DC_DISP_DISP_WIN_OPTIONS: c_uint = 0x402;

pub const DC_DISP_DISP_MEM_HIGH_PRIORITY: c_uint = 0x403;

pub const DC_DISP_DISP_MEM_HIGH_PRIORITY_TIMER: c_uint = 0x404;

pub const DC_DISP_DISP_TIMING_OPTIONS: c_uint = 0x405;

pub const DC_DISP_REF_TO_SYNC: c_uint = 0x406;
pub const DC_DISP_SYNC_WIDTH: c_uint = 0x407;
pub const DC_DISP_BACK_PORCH: c_uint = 0x408;
pub const DC_DISP_ACTIVE: c_uint = 0x409;
pub const DC_DISP_FRONT_PORCH: c_uint = 0x40a;
pub const DC_DISP_H_PULSE0_CONTROL: c_uint = 0x40b;
pub const DC_DISP_H_PULSE0_POSITION_A: c_uint = 0x40c;
pub const DC_DISP_H_PULSE0_POSITION_B: c_uint = 0x40d;
pub const DC_DISP_H_PULSE0_POSITION_C: c_uint = 0x40e;
pub const DC_DISP_H_PULSE0_POSITION_D: c_uint = 0x40f;
pub const DC_DISP_H_PULSE1_CONTROL: c_uint = 0x410;
pub const DC_DISP_H_PULSE1_POSITION_A: c_uint = 0x411;
pub const DC_DISP_H_PULSE1_POSITION_B: c_uint = 0x412;
pub const DC_DISP_H_PULSE1_POSITION_C: c_uint = 0x413;
pub const DC_DISP_H_PULSE1_POSITION_D: c_uint = 0x414;
pub const DC_DISP_H_PULSE2_CONTROL: c_uint = 0x415;
pub const DC_DISP_H_PULSE2_POSITION_A: c_uint = 0x416;
pub const DC_DISP_H_PULSE2_POSITION_B: c_uint = 0x417;
pub const DC_DISP_H_PULSE2_POSITION_C: c_uint = 0x418;
pub const DC_DISP_H_PULSE2_POSITION_D: c_uint = 0x419;
pub const DC_DISP_V_PULSE0_CONTROL: c_uint = 0x41a;
pub const DC_DISP_V_PULSE0_POSITION_A: c_uint = 0x41b;
pub const DC_DISP_V_PULSE0_POSITION_B: c_uint = 0x41c;
pub const DC_DISP_V_PULSE0_POSITION_C: c_uint = 0x41d;
pub const DC_DISP_V_PULSE1_CONTROL: c_uint = 0x41e;
pub const DC_DISP_V_PULSE1_POSITION_A: c_uint = 0x41f;
pub const DC_DISP_V_PULSE1_POSITION_B: c_uint = 0x420;
pub const DC_DISP_V_PULSE1_POSITION_C: c_uint = 0x421;
pub const DC_DISP_V_PULSE2_CONTROL: c_uint = 0x422;
pub const DC_DISP_V_PULSE2_POSITION_A: c_uint = 0x423;
pub const DC_DISP_V_PULSE3_CONTROL: c_uint = 0x424;
pub const DC_DISP_V_PULSE3_POSITION_A: c_uint = 0x425;
pub const DC_DISP_M0_CONTROL: c_uint = 0x426;
pub const DC_DISP_M1_CONTROL: c_uint = 0x427;
pub const DC_DISP_DI_CONTROL: c_uint = 0x428;
pub const DC_DISP_PP_CONTROL: c_uint = 0x429;
pub const DC_DISP_PP_SELECT_A: c_uint = 0x42a;
pub const DC_DISP_PP_SELECT_B: c_uint = 0x42b;
pub const DC_DISP_PP_SELECT_C: c_uint = 0x42c;
pub const DC_DISP_PP_SELECT_D: c_uint = 0x42d;

pub const DC_DISP_DISP_CLOCK_CONTROL: c_uint = 0x42e;

pub const DC_DISP_DISP_INTERFACE_CONTROL: c_uint = 0x42f;

pub const DC_DISP_DISP_COLOR_CONTROL: c_uint = 0x430;

pub const DC_DISP_SHIFT_CLOCK_OPTIONS: c_uint = 0x431;

pub const DC_DISP_DATA_ENABLE_OPTIONS: c_uint = 0x432;

pub const DC_DISP_SERIAL_INTERFACE_OPTIONS: c_uint = 0x433;
pub const DC_DISP_LCD_SPI_OPTIONS: c_uint = 0x434;
pub const DC_DISP_BORDER_COLOR: c_uint = 0x435;
pub const DC_DISP_COLOR_KEY0_LOWER: c_uint = 0x436;
pub const DC_DISP_COLOR_KEY0_UPPER: c_uint = 0x437;
pub const DC_DISP_COLOR_KEY1_LOWER: c_uint = 0x438;
pub const DC_DISP_COLOR_KEY1_UPPER: c_uint = 0x439;
pub const DC_DISP_CURSOR_FOREGROUND: c_uint = 0x43c;
pub const DC_DISP_CURSOR_BACKGROUND: c_uint = 0x43d;
pub const DC_DISP_CURSOR_START_ADDR: c_uint = 0x43e;

pub const DC_DISP_CURSOR_START_ADDR_NS: c_uint = 0x43f;
pub const DC_DISP_CURSOR_POSITION: c_uint = 0x440;
pub const DC_DISP_CURSOR_POSITION_NS: c_uint = 0x441;
pub const DC_DISP_INIT_SEQ_CONTROL: c_uint = 0x442;
pub const DC_DISP_SPI_INIT_SEQ_DATA_A: c_uint = 0x443;
pub const DC_DISP_SPI_INIT_SEQ_DATA_B: c_uint = 0x444;
pub const DC_DISP_SPI_INIT_SEQ_DATA_C: c_uint = 0x445;
pub const DC_DISP_SPI_INIT_SEQ_DATA_D: c_uint = 0x446;
pub const DC_DISP_DC_MCCIF_FIFOCTRL: c_uint = 0x480;
pub const DC_DISP_MCCIF_DISPLAY0A_HYST: c_uint = 0x481;
pub const DC_DISP_MCCIF_DISPLAY0B_HYST: c_uint = 0x482;
pub const DC_DISP_MCCIF_DISPLAY1A_HYST: c_uint = 0x483;
pub const DC_DISP_MCCIF_DISPLAY1B_HYST: c_uint = 0x484;
pub const DC_DISP_DAC_CRT_CTRL: c_uint = 0x4c0;
pub const DC_DISP_DISP_MISC_CONTROL: c_uint = 0x4c1;
pub const DC_DISP_SD_CONTROL: c_uint = 0x4c2;
pub const DC_DISP_SD_CSC_COEFF: c_uint = 0x4c3;

pub const DC_DISP_SD_FLICKER_CONTROL: c_uint = 0x4cd;
pub const DC_DISP_DC_PIXEL_COUNT: c_uint = 0x4ce;

pub const DC_DISP_SD_BL_PARAMETERS: c_uint = 0x4d7;

pub const DC_DISP_SD_BL_CONTROL: c_uint = 0x4dc;
pub const DC_DISP_SD_HW_K_VALUES: c_uint = 0x4dd;
pub const DC_DISP_SD_MAN_K_VALUES: c_uint = 0x4de;
pub const DC_DISP_BLEND_BACKGROUND_COLOR: c_uint = 0x4e4;

pub const DC_DISP_INTERLACE_CONTROL: c_uint = 0x4e5;

pub const DC_DISP_CURSOR_START_ADDR_HI: c_uint = 0x4ec;
pub const DC_DISP_BLEND_CURSOR_CONTROL: c_uint = 0x4f1;

pub const CURSOR_ALPHA: c_uint = 0xff;
pub const DC_WIN_CORE_ACT_CONTROL: c_uint = 0x50e;

pub const DC_WIN_CORE_IHUB_WGRP_LATENCY_CTLA: c_uint = 0x543;

pub const DC_WIN_CORE_IHUB_WGRP_LATENCY_CTLB: c_uint = 0x544;
pub const WATERMARK_MASK: c_uint = 0x1fffffff;
pub const DC_WIN_CORE_PRECOMP_WGRP_PIPE_METER: c_uint = 0x560;

pub const DC_WIN_CORE_IHUB_WGRP_POOL_CONFIG: c_uint = 0x561;

pub const DC_WIN_CORE_IHUB_WGRP_FETCH_METER: c_uint = 0x562;

pub const DC_WIN_CORE_IHUB_LINEBUF_CONFIG: c_uint = 0x563;

pub const DC_WIN_CORE_IHUB_THREAD_GROUP: c_uint = 0x568;

pub const DC_WIN_CSC_YOF: c_uint = 0x611;
pub const DC_WIN_CSC_KYRGB: c_uint = 0x612;
pub const DC_WIN_CSC_KUR: c_uint = 0x613;
pub const DC_WIN_CSC_KVR: c_uint = 0x614;
pub const DC_WIN_CSC_KUG: c_uint = 0x615;
pub const DC_WIN_CSC_KVG: c_uint = 0x616;
pub const DC_WIN_CSC_KUB: c_uint = 0x617;
pub const DC_WIN_CSC_KVB: c_uint = 0x618;
pub const DC_WIN_WIN_OPTIONS: c_uint = 0x700;

pub const DC_WIN_BYTE_SWAP: c_uint = 0x701;

pub const DC_WIN_BUFFER_CONTROL: c_uint = 0x702;

pub const DC_WIN_COLOR_DEPTH: c_uint = 0x703;
pub const WIN_COLOR_DEPTH_P1: c_int = 0;
pub const WIN_COLOR_DEPTH_P2: c_int = 1;
pub const WIN_COLOR_DEPTH_P4: c_int = 2;
pub const WIN_COLOR_DEPTH_P8: c_int = 3;
pub const WIN_COLOR_DEPTH_B4G4R4A4: c_int = 4;
pub const WIN_COLOR_DEPTH_B5G5R5A1: c_int = 5;
pub const WIN_COLOR_DEPTH_B5G6R5: c_int = 6;
pub const WIN_COLOR_DEPTH_A1B5G5R5: c_int = 7;
pub const WIN_COLOR_DEPTH_B8G8R8A8: c_int = 12;
pub const WIN_COLOR_DEPTH_R8G8B8A8: c_int = 13;
pub const WIN_COLOR_DEPTH_B6x2G6x2R6x2A8: c_int = 14;
pub const WIN_COLOR_DEPTH_R6x2G6x2B6x2A8: c_int = 15;
pub const WIN_COLOR_DEPTH_YCbCr422: c_int = 16;
pub const WIN_COLOR_DEPTH_YUV422: c_int = 17;
pub const WIN_COLOR_DEPTH_YCbCr420P: c_int = 18;
pub const WIN_COLOR_DEPTH_YUV420P: c_int = 19;
pub const WIN_COLOR_DEPTH_YCbCr422P: c_int = 20;
pub const WIN_COLOR_DEPTH_YUV422P: c_int = 21;
pub const WIN_COLOR_DEPTH_YCbCr422R: c_int = 22;
pub const WIN_COLOR_DEPTH_YUV422R: c_int = 23;
pub const WIN_COLOR_DEPTH_YCbCr422RA: c_int = 24;
pub const WIN_COLOR_DEPTH_YUV422RA: c_int = 25;
pub const WIN_COLOR_DEPTH_R4G4B4A4: c_int = 27;
pub const WIN_COLOR_DEPTH_R5G5B5A: c_int = 28;
pub const WIN_COLOR_DEPTH_AR5G5B5: c_int = 29;
pub const WIN_COLOR_DEPTH_B5G5R5X1: c_int = 30;
pub const WIN_COLOR_DEPTH_X1B5G5R5: c_int = 31;
pub const WIN_COLOR_DEPTH_R5G5B5X1: c_int = 32;
pub const WIN_COLOR_DEPTH_X1R5G5B5: c_int = 33;
pub const WIN_COLOR_DEPTH_R5G6B5: c_int = 34;
pub const WIN_COLOR_DEPTH_A8R8G8B8: c_int = 35;
pub const WIN_COLOR_DEPTH_A8B8G8R8: c_int = 36;
pub const WIN_COLOR_DEPTH_B8G8R8X8: c_int = 37;
pub const WIN_COLOR_DEPTH_R8G8B8X8: c_int = 38;
pub const WIN_COLOR_DEPTH_YCbCr444P: c_int = 41;
pub const WIN_COLOR_DEPTH_YCrCb420SP: c_int = 42;
pub const WIN_COLOR_DEPTH_YCbCr420SP: c_int = 43;
pub const WIN_COLOR_DEPTH_YCrCb422SP: c_int = 44;
pub const WIN_COLOR_DEPTH_YCbCr422SP: c_int = 45;
pub const WIN_COLOR_DEPTH_YCrCb444SP: c_int = 48;
pub const WIN_COLOR_DEPTH_YCbCr444SP: c_int = 49;
pub const WIN_COLOR_DEPTH_X8B8G8R8: c_int = 65;
pub const WIN_COLOR_DEPTH_X8R8G8B8: c_int = 66;
pub const DC_WIN_POSITION: c_uint = 0x704;

pub const DC_WIN_SIZE: c_uint = 0x705;

pub const DC_WIN_PRESCALED_SIZE: c_uint = 0x706;

pub const DC_WIN_H_INITIAL_DDA: c_uint = 0x707;
pub const DC_WIN_V_INITIAL_DDA: c_uint = 0x708;
pub const DC_WIN_DDA_INC: c_uint = 0x709;

pub const DC_WIN_LINE_STRIDE: c_uint = 0x70a;
pub const DC_WIN_BUF_STRIDE: c_uint = 0x70b;
pub const DC_WIN_UV_BUF_STRIDE: c_uint = 0x70c;
pub const DC_WIN_BUFFER_ADDR_MODE: c_uint = 0x70d;

pub const DC_WIN_DV_CONTROL: c_uint = 0x70e;
pub const DC_WIN_BLEND_NOKEY: c_uint = 0x70f;

pub const DC_WIN_BLEND_1WIN: c_uint = 0x710;

pub const DC_WIN_BLEND_2WIN_X: c_uint = 0x711;

pub const DC_WIN_BLEND_2WIN_Y: c_uint = 0x712;
pub const DC_WIN_BLEND_3WIN_XY: c_uint = 0x713;
pub const DC_WIN_HP_FETCH_CONTROL: c_uint = 0x714;
pub const DC_WINBUF_START_ADDR: c_uint = 0x800;
pub const DC_WINBUF_START_ADDR_NS: c_uint = 0x801;
pub const DC_WINBUF_START_ADDR_U: c_uint = 0x802;
pub const DC_WINBUF_START_ADDR_U_NS: c_uint = 0x803;
pub const DC_WINBUF_START_ADDR_V: c_uint = 0x804;
pub const DC_WINBUF_START_ADDR_V_NS: c_uint = 0x805;
pub const DC_WINBUF_ADDR_H_OFFSET: c_uint = 0x806;
pub const DC_WINBUF_ADDR_H_OFFSET_NS: c_uint = 0x807;
pub const DC_WINBUF_ADDR_V_OFFSET: c_uint = 0x808;
pub const DC_WINBUF_ADDR_V_OFFSET_NS: c_uint = 0x809;
pub const DC_WINBUF_UFLOW_STATUS: c_uint = 0x80a;
pub const DC_WINBUF_SURFACE_KIND: c_uint = 0x80b;

pub const DC_WINBUF_START_ADDR_HI: c_uint = 0x80d;
pub const DC_WINBUF_START_ADDR_HI_U: c_uint = 0x80f;
pub const DC_WINBUF_START_ADDR_HI_V: c_uint = 0x811;
pub const DC_WINBUF_CDE_CONTROL: c_uint = 0x82f;

pub const DC_WINBUF_AD_UFLOW_STATUS: c_uint = 0xbca;
pub const DC_WINBUF_BD_UFLOW_STATUS: c_uint = 0xdca;
pub const DC_WINBUF_CD_UFLOW_STATUS: c_uint = 0xfca;
// Tegra186 and later

pub const DC_DISP_CORE_HEAD_SET_CONTROL_OUTPUT_LUT: c_uint = 0x431;

pub const DC_DISP_COREPVT_HEAD_SET_OUTPUT_LUT_BASE: c_uint = 0x432;
pub const DC_DISP_COREPVT_HEAD_SET_OUTPUT_LUT_BASE_HI: c_uint = 0x433;
pub const DC_DISP_PCALC_HEAD_SET_CROPPED_POINT_IN_CURSOR: c_uint = 0x442;
pub const DC_DISP_PCALC_HEAD_SET_CROPPED_SIZE_IN_CURSOR: c_uint = 0x446;
pub const DC_WINC_PRECOMP_WGRP_PIPE_CAPA: c_uint = 0x500;
pub const DC_WINC_PRECOMP_WGRP_PIPE_CAPB: c_uint = 0x501;
pub const DC_WINC_PRECOMP_WGRP_PIPE_CAPC: c_uint = 0x502;

pub const DC_WINC_PRECOMP_WGRP_PIPE_CAPD: c_uint = 0x503;
pub const DC_WINC_PRECOMP_WGRP_PIPE_CAPE: c_uint = 0x504;

pub const DC_WINC_PRECOMP_WGRP_PIPE_CAPF: c_uint = 0x505;
pub const DC_WIN_CORE_WINDOWGROUP_SET_CONTROL: c_uint = 0x702;

pub const DC_WIN_CROPPED_SIZE: c_uint = 0x706;
pub const DC_WIN_SET_INPUT_SCALER_H_START_PHASE: c_uint = 0x707;
pub const DC_WIN_SET_INPUT_SCALER_V_START_PHASE: c_uint = 0x708;
pub const DC_WIN_PLANAR_STORAGE: c_uint = 0x709;

pub const DC_WIN_PLANAR_STORAGE_UV: c_uint = 0x70a;

pub const DC_WIN_SET_INPUT_SCALER_HPHASE_INCR: c_uint = 0x70b;
pub const DC_WIN_SET_INPUT_SCALER_VPHASE_INCR: c_uint = 0x70c;
pub const DC_WIN_SET_PARAMS: c_uint = 0x70d;

pub const DC_WIN_WINDOWGROUP_SET_CONTROL_INPUT_SCALER: c_uint = 0x70e;

pub const DC_WIN_WINDOWGROUP_SET_INPUT_SCALER_COEFF: c_uint = 0x70f;

pub const DC_WIN_WINDOWGROUP_SET_INPUT_SCALER_USAGE: c_uint = 0x711;

pub const DC_WIN_BLEND_LAYER_CONTROL: c_uint = 0x716;

pub const DC_WIN_BLEND_MATCH_SELECT: c_uint = 0x717;

pub const DC_WIN_BLEND_NOMATCH_SELECT: c_uint = 0x718;
pub const DC_WIN_PRECOMP_WGRP_PARAMS: c_uint = 0x724;

pub const DC_WIN_WINDOW_SET_CONTROL: c_uint = 0x730;

pub const DC_WINBUF_CROPPED_POINT: c_uint = 0x806;

