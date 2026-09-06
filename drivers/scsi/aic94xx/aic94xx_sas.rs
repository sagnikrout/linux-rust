//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/aic94xx/aic94xx_sas.h
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
// Aic94xx SAS/SATA driver SAS definitions and hardware interface header file.
//
// Copyright (C) 2005 Adaptec, Inc.  All rights reserved.
// Copyright (C) 2005 Luben Tuikov <luben_tuikov@adaptec.com>
//

// ---------- DDBs ----------
// DDBs are device descriptor blocks which describe a device in the
// domain that this sequencer can maintain low-level connections for
// us.  They are be 64 bytes.
//
pub const ASD_MAX_DDBS: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct asd_ddb_ssp_smp_target_port {
    pub /: *mut *mut u8 conn_type; / byte 0,
pub const DDB_TP_CONN_TYPE: c_uint = 0x81	  /* Initiator port and addr frame type 0x01 */;
    pub conn_rate: u8,
    pub init_conn_tag: __be16,
    pub /: *mut *mut u8 dest_sas_addr[8]; / bytes 4-11,
    pub send_queue_head: __le16,
    pub sq_suspended: u8,
    pub /: *mut *mut u8 ddb_type; / DDB_TYPE_TARGET,
pub const DDB_TYPE_UNUSED: c_uint = 0xFF;
pub const DDB_TYPE_TARGET: c_uint = 0xFE;
pub const DDB_TYPE_INITIATOR: c_uint = 0xFD;
pub const DDB_TYPE_PM_PORT: c_uint = 0xFC;
    pub _r_a: __le16,
    pub awt_def: __be16,
    pub /: *mut *mut u8 compat_features; / byte 20,
    pub pathway_blocked_count: u8,
    pub arb_wait_time: __be16,
    pub /: *mut *mut __be32 more_compat_features; / byte 24,
    pub conn_mask: u8,
    pub /: *mut *mut u8 flags; / concurrent conn:2,2 and open:0(1),
pub const CONCURRENT_CONN_SUPP: c_uint = 0x04;
pub const OPEN_REQUIRED: c_uint = 0x01;
    pub _r_b: u16,
    pub exec_queue_tail: __le16,
    pub send_queue_tail: __le16,
    pub sister_ddb: __le16,
    pub _r_c: __le16,
    pub max_concurrent_conn: u8,
    pub num_concurrent_conn: u8,
    pub num_contexts: u8,
    pub _r_d: u8,
    pub active_task_count: __le16,
    pub _r_e: [u8; 9],
    pub /: *mut *mut u8 itnl_reason; / I_T nexus loss reason,
    pub _r_f: __le16,
    pub itnl_timeout: __le16,
pub const ITNL_TIMEOUT_CONST: c_uint = 0x7D0 /* 2 seconds */;
    pub itnl_timestamp: __le32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct asd_ddb_stp_sata_target_port {
    pub /: *mut *mut u8 conn_type; / byte 0,
    pub conn_rate: u8,
    pub init_conn_tag: __be16,
    pub /: *mut *mut u8 dest_sas_addr[8]; / bytes 4-11,
    pub send_queue_head: __le16,
    pub sq_suspended: u8,
    pub /: *mut *mut u8 ddb_type; / DDB_TYPE_TARGET,
    pub _r_a: __le16,
    pub awt_def: __be16,
    pub /: *mut *mut u8 compat_features; / byte 20,
    pub pathway_blocked_count: u8,
    pub arb_wait_time: __be16,
    pub /: *mut *mut __be32 more_compat_features; / byte 24,
    pub conn_mask: u8,
    pub /: *mut *mut u8 flags; / concurrent conn:2,2 and open:0(1),
pub const SATA_MULTIPORT: c_uint = 0x80;
pub const SUPPORTS_AFFIL: c_uint = 0x40;
pub const STP_AFFIL_POL: c_uint = 0x20;
    pub _r_b: u8,
    pub /: *mut *mut u8 flags2; / STP close policy:0,
pub const STP_CL_POL_NO_TX: c_uint = 0x00;
pub const STP_CL_POL_BTW_CMDS: c_uint = 0x01;
    pub exec_queue_tail: __le16,
    pub send_queue_tail: __le16,
    pub sister_ddb: __le16,
    pub ata_cmd_scbptr: __le16,
    pub sata_tag_alloc_mask: __le32,
    pub active_task_count: __le16,
    pub _r_c: __le16,
    pub sata_sactive: __le32,
    pub num_sata_tags: u8,
    pub sata_status: u8,
    pub sata_ending_status: u8,
    pub /: *mut *mut u8 itnl_reason; / I_T nexus loss reason,
    pub ncq_data_scb_ptr: __le16,
    pub itnl_timeout: __le16,
    pub itnl_timestamp: __le32,
// C attribute field omitted
// This struct asd_ddb_init_port, describes the device descriptor block
// of an initiator port (when the sequencer is operating in target mode).
// Bytes [0,11] and [20,27] are from the OPEN address frame.
// The sequencer allocates an initiator port DDB entry.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct asd_ddb_init_port {
    pub /: *mut *mut u8 conn_type; / byte 0,
    pub conn_rate: u8,
    pub /: *mut *mut __be16 init_conn_tag; / BE,
    pub dest_sas_addr: [u8; 8],
    pub /: *mut *mut __le16 send_queue_head; / LE, byte 12,
    pub sq_suspended: u8,
    pub /: *mut *mut u8 ddb_type; / DDB_TYPE_INITIATOR,
    pub _r_a: __le16,
    pub /: *mut *mut __be16 awt_def; / BE,
    pub compat_features: u8,
    pub pathway_blocked_count: u8,
    pub /: *mut *mut __be16 arb_wait_time; / BE,
    pub /: *mut *mut __be32 more_compat_features; / BE,
    pub conn_mask: u8,
    pub /: *mut *mut u8 flags; / == 5,
    pub _r_b: u16,
    pub /: *mut *mut __le16 exec_queue_tail; / execution queue tail,
    pub send_queue_tail: __le16,
    pub sister_ddb: __le16,
    pub /: *mut *mut __le16 init_resp_timeout; / initiator response timeout,
    pub _r_c: __le32,
    pub /: *mut *mut __le16 active_tasks; / active task count,
    pub /: *mut *mut __le16 init_list; / initiator list link pointer,
    pub _r_d: __le32,
    pub /: *mut *mut u8 max_conn_to[3]; / from Conn-Disc mode page, in us, LE,
    pub /: *mut *mut u8 itnl_reason; / I_T nexus loss reason,
    pub /: *mut *mut __le16 bus_inact_to; / from Conn-Disc mode page, in 100 us, LE,
    pub /: *mut *mut __le16 itnl_to; / from the Protocol Specific Port Ctrl MP,
    pub itnl_timestamp: __le32,
// C attribute field omitted
// This struct asd_ddb_sata_tag, describes a look-up table to be used
// by the sequencers.  SATA II, IDENTIFY DEVICE data, word 76, bit 8:
// NCQ support.  This table is used by the sequencers to find the
// corresponding SCB, given a SATA II tag value.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct asd_ddb_sata_tag {
    pub scb_pointer: [__le16; 32],
// C attribute field omitted
// This struct asd_ddb_sata_pm_table, describes a port number to
// connection handle look-up table.  SATA targets attached to a port
// multiplier require a 4-bit port number value.  There is one DDB
// entry of this type for each SATA port multiplier (sister DDB).
// Given a SATA PM port number, this table gives us the SATA PM Port
// DDB of the SATA port multiplier port (i.e. the SATA target
// discovered on the port).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct asd_ddb_sata_pm_table {
    pub ddb_pointer: [__le16; 16],
    pub _r_a: [__le16; 16],
// C attribute field omitted
// This struct asd_ddb_sata_pm_port, describes the SATA port multiplier
// port format DDB.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct asd_ddb_sata_pm_port {
    pub _r_a: [u8; 15],
    pub ddb_type: u8,
    pub _r_b: [u8; 13],
    pub pm_port_flags: u8,
pub const PM_PORT_MASK: c_uint = 0xF0;
pub const PM_PORT_SET: c_uint = 0x02;
    pub _r_c: [u8; 6],
    pub sister_ddb: __le16,
    pub ata_cmd_scbptr: __le16,
    pub sata_tag_alloc_mask: __le32,
    pub active_task_count: __le16,
    pub parent_ddb: __le16,
    pub sata_sactive: __le32,
    pub num_sata_tags: u8,
    pub sata_status: u8,
    pub sata_ending_status: u8,
    pub _r_d: [u8; 9],
// C attribute field omitted
// This struct asd_ddb_seq_shared, describes a DDB shared by the
// central and link sequencers.  port_map_by_links is indexed phy
// number [0,7]; each byte is a bit mask of all the phys that are in
// the same port as the indexed phy.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct asd_ddb_seq_shared {
    pub q_free_ddb_head: __le16,
    pub q_free_ddb_tail: __le16,
    pub q_free_ddb_cnt: __le16,
    pub q_used_ddb_head: __le16,
    pub q_used_ddb_tail: __le16,
    pub shared_mem_lock: __le16,
    pub smp_conn_tag: __le16,
    pub est_nexus_buf_cnt: __le16,
    pub est_nexus_buf_thresh: __le16,
    pub _r_a: u32,
    pub settable_max_contexts: u8,
    pub _r_b: [u8; 23],
    pub conn_not_active: u8,
    pub phy_is_up: u8,
    pub _r_c: [u8; 8],
    pub port_map_by_links: [u8; 8],
// C attribute field omitted
// ---------- SG Element ----------
// This struct sg_el, describes the hardware scatter gather buffer
// element.  All entries are little endian.  In an SCB, there are 2 of
// this, plus one more, called a link element of this indicating a
// sublist if needed.
//
// A link element has only the bus address set and the flags (DS) bit
// valid.  The bus address points to the start of the sublist.
//
// If a sublist is needed, then that sublist should also include the 2
// sg_el embedded in the SCB, in which case next_sg_offset is 32,
// since sizeof(sg_el) = 16; EOS should be 1 and EOL 0 in this case.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sg_el {
    pub bus_addr: __le64,
    pub size: __le32,
    pub _r: __le16,
    pub next_sg_offs: u8,
    pub flags: u8,
pub const ASD_SG_EL_DS_MASK: c_uint = 0x30;
pub const ASD_SG_EL_DS_OCM: c_uint = 0x10;
pub const ASD_SG_EL_DS_HM: c_uint = 0x00;
pub const ASD_SG_EL_LIST_MASK: c_uint = 0xC0;
pub const ASD_SG_EL_LIST_EOL: c_uint = 0x40;
pub const ASD_SG_EL_LIST_EOS: c_uint = 0x80;
// C attribute field omitted
// ---------- SCBs ----------
// An SCB (sequencer control block) is comprised of a common header
// and a task part, for a total of 128 bytes.  All fields are in LE
// order, unless otherwise noted.
//
// This struct scb_header, defines the SCB header format.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scb_header {
    pub next_scb: __le64,
    pub /: *mut *mut __le16 index; / transaction context,
    pub opcode: u8,
// C attribute field omitted
// SCB opcodes: Execution queue
//
pub const INITIATE_SSP_TASK: c_uint = 0x00;
pub const INITIATE_LONG_SSP_TASK: c_uint = 0x01;
pub const INITIATE_BIDIR_SSP_TASK: c_uint = 0x02;
pub const SCB_ABORT_TASK: c_uint = 0x03;
pub const INITIATE_SSP_TMF: c_uint = 0x04;
pub const SSP_TARG_GET_DATA: c_uint = 0x05;
pub const SSP_TARG_GET_DATA_GOOD: c_uint = 0x06;
pub const SSP_TARG_SEND_RESP: c_uint = 0x07;
pub const QUERY_SSP_TASK: c_uint = 0x08;
pub const INITIATE_ATA_TASK: c_uint = 0x09;
pub const INITIATE_ATAPI_TASK: c_uint = 0x0a;
pub const CONTROL_ATA_DEV: c_uint = 0x0b;
pub const INITIATE_SMP_TASK: c_uint = 0x0c;
pub const SMP_TARG_SEND_RESP: c_uint = 0x0f;
// SCB opcodes: Send Queue
//
pub const SSP_TARG_SEND_DATA: c_uint = 0x40;
pub const SSP_TARG_SEND_DATA_GOOD: c_uint = 0x41;
// SCB opcodes: Link Queue
//
pub const CONTROL_PHY: c_uint = 0x80;
pub const SEND_PRIMITIVE: c_uint = 0x81;
pub const INITIATE_LINK_ADM_TASK: c_uint = 0x82;
// SCB opcodes: other
//
pub const EMPTY_SCB: c_uint = 0xc0;
pub const INITIATE_SEQ_ADM_TASK: c_uint = 0xc1;
pub const EST_ICL_TARG_WINDOW: c_uint = 0xc2;
pub const COPY_MEM: c_uint = 0xc3;
pub const CLEAR_NEXUS: c_uint = 0xc4;
pub const INITIATE_DDB_ADM_TASK: c_uint = 0xc6;
pub const ESTABLISH_NEXUS_ESCB: c_uint = 0xd0;
pub const LUN_SIZE: c_int = 8;
pub const EFB_MASK: c_uint = 0x80;
pub const TASK_PRIO_MASK: c_uint = 0x78;
pub const TASK_ATTR_MASK: c_uint = 0x07;
// ---------- SCB tasks ----------
// This is both ssp_task and long_ssp_task
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct initiate_ssp_task {
    pub /: *mut *mut u8 proto_conn_rate; / proto:6,4, conn_rate:3,0,
    pub total_xfer_len: __le32,
    pub ssp_frame: ssp_frame_hdr,
    pub ssp_cmd: ssp_command_iu,
    pub /: *mut *mut __le16 sister_scb; / 0xFFFF,
    pub /: *mut *mut __le16 conn_handle; / index to DDB for the intended target,
    pub /: *mut *mut u8 data_dir; / :1,0,
pub const DATA_DIR_NONE: c_uint = 0x00;
pub const DATA_DIR_IN: c_uint = 0x01;
pub const DATA_DIR_OUT: c_uint = 0x02;
pub const DATA_DIR_BYRECIPIENT: c_uint = 0x03;
    pub _r_a: u8,
    pub retry_count: u8,
    pub _r_b: [u8; 5],
    pub /: *mut *mut sg_el sg_element[3]; / 2 real and 1 link,
// C attribute field omitted
// This defines both ata_task and atapi_task.
// ata: C bit of FIS should be 1,
// atapi: C bit of FIS should be 1, and command register should be 0xA0,
// to indicate a packet command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct initiate_ata_task {
    pub proto_conn_rate: u8,
    pub total_xfer_len: __le32,
    pub fis: host_to_dev_fis,
    pub data_offs: __le32,
    pub atapi_packet: [u8; 16],
    pub _r_a: [u8; 12],
    pub sister_scb: __le16,
    pub conn_handle: __le16,
    pub /: *mut *mut u8 ata_flags; / CSMI:6,6, DTM:4,4, QT:3,3, data dir:1,0,
pub const CSMI_TASK: c_uint = 0x40;
pub const DATA_XFER_MODE_DMA: c_uint = 0x10;
pub const ATA_Q_TYPE_MASK: c_uint = 0x08;
pub const ATA_Q_TYPE_UNTAGGED: c_uint = 0x00;
pub const ATA_Q_TYPE_NCQ: c_uint = 0x08;
    pub _r_b: u8,
    pub retry_count: u8,
    pub _r_c: u8,
    pub flags: u8,
pub const STP_AFFIL_POLICY: c_uint = 0x20;
pub const SET_AFFIL_POLICY: c_uint = 0x10;
pub const RET_PARTIAL_SGLIST: c_uint = 0x02;
    pub _r_d: [u8; 3],
    pub sg_element: [sg_el; 3],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct initiate_smp_task {
    pub proto_conn_rate: u8,
    pub _r_a: [u8; 40],
    pub smp_req: sg_el,
    pub sister_scb: __le16,
    pub conn_handle: __le16,
    pub _r_c: [u8; 8],
    pub smp_resp: sg_el,
    pub _r_d: [u8; 32],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct control_phy {
    pub phy_id: u8,
    pub sub_func: u8,
pub const DISABLE_PHY: c_uint = 0x00;
pub const ENABLE_PHY: c_uint = 0x01;
pub const RELEASE_SPINUP_HOLD: c_uint = 0x02;
pub const ENABLE_PHY_NO_SAS_OOB: c_uint = 0x03;
pub const ENABLE_PHY_NO_SATA_OOB: c_uint = 0x04;
pub const PHY_NO_OP: c_uint = 0x05;
pub const EXECUTE_HARD_RESET: c_uint = 0x81;
    pub func_mask: u8,
    pub speed_mask: u8,
    pub hot_plug_delay: u8,
    pub port_type: u8,
    pub flags: u8,
pub const DEV_PRES_TIMER_OVERRIDE_ENABLE: c_uint = 0x01;
pub const DISABLE_PHY_IF_OOB_FAILS: c_uint = 0x02;
    pub timeout_override: __le32,
    pub link_reset_retries: u8,
    pub _r_a: [u8; 47],
    pub conn_handle: __le16,
    pub _r_b: [u8; 56],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct control_ata_dev {
    pub proto_conn_rate: u8,
    pub _r_a: __le32,
    pub fis: host_to_dev_fis,
    pub _r_b: [u8; 32],
    pub sister_scb: __le16,
    pub conn_handle: __le16,
    pub /: *mut *mut u8 ata_flags; / 0,
    pub _r_c: [u8; 55],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct empty_scb {
    pub num_valid: u8,
    pub _r_a: __le32,
pub const ASD_EDBS_PER_SCB: c_int = 7;
// header+data+CRC+DMA suffix data
    pub eb: [sg_el; ASD_EDBS_PER_SCB],
pub const ELEMENT_NOT_VALID: c_uint = 0xC0;
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct initiate_link_adm {
    pub phy_id: u8,
    pub sub_func: u8,
pub const GET_LINK_ERROR_COUNT: c_uint = 0x00;
pub const RESET_LINK_ERROR_COUNT: c_uint = 0x01;
pub const ENABLE_NOTIFY_SPINUP_INTS: c_uint = 0x02;
    pub _r_a: [u8; 57],
    pub conn_handle: __le16,
    pub _r_b: [u8; 56],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct copy_memory {
    pub _r_a: u8,
    pub xfer_len: __le16,
    pub _r_b: __le16,
    pub src_busaddr: __le64,
    pub /: *mut *mut u8 src_ds; / See definition of sg_el,
    pub _r_c: [u8; 45],
    pub conn_handle: __le16,
    pub _r_d: __le64,
    pub dest_busaddr: __le64,
    pub /: *mut *mut u8 dest_ds; / See definition of sg_el,
    pub _r_e: [u8; 39],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct abort_task {
    pub proto_conn_rate: u8,
    pub _r_a: __le32,
    pub ssp_frame: ssp_frame_hdr,
    pub ssp_task: ssp_tmf_iu,
    pub sister_scb: __le16,
    pub conn_handle: __le16,
    pub /: *mut *mut u8 flags; / ovrd_itnl_timer:3,3, suspend_data_trans:2,2,
pub const SUSPEND_DATA_TRANS: c_uint = 0x04;
    pub _r_b: u8,
    pub retry_count: u8,
    pub _r_c: [u8; 5],
    pub /: *mut *mut __le16 index; / Transaction context of task to be queried,
    pub itnl_to: __le16,
    pub _r_d: [u8; 44],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clear_nexus {
    pub nexus: u8,
pub const NEXUS_ADAPTER: c_uint = 0x00;
pub const NEXUS_PORT: c_uint = 0x01;
pub const NEXUS_I_T: c_uint = 0x02;
pub const NEXUS_I_T_L: c_uint = 0x03;
pub const NEXUS_TAG: c_uint = 0x04;
pub const NEXUS_TRANS_CX: c_uint = 0x05;
pub const NEXUS_SATA_TAG: c_uint = 0x06;
pub const NEXUS_T_L: c_uint = 0x07;
pub const NEXUS_L: c_uint = 0x08;
pub const NEXUS_T_TAG: c_uint = 0x09;
    pub _r_a: __le32,
    pub flags: u8,
pub const SUSPEND_TX: c_uint = 0x80;
pub const RESUME_TX: c_uint = 0x40;
pub const SEND_Q: c_uint = 0x04;
pub const EXEC_Q: c_uint = 0x02;
pub const NOTINQ: c_uint = 0x01;
    pub _r_b: [u8; 3],
    pub conn_mask: u8,
    pub _r_c: [u8; 19],
    pub /: *mut *mut ssp_tmf_iu ssp_task; / LUN and TAG,
    pub _r_d: __le16,
    pub conn_handle: __le16,
    pub _r_e: __le64,
    pub /: *mut *mut __le16 index; / Transaction context of task to be cleared,
    pub /: *mut *mut __le16 context; / Clear nexus context,
    pub _r_f: [u8; 44],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct initiate_ssp_tmf {
    pub proto_conn_rate: u8,
    pub _r_a: __le32,
    pub ssp_frame: ssp_frame_hdr,
    pub ssp_task: ssp_tmf_iu,
    pub sister_scb: __le16,
    pub conn_handle: __le16,
    pub /: *mut *mut u8 flags; / itnl override and suspend data tx,
pub const OVERRIDE_ITNL_TIMER: c_int = 8;
    pub _r_b: u8,
    pub retry_count: u8,
    pub _r_c: [u8; 5],
    pub /: *mut *mut __le16 index; / Transaction context of task to be queried,
    pub itnl_to: __le16,
    pub _r_d: [u8; 44],
// C attribute field omitted
// Transmits an arbitrary primitive on the link.
// Used for NOTIFY and BROADCAST.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct send_prim {
    pub phy_id: u8,
    pub /: *mut *mut u8 wait_transmit; / :0,0,
    pub xmit_flags: u8,
pub const XMTPSIZE_MASK: c_uint = 0xF0;
pub const XMTPSIZE_SINGLE: c_uint = 0x10;
pub const XMTPSIZE_REPEATED: c_uint = 0x20;
pub const XMTPSIZE_CONT: c_uint = 0x20;
pub const XMTPSIZE_TRIPLE: c_uint = 0x30;
pub const XMTPSIZE_REDUNDANT: c_uint = 0x60;
pub const XMTPSIZE_INF: c_int = 0;
pub const XMTCONTEN: c_uint = 0x04;
pub const XMTPFRM: c_uint = 0x02	  /* Transmit at the next frame boundary */;
pub const XMTPIMM: c_uint = 0x01	  /* Transmit immediately */;
    pub _r_a: __le16,
    pub /: *mut *mut u8 prim[4]; / K, D0, D1, D2,
    pub _r_b: [u8; 50],
    pub conn_handle: __le16,
    pub _r_c: [u8; 56],
// C attribute field omitted
// This describes both SSP Target Get Data and SSP Target Get Data And
// Send Good Response SCBs.  Used when the sequencer is operating in
// target mode...
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssp_targ_get_data {
    pub proto_conn_rate: u8,
    pub total_xfer_len: __le32,
    pub ssp_frame: ssp_frame_hdr,
    pub xfer_rdy: xfer_rdy_iu,
    pub lun: [u8; LUN_SIZE],
    pub _r_a: __le64,
    pub sister_scb: __le16,
    pub conn_handle: __le16,
    pub /: *mut *mut u8 data_dir; / 01b,
    pub _r_b: u8,
    pub retry_count: u8,
    pub _r_c: [u8; 5],
    pub sg_element: [sg_el; 3],
// C attribute field omitted
// ---------- The actual SCB struct ----------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scb {
    pub header: scb_header,
    pub ssp_task: initiate_ssp_task,
    pub ata_task: initiate_ata_task,
    pub smp_task: initiate_smp_task,
    pub control_phy: control_phy,
    pub control_ata_dev: control_ata_dev,
    pub escb: empty_scb,
    pub link_adm: initiate_link_adm,
    pub cp_mem: copy_memory,
    pub abort_task: abort_task,
    pub clear_nexus: clear_nexus,
    pub ssp_tmf: initiate_ssp_tmf,
}

// ---------- Done List ----------
// The done list entry opcode field is defined below.
// The mnemonic encoding and meaning is as follows:
// TC - Task Complete, status was received and acknowledged
// TF - Task Failed, indicates an error prior to receiving acknowledgment
// for the command:
// - no conn,
// - NACK or R_ERR received in response to this command,
// - credit blocked or not available, or in the case of SMP request,
// - no SMP response was received.
// In these four cases it is known that the target didn't receive the
// command.
// TI - Task Interrupted, error after the command was acknowledged.  It is
// known that the command was received by the target.
// TU - Task Unacked, command was transmitted but neither ACK (R_OK) nor NAK
// (R_ERR) was received due to loss of signal, broken connection, loss of
// dword sync or other reason.  The application client should send the
// appropriate task query.
// TA - Task Aborted, see TF.
// _RESP - The completion includes an empty buffer containing status.
// TO - Timeout.
//
pub const TC_NO_ERROR: c_uint = 0x00;
pub const TC_UNDERRUN: c_uint = 0x01;
pub const TC_OVERRUN: c_uint = 0x02;
pub const TF_OPEN_TO: c_uint = 0x03;
pub const TF_OPEN_REJECT: c_uint = 0x04;
pub const TI_BREAK: c_uint = 0x05;
pub const TI_PROTO_ERR: c_uint = 0x06;
pub const TC_SSP_RESP: c_uint = 0x07;
pub const TI_PHY_DOWN: c_uint = 0x08;
pub const TF_PHY_DOWN: c_uint = 0x09;
pub const TC_LINK_ADM_RESP: c_uint = 0x0a;
pub const TC_CSMI: c_uint = 0x0b;
pub const TC_ATA_RESP: c_uint = 0x0c;
pub const TU_PHY_DOWN: c_uint = 0x0d;
pub const TU_BREAK: c_uint = 0x0e;
pub const TI_SATA_TO: c_uint = 0x0f;
pub const TI_NAK: c_uint = 0x10;
pub const TC_CONTROL_PHY: c_uint = 0x11;
pub const TF_BREAK: c_uint = 0x12;
pub const TC_RESUME: c_uint = 0x13;
pub const TI_ACK_NAK_TO: c_uint = 0x14;
pub const TF_SMPRSP_TO: c_uint = 0x15;
pub const TF_SMP_XMIT_RCV_ERR: c_uint = 0x16;
pub const TC_PARTIAL_SG_LIST: c_uint = 0x17;
pub const TU_ACK_NAK_TO: c_uint = 0x18;
pub const TU_SATA_TO: c_uint = 0x19;
pub const TF_NAK_RECV: c_uint = 0x1a;
pub const TA_I_T_NEXUS_LOSS: c_uint = 0x1b;
pub const TC_ATA_R_ERR_RECV: c_uint = 0x1c;
pub const TF_TMF_NO_CTX: c_uint = 0x1d;
pub const TA_ON_REQ: c_uint = 0x1e;
pub const TF_TMF_NO_TAG: c_uint = 0x1f;
pub const TF_TMF_TAG_FREE: c_uint = 0x20;
pub const TF_TMF_TASK_DONE: c_uint = 0x21;
pub const TF_TMF_NO_CONN_HANDLE: c_uint = 0x22;
pub const TC_TASK_CLEARED: c_uint = 0x23;
pub const TI_SYNCS_RECV: c_uint = 0x24;
pub const TU_SYNCS_RECV: c_uint = 0x25;
pub const TF_IRTT_TO: c_uint = 0x26;
pub const TF_NO_SMP_CONN: c_uint = 0x27;
pub const TF_IU_SHORT: c_uint = 0x28;
pub const TF_DATA_OFFS_ERR: c_uint = 0x29;
pub const TF_INV_CONN_HANDLE: c_uint = 0x2a;
pub const TF_REQUESTED_N_PENDING: c_uint = 0x2b;
// 0xc1 - 0xc7: empty buffer received,
//
// This is the ESCB mask
pub const ESCB_RECVD: c_uint = 0xC0;
// This struct done_list_struct defines the done list entry.
// All fields are LE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct done_list_struct {
    pub /: *mut *mut __le16 index; / aka transaction context,
    pub opcode: u8,
    pub status_block: [u8; 4],
    pub /: *mut *mut u8 toggle; / bit 0,
pub const DL_TOGGLE_MASK: c_uint = 0x01;
// C attribute field omitted
// ---------- PHYS ----------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct asd_phy {
    pub sas_phy: asd_sas_phy,
    pub /: *mut *mut *mut asd_phy_desc phy_desc; / hw profile,
    pub identify_frame: *mut sas_identify_frame,
    pub id_frm_tok: *mut asd_dma_tok,
    pub asd_port: *mut asd_port,
    pub frame_rcvd: [u8; ASD_EDB_SIZE],
}

// Define this to 0 if you do not want NOTIFY (ENABLE SPINIP) sent.
// Default: 0x10 (it's a mask)
//
pub const ASD_NOTIFY_ENABLE_SPINUP: c_uint = 0x10;
// If enabled, set this to the interval between transmission
// of NOTIFY (ENABLE SPINUP). In units of 200 us.
//
pub const ASD_NOTIFY_TIMEOUT: c_int = 2500;
// Initial delay after OOB, before we transmit NOTIFY (ENABLE SPINUP).
// If 0, transmit immediately. In milliseconds.
//
pub const ASD_NOTIFY_DOWN_COUNT: c_int = 0;
// Device present timer timeout constant, 10 ms.
pub const ASD_DEV_PRESENT_TIMEOUT: c_uint = 0x2710;
pub const ASD_SATA_INTERLOCK_TIMEOUT: c_int = 0;
// How long to wait before shutting down an STP connection, unless
// an STP target sent frame(s). 50 usec.
// IGNORED by the sequencer (i.e. value 0 always).
//
pub const ASD_STP_SHUTDOWN_TIMEOUT: c_uint = 0x0;
// ATA soft reset timer timeout. 5 usec.
pub const ASD_SRST_ASSERT_TIMEOUT: c_uint = 0x05;
// 31 sec
pub const ASD_RCV_FIS_TIMEOUT: c_uint = 0x01D905C0;
pub const ASD_ONE_MILLISEC_TIMEOUT: c_uint = 0x03e8;
// COMINIT timer
pub const ASD_TEN_MILLISEC_TIMEOUT: c_uint = 0x2710;

// 1 sec
pub const ASD_SMP_RCV_TIMEOUT: c_uint = 0x000F4240;
