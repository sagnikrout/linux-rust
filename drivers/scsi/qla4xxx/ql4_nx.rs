//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/qla4xxx/ql4_nx.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// QLogic iSCSI HBA Driver
// Copyright (c)  2003-2013 QLogic Corporation
//
// Following are the states of the Phantom. Phantom will set them and
// Host will read to check if the fields are correct.
//
pub const PHAN_INITIALIZE_FAILED: c_uint = 0xffff;
pub const PHAN_INITIALIZE_COMPLETE: c_uint = 0xff01;
// Host writes the following to notify that it has done the init-handshake
pub const PHAN_INITIALIZE_ACK: c_uint = 0xf00f;
pub const PHAN_PEG_RCV_INITIALIZED: c_uint = 0xff01;
// CRB_RELATED

pub const CRB_CMDPEG_CHECK_RETRY_COUNT: c_int = 60;
pub const CRB_CMDPEG_CHECK_DELAY: c_int = 500;

//
// Temperature control.
//
pub const CRB_NIU_XG_PAUSE_CTL_P0: c_uint = 0x1;
pub const CRB_NIU_XG_PAUSE_CTL_P1: c_uint = 0x8;
pub const QLA82XX_HW_H0_CH_HUB_ADR: c_uint = 0x05;
pub const QLA82XX_HW_H1_CH_HUB_ADR: c_uint = 0x0E;
pub const QLA82XX_HW_H2_CH_HUB_ADR: c_uint = 0x03;
pub const QLA82XX_HW_H3_CH_HUB_ADR: c_uint = 0x01;
pub const QLA82XX_HW_H4_CH_HUB_ADR: c_uint = 0x06;
pub const QLA82XX_HW_H5_CH_HUB_ADR: c_uint = 0x07;
pub const QLA82XX_HW_H6_CH_HUB_ADR: c_uint = 0x08;
// Hub 0
pub const QLA82XX_HW_MN_CRB_AGT_ADR: c_uint = 0x15;
pub const QLA82XX_HW_MS_CRB_AGT_ADR: c_uint = 0x25;
// Hub 1
pub const QLA82XX_HW_PS_CRB_AGT_ADR: c_uint = 0x73;
pub const QLA82XX_HW_QMS_CRB_AGT_ADR: c_uint = 0x00;
pub const QLA82XX_HW_RPMX3_CRB_AGT_ADR: c_uint = 0x0b;
pub const QLA82XX_HW_SQGS0_CRB_AGT_ADR: c_uint = 0x01;
pub const QLA82XX_HW_SQGS1_CRB_AGT_ADR: c_uint = 0x02;
pub const QLA82XX_HW_SQGS2_CRB_AGT_ADR: c_uint = 0x03;
pub const QLA82XX_HW_SQGS3_CRB_AGT_ADR: c_uint = 0x04;
pub const QLA82XX_HW_C2C0_CRB_AGT_ADR: c_uint = 0x58;
pub const QLA82XX_HW_C2C1_CRB_AGT_ADR: c_uint = 0x59;
pub const QLA82XX_HW_C2C2_CRB_AGT_ADR: c_uint = 0x5a;
pub const QLA82XX_HW_RPMX2_CRB_AGT_ADR: c_uint = 0x0a;
pub const QLA82XX_HW_RPMX4_CRB_AGT_ADR: c_uint = 0x0c;
pub const QLA82XX_HW_RPMX7_CRB_AGT_ADR: c_uint = 0x0f;
pub const QLA82XX_HW_RPMX9_CRB_AGT_ADR: c_uint = 0x12;
pub const QLA82XX_HW_SMB_CRB_AGT_ADR: c_uint = 0x18;
// Hub 2
pub const QLA82XX_HW_NIU_CRB_AGT_ADR: c_uint = 0x31;
pub const QLA82XX_HW_I2C0_CRB_AGT_ADR: c_uint = 0x19;
pub const QLA82XX_HW_I2C1_CRB_AGT_ADR: c_uint = 0x29;
pub const QLA82XX_HW_SN_CRB_AGT_ADR: c_uint = 0x10;
pub const QLA82XX_HW_I2Q_CRB_AGT_ADR: c_uint = 0x20;
pub const QLA82XX_HW_LPC_CRB_AGT_ADR: c_uint = 0x22;
pub const QLA82XX_HW_ROMUSB_CRB_AGT_ADR: c_uint = 0x21;
pub const QLA82XX_HW_QM_CRB_AGT_ADR: c_uint = 0x66;
pub const QLA82XX_HW_SQG0_CRB_AGT_ADR: c_uint = 0x60;
pub const QLA82XX_HW_SQG1_CRB_AGT_ADR: c_uint = 0x61;
pub const QLA82XX_HW_SQG2_CRB_AGT_ADR: c_uint = 0x62;
pub const QLA82XX_HW_SQG3_CRB_AGT_ADR: c_uint = 0x63;
pub const QLA82XX_HW_RPMX1_CRB_AGT_ADR: c_uint = 0x09;
pub const QLA82XX_HW_RPMX5_CRB_AGT_ADR: c_uint = 0x0d;
pub const QLA82XX_HW_RPMX6_CRB_AGT_ADR: c_uint = 0x0e;
pub const QLA82XX_HW_RPMX8_CRB_AGT_ADR: c_uint = 0x11;
// Hub 3
pub const QLA82XX_HW_PH_CRB_AGT_ADR: c_uint = 0x1A;
pub const QLA82XX_HW_SRE_CRB_AGT_ADR: c_uint = 0x50;
pub const QLA82XX_HW_EG_CRB_AGT_ADR: c_uint = 0x51;
pub const QLA82XX_HW_RPMX0_CRB_AGT_ADR: c_uint = 0x08;
// Hub 4
pub const QLA82XX_HW_PEGN0_CRB_AGT_ADR: c_uint = 0x40;
pub const QLA82XX_HW_PEGN1_CRB_AGT_ADR: c_uint = 0x41;
pub const QLA82XX_HW_PEGN2_CRB_AGT_ADR: c_uint = 0x42;
pub const QLA82XX_HW_PEGN3_CRB_AGT_ADR: c_uint = 0x43;
pub const QLA82XX_HW_PEGNI_CRB_AGT_ADR: c_uint = 0x44;
pub const QLA82XX_HW_PEGND_CRB_AGT_ADR: c_uint = 0x45;
pub const QLA82XX_HW_PEGNC_CRB_AGT_ADR: c_uint = 0x46;
pub const QLA82XX_HW_PEGR0_CRB_AGT_ADR: c_uint = 0x47;
pub const QLA82XX_HW_PEGR1_CRB_AGT_ADR: c_uint = 0x48;
pub const QLA82XX_HW_PEGR2_CRB_AGT_ADR: c_uint = 0x49;
pub const QLA82XX_HW_PEGR3_CRB_AGT_ADR: c_uint = 0x4a;
pub const QLA82XX_HW_PEGN4_CRB_AGT_ADR: c_uint = 0x4b;
// Hub 5
pub const QLA82XX_HW_PEGS0_CRB_AGT_ADR: c_uint = 0x40;
pub const QLA82XX_HW_PEGS1_CRB_AGT_ADR: c_uint = 0x41;
pub const QLA82XX_HW_PEGS2_CRB_AGT_ADR: c_uint = 0x42;
pub const QLA82XX_HW_PEGS3_CRB_AGT_ADR: c_uint = 0x43;
pub const QLA82XX_HW_PEGSI_CRB_AGT_ADR: c_uint = 0x44;
pub const QLA82XX_HW_PEGSD_CRB_AGT_ADR: c_uint = 0x45;
pub const QLA82XX_HW_PEGSC_CRB_AGT_ADR: c_uint = 0x46;
// Hub 6
pub const QLA82XX_HW_CAS0_CRB_AGT_ADR: c_uint = 0x46;
pub const QLA82XX_HW_CAS1_CRB_AGT_ADR: c_uint = 0x47;
pub const QLA82XX_HW_CAS2_CRB_AGT_ADR: c_uint = 0x48;
pub const QLA82XX_HW_CAS3_CRB_AGT_ADR: c_uint = 0x49;
pub const QLA82XX_HW_NCM_CRB_AGT_ADR: c_uint = 0x16;
pub const QLA82XX_HW_TMR_CRB_AGT_ADR: c_uint = 0x17;
pub const QLA82XX_HW_XDMA_CRB_AGT_ADR: c_uint = 0x05;
pub const QLA82XX_HW_OCM0_CRB_AGT_ADR: c_uint = 0x06;
pub const QLA82XX_HW_OCM1_CRB_AGT_ADR: c_uint = 0x07;
// This field defines PCI/X adr [25:20] of agents on the CRB
//
pub const QLA82XX_HW_PX_MAP_CRB_PH: c_int = 0;
pub const QLA82XX_HW_PX_MAP_CRB_PS: c_int = 1;
pub const QLA82XX_HW_PX_MAP_CRB_MN: c_int = 2;
pub const QLA82XX_HW_PX_MAP_CRB_MS: c_int = 3;
pub const QLA82XX_HW_PX_MAP_CRB_SRE: c_int = 5;
pub const QLA82XX_HW_PX_MAP_CRB_NIU: c_int = 6;
pub const QLA82XX_HW_PX_MAP_CRB_QMN: c_int = 7;
pub const QLA82XX_HW_PX_MAP_CRB_SQN0: c_int = 8;
pub const QLA82XX_HW_PX_MAP_CRB_SQN1: c_int = 9;
pub const QLA82XX_HW_PX_MAP_CRB_SQN2: c_int = 10;
pub const QLA82XX_HW_PX_MAP_CRB_SQN3: c_int = 11;
pub const QLA82XX_HW_PX_MAP_CRB_QMS: c_int = 12;
pub const QLA82XX_HW_PX_MAP_CRB_SQS0: c_int = 13;
pub const QLA82XX_HW_PX_MAP_CRB_SQS1: c_int = 14;
pub const QLA82XX_HW_PX_MAP_CRB_SQS2: c_int = 15;
pub const QLA82XX_HW_PX_MAP_CRB_SQS3: c_int = 16;
pub const QLA82XX_HW_PX_MAP_CRB_PGN0: c_int = 17;
pub const QLA82XX_HW_PX_MAP_CRB_PGN1: c_int = 18;
pub const QLA82XX_HW_PX_MAP_CRB_PGN2: c_int = 19;
pub const QLA82XX_HW_PX_MAP_CRB_PGN3: c_int = 20;

pub const QLA82XX_HW_PX_MAP_CRB_PGND: c_int = 21;
pub const QLA82XX_HW_PX_MAP_CRB_PGNI: c_int = 22;
pub const QLA82XX_HW_PX_MAP_CRB_PGS0: c_int = 23;
pub const QLA82XX_HW_PX_MAP_CRB_PGS1: c_int = 24;
pub const QLA82XX_HW_PX_MAP_CRB_PGS2: c_int = 25;
pub const QLA82XX_HW_PX_MAP_CRB_PGS3: c_int = 26;
pub const QLA82XX_HW_PX_MAP_CRB_PGSD: c_int = 27;
pub const QLA82XX_HW_PX_MAP_CRB_PGSI: c_int = 28;
pub const QLA82XX_HW_PX_MAP_CRB_SN: c_int = 29;
pub const QLA82XX_HW_PX_MAP_CRB_EG: c_int = 31;
pub const QLA82XX_HW_PX_MAP_CRB_PH2: c_int = 32;
pub const QLA82XX_HW_PX_MAP_CRB_PS2: c_int = 33;
pub const QLA82XX_HW_PX_MAP_CRB_CAM: c_int = 34;
pub const QLA82XX_HW_PX_MAP_CRB_CAS0: c_int = 35;
pub const QLA82XX_HW_PX_MAP_CRB_CAS1: c_int = 36;
pub const QLA82XX_HW_PX_MAP_CRB_CAS2: c_int = 37;
pub const QLA82XX_HW_PX_MAP_CRB_C2C0: c_int = 38;
pub const QLA82XX_HW_PX_MAP_CRB_C2C1: c_int = 39;
pub const QLA82XX_HW_PX_MAP_CRB_TIMR: c_int = 40;
pub const QLA82XX_HW_PX_MAP_CRB_RPMX1: c_int = 42;
pub const QLA82XX_HW_PX_MAP_CRB_RPMX2: c_int = 43;
pub const QLA82XX_HW_PX_MAP_CRB_RPMX3: c_int = 44;
pub const QLA82XX_HW_PX_MAP_CRB_RPMX4: c_int = 45;
pub const QLA82XX_HW_PX_MAP_CRB_RPMX5: c_int = 46;
pub const QLA82XX_HW_PX_MAP_CRB_RPMX6: c_int = 47;
pub const QLA82XX_HW_PX_MAP_CRB_RPMX7: c_int = 48;
pub const QLA82XX_HW_PX_MAP_CRB_XDMA: c_int = 49;
pub const QLA82XX_HW_PX_MAP_CRB_I2Q: c_int = 50;
pub const QLA82XX_HW_PX_MAP_CRB_ROMUSB: c_int = 51;
pub const QLA82XX_HW_PX_MAP_CRB_CAS3: c_int = 52;
pub const QLA82XX_HW_PX_MAP_CRB_RPMX0: c_int = 53;
pub const QLA82XX_HW_PX_MAP_CRB_RPMX8: c_int = 54;
pub const QLA82XX_HW_PX_MAP_CRB_RPMX9: c_int = 55;
pub const QLA82XX_HW_PX_MAP_CRB_OCM0: c_int = 56;
pub const QLA82XX_HW_PX_MAP_CRB_OCM1: c_int = 57;
pub const QLA82XX_HW_PX_MAP_CRB_SMB: c_int = 58;
pub const QLA82XX_HW_PX_MAP_CRB_I2C0: c_int = 59;
pub const QLA82XX_HW_PX_MAP_CRB_I2C1: c_int = 60;
pub const QLA82XX_HW_PX_MAP_CRB_LPC: c_int = 61;
pub const QLA82XX_HW_PX_MAP_CRB_PGNC: c_int = 62;
pub const QLA82XX_HW_PX_MAP_CRB_PGR0: c_int = 63;
pub const QLA82XX_HW_PX_MAP_CRB_PGR1: c_int = 4;
pub const QLA82XX_HW_PX_MAP_CRB_PGR2: c_int = 30;
pub const QLA82XX_HW_PX_MAP_CRB_PGR3: c_int = 41;
// This field defines CRB adr [31:20] of the agents
//

// Lock IDs for ROM lock
pub const ROM_LOCK_DRIVER: c_uint = 0x0d417340;
pub const QLA82XX_PCI_CRB_WINDOWSIZE: c_uint = 0x00100000    /* all are 1MB windows */;

// HACK upon HACK upon HACK (for PCIE builds)

// window 1 pcie slot

//
// ====================== BASE ADDRESSES ON-CHIP ======================
// Base addresses of major components on-chip.
// ====================== BASE ADDRESSES ON-CHIP ======================
//

// Imbus address bit used to indicate a host address. This bit is
// eliminated by the pcie bar and bar select before presentation
// over pcie.
// host memory via IMBUS

// PCI Windowing for DDR regions.

//
// Register offsets for MN
//

pub const MIU_TEST_AGT_ADDR_MASK: c_uint = 0xfffffff8;

// MIU_TEST_AGT_CTRL flags. work for SIU as well
pub const MIU_TA_CTL_START: c_int = 1;
pub const MIU_TA_CTL_ENABLE: c_int = 2;
pub const MIU_TA_CTL_WRITE: c_int = 4;
pub const MIU_TA_CTL_BUSY: c_int = 8;

// CAM RAM

pub const HALT_STATUS_UNRECOVERABLE: c_uint = 0x80000000;
pub const HALT_STATUS_RECOVERABLE: c_uint = 0x40000000;

// Driver Coexistence Defines

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qla_regs {
    QLA8XXX_PEG_HALT_STATUS1 = 0,
    QLA8XXX_PEG_HALT_STATUS2,
    QLA8XXX_PEG_ALIVE_COUNTER,
    QLA8XXX_CRB_DRV_ACTIVE,
    QLA8XXX_CRB_DEV_STATE,
    QLA8XXX_CRB_DRV_STATE,
    QLA8XXX_CRB_DRV_SCRATCH,
    QLA8XXX_CRB_DEV_PART_INFO,
    QLA8XXX_CRB_DRV_IDC_VERSION,
    QLA8XXX_FW_VERSION_MAJOR,
    QLA8XXX_FW_VERSION_MINOR,
    QLA8XXX_FW_VERSION_SUB,
    QLA8XXX_CRB_CMDPEG_STATE,
    QLA8XXX_CRB_TEMP_STATE,
}

// Every driver should use these Device State
pub const QLA8XXX_DEV_COLD: c_int = 1;
pub const QLA8XXX_DEV_INITIALIZING: c_int = 2;
pub const QLA8XXX_DEV_READY: c_int = 3;
pub const QLA8XXX_DEV_NEED_RESET: c_int = 4;
pub const QLA8XXX_DEV_NEED_QUIESCENT: c_int = 5;
pub const QLA8XXX_DEV_FAILED: c_int = 6;
pub const QLA8XXX_DEV_QUIESCENT: c_int = 7;

pub const QLA82XX_IDC_VERSION: c_uint = 0x1;
pub const ROM_DEV_INIT_TIMEOUT: c_int = 30;
pub const ROM_DRV_RESET_ACK_TIMEOUT: c_int = 10;

//
// The PCI VendorID and DeviceID for our board.
//
pub const QLA82XX_MSIX_TBL_SPACE: c_int = 8192;
pub const QLA82XX_PCI_REG_MSIX_TBL: c_uint = 0x44;
pub const QLA82XX_PCI_MSIX_CONTROL: c_uint = 0x40;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crb_128M_2M_sub_block_map {
    pub valid: unsigned,
    pub start_128M: unsigned,
    pub end_128M: unsigned,
    pub start_2M: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crb_128M_2M_block_map {
    pub sub_block: [crb_128M_2M_sub_block_map; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crb_addr_pair {
    pub addr: c_long,
    pub data: c_long,
}

pub const MAX_CTL_CHECK: c_int = 1000;

//
// PCI related defines.
//
// Interrupt related defines.
//

//
// Message Signaled Interrupts
//

//

//
// Interrupt state machine and other bits.
//

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
// struct qla4_8xxx_legacy_intr_set defined in ql4_def.h

// Magic number to let user know flash is programmed
pub const QLA82XX_BDINFO_MAGIC: c_uint = 0x12345678;

// QLA82XX additions

// Minidump related
// Entry Type Defines
pub const QLA8XXX_RDNOP: c_int = 0;
pub const QLA8XXX_RDCRB: c_int = 1;
pub const QLA8XXX_RDMUX: c_int = 2;
pub const QLA8XXX_QUEUE: c_int = 3;
pub const QLA8XXX_BOARD: c_int = 4;
pub const QLA8XXX_RDOCM: c_int = 6;
pub const QLA8XXX_PREGS: c_int = 7;
pub const QLA8XXX_L1DTG: c_int = 8;
pub const QLA8XXX_L1ITG: c_int = 9;
pub const QLA8XXX_L1DAT: c_int = 11;
pub const QLA8XXX_L1INS: c_int = 12;
pub const QLA8XXX_L2DTG: c_int = 21;
pub const QLA8XXX_L2ITG: c_int = 22;
pub const QLA8XXX_L2DAT: c_int = 23;
pub const QLA8XXX_L2INS: c_int = 24;
pub const QLA83XX_POLLRD: c_int = 35;
pub const QLA83XX_RDMUX2: c_int = 36;
pub const QLA83XX_POLLRDMWR: c_int = 37;
pub const QLA8044_RDDFE: c_int = 38;
pub const QLA8044_RDMDIO: c_int = 39;
pub const QLA8044_POLLWR: c_int = 40;
pub const QLA8XXX_RDROM: c_int = 71;
pub const QLA8XXX_RDMEM: c_int = 72;
pub const QLA8XXX_CNTRL: c_int = 98;
pub const QLA83XX_TLHDR: c_int = 99;
pub const QLA8XXX_RDEND: c_int = 255;
// Opcodes for Control Entries.
// These Flags are bit fields.
//
pub const QLA8XXX_DBG_OPCODE_WR: c_uint = 0x01;
pub const QLA8XXX_DBG_OPCODE_RW: c_uint = 0x02;
pub const QLA8XXX_DBG_OPCODE_AND: c_uint = 0x04;
pub const QLA8XXX_DBG_OPCODE_OR: c_uint = 0x08;
pub const QLA8XXX_DBG_OPCODE_POLL: c_uint = 0x10;
pub const QLA8XXX_DBG_OPCODE_RDSTATE: c_uint = 0x20;
pub const QLA8XXX_DBG_OPCODE_WRSTATE: c_uint = 0x40;
pub const QLA8XXX_DBG_OPCODE_MDSTATE: c_uint = 0x80;
// Driver Flags
pub const QLA8XXX_DBG_SKIPPED_FLAG: c_uint = 0x80 /* driver skipped this entry  */;
pub const QLA8XXX_DBG_SIZE_ERR_FLAG: c_uint = 0x40 /* Entry vs Capture size;
// mismatch
// Driver_code is for driver to write some info about the entry
// currently not used.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla8xxx_minidump_entry_hdr {
    pub entry_type: u32,
    pub entry_size: u32,
    pub entry_capture_size: u32,
    pub entry_capture_mask: u8,
    pub entry_code: u8,
    pub driver_code: u8,
    pub driver_flags: u8,
    pub d_ctrl: },
}

// Read CRB entry header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla8xxx_minidump_entry_crb {
    pub h: qla8xxx_minidump_entry_hdr,
    pub addr: u32,
    pub addr_stride: u8,
    pub state_index_a: u8,
    pub poll_timeout: u16,
    pub crb_strd: },
    pub data_size: u32,
    pub op_count: u32,
    pub opcode: u8,
    pub state_index_v: u8,
    pub shl: u8,
    pub shr: u8,
    pub crb_ctrl: },
    pub value_1: u32,
    pub value_2: u32,
    pub value_3: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla8xxx_minidump_entry_cache {
    pub h: qla8xxx_minidump_entry_hdr,
    pub tag_reg_addr: u32,
    pub tag_value_stride: u16,
    pub init_tag_value: u16,
    pub addr_ctrl: },
    pub data_size: u32,
    pub op_count: u32,
    pub control_addr: u32,
    pub write_value: u16,
    pub poll_mask: u8,
    pub poll_wait: u8,
    pub cache_ctrl: },
    pub read_addr: u32,
    pub read_addr_stride: u8,
    pub read_addr_cnt: u8,
    pub rsvd_1: u16,
    pub read_ctrl: },
}

// Read OCM
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla8xxx_minidump_entry_rdocm {
    pub h: qla8xxx_minidump_entry_hdr,
    pub rsvd_0: u32,
    pub rsvd_1: u32,
    pub data_size: u32,
    pub op_count: u32,
    pub rsvd_2: u32,
    pub rsvd_3: u32,
    pub read_addr: u32,
    pub read_addr_stride: u32,
}

// Read Memory
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla8xxx_minidump_entry_rdmem {
    pub h: qla8xxx_minidump_entry_hdr,
    pub rsvd: [u32; 6],
    pub read_addr: u32,
    pub read_data_size: u32,
}

// Read ROM
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla8xxx_minidump_entry_rdrom {
    pub h: qla8xxx_minidump_entry_hdr,
    pub rsvd: [u32; 6],
    pub read_addr: u32,
    pub read_data_size: u32,
}

// Mux entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla8xxx_minidump_entry_mux {
    pub h: qla8xxx_minidump_entry_hdr,
    pub select_addr: u32,
    pub rsvd_0: u32,
    pub data_size: u32,
    pub op_count: u32,
    pub select_value: u32,
    pub select_value_stride: u32,
    pub read_addr: u32,
    pub rsvd_1: u32,
}

// Queue entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla8xxx_minidump_entry_queue {
    pub h: qla8xxx_minidump_entry_hdr,
    pub select_addr: u32,
    pub queue_id_stride: u16,
    pub rsvd_0: u16,
    pub q_strd: },
    pub data_size: u32,
    pub op_count: u32,
    pub rsvd_1: u32,
    pub rsvd_2: u32,
    pub read_addr: u32,
    pub read_addr_stride: u8,
    pub read_addr_cnt: u8,
    pub rsvd_3: u16,
    pub rd_strd: },
}

pub const MBC_DIAGNOSTIC_MINIDUMP_TEMPLATE: c_uint = 0x129;
pub const RQST_TMPLT_SIZE: c_uint = 0x0;
pub const RQST_TMPLT: c_uint = 0x1;
pub const MD_DIRECT_ROM_WINDOW: c_uint = 0x42110030;
pub const MD_DIRECT_ROM_READ_BASE: c_uint = 0x42150000;
pub const MD_MIU_TEST_AGT_CTRL: c_uint = 0x41000090;
pub const MD_MIU_TEST_AGT_ADDR_LO: c_uint = 0x41000094;
pub const MD_MIU_TEST_AGT_ADDR_HI: c_uint = 0x41000098;
pub const MD_MIU_TEST_AGT_WRDATA_LO: c_uint = 0x410000A0;
pub const MD_MIU_TEST_AGT_WRDATA_HI: c_uint = 0x410000A4;
pub const MD_MIU_TEST_AGT_WRDATA_ULO: c_uint = 0x410000B0;
pub const MD_MIU_TEST_AGT_WRDATA_UHI: c_uint = 0x410000B4;
