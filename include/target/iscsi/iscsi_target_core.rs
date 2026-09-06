//! Automatically rewritten from C Header to Rust Module
//! Source: include/target/iscsi/iscsi_target_core.h
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

pub const ISCSI_MAX_DATASN_MISSING_COUNT: c_int = 16;
pub const ISCSI_TX_THREAD_TCP_TIMEOUT: c_int = 2;
pub const ISCSI_RX_THREAD_TCP_TIMEOUT: c_int = 2;
pub const SECONDS_FOR_ASYNC_LOGOUT: c_int = 10;
pub const SECONDS_FOR_ASYNC_TEXT: c_int = 10;
pub const SECONDS_FOR_LOGOUT_COMP: c_int = 15;

pub const ISCSIT_MIN_TAGS: c_int = 16;
pub const ISCSIT_EXTRA_TAGS: c_int = 8;
pub const ISCSIT_TCP_BACKLOG: c_int = 256;

pub const ISCSI_IQN_LEN: c_int = 224;

// struct iscsi_node_attrib sanity values
pub const NA_DATAOUT_TIMEOUT: c_int = 3;
pub const NA_DATAOUT_TIMEOUT_MAX: c_int = 60;
pub const NA_DATAOUT_TIMEOUT_MIX: c_int = 2;
pub const NA_DATAOUT_TIMEOUT_RETRIES: c_int = 5;
pub const NA_DATAOUT_TIMEOUT_RETRIES_MAX: c_int = 15;
pub const NA_DATAOUT_TIMEOUT_RETRIES_MIN: c_int = 1;
pub const NA_NOPIN_TIMEOUT: c_int = 15;
pub const NA_NOPIN_TIMEOUT_MAX: c_int = 60;
pub const NA_NOPIN_TIMEOUT_MIN: c_int = 3;
pub const NA_NOPIN_RESPONSE_TIMEOUT: c_int = 30;
pub const NA_NOPIN_RESPONSE_TIMEOUT_MAX: c_int = 60;
pub const NA_NOPIN_RESPONSE_TIMEOUT_MIN: c_int = 3;
pub const NA_RANDOM_DATAIN_PDU_OFFSETS: c_int = 0;
pub const NA_RANDOM_DATAIN_SEQ_OFFSETS: c_int = 0;
pub const NA_RANDOM_R2T_OFFSETS: c_int = 0;
// struct iscsi_tpg_attrib sanity values
pub const TA_AUTHENTICATION: c_int = 1;
pub const TA_LOGIN_TIMEOUT: c_int = 15;
pub const TA_LOGIN_TIMEOUT_MAX: c_int = 30;
pub const TA_LOGIN_TIMEOUT_MIN: c_int = 5;
pub const TA_GENERATE_NODE_ACLS: c_int = 0;
pub const TA_DEFAULT_CMDSN_DEPTH: c_int = 64;
pub const TA_DEFAULT_CMDSN_DEPTH_MAX: c_int = 512;
pub const TA_DEFAULT_CMDSN_DEPTH_MIN: c_int = 1;
pub const TA_CACHE_DYNAMIC_ACLS: c_int = 0;
// Enabled by default in demo mode (generic_node_acls=1)
pub const TA_DEMO_MODE_WRITE_PROTECT: c_int = 1;
// Disabled by default in production mode w/ explict ACLs
pub const TA_PROD_MODE_WRITE_PROTECT: c_int = 0;
pub const TA_DEMO_MODE_DISCOVERY: c_int = 1;
pub const TA_DEFAULT_ERL: c_int = 0;
pub const TA_CACHE_CORE_NPS: c_int = 0;
// T10 protection information disabled by default
pub const TA_DEFAULT_T10_PI: c_int = 0;
pub const TA_DEFAULT_FABRIC_PROT_TYPE: c_int = 0;
// TPG status needs to be enabled to return sendtargets discovery endpoint info
pub const TA_DEFAULT_TPG_ENABLED_SENDTARGETS: c_int = 1;
//
// Used to control the sending of keys with optional to respond state bit,
// as a workaround for non RFC compliant initiators,that do not propose,
// nor respond to specific keys required for login to complete.
//
// See iscsi_check_proposer_for_optional_reply() for more details.
//
pub const TA_DEFAULT_LOGIN_KEYS_WORKAROUND: c_int = 1;
pub const ISCSI_IOV_DATA_BUFFER: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iscsit_transport_type {
    ISCSI_TCP				= 0,
    ISCSI_SCTP_TCP				= 1,
    ISCSI_SCTP_UDP				= 2,
    ISCSI_IWARP_TCP				= 3,
    ISCSI_IWARP_SCTP			= 4,
    ISCSI_INFINIBAND			= 5,
    ISCSI_CXGBIT				= 6,
}

// RFC-3720 7.1.4  Standard Connection State Diagram for a Target
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum target_conn_state_table {
    TARG_CONN_STATE_FREE			= 0x1,
    TARG_CONN_STATE_XPT_UP			= 0x3,
    TARG_CONN_STATE_IN_LOGIN		= 0x4,
    TARG_CONN_STATE_LOGGED_IN		= 0x5,
    TARG_CONN_STATE_IN_LOGOUT		= 0x6,
    TARG_CONN_STATE_LOGOUT_REQUESTED	= 0x7,
    TARG_CONN_STATE_CLEANUP_WAIT		= 0x8,
}

// RFC-3720 7.3.2  Session State Diagram for a Target
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum target_sess_state_table {
    TARG_SESS_STATE_FREE			= 0x1,
    TARG_SESS_STATE_ACTIVE			= 0x2,
    TARG_SESS_STATE_LOGGED_IN		= 0x3,
    TARG_SESS_STATE_FAILED			= 0x4,
    TARG_SESS_STATE_IN_CONTINUE		= 0x5,
}

// struct iscsi_data_count->type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum data_count_type {
    ISCSI_RX_DATA	= 1,
    ISCSI_TX_DATA	= 2,
}

// struct iscsi_datain_req->dr_complete
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum datain_req_comp_table {
    DATAIN_COMPLETE_NORMAL			= 1,
    DATAIN_COMPLETE_WITHIN_COMMAND_RECOVERY = 2,
    DATAIN_COMPLETE_CONNECTION_RECOVERY	= 3,
}

// struct iscsi_datain_req->recovery
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum datain_req_rec_table {
    DATAIN_WITHIN_COMMAND_RECOVERY		= 1,
    DATAIN_CONNECTION_RECOVERY		= 2,
}

// struct iscsi_portal_group->state
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tpg_state_table {
    TPG_STATE_FREE				= 0,
    TPG_STATE_ACTIVE			= 1,
    TPG_STATE_INACTIVE			= 2,
    TPG_STATE_COLD_RESET			= 3,
}

// struct iscsi_tiqn->tiqn_state
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tiqn_state_table {
    TIQN_STATE_ACTIVE			= 1,
    TIQN_STATE_SHUTDOWN			= 2,
}

// struct iscsit_cmd->cmd_flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cmd_flags_table {
    ICF_GOT_LAST_DATAOUT			= 0x00000001,
    ICF_GOT_DATACK_SNACK			= 0x00000002,
    ICF_NON_IMMEDIATE_UNSOLICITED_DATA	= 0x00000004,
    ICF_SENT_LAST_R2T			= 0x00000008,
    ICF_WITHIN_COMMAND_RECOVERY		= 0x00000010,
    ICF_CONTIG_MEMORY			= 0x00000020,
    ICF_ATTACHED_TO_RQUEUE			= 0x00000040,
    ICF_OOO_CMDSN				= 0x00000080,
    ICF_SENDTARGETS_ALL			= 0x00000100,
    ICF_SENDTARGETS_SINGLE			= 0x00000200,
}

// struct iscsit_cmd->i_state
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cmd_i_state_table {
    ISTATE_NO_STATE			= 0,
    ISTATE_NEW_CMD			= 1,
    ISTATE_DEFERRED_CMD		= 2,
    ISTATE_UNSOLICITED_DATA		= 3,
    ISTATE_RECEIVE_DATAOUT		= 4,
    ISTATE_RECEIVE_DATAOUT_RECOVERY	= 5,
    ISTATE_RECEIVED_LAST_DATAOUT	= 6,
    ISTATE_WITHIN_DATAOUT_RECOVERY	= 7,
    ISTATE_IN_CONNECTION_RECOVERY	= 8,
    ISTATE_RECEIVED_TASKMGT		= 9,
    ISTATE_SEND_ASYNCMSG		= 10,
    ISTATE_SENT_ASYNCMSG		= 11,
    ISTATE_SEND_DATAIN		= 12,
    ISTATE_SEND_LAST_DATAIN		= 13,
    ISTATE_SENT_LAST_DATAIN		= 14,
    ISTATE_SEND_LOGOUTRSP		= 15,
    ISTATE_SENT_LOGOUTRSP		= 16,
    ISTATE_SEND_NOPIN		= 17,
    ISTATE_SENT_NOPIN		= 18,
    ISTATE_SEND_REJECT		= 19,
    ISTATE_SENT_REJECT		= 20,
    ISTATE_SEND_R2T			= 21,
    ISTATE_SENT_R2T			= 22,
    ISTATE_SEND_R2T_RECOVERY	= 23,
    ISTATE_SENT_R2T_RECOVERY	= 24,
    ISTATE_SEND_LAST_R2T		= 25,
    ISTATE_SENT_LAST_R2T		= 26,
    ISTATE_SEND_LAST_R2T_RECOVERY	= 27,
    ISTATE_SENT_LAST_R2T_RECOVERY	= 28,
    ISTATE_SEND_STATUS		= 29,
    ISTATE_SEND_STATUS_BROKEN_PC	= 30,
    ISTATE_SENT_STATUS		= 31,
    ISTATE_SEND_STATUS_RECOVERY	= 32,
    ISTATE_SENT_STATUS_RECOVERY	= 33,
    ISTATE_SEND_TASKMGTRSP		= 34,
    ISTATE_SENT_TASKMGTRSP		= 35,
    ISTATE_SEND_TEXTRSP		= 36,
    ISTATE_SENT_TEXTRSP		= 37,
    ISTATE_SEND_NOPIN_WANT_RESPONSE	= 38,
    ISTATE_SENT_NOPIN_WANT_RESPONSE	= 39,
    ISTATE_SEND_NOPIN_NO_RESPONSE	= 40,
    ISTATE_REMOVE			= 41,
    ISTATE_FREE			= 42,
}

// Used for iscsi_recover_cmdsn() return values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum recover_cmdsn_ret_table {
    CMDSN_ERROR_CANNOT_RECOVER	= -1,
    CMDSN_NORMAL_OPERATION		= 0,
    CMDSN_LOWER_THAN_EXP		= 1,
    CMDSN_HIGHER_THAN_EXP		= 2,
    CMDSN_MAXCMDSN_OVERRUN		= 3,
}

// Used for iscsi_handle_immediate_data() return values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum immedate_data_ret_table {
    IMMEDIATE_DATA_CANNOT_RECOVER	= -1,
    IMMEDIATE_DATA_NORMAL_OPERATION = 0,
    IMMEDIATE_DATA_ERL1_CRC_FAILURE = 1,
}

// Used for iscsi_decide_dataout_action() return values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dataout_action_ret_table {
    DATAOUT_CANNOT_RECOVER		= -1,
    DATAOUT_NORMAL			= 0,
    DATAOUT_SEND_R2T		= 1,
    DATAOUT_SEND_TO_TRANSPORT	= 2,
    DATAOUT_WITHIN_COMMAND_RECOVERY = 3,
}

// Used for struct iscsi_node_auth->naf_flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum naf_flags_table {
    NAF_USERID_SET			= 0x01,
    NAF_PASSWORD_SET		= 0x02,
    NAF_USERID_IN_SET		= 0x04,
    NAF_PASSWORD_IN_SET		= 0x08,
}

// Used by various struct timer_list to manage iSCSI specific state
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iscsi_timer_flags_table {
    ISCSI_TF_RUNNING		= 0x01,
    ISCSI_TF_STOP			= 0x02,
    ISCSI_TF_EXPIRED		= 0x04,
}

// Used for struct iscsi_np->np_flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum np_flags_table {
    NPF_IP_NETWORK		= 0x00,
}

// Used for struct iscsi_np->np_thread_state
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum np_thread_state_table {
    ISCSI_NP_THREAD_ACTIVE		= 1,
    ISCSI_NP_THREAD_INACTIVE	= 2,
    ISCSI_NP_THREAD_RESET		= 3,
    ISCSI_NP_THREAD_SHUTDOWN	= 4,
    ISCSI_NP_THREAD_EXIT		= 5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_conn_ops {
    pub /: *mut *mut u8 HeaderDigest; / [0,1] == [None,CRC32C],
    pub /: *mut *mut u8 DataDigest; / [0,1] == [None,CRC32C],
    pub /: *mut *mut *mut *mut u32 MaxRecvDataSegmentLength; / [512..224-1],
    pub /: *mut *mut *mut *mut u32 MaxXmitDataSegmentLength; / [512..224-1],
//
// iSER specific connection parameters
//
    pub /: *mut *mut *mut *mut u32 InitiatorRecvDataSegmentLength; / [512..224-1],
    pub /: *mut *mut *mut *mut u32 TargetRecvDataSegmentLength; / [512..224-1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_sess_ops {
    pub InitiatorName: [c_char; ISCSI_IQN_LEN],
    pub InitiatorAlias: [c_char; 256],
    pub TargetName: [c_char; ISCSI_IQN_LEN],
    pub TargetAlias: [c_char; 256],
    pub TargetAddress: [c_char; 256],
    pub /: *mut *mut u16 TargetPortalGroupTag; / [0..65535],
    pub /: *mut *mut u16 MaxConnections; / [1..65535],
    pub /: *mut *mut u8 InitialR2T; / [0,1] == [No,Yes],
    pub /: *mut *mut u8 ImmediateData; / [0,1] == [No,Yes],
    pub /: *mut *mut *mut *mut u32 MaxBurstLength; / [512..224-1],
    pub /: *mut *mut *mut *mut u32 FirstBurstLength; / [512..224-1],
    pub /: *mut *mut u16 DefaultTime2Wait; / [0..3600],
    pub /: *mut *mut u16 DefaultTime2Retain; / [0..3600],
    pub /: *mut *mut u16 MaxOutstandingR2T; / [1..65535],
    pub /: *mut *mut u8 DataPDUInOrder; / [0,1] == [No,Yes],
    pub /: *mut *mut u8 DataSequenceInOrder; / [0,1] == [No,Yes],
    pub /: *mut *mut u8 ErrorRecoveryLevel; / [0..2],
    pub [Normal,Discovery]*/: *mut *mut u8 SessionType; / [0,1] ==,
//
// iSER specific session parameters
//
    pub /: *mut *mut u8 RDMAExtensions; / [0,1] == [No,Yes],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_queue_req {
    pub state: c_int,
    pub cmd: *mut iscsit_cmd,
    pub qr_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_param_list {
    pub iser: bool,
    pub param_list: list_head,
    pub extra_response_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_datain_req {
    pub dr_complete: datain_req_comp_table,
    pub generate_recovery_values: c_int,
    pub recovery: datain_req_rec_table,
    pub begrun: u32,
    pub runlength: u32,
    pub data_length: u32,
    pub data_offset: u32,
    pub data_sn: u32,
    pub next_burst_len: u32,
    pub read_data_done: u32,
    pub seq_send_order: u32,
    pub cmd_datain_node: list_head,
    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_ooo_cmdsn {
    pub cid: u16,
    pub batch_count: u32,
    pub cmdsn: u32,
    pub exp_cmdsn: u32,
    pub cmd: *mut iscsit_cmd,
    pub ooo_list: list_head,
    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_datain {
    pub flags: u8,
    pub data_sn: u32,
    pub length: u32,
    pub offset: u32,
    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_r2t {
    pub seq_complete: c_int,
    pub recovery_r2t: c_int,
    pub sent_r2t: c_int,
    pub r2t_sn: u32,
    pub offset: u32,
    pub targ_xfer_tag: u32,
    pub xfer_len: u32,
    pub r2t_list: list_head,
    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsit_cmd {
    pub dataout_timer_flags: iscsi_timer_flags_table,
// DataOUT timeout retries
    pub dataout_timeout_retries: u8,
// Within command recovery count
    pub error_recovery_count: u8,
// iSCSI dependent state for out or order CmdSNs
    pub deferred_i_state: cmd_i_state_table,
// iSCSI dependent state
    pub i_state: cmd_i_state_table,
// Command is an immediate command (ISCSI_OP_IMMEDIATE set)
    pub immediate_cmd: u8,
// Immediate data present
    pub immediate_data: u8,
// iSCSI Opcode
    pub iscsi_opcode: u8,
// iSCSI Response Code
    pub iscsi_response: u8,
// Logout reason when iscsi_opcode == ISCSI_INIT_LOGOUT_CMND
    pub logout_reason: u8,
// Logout response code when iscsi_opcode == ISCSI_INIT_LOGOUT_CMND
    pub logout_response: u8,
// MaxCmdSN has been incremented
    pub maxcmdsn_inc: u8,
// Immediate Unsolicited Dataout
    pub unsolicited_data: u8,
// Reject reason code
    pub reject_reason: u8,
// CID contained in logout PDU when opcode == ISCSI_INIT_LOGOUT_CMND
    pub logout_cid: u16,
// Command flags
    pub cmd_flags: cmd_flags_table,
// Initiator Task Tag assigned from Initiator
    pub init_task_tag: itt_t,
// Target Transfer Tag assigned from Target
    pub targ_xfer_tag: u32,
// CmdSN assigned from Initiator
    pub cmd_sn: u32,
// ExpStatSN assigned from Initiator
    pub exp_stat_sn: u32,
// StatSN assigned to this ITT
    pub stat_sn: u32,
// DataSN Counter
    pub data_sn: u32,
// R2TSN Counter
    pub r2t_sn: u32,
// Last DataSN acknowledged via DataAck SNACK
    pub acked_data_sn: u32,
// Used for echoing NOPOUT ping data
    pub buf_ptr_size: u32,
// Used to store DataDigest
    pub data_crc: u32,
// Counter for MaxOutstandingR2T
    pub outstanding_r2ts: u32,
// Next R2T Offset when DataSequenceInOrder=Yes
    pub r2t_offset: u32,
// Iovec current and orig count for iscsit_cmd->iov_data
    pub iov_data_count: u32,
    pub orig_iov_data_count: u32,
// Number of miscellaneous iovecs used for IP stack calls
    pub iov_misc_count: u32,
// Number of struct iscsi_pdu in struct iscsit_cmd->pdu_list
    pub pdu_count: u32,
// Next struct iscsi_pdu to send in struct iscsit_cmd->pdu_list
    pub pdu_send_order: u32,
// Current struct iscsi_pdu in struct iscsit_cmd->pdu_list
    pub pdu_start: u32,
// Next struct iscsi_seq to send in struct iscsit_cmd->seq_list
    pub seq_send_order: u32,
// Number of struct iscsi_seq in struct iscsit_cmd->seq_list
    pub seq_count: u32,
// Current struct iscsi_seq in struct iscsit_cmd->seq_list
    pub seq_no: u32,
// Lowest offset in current DataOUT sequence
    pub seq_start_offset: u32,
// Highest offset in current DataOUT sequence
    pub seq_end_offset: u32,
// Total size in bytes received so far of READ data
    pub read_data_done: u32,
// Total size in bytes received so far of WRITE data
    pub write_data_done: u32,
// Counter for FirstBurstLength key
    pub first_burst_len: u32,
// Counter for MaxBurstLength key
    pub next_burst_len: u32,
// Transfer size used for IP stack calls
    pub tx_size: u32,
// Buffer used for various purposes
    pub buf_ptr: *mut c_void,
// Used by SendTargets=[iqn.,eui.] discovery
    pub text_in_ptr: *mut c_void,
// See include/linux/dma-mapping.h
    pub data_direction: dma_data_direction,
// iSCSI PDU Header + CRC
    pub ISCSI_CRC_LEN]: unsigned char pdu[ISCSI_HDR_LEN +,
// Number of times struct iscsit_cmd is present in immediate queue
    pub immed_queue_count: core::sync::atomic::AtomicI32,
    pub response_queue_count: core::sync::atomic::AtomicI32,
    pub datain_lock: spinlock_t,
    pub dataout_timeout_lock: spinlock_t,
// spinlock for protecting struct iscsit_cmd->i_state
    pub istate_lock: spinlock_t,
// spinlock for adding within command recovery entries
    pub error_lock: spinlock_t,
// spinlock for adding R2Ts
    pub r2t_lock: spinlock_t,
// DataIN List
    pub datain_list: list_head,
// R2T List
    pub cmd_r2t_list: list_head,
// Timer for DataOUT
    pub dataout_timer: timer_list,
// Iovecs for SCSI data payload RX/TX w/ kernel level sockets
    pub iov_data: *mut kvec,
    pub overflow_buf: *mut c_void,
// Iovecs for miscellaneous purposes
pub const ISCSI_MISC_IOVECS: c_int = 5;
    pub iov_misc: [kvec; ISCSI_MISC_IOVECS],
// Array of struct iscsi_pdu used for DataPDUInOrder=No
    pub pdu_list: *mut iscsi_pdu,
// Current struct iscsi_pdu used for DataPDUInOrder=No
    pub pdu_ptr: *mut iscsi_pdu,
// Array of struct iscsi_seq used for DataSequenceInOrder=No
    pub seq_list: *mut iscsi_seq,
// Current struct iscsi_seq used for DataSequenceInOrder=No
    pub seq_ptr: *mut iscsi_seq,
// TMR Request when iscsi_opcode == ISCSI_OP_SCSI_TMFUNC
    pub tmr_req: *mut iscsi_tmr_req,
// Connection this command is alligient to
    pub conn: *mut iscsit_conn,
// Pointer to connection recovery entry
    pub cr: *mut iscsi_conn_recovery,
// Session the command is part of,  used for connection recovery
    pub sess: *mut iscsit_session,
// list_head for connection list
    pub i_conn_node: list_head,
// The TCM I/O descriptor that is accessed via container_of()
    pub se_cmd: se_cmd,
// Sense buffer that will be mapped into outgoing status
    pub sense_buffer: [c_uchar; ISCSI_SENSE_BUFFER_LEN],
    pub padding: u32,
    pub pad_bytes: [u8; 4],
    pub first_data_sg: *mut scatterlist,
    pub first_data_sg_off: u32,
    pub kmapped_nents: u32,
    pub sense_reason: sense_reason_t,
    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_tmr_req {
    pub task_reassign:1: bool,
    pub exp_data_sn: u32,
    pub ref_cmd: *mut iscsit_cmd,
    pub conn_recovery: *mut iscsi_conn_recovery,
    pub se_tmr_req: *mut se_tmr_req,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsit_conn {
    pub queues_wq: wait_queue_head_t,
// Authentication Successful for this connection
    pub auth_complete: u8,
// State connection is currently in
    pub conn_state: u8,
    pub conn_logout_reason: u8,
    pub network_transport: u8,
    pub nopin_timer_flags: iscsi_timer_flags_table,
    pub nopin_response_timer_flags: iscsi_timer_flags_table,
// Used to know what thread encountered a transport failure
    pub which_thread: u8,
// connection id assigned by the Initiator
    pub cid: u16,
// Remote TCP Port
    pub login_port: u16,
    pub net_size: c_int,
    pub login_family: c_int,
    pub auth_id: u32,
    pub conn_flags: u32,
// Used for iscsi_tx_login_rsp()
    pub login_itt: itt_t,
    pub exp_statsn: u32,
// Per connection status sequence number
    pub stat_sn: u32,
    pub login_sockaddr: sockaddr_storage,
    pub local_sockaddr: sockaddr_storage,
    pub conn_usage_count: c_int,
    pub conn_waiting_on_uc: c_int,
    pub check_immediate_queue: core::sync::atomic::AtomicI32,
    pub conn_logout_remove: core::sync::atomic::AtomicI32,
    pub connection_exit: core::sync::atomic::AtomicI32,
    pub connection_recovery: core::sync::atomic::AtomicI32,
    pub connection_reinstatement: core::sync::atomic::AtomicI32,
    pub connection_wait_rcfr: core::sync::atomic::AtomicI32,
    pub sleep_on_conn_wait_comp: core::sync::atomic::AtomicI32,
    pub transport_failed: core::sync::atomic::AtomicI32,
    pub conn_post_wait_comp: completion,
    pub conn_wait_comp: completion,
    pub conn_wait_rcfr_comp: completion,
    pub conn_waiting_on_uc_comp: completion,
    pub conn_logout_comp: completion,
    pub tx_half_close_comp: completion,
    pub rx_half_close_comp: completion,
// socket used by this connection
    pub sock: *mut socket,
    pub ): *mut *mut void (orig_data_ready)(struct sock,
    pub ): *mut *mut void (orig_state_change)(struct sock,
pub const LOGIN_FLAGS_READY: c_int = 0;
pub const LOGIN_FLAGS_INITIAL_PDU: c_int = 1;
pub const LOGIN_FLAGS_READ_ACTIVE: c_int = 2;
pub const LOGIN_FLAGS_WRITE_ACTIVE: c_int = 3;
pub const LOGIN_FLAGS_CLOSED: c_int = 4;
pub const LOGIN_FLAGS_WORKER_RUNNING: c_int = 5;
    pub login_flags: c_ulong,
    pub login_work: delayed_work,
    pub login: *mut iscsi_login,
    pub nopin_timer: timer_list,
    pub nopin_response_timer: timer_list,
    pub login_timer: timer_list,
    pub login_kworker: *mut task_struct,
// Spinlock used for add/deleting cmd's from conn_cmd_list
    pub cmd_lock: spinlock_t,
    pub conn_usage_lock: spinlock_t,
    pub immed_queue_lock: spinlock_t,
    pub nopin_timer_lock: spinlock_t,
    pub response_queue_lock: spinlock_t,
    pub state_lock: spinlock_t,
    pub login_timer_lock: spinlock_t,
    pub login_worker_lock: spinlock_t,
// Used for scheduling TX and RX connection kthreads
    pub conn_cpumask: cpumask_var_t,
    pub allowed_cpumask: cpumask_var_t,
    pub conn_rx_reset_cpumask:1: c_uint,
    pub conn_tx_reset_cpumask:1: c_uint,
// list_head of struct iscsit_cmd for this connection
    pub conn_cmd_list: list_head,
    pub immed_queue_list: list_head,
    pub response_queue_list: list_head,
    pub conn_ops: *mut iscsi_conn_ops,
    pub conn_login: *mut iscsi_login,
    pub conn_transport: *mut iscsit_transport,
    pub param_list: *mut iscsi_param_list,
// Used for per connection auth state machine
    pub auth_protocol: *mut c_void,
    pub context: *mut c_void,
    pub login_thread: *mut iscsi_login_thread_s,
    pub tpg: *mut iscsi_portal_group,
    pub tpg_np: *mut iscsi_tpg_np,
// Pointer to parent session
    pub sess: *mut iscsit_session,
    pub cmd_cnt: *mut target_cmd_counter,
    pub bitmap_id: c_int,
    pub rx_thread_active: c_int,
    pub rx_thread: *mut task_struct,
    pub rx_login_comp: completion,
    pub tx_thread_active: c_int,
    pub tx_thread: *mut task_struct,
// list_head for session connection list
    pub conn_list: list_head,
    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_conn_recovery {
    pub cid: u16,
    pub cmd_count: u32,
    pub maxrecvdatasegmentlength: u32,
    pub maxxmitdatasegmentlength: u32,
    pub ready_for_reallegiance: c_int,
    pub conn_recovery_cmd_list: list_head,
    pub conn_recovery_cmd_lock: spinlock_t,
    pub time2retain_timer: timer_list,
    pub sess: *mut iscsit_session,
    pub cr_list: list_head,
    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsit_session {
    pub initiator_vendor: u8,
    pub isid: [u8; 6],
    pub time2retain_timer_flags: iscsi_timer_flags_table,
    pub version_active: u8,
    pub cid_called: u16,
    pub conn_recovery_count: u16,
    pub tsih: u16,
// state session is currently in
    pub session_state: u32,
// session wide counter: initiator assigned task tag
    pub init_task_tag: itt_t,
// session wide counter: target assigned task tag
    pub targ_xfer_tag: u32,
    pub cmdsn_window: u32,
// protects cmdsn values
    pub cmdsn_mutex: mutex,
// session wide counter: expected command sequence number
    pub exp_cmd_sn: u32,
// session wide counter: maximum allowed command sequence number
    pub max_cmd_sn: core::sync::atomic::AtomicI32,
    pub sess_ooo_cmdsn_list: list_head,
// LIO specific session ID
    pub sid: u32,
    pub auth_type: [c_char; 8],
// unique within the target
    pub session_index: c_int,
// Used for session reference counting
    pub session_usage_count: c_int,
    pub session_waiting_on_uc: c_int,
    pub cmd_pdus: atomic_long_t,
    pub rsp_pdus: atomic_long_t,
    pub tx_data_octets: atomic_long_t,
    pub rx_data_octets: atomic_long_t,
    pub conn_digest_errors: atomic_long_t,
    pub conn_timeout_errors: atomic_long_t,
    pub creation_time: u64,
// Number of active connections
    pub nconn: core::sync::atomic::AtomicI32,
    pub session_continuation: core::sync::atomic::AtomicI32,
    pub session_fall_back_to_erl0: core::sync::atomic::AtomicI32,
    pub session_logout: core::sync::atomic::AtomicI32,
    pub session_reinstatement: core::sync::atomic::AtomicI32,
    pub session_stop_active: core::sync::atomic::AtomicI32,
    pub session_close: core::sync::atomic::AtomicI32,
// connection list
    pub sess_conn_list: list_head,
    pub cr_active_list: list_head,
    pub cr_inactive_list: list_head,
    pub conn_lock: spinlock_t,
    pub cr_a_lock: spinlock_t,
    pub cr_i_lock: spinlock_t,
    pub session_usage_lock: spinlock_t,
    pub ttt_lock: spinlock_t,
    pub async_msg_comp: completion,
    pub reinstatement_comp: completion,
    pub session_wait_comp: completion,
    pub session_waiting_on_uc_comp: completion,
    pub time2retain_timer: timer_list,
    pub sess_ops: *mut iscsi_sess_ops,
    pub se_sess: *mut se_session,
    pub tpg: *mut iscsi_portal_group,
    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_login {
    pub auth_complete: u8,
    pub checked_for_existing: u8,
    pub current_stage: u8,
    pub leading_connection: u8,
    pub first_request: u8,
    pub version_min: u8,
    pub version_max: u8,
    pub login_complete: u8,
    pub login_failed: u8,
    pub zero_tsih: bool,
    pub isid: [c_char; 6],
    pub cmd_sn: u32,
    pub init_task_tag: itt_t,
    pub initial_exp_statsn: u32,
    pub rsp_length: u32,
    pub cid: u16,
    pub tsih: u16,
    pub req: [c_char; ISCSI_HDR_LEN],
    pub rsp: [c_char; ISCSI_HDR_LEN],
    pub req_buf: *mut c_char,
    pub rsp_buf: *mut c_char,
    pub conn: *mut iscsit_conn,
    pub np: *mut iscsi_np,
    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_node_attrib {
    pub authentication: i32,
    pub dataout_timeout: u32,
    pub dataout_timeout_retries: u32,
    pub default_erl: u32,
    pub nopin_timeout: u32,
    pub nopin_response_timeout: u32,
    pub random_datain_pdu_offsets: u32,
    pub random_datain_seq_offsets: u32,
    pub random_r2t_offsets: u32,
    pub tmr_cold_reset: u32,
    pub tmr_warm_reset: u32,
    pub nacl: *mut iscsi_node_acl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_node_auth {
    pub naf_flags: naf_flags_table,
    pub authenticate_target: c_int,
// Used for iscsit_global->discovery_auth,
// set to zero (auth disabled) by default
    pub enforce_discovery_auth: c_int,
pub const MAX_USER_LEN: c_int = 256;
pub const MAX_PASS_LEN: c_int = 256;
    pub userid: [c_char; MAX_USER_LEN],
    pub password: [c_char; MAX_PASS_LEN],
    pub userid_mutual: [c_char; MAX_USER_LEN],
    pub password_mutual: [c_char; MAX_PASS_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_node_stat_grps {
    pub iscsi_sess_stats_group: config_group,
    pub iscsi_conn_stats_group: config_group,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_node_acl {
    pub se_node_acl: se_node_acl,
    pub node_attrib: iscsi_node_attrib,
    pub node_auth: iscsi_node_auth,
    pub node_stat_grps: iscsi_node_stat_grps,
}

extern "C" {
    pub fn container_of(_arg: se_nacl, iscsi_node_acl: struct, _arg: se_node_acl) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_tpg_attrib {
    pub authentication: u32,
    pub login_timeout: u32,
    pub generate_node_acls: u32,
    pub cache_dynamic_acls: u32,
    pub default_cmdsn_depth: u32,
    pub demo_mode_write_protect: u32,
    pub prod_mode_write_protect: u32,
    pub demo_mode_discovery: u32,
    pub default_erl: u32,
    pub t10_pi: u8,
    pub fabric_prot_type: u32,
    pub tpg_enabled_sendtargets: u32,
    pub login_keys_workaround: u32,
    pub tpg: *mut iscsi_portal_group,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_np {
    pub np_network_transport: c_int,
    pub np_ip_proto: c_int,
    pub np_sock_type: c_int,
    pub np_thread_state: np_thread_state_table,
    pub enabled: bool,
    pub np_reset_count: core::sync::atomic::AtomicI32,
    pub np_exports: u32,
    pub np_flags: np_flags_table,
    pub np_thread_lock: spinlock_t,
    pub np_restart_comp: completion,
    pub np_socket: *mut socket,
    pub np_sockaddr: sockaddr_storage,
    pub np_thread: *mut task_struct,
    pub np_context: *mut c_void,
    pub np_transport: *mut iscsit_transport,
    pub np_list: list_head,
    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_tpg_np {
    pub tpg_np: *mut iscsi_np,
    pub tpg: *mut iscsi_portal_group,
    pub tpg_np_parent: *mut iscsi_tpg_np,
    pub tpg_np_list: list_head,
    pub tpg_np_child_list: list_head,
    pub tpg_np_parent_list: list_head,
    pub se_tpg_np: se_tpg_np,
    pub tpg_np_parent_lock: spinlock_t,
    pub tpg_np_comp: completion,
    pub tpg_np_kref: kref,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_portal_group {
    pub tpg_chap_id: c_uchar,
// TPG State
    pub tpg_state: tpg_state_table,
// Target Portal Group Tag
    pub tpgt: u16,
// Id assigned to target sessions
    pub ntsih: u16,
// Number of active sessions
    pub nsessions: u32,
// Number of Network Portals available for this TPG
    pub num_tpg_nps: u32,
// Per TPG LIO specific session ID.
    pub sid: u32,
// Spinlock for adding/removing Network Portals
    pub tpg_np_lock: spinlock_t,
    pub tpg_state_lock: spinlock_t,
    pub tpg_se_tpg: se_portal_group,
    pub tpg_access_lock: mutex,
    pub np_login_sem: semaphore,
    pub tpg_attrib: iscsi_tpg_attrib,
    pub tpg_demo_auth: iscsi_node_auth,
// Pointer to default list of iSCSI parameters for TPG
    pub param_list: *mut iscsi_param_list,
    pub tpg_tiqn: *mut iscsi_tiqn,
    pub tpg_gnp_list: list_head,
    pub tpg_list: list_head,
    pub ____cacheline_aligned: },
    pub tpg_se_tpg): return container_of(se_tpg, struct iscsi_portal_group,,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_wwn_stat_grps {
    pub iscsi_stat_group: config_group,
    pub iscsi_instance_group: config_group,
    pub iscsi_sess_err_group: config_group,
    pub iscsi_tgt_attr_group: config_group,
    pub iscsi_login_stats_group: config_group,
    pub iscsi_logout_stats_group: config_group,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_tiqn {
    pub tiqn: [c_uchar; ISCSI_IQN_LEN],
    pub tiqn_state: tiqn_state_table,
    pub tiqn_access_count: c_int,
    pub tiqn_active_tpgs: u32,
    pub tiqn_ntpgs: u32,
    pub tiqn_num_tpg_nps: u32,
    pub tiqn_nsessions: u32,
    pub tiqn_list: list_head,
    pub tiqn_tpg_list: list_head,
    pub tiqn_state_lock: spinlock_t,
    pub tiqn_tpg_lock: spinlock_t,
    pub tiqn_wwn: se_wwn,
    pub tiqn_stat_grps: iscsi_wwn_stat_grps,
    pub tiqn_index: c_int,
    pub sess_err_stats: iscsi_sess_err_stats,
    pub login_stats: iscsi_login_stats,
    pub logout_stats: iscsi_logout_stats,
    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsit_global {
// In core shutdown
    pub in_shutdown: u32,
    pub active_ts: u32,
// Unique identifier used for the authentication daemon
    pub auth_id: u32,
    pub inactive_ts: u32,
pub const ISCSIT_BITMAP_BITS: c_int = 262144;
// Thread Set bitmap pointer
    pub ts_bitmap: *mut c_ulong,
    pub ts_bitmap_lock: spinlock_t,
    pub allowed_cpumask: cpumask_var_t,
// Used for iSCSI discovery session authentication
    pub discovery_acl: iscsi_node_acl,
    pub discovery_tpg: *mut iscsi_portal_group,
}
