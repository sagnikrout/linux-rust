//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/qla2xxx/qla_nx.h
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
// QLogic Fibre Channel HBA Driver
// Copyright (c)  2003-2014 QLogic Corporation
//

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

pub const QLA82XX_DMA_SHIFT_VALUE: c_uint = 0x55555555;
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

pub const QLA82XX_PCI_CRB_WINDOWSIZE: c_uint = 0x00100000	 /* all are 1MB windows */;

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

pub const QLA82XX_PCI_CRBSPACE: c_uint = 0x06000000UL;
pub const QLA82XX_PCI_DIRECT_CRB: c_uint = 0x04400000UL;
pub const QLA82XX_PCI_CAMQM: c_uint = 0x04800000UL;
pub const QLA82XX_PCI_CAMQM_MAX: c_uint = 0x04ffffffUL;
pub const QLA82XX_PCI_DDR_NET: c_uint = 0x00000000UL;
pub const QLA82XX_PCI_QDR_NET: c_uint = 0x04000000UL;
pub const QLA82XX_PCI_QDR_NET_MAX: c_uint = 0x043fffffUL;
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

// Every driver should use these Device State
pub const QLA8XXX_BAD_VALUE: c_uint = 0xbad0bad0;
pub const QLA82XX_IDC_VERSION: c_int = 1;
pub const QLA82XX_ROM_DEV_INIT_TIMEOUT: c_int = 30;
pub const QLA82XX_ROM_DRV_RESET_ACK_TIMEOUT: c_int = 10;

// Different drive state
pub const QLA82XX_DRVST_NOT_RDY: c_int = 0;
pub const QLA82XX_DRVST_RST_RDY: c_int = 1;
pub const QLA82XX_DRVST_QSNT_RDY: c_int = 2;
// Different drive active state
pub const QLA82XX_DRV_NOT_ACTIVE: c_int = 0;
pub const QLA82XX_DRV_ACTIVE: c_int = 1;
//
// The PCI VendorID and DeviceID for our board.
//
pub const PCI_DEVICE_ID_QLOGIC_ISP8021: c_uint = 0x8021;
pub const PCI_DEVICE_ID_QLOGIC_ISP8044: c_uint = 0x8044;
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla82xx_legacy_intr_set {
    pub int_vec_bit: u32,
    pub tgt_status_reg: u32,
    pub tgt_mask_reg: u32,
    pub pci_int_reg: u32,
}

pub const BRDCFG_START: c_uint = 0x4000;
pub const BOOTLD_START: c_uint = 0x10000;
pub const IMAGE_START: c_uint = 0x100000;
pub const FLASH_ADDR_START: c_uint = 0x43000;
// Magic number to let user know flash is programmed
pub const QLA82XX_BDINFO_MAGIC: c_uint = 0x12345678;

pub const QLA82XX_FW_MIN_SIZE: c_uint = 0x3fffff;
// UNIFIED ROMIMAGE START
pub const QLA82XX_URI_FW_MIN_SIZE: c_uint = 0xc8000;
pub const QLA82XX_URI_DIR_SECT_PRODUCT_TBL: c_uint = 0x0;
pub const QLA82XX_URI_DIR_SECT_BOOTLD: c_uint = 0x6;
pub const QLA82XX_URI_DIR_SECT_FW: c_uint = 0x7;
// Offsets
pub const QLA82XX_URI_CHIP_REV_OFF: c_int = 10;
pub const QLA82XX_URI_FLAGS_OFF: c_int = 11;
pub const QLA82XX_URI_BIOS_VERSION_OFF: c_int = 12;
pub const QLA82XX_URI_BOOTLD_IDX_OFF: c_int = 27;
pub const QLA82XX_URI_FIRMWARE_IDX_OFF: c_int = 29;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla82xx_uri_table_desc {
    pub findex: __le32,
    pub num_entries: __le32,
    pub entry_size: __le32,
    pub reserved: [__le32; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla82xx_uri_data_desc {
    pub findex: __le32,
    pub size: __le32,
    pub reserved: [__le32; 5],
}

// UNIFIED ROMIMAGE END
pub const QLA82XX_UNIFIED_ROMIMAGE: c_int = 3;
pub const QLA82XX_FLASH_ROMIMAGE: c_int = 4;
pub const QLA82XX_UNKNOWN_ROMIMAGE: c_uint = 0xff;

// Request and response queue size

//
// ISP 8021 I/O Register Set structure definitions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_reg_82xx {
    pub /: *mut *mut *mut __le32 req_q_out[64]; / Request Queue out-Pointer (64  4),
    pub /: *mut *mut __le32 rsp_q_in[64]; / Response Queue In-Pointer.,
    pub /: *mut *mut __le32 rsp_q_out[64]; / Response Queue Out-Pointer.,
    pub /: *mut *mut __le16 mailbox_in[32]; / Mailbox In registers,
    pub unused_1: [__le16; 32],
    pub /: *mut *mut __le32 hint; / Host interrupt register,
    pub unused_2: [__le16; 62],
    pub /: *mut *mut __le16 mailbox_out[32]; / Mailbox Out registers,
    pub unused_3: [__le32; 48],
    pub /: *mut *mut __le32 host_status; / host status,

    pub /: *mut *mut __le32 host_int; / Interrupt status.,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcp_cmnd {
    pub lun: scsi_lun,
    pub crn: u8,
    pub task_attribute: u8,
    pub task_management: u8,
    pub additional_cdb_len: u8,
pub const QLA_CDB_BUF_SIZE: c_int = 256;
pub const QLA_FCP_DL_SIZE: c_int = 4;
    pub /: *mut *mut uint8_t cdb[QLA_CDB_BUF_SIZE + QLA_FCP_DL_SIZE]; / 256 for CDB len and 4 for FCP_DL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsd_dma {
    pub list: list_head,
    pub dsd_list_dma: dma_addr_t,
    pub dsd_addr: *mut c_void,
}

pub const QLA_DSDS_PER_IOCB: c_int = 37;
pub const QLA_DSD_SIZE: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ct6_dsd {
    pub fcp_cmnd_len: u16,
    pub fcp_cmnd_dma: dma_addr_t,
    pub fcp_cmnd: *mut fcp_cmnd,
    pub dsd_use_cnt: c_int,
    pub dsd_list: list_head,
}

pub const MBC_TOGGLE_INTERRUPT: c_uint = 0x10;
pub const MBC_SET_LED_CONFIG: c_uint = 0x125	/* FCoE specific LED control */;
pub const MBC_GET_LED_CONFIG: c_uint = 0x126	/* FCoE specific LED control */;
// Flash  offset
pub const FLT_REG_BOOTLOAD_82XX: c_uint = 0x72;
pub const FLT_REG_BOOT_CODE_82XX: c_uint = 0x78;
pub const FLT_REG_FW_82XX: c_uint = 0x74;
pub const FLT_REG_GOLD_FW_82XX: c_uint = 0x75;
pub const FLT_REG_VPD_8XXX: c_uint = 0x81;
pub const FA_VPD_SIZE_82XX: c_uint = 0x400;
pub const FA_FLASH_LAYOUT_ADDR_82: c_uint = 0xFC400;
pub const FA_FLASH_MCU_OFF: c_uint = 0x13000;
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
// Minidump related
//
// Version of the template
// 4 Bytes
// X.Major.Minor.RELEASE
//
pub const QLA82XX_MINIDUMP_VERSION: c_uint = 0x10101;
//
// Entry Type Defines
//
pub const QLA82XX_RDNOP: c_int = 0;
pub const QLA82XX_RDCRB: c_int = 1;
pub const QLA82XX_RDMUX: c_int = 2;
pub const QLA82XX_QUEUE: c_int = 3;
pub const QLA82XX_BOARD: c_int = 4;
pub const QLA82XX_RDSRE: c_int = 5;
pub const QLA82XX_RDOCM: c_int = 6;
pub const QLA82XX_CACHE: c_int = 10;
pub const QLA82XX_L1DAT: c_int = 11;
pub const QLA82XX_L1INS: c_int = 12;
pub const QLA82XX_L2DTG: c_int = 21;
pub const QLA82XX_L2ITG: c_int = 22;
pub const QLA82XX_L2DAT: c_int = 23;
pub const QLA82XX_L2INS: c_int = 24;
pub const QLA82XX_RDROM: c_int = 71;
pub const QLA82XX_RDMEM: c_int = 72;
pub const QLA82XX_CNTRL: c_int = 98;
pub const QLA82XX_TLHDR: c_int = 99;
pub const QLA82XX_RDEND: c_int = 255;
pub const QLA8044_POLLRD: c_int = 35;
pub const QLA8044_RDMUX2: c_int = 36;
pub const QLA8044_L1DTG: c_int = 8;
pub const QLA8044_L1ITG: c_int = 9;
pub const QLA8044_POLLRDMWR: c_int = 37;
//
// Opcodes for Control Entries.
// These Flags are bit fields.
//
pub const QLA82XX_DBG_OPCODE_WR: c_uint = 0x01;
pub const QLA82XX_DBG_OPCODE_RW: c_uint = 0x02;
pub const QLA82XX_DBG_OPCODE_AND: c_uint = 0x04;
pub const QLA82XX_DBG_OPCODE_OR: c_uint = 0x08;
pub const QLA82XX_DBG_OPCODE_POLL: c_uint = 0x10;
pub const QLA82XX_DBG_OPCODE_RDSTATE: c_uint = 0x20;
pub const QLA82XX_DBG_OPCODE_WRSTATE: c_uint = 0x40;
pub const QLA82XX_DBG_OPCODE_MDSTATE: c_uint = 0x80;
//
// Template Header and Entry Header definitions start here.
//
// Template Header
// Parts of the template header can be modified by the driver.
// These include the saved_state_array, capture_debug_level, driver_timestamp
//
pub const QLA82XX_DBG_STATE_ARRAY_LEN: c_int = 16;
pub const QLA82XX_DBG_CAP_SIZE_ARRAY_LEN: c_int = 8;
pub const QLA82XX_DBG_RSVD_ARRAY_LEN: c_int = 8;
//
// Driver Flags
//
pub const QLA82XX_DBG_SKIPPED_FLAG: c_uint = 0x80	/* driver skipped this entry */;
pub const QLA82XX_DEFAULT_CAP_MASK: c_uint = 0xFF	/* default capture mask */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla82xx_md_template_hdr {
    pub entry_type: u32,
    pub first_entry_offset: u32,
    pub size_of_template: u32,
    pub capture_debug_level: u32,
    pub num_of_entries: u32,
    pub version: u32,
    pub driver_timestamp: u32,
    pub template_checksum: u32,
    pub driver_capture_mask: u32,
    pub driver_info: [u32; 3],
    pub saved_state_array: [u32; QLA82XX_DBG_STATE_ARRAY_LEN],
    pub capture_size_array: [u32; QLA82XX_DBG_CAP_SIZE_ARRAY_LEN],
// markers_array used to capture some special locations on board
    pub markers_array: [u32; QLA82XX_DBG_RSVD_ARRAY_LEN],
    pub /: *mut *mut uint32_t num_of_free_entries; / For internal use,
    pub /: *mut *mut uint32_t free_entry_offset; / For internal use,
    pub /: *mut *mut uint32_t total_table_size; / For internal use,
    pub /: *mut *mut uint32_t bkup_table_offset; / For internal use,
    pub __packed: },
//
// Entry Header:  Common to All Entry Types
//
// Driver Code is for driver to write some info about the entry.
// Currently not used.
//
    pub entry_type: u32,
    pub entry_size: u32,
    pub entry_capture_size: u32,
    pub entry_capture_mask: u8,
    pub entry_code: u8,
    pub driver_code: u8,
    pub driver_flags: u8,
    pub d_ctrl: },
    pub qla82xx_md_entry_hdr_t: } __packed,
//
// Read CRB entry header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla82xx_md_entry_crb {
    pub h: qla82xx_md_entry_hdr_t,
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
    pub __packed: },
//
// Cache entry header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla82xx_md_entry_cache {
    pub h: qla82xx_md_entry_hdr_t,
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
    pub __packed: },
//
// Read OCM
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla82xx_md_entry_rdocm {
    pub h: qla82xx_md_entry_hdr_t,
    pub rsvd_0: u32,
    pub rsvd_1: u32,
    pub data_size: u32,
    pub op_count: u32,
    pub rsvd_2: u32,
    pub rsvd_3: u32,
    pub read_addr: u32,
    pub read_addr_stride: u32,
    pub read_addr_cntrl: u32,
    pub __packed: },
//
// Read Memory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla82xx_md_entry_rdmem {
    pub h: qla82xx_md_entry_hdr_t,
    pub rsvd: [u32; 6],
    pub read_addr: u32,
    pub read_data_size: u32,
    pub __packed: },
//
// Read ROM
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla82xx_md_entry_rdrom {
    pub h: qla82xx_md_entry_hdr_t,
    pub rsvd: [u32; 6],
    pub read_addr: u32,
    pub read_data_size: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla82xx_md_entry_mux {
    pub h: qla82xx_md_entry_hdr_t,
    pub select_addr: u32,
    pub rsvd_0: u32,
    pub data_size: u32,
    pub op_count: u32,
    pub select_value: u32,
    pub select_value_stride: u32,
    pub read_addr: u32,
    pub rsvd_1: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla82xx_md_entry_queue {
    pub h: qla82xx_md_entry_hdr_t,
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
    pub __packed: },
pub const MBC_DIAGNOSTIC_MINIDUMP_TEMPLATE: c_uint = 0x129;
pub const RQST_TMPLT_SIZE: c_uint = 0x0;
pub const RQST_TMPLT: c_uint = 0x1;
pub const MD_DIRECT_ROM_WINDOW: c_uint = 0x42110030;
pub const MD_DIRECT_ROM_READ_BASE: c_uint = 0x42150000;
pub const MD_MIU_TEST_AGT_CTRL: c_uint = 0x41000090;
pub const MD_MIU_TEST_AGT_ADDR_LO: c_uint = 0x41000094;
pub const MD_MIU_TEST_AGT_ADDR_HI: c_uint = 0x41000098;
    pub MD_MIU_TEST_AGT_RDDATA: [extern int; 4],
pub const CRB_NIU_XG_PAUSE_CTL_P0: c_uint = 0x1;
pub const CRB_NIU_XG_PAUSE_CTL_P1: c_uint = 0x8;

//
// Temperature control.
//
}

pub const LEG_INTR_PTR_OFFSET: c_uint = 0x38C0;
pub const LEG_INTR_TRIG_OFFSET: c_uint = 0x38C4;
pub const LEG_INTR_MASK_OFFSET: c_uint = 0x38C8;
