//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/amd_acpi.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atif_verify_interface {
    pub /: *mut *mut u16 size; / structure size in bytes (includes size field),
    pub /: *mut *mut u16 version; / version,
    pub /: *mut *mut u32 notification_mask; / supported notifications mask,
    pub /: *mut *mut u32 function_bits; / supported functions bit vector,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atif_system_params {
    pub /: *mut *mut u16 size; / structure size in bytes (includes size field),
    pub /: *mut *mut u32 valid_mask; / valid flags mask,
    pub /: *mut *mut u32 flags; / flags,
    pub /: *mut *mut u8 command_code; / notify command code,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atif_sbios_requests {
    pub /: *mut *mut u16 size; / structure size in bytes (includes size field),
    pub /: *mut *mut u32 pending; / pending sbios requests,
    pub /: *mut *mut u8 panel_exp_mode; / panel expansion mode,
    pub /: *mut *mut u8 thermal_gfx; / thermal state: target gfx controller,
    pub /: *mut *mut u8 thermal_state; / thermal state: state id (0: exit state, non-0: state),
    pub /: *mut *mut u8 forced_power_gfx; / forced power state: target gfx controller,
    pub /: *mut *mut u8 forced_power_state; / forced power state: state id,
    pub /: *mut *mut u8 system_power_src; / system power source,
    pub /: *mut *mut u8 backlight_level; / panel backlight level (0-255),
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atif_qbtc_arguments {
    pub /: *mut *mut u16 size; / structure size in bytes (includes size field),
    pub /: *mut *mut u8 requested_display; / which display is requested,
    pub __packed: },
pub const ATIF_QBTC_MAX_DATA_POINTS: c_int = 99;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atif_qbtc_data_point {
    pub /: *mut *mut u8 luminance; / luminance in percent,
    pub /: *mut *mut u8 input_signal; / input signal in range 0-255,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atif_qbtc_output {
    pub /: *mut *mut u16 size; / structure size in bytes (includes size field),
    pub /: *mut *mut u16 flags; / all zeroes,
    pub /: *mut *mut u8 error_code; / error code,
    pub /: *mut *mut u8 ac_level; / default brightness on AC power,
    pub /: *mut *mut u8 dc_level; / default brightness on DC power,
    pub /: *mut *mut u8 min_input_signal; / max input signal in range 0-255,
    pub /: *mut *mut u8 max_input_signal; / min input signal in range 0-255,
    pub /: *mut *mut u8 number_of_points; / number of data points,
    pub data_points: [atif_qbtc_data_point; ATIF_QBTC_MAX_DATA_POINTS],
    pub __packed: },
    pub MAX_LUMINANCE_DATA_POINTS): static_assert(ATIF_QBTC_MAX_DATA_POINTS ==,
    pub amdgpu_dm_luminance_data)): static_assert(sizeof(struct atif_qbtc_data_point) == sizeof(struct,
pub const ATIF_NOTIFY_MASK: c_uint = 0x3;
pub const ATIF_NOTIFY_NONE: c_int = 0;
pub const ATIF_NOTIFY_81: c_int = 1;
pub const ATIF_NOTIFY_N: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atcs_verify_interface {
    pub /: *mut *mut u16 size; / structure size in bytes (includes size field),
    pub /: *mut *mut u16 version; / version,
    pub /: *mut *mut u32 function_bits; / supported functions bit vector,
    pub __packed: },
pub const ATCS_VALID_FLAGS_MASK: c_uint = 0x3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atcs_pref_req_input {
    pub /: *mut *mut u16 size; / structure size in bytes (includes size field),
    pub /: *mut *mut u16 client_id; / client id (bit 2-0: func num, 7-3: dev num, 15-8: bus num),
    pub /: *mut *mut u16 valid_flags_mask; / valid flags mask,
    pub /: *mut *mut u16 flags; / flags,
    pub /: *mut *mut u8 req_type; / request type,
    pub /: *mut *mut u8 perf_req; / performance request,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atcs_pref_req_output {
    pub /: *mut *mut u16 size; / structure size in bytes (includes size field),
    pub /: *mut *mut u8 ret_val; / return value,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atcs_pwr_shift_input {
    pub /: *mut *mut u16 size; / structure size in bytes (includes size field),
    pub /: *mut *mut u16 dgpu_id; / client id (bit 2-0: func num, 7-3: dev num, 15-8: bus num),
    pub /: *mut *mut u8 dev_acpi_state; / D0 = 0, D3 hot = 3,
    pub /: *mut *mut u8 drv_state; / 0 = operational, 1 = not operational,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atcs_get_uma_size_output {
    pub /: *mut *mut u16 size; / structure size in bytes (includes size field),
    pub /: *mut *mut u32 uma_size_mb; / allocated UMA size in MB,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atcs_set_uma_allocation_size_input {
    pub /: *mut *mut u16 size; / structure size in bytes (includes size field),
    pub /: *mut *mut u8 uma_size_index; / UMA size index,
    pub /: *mut *mut u8 uma_size_type; / UMA size type,
    pub __packed: },
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
// bit 3:
// 1 - system bios controls overclocking
// bit 4:
// 1 - enable overclocking
//
pub const ATIF_FUNCTION_GET_SYSTEM_BIOS_REQUESTS: c_uint = 0x2;
// ARG0: ATIF_FUNCTION_GET_SYSTEM_BIOS_REQUESTS
// ARG1: none
// OUTPUT:
// WORD  - structure size in bytes (includes size field)
// DWORD - pending sbios requests
// BYTE  - reserved (all zeroes)
// BYTE  - thermal state: target gfx controller
// BYTE  - thermal state: state id (0: exit state, non-0: state)
// BYTE  - forced power state: target gfx controller
// BYTE  - forced power state: state id (0: forced state, non-0: state)
// BYTE  - system power source
// BYTE  - panel backlight level (0-255)
// BYTE  - GPU package power limit: target gfx controller
// DWORD - GPU package power limit: value (24:8 fractional format, Watts)
//
// pending sbios requests

// target gfx controller

// system power source

pub const ATIF_FUNCTION_TEMPERATURE_CHANGE_NOTIFICATION: c_uint = 0xD;
// ARG0: ATIF_FUNCTION_TEMPERATURE_CHANGE_NOTIFICATION
// ARG1:
// WORD  - structure size in bytes (includes size field)
// WORD  - gfx controller id
// BYTE  - current temperature (degress Celsius)
// OUTPUT: none
//
pub const ATIF_FUNCTION_QUERY_BRIGHTNESS_TRANSFER_CHARACTERISTICS: c_uint = 0x10;
// ARG0: ATIF_FUNCTION_QUERY_BRIGHTNESS_TRANSFER_CHARACTERISTICS
// ARG1:
// WORD  - structure size in bytes (includes size field)
// BYTE  - requested display
// OUTPUT:
// WORD  - structure size in bytes (includes size field)
// WORD  - flags (currently all 16 bits are reserved)
// BYTE  - error code (on failure, disregard all below fields)
// BYTE  - AC level (default brightness in percent when machine has full power)
// BYTE  - DC level (default brightness in percent when machine is on battery)
// BYTE  - min input signal, in range 0-255, corresponding to 0% backlight
// BYTE  - max input signal, in range 0-255, corresponding to 100% backlight
// BYTE  - number of reported data points
// BYTE  - luminance level in percent  \ repeated structure
// BYTE  - input signal in range 0-255 / does not have entries for 0% and 100%
//
// requested display

// error code

pub const ATIF_FUNCTION_READY_TO_UNDOCK_NOTIFICATION: c_uint = 0x11;
// ARG0: ATIF_FUNCTION_READY_TO_UNDOCK_NOTIFICATION
// ARG1: none
// OUTPUT: none
//
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
pub const ATCS_FUNCTION_POWER_SHIFT_CONTROL: c_uint = 0x8;
// ARG0: ATCS_FUNCTION_POWER_SHIFT_CONTROL
// ARG1:
// WORD  - structure size in bytes (includes size field)
// WORD  - dGPU id (bit 2-0: func num, 7-3: dev num, 15-8: bus num)
// BYTE  - Device ACPI state
// BYTE  - Driver state
// OUTPUT: none
//
pub const ATCS_FUNCTION_GET_UMA_SIZE: c_uint = 0x6;
// ARG0: ATCS_FUNCTION_GET_UMA_SIZE
// ARG1: none
// OUTPUT:
// WORD  - structure size in bytes (includes size field)
// DWORD - allocated UMA size in MB
//
pub const ATCS_FUNCTION_SET_UMA_ALLOCATION_SIZE: c_uint = 0xA;
// ARG0: ATCS_FUNCTION_SET_UMA_ALLOCATION_SIZE
// ARG1:
// WORD  - structure size in bytes (includes size field)
// BYTE  - UMA size index
// BYTE  - UMA size type
// OUTPUT: none
//
