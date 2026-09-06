//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/mei/hw-me-regs.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (c) 2003-2022, Intel Corporation. All rights reserved.
// Intel Management Engine Interface (Intel MEI) Linux driver
//

//
// MEI device IDs
//
pub const PCI_DEVICE_ID_INTEL_MEI_82946GZ: c_uint = 0x2974  /* 82946GZ/GL */;
pub const PCI_DEVICE_ID_INTEL_MEI_82G35: c_uint = 0x2984  /* 82G35 Express */;
pub const PCI_DEVICE_ID_INTEL_MEI_82Q965: c_uint = 0x2994  /* 82Q963/Q965 */;
pub const PCI_DEVICE_ID_INTEL_MEI_82G965: c_uint = 0x29A4  /* 82P965/G965 */;
pub const PCI_DEVICE_ID_INTEL_MEI_82GM965: c_uint = 0x2A04  /* Mobile PM965/GM965 */;
pub const PCI_DEVICE_ID_INTEL_MEI_82GME965: c_uint = 0x2A14  /* Mobile GME965/GLE960 */;
pub const PCI_DEVICE_ID_INTEL_MEI_ICH9_82Q35: c_uint = 0x29B4  /* 82Q35 Express */;
pub const PCI_DEVICE_ID_INTEL_MEI_ICH9_82G33: c_uint = 0x29C4  /* 82G33/G31/P35/P31 Express */;
pub const PCI_DEVICE_ID_INTEL_MEI_ICH9_82Q33: c_uint = 0x29D4  /* 82Q33 Express */;
pub const PCI_DEVICE_ID_INTEL_MEI_ICH9_82X38: c_uint = 0x29E4  /* 82X38/X48 Express */;
pub const PCI_DEVICE_ID_INTEL_MEI_ICH9_3200: c_uint = 0x29F4  /* 3200/3210 Server */;
pub const PCI_DEVICE_ID_INTEL_MEI_ICH9_6: c_uint = 0x28B4  /* Bearlake */;
pub const PCI_DEVICE_ID_INTEL_MEI_ICH9_7: c_uint = 0x28C4  /* Bearlake */;
pub const PCI_DEVICE_ID_INTEL_MEI_ICH9_8: c_uint = 0x28D4  /* Bearlake */;
pub const PCI_DEVICE_ID_INTEL_MEI_ICH9_9: c_uint = 0x28E4  /* Bearlake */;
pub const PCI_DEVICE_ID_INTEL_MEI_ICH9_10: c_uint = 0x28F4  /* Bearlake */;
pub const PCI_DEVICE_ID_INTEL_MEI_ICH9M_1: c_uint = 0x2A44  /* Cantiga */;
pub const PCI_DEVICE_ID_INTEL_MEI_ICH9M_2: c_uint = 0x2A54  /* Cantiga */;
pub const PCI_DEVICE_ID_INTEL_MEI_ICH9M_3: c_uint = 0x2A64  /* Cantiga */;
pub const PCI_DEVICE_ID_INTEL_MEI_ICH9M_4: c_uint = 0x2A74  /* Cantiga */;
pub const PCI_DEVICE_ID_INTEL_MEI_ICH10_1: c_uint = 0x2E04  /* Eaglelake */;
pub const PCI_DEVICE_ID_INTEL_MEI_ICH10_2: c_uint = 0x2E14  /* Eaglelake */;
pub const PCI_DEVICE_ID_INTEL_MEI_ICH10_3: c_uint = 0x2E24  /* Eaglelake */;
pub const PCI_DEVICE_ID_INTEL_MEI_ICH10_4: c_uint = 0x2E34  /* Eaglelake */;
pub const PCI_DEVICE_ID_INTEL_MEI_IBXPK_1: c_uint = 0x3B64  /* Calpella */;
pub const PCI_DEVICE_ID_INTEL_MEI_IBXPK_2: c_uint = 0x3B65  /* Calpella */;
pub const PCI_DEVICE_ID_INTEL_MEI_CPT_1: c_uint = 0x1C3A  /* Couger Point */;
pub const PCI_DEVICE_ID_INTEL_MEI_PBG_1: c_uint = 0x1D3A  /* C600/X79 Patsburg */;
pub const PCI_DEVICE_ID_INTEL_MEI_PPT_1: c_uint = 0x1E3A  /* Panther Point */;
pub const PCI_DEVICE_ID_INTEL_MEI_PPT_2: c_uint = 0x1CBA  /* Panther Point */;
pub const PCI_DEVICE_ID_INTEL_MEI_PPT_3: c_uint = 0x1DBA  /* Panther Point */;
pub const PCI_DEVICE_ID_INTEL_MEI_LPT_H: c_uint = 0x8C3A  /* Lynx Point H */;
pub const PCI_DEVICE_ID_INTEL_MEI_LPT_W: c_uint = 0x8D3A  /* Lynx Point - Wellsburg */;
pub const PCI_DEVICE_ID_INTEL_MEI_LPT_LP: c_uint = 0x9C3A  /* Lynx Point LP */;
pub const PCI_DEVICE_ID_INTEL_MEI_LPT_HR: c_uint = 0x8CBA  /* Lynx Point H Refresh */;
pub const PCI_DEVICE_ID_INTEL_MEI_WPT_LP: c_uint = 0x9CBA  /* Wildcat Point LP */;
pub const PCI_DEVICE_ID_INTEL_MEI_WPT_LP_2: c_uint = 0x9CBB  /* Wildcat Point LP 2 */;
pub const PCI_DEVICE_ID_INTEL_MEI_SPT: c_uint = 0x9D3A  /* Sunrise Point */;
pub const PCI_DEVICE_ID_INTEL_MEI_SPT_2: c_uint = 0x9D3B  /* Sunrise Point 2 */;
pub const PCI_DEVICE_ID_INTEL_MEI_SPT_3: c_uint = 0x9D3E  /* Sunrise Point 3 (iToutch) */;
pub const PCI_DEVICE_ID_INTEL_MEI_SPT_H: c_uint = 0xA13A  /* Sunrise Point H */;
pub const PCI_DEVICE_ID_INTEL_MEI_SPT_H_2: c_uint = 0xA13B  /* Sunrise Point H 2 */;
pub const PCI_DEVICE_ID_INTEL_MEI_LBG: c_uint = 0xA1BA  /* Lewisburg (SPT) */;
pub const PCI_DEVICE_ID_INTEL_MEI_BXT_M: c_uint = 0x1A9A  /* Broxton M */;
pub const PCI_DEVICE_ID_INTEL_MEI_APL_I: c_uint = 0x5A9A  /* Apollo Lake I */;
pub const PCI_DEVICE_ID_INTEL_MEI_DNV_IE: c_uint = 0x19E5  /* Denverton IE */;
pub const PCI_DEVICE_ID_INTEL_MEI_GLK: c_uint = 0x319A  /* Gemini Lake */;
pub const PCI_DEVICE_ID_INTEL_MEI_KBP: c_uint = 0xA2BA  /* Kaby Point */;
pub const PCI_DEVICE_ID_INTEL_MEI_KBP_2: c_uint = 0xA2BB  /* Kaby Point 2 */;
pub const PCI_DEVICE_ID_INTEL_MEI_KBP_3: c_uint = 0xA2BE  /* Kaby Point 3 (iTouch) */;
pub const PCI_DEVICE_ID_INTEL_MEI_CNP_LP: c_uint = 0x9DE0  /* Cannon Point LP */;
pub const PCI_DEVICE_ID_INTEL_MEI_CNP_LP_3: c_uint = 0x9DE4  /* Cannon Point LP 3 (iTouch) */;
pub const PCI_DEVICE_ID_INTEL_MEI_CNP_H: c_uint = 0xA360  /* Cannon Point H */;
pub const PCI_DEVICE_ID_INTEL_MEI_CNP_H_3: c_uint = 0xA364  /* Cannon Point H 3 (iTouch) */;
pub const PCI_DEVICE_ID_INTEL_MEI_CMP_LP: c_uint = 0x02e0  /* Comet Point LP */;
pub const PCI_DEVICE_ID_INTEL_MEI_CMP_LP_3: c_uint = 0x02e4  /* Comet Point LP 3 (iTouch) */;
pub const PCI_DEVICE_ID_INTEL_MEI_CMP_V: c_uint = 0xA3BA  /* Comet Point Lake V */;
pub const PCI_DEVICE_ID_INTEL_MEI_CMP_H: c_uint = 0x06e0  /* Comet Lake H */;
pub const PCI_DEVICE_ID_INTEL_MEI_CMP_H_3: c_uint = 0x06e4  /* Comet Lake H 3 (iTouch) */;
pub const PCI_DEVICE_ID_INTEL_MEI_CDF: c_uint = 0x18D3  /* Cedar Fork */;
pub const PCI_DEVICE_ID_INTEL_MEI_ICP_LP: c_uint = 0x34E0  /* Ice Lake Point LP */;
pub const PCI_DEVICE_ID_INTEL_MEI_ICP_N: c_uint = 0x38E0  /* Ice Lake Point N */;
pub const PCI_DEVICE_ID_INTEL_MEI_JSP_N: c_uint = 0x4DE0  /* Jasper Lake Point N */;
pub const PCI_DEVICE_ID_INTEL_MEI_TGP_LP: c_uint = 0xA0E0  /* Tiger Lake Point LP */;
pub const PCI_DEVICE_ID_INTEL_MEI_TGP_H: c_uint = 0x43E0  /* Tiger Lake Point H */;
pub const PCI_DEVICE_ID_INTEL_MEI_MCC: c_uint = 0x4B70  /* Mule Creek Canyon (EHL) */;
pub const PCI_DEVICE_ID_INTEL_MEI_MCC_4: c_uint = 0x4B75  /* Mule Creek Canyon 4 (EHL) */;
pub const PCI_DEVICE_ID_INTEL_MEI_EBG: c_uint = 0x1BE0  /* Emmitsburg WS */;
pub const PCI_DEVICE_ID_INTEL_MEI_ADP_S: c_uint = 0x7AE8  /* Alder Lake Point S */;
pub const PCI_DEVICE_ID_INTEL_MEI_ADP_LP: c_uint = 0x7A60  /* Alder Lake Point LP */;
pub const PCI_DEVICE_ID_INTEL_MEI_ADP_P: c_uint = 0x51E0  /* Alder Lake Point P */;
pub const PCI_DEVICE_ID_INTEL_MEI_ADP_N: c_uint = 0x54E0  /* Alder Lake Point N */;
pub const PCI_DEVICE_ID_INTEL_MEI_RPL_S: c_uint = 0x7A68  /* Raptor Lake Point S */;
pub const PCI_DEVICE_ID_INTEL_MEI_MTL_M: c_uint = 0x7E70  /* Meteor Lake Point M */;
pub const PCI_DEVICE_ID_INTEL_MEI_ARL_S: c_uint = 0x7F68  /* Arrow Lake Point S */;
pub const PCI_DEVICE_ID_INTEL_MEI_ARL_H: c_uint = 0x7770  /* Arrow Lake Point H */;
pub const PCI_DEVICE_ID_INTEL_MEI_LNL_M: c_uint = 0xA870  /* Lunar Lake Point M */;
pub const PCI_DEVICE_ID_INTEL_MEI_PTL_H: c_uint = 0xE370  /* Panther Lake H */;
pub const PCI_DEVICE_ID_INTEL_MEI_PTL_P: c_uint = 0xE470  /* Panther Lake P */;
pub const PCI_DEVICE_ID_INTEL_MEI_WCL_P: c_uint = 0x4D70  /* Wildcat Lake P */;
pub const PCI_DEVICE_ID_INTEL_MEI_NVL_S: c_uint = 0x6E68  /* Nova Lake Point S */;
pub const PCI_DEVICE_ID_INTEL_MEI_NVL_H: c_uint = 0xD370  /* Nova Lake Point H */;
pub const PCI_DEVICE_ID_INTEL_MEI_CRI: c_uint = 0x6766  /* Crescent Island */;
//
// MEI HW Section
//
// Host Firmware Status Registers in PCI Config Space
pub const PCI_CFG_HFS_1: c_uint = 0x40;

pub const PCI_CFG_HFS_2: c_uint = 0x48;

pub const PCI_CFG_HFS_3: c_uint = 0x60;

pub const PCI_CFG_HFS_4: c_uint = 0x64;
pub const PCI_CFG_HFS_5: c_uint = 0x68;

pub const PCI_CFG_HFS_6: c_uint = 0x6C;
// MEI registers
// H_CB_WW - Host Circular Buffer (CB) Write Window register
pub const H_CB_WW: c_int = 0;
// H_CSR - Host Control Status register
pub const H_CSR: c_int = 4;
// ME_CB_RW - ME Circular Buffer Read Window register (read only)
pub const ME_CB_RW: c_int = 8;
// ME_CSR_HA - ME Control Status Host Access register (read only)
pub const ME_CSR_HA: c_uint = 0xC;
// H_HGC_CSR - PGI register
pub const H_HPG_CSR: c_uint = 0x10;
// H_D0I3C - D0I3 Control
pub const H_D0I3C: c_uint = 0x800;
pub const H_GSC_EXT_OP_MEM_BASE_ADDR_LO_REG: c_uint = 0x100;
pub const H_GSC_EXT_OP_MEM_BASE_ADDR_HI_REG: c_uint = 0x104;
pub const H_GSC_EXT_OP_MEM_LIMIT_REG: c_uint = 0x108;

// register bits of H_CSR (Host Control Status register)
// Host Circular Buffer Depth - maximum number of 32-bit entries in CB
pub const H_CBD: c_uint = 0xFF000000;
// Host Circular Buffer Write Pointer
pub const H_CBWP: c_uint = 0x00FF0000;
// Host Circular Buffer Read Pointer
pub const H_CBRP: c_uint = 0x0000FF00;
// Host Reset
pub const H_RST: c_uint = 0x00000010;
// Host Ready
pub const H_RDY: c_uint = 0x00000008;
// Host Interrupt Generate
pub const H_IG: c_uint = 0x00000004;
// Host Interrupt Status
pub const H_IS: c_uint = 0x00000002;
// Host Interrupt Enable
pub const H_IE: c_uint = 0x00000001;
// Host D0I3 Interrupt Enable
pub const H_D0I3C_IE: c_uint = 0x00000020;
// Host D0I3 Interrupt Status
pub const H_D0I3C_IS: c_uint = 0x00000040;
// H_CSR masks

// register bits of ME_CSR_HA (ME Control Status Host Access register)
// ME CB (Circular Buffer) Depth HRA (Host Read Access) - host read only
pub const ME_CBD_HRA: c_uint = 0xFF000000;
// ME CB Write Pointer HRA - host read only access to ME_CBWP
pub const ME_CBWP_HRA: c_uint = 0x00FF0000;
// ME CB Read Pointer HRA - host read only access to ME_CBRP
pub const ME_CBRP_HRA: c_uint = 0x0000FF00;
// ME Power Gate Isolation Capability HRA  - host ready only access
pub const ME_PGIC_HRA: c_uint = 0x00000040;
// ME Reset HRA - host read only access to ME_RST
pub const ME_RST_HRA: c_uint = 0x00000010;
// ME Ready HRA - host read only access to ME_RDY
pub const ME_RDY_HRA: c_uint = 0x00000008;
// ME Interrupt Generate HRA - host read only access to ME_IG
pub const ME_IG_HRA: c_uint = 0x00000004;
// ME Interrupt Status HRA - host read only access to ME_IS
pub const ME_IS_HRA: c_uint = 0x00000002;
// ME Interrupt Enable HRA - host read only access to ME_IE
pub const ME_IE_HRA: c_uint = 0x00000001;
// TRC control shadow register
pub const ME_TRC: c_uint = 0x00000030;
// H_HPG_CSR register bits
pub const H_HPG_CSR_PGIHEXR: c_uint = 0x00000001;
pub const H_HPG_CSR_PGI: c_uint = 0x00000002;
// H_D0I3C register bits
pub const H_D0I3C_CIP: c_uint = 0x00000001;
pub const H_D0I3C_IR: c_uint = 0x00000002;
pub const H_D0I3C_I3: c_uint = 0x00000004;
pub const H_D0I3C_RR: c_uint = 0x00000008;
