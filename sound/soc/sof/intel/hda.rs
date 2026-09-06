//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/sof/intel/hda.h
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
// Copyright(c) 2017 Intel Corporation
//
// Author: Liam Girdwood <liam.r.girdwood@linux.intel.com>
//

// PCI registers
pub const PCI_TCSEL: c_uint = 0x44;

pub const PCI_CGCTL: c_uint = 0x48;
// PCI_PGCTL bits

// PCI_CGCTL bits

// Legacy HDA registers and bits used - widths are variable
pub const SOF_HDA_GCAP: c_uint = 0x0;
pub const SOF_HDA_GCTL: c_uint = 0x8;
// accept unsol. response enable

pub const SOF_HDA_LLCH: c_uint = 0x14;
pub const SOF_HDA_INTCTL: c_uint = 0x20;
pub const SOF_HDA_INTSTS: c_uint = 0x24;
pub const SOF_HDA_WAKESTS: c_uint = 0x0E;

pub const SOF_HDA_RIRBSTS: c_uint = 0x5d;
// SOF_HDA_GCTL register bist

// SOF_HDA_INCTL regs

pub const SOF_HDA_INT_ALL_STREAM: c_uint = 0xff;
// SOF_HDA_INTSTS regs

pub const SOF_HDA_MAX_CAPS: c_int = 10;
pub const SOF_HDA_CAP_ID_OFF: c_int = 16;

pub const SOF_HDA_CAP_NEXT_MASK: c_uint = 0xFFFF;
pub const SOF_HDA_GTS_CAP_ID: c_uint = 0x1;
pub const SOF_HDA_ML_CAP_ID: c_uint = 0x2;
pub const SOF_HDA_PP_CAP_ID: c_uint = 0x3;
pub const SOF_HDA_REG_PP_PPCH: c_uint = 0x10;
pub const SOF_HDA_REG_PP_PPCTL: c_uint = 0x04;
pub const SOF_HDA_REG_PP_PPSTS: c_uint = 0x08;

// Vendor Specific Registers
pub const SOF_HDA_VS_D0I3C: c_uint = 0x104A;
// D0I3C Register fields

// DPIB entry size: 8 Bytes = 2 DWords
pub const SOF_HDA_DPIB_ENTRY_SIZE: c_uint = 0x8;
pub const SOF_HDA_SPIB_CAP_ID: c_uint = 0x4;
pub const SOF_HDA_DRSM_CAP_ID: c_uint = 0x5;
pub const SOF_HDA_SPIB_BASE: c_uint = 0x08;
pub const SOF_HDA_SPIB_INTERVAL: c_uint = 0x08;
pub const SOF_HDA_SPIB_SPIB: c_uint = 0x00;
pub const SOF_HDA_SPIB_MAXFIFO: c_uint = 0x04;
pub const SOF_HDA_PPHC_BASE: c_uint = 0x10;
pub const SOF_HDA_PPHC_INTERVAL: c_uint = 0x10;
pub const SOF_HDA_PPLC_BASE: c_uint = 0x10;
pub const SOF_HDA_PPLC_MULTI: c_uint = 0x10;
pub const SOF_HDA_PPLC_INTERVAL: c_uint = 0x10;
pub const SOF_HDA_DRSM_BASE: c_uint = 0x08;
pub const SOF_HDA_DRSM_INTERVAL: c_uint = 0x08;
// Descriptor error interrupt
pub const SOF_HDA_CL_DMA_SD_INT_DESC_ERR: c_uint = 0x10;
// FIFO error interrupt
pub const SOF_HDA_CL_DMA_SD_INT_FIFO_ERR: c_uint = 0x08;
// Buffer completion interrupt
pub const SOF_HDA_CL_DMA_SD_INT_COMPLETE: c_uint = 0x04;

pub const SOF_HDA_SD_CTL_DMA_START: c_uint = 0x02 /* Stream DMA start bit */;
// Intel HD Audio Code Loader DMA Registers
pub const SOF_HDA_ADSP_LOADER_BASE: c_uint = 0x80;
pub const SOF_HDA_ADSP_DPLBASE: c_uint = 0x70;
pub const SOF_HDA_ADSP_DPUBASE: c_uint = 0x74;
pub const SOF_HDA_ADSP_DPLBASE_ENABLE: c_uint = 0x01;
// Stream Registers
pub const SOF_HDA_ADSP_REG_SD_CTL: c_uint = 0x00;
pub const SOF_HDA_ADSP_REG_SD_STS: c_uint = 0x03;
pub const SOF_HDA_ADSP_REG_SD_LPIB: c_uint = 0x04;
pub const SOF_HDA_ADSP_REG_SD_CBL: c_uint = 0x08;
pub const SOF_HDA_ADSP_REG_SD_LVI: c_uint = 0x0C;
pub const SOF_HDA_ADSP_REG_SD_FIFOW: c_uint = 0x0E;
pub const SOF_HDA_ADSP_REG_SD_FIFOSIZE: c_uint = 0x10;
pub const SOF_HDA_ADSP_REG_SD_FORMAT: c_uint = 0x12;
pub const SOF_HDA_ADSP_REG_SD_FIFOL: c_uint = 0x14;
pub const SOF_HDA_ADSP_REG_SD_BDLPL: c_uint = 0x18;
pub const SOF_HDA_ADSP_REG_SD_BDLPU: c_uint = 0x1C;
pub const SOF_HDA_ADSP_SD_ENTRY_SIZE: c_uint = 0x20;
// SDxFIFOS FIFOS

// CL: Software Position Based FIFO Capability Registers

pub const SOF_HDA_ADSP_REG_CL_SPBFIFO_SPBFCH: c_uint = 0x0;
pub const SOF_HDA_ADSP_REG_CL_SPBFIFO_SPBFCCTL: c_uint = 0x4;
pub const SOF_HDA_ADSP_REG_CL_SPBFIFO_SPIB: c_uint = 0x8;
pub const SOF_HDA_ADSP_REG_CL_SPBFIFO_MAXFIFOS: c_uint = 0xc;
// Stream Number
pub const SOF_HDA_CL_SD_CTL_STREAM_TAG_SHIFT: c_int = 20;

pub const HDA_DSP_HDA_BAR: c_int = 0;
pub const HDA_DSP_PP_BAR: c_int = 1;
pub const HDA_DSP_SPIB_BAR: c_int = 2;
pub const HDA_DSP_DRSM_BAR: c_int = 3;
pub const HDA_DSP_BAR: c_int = 4;

// SRAM window 0 FW "registers"

// FW and ROM share offset 4

pub const HDA_DSP_MBOX_UPLINK_OFFSET: c_uint = 0x81000;
pub const HDA_DSP_STREAM_RESET_TIMEOUT: c_int = 300;
//
// Timeout in us, for setting the stream RUN bit, during
// start/stop the stream. The timeout expires if new RUN bit
// value cannot be read back within the specified time.
//
pub const HDA_DSP_STREAM_RUN_TIMEOUT: c_int = 300;
pub const HDA_DSP_SPIB_ENABLE: c_int = 1;
pub const HDA_DSP_SPIB_DISABLE: c_int = 0;

pub const HDA_DSP_STACK_DUMP_SIZE: c_int = 32;
// ROM/FW status register

// Wait states
pub const FSR_WAIT_FOR_IPC_BUSY: c_uint = 0x1;
pub const FSR_WAIT_FOR_IPC_DONE: c_uint = 0x2;
pub const FSR_WAIT_FOR_CACHE_INVALIDATION: c_uint = 0x3;
pub const FSR_WAIT_FOR_LP_SRAM_OFF: c_uint = 0x4;
pub const FSR_WAIT_FOR_DMA_BUFFER_FULL: c_uint = 0x5;
pub const FSR_WAIT_FOR_CSE_CSR: c_uint = 0x6;
// Module codes
pub const FSR_MOD_ROM: c_uint = 0x0;
pub const FSR_MOD_ROM_BYP: c_uint = 0x1;
pub const FSR_MOD_BASE_FW: c_uint = 0x2;
pub const FSR_MOD_LP_BOOT: c_uint = 0x3;
pub const FSR_MOD_BRNGUP: c_uint = 0x4;
pub const FSR_MOD_ROM_EXT: c_uint = 0x5;
// State codes (module dependent)
// Module independent states
pub const FSR_STATE_INIT: c_uint = 0x0;
pub const FSR_STATE_INIT_DONE: c_uint = 0x1;
pub const FSR_STATE_FW_ENTERED: c_uint = 0x5;
// ROM states

pub const FSR_STATE_ROM_CSE_MANIFEST_LOADED: c_uint = 0x2;
pub const FSR_STATE_ROM_FW_MANIFEST_LOADED: c_uint = 0x3;
pub const FSR_STATE_ROM_FW_FW_LOADED: c_uint = 0x4;

pub const FSR_STATE_ROM_VERIFY_FEATURE_MASK: c_uint = 0x6;
pub const FSR_STATE_ROM_GET_LOAD_OFFSET: c_uint = 0x7;
pub const FSR_STATE_ROM_FETCH_ROM_EXT: c_uint = 0x8;
pub const FSR_STATE_ROM_FETCH_ROM_EXT_DONE: c_uint = 0x9;
pub const FSR_STATE_ROM_BASEFW_ENTERED: c_uint = 0xf /* SKL */;
// (ROM) CSE states
pub const FSR_STATE_ROM_CSE_IMR_REQUEST: c_uint = 0x10;
pub const FSR_STATE_ROM_CSE_IMR_GRANTED: c_uint = 0x11;
pub const FSR_STATE_ROM_CSE_VALIDATE_IMAGE_REQUEST: c_uint = 0x12;
pub const FSR_STATE_ROM_CSE_IMAGE_VALIDATED: c_uint = 0x13;
pub const FSR_STATE_ROM_CSE_IPC_IFACE_INIT: c_uint = 0x20;
pub const FSR_STATE_ROM_CSE_IPC_RESET_PHASE_1: c_uint = 0x21;
pub const FSR_STATE_ROM_CSE_IPC_OPERATIONAL_ENTRY: c_uint = 0x22;
pub const FSR_STATE_ROM_CSE_IPC_OPERATIONAL: c_uint = 0x23;
pub const FSR_STATE_ROM_CSE_IPC_DOWN: c_uint = 0x24;
// BRINGUP (or BRNGUP) states

pub const FSR_STATE_BRINGUP_HPSRAM_LOAD: c_uint = 0x2;

pub const FSR_STATE_BRINGUP_IMR_RESTORE: c_uint = 0x4;

// ROM  status/error values
pub const HDA_DSP_ROM_CSE_ERROR: c_int = 40;
pub const HDA_DSP_ROM_CSE_WRONG_RESPONSE: c_int = 41;
pub const HDA_DSP_ROM_IMR_TO_SMALL: c_int = 42;
pub const HDA_DSP_ROM_BASE_FW_NOT_FOUND: c_int = 43;
pub const HDA_DSP_ROM_CSE_VALIDATION_FAILED: c_int = 44;
pub const HDA_DSP_ROM_IPC_FATAL_ERROR: c_int = 45;
pub const HDA_DSP_ROM_L2_CACHE_ERROR: c_int = 46;
pub const HDA_DSP_ROM_LOAD_OFFSET_TO_SMALL: c_int = 47;
pub const HDA_DSP_ROM_API_PTR_INVALID: c_int = 50;
pub const HDA_DSP_ROM_BASEFW_INCOMPAT: c_int = 51;
pub const HDA_DSP_ROM_UNHANDLED_INTERRUPT: c_uint = 0xBEE00000;
pub const HDA_DSP_ROM_MEMORY_HOLE_ECC: c_uint = 0xECC00000;
pub const HDA_DSP_ROM_KERNEL_EXCEPTION: c_uint = 0xCAFE0000;
pub const HDA_DSP_ROM_USER_EXCEPTION: c_uint = 0xBEEF0000;
pub const HDA_DSP_ROM_UNEXPECTED_RESET: c_uint = 0xDECAF000;
pub const HDA_DSP_ROM_NULL_FW_ENTRY: c_uint = 0x4c4c4e55;
pub const HDA_DSP_ROM_IPC_CONTROL: c_uint = 0x01000000;
pub const HDA_DSP_ROM_IPC_PURGE_FW: c_uint = 0x00004000;
// various timeout values
pub const HDA_DSP_PU_TIMEOUT: c_int = 50;
pub const HDA_DSP_PD_TIMEOUT: c_int = 50;
pub const HDA_DSP_RESET_TIMEOUT_US: c_int = 50000;
pub const HDA_DSP_BASEFW_TIMEOUT_US: c_int = 3000000;
pub const HDA_DSP_INIT_TIMEOUT_US: c_int = 500000;
pub const HDA_DSP_CTRL_RESET_TIMEOUT: c_int = 100;

pub const HDA_DSP_REG_POLL_RETRY_COUNT: c_int = 50;

// Intel HD Audio General DSP Registers
pub const HDA_DSP_GEN_BASE: c_uint = 0x0;

// Intel HD Audio Inter-Processor Communication Registers
pub const HDA_DSP_IPC_BASE: c_uint = 0x40;

// Intel Vendor Specific Registers
pub const HDA_VS_INTEL_EM2: c_uint = 0x1030;

pub const HDA_VS_INTEL_LTRP: c_uint = 0x1048;
pub const HDA_VS_INTEL_LTRP_GB_MASK: c_uint = 0x3F;
// HIPCI

pub const HDA_DSP_REG_HIPCI_MSG_MASK: c_uint = 0x7FFFFFFF;
// HIPCIE

pub const HDA_DSP_REG_HIPCIE_MSG_MASK: c_uint = 0x3FFFFFFF;
// HIPCCTL

// HIPCT

pub const HDA_DSP_REG_HIPCT_MSG_MASK: c_uint = 0x7FFFFFFF;
// HIPCTE
pub const HDA_DSP_REG_HIPCTE_MSG_MASK: c_uint = 0x3FFFFFFF;

// Delay before scheduling D0i3 entry
pub const BXT_D0I3_DELAY: c_int = 5000;
pub const FW_CL_STREAM_NUMBER: c_uint = 0x1;
pub const HDA_FW_BOOT_ATTEMPTS: c_int = 3;
// ADSPCS - Audio DSP Control & Status
//
// Core Reset - asserted high
// CRST Mask for a given core mask pattern, cm
//
pub const HDA_DSP_ADSPCS_CRST_SHIFT: c_int = 0;

//
// Core run/stall - when set to '1' core is stalled
// CSTALL Mask for a given core mask pattern, cm
//
pub const HDA_DSP_ADSPCS_CSTALL_SHIFT: c_int = 8;

//
// Set Power Active - when set to '1' turn cores on
// SPA Mask for a given core mask pattern, cm
//
pub const HDA_DSP_ADSPCS_SPA_SHIFT: c_int = 16;

//
// Current Power Active - power status of cores, set by hardware
// CPA Mask for a given core mask pattern, cm
//
pub const HDA_DSP_ADSPCS_CPA_SHIFT: c_int = 24;

//
// Mask for a given number of cores
// nc = number of supported cores
//

// Intel HD Audio Inter-Processor Communication Registers for Cannonlake
pub const CNL_DSP_IPC_BASE: c_uint = 0xc0;

// HIPCI

pub const CNL_DSP_REG_HIPCIDR_MSG_MASK: c_uint = 0x7FFFFFFF;
// HIPCIE

pub const CNL_DSP_REG_HIPCIDA_MSG_MASK: c_uint = 0x7FFFFFFF;
// HIPCCTL

// HIPCT

pub const CNL_DSP_REG_HIPCTDR_MSG_MASK: c_uint = 0x7FFFFFFF;
// HIPCTDA

pub const CNL_DSP_REG_HIPCTDA_MSG_MASK: c_uint = 0x7FFFFFFF;
// HIPCTDD
pub const CNL_DSP_REG_HIPCTDD_MSG_MASK: c_uint = 0x7FFFFFFF;
// BDL
pub const HDA_DSP_BDL_SIZE: c_int = 4096;

// Number of DAIs
pub const SOF_SKL_NUM_DAIS_NOCODEC: c_int = 9;

pub const SOF_SKL_NUM_DAIS: c_int = 16;

// Intel HD Audio SRAM Window 0
pub const HDA_DSP_SRAM_REG_ROM_STATUS_SKL: c_uint = 0x8000;
pub const HDA_ADSP_SRAM0_BASE_SKL: c_uint = 0x8000;
// Firmware status window

// Host Device Memory Space
pub const APL_SSP_BASE_OFFSET: c_uint = 0x2000;
pub const CNL_SSP_BASE_OFFSET: c_uint = 0x10000;
// Host Device Memory Size of a Single SSP
pub const SSP_DEV_MEM_SIZE: c_uint = 0x1000;
// SSP Count of the Platform
pub const APL_SSP_COUNT: c_int = 6;
pub const CNL_SSP_COUNT: c_int = 3;
pub const ICL_SSP_COUNT: c_int = 6;
pub const TGL_SSP_COUNT: c_int = 3;
pub const MTL_SSP_COUNT: c_int = 3;
// SSP Registers
pub const SSP_SSC1_OFFSET: c_uint = 0x4;

pub const HDA_EXT_ADDR: c_int = 0;

pub const HDA_IDISP_ADDR: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_intel_dsp_bdl {
    pub addr_l: __le32,
    pub addr_h: __le32,
    pub size: __le32,
    pub ioc: __le32,
    pub __attribute((packed)): },
pub const SOF_HDA_PLAYBACK_STREAMS: c_int = 16;
pub const SOF_HDA_CAPTURE_STREAMS: c_int = 16;
pub const SOF_HDA_PLAYBACK: c_int = 0;
pub const SOF_HDA_CAPTURE: c_int = 1;
// stream flags
pub const SOF_HDA_STREAM_DMI_L1_COMPATIBLE: c_int = 1;
//
// Time in ms for opportunistic D0I3 entry delay.
// This has been deliberately chosen to be long to avoid race conditions.
// Could be optimized in future.
//
pub const SOF_HDA_D0I3_WORK_DELAY_MS: c_int = 5000;
// HDA DSP D0 substate
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_hda_D0_substate {
    SOF_HDA_DSP_PM_D0I0,	/* default D0 substate */
    SOF_HDA_DSP_PM_D0I3,	/* low power D0 substate */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ace3_mic_privacy {
    pub active: bool,
    pub work: work_struct,
}

// represents DSP HDA controller frontend - i.e. host facing control
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_intel_hda_dev {
    pub imrboot_supported: bool,
    pub skip_imr_boot: bool,
    pub booted_from_imr: bool,
    pub boot_iteration: c_int,
//
// DMA buffers for base firmware download. By default the buffers are
// allocated once and kept through the lifetime of the driver.
// See module parameter: persistent_cl_buffer
//
    pub cl_dmab: snd_dma_buffer,
    pub cl_dmab_contains_basefw: bool,
    pub iccmax_dmab: snd_dma_buffer,
    pub hbus: hda_bus,
// hw config
    pub desc: *const sof_intel_dsp_desc,
// trace
    pub dtrace_stream: *mut hdac_ext_stream,
// if position update IPC needed
    pub no_ipc_position: u32,
// the maximum number of streams (playback + capture) supported
    pub stream_max: u32,
//
// ACE2+ link DMA stream allocation constraints (stream index =
// stream_tag - 1, shared between input and output directions). All
// masks are cleared by hda_dsp_ctrl_init_chip() on controller reset
// (CRST#).
//
// - Concurrent (cross-direction) constraint: a SoundWire stream and
// a HDA/iDisp/UAOL stream cannot share a physical stream index
// across directions, the resulting LLP/timestamp values are wrong.
// link_dma_active_sdw_mask and link_dma_active_multi_mask
// (indexed by SNDRV_PCM_STREAM_*) track currently allocated
// streams per direction in each of the conflicting groups; SSP
// and DMIC do not participate. Bits are cleared on stream release.
//
// - Sequential (playback only) constraint: once a HDA/iDisp link
// has used a playback stream index, that index cannot drive a
// non-HDA/iDisp link in the same direction until the next CRST#.
// link_dma_out_hda_used_mask records this.
//
    pub 1]: u32 link_dma_active_sdw_mask[SNDRV_PCM_STREAM_LAST +,
    pub 1]: u32 link_dma_active_multi_mask[SNDRV_PCM_STREAM_LAST +,
    pub link_dma_out_hda_used_mask: u32,
// PM related
    pub /: *mut *mut bool l1_disabled;/ is DMI link L1 disabled?,
// DMIC device
    pub dmic_dev: *mut platform_device,
// delayed work to enter D0I3 opportunistically
    pub d0i3_work: delayed_work,
// ACPI information stored between scan and probe steps
    pub info: sdw_intel_acpi_info,
// sdw context allocated by SoundWire driver
    pub sdw: *mut sdw_intel_ctx,
// FW clock config, 0:HPRO, 1:LPRO
    pub clk_config_lpro: bool,
    pub waitq: wait_queue_head_t,
    pub code_loading: bool,
// Intel NHLT information
    pub nhlt: *mut nhlt_acpi_table,
// work queue for mic privacy state change notification sending
    pub mic_privacy: sof_ace3_mic_privacy,
//
// Pointing to the IPC message if immediate sending was not possible
// because the downlink communication channel was BUSY at the time.
// The message will be re-tried when the channel becomes free (the ACK
// is received from the DSP for the previous message)
//
    pub delayed_ipc_tx_msg: *mut snd_sof_ipc_msg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_intel_hda_stream {
    pub sdev: *mut snd_sof_dev,
    pub hext_stream: hdac_ext_stream,
    pub sof_intel_stream: sof_intel_stream,
    pub /: *mut *mut int host_reserved; / reserve host DMA channel,
    pub flags: u32,
    pub ioc: completion,
}

pub const SOF_STREAM_SD_OFFSET_CRST: c_uint = 0x1;
//
// DAI support
//
extern "C" {
    pub fn hda_is_chain_dma_supported(sdev: *mut snd_sof_dev, dai_type: u32) -> bool;
}
//
// DSP Core services.
//
extern "C" {
    pub fn hda_dsp_probe_early(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn hda_dsp_probe(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn hda_dsp_remove(sdev: *mut snd_sof_dev);
}
extern "C" {
    pub fn hda_dsp_remove_late(sdev: *mut snd_sof_dev);
}
extern "C" {
    pub fn hda_dsp_core_power_up(sdev: *mut snd_sof_dev, core_mask: c_uint) -> c_int;
}
extern "C" {
    pub fn hda_dsp_core_run(sdev: *mut snd_sof_dev, core_mask: c_uint) -> c_int;
}
extern "C" {
    pub fn hda_dsp_enable_core(sdev: *mut snd_sof_dev, core_mask: c_uint) -> c_int;
}
extern "C" {
    pub fn hda_power_down_dsp(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn hda_dsp_core_get(sdev: *mut snd_sof_dev, core: c_int) -> c_int;
}
extern "C" {
    pub fn hda_dsp_ipc_int_enable(sdev: *mut snd_sof_dev);
}
extern "C" {
    pub fn hda_dsp_ipc_int_disable(sdev: *mut snd_sof_dev);
}
extern "C" {
    pub fn hda_dsp_core_is_enabled(sdev: *mut snd_sof_dev, core_mask: c_uint) -> bool;
}
extern "C" {
    pub fn hda_dsp_suspend(sdev: *mut snd_sof_dev, target_state: u32) -> c_int;
}
extern "C" {
    pub fn hda_dsp_resume(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn hda_dsp_runtime_suspend(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn hda_dsp_runtime_resume(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn hda_dsp_runtime_idle(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn hda_dsp_shutdown_dma_flush(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn hda_dsp_shutdown(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn hda_dsp_set_hw_params_upon_resume(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn hda_dsp_dump(sdev: *mut snd_sof_dev, flags: u32);
}
extern "C" {
    pub fn hda_ipc4_dsp_dump(sdev: *mut snd_sof_dev, flags: u32);
}
extern "C" {
    pub fn hda_ipc_dump(sdev: *mut snd_sof_dev);
}
extern "C" {
    pub fn hda_ipc_irq_dump(sdev: *mut snd_sof_dev);
}
extern "C" {
    pub fn hda_dsp_d0i3_work(work: *mut work_struct);
}
extern "C" {
    pub fn hda_dsp_disable_interrupts(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn hda_check_ipc_irq(sdev: *mut snd_sof_dev) -> bool;
}
extern "C" {
    pub fn hda_get_interface_mask(sdev: *mut snd_sof_dev) -> u32;
}
//
// DSP PCM Operations.
//
extern "C" {
    pub fn hda_dsp_get_mult_div(sdev: *mut snd_sof_dev, rate: c_int) -> u32;
}
extern "C" {
    pub fn hda_dsp_get_bits(sdev: *mut snd_sof_dev, sample_bits: c_int) -> u32;
}
extern "C" {
    pub fn hda_dsp_pcm_ack(sdev: *mut snd_sof_dev, substream: *mut snd_pcm_substream) -> c_int;
}
//
// DSP Stream Operations.
//
extern "C" {
    pub fn hda_dsp_stream_init(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn hda_dsp_stream_free(sdev: *mut snd_sof_dev);
}
extern "C" {
    pub fn hda_dsp_stream_threaded_handler(irq: c_int, context: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn hda_dsp_check_ipc_irq(sdev: *mut snd_sof_dev) -> bool;
}
extern "C" {
    pub fn hda_dsp_check_stream_irq(sdev: *mut snd_sof_dev) -> bool;
}
extern "C" {
    pub fn hda_dsp_stream_put(sdev: *mut snd_sof_dev, direction: c_int, stream_tag: c_int) -> c_int;
}
extern "C" {
    pub fn hda_dsp_stream_pair_put(sdev: *mut snd_sof_dev, direction: c_int, stream_tag: c_int) -> c_int;
}
//
// DSP IPC Operations.
//
extern "C" {
    pub fn hda_dsp_ipc_get_reply(sdev: *mut snd_sof_dev);
}
extern "C" {
    pub fn hda_dsp_ipc_get_mailbox_offset(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn hda_dsp_ipc_get_window_offset(sdev: *mut snd_sof_dev, id: u32) -> c_int;
}
extern "C" {
    pub fn hda_dsp_ipc_irq_thread(irq: c_int, context: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn hda_dsp_ipc_cmd_done(sdev: *mut snd_sof_dev, dir: c_int) -> c_int;
}
extern "C" {
    pub fn hda_dsp_get_state(sdev: *mut snd_sof_dev, level: *const c_char);
}
//
// DSP Code loader.
//
extern "C" {
    pub fn hda_dsp_cl_boot_firmware(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn hda_dsp_cl_boot_firmware_iccmax(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn hda_cl_copy_fw(sdev: *mut snd_sof_dev, hext_stream: *mut hdac_ext_stream) -> c_int;
}
extern "C" {
    pub fn hda_cl_trigger(dev: *mut device, hext_stream: *mut hdac_ext_stream, cmd: c_int) -> c_int;
}
extern "C" {
    pub fn cl_dsp_init(sdev: *mut snd_sof_dev, stream_tag: c_int, imr_boot: bool) -> c_int;
}
pub const HDA_CL_STREAM_FORMAT: c_uint = 0x40;
// pre and post fw run ops
extern "C" {
    pub fn hda_dsp_pre_fw_run(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn hda_dsp_post_fw_run(sdev: *mut snd_sof_dev) -> c_int;
}
// parse platform specific ext manifest ops
//
// HDA Controller Operations.
//
extern "C" {
    pub fn hda_dsp_ctrl_get_caps(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn hda_dsp_ctrl_ppcap_enable(sdev: *mut snd_sof_dev, enable: bool);
}
extern "C" {
    pub fn hda_dsp_ctrl_ppcap_int_enable(sdev: *mut snd_sof_dev, enable: bool);
}
extern "C" {
    pub fn hda_dsp_ctrl_link_reset(sdev: *mut snd_sof_dev, reset: bool) -> c_int;
}
extern "C" {
    pub fn hda_dsp_ctrl_misc_clock_gating(sdev: *mut snd_sof_dev, enable: bool);
}
extern "C" {
    pub fn hda_dsp_ctrl_clock_power_gating(sdev: *mut snd_sof_dev, enable: bool) -> c_int;
}
extern "C" {
    pub fn hda_dsp_ctrl_init_chip(sdev: *mut snd_sof_dev, detect_codec: bool) -> c_int;
}
extern "C" {
    pub fn hda_dsp_ctrl_stop_chip(sdev: *mut snd_sof_dev);
}
//
// HDA bus operations.
//
extern "C" {
    pub fn sof_hda_bus_init(sdev: *mut snd_sof_dev, dev: *mut device);
}
extern "C" {
    pub fn sof_hda_bus_exit(sdev: *mut snd_sof_dev);
}

//
// HDA Codec operations.
//
extern "C" {
    pub fn hda_codec_probe_bus(sdev: *mut snd_sof_dev);
}
extern "C" {
    pub fn hda_codec_jack_wake_enable(sdev: *mut snd_sof_dev, enable: bool);
}
extern "C" {
    pub fn hda_codec_jack_check(sdev: *mut snd_sof_dev);
}
extern "C" {
    pub fn hda_codec_check_for_state_change(sdev: *mut snd_sof_dev);
}
extern "C" {
    pub fn hda_codec_init_cmd_io(sdev: *mut snd_sof_dev);
}
extern "C" {
    pub fn hda_codec_resume_cmd_io(sdev: *mut snd_sof_dev);
}
extern "C" {
    pub fn hda_codec_stop_cmd_io(sdev: *mut snd_sof_dev);
}
extern "C" {
    pub fn hda_codec_suspend_cmd_io(sdev: *mut snd_sof_dev);
}
extern "C" {
    pub fn hda_codec_detect_mask(sdev: *mut snd_sof_dev);
}
extern "C" {
    pub fn hda_codec_rirb_status_clear(sdev: *mut snd_sof_dev);
}
extern "C" {
    pub fn hda_codec_check_rirb_status(sdev: *mut snd_sof_dev) -> bool;
}
extern "C" {
    pub fn hda_codec_set_codec_wakeup(sdev: *mut snd_sof_dev, status: bool);
}
extern "C" {
    pub fn hda_codec_device_remove(sdev: *mut snd_sof_dev);
}

extern "C" {
    pub fn hda_codec_i915_display_power(sdev: *mut snd_sof_dev, enable: bool);
}
extern "C" {
    pub fn hda_codec_i915_init(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn hda_codec_i915_exit(sdev: *mut snd_sof_dev) -> c_int;
}

//
// Trace Control.
//
extern "C" {
    pub fn hda_dsp_trace_release(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn hda_dsp_trace_trigger(sdev: *mut snd_sof_dev, cmd: c_int) -> c_int;
}
//
// SoundWire support
//

extern "C" {
    pub fn hda_sdw_check_lcount_common(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn hda_sdw_check_lcount_ext(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn hda_sdw_check_lcount(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn hda_sdw_startup(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn hda_common_enable_sdw_irq(sdev: *mut snd_sof_dev, enable: bool);
}
extern "C" {
    pub fn hda_sdw_int_enable(sdev: *mut snd_sof_dev, enable: bool);
}
extern "C" {
    pub fn hda_sdw_check_wakeen_irq_common(sdev: *mut snd_sof_dev) -> bool;
}
extern "C" {
    pub fn hda_sdw_process_wakeen_common(sdev: *mut snd_sof_dev);
}
extern "C" {
    pub fn hda_sdw_process_wakeen(sdev: *mut snd_sof_dev);
}
extern "C" {
    pub fn hda_common_check_sdw_irq(sdev: *mut snd_sof_dev) -> bool;
}

// common dai driver
extern "C" {
    pub fn hda_dsp_dais_suspend(sdev: *mut snd_sof_dev) -> c_int;
}
//
// Platform Specific HW abstraction Ops.
//
extern "C" {
    pub fn sof_skl_ops_init(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn sof_apl_ops_init(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn sof_cnl_ops_init(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn sof_tgl_ops_init(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn sof_icl_ops_init(sdev: *mut snd_sof_dev) -> c_int;
}
// Probes support

extern "C" {
    pub fn hda_probes_register(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn hda_probes_unregister(sdev: *mut snd_sof_dev);
}

// SOF client registration for HDA platforms
extern "C" {
    pub fn hda_register_clients(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn hda_unregister_clients(sdev: *mut snd_sof_dev);
}
// machine driver select
// PCI driver selection and probe
extern "C" {
    pub fn hda_pci_intel_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int;
}

extern "C" {
    pub fn hda_set_dai_drv_ops(sdev: *mut snd_sof_dev, ops: *mut snd_sof_dsp_ops);
}
extern "C" {
    pub fn hda_ops_free(sdev: *mut snd_sof_dev);
}
// SKL/KBL
extern "C" {
    pub fn hda_dsp_cl_boot_firmware_skl(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn hda_dsp_core_stall_reset(sdev: *mut snd_sof_dev, core_mask: c_uint) -> c_int;
}
// IPC4
extern "C" {
    pub fn cnl_ipc4_irq_thread(irq: c_int, context: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn cnl_ipc4_send_msg(sdev: *mut snd_sof_dev, msg: *mut snd_sof_ipc_msg) -> c_int;
}
extern "C" {
    pub fn hda_dsp_ipc4_irq_thread(irq: c_int, context: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn hda_ipc4_tx_is_busy(sdev: *mut snd_sof_dev) -> bool;
}
extern "C" {
    pub fn hda_dsp_ipc4_send_msg(sdev: *mut snd_sof_dev, msg: *mut snd_sof_ipc_msg) -> c_int;
}
extern "C" {
    pub fn hda_ipc4_dump(sdev: *mut snd_sof_dev);
}
//
// struct hda_dai_widget_dma_ops - DAI DMA ops optional by default unless specified otherwise
// @get_hext_stream: Mandatory function pointer to get the saved pointer to struct hdac_ext_stream
// @assign_hext_stream: Function pointer to assign a hdac_ext_stream
// @release_hext_stream: Function pointer to release the hdac_ext_stream
// @setup_hext_stream: Function pointer for hdac_ext_stream setup
// @reset_hext_stream: Function pointer for hdac_ext_stream reset
// @pre_trigger: Function pointer for DAI DMA pre-trigger actions
// @trigger: Function pointer for DAI DMA trigger actions
// @post_trigger: Function pointer for DAI DMA post-trigger actions
// @codec_dai_set_stream: Function pointer to set codec-side stream information
// @calc_stream_format: Function pointer to determine stream format from hw_params and
// for HDaudio codec DAI from the .sig bits
// @get_hlink: Mandatory function pointer to retrieve hlink, mainly to program LOSIDV
// for legacy HDaudio links or program HDaudio Extended Link registers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hda_dai_widget_dma_ops {
    pub substream): *mut snd_pcm_substream,
    pub hlink): *mut hdac_ext_link,
    pub substream): *mut snd_pcm_substream,
    pub format_val): c_uint,
    pub hext_sream): *mut *mut *mut void (reset_hext_stream)(struct snd_sof_dev sdev, struct hdac_ext_stream,
    pub cmd): *mut *mut snd_pcm_substream substream, int,
    pub cmd): *mut *mut snd_pcm_substream substream, int,
    pub cmd): *mut *mut snd_pcm_substream substream, int,
    pub hstream): *mut hdac_stream,
    pub params): *mut snd_pcm_hw_params,
    pub substream): *mut snd_pcm_substream,
}

extern "C" {
    pub fn snd_soc_component_get_drvdata(_arg: component) -> return;
}
