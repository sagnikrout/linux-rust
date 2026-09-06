//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/freescale/enetc/enetc_hw.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
// Copyright 2017-2019 NXP

pub const ENETC_MM_VERIFY_RETRIES: c_int = 3;
pub const ENETC_NUM_TC: c_int = 8;
// ENETC device IDs
pub const ENETC_DEV_ID_PF: c_uint = 0xe100;
pub const ENETC_DEV_ID_VF: c_uint = 0xef00;
pub const ENETC_DEV_ID_PTP: c_uint = 0xee02;
// ENETC register block BAR
pub const ENETC_BAR_REGS: c_int = 0;
// SI regs, offset: 0h
pub const ENETC_SIMR: c_int = 0;

pub const ENETC_SICTR0: c_uint = 0x18;
pub const ENETC_SICTR1: c_uint = 0x1c;
pub const ENETC_SIPCAPR0: c_uint = 0x20;

pub const ENETC_SIPCAPR1: c_uint = 0x24;
pub const ENETC_SITGTGR: c_uint = 0x30;
pub const ENETC_SIRBGCR: c_uint = 0x38;
// cache attribute registers for transactions initiated by ENETC
pub const ENETC_SICAR0: c_uint = 0x40;
pub const ENETC_SICAR1: c_uint = 0x44;
pub const ENETC_SICAR2: c_uint = 0x48;
// rd snoop, no alloc
// wr snoop, no alloc, partial cache line update for BDs and full cache line
// update for data
//
pub const ENETC_SICAR_RD_COHERENT: c_uint = 0x2b2b0000;
pub const ENETC_SICAR_WR_COHERENT: c_uint = 0x00006727;
pub const ENETC_SICAR_MSI: c_uint = 0x00300030 /* rd/wr device, no snoop, no alloc */;
pub const ENETC_SIPMAR0: c_uint = 0x80;
pub const ENETC_SIPMAR1: c_uint = 0x84;
pub const ENETC_SICVLANR1: c_uint = 0x90;
pub const ENETC_SICVLANR2: c_uint = 0x94;

// VF-PF Message passing

// msg size encoding: default and max msg value of 1024B encoded as 0
pub const ENETC_PSIMSGRR: c_uint = 0x204;

// Message received mask, n is the active number of VSIs.
// It is available for ENETC_PSIMSGRR, ENETC_PSIIER, and
// ENETC_PSIIDR registers.
//

// Message received bit, n is VSI index. It is available for
// ENETC_PSIMSGRR, ENETC_PSIIER, and ENETC_PSIIDR registers.
//

pub const ENETC_VSIMSGSR: c_uint = 0x204	/* RO */;

pub const ENETC_VSIMSGSNDAR0: c_uint = 0x210;
pub const ENETC_VSIMSGSNDAR1: c_uint = 0x214;

// SI statistics
pub const ENETC_SIROCT: c_uint = 0x300;
pub const ENETC_SIRFRM: c_uint = 0x308;
pub const ENETC_SIRUCA: c_uint = 0x310;
pub const ENETC_SIRMCA: c_uint = 0x318;
pub const ENETC_SITOCT: c_uint = 0x320;
pub const ENETC_SITFRM: c_uint = 0x328;
pub const ENETC_SITUCA: c_uint = 0x330;
pub const ENETC_SITMCA: c_uint = 0x338;

// Control BDR regs
pub const ENETC_SICBDRMR: c_uint = 0x800;
pub const ENETC_SICBDRSR: c_uint = 0x804	/* RO */;
pub const ENETC_SICBDRBAR0: c_uint = 0x810;
pub const ENETC_SICBDRBAR1: c_uint = 0x814;
pub const ENETC_SICBDRPIR: c_uint = 0x818;
pub const ENETC_SICBDRCIR: c_uint = 0x81c;
pub const ENETC_SICBDRLENR: c_uint = 0x820;
pub const ENETC_SICAPR0: c_uint = 0x900;
pub const ENETC_SICAPR1: c_uint = 0x904;
pub const ENETC_PSIIER: c_uint = 0xa00;
pub const ENETC_PSIIDR: c_uint = 0xa08;
pub const ENETC_SITXIDR: c_uint = 0xa18;
pub const ENETC_SIRXIDR: c_uint = 0xa28;
pub const ENETC_SIMSIVR: c_uint = 0xa30;

pub const ENETC_SIUEFDCR: c_uint = 0xe28;
pub const ENETC_SIRFSCAPR: c_uint = 0x1200;

pub const ENETC_SIRSSCAPR: c_uint = 0x1600;

// SI BDR sub-blocks, n = 0..7
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum enetc_bdr_type {

// RX BDR reg offsets
pub const ENETC_RBMR: c_int = 0;

pub const ENETC_RBSR: c_uint = 0x4;
pub const ENETC_RBBSR: c_uint = 0x8;
pub const ENETC_RBCIR: c_uint = 0xc;
pub const ENETC_RBBAR0: c_uint = 0x10;
pub const ENETC_RBBAR1: c_uint = 0x14;
pub const ENETC_RBPIR: c_uint = 0x18;
pub const ENETC_RBLENR: c_uint = 0x20;
pub const ENETC_RBIER: c_uint = 0xa0;

pub const ENETC_RBIDR: c_uint = 0xa4;
pub const ENETC_RBICR0: c_uint = 0xa8;

pub const ENETC_RBICR0_ICPT_MASK: c_uint = 0x1ff;

pub const ENETC_RBICR1: c_uint = 0xac;

// TX BDR reg offsets
pub const ENETC_TBMR: c_int = 0;

pub const ENETC_TBSR: c_uint = 0x4;
pub const ENETC_TBBAR0: c_uint = 0x10;
pub const ENETC_TBBAR1: c_uint = 0x14;
pub const ENETC_TBPIR: c_uint = 0x18;
pub const ENETC_TBCIR: c_uint = 0x1c;
pub const ENETC_TBCIR_IDX_MASK: c_uint = 0xffff;
pub const ENETC_TBLENR: c_uint = 0x20;
pub const ENETC_TBIER: c_uint = 0xa0;

pub const ENETC_TBIDR: c_uint = 0xa4;
pub const ENETC_TBICR0: c_uint = 0xa8;

pub const ENETC_TBICR0_ICPT_MASK: c_uint = 0xf;

pub const ENETC_TBICR1: c_uint = 0xac;

// Port regs, offset: 1_0000h
pub const ENETC_PORT_BASE: c_uint = 0x10000;
pub const ENETC_PMR: c_uint = 0x0000;

pub const ENETC_PMR_PSPEED_10M: c_int = 0;

pub const ENETC_PSR: c_uint = 0x0004 /* RO */;
pub const ENETC_PSIPMMR: c_uint = 0x0018;

pub const ENETC_PSIPVMR: c_uint = 0x001c;

pub const ENETC_PVCLCTR: c_uint = 0x0208;
pub const ENETC_PCVLANR1: c_uint = 0x0210;
pub const ENETC_PCVLANR2: c_uint = 0x0214;

pub const ENETC_PPAUONTR: c_uint = 0x0410;
pub const ENETC_PPAUOFFTR: c_uint = 0x0414;
pub const ENETC_PTXMBAR: c_uint = 0x0608;
pub const ENETC_PCAPR0: c_uint = 0x0900;

pub const ENETC_PCAPR1: c_uint = 0x0904;

pub const ENETC_RSSHASH_KEY_SIZE: c_int = 40;
pub const ENETC_PRSSCAPR: c_uint = 0x1404;

pub const ENETC_PSIVLANFMR: c_uint = 0x1700;

pub const ENETC_PRFSMR: c_uint = 0x1800;

pub const ENETC_PRFSCAPR: c_uint = 0x1804;

pub const ENETC_PFPMR: c_uint = 0x1900;

pub const ENETC_EMDIO_BASE: c_uint = 0x1c00;

pub const ENETC_MMCSR: c_uint = 0x1f00;

pub const ENETC_MMFAECR: c_uint = 0x1f08;
pub const ENETC_MMFSECR: c_uint = 0x1f0c;
pub const ENETC_MMFAOCR: c_uint = 0x1f10;
pub const ENETC_MMFCRXR: c_uint = 0x1f14;
pub const ENETC_MMFCTXR: c_uint = 0x1f18;
pub const ENETC_MMHCR: c_uint = 0x1f1c;

pub const ENETC_PMAC_OFFSET: c_uint = 0x1000;

pub const ENETC_PM0_CMD_CFG: c_uint = 0x8008;

pub const ENETC_PM0_MAXFRM: c_uint = 0x8014;

pub const ENETC_PM0_RX_FIFO: c_uint = 0x801c;
pub const ENETC_PM0_RX_FIFO_VAL: c_int = 1;

pub const ENETC_PM_IMDIO_BASE: c_uint = 0x8030;

pub const ENETC_PM0_PAUSE_QUANTA: c_uint = 0x8054;
pub const ENETC_PM0_PAUSE_THRESH: c_uint = 0x8064;

pub const ENETC_PM0_SINGLE_STEP: c_uint = 0x80c0;

pub const ENETC_PM0_IF_MODE: c_uint = 0x8300;

pub const ENETC_PM0_IFM_IFMODE_XGMII: c_int = 0;
pub const ENETC_PM0_IFM_IFMODE_GMII: c_int = 2;
pub const ENETC_PSIDCAPR: c_uint = 0x1b08;

pub const ENETC_PSFCAPR: c_uint = 0x1b18;

pub const ENETC_PSGCAPR: c_uint = 0x1b28;

pub const ENETC_PFMCAPR: c_uint = 0x1b38;

// Port MAC counters: Port MAC 0 corresponds to the eMAC and
// Port MAC 1 to the pMAC.
//

// Port counters

pub const ENETC_PBFDSIR: c_uint = 0x0810;
pub const ENETC_PFDMSAPR: c_uint = 0x0814;
pub const ENETC_UFDMF: c_uint = 0x1680;
pub const ENETC_MFDMF: c_uint = 0x1684;
pub const ENETC_PUFDVFR: c_uint = 0x1780;
pub const ENETC_PMFDVFR: c_uint = 0x1784;
pub const ENETC_PBFDVFR: c_uint = 0x1788;

// Global regs, offset: 2_0000h
pub const ENETC_GLOBAL_BASE: c_uint = 0x20000;
pub const ENETC_G_EIPBRR0: c_uint = 0x0bf8;

pub const ENETC_REV_1_0: c_uint = 0x0100;

pub const ENETC_REV_4_3: c_uint = 0x0403;

pub const ENETC_G_EIPBRR1: c_uint = 0x0bfc;

pub const ENETC_G_EPFBLPR1_XGMII: c_uint = 0x80000000;

// PCI device info
    struct enetc_hw {
// SI registers, used by all PCI functions
    void __iomem *reg;
// Port registers, PF only
    void __iomem *port;
// IP global registers, PF only
    void __iomem *global;
}

// ENETC register accessors
// MDIO issue workaround (on LS1028A) -
// Due to a hardware issue, an access to MDIO registers
// that is concurrent with other ENETC register accesses
// may lead to the MDIO access being dropped or corrupted.
// To protect the MDIO accesses a readers-writers locking
// scheme is used, where the MDIO register accesses are
// protected by write locks to insure exclusivity, while
// the remaining ENETC registers are accessed under read
// locks since they only compete with MDIO accesses.
//
// use this locking primitive only on the fast datapath to
// group together multiple non-MDIO register accesses to
// minimize the overhead of the lock
//
// use these accessors only on the fast datapath under
// the enetc_lock_mdio() locking primitive to minimize
// the overhead of the lock
//
extern "C" {
    pub fn ioread32(_arg: reg) -> return;
}
// internal helpers for the MDIO w/a

extern "C" {
    pub fn ioread64(_arg: reg) -> return;
}

// using this to read out stats on 32b systems

// general register accessors

// port register accessors - PF only

// global register accessors - PF only

// BDR register accessors, see ENETC_BDR()

// Buffer Descriptors (BD)
#[repr(C)]
#[derive(Copy, Clone)]
pub union enetc_tx_bd {
    pub addr: __le64,
    pub buf_len: __le16,
    pub /: *mut *mut __le16 hdr_len; / For LSO, ENETC 4.1 and later,
}

pub const ENETC_TXBD_L4T_UDP: c_int = 1;
pub const ENETC_TXBD_L4T_TCP: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum enetc_txbd_flags {
    ENETC_TXBD_FLAGS_L4CS = BIT(0), /* For ENETC 4.1 and later */
    ENETC_TXBD_FLAGS_TSE = BIT(1),
    ENETC_TXBD_FLAGS_LSO = BIT(1), /* For ENETC 4.1 and later */
    ENETC_TXBD_FLAGS_W = BIT(2),
    ENETC_TXBD_FLAGS_CSUM_LSO = BIT(3), /* For ENETC 4.1 and later */
    ENETC_TXBD_FLAGS_TXSTART = BIT(4),
    ENETC_TXBD_FLAGS_EX = BIT(6),
    ENETC_TXBD_FLAGS_F = BIT(7)
}

pub const ENETC_TXBD_FLAGS_OFFSET: c_int = 24;

extern "C" {
    pub fn cpu_to_le32(_arg: temp) -> return;
}
// Extension flags

#[repr(C)]
#[derive(Copy, Clone)]
pub union enetc_rx_bd {
    pub addr: __le64,
    pub reserved: [u8; 8],
    pub w: },
    pub inet_csum: __le16,
    pub parse_summary: __le16,
    pub rss_hash: __le32,
    pub buf_len: __le16,
    pub vlan_opt: __le16,
    pub flags: __le16,
    pub error: __le16,
}

pub const ENETC_RXBD_ERR_MASK: c_uint = 0xff;

pub const ENETC_CBD_STATUS_MASK: c_uint = 0xf;
pub const ENETC_TPID_8021Q: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct enetc_cmd_rfse {
    pub smac_h: [u8; 6],
    pub smac_m: [u8; 6],
    pub dmac_h: [u8; 6],
    pub dmac_m: [u8; 6],
    pub sip_h: [__be32; 4],
    pub sip_m: [__be32; 4],
    pub dip_h: [__be32; 4],
    pub dip_m: [__be32; 4],
    pub ethtype_h: u16,
    pub ethtype_m: u16,
    pub ethtype4_h: u16,
    pub ethtype4_m: u16,
    pub sport_h: u16,
    pub sport_m: u16,
    pub dport_h: u16,
    pub dport_m: u16,
    pub vlan_h: u16,
    pub vlan_m: u16,
    pub proto_h: u8,
    pub proto_m: u8,
    pub flags: u16,
    pub result: u16,
    pub mode: u16,
}

pub const ENETC_RFSE_MODE_BD: c_int = 2;
pub const ENETC_SI_INT_IDX: c_int = 0;
// base index for Rx/Tx interrupts
pub const ENETC_BDR_INT_BASE_IDX: c_int = 1;
// Messaging
// Command completion status
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum enetc_msg_cmd_status {
    ENETC_MSG_CMD_STATUS_OK,
    ENETC_MSG_CMD_STATUS_FAIL
}

// VSI-PSI command message types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum enetc_msg_cmd_type {
    ENETC_MSG_CMD_MNG_MAC = 1, /* manage MAC address */
    ENETC_MSG_CMD_MNG_RX_MAC_FILTER,/* manage RX MAC table */
    ENETC_MSG_CMD_MNG_RX_VLAN_FILTER /* manage RX VLAN table */
}

// VSI-PSI command action types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum enetc_msg_cmd_action_type {
    ENETC_MSG_CMD_MNG_ADD = 1,
    ENETC_MSG_CMD_MNG_REMOVE
}

// PSI-VSI command header format
#[repr(C)]
#[derive(Copy, Clone)]
pub struct enetc_msg_cmd_header {
    pub /: *mut *mut u16 type; / command class type,
    pub /: *mut *mut u16 id; / denotes the specific required action,
}

// Common H/W utility functions
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bdcr_cmd_class {
    BDCR_CMD_UNSPEC = 0,
    BDCR_CMD_MAC_FILTER,
    BDCR_CMD_VLAN_FILTER,
    BDCR_CMD_RSS,
    BDCR_CMD_RFS,
    BDCR_CMD_PORT_GCL,
    BDCR_CMD_RECV_CLASSIFIER,
    BDCR_CMD_STREAM_IDENTIFY,
    BDCR_CMD_STREAM_FILTER,
    BDCR_CMD_STREAM_GCL,
    BDCR_CMD_FLOW_METER,
    __BDCR_CMD_MAX_LEN,
    BDCR_CMD_MAX_LEN = __BDCR_CMD_MAX_LEN - 1,
}

// class 5, command 0
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tgs_gcl_conf {
    pub /: *mut *mut u8 atc; / init gate value,
    pub res: [u8; 7],
    pub res1: [u8; 4],
    pub acl_len: __le16,
    pub res2: [u8; 2],
}

// gate control list entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gce {
    pub period: __le32,
    pub gate: u8,
    pub res: [u8; 3],
}

// tgs_gcl_conf address point to this data space
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tgs_gcl_data {
    pub btl: __le32,
    pub bth: __le32,
    pub ct: __le32,
    pub cte: __le32,
    pub entry: [gce; ],
}

// class 7, command 0, Stream Identity Entry Configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct streamid_conf {
    pub /: *mut *mut __le32 stream_handle; / init gate value,
    pub iports: __le32,
    pub id_type: u8,
    pub oui: [u8; 3],
    pub res: [u8; 3],
    pub en: u8,
}

pub const ENETC_CBDR_SID_VID_MASK: c_uint = 0xfff;

pub const ENETC_CBDR_SID_TG_MASK: c_uint = 0xc000;
// streamid_conf address point to this data space
#[repr(C)]
#[derive(Copy, Clone)]
pub struct streamid_data {
    pub dmac: [u8; 6],
    pub smac: [u8; 6],
}

pub const ENETC_CBDR_SFI_PRI_MASK: c_uint = 0x7;

// class 8, command 0, Stream Filter Instance, Short Format
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfi_conf {
    pub stream_handle: __le32,
    pub multi: u8,
    pub res: [u8; 2],
    pub sthm: u8,
// Max Service Data Unit or Flow Meter Instance Table index.
// Depending on the value of FLT this represents either Max
// Service Data Unit (max frame size) allowed by the filter
// entry or is an index into the Flow Meter Instance table
// index identifying the policer which will be used to police
// it.
//
    pub fm_inst_table_index: __le16,
    pub msdu: __le16,
    pub sg_inst_table_index: __le16,
    pub res1: [u8; 2],
    pub input_ports: __le32,
    pub res2: [u8; 3],
    pub en: u8,
}

// class 8, command 2 stream Filter Instance status query short format
// command no need structure define
// Stream Filter Instance Query Statistics Response data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfi_counter_data {
    pub matchl: u32,
    pub matchh: u32,
    pub msdu_dropl: u32,
    pub msdu_droph: u32,
    pub stream_gate_dropl: u32,
    pub stream_gate_droph: u32,
    pub flow_meter_dropl: u32,
    pub flow_meter_droph: u32,
}

pub const ENETC_CBDR_SGI_OIPV_MASK: c_uint = 0x7;

pub const ENETC_CBDR_SGI_ACLLEN_MASK: c_uint = 0x3;
pub const ENETC_CBDR_SGI_OCLLEN_MASK: c_uint = 0xc;

// class 9, command 0, Stream Gate Instance Table, Short Format
// class 9, command 2, Stream Gate Instance Table entry query write back
// Short Format
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgi_table {
    pub res: [u8; 8],
    pub oipv: u8,
    pub res0: [u8; 2],
    pub ocgtst: u8,
    pub res1: [u8; 7],
    pub gset: u8,
    pub oacl_len: u8,
    pub res2: [u8; 2],
    pub en: u8,
}

pub const ENETC_CBDR_SGI_AIPV_MASK: c_uint = 0x7;

// class 9, command 1, Stream Gate Control List, Long Format
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgcl_conf {
    pub aipv: u8,
    pub res: [u8; 2],
    pub agtst: u8,
    pub res1: [u8; 4],
    pub res2: [u8; 4],
    pub acl_len: u8,
    pub res3: [u8; 3],
}

pub const ENETC_CBDR_SGL_IPV_MASK: c_uint = 0xe;
// Stream Gate Control List Entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgce {
    pub interval: u32,
    pub msdu: [u8; 3],
    pub multi: u8,
}

// stream control list class 9 , cmd 1 data buffer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgcl_data {
    pub btl: u32,
    pub bth: u32,
    pub ct: u32,
    pub cte: u32,
    pub sgcl: [sgce; ],
}

// class 10: command 0/1, Flow Meter Instance Set, short Format
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fmi_conf {
    pub cir: __le32,
    pub cbs: __le32,
    pub eir: __le32,
    pub ebs: __le32,
    pub conf: u8,
    pub res1: u8,
    pub ir_fpp: u8,
    pub res2: [u8; 4],
    pub en: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enetc_cbd {
    pub sfi_conf: sfi_conf,
    pub sgi_table: sgi_table,
    pub fmi_conf: fmi_conf,
    pub addr: [__le32; 2],
    pub opt: [__le32; 4],
    pub gcl_conf: tgs_gcl_conf,
    pub sid_set: streamid_conf,
    pub sgcl_conf: sgcl_conf,
}

// Port traffic class frame preemption register

// port time gating control register
pub const ENETC_PTGCR: c_uint = 0x11a00;

// Port time gating capability register
pub const ENETC_PTGCAPR: c_uint = 0x11a08;

// Port time specific departure

// PSFP setting
pub const ENETC_PPSFPMR: c_uint = 0x11b00;

