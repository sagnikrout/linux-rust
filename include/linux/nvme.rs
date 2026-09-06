//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/nvme.h
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
// Definitions for the NVM Express interface
// Copyright (c) 2011-2014, Intel Corporation.
//

// NQN names in commands fields specified one size
pub const NVMF_NQN_FIELD_LEN: c_int = 256;
// However the max length of a qualified name is another size
pub const NVMF_NQN_SIZE: c_int = 223;
pub const NVMF_TRSVCID_SIZE: c_int = 32;
pub const NVMF_TRADDR_SIZE: c_int = 256;
pub const NVMF_TSAS_SIZE: c_int = 256;

pub const NVME_NSID_ALL: c_uint = 0xffffffff;
// Special NSSR value, 'NVMe'
pub const NVME_SUBSYS_RESET: c_uint = 0x4E564D65;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvme_subsys_type {
// Referral to another discovery type target subsystem
    NVME_NQN_DISC	= 1,

// NVME type target subsystem
    NVME_NQN_NVME	= 2,

// Current discovery type target subsystem
    NVME_NQN_CURR	= 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvme_ctrl_type {
    NVME_CTRL_IO	= 1,		/* I/O controller */
    NVME_CTRL_DISC	= 2,		/* Discovery controller */
    NVME_CTRL_ADMIN	= 3,		/* Administrative controller */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvme_dctype {
    NVME_DCTYPE_NOT_REPORTED	= 0,
    NVME_DCTYPE_DDC			= 1, /* Direct Discovery Controller */
    NVME_DCTYPE_CDC			= 2, /* Central Discovery Controller */
}

// Address Family codes for Discovery Log Page entry ADRFAM field
// Transport Type codes for Discovery Log Page entry TRTYPE field
// Transport Requirements codes for Discovery Log Page entry TREQ field

// RDMA QP Service Type codes for Discovery Log Page entry TSAS
// RDMA_QPTYPE field
//
// RDMA Provider Type codes for Discovery Log Page entry TSAS
// RDMA_PRTYPE field
//
// RDMA Connection Management Service Type codes for Discovery Log Page
// entry TSAS RDMA_CMS field
//
// TSAS SECTYPE for TCP transport
pub const NVME_AQ_DEPTH: c_int = 32;
pub const NVME_NR_AEN_COMMANDS: c_int = 1;

//
// Subtract one to leave an empty queue entry for 'Full Queue' condition. See
// NVM-Express 1.2 specification, section 4.1.2.
//

// Location
//
// Space Control
//
// Buffer Size
//
// Write Throughput
//

//
// Submission and Completion Queue Entry Sizes for the NVM command set.
// (In bytes and specified as a power of two (2^n)).
//
pub const NVME_ADM_SQES: c_int = 6;
pub const NVME_NVM_IOSQES: c_int = 6;
pub const NVME_NVM_IOCQES: c_int = 4;
//
// Controller Configuration (CC) register (Offset 14h)
//
// Enable (EN): bit 0
// Bits 03:01 are reserved (NVMe Base Specification rev 2.1)
// I/O Command Set Selected (CSS): bits 06:04
// Memory Page Size (MPS): bits 10:07
// Arbitration Mechanism Selected (AMS): bits 13:11
// Shutdown Notification (SHN): bits 15:14
// I/O Submission Queue Entry Size (IOSQES): bits 19:16
// I/O Completion Queue Entry Size (IOCQES): bits 23:20
// Controller Ready Independent of Media Enable (CRIME): bit 24
// Bits 25:31 are reserved (NVMe Base Specification rev 2.1)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_id_power_state {
    pub /: *mut *mut __le16 max_power; / centiwatts,
    pub rsvd2: __u8,
    pub flags: __u8,
    pub /: *mut *mut __le32 entry_lat; / microseconds,
    pub /: *mut *mut __le32 exit_lat; / microseconds,
    pub read_tput: __u8,
    pub read_lat: __u8,
    pub write_tput: __u8,
    pub write_lat: __u8,
    pub idle_power: __le16,
    pub idle_scale: __u8,
    pub rsvd19: __u8,
    pub active_power: __le16,
    pub active_work_scale: __u8,
    pub rsvd23: [__u8; 9],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvme_ctrl_attr {
    NVME_CTRL_ATTR_HID_128_BIT	= (1 << 0),
    NVME_CTRL_ATTR_TBKAS		= (1 << 6),
    NVME_CTRL_ATTR_ELBAS		= (1 << 15),
    NVME_CTRL_ATTR_RHII		= (1 << 18),
    NVME_CTRL_ATTR_FDPS		= (1 << 19),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_id_ctrl {
    pub vid: __le16,
    pub ssvid: __le16,
    pub sn: [c_char; 20],
    pub mn: [c_char; 40],
    pub fr: [c_char; 8],
    pub rab: __u8,
    pub ieee: [__u8; 3],
    pub cmic: __u8,
    pub mdts: __u8,
    pub cntlid: __le16,
    pub ver: __le32,
    pub rtd3r: __le32,
    pub rtd3e: __le32,
    pub oaes: __le32,
    pub ctratt: __le32,
    pub rsvd100: [__u8; 11],
    pub cntrltype: __u8,
    pub fguid: [__u8; 16],
    pub crdt1: __le16,
    pub crdt2: __le16,
    pub crdt3: __le16,
    pub rsvd134: [__u8; 122],
    pub oacs: __le16,
    pub acl: __u8,
    pub aerl: __u8,
    pub frmw: __u8,
    pub lpa: __u8,
    pub elpe: __u8,
    pub npss: __u8,
    pub avscc: __u8,
    pub apsta: __u8,
    pub wctemp: __le16,
    pub cctemp: __le16,
    pub mtfa: __le16,
    pub hmpre: __le32,
    pub hmmin: __le32,
    pub tnvmcap: [__u8; 16],
    pub unvmcap: [__u8; 16],
    pub rpmbs: __le32,
    pub edstt: __le16,
    pub dsto: __u8,
    pub fwug: __u8,
    pub kas: __le16,
    pub hctma: __le16,
    pub mntmt: __le16,
    pub mxtmt: __le16,
    pub sanicap: __le32,
    pub hmminds: __le32,
    pub hmmaxd: __le16,
    pub nvmsetidmax: __le16,
    pub endgidmax: __le16,
    pub anatt: __u8,
    pub anacap: __u8,
    pub anagrpmax: __le32,
    pub nanagrpid: __le32,
    pub rsvd352: [__u8; 160],
    pub sqes: __u8,
    pub cqes: __u8,
    pub maxcmd: __le16,
    pub nn: __le32,
    pub oncs: __le16,
    pub fuses: __le16,
    pub fna: __u8,
    pub vwc: __u8,
    pub awun: __le16,
    pub awupf: __le16,
    pub nvscc: __u8,
    pub nwpc: __u8,
    pub acwu: __le16,
    pub rsvd534: [__u8; 2],
    pub sgls: __le32,
    pub mnan: __le32,
    pub rsvd544: [__u8; 224],
    pub subnqn: [c_char; 256],
    pub rsvd1024: [__u8; 768],
    pub ioccsz: __le32,
    pub iorcsz: __le32,
    pub icdoff: __le16,
    pub ctrattr: __u8,
    pub msdbd: __u8,
    pub rsvd1804: [__u8; 2],
    pub dctype: __u8,
    pub rsvd1807: [__u8; 241],
    pub psd: [nvme_id_power_state; 32],
    pub vs: [__u8; 1024],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_lbaf {
    pub ms: __le16,
    pub ds: __u8,
    pub rp: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_id_ns {
    pub nsze: __le64,
    pub ncap: __le64,
    pub nuse: __le64,
    pub nsfeat: __u8,
    pub nlbaf: __u8,
    pub flbas: __u8,
    pub mc: __u8,
    pub dpc: __u8,
    pub dps: __u8,
    pub nmic: __u8,
    pub rescap: __u8,
    pub fpi: __u8,
    pub dlfeat: __u8,
    pub nawun: __le16,
    pub nawupf: __le16,
    pub nacwu: __le16,
    pub nabsn: __le16,
    pub nabo: __le16,
    pub nabspf: __le16,
    pub noiob: __le16,
    pub nvmcap: [__u8; 16],
    pub npwg: __le16,
    pub npwa: __le16,
    pub npdg: __le16,
    pub npda: __le16,
    pub nows: __le16,
    pub rsvd74: [__u8; 18],
    pub anagrpid: __le32,
    pub rsvd96: [__u8; 3],
    pub nsattr: __u8,
    pub nvmsetid: __le16,
    pub endgid: __le16,
    pub nguid: [__u8; 16],
    pub eui64: [__u8; 8],
    pub lbaf: [nvme_lbaf; 64],
    pub vs: [__u8; 3712],
}

// I/O Command Set Independent Identify Namespace Data Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_id_ns_cs_indep {
    pub nsfeat: __u8,
    pub nmic: __u8,
    pub rescap: __u8,
    pub fpi: __u8,
    pub anagrpid: __le32,
    pub nsattr: __u8,
    pub rsvd9: __u8,
    pub nvmsetid: __le16,
    pub endgid: __le16,
    pub nstat: __u8,
    pub rsvd15: [__u8; 4081],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_zns_lbafe {
    pub zsze: __le64,
    pub zdes: __u8,
    pub rsvd9: [__u8; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_id_ns_zns {
    pub zoc: __le16,
    pub ozcs: __le16,
    pub mar: __le32,
    pub mor: __le32,
    pub rrl: __le32,
    pub frl: __le32,
    pub rsvd20: [__u8; 2796],
    pub lbafe: [nvme_zns_lbafe; 64],
    pub vs: [__u8; 256],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_id_ctrl_zns {
    pub zasl: __u8,
    pub rsvd1: [__u8; 4095],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_id_ns_nvm {
    pub lbstm: __le64,
    pub pic: __u8,
    pub rsvd9: [__u8; 3],
    pub elbaf: [__le32; 64],
    pub npdgl: __le32,
    pub nprg: __le32,
    pub npra: __le32,
    pub nors: __le32,
    pub npdal: __le32,
    pub rsvd288: [__u8; 3808],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_id_ctrl_nvm {
    pub vsl: __u8,
    pub wzsl: __u8,
    pub wusl: __u8,
    pub dmrl: __u8,
    pub dmrsl: __le32,
    pub dmsl: __le64,
    pub rsvd16: [__u8; 4080],
}

// In NVMe version 2.0 and below, OPTPERF is only bit 4 of NSFEAT
// Since version 2.1, OPTPERF is bits 4 and 5 of NSFEAT
// Identify Namespace Metadata Capabilities (MC):
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_ns_id_desc {
    pub nidt: __u8,
    pub nidl: __u8,
    pub reserved: __le16,
}

pub const NVME_NIDT_EUI64_LEN: c_int = 8;
pub const NVME_NIDT_NGUID_LEN: c_int = 16;
pub const NVME_NIDT_UUID_LEN: c_int = 16;
pub const NVME_NIDT_CSI_LEN: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_endurance_group_log {
    pub egcw: __u8,
    pub egfeat: __u8,
    pub rsvd2: __u8,
    pub avsp: __u8,
    pub avspt: __u8,
    pub pused: __u8,
    pub did: __le16,
    pub rsvd8: [__u8; 24],
    pub ee: [__u8; 16],
    pub dur: [__u8; 16],
    pub duw: [__u8; 16],
    pub muw: [__u8; 16],
    pub hrc: [__u8; 16],
    pub hwc: [__u8; 16],
    pub mdie: [__u8; 16],
    pub neile: [__u8; 16],
    pub tegcap: [__u8; 16],
    pub uegcap: [__u8; 16],
    pub rsvd192: [__u8; 320],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_rotational_media_log {
    pub endgid: __le16,
    pub numa: __le16,
    pub nrs: __le16,
    pub rsvd6: [__u8; 2],
    pub spinc: __le32,
    pub fspinc: __le32,
    pub ldc: __le32,
    pub fldc: __le32,
    pub rsvd24: [__u8; 488],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_fdp_config {
    pub flags: __u8,

    pub fdpcidx: __u8,
    pub reserved: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_fdp_ruh_desc {
    pub ruht: __u8,
    pub reserved: [__u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_fdp_config_desc {
    pub dsze: __le16,
    pub fdpa: __u8,
    pub vss: __u8,
    pub nrg: __le32,
    pub nruh: __le16,
    pub maxpids: __le16,
    pub nns: __le32,
    pub runs: __le64,
    pub erutl: __le32,
    pub rsvd28: [__u8; 36],
    pub ruhs: [nvme_fdp_ruh_desc; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_fdp_config_log {
    pub numfdpc: __le16,
    pub ver: __u8,
    pub rsvd3: __u8,
    pub sze: __le32,
    pub rsvd8: [__u8; 8],
//
// This is followed by variable number of nvme_fdp_config_desc
// structures, but sparse doesn't like nested variable sized arrays.
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_smart_log {
    pub critical_warning: __u8,
    pub temperature: [__u8; 2],
    pub avail_spare: __u8,
    pub spare_thresh: __u8,
    pub percent_used: __u8,
    pub endu_grp_crit_warn_sumry: __u8,
    pub rsvd7: [__u8; 25],
    pub data_units_read: [__u8; 16],
    pub data_units_written: [__u8; 16],
    pub host_reads: [__u8; 16],
    pub host_writes: [__u8; 16],
    pub ctrl_busy_time: [__u8; 16],
    pub power_cycles: [__u8; 16],
    pub power_on_hours: [__u8; 16],
    pub unsafe_shutdowns: [__u8; 16],
    pub media_errors: [__u8; 16],
    pub num_err_log_entries: [__u8; 16],
    pub warning_temp_time: __le32,
    pub critical_comp_time: __le32,
    pub temp_sensor: [__le16; 8],
    pub thm_temp1_trans_count: __le32,
    pub thm_temp2_trans_count: __le32,
    pub thm_temp1_total_time: __le32,
    pub thm_temp2_total_time: __le32,
    pub rsvd232: [__u8; 280],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_fw_slot_info_log {
    pub afi: __u8,
    pub rsvd1: [__u8; 7],
    pub frs: [__le64; 7],
    pub rsvd64: [__u8; 448],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_effects_log {
    pub acs: [__le32; 256],
    pub iocs: [__le32; 256],
    pub resv: [__u8; 2048],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvme_ana_state {
    NVME_ANA_OPTIMIZED		= 0x01,
    NVME_ANA_NONOPTIMIZED		= 0x02,
    NVME_ANA_INACCESSIBLE		= 0x03,
    NVME_ANA_PERSISTENT_LOSS	= 0x04,
    NVME_ANA_CHANGE			= 0x0f,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_ana_group_desc {
    pub grpid: __le32,
    pub nnsids: __le32,
    pub chgcnt: __le64,
    pub state: __u8,
    pub rsvd17: [__u8; 15],
    pub nsids: [__le32; ],
}

// flag for the log specific field of the ANA log

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_ana_rsp_hdr {
    pub chgcnt: __le64,
    pub ngrps: __le16,
    pub rsvd10: [__le16; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_zone_descriptor {
    pub zt: __u8,
    pub zs: __u8,
    pub za: __u8,
    pub rsvd3: [__u8; 5],
    pub zcap: __le64,
    pub zslba: __le64,
    pub wp: __le64,
    pub rsvd32: [__u8; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_zone_report {
    pub nr_zones: __le64,
    pub resv8: [__u8; 56],
    pub entries: [nvme_zone_descriptor; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_lba_range_type {
    pub type: __u8,
    pub attributes: __u8,
    pub rsvd2: [__u8; 14],
    pub slba: __le64,
    pub nlb: __le64,
    pub guid: [__u8; 16],
    pub rsvd48: [__u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvme_pr_type {
    NVME_PR_WRITE_EXCLUSIVE			= 1,
    NVME_PR_EXCLUSIVE_ACCESS		= 2,
    NVME_PR_WRITE_EXCLUSIVE_REG_ONLY	= 3,
    NVME_PR_EXCLUSIVE_ACCESS_REG_ONLY	= 4,
    NVME_PR_WRITE_EXCLUSIVE_ALL_REGS	= 5,
    NVME_PR_EXCLUSIVE_ACCESS_ALL_REGS	= 6,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvme_eds {
    NVME_EXTENDED_DATA_STRUCT	= 0x1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_registered_ctrl {
    pub cntlid: __le16,
    pub rcsts: __u8,
    pub rsvd3: [__u8; 5],
    pub hostid: __le64,
    pub rkey: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_reservation_status {
    pub gen: __le32,
    pub rtype: __u8,
    pub regctl: [__u8; 2],
    pub resv5: [__u8; 2],
    pub ptpls: __u8,
    pub resv10: [__u8; 14],
    pub regctl_ds: [nvme_registered_ctrl; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_registered_ctrl_ext {
    pub cntlid: __le16,
    pub rcsts: __u8,
    pub rsvd3: [__u8; 5],
    pub rkey: __le64,
    pub hostid: [__u8; 16],
    pub rsvd32: [__u8; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_reservation_status_ext {
    pub gen: __le32,
    pub rtype: __u8,
    pub regctl: [__u8; 2],
    pub resv5: [__u8; 2],
    pub ptpls: __u8,
    pub resv10: [__u8; 14],
    pub rsvd24: [__u8; 40],
    pub regctl_eds: [nvme_registered_ctrl_ext; ],
}

// I/O commands
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvme_opcode {
    nvme_cmd_flush		= 0x00,
    nvme_cmd_write		= 0x01,
    nvme_cmd_read		= 0x02,
    nvme_cmd_write_uncor	= 0x04,
    nvme_cmd_compare	= 0x05,
    nvme_cmd_write_zeroes	= 0x08,
    nvme_cmd_dsm		= 0x09,
    nvme_cmd_verify		= 0x0c,
    nvme_cmd_resv_register	= 0x0d,
    nvme_cmd_resv_report	= 0x0e,
    nvme_cmd_resv_acquire	= 0x11,
    nvme_cmd_io_mgmt_recv	= 0x12,
    nvme_cmd_resv_release	= 0x15,
    nvme_cmd_zone_mgmt_send	= 0x79,
    nvme_cmd_zone_mgmt_recv	= 0x7a,
    nvme_cmd_zone_append	= 0x7d,
    nvme_cmd_vendor_start	= 0x80,
}

//
// Descriptor subtype - lower 4 bits of nvme_(keyed_)sgl_desc identifier
//
// @NVME_SGL_FMT_ADDRESS:     absolute address of the data block
// @NVME_SGL_FMT_OFFSET:      relative offset of the in-capsule data block
// @NVME_SGL_FMT_TRANSPORT_A: transport defined format, value 0xA
// @NVME_SGL_FMT_INVALIDATE:  RDMA transport specific remote invalidation
// request subtype
//
// Descriptor type - upper 4 bits of nvme_(keyed_)sgl_desc identifier
//
// For struct nvme_sgl_desc:
// @NVME_SGL_FMT_DATA_DESC:		data block descriptor
// @NVME_SGL_FMT_SEG_DESC:		sgl segment descriptor
// @NVME_SGL_FMT_LAST_SEG_DESC:	last sgl segment descriptor
//
// For struct nvme_keyed_sgl_desc:
// @NVME_KEY_SGL_FMT_DATA_DESC:	keyed data block descriptor
//
// Transport-specific SGL types:
// @NVME_TRANSPORT_SGL_DATA_DESC:	Transport SGL data dlock descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_sgl_desc {
    pub addr: __le64,
    pub length: __le32,
    pub rsvd: [__u8; 3],
    pub type: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_keyed_sgl_desc {
    pub addr: __le64,
    pub length: [__u8; 3],
    pub key: [__u8; 4],
    pub type: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nvme_data_ptr {
    pub prp1: __le64,
    pub prp2: __le64,
}

//
// Lowest two bits of our flags field (FUSE field in the spec):
//
// @NVME_CMD_FUSE_FIRST:   Fused Operation, first command
// @NVME_CMD_FUSE_SECOND:  Fused Operation, second command
//
// Highest two bits in our flags field (PSDT field in the spec):
//
// @NVME_CMD_PSDT_SGL_METABUF:	Use SGLS for this transfer,
// If used, MPTR contains addr of single physical buffer (byte aligned).
// @NVME_CMD_PSDT_SGL_METASEG:	Use SGLS for this transfer,
// If used, MPTR contains an address of an SGL segment containing
// exactly 1 SGL descriptor (qword aligned).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_common_command {
    pub opcode: __u8,
    pub flags: __u8,
    pub command_id: __u16,
    pub nsid: __le32,
    pub cdw2: [__le32; 2],
    pub metadata: __le64,
    pub dptr: nvme_data_ptr,
    pub cdw10: __le32,
    pub cdw11: __le32,
    pub cdw12: __le32,
    pub cdw13: __le32,
    pub cdw14: __le32,
    pub cdw15: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_rw_command {
    pub opcode: __u8,
    pub flags: __u8,
    pub command_id: __u16,
    pub nsid: __le32,
    pub cdw2: __le32,
    pub cdw3: __le32,
    pub metadata: __le64,
    pub dptr: nvme_data_ptr,
    pub slba: __le64,
    pub length: __le16,
    pub control: __le16,
    pub dsmgmt: __le32,
    pub reftag: __le32,
    pub lbat: __le16,
    pub lbatm: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_dsm_cmd {
    pub opcode: __u8,
    pub flags: __u8,
    pub command_id: __u16,
    pub nsid: __le32,
    pub rsvd2: [__u64; 2],
    pub dptr: nvme_data_ptr,
    pub nr: __le32,
    pub attributes: __le32,
    pub rsvd12: [__u32; 4],
}

pub const NVME_DSM_MAX_RANGES: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_dsm_range {
    pub cattr: __le32,
    pub nlb: __le32,
    pub slba: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_write_zeroes_cmd {
    pub opcode: __u8,
    pub flags: __u8,
    pub command_id: __u16,
    pub nsid: __le32,
    pub rsvd2: __u64,
    pub metadata: __le64,
    pub dptr: nvme_data_ptr,
    pub slba: __le64,
    pub length: __le16,
    pub control: __le16,
    pub dsmgmt: __le32,
    pub reftag: __le32,
    pub lbat: __le16,
    pub lbatm: __le16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvme_zone_mgmt_action {
    NVME_ZONE_CLOSE		= 0x1,
    NVME_ZONE_FINISH	= 0x2,
    NVME_ZONE_OPEN		= 0x3,
    NVME_ZONE_RESET		= 0x4,
    NVME_ZONE_OFFLINE	= 0x5,
    NVME_ZONE_SET_DESC_EXT	= 0x10,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_zone_mgmt_send_cmd {
    pub opcode: __u8,
    pub flags: __u8,
    pub command_id: __u16,
    pub nsid: __le32,
    pub cdw2: [__le32; 2],
    pub metadata: __le64,
    pub dptr: nvme_data_ptr,
    pub slba: __le64,
    pub cdw12: __le32,
    pub zsa: __u8,
    pub select_all: __u8,
    pub rsvd13: [__u8; 2],
    pub cdw14: [__le32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_zone_mgmt_recv_cmd {
    pub opcode: __u8,
    pub flags: __u8,
    pub command_id: __u16,
    pub nsid: __le32,
    pub rsvd2: [__le64; 2],
    pub dptr: nvme_data_ptr,
    pub slba: __le64,
    pub numd: __le32,
    pub zra: __u8,
    pub zrasf: __u8,
    pub pr: __u8,
    pub rsvd13: __u8,
    pub cdw14: [__le32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_io_mgmt_recv_cmd {
    pub opcode: __u8,
    pub flags: __u8,
    pub command_id: __u16,
    pub nsid: __le32,
    pub rsvd2: [__le64; 2],
    pub dptr: nvme_data_ptr,
    pub mo: __u8,
    pub rsvd11: __u8,
    pub mos: __u16,
    pub numd: __le32,
    pub cdw12: [__le32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_fdp_ruh_status_desc {
    pub pid: __le16,
    pub ruhid: __le16,
    pub earutr: __le32,
    pub ruamw: __le64,
    pub reserved: [__u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_fdp_ruh_status {
    pub rsvd0: [__u8; 14],
    pub nruhsd: __le16,
    pub ruhsd: [nvme_fdp_ruh_status_desc; ],
}

// Features
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_feat_auto_pst {
    pub entries: [__le64; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_feat_host_behavior {
    pub acre: __u8,
    pub etdas: __u8,
    pub lbafee: __u8,
    pub resv1: [__u8; 509],
}

// Admin commands
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvme_admin_opcode {
    nvme_admin_delete_sq		= 0x00,
    nvme_admin_create_sq		= 0x01,
    nvme_admin_get_log_page		= 0x02,
    nvme_admin_delete_cq		= 0x04,
    nvme_admin_create_cq		= 0x05,
    nvme_admin_identify		= 0x06,
    nvme_admin_abort_cmd		= 0x08,
    nvme_admin_set_features		= 0x09,
    nvme_admin_get_features		= 0x0a,
    nvme_admin_async_event		= 0x0c,
    nvme_admin_ns_mgmt		= 0x0d,
    nvme_admin_activate_fw		= 0x10,
    nvme_admin_download_fw		= 0x11,
    nvme_admin_dev_self_test	= 0x14,
    nvme_admin_ns_attach		= 0x15,
    nvme_admin_keep_alive		= 0x18,
    nvme_admin_directive_send	= 0x19,
    nvme_admin_directive_recv	= 0x1a,
    nvme_admin_virtual_mgmt		= 0x1c,
    nvme_admin_nvme_mi_send		= 0x1d,
    nvme_admin_nvme_mi_recv		= 0x1e,
    nvme_admin_dbbuf		= 0x7C,
    nvme_admin_format_nvm		= 0x80,
    nvme_admin_security_send	= 0x81,
    nvme_admin_security_recv	= 0x82,
    nvme_admin_sanitize_nvm		= 0x84,
    nvme_admin_get_lba_status	= 0x86,
    nvme_admin_vendor_start		= 0xC0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_supported_log {
    pub lids: [__le32; 256],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_supported_features_log {
    pub fis: [__le32; 256],
}

// NVMe Namespace Write Protect State
pub const NVME_MAX_CHANGED_NAMESPACES: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_identify {
    pub opcode: __u8,
    pub flags: __u8,
    pub command_id: __u16,
    pub nsid: __le32,
    pub rsvd2: [__u64; 2],
    pub dptr: nvme_data_ptr,
    pub cns: __u8,
    pub rsvd3: __u8,
    pub ctrlid: __le16,
    pub cnssid: __le16,
    pub rsvd11: __u8,
    pub csi: __u8,
    pub rsvd12: [__u32; 4],
}

pub const NVME_IDENTIFY_DATA_SIZE: c_int = 4096;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_features {
    pub opcode: __u8,
    pub flags: __u8,
    pub command_id: __u16,
    pub nsid: __le32,
    pub rsvd2: [__u64; 2],
    pub dptr: nvme_data_ptr,
    pub fid: __le32,
    pub dword11: __le32,
    pub dword12: __le32,
    pub dword13: __le32,
    pub dword14: __le32,
    pub dword15: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_host_mem_buf_desc {
    pub addr: __le64,
    pub size: __le32,
    pub rsvd: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_create_cq {
    pub opcode: __u8,
    pub flags: __u8,
    pub command_id: __u16,
    pub rsvd1: [__u32; 5],
    pub prp1: __le64,
    pub rsvd8: __u64,
    pub cqid: __le16,
    pub qsize: __le16,
    pub cq_flags: __le16,
    pub irq_vector: __le16,
    pub rsvd12: [__u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_create_sq {
    pub opcode: __u8,
    pub flags: __u8,
    pub command_id: __u16,
    pub rsvd1: [__u32; 5],
    pub prp1: __le64,
    pub rsvd8: __u64,
    pub sqid: __le16,
    pub qsize: __le16,
    pub sq_flags: __le16,
    pub cqid: __le16,
    pub rsvd12: [__u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_delete_queue {
    pub opcode: __u8,
    pub flags: __u8,
    pub command_id: __u16,
    pub rsvd1: [__u32; 9],
    pub qid: __le16,
    pub rsvd10: __u16,
    pub rsvd11: [__u32; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_abort_cmd {
    pub opcode: __u8,
    pub flags: __u8,
    pub command_id: __u16,
    pub rsvd1: [__u32; 9],
    pub sqid: __le16,
    pub cid: __u16,
    pub rsvd11: [__u32; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_download_firmware {
    pub opcode: __u8,
    pub flags: __u8,
    pub command_id: __u16,
    pub rsvd1: [__u32; 5],
    pub dptr: nvme_data_ptr,
    pub numd: __le32,
    pub offset: __le32,
    pub rsvd12: [__u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_format_cmd {
    pub opcode: __u8,
    pub flags: __u8,
    pub command_id: __u16,
    pub nsid: __le32,
    pub rsvd2: [__u64; 4],
    pub cdw10: __le32,
    pub rsvd11: [__u32; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_get_log_page_command {
    pub opcode: __u8,
    pub flags: __u8,
    pub command_id: __u16,
    pub nsid: __le32,
    pub rsvd2: [__u64; 2],
    pub dptr: nvme_data_ptr,
    pub lid: __u8,
    pub /: *mut *mut __u8 lsp; / upper 4 bits reserved,
    pub numdl: __le16,
    pub numdu: __le16,
    pub lsi: __le16,
    pub lpol: __le32,
    pub lpou: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_directive_cmd {
    pub opcode: __u8,
    pub flags: __u8,
    pub command_id: __u16,
    pub nsid: __le32,
    pub rsvd2: [__u64; 2],
    pub dptr: nvme_data_ptr,
    pub numd: __le32,
    pub doper: __u8,
    pub dtype: __u8,
    pub dspec: __le16,
    pub endir: __u8,
    pub tdtype: __u8,
    pub rsvd15: __u16,
    pub rsvd16: [__u32; 3],
}

//
// Fabrics subcommands.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvmf_fabrics_opcode {
    nvme_fabrics_command		= 0x7f,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvmf_capsule_command {
    nvme_fabrics_type_property_set	= 0x00,
    nvme_fabrics_type_connect	= 0x01,
    nvme_fabrics_type_property_get	= 0x04,
    nvme_fabrics_type_auth_send	= 0x05,
    nvme_fabrics_type_auth_receive	= 0x06,
}

//
// If not fabrics command, fctype will be ignored.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmf_common_command {
    pub opcode: __u8,
    pub resv1: __u8,
    pub command_id: __u16,
    pub fctype: __u8,
    pub resv2: [__u8; 35],
    pub ts: [__u8; 24],
}

//
// The legal cntlid range a NVMe Target will provide.
// Note that cntlid of value 0 is considered illegal in the fabrics world.
// Devices based on earlier specs did not have the subsystem concept;
// therefore, those devices had their cntlid value set to 0 as a result.
//
pub const NVME_CNTLID_MIN: c_int = 1;
pub const NVME_CNTLID_MAX: c_uint = 0xffef;
pub const NVME_CNTLID_DYNAMIC: c_uint = 0xffff;
pub const MAX_DISC_LOGS: c_int = 255;
// Discovery log page entry flags (EFLAGS):
// Discovery log page entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmf_disc_rsp_page_entry {
    pub trtype: __u8,
    pub adrfam: __u8,
    pub subtype: __u8,
    pub treq: __u8,
    pub portid: __le16,
    pub cntlid: __le16,
    pub asqsz: __le16,
    pub eflags: __le16,
    pub resv10: [__u8; 20],
    pub trsvcid: [c_char; NVMF_TRSVCID_SIZE],
    pub resv64: [__u8; 192],
    pub subnqn: [c_char; NVMF_NQN_FIELD_LEN],
    pub traddr: [c_char; NVMF_TRADDR_SIZE],
#[repr(C)]
#[derive(Copy, Clone)]
pub union tsas {
    pub common: [c_char; NVMF_TSAS_SIZE],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma {
    pub qptype: __u8,
    pub prtype: __u8,
    pub cms: __u8,
    pub resv3: [__u8; 5],
    pub pkey: __u16,
    pub resv10: [__u8; 246],
    pub rdma: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp {
    pub sectype: __u8,
    pub tcp: },
    pub tsas: },
}

// Discovery log page header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmf_disc_rsp_page_hdr {
    pub genctr: __le64,
    pub numrec: __le64,
    pub recfmt: __le16,
    pub resv14: [__u8; 1006],
    pub entries: [nvmf_disc_rsp_page_entry; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmf_connect_command {
    pub opcode: __u8,
    pub resv1: __u8,
    pub command_id: __u16,
    pub fctype: __u8,
    pub resv2: [__u8; 19],
    pub dptr: nvme_data_ptr,
    pub recfmt: __le16,
    pub qid: __le16,
    pub sqsize: __le16,
    pub cattr: __u8,
    pub resv3: __u8,
    pub kato: __le32,
    pub resv4: [__u8; 12],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmf_connect_data {
    pub hostid: uuid_t,
    pub cntlid: __le16,
    pub resv4: [c_char; 238],
    pub subsysnqn: [c_char; NVMF_NQN_FIELD_LEN],
    pub hostnqn: [c_char; NVMF_NQN_FIELD_LEN],
    pub resv5: [c_char; 256],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmf_property_set_command {
    pub opcode: __u8,
    pub resv1: __u8,
    pub command_id: __u16,
    pub fctype: __u8,
    pub resv2: [__u8; 35],
    pub attrib: __u8,
    pub resv3: [__u8; 3],
    pub offset: __le32,
    pub value: __le64,
    pub resv4: [__u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmf_property_get_command {
    pub opcode: __u8,
    pub resv1: __u8,
    pub command_id: __u16,
    pub fctype: __u8,
    pub resv2: [__u8; 35],
    pub attrib: __u8,
    pub resv3: [__u8; 3],
    pub offset: __le32,
    pub resv4: [__u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmf_auth_common_command {
    pub opcode: __u8,
    pub resv1: __u8,
    pub command_id: __u16,
    pub fctype: __u8,
    pub resv2: [__u8; 19],
    pub dptr: nvme_data_ptr,
    pub resv3: __u8,
    pub spsp0: __u8,
    pub spsp1: __u8,
    pub secp: __u8,
    pub al_tl: __le32,
    pub resv4: [__u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmf_auth_send_command {
    pub opcode: __u8,
    pub resv1: __u8,
    pub command_id: __u16,
    pub fctype: __u8,
    pub resv2: [__u8; 19],
    pub dptr: nvme_data_ptr,
    pub resv3: __u8,
    pub spsp0: __u8,
    pub spsp1: __u8,
    pub secp: __u8,
    pub tl: __le32,
    pub resv4: [__u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmf_auth_receive_command {
    pub opcode: __u8,
    pub resv1: __u8,
    pub command_id: __u16,
    pub fctype: __u8,
    pub resv2: [__u8; 19],
    pub dptr: nvme_data_ptr,
    pub resv3: __u8,
    pub spsp0: __u8,
    pub spsp1: __u8,
    pub secp: __u8,
    pub al: __le32,
    pub resv4: [__u8; 16],
}

// Value for secp
// Defined value for auth_type
// Defined messages for auth_id
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmf_auth_dhchap_protocol_descriptor {
    pub authid: __u8,
    pub rsvd: __u8,
    pub halen: __u8,
    pub dhlen: __u8,
    pub idlist: [__u8; 60],
}

// Defined hash functions for DH-HMAC-CHAP authentication
// Maximum digest size for any NVME_AUTH_HASH_* value
// Defined Diffie-Hellman group identifiers for DH-HMAC-CHAP authentication
#[repr(C)]
#[derive(Copy, Clone)]
pub union nvmf_auth_protocol {
    pub dhchap: nvmf_auth_dhchap_protocol_descriptor,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmf_auth_dhchap_negotiate_data {
    pub auth_type: __u8,
    pub auth_id: __u8,
    pub rsvd: __le16,
    pub t_id: __le16,
    pub sc_c: __u8,
    pub napd: __u8,
    pub auth_protocol: [nvmf_auth_protocol; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmf_auth_dhchap_challenge_data {
    pub auth_type: __u8,
    pub auth_id: __u8,
    pub rsvd1: __u16,
    pub t_id: __le16,
    pub hl: __u8,
    pub rsvd2: __u8,
    pub hashid: __u8,
    pub dhgid: __u8,
    pub dhvlen: __le16,
    pub seqnum: __le32,
// 'hl' bytes of challenge value
    pub cval: [__u8; ],
// followed by 'dhvlen' bytes of DH value
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmf_auth_dhchap_reply_data {
    pub auth_type: __u8,
    pub auth_id: __u8,
    pub rsvd1: __le16,
    pub t_id: __le16,
    pub hl: __u8,
    pub rsvd2: __u8,
    pub cvalid: __u8,
    pub rsvd3: __u8,
    pub dhvlen: __le16,
    pub seqnum: __le32,
// 'hl' bytes of response data
    pub rval: [__u8; ],
// followed by 'hl' bytes of Challenge value
// followed by 'dhvlen' bytes of DH value
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmf_auth_dhchap_success1_data {
    pub auth_type: __u8,
    pub auth_id: __u8,
    pub rsvd1: __le16,
    pub t_id: __le16,
    pub hl: __u8,
    pub rsvd2: __u8,
    pub rvalid: __u8,
    pub rsvd3: [__u8; 7],
// 'hl' bytes of response value
    pub rval: [__u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmf_auth_dhchap_success2_data {
    pub auth_type: __u8,
    pub auth_id: __u8,
    pub rsvd1: __le16,
    pub t_id: __le16,
    pub rsvd2: [__u8; 10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmf_auth_dhchap_failure_data {
    pub auth_type: __u8,
    pub auth_id: __u8,
    pub rsvd1: __le16,
    pub t_id: __le16,
    pub rescode: __u8,
    pub rescode_exp: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_dbbuf {
    pub opcode: __u8,
    pub flags: __u8,
    pub command_id: __u16,
    pub rsvd1: [__u32; 5],
    pub prp1: __le64,
    pub prp2: __le64,
    pub rsvd12: [__u32; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct streams_directive_params {
    pub msl: __le16,
    pub nssa: __le16,
    pub nsso: __le16,
    pub rsvd: [__u8; 10],
    pub sws: __le32,
    pub sgs: __le16,
    pub nsa: __le16,
    pub nso: __le16,
    pub rsvd2: [__u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_command {
    pub common: nvme_common_command,
    pub rw: nvme_rw_command,
    pub identify: nvme_identify,
    pub features: nvme_features,
    pub create_cq: nvme_create_cq,
    pub create_sq: nvme_create_sq,
    pub delete_queue: nvme_delete_queue,
    pub dlfw: nvme_download_firmware,
    pub format: nvme_format_cmd,
    pub dsm: nvme_dsm_cmd,
    pub write_zeroes: nvme_write_zeroes_cmd,
    pub zms: nvme_zone_mgmt_send_cmd,
    pub zmr: nvme_zone_mgmt_recv_cmd,
    pub abort: nvme_abort_cmd,
    pub get_log_page: nvme_get_log_page_command,
    pub fabrics: nvmf_common_command,
    pub connect: nvmf_connect_command,
    pub prop_set: nvmf_property_set_command,
    pub prop_get: nvmf_property_get_command,
    pub auth_common: nvmf_auth_common_command,
    pub auth_send: nvmf_auth_send_command,
    pub auth_receive: nvmf_auth_receive_command,
    pub dbbuf: nvme_dbbuf,
    pub directive: nvme_directive_cmd,
    pub imr: nvme_io_mgmt_recv_cmd,
}

extern "C" {
    pub fn nvme_get_fabrics_opcode_str(_arg: cmd->fabrics.fctype) -> return;
}
extern "C" {
    pub fn nvme_opcode_str(_arg: qid, _arg: cmd->common.opcode) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_error_slot {
    pub error_count: __le64,
    pub sqid: __le16,
    pub cmdid: __le16,
    pub status_field: __le16,
    pub param_error_location: __le16,
    pub lba: __le64,
    pub nsid: __le32,
    pub vs: __u8,
    pub resv: [__u8; 3],
    pub cs: __le64,
    pub resv2: [__u8; 24],
}

//
// What a mess...
//
// Why can't we simply have a Fabrics In and Fabrics out command?
//
// Generic Command Status:
//
// Command Specific Status:
//
// I/O Command Set Specific - NVM commands:
//
// I/O Command Set Specific - Fabrics commands:
//
// I/O Command Set Specific - Zoned commands:
//
// Media and Data Integrity Errors:
//
// Path-related Errors:
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_completion {
//
// Used by Admin and Fabrics commands to return data:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union nvme_result {
    pub u16: __le16,
    pub u32: __le32,
    pub u64: __le64,
    pub result: },
    pub /: *mut *mut __le16 sq_head; / how much of this queue may be reclaimed,
    pub /: *mut *mut __le16 sq_id; / submission queue that generated this entry,
    pub /: *mut *mut __u16 command_id; / of the command which completed,
    pub /: *mut *mut __le16 status; / did the command fail, and if so, why?,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_pr_log {
    pub count: __le64,
    pub type: __u8,
    pub nr_pages: __u8,
    pub rsvd1: [__u8; 2],
    pub nsid: __le32,
    pub rsvd2: [__u8; 48],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmet_pr_register_data {
    pub crkey: __le64,
    pub nrkey: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmet_pr_acquire_data {
    pub crkey: __le64,
    pub prkey: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmet_pr_release_data {
    pub crkey: __le64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvme_pr_capabilities {
    NVME_PR_SUPPORT_PTPL				= 1,
    NVME_PR_SUPPORT_WRITE_EXCLUSIVE			= 1 << 1,
    NVME_PR_SUPPORT_EXCLUSIVE_ACCESS		= 1 << 2,
    NVME_PR_SUPPORT_WRITE_EXCLUSIVE_REG_ONLY	= 1 << 3,
    NVME_PR_SUPPORT_EXCLUSIVE_ACCESS_REG_ONLY	= 1 << 4,
    NVME_PR_SUPPORT_WRITE_EXCLUSIVE_ALL_REGS	= 1 << 5,
    NVME_PR_SUPPORT_EXCLUSIVE_ACCESS_ALL_REGS	= 1 << 6,
    NVME_PR_SUPPORT_IEKEY_VER_1_3_DEF		= 1 << 7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvme_pr_register_action {
    NVME_PR_REGISTER_ACT_REG		= 0,
    NVME_PR_REGISTER_ACT_UNREG		= 1,
    NVME_PR_REGISTER_ACT_REPLACE		= 1 << 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvme_pr_acquire_action {
    NVME_PR_ACQUIRE_ACT_ACQUIRE		= 0,
    NVME_PR_ACQUIRE_ACT_PREEMPT		= 1,
    NVME_PR_ACQUIRE_ACT_PREEMPT_AND_ABORT	= 1 << 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvme_pr_release_action {
    NVME_PR_RELEASE_ACT_RELEASE		= 0,
    NVME_PR_RELEASE_ACT_CLEAR		= 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvme_pr_change_ptpl {
    NVME_PR_CPTPL_NO_CHANGE			= 0,
    NVME_PR_CPTPL_RESV			= 1 << 30,
    NVME_PR_CPTPL_CLEARED			= 2 << 30,
    NVME_PR_CPTPL_PERSIST			= 3 << 30,
}

// Section 8.3.4.5.2 of the NVMe 2.1
pub const NVME_AUTH_DHCHAP_MAX_HASH_IDS: c_int = 30;
pub const NVME_AUTH_DHCHAP_MAX_DH_IDS: c_int = 30;
