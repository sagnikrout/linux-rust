//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/amd_shared.h
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
// Copyright 2015 Advanced Micro Devices, Inc.
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

//
// Chip flags
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amd_chip_flags {
    AMD_ASIC_MASK = 0x0000ffffUL,
    AMD_FLAGS_MASK  = 0xffff0000UL,
    AMD_IS_MOBILITY = 0x00010000UL,
    AMD_IS_APU      = 0x00020000UL,
    AMD_IS_PX       = 0x00040000UL,
    AMD_EXP_HW_SUPPORT = 0x00080000UL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amd_apu_flags {
    AMD_APU_IS_RAVEN = 0x00000001UL,
    AMD_APU_IS_RAVEN2 = 0x00000002UL,
    AMD_APU_IS_PICASSO = 0x00000004UL,
    AMD_APU_IS_RENOIR = 0x00000008UL,
    AMD_APU_IS_GREEN_SARDINE = 0x00000010UL,
    AMD_APU_IS_VANGOGH = 0x00000020UL,
    AMD_APU_IS_CYAN_SKILLFISH2 = 0x00000040UL,
}

//
// DOC: IP Blocks
//
// GPUs are composed of IP (intellectual property) blocks. These
// IP blocks provide various functionalities: display, graphics,
// video decode, etc. The IP blocks that comprise a particular GPU
// are listed in the GPU's respective SoC file. amdgpu_device.c
// acquires the list of IP blocks for the GPU in use on initialization.
// It can then operate on this list to perform standard driver operations
// such as: init, fini, suspend, resume, etc.
//
// IP block implementations are named using the following convention:
// <functionality>_v<version> (E.g.: gfx_v6_0).
//
// enum amd_ip_block_type - Used to classify IP blocks by functionality.
//
// @AMD_IP_BLOCK_TYPE_COMMON: GPU Family
// @AMD_IP_BLOCK_TYPE_GMC: Graphics Memory Controller
// @AMD_IP_BLOCK_TYPE_IH: Interrupt Handler
// @AMD_IP_BLOCK_TYPE_SMC: System Management Controller
// @AMD_IP_BLOCK_TYPE_PSP: Platform Security Processor
// @AMD_IP_BLOCK_TYPE_DCE: Display and Compositing Engine
// @AMD_IP_BLOCK_TYPE_GFX: Graphics and Compute Engine
// @AMD_IP_BLOCK_TYPE_SDMA: System DMA Engine
// @AMD_IP_BLOCK_TYPE_UVD: Unified Video Decoder
// @AMD_IP_BLOCK_TYPE_VCE: Video Compression Engine
// @AMD_IP_BLOCK_TYPE_ACP: Audio Co-Processor
// @AMD_IP_BLOCK_TYPE_VCN: Video Core/Codec Next
// @AMD_IP_BLOCK_TYPE_MES: Micro-Engine Scheduler
// @AMD_IP_BLOCK_TYPE_JPEG: JPEG Engine
// @AMD_IP_BLOCK_TYPE_VPE: Video Processing Engine
// @AMD_IP_BLOCK_TYPE_UMSCH_MM: User Mode Scheduler for Multimedia
// @AMD_IP_BLOCK_TYPE_ISP: Image Signal Processor
// @AMD_IP_BLOCK_TYPE_RAS: Reliability, Availability, Serviceability
// @AMD_IP_BLOCK_TYPE_NUM: Total number of IP block types
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amd_ip_block_type {
    AMD_IP_BLOCK_TYPE_COMMON,
    AMD_IP_BLOCK_TYPE_GMC,
    AMD_IP_BLOCK_TYPE_IH,
    AMD_IP_BLOCK_TYPE_SMC,
    AMD_IP_BLOCK_TYPE_PSP,
    AMD_IP_BLOCK_TYPE_DCE,
    AMD_IP_BLOCK_TYPE_GFX,
    AMD_IP_BLOCK_TYPE_SDMA,
    AMD_IP_BLOCK_TYPE_UVD,
    AMD_IP_BLOCK_TYPE_VCE,
    AMD_IP_BLOCK_TYPE_ACP,
    AMD_IP_BLOCK_TYPE_VCN,
    AMD_IP_BLOCK_TYPE_MES,
    AMD_IP_BLOCK_TYPE_JPEG,
    AMD_IP_BLOCK_TYPE_VPE,
    AMD_IP_BLOCK_TYPE_UMSCH_MM,
    AMD_IP_BLOCK_TYPE_ISP,
    AMD_IP_BLOCK_TYPE_RAS,
    AMD_IP_BLOCK_TYPE_NUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amd_clockgating_state {
    AMD_CG_STATE_GATE = 0,
    AMD_CG_STATE_UNGATE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amd_powergating_state {
    AMD_PG_STATE_GATE = 0,
    AMD_PG_STATE_UNGATE,
}

// CG flags

// PG flags

//
// enum PP_FEATURE_MASK - Used to mask power play features.
//
// @PP_SCLK_DPM_MASK: Dynamic adjustment of the system (graphics) clock.
// @PP_MCLK_DPM_MASK: Dynamic adjustment of the memory clock.
// @PP_PCIE_DPM_MASK: Dynamic adjustment of PCIE clocks and lanes.
// @PP_SCLK_DEEP_SLEEP_MASK: System (graphics) clock deep sleep.
// @PP_POWER_CONTAINMENT_MASK: Power containment.
// @PP_UVD_HANDSHAKE_MASK: Unified video decoder handshake.
// @PP_SMC_VOLTAGE_CONTROL_MASK: Dynamic voltage control.
// @PP_VBI_TIME_SUPPORT_MASK: Vertical blank interval support.
// @PP_ULV_MASK: Ultra low voltage.
// @PP_ENABLE_GFX_CG_THRU_SMU: SMU control of GFX engine clockgating.
// @PP_CLOCK_STRETCH_MASK: Clock stretching.
// @PP_OD_FUZZY_FAN_CONTROL_MASK: Overdrive fuzzy fan control.
// @PP_SOCCLK_DPM_MASK: Dynamic adjustment of the SoC clock.
// @PP_DCEFCLK_DPM_MASK: Dynamic adjustment of the Display Controller Engine Fabric clock.
// @PP_OVERDRIVE_MASK: Over- and under-clocking support.
// @PP_GFXOFF_MASK: Dynamic graphics engine power control.
// @PP_ACG_MASK: Adaptive clock generator.
// @PP_STUTTER_MODE: Stutter mode.
// @PP_AVFS_MASK: Adaptive voltage and frequency scaling.
// @PP_GFX_DCS_MASK: GFX Async DCS.
//
// To override these settings on boot, append amdgpu.ppfeaturemask=<mask> to
// the kernel's command line parameters. This is usually done through a system's
// boot loader (E.g. GRUB). If manually loading the driver, pass
// ppfeaturemask=<mask> as a modprobe parameter.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PP_FEATURE_MASK {
    PP_SCLK_DPM_MASK = 0x1,
    PP_MCLK_DPM_MASK = 0x2,
    PP_PCIE_DPM_MASK = 0x4,
    PP_SCLK_DEEP_SLEEP_MASK = 0x8,
    PP_POWER_CONTAINMENT_MASK = 0x10,
    PP_UVD_HANDSHAKE_MASK = 0x20,
    PP_SMC_VOLTAGE_CONTROL_MASK = 0x40,
    PP_VBI_TIME_SUPPORT_MASK = 0x80,
    PP_ULV_MASK = 0x100,
    PP_ENABLE_GFX_CG_THRU_SMU = 0x200,
    PP_CLOCK_STRETCH_MASK = 0x400,
    PP_OD_FUZZY_FAN_CONTROL_MASK = 0x800,
    PP_SOCCLK_DPM_MASK = 0x1000,
    PP_DCEFCLK_DPM_MASK = 0x2000,
    PP_OVERDRIVE_MASK = 0x4000,
    PP_GFXOFF_MASK = 0x8000,
    PP_ACG_MASK = 0x10000,
    PP_STUTTER_MODE = 0x20000,
    PP_AVFS_MASK = 0x40000,
    PP_GFX_DCS_MASK = 0x80000,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amd_harvest_ip_mask {
    AMD_HARVEST_IP_VCN_MASK = 0x1,
    AMD_HARVEST_IP_JPEG_MASK = 0x2,
    AMD_HARVEST_IP_DMU_MASK = 0x4,
}

//
// enum DC_FEATURE_MASK - Bits that control DC feature defaults
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DC_FEATURE_MASK {
// Default value can be found at "uint amdgpu_dc_feature_mask"
//
// @DC_FBC_MASK: (0x1) disabled by default
//
    DC_FBC_MASK = (1 << 0),
//
// @DC_MULTI_MON_PP_MCLK_SWITCH_MASK: (0x2) enabled by default
//
    DC_MULTI_MON_PP_MCLK_SWITCH_MASK = (1 << 1),
//
// @DC_DISABLE_FRACTIONAL_PWM_MASK: (0x4) disabled by default
//
    DC_DISABLE_FRACTIONAL_PWM_MASK = (1 << 2),
//
// @DC_PSR_MASK: (0x8) disabled by default for DCN < 3.1
//
    DC_PSR_MASK = (1 << 3),
//
// @DC_EDP_NO_POWER_SEQUENCING: (0x10) disabled by default
//
    DC_EDP_NO_POWER_SEQUENCING = (1 << 4),
//
// @DC_DISABLE_LTTPR_DP1_4A: (0x20) disabled by default
//
    DC_DISABLE_LTTPR_DP1_4A = (1 << 5),
//
// @DC_DISABLE_LTTPR_DP2_0: (0x40) disabled by default
//
    DC_DISABLE_LTTPR_DP2_0 = (1 << 6),
//
// @DC_PSR_ALLOW_SMU_OPT: (0x80) disabled by default
//
    DC_PSR_ALLOW_SMU_OPT = (1 << 7),
//
// @DC_PSR_ALLOW_MULTI_DISP_OPT: (0x100) disabled by default
//
    DC_PSR_ALLOW_MULTI_DISP_OPT = (1 << 8),
//
// @DC_REPLAY_MASK: (0x200) disabled by default for DCN < 3.1.4
//
    DC_REPLAY_MASK = (1 << 9),
//
// @DC_FRL_MASK: (0x400) disabled by default
//
    DC_FRL_MASK = (1 << 10),
}

//
// enum DC_DEBUG_MASK - Bits that are useful for debugging the Display Core IP
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DC_DEBUG_MASK {
//
// @DC_DISABLE_PIPE_SPLIT: (0x1) If set, disable pipe-splitting
//
    DC_DISABLE_PIPE_SPLIT = 0x1,

//
// @DC_DISABLE_STUTTER: (0x2) If set, disable memory stutter mode
//
    DC_DISABLE_STUTTER = 0x2,

//
// @DC_DISABLE_DSC: (0x4) If set, disable display stream compression
//
    DC_DISABLE_DSC = 0x4,

//
// @DC_DISABLE_CLOCK_GATING: (0x8) If set, disable clock gating optimizations
//
    DC_DISABLE_CLOCK_GATING = 0x8,

//
// @DC_DISABLE_PSR: (0x10) If set, disable Panel self refresh v1 and PSR-SU
//
    DC_DISABLE_PSR = 0x10,

//
// @DC_FORCE_SUBVP_MCLK_SWITCH: (0x20) If set, force mclk switch in subvp, even
// if mclk switch in vblank is possible
//
    DC_FORCE_SUBVP_MCLK_SWITCH = 0x20,

//
// @DC_DISABLE_MPO: (0x40) If set, disable multi-plane offloading
//
    DC_DISABLE_MPO = 0x40,

//
// @DC_ENABLE_DPIA_TRACE: (0x80) If set, enable trace logging for DPIA
//
    DC_ENABLE_DPIA_TRACE = 0x80,

//
// @DC_ENABLE_DML2: (0x100) If set, force usage of DML2, even if the DCN version
// does not default to it.
//
    DC_ENABLE_DML2 = 0x100,

//
// @DC_DISABLE_PSR_SU: (0x200) If set, disable PSR SU
//
    DC_DISABLE_PSR_SU = 0x200,

//
// @DC_DISABLE_REPLAY: (0x400) If set, disable Panel Replay
//
    DC_DISABLE_REPLAY = 0x400,

//
// @DC_DISABLE_IPS: (0x800) If set, disable all Idle Power States, all the time.
// If more than one IPS debug bit is set, the lowest bit takes
// precedence. For example, if DC_FORCE_IPS_ENABLE and
// DC_DISABLE_IPS_DYNAMIC are set, then DC_DISABLE_IPS_DYNAMIC takes
// precedence.
//
    DC_DISABLE_IPS = 0x800,

//
// @DC_DISABLE_IPS_DYNAMIC: (0x1000) If set, disable all IPS, all the time,
// *except* when driver goes into suspend.
//
    DC_DISABLE_IPS_DYNAMIC = 0x1000,

//
// @DC_DISABLE_IPS2_DYNAMIC: (0x2000) If set, disable IPS2 (IPS1 allowed) if
// there is an enabled display. Otherwise, enable all IPS.
//
    DC_DISABLE_IPS2_DYNAMIC = 0x2000,

//
// @DC_FORCE_IPS_ENABLE: (0x4000) If set, force enable all IPS, all the time.
//
    DC_FORCE_IPS_ENABLE = 0x4000,
//
// @DC_DISABLE_ACPI_EDID: (0x8000) If set, don't attempt to fetch EDID for
// eDP display from ACPI _DDC method.
//
    DC_DISABLE_ACPI_EDID = 0x8000,

//
// @DC_DISABLE_HDMI_CEC: (0x10000) If set, disable HDMI-CEC feature in amdgpu driver.
//
    DC_DISABLE_HDMI_CEC = 0x10000,

//
// @DC_DISABLE_SUBVP_FAMS: (0x20000) If set, disable DCN Sub-Viewport & Firmware Assisted
// Memory Clock Switching (FAMS) feature in amdgpu driver.
//
    DC_DISABLE_SUBVP_FAMS = 0x20000,
//
// @DC_DISABLE_CUSTOM_BRIGHTNESS_CURVE: (0x40000) If set, disable support for custom
// brightness curves
//
    DC_DISABLE_CUSTOM_BRIGHTNESS_CURVE = 0x40000,

//
// @DC_HDCP_LC_FORCE_FW_ENABLE: (0x80000) If set, use HDCP Locality Check FW
// path regardless of reported HW capabilities.
//
    DC_HDCP_LC_FORCE_FW_ENABLE = 0x80000,

//
// @DC_HDCP_LC_ENABLE_SW_FALLBACK: (0x100000) If set, upon HDCP Locality Check FW
// path failure, retry using legacy SW path.
//
    DC_HDCP_LC_ENABLE_SW_FALLBACK = 0x100000,

//
// @DC_SKIP_DETECTION_LT: (0x200000) If set, skip detection link training
//
    DC_SKIP_DETECTION_LT = 0x200000,
}

//
// struct amd_ip_funcs - general hooks for managing amdgpu IP Blocks
// @name: Name of IP block
// @early_init: sets up early driver state (pre sw_init),
// does not configure hw - Optional
// @late_init: sets up late driver/hw state (post hw_init) - Optional
// @sw_init: sets up driver state, does not configure hw
// @sw_fini: tears down driver state, does not configure hw
// @early_fini: tears down stuff before dev detached from driver
// @hw_init: sets up the hw state
// @hw_fini: tears down the hw state
// @late_fini: final cleanup
// @prepare_suspend: handle IP specific changes to prepare for suspend
// (such as allocating any required memory)
// @suspend: handles IP specific hw/sw changes for suspend
// @resume: handles IP specific hw/sw changes for resume
// @complete: handles IP specific changes after resume
// @is_idle: returns current IP block idle status
// @wait_for_idle: poll for idle
// @soft_reset: soft reset the IP block
// @set_clockgating_state: enable/disable cg for the IP block
// @set_powergating_state: enable/disable pg for the IP block
// @get_clockgating_state: get current clockgating status
// @dump_ip_state: dump the IP state of the ASIC during a gpu hang
// @print_ip_state: print the IP state in devcoredump for each IP of the ASIC
//
// These hooks provide an interface for controlling the operational state
// of IP blocks. After acquiring a list of IP blocks for the GPU in use,
// the driver can make chip-wide state changes by walking this list and
// making calls to hooks from each IP block. This list is ordered to ensure
// that the driver initializes the IP blocks in a safe sequence.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_ip_funcs {
    pub name: *mut c_char,
    pub ip_block): *mut *mut int (early_init)(struct amdgpu_ip_block,
    pub ip_block): *mut *mut int (late_init)(struct amdgpu_ip_block,
    pub ip_block): *mut *mut int (sw_init)(struct amdgpu_ip_block,
    pub ip_block): *mut *mut int (sw_fini)(struct amdgpu_ip_block,
    pub ip_block): *mut *mut int (early_fini)(struct amdgpu_ip_block,
    pub ip_block): *mut *mut int (hw_init)(struct amdgpu_ip_block,
    pub ip_block): *mut *mut int (hw_fini)(struct amdgpu_ip_block,
    pub ip_block): *mut *mut void (late_fini)(struct amdgpu_ip_block,
    pub ip_block): *mut *mut int (prepare_suspend)(struct amdgpu_ip_block,
    pub ip_block): *mut *mut int (suspend)(struct amdgpu_ip_block,
    pub ip_block): *mut *mut int (resume)(struct amdgpu_ip_block,
    pub ip_block): *mut *mut void (complete)(struct amdgpu_ip_block,
    pub ip_block): *mut *mut bool (is_idle)(struct amdgpu_ip_block,
    pub ip_block): *mut *mut int (wait_for_idle)(struct amdgpu_ip_block,
    pub ip_block): *mut *mut int (soft_reset)(struct amdgpu_ip_block,
    pub state): amd_clockgating_state,
    pub state): amd_powergating_state,
    pub flags): *mut *mut *mut void (get_clockgating_state)(struct amdgpu_ip_block ip_block, u64,
    pub ip_block): *mut *mut void (dump_ip_state)(struct amdgpu_ip_block,
    pub p): *mut *mut *mut void (print_ip_state)(struct amdgpu_ip_block ip_block, struct drm_printer,
}
