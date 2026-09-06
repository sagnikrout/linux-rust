//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/radeon_acpi.h
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
// Copyright 2012 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// AMD hw uses four ACPI control methods:
// 1. ATIF
// ARG0: (ACPI_INTEGER) function code
// ARG1: (ACPI_BUFFER) parameter buffer, 256 bytes
// OUTPUT: (ACPI_BUFFER) output buffer, 256 bytes
// ATIF provides an entry point for the gfx driver to interact with the sbios.
// The AMD ACPI notification mechanism uses Notify (VGA, 0x81) or a custom
// notification. Which notification is used as indicated by the ATIF Control
// Method GET_SYSTEM_PARAMETERS. When the driver receives Notify (VGA, 0x81) or
// a custom notification it invokes ATIF Control Method GET_SYSTEM_BIOS_REQUESTS
// to identify pending System BIOS requests and associated parameters. For
// example, if one of the pending requests is DISPLAY_SWITCH_REQUEST, the driver
// will perform display device detection and invoke ATIF Control Method
// SELECT_ACTIVE_DISPLAYS.
//
// 2. ATPX
// ARG0: (ACPI_INTEGER) function code
// ARG1: (ACPI_BUFFER) parameter buffer, 256 bytes
// OUTPUT: (ACPI_BUFFER) output buffer, 256 bytes
// ATPX methods are used on PowerXpress systems to handle mux switching and
// discrete GPU power control.
//
// 3. ATRM
// ARG0: (ACPI_INTEGER) offset of vbios rom data
// ARG1: (ACPI_BUFFER) size of the buffer to fill (up to 4K).
// OUTPUT: (ACPI_BUFFER) output buffer
// ATRM provides an interfacess to access the discrete GPU vbios image on
// PowerXpress systems with multiple GPUs.
//
// 4. ATCS
// ARG0: (ACPI_INTEGER) function code
// ARG1: (ACPI_BUFFER) parameter buffer, 256 bytes
// OUTPUT: (ACPI_BUFFER) output buffer, 256 bytes
// ATCS provides an interface to AMD chipset specific functionality.
//
// ATIF
pub const ATIF_FUNCTION_VERIFY_INTERFACE: c_uint = 0x0;
// ARG0: ATIF_FUNCTION_VERIFY_INTERFACE
// ARG1: none
// OUTPUT:
// WORD  - structure size in bytes (includes size field)
// WORD  - version
// DWORD - supported notifications mask
// DWORD - supported functions bit vector
//
// Notifications mask

// supported functions vector

pub const ATIF_FUNCTION_GET_SYSTEM_PARAMETERS: c_uint = 0x1;
// ARG0: ATIF_FUNCTION_GET_SYSTEM_PARAMETERS
// ARG1: none
// OUTPUT:
// WORD  - structure size in bytes (includes size field)
// DWORD - valid flags mask
// DWORD - flags
//
// OR
//
// WORD  - structure size in bytes (includes size field)
// DWORD - valid flags mask
// DWORD - flags
// BYTE  - notify command code
//
// flags
// bits 1:0:
// 0 - Notify(VGA, 0x81) is not used for notification
// 1 - Notify(VGA, 0x81) is used for notification
// 2 - Notify(VGA, n) is used for notification where
// n (0xd0-0xd9) is specified in notify command code.
// bit 2:
// 1 - lid changes not reported though int10
//
pub const ATIF_FUNCTION_GET_SYSTEM_BIOS_REQUESTS: c_uint = 0x2;
// ARG0: ATIF_FUNCTION_GET_SYSTEM_BIOS_REQUESTS
// ARG1: none
// OUTPUT:
// WORD  - structure size in bytes (includes size field)
// DWORD - pending sbios requests
// BYTE  - panel expansion mode
// BYTE  - thermal state: target gfx controller
// BYTE  - thermal state: state id (0: exit state, non-0: state)
// BYTE  - forced power state: target gfx controller
// BYTE  - forced power state: state id
// BYTE  - system power source
// BYTE  - panel backlight level (0-255)
//
// pending sbios requests

// panel expansion mode

// target gfx controller

// system power source

pub const ATIF_FUNCTION_SELECT_ACTIVE_DISPLAYS: c_uint = 0x3;
// ARG0: ATIF_FUNCTION_SELECT_ACTIVE_DISPLAYS
// ARG1:
// WORD  - structure size in bytes (includes size field)
// WORD  - selected displays
// WORD  - connected displays
// OUTPUT:
// WORD  - structure size in bytes (includes size field)
// WORD  - selected displays
//

pub const ATIF_FUNCTION_GET_LID_STATE: c_uint = 0x4;
// ARG0: ATIF_FUNCTION_GET_LID_STATE
// ARG1: none
// OUTPUT:
// WORD  - structure size in bytes (includes size field)
// BYTE  - lid state (0: open, 1: closed)
//
// GET_LID_STATE only works at boot and resume, for general lid
// status, use the kernel provided status
//
pub const ATIF_FUNCTION_GET_TV_STANDARD_FROM_CMOS: c_uint = 0x5;
// ARG0: ATIF_FUNCTION_GET_TV_STANDARD_FROM_CMOS
// ARG1: none
// OUTPUT:
// WORD  - structure size in bytes (includes size field)
// BYTE  - 0
// BYTE  - TV standard
//

pub const ATIF_FUNCTION_SET_TV_STANDARD_IN_CMOS: c_uint = 0x6;
// ARG0: ATIF_FUNCTION_SET_TV_STANDARD_IN_CMOS
// ARG1:
// WORD  - structure size in bytes (includes size field)
// BYTE  - 0
// BYTE  - TV standard
// OUTPUT: none
//
pub const ATIF_FUNCTION_GET_PANEL_EXPANSION_MODE_FROM_CMOS: c_uint = 0x7;
// ARG0: ATIF_FUNCTION_GET_PANEL_EXPANSION_MODE_FROM_CMOS
// ARG1: none
// OUTPUT:
// WORD  - structure size in bytes (includes size field)
// BYTE  - panel expansion mode
//
pub const ATIF_FUNCTION_SET_PANEL_EXPANSION_MODE_IN_CMOS: c_uint = 0x8;
// ARG0: ATIF_FUNCTION_SET_PANEL_EXPANSION_MODE_IN_CMOS
// ARG1:
// WORD  - structure size in bytes (includes size field)
// BYTE  - panel expansion mode
// OUTPUT: none
//
pub const ATIF_FUNCTION_TEMPERATURE_CHANGE_NOTIFICATION: c_uint = 0xD;
// ARG0: ATIF_FUNCTION_TEMPERATURE_CHANGE_NOTIFICATION
// ARG1:
// WORD  - structure size in bytes (includes size field)
// WORD  - gfx controller id
// BYTE  - current temperature (degress Celsius)
// OUTPUT: none
//
pub const ATIF_FUNCTION_GET_GRAPHICS_DEVICE_TYPES: c_uint = 0xF;
// ARG0: ATIF_FUNCTION_GET_GRAPHICS_DEVICE_TYPES
// ARG1: none
// OUTPUT:
// WORD  - number of gfx devices
// WORD  - device structure size in bytes (excludes device size field)
// DWORD - flags         \
// WORD  - bus number     } repeated structure
// WORD  - device number
//
// flags

pub const ATIF_FUNCTION_GET_EXTERNAL_GPU_INFORMATION: c_uint = 0x15;
// ARG0: ATIF_FUNCTION_GET_EXTERNAL_GPU_INFORMATION
// ARG1: none
// OUTPUT:
// WORD  - number of reported external gfx devices
// WORD  - device structure size in bytes (excludes device size field)
// WORD  - flags         \
// WORD  - bus number    / repeated structure
//
// flags

// ATPX
pub const ATPX_FUNCTION_VERIFY_INTERFACE: c_uint = 0x0;
// ARG0: ATPX_FUNCTION_VERIFY_INTERFACE
// ARG1: none
// OUTPUT:
// WORD  - structure size in bytes (includes size field)
// WORD  - version
// DWORD - supported functions bit vector
//
// supported functions vector

pub const ATPX_FUNCTION_GET_PX_PARAMETERS: c_uint = 0x1;
// ARG0: ATPX_FUNCTION_GET_PX_PARAMETERS
// ARG1: none
// OUTPUT:
// WORD  - structure size in bytes (includes size field)
// DWORD - valid flags mask
// DWORD - flags
//
// flags

pub const ATPX_FUNCTION_POWER_CONTROL: c_uint = 0x2;
// ARG0: ATPX_FUNCTION_POWER_CONTROL
// ARG1:
// WORD  - structure size in bytes (includes size field)
// BYTE  - dGPU power state (0: power off, 1: power on)
// OUTPUT: none
//
pub const ATPX_FUNCTION_DISPLAY_MUX_CONTROL: c_uint = 0x3;
// ARG0: ATPX_FUNCTION_DISPLAY_MUX_CONTROL
// ARG1:
// WORD  - structure size in bytes (includes size field)
// WORD  - display mux control (0: iGPU, 1: dGPU)
// OUTPUT: none
//

pub const ATPX_FUNCTION_I2C_MUX_CONTROL: c_uint = 0x4;
// ARG0: ATPX_FUNCTION_I2C_MUX_CONTROL
// ARG1:
// WORD  - structure size in bytes (includes size field)
// WORD  - i2c/aux/hpd mux control (0: iGPU, 1: dGPU)
// OUTPUT: none
//
pub const ATPX_FUNCTION_GRAPHICS_DEVICE_SWITCH_START_NOTIFICATION: c_uint = 0x5;
// ARG0: ATPX_FUNCTION_GRAPHICS_DEVICE_SWITCH_START_NOTIFICATION
// ARG1:
// WORD  - structure size in bytes (includes size field)
// WORD  - target gpu (0: iGPU, 1: dGPU)
// OUTPUT: none
//
pub const ATPX_FUNCTION_GRAPHICS_DEVICE_SWITCH_END_NOTIFICATION: c_uint = 0x6;
// ARG0: ATPX_FUNCTION_GRAPHICS_DEVICE_SWITCH_END_NOTIFICATION
// ARG1:
// WORD  - structure size in bytes (includes size field)
// WORD  - target gpu (0: iGPU, 1: dGPU)
// OUTPUT: none
//
pub const ATPX_FUNCTION_GET_DISPLAY_CONNECTORS_MAPPING: c_uint = 0x8;
// ARG0: ATPX_FUNCTION_GET_DISPLAY_CONNECTORS_MAPPING
// ARG1: none
// OUTPUT:
// WORD  - number of display connectors
// WORD  - connector structure size in bytes (excludes connector size field)
// BYTE  - flags                                                     \
// BYTE  - ATIF display vector bit position                           } repeated
// BYTE  - adapter id (0: iGPU, 1-n: dGPU ordered by pcie bus number) } structure
// WORD  - connector ACPI id
//
// flags

pub const ATPX_FUNCTION_GET_DISPLAY_DETECTION_PORTS: c_uint = 0x9;
// ARG0: ATPX_FUNCTION_GET_DISPLAY_DETECTION_PORTS
// ARG1: none
// OUTPUT:
// WORD  - number of HPD/DDC ports
// WORD  - port structure size in bytes (excludes port size field)
// BYTE  - ATIF display vector bit position \
// BYTE  - hpd id                            } reapeated structure
// BYTE  - ddc id
//
// available on A+A systems only
//
// hpd id

// ddc id

// ATCS
pub const ATCS_FUNCTION_VERIFY_INTERFACE: c_uint = 0x0;
// ARG0: ATCS_FUNCTION_VERIFY_INTERFACE
// ARG1: none
// OUTPUT:
// WORD  - structure size in bytes (includes size field)
// WORD  - version
// DWORD - supported functions bit vector
//
// supported functions vector

pub const ATCS_FUNCTION_GET_EXTERNAL_STATE: c_uint = 0x1;
// ARG0: ATCS_FUNCTION_GET_EXTERNAL_STATE
// ARG1: none
// OUTPUT:
// WORD  - structure size in bytes (includes size field)
// DWORD - valid flags mask
// DWORD - flags (0: undocked, 1: docked)
//
// flags

pub const ATCS_FUNCTION_PCIE_PERFORMANCE_REQUEST: c_uint = 0x2;
// ARG0: ATCS_FUNCTION_PCIE_PERFORMANCE_REQUEST
// ARG1:
// WORD  - structure size in bytes (includes size field)
// WORD  - client id (bit 2-0: func num, 7-3: dev num, 15-8: bus num)
// WORD  - valid flags mask
// WORD  - flags
// BYTE  - request type
// BYTE  - performance request
// OUTPUT:
// WORD  - structure size in bytes (includes size field)
// BYTE  - return value
//
// flags

// request type

// performance request

// return value

pub const ATCS_FUNCTION_PCIE_DEVICE_READY_NOTIFICATION: c_uint = 0x3;
// ARG0: ATCS_FUNCTION_PCIE_DEVICE_READY_NOTIFICATION
// ARG1: none
// OUTPUT: none
//
pub const ATCS_FUNCTION_SET_PCIE_BUS_WIDTH: c_uint = 0x4;
// ARG0: ATCS_FUNCTION_SET_PCIE_BUS_WIDTH
// ARG1:
// WORD  - structure size in bytes (includes size field)
// WORD  - client id (bit 2-0: func num, 7-3: dev num, 15-8: bus num)
// BYTE  - number of active lanes
// OUTPUT:
// WORD  - structure size in bytes (includes size field)
// BYTE  - number of active lanes
//

extern "C" {
    pub fn radeon_register_atpx_handler();
}
extern "C" {
    pub fn radeon_unregister_atpx_handler();
}
extern "C" {
    pub fn radeon_has_atpx_dgpu_power_cntl() -> bool;
}
extern "C" {
    pub fn radeon_is_atpx_hybrid() -> bool;
}
extern "C" {
    pub fn radeon_has_atpx() -> bool;
}
extern "C" {
    pub fn radeon_atpx_dgpu_req_power_for_displays() -> bool;
}

