//! Automatically rewritten from C to Rust
//! Source: sound/soc/sof/intel/icl.c
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
// Copyright(c) 2020 Intel Corporation
//
// Author: Fred Oh <fred.oh@linux.intel.com>
//
// Hardware interface for audio DSP on IceLake.
//

pub const ICL_DSP_HPRO_CORE_ID: c_int = 3;
    static const struct snd_sof_debugfs_map icl_dsp_debugfs[] = {
    {"hda", HDA_DSP_HDA_BAR, 0, 0x4000, SOF_DEBUGFS_ACCESS_ALWAYS},
    {"pp", HDA_DSP_PP_BAR,  0, 0x1000, SOF_DEBUGFS_ACCESS_ALWAYS},
    {"dsp", HDA_DSP_BAR,  0, 0x10000, SOF_DEBUGFS_ACCESS_ALWAYS},
    };
#[no_mangle]
unsafe extern "C" fn icl_dsp_core_stall(sdev: *mut snd_sof_dev, core_mask: c_uint) -> c_int {
    static int icl_dsp_core_stall(struct snd_sof_dev *sdev, unsigned int core_mask)
    {
    struct sof_intel_hda_dev *hda = sdev.pdata.hw_pdata;
    const struct sof_intel_dsp_desc *chip = hda.desc;
// make sure core_mask in host managed cores
    core_mask &= chip.host_managed_cores_mask;
    if (!core_mask) {
    dev_err(sdev.dev, "error: core_mask is not in host managed cores\n");
    return -EINVAL;
    }
// stall core
    snd_sof_dsp_update_bits_unlocked(sdev, HDA_DSP_BAR, HDA_DSP_REG_ADSPCS,
    HDA_DSP_ADSPCS_CSTALL_MASK(core_mask),
    HDA_DSP_ADSPCS_CSTALL_MASK(core_mask));
    return 0;
    }
//
// post fw run operation for ICL.
// Core 3 will be powered up and in stall when HPRO is enabled
//
#[no_mangle]
unsafe extern "C" fn icl_dsp_post_fw_run(sdev: *mut snd_sof_dev) -> c_int {
    static int icl_dsp_post_fw_run(struct snd_sof_dev *sdev)
    {
    struct sof_intel_hda_dev *hda = sdev.pdata.hw_pdata;
    int ret;
    if (sdev.first_boot) {
    struct sof_intel_hda_dev *hdev = sdev.pdata.hw_pdata;
    ret = hda_sdw_startup(sdev);
    if (ret < 0) {
    dev_err(sdev.dev, "error: could not startup SoundWire links\n");
    return ret;
    }
// Check if IMR boot is usable
    if (!sof_debug_check_flag(SOF_DBG_IGNORE_D3_PERSISTENT) &&
    sdev.fw_ready.flags & SOF_IPC_INFO_D3_PERSISTENT)
    hdev.imrboot_supported = true;
    }
    hda_sdw_int_enable(sdev, true);
//
// The recommended HW programming sequence for ICL is to
// power up core 3 and keep it in stall if HPRO is enabled.
//
    if (!hda.clk_config_lpro) {
    ret = hda_dsp_enable_core(sdev, BIT(ICL_DSP_HPRO_CORE_ID));
    if (ret < 0) {
    dev_err(sdev.dev, "error: dsp core power up failed on core %d\n",
    ICL_DSP_HPRO_CORE_ID);
    return ret;
    }
    sdev.enabled_cores_mask |= BIT(ICL_DSP_HPRO_CORE_ID);
    sdev.dsp_core_ref_count[ICL_DSP_HPRO_CORE_ID]++;
    snd_sof_dsp_stall(sdev, BIT(ICL_DSP_HPRO_CORE_ID));
    }
// re-enable clock gating and power gating
    return hda_dsp_ctrl_clock_power_gating(sdev, true);
    }
// Icelake ops
    struct snd_sof_dsp_ops sof_icl_ops;
#[no_mangle]
pub unsafe extern "C" fn sof_icl_ops_init(sdev: *mut snd_sof_dev) -> c_int {
    int sof_icl_ops_init(struct snd_sof_dev *sdev)
    {
// common defaults
    memcpy(&sof_icl_ops, &sof_hda_common_ops, sizeof(struct snd_sof_dsp_ops));
// probe/remove/shutdown
    sof_icl_ops.shutdown	= hda_dsp_shutdown;
    if (sdev.pdata.ipc_type == SOF_IPC_TYPE_3) {
// doorbell
    sof_icl_ops.irq_thread	= cnl_ipc_irq_thread;
// ipc
    sof_icl_ops.send_msg	= cnl_ipc_send_msg;
// debug
    sof_icl_ops.ipc_dump	= cnl_ipc_dump;
    sof_icl_ops.set_power_state = hda_dsp_set_power_state_ipc3;
    }
    if (sdev.pdata.ipc_type == SOF_IPC_TYPE_4) {
    struct sof_ipc4_fw_data *ipc4_data;
    sdev.private = kzalloc_obj(*ipc4_data);
    if (!sdev.private)
    return -ENOMEM;
    ipc4_data = sdev.private;
    ipc4_data.manifest_fw_hdr_offset = SOF_MAN4_FW_HDR_OFFSET;
    ipc4_data.mtrace_type = SOF_IPC4_MTRACE_INTEL_CAVS_2;
// External library loading support
    ipc4_data.load_library = hda_dsp_ipc4_load_library;
// doorbell
    sof_icl_ops.irq_thread	= cnl_ipc4_irq_thread;
// ipc
    sof_icl_ops.send_msg	= cnl_ipc4_send_msg;
// debug
    sof_icl_ops.ipc_dump	= cnl_ipc4_dump;
    sof_icl_ops.set_power_state = hda_dsp_set_power_state_ipc4;
    }
// debug
    sof_icl_ops.debug_map	= icl_dsp_debugfs;
    sof_icl_ops.debug_map_count	= ARRAY_SIZE(icl_dsp_debugfs);
// pre/post fw run
    sof_icl_ops.post_fw_run = icl_dsp_post_fw_run;
// firmware run
    sof_icl_ops.run = hda_dsp_cl_boot_firmware_iccmax;
    sof_icl_ops.stall = icl_dsp_core_stall;
// dsp core get/put
    sof_icl_ops.core_get = hda_dsp_core_get;
// set DAI driver ops
    hda_set_dai_drv_ops(sdev, &sof_icl_ops);
    return 0;
    };
    const struct sof_intel_dsp_desc icl_chip_info = {
// Icelake
    .cores_num = 4,
    .init_core_mask = 1,
    .host_managed_cores_mask = GENMASK(3, 0),
    .ipc_req = CNL_DSP_REG_HIPCIDR,
    .ipc_req_mask = CNL_DSP_REG_HIPCIDR_BUSY,
    .ipc_ack = CNL_DSP_REG_HIPCIDA,
    .ipc_ack_mask = CNL_DSP_REG_HIPCIDA_DONE,
    .ipc_ctl = CNL_DSP_REG_HIPCCTL,
    .rom_status_reg = HDA_DSP_SRAM_REG_ROM_STATUS,
    .rom_init_timeout	= 300,
    .ssp_count = ICL_SSP_COUNT,
    .ssp_base_offset = CNL_SSP_BASE_OFFSET,
    .sdw_shim_base = SDW_SHIM_BASE,
    .sdw_alh_base = SDW_ALH_BASE,
    .d0i3_offset = SOF_HDA_VS_D0I3C,
    .read_sdw_lcount =  hda_sdw_check_lcount_common,
    .enable_sdw_irq	= hda_common_enable_sdw_irq,
    .check_sdw_irq	= hda_common_check_sdw_irq,
    .check_sdw_wakeen_irq = hda_sdw_check_wakeen_irq_common,
    .sdw_process_wakeen = hda_sdw_process_wakeen_common,
    .check_ipc_irq	= hda_dsp_check_ipc_irq,
    .cl_init = cl_dsp_init,
    .power_down_dsp = hda_power_down_dsp,
    .disable_interrupts = hda_dsp_disable_interrupts,
    .hw_ip_version = SOF_INTEL_CAVS_2_0,
    .platform = "icl",
    };
