//! Automatically rewritten from C to Rust
//! Source: sound/soc/sof/intel/skl.c
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
// Copyright(c) 2018-2022 Intel Corporation
//
// Hardware interface for audio DSP on Skylake and Kabylake.
//

pub const SRAM_MEMORY_WINDOW_BASE: c_uint = 0x8000;
    static const __maybe_unused struct snd_sof_debugfs_map skl_dsp_debugfs[] = {
    {"hda", HDA_DSP_HDA_BAR, 0, 0x4000},
    {"pp", HDA_DSP_PP_BAR,  0, 0x1000},
    {"dsp", HDA_DSP_BAR,  0, 0x10000},
    };
#[no_mangle]
unsafe extern "C" fn skl_dsp_ipc_get_window_offset(sdev: *mut snd_sof_dev, id: u32) -> c_int {
    static int skl_dsp_ipc_get_window_offset(struct snd_sof_dev *sdev, u32 id)
    {
    return SRAM_MEMORY_WINDOW_BASE + (0x2000 * id);
    }
#[no_mangle]
unsafe extern "C" fn skl_dsp_ipc_get_mailbox_offset(sdev: *mut snd_sof_dev) -> c_int {
    static int skl_dsp_ipc_get_mailbox_offset(struct snd_sof_dev *sdev)
    {
    return SRAM_MEMORY_WINDOW_BASE + 0x1000;
    }
// skylake ops
    struct snd_sof_dsp_ops sof_skl_ops;
    EXPORT_SYMBOL_NS(sof_skl_ops, "SND_SOC_SOF_INTEL_HDA_COMMON");
#[no_mangle]
pub unsafe extern "C" fn sof_skl_ops_init(sdev: *mut snd_sof_dev) -> c_int {
    int sof_skl_ops_init(struct snd_sof_dev *sdev)
    {
    struct sof_ipc4_fw_data *ipc4_data;
// common defaults
    memcpy(&sof_skl_ops, &sof_hda_common_ops, sizeof(struct snd_sof_dsp_ops));
// probe/remove/shutdown
    sof_skl_ops.shutdown	= hda_dsp_shutdown;
    sdev.private = kzalloc_obj(*ipc4_data);
    if (!sdev.private)
    return -ENOMEM;
    ipc4_data = sdev.private;
    ipc4_data.manifest_fw_hdr_offset = SOF_MAN4_FW_HDR_OFFSET_CAVS_1_5;
    ipc4_data.mtrace_type = SOF_IPC4_MTRACE_INTEL_CAVS_1_5;
    sof_skl_ops.get_window_offset = skl_dsp_ipc_get_window_offset;
    sof_skl_ops.get_mailbox_offset = skl_dsp_ipc_get_mailbox_offset;
// doorbell
    sof_skl_ops.irq_thread	= hda_dsp_ipc4_irq_thread;
// ipc
    sof_skl_ops.send_msg	= hda_dsp_ipc4_send_msg;
// set DAI driver ops
    hda_set_dai_drv_ops(sdev, &sof_skl_ops);
// debug
    sof_skl_ops.debug_map	= skl_dsp_debugfs;
    sof_skl_ops.debug_map_count	= ARRAY_SIZE(skl_dsp_debugfs);
    sof_skl_ops.ipc_dump	= hda_ipc4_dump;
// firmware run
    sof_skl_ops.run = hda_dsp_cl_boot_firmware_skl;
// pre/post fw run
    sof_skl_ops.post_fw_run = hda_dsp_post_fw_run;
    return 0;
    };
    EXPORT_SYMBOL_NS(sof_skl_ops_init, "SND_SOC_SOF_INTEL_HDA_COMMON");
    const struct sof_intel_dsp_desc skl_chip_info = {
    .cores_num = 2,
    .init_core_mask = 1,
    .host_managed_cores_mask = GENMASK(1, 0),
    .ipc_req = HDA_DSP_REG_HIPCI,
    .ipc_req_mask = HDA_DSP_REG_HIPCI_BUSY,
    .ipc_ack = HDA_DSP_REG_HIPCIE,
    .ipc_ack_mask = HDA_DSP_REG_HIPCIE_DONE,
    .ipc_ctl = HDA_DSP_REG_HIPCCTL,
    .rom_status_reg = HDA_DSP_SRAM_REG_ROM_STATUS_SKL,
    .rom_init_timeout	= 300,
    .check_ipc_irq	= hda_dsp_check_ipc_irq,
    .power_down_dsp = hda_power_down_dsp,
    .disable_interrupts = hda_dsp_disable_interrupts,
    .hw_ip_version = SOF_INTEL_CAVS_1_5,
    .platform = "skl",
    };
    EXPORT_SYMBOL_NS(skl_chip_info, "SND_SOC_SOF_INTEL_HDA_COMMON");
