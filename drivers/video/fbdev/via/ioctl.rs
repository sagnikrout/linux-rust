//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/via/ioctl.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 1998-2008 VIA Technologies, Inc. All Rights Reserved.
// Copyright 2001-2008 S3 Graphics, Inc. All Rights Reserved.
//

// Macro flag: #define __user

// VIAFB IOCTL definition
pub const VIAFB_GET_INFO_SIZE: c_uint = 0x56494101	/* 'VIA\01' */;
pub const VIAFB_GET_INFO: c_uint = 0x56494102	/* 'VIA\02' */;
pub const VIAFB_HOTPLUG: c_uint = 0x56494103	/* 'VIA\03' */;
pub const VIAFB_SET_HOTPLUG_FLAG: c_uint = 0x56494104	/* 'VIA\04' */;
pub const VIAFB_GET_RESOLUTION: c_uint = 0x56494105	/* 'VIA\05' */;
pub const VIAFB_GET_SAMM_INFO: c_uint = 0x56494107	/* 'VIA\07' */;
pub const VIAFB_TURN_ON_OUTPUT_DEVICE: c_uint = 0x56494108	/* 'VIA\08' */;
pub const VIAFB_TURN_OFF_OUTPUT_DEVICE: c_uint = 0x56494109	/* 'VIA\09' */;
pub const VIAFB_GET_DEVICE: c_uint = 0x5649410B;
pub const VIAFB_GET_DRIVER_VERSION: c_uint = 0x56494112	/* 'VIA\12' */;
pub const VIAFB_GET_CHIP_INFO: c_uint = 0x56494113	/* 'VIA\13' */;
pub const VIAFB_GET_DEVICE_INFO: c_uint = 0x56494115;
pub const VIAFB_GET_DEVICE_SUPPORT: c_uint = 0x56494118;
pub const VIAFB_GET_DEVICE_CONNECT: c_uint = 0x56494119;
pub const VIAFB_GET_PANEL_SUPPORT_EXPAND: c_uint = 0x5649411A;
pub const VIAFB_GET_DRIVER_NAME: c_uint = 0x56494122;
pub const VIAFB_GET_DEVICE_SUPPORT_STATE: c_uint = 0x56494123;
pub const VIAFB_GET_GAMMA_LUT: c_uint = 0x56494124;
pub const VIAFB_SET_GAMMA_LUT: c_uint = 0x56494125;
pub const VIAFB_GET_GAMMA_SUPPORT_STATE: c_uint = 0x56494126;
pub const VIAFB_SYNC_SURFACE: c_uint = 0x56494130;
pub const VIAFB_GET_DRIVER_CAPS: c_uint = 0x56494131;
pub const VIAFB_GET_IGA_SCALING_INFO: c_uint = 0x56494132;
pub const VIAFB_GET_PANEL_MAX_SIZE: c_uint = 0x56494133;
pub const VIAFB_GET_PANEL_MAX_POSITION: c_uint = 0x56494134;
pub const VIAFB_SET_PANEL_SIZE: c_uint = 0x56494135;
pub const VIAFB_SET_PANEL_POSITION: c_uint = 0x56494136;
pub const VIAFB_GET_PANEL_POSITION: c_uint = 0x56494137;
pub const VIAFB_GET_PANEL_SIZE: c_uint = 0x56494138;
pub const None_Device: c_uint = 0x00;
pub const CRT_Device: c_uint = 0x01;
pub const LCD_Device: c_uint = 0x02;
pub const DVI_Device: c_uint = 0x08;
pub const CRT2_Device: c_uint = 0x10;
pub const LCD2_Device: c_uint = 0x40;
pub const OP_LCD_CENTERING: c_uint = 0x01;
pub const OP_LCD_PANEL_ID: c_uint = 0x02;
pub const OP_LCD_MODE: c_uint = 0x03;
// SAMM operation flag
pub const OP_SAMM: c_uint = 0x80;
pub const LCD_PANEL_ID_MAXIMUM: c_int = 23;
pub const STATE_ON: c_uint = 0x1;
pub const STATE_OFF: c_uint = 0x0;
pub const STATE_DEFAULT: c_uint = 0xFFFF;
pub const MAX_ACTIVE_DEV_NUM: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_t {
    pub crt:1: c_ushort,
    pub dvi:1: c_ushort,
    pub lcd:1: c_ushort,
    pub samm:1: c_ushort,
    pub lcd_dsp_cent:1: c_ushort,
    pub lcd_mode:1: c_uchar,
    pub epia_dvi:1: c_ushort,
    pub lcd_dual_edge:1: c_ushort,
    pub lcd2:1: c_ushort,
    pub primary_dev: c_ushort,
    pub lcd_panel_id: c_uchar,
    pub yres: unsigned short xres,,
    pub yres1: unsigned short xres1,,
    pub refresh: c_ushort,
    pub bpp: c_ushort,
    pub refresh1: c_ushort,
    pub bpp1: c_ushort,
    pub sequence: c_ushort,
    pub bus_width: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct viafb_ioctl_info {
    pub /: *mut *mut u32 viafb_id; / for identifying viafb,
pub const VIAID: c_uint = 0x56494146	/* Identify myself with 'VIAF' */;
    pub vendor_id: u16,
    pub device_id: u16,
    pub version: u8,
    pub revision: u8,
    pub /: *mut *mut u8 reserved[246]; / for future use,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct viafb_ioctl_mode {
    pub xres: u32,
    pub yres: u32,
    pub refresh: u32,
    pub bpp: u32,
    pub xres_sec: u32,
    pub yres_sec: u32,
    pub virtual_xres_sec: u32,
    pub virtual_yres_sec: u32,
    pub refresh_sec: u32,
    pub bpp_sec: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct viafb_ioctl_samm {
    pub samm_status: u32,
    pub size_prim: u32,
    pub size_sec: u32,
    pub mem_base: u32,
    pub offset_sec: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct viafb_driver_version {
    pub iMajorNum: c_int,
    pub iKernelNum: c_int,
    pub iOSNum: c_int,
    pub iMinorNum: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct viafb_ioctl_lcd_attribute {
    pub panel_id: c_uint,
    pub display_center: c_uint,
    pub lcd_mode: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct viafb_ioctl_setting {
// Enable or disable active devices
    pub device_flag: c_ushort,
// Indicate which device should be turn on or turn off.
    pub device_status: c_ushort,
    pub reserved: c_uint,
// Indicate which LCD's attribute can be changed.
    pub lcd_operation_flag: c_ushort,
// 1: SAMM ON  0: SAMM OFF
    pub samm_status: c_ushort,
// horizontal resolution of first device
    pub first_dev_hor_res: c_ushort,
// vertical resolution of first device
    pub first_dev_ver_res: c_ushort,
// horizontal resolution of second device
    pub second_dev_hor_res: c_ushort,
// vertical resolution of second device
    pub second_dev_ver_res: c_ushort,
// refresh rate of first device
    pub first_dev_refresh: c_ushort,
// bpp of first device
    pub first_dev_bpp: c_ushort,
// refresh rate of second device
    pub second_dev_refresh: c_ushort,
// bpp of second device
    pub second_dev_bpp: c_ushort,
// Indicate which device are primary display device.
    pub primary_device: c_uint,
    pub struct_reserved: [c_uint; 35],
    pub lcd_attributes: viafb_ioctl_lcd_attribute,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _UTFunctionCaps {
    pub dw3DScalingState: c_uint,
    pub reserved: [c_uint; 31],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _POSITIONVALUE {
    pub dwX: c_uint,
    pub dwY: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _panel_size_pos_info {
    pub device_type: c_uint,
    pub x: c_int,
    pub y: c_int,
}

extern "C" {
    pub fn viafb_ioctl_get_viafb_info(arg: u_long) -> c_int;
}
extern "C" {
    pub fn viafb_ioctl_hotplug(hres: c_int, vres: c_int, bpp: c_int) -> c_int;
}
