//! Automatically rewritten from C to Rust
//! Source: sound/soc/sof/amd/pci-rn.c
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
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2021 Advanced Micro Devices, Inc. All rights reserved.
//
// Authors: Ajit Kumar Pandey <AjitKumar.Pandey@amd.com>
//
// PCI interface for Renoir ACP device
//

pub const ACP3x_REG_START: c_uint = 0x1240000;
pub const ACP3x_REG_END: c_uint = 0x125C000;
pub const ACP3X_FUTURE_REG_ACLK_0: c_uint = 0x1860;
    static const struct sof_amd_acp_desc renoir_chip_info = {
    .pgfsm_base	= ACP3X_PGFSM_BASE,
    .ext_intr_stat	= ACP3X_EXT_INTR_STAT,
    .dsp_intr_base	= ACP3X_DSP_SW_INTR_BASE,
    .acp_error_stat = ACP3X_ERROR_STATUS,
    .acp_sw0_i2s_err_reason = ACP3X_SW_I2S_ERROR_REASON,
    .sram_pte_offset = ACP3X_SRAM_PTE_OFFSET,
    .hw_semaphore_offset = ACP3X_AXI2DAGB_SEM_0,
    .acp_clkmux_sel	= ACP3X_CLKMUX_SEL,
    .probe_reg_offset = ACP3X_FUTURE_REG_ACLK_0,
    };
    static const struct sof_dev_desc renoir_desc = {
    .machines		= snd_soc_acpi_amd_sof_machines,
    .use_acpi_target_states	= true,
    .resindex_lpe_base	= 0,
    .resindex_pcicfg_base	= -1,
    .resindex_imr_base	= -1,
    .irqindex_host_ipc	= -1,
    .chip_info		= &renoir_chip_info,
    .ipc_supported_mask	= BIT(SOF_IPC_TYPE_3),
    .ipc_default		= SOF_IPC_TYPE_3,
    .default_fw_path = {
    [SOF_IPC_TYPE_3] = "amd/sof",
    },
    .default_tplg_path = {
    [SOF_IPC_TYPE_3] = "amd/sof-tplg",
    },
    .default_fw_filename	= {
    [SOF_IPC_TYPE_3] = "sof-rn.ri",
    },
    .nocodec_tplg_filename	= "sof-acp.tplg",
    .ops			= &sof_renoir_ops,
    .ops_init		= sof_renoir_ops_init,
    };
#[no_mangle]
unsafe extern "C" fn acp_pci_rn_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int {
    static int acp_pci_rn_probe(struct pci_dev *pci, const struct pci_device_id *pci_id)
    {
    unsigned int flag;
    if (pci.revision != ACP_RN_PCI_ID)
    return -ENODEV;
    flag = snd_amd_acp_find_config(pci);
    if (flag != FLAG_AMD_SOF && flag != FLAG_AMD_SOF_ONLY_DMIC)
    return -ENODEV;
    return sof_pci_probe(pci, pci_id);
    };
#[no_mangle]
unsafe extern "C" fn acp_pci_rn_remove(pci: *mut pci_dev) {
    static void acp_pci_rn_remove(struct pci_dev *pci)
    {
    return sof_pci_remove(pci);
    }
// PCI IDs
    static const struct pci_device_id rn_pci_ids[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_AMD, ACP_PCI_DEV_ID),
    .driver_data = (unsigned long)&renoir_desc},
    { 0, }
    };
    MODULE_DEVICE_TABLE(pci, rn_pci_ids);
// pci_driver definition
    static struct pci_driver snd_sof_pci_amd_rn_driver = {
    .name = KBUILD_MODNAME,
    .id_table = rn_pci_ids,
    .probe = acp_pci_rn_probe,
    .remove = acp_pci_rn_remove,
    .driver = {
    .pm = pm_ptr(&sof_pci_pm),
    },
    };
    module_pci_driver(snd_sof_pci_amd_rn_driver);
    MODULE_LICENSE("Dual BSD/GPL");
    MODULE_DESCRIPTION("RENOIR SOF Driver");
    MODULE_IMPORT_NS("SND_SOC_SOF_AMD_COMMON");
    MODULE_IMPORT_NS("SND_SOC_SOF_PCI_DEV");
