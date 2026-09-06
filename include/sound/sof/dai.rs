//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/sof/dai.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
//

//
// DAI Configuration.
//
// Each different DAI type will have it's own structure and IPC cmd.
//

// keep old definitions for backwards compatibility

pub const SOF_DAI_FMT_FORMAT_MASK: c_uint = 0x000f;
pub const SOF_DAI_FMT_CLOCK_MASK: c_uint = 0x00f0;
pub const SOF_DAI_FMT_INV_MASK: c_uint = 0x0f00;
pub const SOF_DAI_FMT_CLOCK_PROVIDER_MASK: c_uint = 0xf000;
//
// DAI_CONFIG flags. The 4 LSB bits are used for the commands, HW_PARAMS, HW_FREE and PAUSE
// representing when the IPC is sent. The 4 MSB bits are used to add quirks along with the above
// commands.
//
pub const SOF_DAI_CONFIG_FLAGS_CMD_MASK: c_uint = 0xF;

// < DAI_CONFIG sent during pause trigger. Only available ABI 3.20 onwards

pub const SOF_DAI_CONFIG_FLAGS_QUIRK_SHIFT: c_int = 4;

//
// This should be used along with the SOF_DAI_CONFIG_FLAGS_HW_PARAMS to indicate that pipeline
// stop/pause and DAI DMA stop/pause should happen in two steps. This change is only available
// ABI 3.20 onwards.
//

// \brief Types of DAI
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_ipc_dai_type {
    SOF_DAI_INTEL_NONE = 0,		/**< None */
    SOF_DAI_INTEL_SSP,		/**< Intel SSP */
    SOF_DAI_INTEL_DMIC,		/**< Intel DMIC */
    SOF_DAI_INTEL_HDA,		/**< Intel HD/A */
    SOF_DAI_INTEL_ALH,		/**< Intel ALH  */
    SOF_DAI_IMX_SAI,		/**< i.MX SAI */
    SOF_DAI_IMX_ESAI,		/**< i.MX ESAI */
    SOF_DAI_AMD_BT,			/**< AMD ACP BT*/
    SOF_DAI_AMD_SP,			/**< AMD ACP SP */
    SOF_DAI_AMD_DMIC,		/**< AMD ACP DMIC */
    SOF_DAI_MEDIATEK_AFE,		/**< Mediatek AFE */
    SOF_DAI_AMD_HS,			/**< Amd HS */
    SOF_DAI_AMD_SP_VIRTUAL,		/**< AMD ACP SP VIRTUAL */
    SOF_DAI_AMD_HS_VIRTUAL,		/**< AMD ACP HS VIRTUAL */
    SOF_DAI_IMX_MICFIL,		/** < i.MX MICFIL PDM */
    SOF_DAI_AMD_SDW,		/**< AMD ACP SDW */
    SOF_DAI_INTEL_UAOL,		/**< Intel UAOL */
    SOF_DAI_AMD_I2S,		/**< AMD ACP I2S */
}

// general purpose DAI configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_dai_config {
    pub hdr: sof_ipc_cmd_hdr,
    pub /: *mut *mut *mut uint32_t type; /< DAI type - enum sof_ipc_dai_type,
    pub /: *mut *mut *mut uint32_t dai_index; /< index of this type dai,
// physical protocol and clocking
    pub /: *mut *mut *mut uint16_t format; /< SOF_DAI_FMT_,
    pub /: *mut *mut *mut uint8_t group_id; /< group ID, 0 means no group (ABI 3.17),
    pub /: *mut *mut *mut uint8_t flags; /< SOF_DAI_CONFIG_FLAGS_ (ABI 3.19),
// reserved for future use
    pub reserved: [u32; 8],
// HW specific data
    pub ssp: sof_ipc_dai_ssp_params,
    pub dmic: sof_ipc_dai_dmic_params,
    pub hda: sof_ipc_dai_hda_params,
    pub alh: sof_ipc_dai_alh_params,
    pub esai: sof_ipc_dai_esai_params,
    pub sai: sof_ipc_dai_sai_params,
    pub acpbt: sof_ipc_dai_acp_params,
    pub acpsp: sof_ipc_dai_acp_params,
    pub acpdmic: sof_ipc_dai_acpdmic_params,
    pub acphs: sof_ipc_dai_acp_params,
    pub afe: sof_ipc_dai_mtk_afe_params,
    pub micfil: sof_ipc_dai_micfil_params,
    pub acp_sdw: sof_ipc_dai_acp_sdw_params,
    pub acp_i2s: sof_ipc_dai_acp_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_dai_private_data {
    pub comp_dai: *mut sof_ipc_comp_dai,
    pub dai_config: *mut sof_ipc_dai_config,
}
