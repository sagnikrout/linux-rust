//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qlogic/netxen/netxen_nic_hdr.h
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
// Copyright (C) 2003 - 2009 NetXen, Inc.
// Copyright (C) 2009 - QLogic Corporation.
// All rights reserved.
//

//
// The basic unit of access when reading/writing control registers.
//
// Hub 0
// Hub 1
// Hub 2
// Hub 3
// Hub 4
// Hub 5
// Hub 6
// Floaters - non existent modules
pub const NETXEN_HW_EFC_RPMX0_CRB_AGT_ADR: c_uint = 0x67;
// This field defines PCI/X adr [25:20] of agents on the CRB
// This field defines CRB adr [31:20] of the agents

pub const CRB_REG_EX_PC: c_uint = 0x3c;

// Lock IDs for ROM lock
pub const ROM_LOCK_DRIVER: c_uint = 0x0d417340;
//
// Definitions specific to M25P flash
//
// Instructions
//
pub const M25P_INSTR_WREN: c_uint = 0x06;
pub const M25P_INSTR_WRDI: c_uint = 0x04;
pub const M25P_INSTR_RDID: c_uint = 0x9f;
pub const M25P_INSTR_RDSR: c_uint = 0x05;
pub const M25P_INSTR_WRSR: c_uint = 0x01;
pub const M25P_INSTR_READ: c_uint = 0x03;
pub const M25P_INSTR_FAST_READ: c_uint = 0x0b;
pub const M25P_INSTR_PP: c_uint = 0x02;
pub const M25P_INSTR_SE: c_uint = 0xd8;
pub const M25P_INSTR_BE: c_uint = 0xc7;
pub const M25P_INSTR_DP: c_uint = 0xb9;
pub const M25P_INSTR_RES: c_uint = 0xab;
// all are 1MB windows
pub const NETXEN_PCI_CRB_WINDOWSIZE: c_uint = 0x00100000;

pub const NETXEN_PCI_MAPSIZE: c_int = 128;

//
// Register offsets for MN
//

// 200ms delay in each loop
pub const NETXEN_NIU_PHY_WAITLEN: c_int = 200000;
// 10 seconds before we give up
pub const NETXEN_NIU_PHY_WAITMAX: c_int = 50;
pub const NETXEN_NIU_MAX_GBE_PORTS: c_int = 4;
pub const NETXEN_NIU_MAX_XG_PORTS: c_int = 2;

// P3 802.3ap

pub const TA_CTL_START: c_int = 1;
pub const TA_CTL_ENABLE: c_int = 2;
pub const TA_CTL_WRITE: c_int = 4;
pub const TA_CTL_BUSY: c_int = 8;
//
// Register offsets for MN
//

pub const MIU_TEST_AGT_ADDR_MASK: c_uint = 0xfffffff8;

//
// Register offsets for MS
//

pub const SIU_TEST_AGT_ADDR_MASK: c_uint = 0x3ffff8;

// XG Link status
pub const XG_LINK_UP: c_uint = 0x10;
pub const XG_LINK_DOWN: c_uint = 0x20;
pub const XG_LINK_UP_P3: c_uint = 0x01;
pub const XG_LINK_DOWN_P3: c_uint = 0x02;
pub const XG_LINK_STATE_P3_MASK: c_uint = 0xf;

pub const P3_LINK_SPEED_MHZ: c_int = 100;
pub const P3_LINK_SPEED_MASK: c_uint = 0xff;

pub const NETXEN_MSI_MODE: c_uint = 0x1;
pub const NETXEN_INTX_MODE: c_uint = 0x2;

//
// capabilities register, can be used to selectively enable/disable features
// for backward compatibility
//

pub const INTR_SCHEME_PERPORT: c_uint = 0x1;
pub const MSI_MODE_MULTIFUNC: c_uint = 0x1;
// used for ethtool tests

//
// CrbPortPhanCntrHi/Lo is used to pass the address of HostPhantomIndex address
// which can be read by the Phantom host to get producer/consumer indexes from
// Phantom/Casper. If it is not HOST_SHARED_MEMORY, then the following
// registers will be used for the addresses of the ring's shared memory
// on the Phantom.
//

//
// Temperature control.
//
// Lock IDs for PHY lock
pub const PHY_LOCK_DRIVER: c_uint = 0x44524956;
// Used for PS PCI Memory access

// via CRB  (PS side only)

pub const PCIE_DCR: c_uint = 0x00d8;

pub const NETXEN_PORT_MODE_NONE: c_int = 0;
pub const NETXEN_PORT_MODE_XG: c_int = 1;
pub const NETXEN_PORT_MODE_GB: c_int = 2;
pub const NETXEN_PORT_MODE_802_3_AP: c_int = 3;
pub const NETXEN_PORT_MODE_AUTO_NEG: c_int = 4;
pub const NETXEN_PORT_MODE_AUTO_NEG_1G: c_int = 5;
pub const NETXEN_PORT_MODE_AUTO_NEG_XG: c_int = 6;

pub const NX_PEG_TUNE_MN_PRESENT: c_uint = 0x1;

// MiniDIMM related macros

pub const NETXEN_DIMM_PRESENT: c_uint = 0x1;
pub const NETXEN_DIMM_MEMTYPE_DDR2_SDRAM: c_uint = 0x2;
pub const NETXEN_DIMM_SIZE: c_uint = 0x4;

pub const NETXEN_DIMM_VALID_FLAG: c_uint = 0x80000000;
pub const NETXEN_DIMM_MEM_DDR2_SDRAM: c_uint = 0x8;
pub const NETXEN_DIMM_STD_MEM_SIZE: c_int = 512;
pub const NETXEN_DIMM_TYPE_RDIMM: c_uint = 0x1;
pub const NETXEN_DIMM_TYPE_UDIMM: c_uint = 0x2;
pub const NETXEN_DIMM_TYPE_SO_DIMM: c_uint = 0x4;
pub const NETXEN_DIMM_TYPE_Micro_DIMM: c_uint = 0x8;
pub const NETXEN_DIMM_TYPE_Mini_RDIMM: c_uint = 0x10;
pub const NETXEN_DIMM_TYPE_Mini_UDIMM: c_uint = 0x20;
// Device State
pub const NX_DEV_COLD: c_int = 1;
pub const NX_DEV_INITALIZING: c_int = 2;
pub const NX_DEV_READY: c_int = 3;
pub const NX_DEV_NEED_RESET: c_int = 4;
pub const NX_DEV_NEED_QUISCENT: c_int = 5;
pub const NX_DEV_NEED_AER: c_int = 6;
pub const NX_DEV_FAILED: c_int = 7;
pub const NX_RCODE_DRIVER_INFO: c_uint = 0x20000000;
pub const NX_RCODE_DRIVER_CAN_RELOAD: c_uint = 0x40000000;
pub const NX_RCODE_FATAL_ERROR: c_uint = 0x80000000;

pub const FW_FAIL_THRESH: c_int = 3;
pub const FW_POLL_THRESH: c_int = 10;

//
// PCI Interrupt Vector Values.
//
pub const PCIX_INT_VECTOR_BIT_F0: c_uint = 0x0080;
pub const PCIX_INT_VECTOR_BIT_F1: c_uint = 0x0100;
pub const PCIX_INT_VECTOR_BIT_F2: c_uint = 0x0200;
pub const PCIX_INT_VECTOR_BIT_F3: c_uint = 0x0400;
pub const PCIX_INT_VECTOR_BIT_F4: c_uint = 0x0800;
pub const PCIX_INT_VECTOR_BIT_F5: c_uint = 0x1000;
pub const PCIX_INT_VECTOR_BIT_F6: c_uint = 0x2000;
pub const PCIX_INT_VECTOR_BIT_F7: c_uint = 0x4000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netxen_legacy_intr_set {
    pub int_vec_bit: u32,
    pub tgt_status_reg: u32,
    pub tgt_mask_reg: u32,
    pub pci_int_reg: u32,
}

