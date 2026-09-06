//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/sysdev/fsl_pci.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// MPC85xx/86xx PCI Express structure define
//
// Copyright 2007,2011 Freescale Semiconductor, Inc
//

// FSL PCI controller BRR1 register
pub const PCI_FSL_BRR1: c_uint = 0xbf8;
pub const PCI_FSL_BRR1_VER: c_uint = 0xffff;
pub const PCIE_LTSSM: c_uint = 0x0404		/* PCIE Link Training and Status */;
pub const PCIE_LTSSM_L0: c_uint = 0x16		/* L0 state */;
pub const PCIE_FSL_CSR_CLASSCODE: c_uint = 0x474	/* FSL GPEX CSR */;
pub const PCIE_IP_REV_2_2: c_uint = 0x02080202 /* PCIE IP block version Rev2.2 */;
pub const PCIE_IP_REV_3_0: c_uint = 0x02080300 /* PCIE IP block version Rev3.0 */;
pub const PIWAR_EN: c_uint = 0x80000000	/* Enable */;
pub const PIWAR_PF: c_uint = 0x20000000	/* prefetch */;
pub const PIWAR_TGI_LOCAL: c_uint = 0x00f00000	/* target - local memory */;
pub const PIWAR_READ_SNOOP: c_uint = 0x00050000;
pub const PIWAR_WRITE_SNOOP: c_uint = 0x00005000;
pub const PIWAR_SZ_MASK: c_uint = 0x0000003f;
pub const PEX_PMCR_PTOMR: c_uint = 0x1;
pub const PEX_PMCR_EXL2S: c_uint = 0x2;
pub const PME_DISR_EN_PTOD: c_uint = 0x00008000;
pub const PME_DISR_EN_ENL23D: c_uint = 0x00002000;
pub const PME_DISR_EN_EXL23D: c_uint = 0x00001000;
// PCI/PCI Express outbound window reg
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_outbound_window_regs {
    pub /: *mut *mut __be32 potar; / 0x.0 - Outbound translation address register,
    pub /: *mut *mut __be32 potear; / 0x.4 - Outbound translation extended address register,
    pub /: *mut *mut __be32 powbar; / 0x.8 - Outbound window base address register,
    pub res1: [u8; 4],
    pub /: *mut *mut __be32 powar; / 0x.10 - Outbound window attributes register,
    pub res2: [u8; 12],
}

// PCI/PCI Express inbound window reg
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_inbound_window_regs {
    pub /: *mut *mut __be32 pitar; / 0x.0 - Inbound translation address register,
    pub res1: [u8; 4],
    pub /: *mut *mut __be32 piwbar; / 0x.8 - Inbound window base address register,
    pub /: *mut *mut __be32 piwbear; / 0x.c - Inbound window base extended address register,
    pub /: *mut *mut __be32 piwar; / 0x.10 - Inbound window attributes register,
    pub res2: [u8; 12],
}

// PCI/PCI Express IO block registers for 85xx/86xx
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccsr_pci {
    pub /: *mut *mut __be32 config_addr; / 0x.000 - PCI/PCIE Configuration Address Register,
    pub /: *mut *mut __be32 config_data; / 0x.004 - PCI/PCIE Configuration Data Register,
    pub /: *mut *mut __be32 int_ack; / 0x.008 - PCI Interrupt Acknowledge Register,
    pub /: *mut *mut __be32 pex_otb_cpl_tor; / 0x.00c - PCIE Outbound completion timeout register,
    pub /: *mut *mut __be32 pex_conf_tor; / 0x.010 - PCIE configuration timeout register,
    pub /: *mut *mut __be32 pex_config; / 0x.014 - PCIE CONFIG Register,
    pub /: *mut *mut __be32 pex_int_status; / 0x.018 - PCIE interrupt status,
    pub res2: [u8; 4],
    pub /: *mut *mut __be32 pex_pme_mes_dr; / 0x.020 - PCIE PME and message detect register,
    pub /: *mut *mut __be32 pex_pme_mes_disr; / 0x.024 - PCIE PME and message disable register,
    pub /: *mut *mut __be32 pex_pme_mes_ier; / 0x.028 - PCIE PME and message interrupt enable register,
    pub /: *mut *mut __be32 pex_pmcr; / 0x.02c - PCIE power management command register,
    pub res3: [u8; 3016],
    pub /: *mut *mut __be32 block_rev1; / 0x.bf8 - PCIE Block Revision register 1,
    pub /: *mut *mut __be32 block_rev2; / 0x.bfc - PCIE Block Revision register 2,
// PCI/PCI Express outbound window 0-4
// Window 0 is the default window and is the only window enabled upon reset.
// The default outbound register set is used when a transaction misses
// in all of the other outbound windows.
//
    pub pow: [pci_outbound_window_regs; 5],
    pub res14: [u8; 96],
    pub /: *mut *mut pci_inbound_window_regs pmit; / 0xd00 - 0xd9c Inbound MSI,
    pub res6: [u8; 96],
// PCI/PCI Express inbound window 3-0
// inbound window 1 supports only a 32-bit base address and does not
// define an inbound window base extended address register.
//
    pub piw: [pci_inbound_window_regs; 4],
    pub /: *mut *mut __be32 pex_err_dr; / 0x.e00 - PCI/PCIE error detect register,
    pub res21: [u8; 4],
    pub /: *mut *mut __be32 pex_err_en; / 0x.e08 - PCI/PCIE error interrupt enable register,
    pub res22: [u8; 4],
    pub /: *mut *mut __be32 pex_err_disr; / 0x.e10 - PCI/PCIE error disable register,
    pub res23: [u8; 12],
    pub /: *mut *mut __be32 pex_err_cap_stat; / 0x.e20 - PCI/PCIE error capture status register,
    pub res24: [u8; 4],
    pub /: *mut *mut __be32 pex_err_cap_r0; / 0x.e28 - PCIE error capture register 0,
    pub /: *mut *mut __be32 pex_err_cap_r1; / 0x.e2c - PCIE error capture register 0,
    pub /: *mut *mut __be32 pex_err_cap_r2; / 0x.e30 - PCIE error capture register 0,
    pub /: *mut *mut __be32 pex_err_cap_r3; / 0x.e34 - PCIE error capture register 0,
    pub res_e38: [u8; 200],
    pub /: *mut *mut __be32 pdb_stat; / 0x.f00 - PCIE Debug Status,
    pub res_f04: [u8; 16],
    pub 0*/: *mut *mut __be32 pex_csr0; / 0x.f14 - PEX Control/Status register,
pub const PEX_CSR0_LTSSM_MASK: c_uint = 0xFC;
pub const PEX_CSR0_LTSSM_SHIFT: c_int = 2;
pub const PEX_CSR0_LTSSM_L0: c_uint = 0x11;
    pub 1*/: *mut *mut __be32 pex_csr1; / 0x.f18 - PEX Control/Status register,
    pub res_f1c: [u8; 228],
}

extern "C" {
    pub fn fsl_pcibios_fixup_bus(bus: *mut pci_bus);
}
extern "C" {
    pub fn fsl_pcibios_fixup_phb(phb: *mut pci_controller);
}
extern "C" {
    pub fn mpc83xx_add_bridge(dev: *mut device_node) -> c_int;
}
extern "C" {
    pub fn fsl_pci_immrbar_base(hose: *mut pci_controller) -> u64;
}

extern "C" {
    pub fn fsl_pci_assign_primary() -> void __init;
}

extern "C" {
    pub fn fsl_pci_mcheck_exception(: *mut pt_regs) -> c_int;
}

