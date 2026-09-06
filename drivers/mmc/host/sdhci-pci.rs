//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mmc/host/sdhci-pci.h
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


// SPDX-License-Identifier: GPL-2.0
//
// PCI device IDs, sub IDs
//
pub const PCI_DEVICE_ID_O2_SDS0: c_uint = 0x8420;
pub const PCI_DEVICE_ID_O2_SDS1: c_uint = 0x8421;
pub const PCI_DEVICE_ID_O2_FUJIN2: c_uint = 0x8520;
pub const PCI_DEVICE_ID_O2_SEABIRD0: c_uint = 0x8620;
pub const PCI_DEVICE_ID_O2_SEABIRD1: c_uint = 0x8621;
pub const PCI_DEVICE_ID_O2_GG8_9860: c_uint = 0x9860;
pub const PCI_DEVICE_ID_O2_GG8_9861: c_uint = 0x9861;
pub const PCI_DEVICE_ID_O2_GG8_9862: c_uint = 0x9862;
pub const PCI_DEVICE_ID_O2_GG8_9863: c_uint = 0x9863;
pub const PCI_DEVICE_ID_INTEL_PCH_SDIO0: c_uint = 0x8809;
pub const PCI_DEVICE_ID_INTEL_PCH_SDIO1: c_uint = 0x880a;
pub const PCI_DEVICE_ID_INTEL_BYT_EMMC: c_uint = 0x0f14;
pub const PCI_DEVICE_ID_INTEL_BYT_SDIO: c_uint = 0x0f15;
pub const PCI_DEVICE_ID_INTEL_BYT_SD: c_uint = 0x0f16;
pub const PCI_DEVICE_ID_INTEL_BYT_EMMC2: c_uint = 0x0f50;
pub const PCI_DEVICE_ID_INTEL_BSW_EMMC: c_uint = 0x2294;
pub const PCI_DEVICE_ID_INTEL_BSW_SDIO: c_uint = 0x2295;
pub const PCI_DEVICE_ID_INTEL_BSW_SD: c_uint = 0x2296;
pub const PCI_DEVICE_ID_INTEL_MRFLD_MMC: c_uint = 0x1190;
pub const PCI_DEVICE_ID_INTEL_CLV_SDIO0: c_uint = 0x08f9;
pub const PCI_DEVICE_ID_INTEL_CLV_SDIO1: c_uint = 0x08fa;
pub const PCI_DEVICE_ID_INTEL_CLV_SDIO2: c_uint = 0x08fb;
pub const PCI_DEVICE_ID_INTEL_CLV_EMMC0: c_uint = 0x08e5;
pub const PCI_DEVICE_ID_INTEL_CLV_EMMC1: c_uint = 0x08e6;
pub const PCI_DEVICE_ID_INTEL_QRK_SD: c_uint = 0x08A7;
pub const PCI_DEVICE_ID_INTEL_SPT_EMMC: c_uint = 0x9d2b;
pub const PCI_DEVICE_ID_INTEL_SPT_SDIO: c_uint = 0x9d2c;
pub const PCI_DEVICE_ID_INTEL_SPT_SD: c_uint = 0x9d2d;
pub const PCI_DEVICE_ID_INTEL_DNV_EMMC: c_uint = 0x19db;
pub const PCI_DEVICE_ID_INTEL_CDF_EMMC: c_uint = 0x18db;
pub const PCI_DEVICE_ID_INTEL_BXT_SD: c_uint = 0x0aca;
pub const PCI_DEVICE_ID_INTEL_BXT_EMMC: c_uint = 0x0acc;
pub const PCI_DEVICE_ID_INTEL_BXT_SDIO: c_uint = 0x0ad0;
pub const PCI_DEVICE_ID_INTEL_BXTM_SD: c_uint = 0x1aca;
pub const PCI_DEVICE_ID_INTEL_BXTM_EMMC: c_uint = 0x1acc;
pub const PCI_DEVICE_ID_INTEL_BXTM_SDIO: c_uint = 0x1ad0;
pub const PCI_DEVICE_ID_INTEL_APL_SD: c_uint = 0x5aca;
pub const PCI_DEVICE_ID_INTEL_APL_EMMC: c_uint = 0x5acc;
pub const PCI_DEVICE_ID_INTEL_APL_SDIO: c_uint = 0x5ad0;
pub const PCI_DEVICE_ID_INTEL_GLK_SD: c_uint = 0x31ca;
pub const PCI_DEVICE_ID_INTEL_GLK_EMMC: c_uint = 0x31cc;
pub const PCI_DEVICE_ID_INTEL_GLK_SDIO: c_uint = 0x31d0;
pub const PCI_DEVICE_ID_INTEL_CNP_EMMC: c_uint = 0x9dc4;
pub const PCI_DEVICE_ID_INTEL_CNP_SD: c_uint = 0x9df5;
pub const PCI_DEVICE_ID_INTEL_CNPH_SD: c_uint = 0xa375;
pub const PCI_DEVICE_ID_INTEL_ICP_EMMC: c_uint = 0x34c4;
pub const PCI_DEVICE_ID_INTEL_ICP_SD: c_uint = 0x34f8;
pub const PCI_DEVICE_ID_INTEL_EHL_EMMC: c_uint = 0x4b47;
pub const PCI_DEVICE_ID_INTEL_EHL_SD: c_uint = 0x4b48;
pub const PCI_DEVICE_ID_INTEL_CML_EMMC: c_uint = 0x02c4;
pub const PCI_DEVICE_ID_INTEL_CML_SD: c_uint = 0x02f5;
pub const PCI_DEVICE_ID_INTEL_CMLH_SD: c_uint = 0x06f5;
pub const PCI_DEVICE_ID_INTEL_JSL_EMMC: c_uint = 0x4dc4;
pub const PCI_DEVICE_ID_INTEL_JSL_SD: c_uint = 0x4df8;
pub const PCI_DEVICE_ID_INTEL_LKF_EMMC: c_uint = 0x98c4;
pub const PCI_DEVICE_ID_INTEL_LKF_SD: c_uint = 0x98f8;
pub const PCI_DEVICE_ID_INTEL_ADL_EMMC: c_uint = 0x54c4;
pub const PCI_DEVICE_ID_SYSKONNECT_8000: c_uint = 0x8000;
pub const PCI_DEVICE_ID_VIA_95D0: c_uint = 0x95d0;
pub const PCI_DEVICE_ID_REALTEK_5250: c_uint = 0x5250;
pub const PCI_SUBDEVICE_ID_NI_7884: c_uint = 0x7884;
pub const PCI_SUBDEVICE_ID_NI_78E3: c_uint = 0x78e3;
pub const PCI_VENDOR_ID_ARASAN: c_uint = 0x16e6;
pub const PCI_DEVICE_ID_ARASAN_PHY_EMMC: c_uint = 0x0670;
pub const PCI_DEVICE_ID_SYNOPSYS_DWC_MSHC: c_uint = 0xc202;
pub const PCI_DEVICE_ID_GLI_9755: c_uint = 0x9755;
pub const PCI_DEVICE_ID_GLI_9750: c_uint = 0x9750;
pub const PCI_DEVICE_ID_GLI_9763E: c_uint = 0xe763;
pub const PCI_DEVICE_ID_GLI_9767: c_uint = 0x9767;
//
// PCI device class and mask
//

pub const PCI_CLASS_MASK: c_uint = 0xFFFF00;
//
// Macros for PCI device-description
//

//
// PCI registers
//
pub const PCI_SDHCI_IFPIO: c_uint = 0x00;
pub const PCI_SDHCI_IFDMA: c_uint = 0x01;
pub const PCI_SDHCI_IFVENDOR: c_uint = 0x02;
pub const PCI_SLOT_INFO: c_uint = 0x40	/* 8 bits */;

pub const PCI_SLOT_INFO_FIRST_BAR_MASK: c_uint = 0x07;
pub const MAX_SLOTS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdhci_pci_fixes {
    pub quirks: c_uint,
    pub quirks2: c_uint,
    pub allow_runtime_pm: bool,
    pub own_cd_for_runtime_pm: bool,
    pub ): *mut *mut int (probe) (struct sdhci_pci_chip,
    pub ): *mut *mut int (probe_slot) (struct sdhci_pci_slot,
    pub ): *mut *mut int (add_host) (struct sdhci_pci_slot,
    pub int): *mut *mut *mut void (remove_slot) (struct sdhci_pci_slot ,,
    pub int): *mut *mut *mut void (remove_host) (struct sdhci_pci_slot ,,

    pub ): *mut *mut int (suspend) (struct sdhci_pci_chip,
    pub ): *mut *mut int (resume) (struct sdhci_pci_chip,

    pub ): *mut *mut int (runtime_suspend) (struct sdhci_pci_chip,
    pub ): *mut *mut int (runtime_resume) (struct sdhci_pci_chip,

    pub ops: *const sdhci_ops,
    pub cd_gpio_override: *const dmi_system_id,
    pub priv_size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdhci_pci_slot {
    pub chip: *mut sdhci_pci_chip,
    pub host: *mut sdhci_host,
    pub cd_idx: c_int,
    pub cd_override_level: bool,
    pub host): *mut *mut void (hw_reset)(struct sdhci_host,
    pub ____cacheline_aligned: unsigned long private[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdhci_pci_chip {
    pub pdev: *mut pci_dev,
    pub quirks: c_uint,
    pub quirks2: c_uint,
    pub allow_runtime_pm: bool,
    pub pm_retune: bool,
    pub rpm_retune: bool,
    pub fixes: *const sdhci_pci_fixes,
    pub /: *mut *mut int num_slots; / Slots on controller,
    pub /: *mut *mut *mut sdhci_pci_slot slots[MAX_SLOTS]; / Pointers to host slots,
}

extern "C" {
    pub fn sdhci_pci_uhs2_add_host(slot: *mut sdhci_pci_slot) -> c_int;
}
extern "C" {
    pub fn sdhci_pci_uhs2_remove_host(slot: *mut sdhci_pci_slot, dead: c_int);
}

extern "C" {
    pub fn sdhci_pci_resume_host(chip: *mut sdhci_pci_chip) -> c_int;
}

extern "C" {
    pub fn sdhci_pci_enable_dma(host: *mut sdhci_host) -> c_int;
}
