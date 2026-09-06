//! Automatically rewritten from C to Rust
//! Source: sound/soc/sof/intel/pci-mtl.c
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
// Author: Ranjani Sridharan <ranjani.sridharan@linux.intel.com>
//

// platform specific devices

// Meteorlake ops
    static struct snd_sof_dsp_ops sof_mtl_ops;
#[no_mangle]
unsafe extern "C" fn sof_mtl_ops_init(sdev: *mut snd_sof_dev) -> c_int {
    static int sof_mtl_ops_init(struct snd_sof_dev *sdev)
    {
    return sof_mtl_set_ops(sdev, &sof_mtl_ops);
    }
    static const struct sof_dev_desc mtl_desc = {
    .use_acpi_target_states	= true,
    .machines               = snd_soc_acpi_intel_mtl_machines,
    .alt_machines		= snd_soc_acpi_intel_mtl_sdw_machines,
    .resindex_lpe_base      = 0,
    .resindex_pcicfg_base   = -1,
    .resindex_imr_base      = -1,
    .irqindex_host_ipc      = -1,
    .chip_info = &mtl_chip_info,
    .ipc_supported_mask	= BIT(SOF_IPC_TYPE_4),
    .ipc_default		= SOF_IPC_TYPE_4,
    .dspless_mode_supported	= true,		/* Only supported for HDaudio */
    .default_fw_path = {
    [SOF_IPC_TYPE_4] = "intel/sof-ipc4/mtl",
    },
    .default_lib_path = {
    [SOF_IPC_TYPE_4] = "intel/sof-ipc4-lib/mtl",
    },
    .default_tplg_path = {
    [SOF_IPC_TYPE_4] = "intel/sof-ipc4-tplg",
    },
    .default_fw_filename = {
    [SOF_IPC_TYPE_4] = "sof-mtl.ri",
    },
    .nocodec_tplg_filename = "sof-mtl-nocodec.tplg",
    .ops = &sof_mtl_ops,
    .ops_init = sof_mtl_ops_init,
    .ops_free = hda_ops_free,
    };
    static const struct sof_dev_desc arl_desc = {
    .use_acpi_target_states = true,
    .machines               = snd_soc_acpi_intel_arl_machines,
    .alt_machines           = snd_soc_acpi_intel_arl_sdw_machines,
    .resindex_lpe_base      = 0,
    .resindex_pcicfg_base   = -1,
    .resindex_imr_base      = -1,
    .irqindex_host_ipc      = -1,
    .chip_info = &mtl_chip_info,
    .ipc_supported_mask     = BIT(SOF_IPC_TYPE_4),
    .ipc_default            = SOF_IPC_TYPE_4,
    .dspless_mode_supported = true,         /* Only supported for HDaudio */
    .default_fw_path = {
    [SOF_IPC_TYPE_4] = "intel/sof-ipc4/arl",
    },
    .default_lib_path = {
    [SOF_IPC_TYPE_4] = "intel/sof-ipc4-lib/arl",
    },
    .default_tplg_path = {
    [SOF_IPC_TYPE_4] = "intel/sof-ipc4-tplg",
    },
    .default_fw_filename = {
    [SOF_IPC_TYPE_4] = "sof-arl.ri",
    },
    .nocodec_tplg_filename = "sof-arl-nocodec.tplg",
    .ops = &sof_mtl_ops,
    .ops_init = sof_mtl_ops_init,
    .ops_free = hda_ops_free,
    };
    static const struct sof_dev_desc arl_s_desc = {
    .use_acpi_target_states = true,
    .machines               = snd_soc_acpi_intel_arl_machines,
    .alt_machines           = snd_soc_acpi_intel_arl_sdw_machines,
    .resindex_lpe_base      = 0,
    .resindex_pcicfg_base   = -1,
    .resindex_imr_base      = -1,
    .irqindex_host_ipc      = -1,
    .chip_info = &arl_s_chip_info,
    .ipc_supported_mask     = BIT(SOF_IPC_TYPE_4),
    .ipc_default            = SOF_IPC_TYPE_4,
    .dspless_mode_supported = true,         /* Only supported for HDaudio */
    .default_fw_path = {
    [SOF_IPC_TYPE_4] = "intel/sof-ipc4/arl-s",
    },
    .default_lib_path = {
    [SOF_IPC_TYPE_4] = "intel/sof-ipc4-lib/arl-s",
    },
    .default_tplg_path = {
    [SOF_IPC_TYPE_4] = "intel/sof-ipc4-tplg",
    },
    .default_fw_filename = {
    [SOF_IPC_TYPE_4] = "sof-arl-s.ri",
    },
    .nocodec_tplg_filename = "sof-arl-nocodec.tplg",
    .ops = &sof_mtl_ops,
    .ops_init = sof_mtl_ops_init,
    .ops_free = hda_ops_free,
    };
// PCI IDs
    static const struct pci_device_id sof_pci_ids[] = {
    { PCI_DEVICE_DATA(INTEL, HDA_MTL, &mtl_desc) },
    { PCI_DEVICE_DATA(INTEL, HDA_ARL_S, &arl_s_desc) },
    { PCI_DEVICE_DATA(INTEL, HDA_ARL, &arl_desc) },
    { 0, }
    };
    MODULE_DEVICE_TABLE(pci, sof_pci_ids);
// pci_driver definition
    static struct pci_driver snd_sof_pci_intel_mtl_driver = {
    .name = "sof-audio-pci-intel-mtl",
    .id_table = sof_pci_ids,
    .probe = hda_pci_intel_probe,
    .remove = sof_pci_remove,
    .shutdown = sof_pci_shutdown,
    .driver = {
    .pm = pm_ptr(&sof_pci_pm),
    },
    };
    module_pci_driver(snd_sof_pci_intel_mtl_driver);
    MODULE_LICENSE("Dual BSD/GPL");
    MODULE_DESCRIPTION("SOF support for MeteorLake platforms");
    MODULE_IMPORT_NS("SND_SOC_SOF_INTEL_HDA_GENERIC");
    MODULE_IMPORT_NS("SND_SOC_SOF_INTEL_HDA_COMMON");
    MODULE_IMPORT_NS("SND_SOC_SOF_PCI_DEV");
