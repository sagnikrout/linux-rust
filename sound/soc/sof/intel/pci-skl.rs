//! Automatically rewritten from C to Rust
//! Source: sound/soc/sof/intel/pci-skl.c
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

// platform specific devices

    static struct sof_dev_desc skl_desc = {
    .machines		= snd_soc_acpi_intel_skl_machines,
    .resindex_lpe_base	= 0,
    .resindex_pcicfg_base	= -1,
    .resindex_imr_base	= -1,
    .chip_info = &skl_chip_info,
    .irqindex_host_ipc	= -1,
    .ipc_supported_mask	= BIT(SOF_IPC_TYPE_4),
    .ipc_default		= SOF_IPC_TYPE_4,
    .dspless_mode_supported	= true,		/* Only supported for HDaudio */
    .default_fw_path = {
    [SOF_IPC_TYPE_4] = "intel/avs/skl",
    },
    .default_tplg_path = {
    [SOF_IPC_TYPE_4] = "intel/avs-tplg",
    },
    .default_fw_filename = {
    [SOF_IPC_TYPE_4] = "dsp_basefw.bin",
    },
    .nocodec_tplg_filename = "sof-skl-nocodec.tplg",
    .ops = &sof_skl_ops,
    .ops_init = sof_skl_ops_init,
    .ops_free = hda_ops_free,
    };
    static struct sof_dev_desc kbl_desc = {
    .machines		= snd_soc_acpi_intel_kbl_machines,
    .resindex_lpe_base	= 0,
    .resindex_pcicfg_base	= -1,
    .resindex_imr_base	= -1,
    .chip_info = &skl_chip_info,
    .irqindex_host_ipc	= -1,
    .ipc_supported_mask	= BIT(SOF_IPC_TYPE_4),
    .ipc_default		= SOF_IPC_TYPE_4,
    .dspless_mode_supported	= true,		/* Only supported for HDaudio */
    .default_fw_path = {
    [SOF_IPC_TYPE_4] = "intel/avs/kbl",
    },
    .default_tplg_path = {
    [SOF_IPC_TYPE_4] = "intel/avs-tplg",
    },
    .default_fw_filename = {
    [SOF_IPC_TYPE_4] = "dsp_basefw.bin",
    },
    .nocodec_tplg_filename = "sof-kbl-nocodec.tplg",
    .ops = &sof_skl_ops,
    .ops_init = sof_skl_ops_init,
    .ops_free = hda_ops_free,
    };
// PCI IDs
    static const struct pci_device_id sof_pci_ids[] = {
    { PCI_DEVICE_DATA(INTEL, HDA_SKL_LP, &skl_desc) },
    { PCI_DEVICE_DATA(INTEL, HDA_KBL_LP, &kbl_desc) },
    { 0, }
    };
    MODULE_DEVICE_TABLE(pci, sof_pci_ids);
// pci_driver definition
    static struct pci_driver snd_sof_pci_intel_skl_driver = {
    .name = "sof-audio-pci-intel-skl",
    .id_table = sof_pci_ids,
    .probe = hda_pci_intel_probe,
    .remove = sof_pci_remove,
    .shutdown = sof_pci_shutdown,
    .driver = {
    .pm = pm_ptr(&sof_pci_pm),
    },
    };
    module_pci_driver(snd_sof_pci_intel_skl_driver);
    MODULE_LICENSE("Dual BSD/GPL");
    MODULE_DESCRIPTION("SOF support for SkyLake platforms");
    MODULE_IMPORT_NS("SND_SOC_SOF_INTEL_HDA_GENERIC");
    MODULE_IMPORT_NS("SND_SOC_SOF_INTEL_HDA_COMMON");
    MODULE_IMPORT_NS("SND_SOC_SOF_PCI_DEV");
