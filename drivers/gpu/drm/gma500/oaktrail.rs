//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/gma500/oaktrail.h
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
// Copyright (c) 2007-2011, Intel Corporation.
// All Rights Reserved.
//
// MID device specific descriptors
#[repr(C)]
#[derive(Copy, Clone)]
pub struct oaktrail_timing_info {
    pub pixel_clock: u16,
    pub hactive_lo: u8,
    pub hblank_lo: u8,
    pub hblank_hi:4: u8,
    pub hactive_hi:4: u8,
    pub vactive_lo: u8,
    pub vblank_lo: u8,
    pub vblank_hi:4: u8,
    pub vactive_hi:4: u8,
    pub hsync_offset_lo: u8,
    pub hsync_pulse_width_lo: u8,
    pub vsync_pulse_width_lo:4: u8,
    pub vsync_offset_lo:4: u8,
    pub vsync_pulse_width_hi:2: u8,
    pub vsync_offset_hi:2: u8,
    pub hsync_pulse_width_hi:2: u8,
    pub hsync_offset_hi:2: u8,
    pub width_mm_lo: u8,
    pub height_mm_lo: u8,
    pub height_mm_hi:4: u8,
    pub width_mm_hi:4: u8,
    pub hborder: u8,
    pub vborder: u8,
    pub unknown0:1: u8,
    pub hsync_positive:1: u8,
    pub vsync_positive:1: u8,
    pub separate_sync:2: u8,
    pub stereo:1: u8,
    pub unknown6:1: u8,
    pub interlaced:1: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gct_r10_timing_info {
    pub pixel_clock: u16,
    pub hactive_lo:8: u32,
    pub hactive_hi:4: u32,
    pub hblank_lo:8: u32,
    pub hblank_hi:4: u32,
    pub hsync_offset_lo:8: u32,
    pub hsync_offset_hi:2: u16,
    pub hsync_pulse_width_lo:8: u16,
    pub hsync_pulse_width_hi:2: u16,
    pub hsync_positive:1: u16,
    pub rsvd_1:3: u16,
    pub vactive_lo:8: u8,
    pub vactive_hi:4: u16,
    pub vblank_lo:8: u16,
    pub vblank_hi:4: u16,
    pub vsync_offset_lo:4: u16,
    pub vsync_offset_hi:2: u16,
    pub vsync_pulse_width_lo:4: u16,
    pub vsync_pulse_width_hi:2: u16,
    pub vsync_positive:1: u16,
    pub rsvd_2:3: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct oaktrail_panel_descriptor_v1 {
    pub /: *mut *mut u32 Panel_Port_Control; / 1 dword, Register 0x61180 if LVDS,
// 0x61190 if MIPI
    pub 0x61208,*/: *mut *mut u32 Panel_Power_On_Sequencing;/1 dword,Register,
    pub 0x6120C,*/: *mut *mut u32 Panel_Power_Off_Sequencing;/1 dword,Register,
    pub /: *mut *mut u32 Panel_Power_Cycle_Delay_and_Reference_Divisor;/ 1 dword,
// Register 0x61210
    pub /: *mut *mut oaktrail_timing_info DTD;/18 bytes, Standard definition,
    pub /: *mut *mut u16 Panel_Backlight_Inverter_Descriptor;/ 16 bits, as follows,
// Bit 0, Frequency, 15 bits,0 - 32767Hz
// Bit 15, Polarity, 1 bit, 0: Normal, 1: Inverted
    pub Panel_MIPI_Display_Descriptor: u16,
// 16 bits, Defined as follows:
// if MIPI, 0x0000 if LVDS
// Bit 0, Type, 2 bits,
// 0: Type-1,
// 1: Type-2,
// 2: Type-3,
// 3: Type-4
// Bit 2, Pixel Format, 4 bits
// Bit0: 16bpp (not supported in LNC),
// Bit1: 18bpp loosely packed,
// Bit2: 18bpp packed,
// Bit3: 24bpp
// Bit 6, Reserved, 2 bits, 00b
// Bit 8, Minimum Supported Frame Rate, 6 bits, 0 - 63Hz
// Bit 14, Reserved, 2 bits, 00b
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct oaktrail_panel_descriptor_v2 {
    pub /: *mut *mut u32 Panel_Port_Control; / 1 dword, Register 0x61180 if LVDS,
// 0x61190 if MIPI
    pub 0x61208,*/: *mut *mut u32 Panel_Power_On_Sequencing;/1 dword,Register,
    pub 0x6120C,*/: *mut *mut u32 Panel_Power_Off_Sequencing;/1 dword,Register,
    pub /: *mut *mut u8 Panel_Power_Cycle_Delay_and_Reference_Divisor;/ 1 byte,
// Register 0x61210
    pub /: *mut *mut oaktrail_timing_info DTD;/18 bytes, Standard definition,
    pub follows*/: *mut *mut u16 Panel_Backlight_Inverter_Descriptor;/16 bits, as,
// Bit 0, Frequency, 16 bits, 0 - 32767Hz
    pub /: *mut *mut u8 Panel_Initial_Brightness;/ [7:0] 0 - 100%,
// Bit 7, Polarity, 1 bit,0: Normal, 1: Inverted
    pub Panel_MIPI_Display_Descriptor: u16,
// 16 bits, Defined as follows:
// if MIPI, 0x0000 if LVDS
// Bit 0, Type, 2 bits,
// 0: Type-1,
// 1: Type-2,
// 2: Type-3,
// 3: Type-4
// Bit 2, Pixel Format, 4 bits
// Bit0: 16bpp (not supported in LNC),
// Bit1: 18bpp loosely packed,
// Bit2: 18bpp packed,
// Bit3: 24bpp
// Bit 6, Reserved, 2 bits, 00b
// Bit 8, Minimum Supported Frame Rate, 6 bits, 0 - 63Hz
// Bit 14, Reserved, 2 bits, 00b
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union oaktrail_panel_rx {
    pub lane,*/: *mut *mut u16 NumberOfLanes:2; /Num of Lanes, 2 bits,0 = 1,
// 1 = 2 lanes, 2 = 3 lanes, 3 = 4 lanes.
    pub /: *mut *mut u16 MaxLaneFreq:3; / 0: 100MHz, 1: 200MHz, 2: 300MHz,,
// 3: 400MHz, 4: 500MHz, 5: 600MHz, 6: 700MHz, 7: 800MHz.
    pub /: *mut *mut u16 SupportedVideoTransferMode:2; /0: Non-burst only,
// 1: Burst and non-burst
// 2/3: Reserved
    pub Non-continuous*/: *mut *mut u16 HSClkBehavior:1; /0: Continuous, 1:,
    pub Yes*/: *mut *mut u16 DuoDisplaySupport:1; /1 bit,0: No, 1:,
    pub Yes*/: *mut *mut u16 ECC_ChecksumCapabilities:1;/1 bit,0: No, 1:,
    pub /: *mut *mut u16 BidirectionalCommunication:1;/1 bit,0: No, 1: Yes,
    pub /: *mut *mut u16 Rsvd:5;/5 bits,00000b,
    pub panelrx: },
    pub panel_receiver: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gct_r0 {
    pub panels*/: *mut *mut u8 PanelType:4; /4 bits, Bit field for,
// 0 - 3: 0 = LVDS, 1 = MIPI
// 2 bits,Specifies which of the
    pub BootPanelIndex:2: u8,
// 4 panels to use by default
    pub of*/: *mut *mut u8 BootMIPI_DSI_RxIndex:2;/Specifies which,
// the 4 MIPI DSI receivers to use
    pub PD: },
    pub PanelDescriptor: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gct_r1 {
    pub panels*/: *mut *mut u8 PanelType:4; /4 bits, Bit field for,
// 0 - 3: 0 = LVDS, 1 = MIPI
// 2 bits,Specifies which of the
    pub BootPanelIndex:2: u8,
// 4 panels to use by default
    pub of*/: *mut *mut u8 BootMIPI_DSI_RxIndex:2;/Specifies which,
// the 4 MIPI DSI receivers to use
    pub PD: },
    pub PanelDescriptor: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gct_r10 {
    pub DTD: gct_r10_timing_info,
    pub Panel_MIPI_Display_Descriptor: u16,
    pub Panel_MIPI_Receiver_Descriptor: u16,
    pub Panel_Backlight_Inverter_Descriptor: u16,
    pub Panel_Initial_Brightness: u8,
    pub MIPI_Ctlr_Init_ptr: u32,
    pub MIPI_Panel_Init_ptr: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct oaktrail_gct_data {
    pub /: *mut *mut u8 bpi; / boot panel index, number of panel used during boot,
    pub /: *mut *mut u8 pt; / panel type, 4 bit field, 0=lvds, 1=mipi,
    pub /: *mut *mut oaktrail_timing_info DTD; / timing info for the selected panel,
    pub Panel_Port_Control: u32,
    pub 0x61208,*/: *mut *mut u32 PP_On_Sequencing;/1 dword,Register,
    pub 0x6120C,*/: *mut *mut u32 PP_Off_Sequencing;/1 dword,Register,
    pub PP_Cycle_Delay: u32,
    pub Panel_Backlight_Inverter_Descriptor: u16,
    pub Panel_MIPI_Display_Descriptor: u16,
    pub __packed: },
pub const MODE_SETTING_IN_CRTC: c_uint = 0x1;
pub const MODE_SETTING_IN_ENCODER: c_uint = 0x2;
pub const MODE_SETTING_ON_GOING: c_uint = 0x3;
pub const MODE_SETTING_IN_DSR: c_uint = 0x4;
pub const MODE_SETTING_ENCODER_DONE: c_uint = 0x8;
//
// Moorestown HDMI interfaces
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct oaktrail_hdmi_dev {
    pub dev: *mut pci_dev,
    pub regs: *mut void __iomem,
    pub mmio_len: unsigned int mmio,,
    pub dpms_mode: c_int,
    pub i2c_dev: *mut hdmi_i2c_dev,
// register state
    pub saveDPLL_CTRL: u32,
    pub saveDPLL_DIV_CTRL: u32,
    pub saveDPLL_ADJUST: u32,
    pub saveDPLL_UPDATE: u32,
    pub saveDPLL_CLK_ENABLE: u32,
    pub savePCH_HTOTAL_B: u32,
    pub savePCH_HBLANK_B: u32,
    pub savePCH_HSYNC_B: u32,
    pub savePCH_VTOTAL_B: u32,
    pub savePCH_VBLANK_B: u32,
    pub savePCH_VSYNC_B: u32,
    pub savePCH_PIPEBCONF: u32,
    pub savePCH_PIPEBSRC: u32,
}

extern "C" {
    pub fn oaktrail_hdmi_setup(dev: *mut drm_device);
}
extern "C" {
    pub fn oaktrail_hdmi_teardown(dev: *mut drm_device);
}
extern "C" {
    pub fn oaktrail_hdmi_i2c_init(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn oaktrail_hdmi_i2c_exit(dev: *mut pci_dev);
}
extern "C" {
    pub fn oaktrail_hdmi_save(dev: *mut drm_device);
}
extern "C" {
    pub fn oaktrail_hdmi_restore(dev: *mut drm_device);
}
extern "C" {
    pub fn oaktrail_hdmi_init(dev: *mut drm_device, mode_dev: *mut psb_intel_mode_device);
}
extern "C" {
    pub fn oaktrail_crtc_hdmi_dpms(crtc: *mut drm_crtc, mode: c_int);
}
