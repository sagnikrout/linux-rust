//! Automatically rewritten from C Header to Rust Module
//! Source: include/target/target_core_base.h
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
// Maximum size of a CDB that can be stored in se_cmd without allocating
// memory dynamically for the CDB.
//
pub const TCM_MAX_COMMAND_SIZE: c_int = 32;
//
// From include/scsi/scsi_cmnd.h:SCSI_SENSE_BUFFERSIZE, currently
// defined 96, but the real limit is 252 (or 260 including the header)
//
pub const TRANSPORT_SENSE_BUFFER: c_int = 96;
// Used by transport_send_check_condition_and_sense()
pub const SPC_SENSE_KEY_OFFSET: c_int = 2;
pub const SPC_ADD_SENSE_LEN_OFFSET: c_int = 7;
pub const SPC_DESC_TYPE_OFFSET: c_int = 8;
pub const SPC_ADDITIONAL_DESC_LEN_OFFSET: c_int = 9;
pub const SPC_VALIDITY_OFFSET: c_int = 10;
pub const SPC_ASC_KEY_OFFSET: c_int = 12;
pub const SPC_ASCQ_KEY_OFFSET: c_int = 13;
pub const TRANSPORT_IQN_LEN: c_int = 224;
// Used by target_core_store_alua_lu_gp() and target_core_alua_lu_gp_show_attr_members()
pub const LU_GROUP_NAME_BUF: c_int = 256;
// Used by core_alua_store_tg_pt_gp_info() and target_core_alua_tg_pt_gp_show_attr_members()
pub const TG_PT_GROUP_NAME_BUF: c_int = 256;
// Used to parse VPD into struct t10_vpd
pub const VPD_TMP_BUF_SIZE: c_int = 254;
// Used by transport_generic_cmd_sequencer()
pub const READ_BLOCK_LEN: c_int = 6;
pub const READ_CAP_LEN: c_int = 8;
pub const READ_POSITION_LEN: c_int = 20;
pub const INQUIRY_LEN: c_int = 36;
// Used by transport_get_inquiry_vpd_serial()
pub const INQUIRY_VPD_SERIAL_LEN: c_int = 254;
// Used by transport_get_inquiry_vpd_device_ident()
pub const INQUIRY_VPD_DEVICE_IDENTIFIER_LEN: c_int = 254;
// Attempts before moving from SHORT to LONG
pub const PYX_TRANSPORT_WINDOW_CLOSED_THRESHOLD: c_int = 3;

// struct se_dev_attrib sanity values
// Default max_unmap_lba_count
pub const DA_MAX_UNMAP_LBA_COUNT: c_int = 0;
// Default max_unmap_block_desc_count
pub const DA_MAX_UNMAP_BLOCK_DESC_COUNT: c_int = 0;
// Default unmap_granularity
pub const DA_UNMAP_GRANULARITY_DEFAULT: c_int = 0;
// Default unmap_granularity_alignment
pub const DA_UNMAP_GRANULARITY_ALIGNMENT_DEFAULT: c_int = 0;
// Default unmap_zeroes_data
pub const DA_UNMAP_ZEROES_DATA_DEFAULT: c_int = 0;
// Default max_write_same_len, disabled by default
pub const DA_MAX_WRITE_SAME_LEN: c_int = 0;
// Use a model alias based on the configfs backend device name
pub const DA_EMULATE_MODEL_ALIAS: c_int = 0;
// Emulation for WriteCache and SYNCHRONIZE_CACHE
pub const DA_EMULATE_WRITE_CACHE: c_int = 0;
// Emulation for TASK_ABORTED status (TAS) by default
pub const DA_EMULATE_TAS: c_int = 1;
// Emulation for Thin Provisioning UNMAP using block/blk-lib.c:blkdev_issue_discard()
pub const DA_EMULATE_TPU: c_int = 0;
//
// Emulation for Thin Provisioning WRITE_SAME w/ UNMAP=1 bit using
// block/blk-lib.c:blkdev_issue_discard()
//
pub const DA_EMULATE_TPWS: c_int = 0;
// Emulation for CompareAndWrite (AtomicTestandSet) by default
pub const DA_EMULATE_CAW: c_int = 1;
// Emulation for 3rd Party Copy (ExtendedCopy) by default
pub const DA_EMULATE_3PC: c_int = 1;
// No Emulation for PSCSI by default
pub const DA_EMULATE_ALUA: c_int = 0;
// Emulate SCSI2 RESERVE/RELEASE and Persistent Reservations by default
pub const DA_EMULATE_PR: c_int = 1;
// Emulation for REPORT SUPPORTED OPERATION CODES
pub const DA_EMULATE_RSOC: c_int = 1;
// Enforce SCSI Initiator Port TransportID with 'ISID' for PR
pub const DA_ENFORCE_PR_ISIDS: c_int = 1;
// Force SPC-3 PR Activate Persistence across Target Power Loss
pub const DA_FORCE_PR_APTPL: c_int = 0;
pub const DA_STATUS_MAX_SECTORS_MIN: c_int = 16;
pub const DA_STATUS_MAX_SECTORS_MAX: c_int = 8192;
// By default don't report non-rotating (solid state) medium
pub const DA_IS_NONROT: c_int = 0;
// Queue Algorithm Modifier default for restricted reordering in control mode page
pub const DA_EMULATE_REST_REORD: c_int = 0;
pub const SE_INQUIRY_BUF: c_int = 1024;
pub const SE_MODE_PAGE_BUF: c_int = 512;
pub const SE_SENSE_BUF: c_int = 96;
// Peripheral Device Text Identification Information
pub const PD_TEXT_ID_INFO_LEN: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum target_compl_type {
// Use the fabric driver's default completion type
    TARGET_FABRIC_DEFAULT_COMPL,
// Complete from the backend calling context
    TARGET_DIRECT_COMPL,
// Defer completion to the LIO workqueue
    TARGET_QUEUE_COMPL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum target_submit_type {
// Use the fabric driver's default submission type
    TARGET_FABRIC_DEFAULT_SUBMIT,
// Submit from the calling context
    TARGET_DIRECT_SUBMIT,
// Defer submission to the LIO workqueue
    TARGET_QUEUE_SUBMIT,
}

// struct se_hba->hba_flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hba_flags_table {
    HBA_FLAGS_INTERNAL_USE	= 0x01,
    HBA_FLAGS_PSCSI_MODE	= 0x02,
}

// Special transport agnostic struct se_cmd->t_states
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum transport_state_table {
    TRANSPORT_NO_STATE	= 0,
    TRANSPORT_NEW_CMD	= 1,
    TRANSPORT_WRITE_PENDING	= 3,
    TRANSPORT_PROCESSING	= 5,
    TRANSPORT_COMPLETE	= 6,
    TRANSPORT_ISTATE_PROCESSING = 11,
    TRANSPORT_COMPLETE_QF_WP = 18,
    TRANSPORT_COMPLETE_QF_OK = 19,
    TRANSPORT_COMPLETE_QF_ERR = 20,
}

// Used for struct se_cmd->se_cmd_flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum se_cmd_flags_table {
    SCF_SUPPORTED_SAM_OPCODE		= (1 << 0),
    SCF_TRANSPORT_TASK_SENSE		= (1 << 1),
    SCF_EMULATED_TASK_SENSE			= (1 << 2),
    SCF_SCSI_DATA_CDB			= (1 << 3),
    SCF_SCSI_TMR_CDB			= (1 << 4),
    SCF_FUA					= (1 << 5),
    SCF_SE_LUN_CMD				= (1 << 6),
    SCF_BIDI				= (1 << 7),
    SCF_SENT_CHECK_CONDITION		= (1 << 8),
    SCF_OVERFLOW_BIT			= (1 << 9),
    SCF_UNDERFLOW_BIT			= (1 << 10),
    SCF_ALUA_NON_OPTIMIZED			= (1 << 11),
    SCF_PASSTHROUGH_SG_TO_MEM_NOALLOC	= (1 << 12),
    SCF_COMPARE_AND_WRITE			= (1 << 13),
    SCF_PASSTHROUGH_PROT_SG_TO_MEM_NOALLOC	= (1 << 14),
    SCF_ACK_KREF				= (1 << 15),
    SCF_USE_CPUID				= (1 << 16),
    SCF_TASK_ATTR_SET			= (1 << 17),
    SCF_TREAT_READ_AS_NORMAL		= (1 << 18),
    SCF_TASK_ORDERED_SYNC			= (1 << 19),
    SCF_ATOMIC				= (1 << 20),
}

//
// Used by transport_send_check_condition_and_sense()
// to signal which ASC/ASCQ sense payload should be built.
//
pub type sense_reason_t = unsigned ;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tcm_sense_reason_table {

    TCM_NO_SENSE				= R(0x00),
    TCM_NON_EXISTENT_LUN			= R(0x01),
    TCM_UNSUPPORTED_SCSI_OPCODE		= R(0x02),
    TCM_INCORRECT_AMOUNT_OF_DATA		= R(0x03),
    TCM_UNEXPECTED_UNSOLICITED_DATA		= R(0x04),
    TCM_SERVICE_CRC_ERROR			= R(0x05),
    TCM_SNACK_REJECTED			= R(0x06),
    TCM_SECTOR_COUNT_TOO_MANY		= R(0x07),
    TCM_INVALID_CDB_FIELD			= R(0x08),
    TCM_INVALID_PARAMETER_LIST		= R(0x09),
    TCM_LOGICAL_UNIT_COMMUNICATION_FAILURE	= R(0x0a),
    TCM_UNKNOWN_MODE_PAGE			= R(0x0b),
    TCM_WRITE_PROTECTED			= R(0x0c),
    TCM_CHECK_CONDITION_ABORT_CMD		= R(0x0d),
    TCM_CHECK_CONDITION_UNIT_ATTENTION	= R(0x0e),

    TCM_RESERVATION_CONFLICT		= R(0x10),
    TCM_ADDRESS_OUT_OF_RANGE		= R(0x11),
    TCM_OUT_OF_RESOURCES			= R(0x12),
    TCM_PARAMETER_LIST_LENGTH_ERROR		= R(0x13),
    TCM_MISCOMPARE_VERIFY			= R(0x14),
    TCM_LOGICAL_BLOCK_GUARD_CHECK_FAILED	= R(0x15),
    TCM_LOGICAL_BLOCK_APP_TAG_CHECK_FAILED	= R(0x16),
    TCM_LOGICAL_BLOCK_REF_TAG_CHECK_FAILED	= R(0x17),
    TCM_COPY_TARGET_DEVICE_NOT_REACHABLE	= R(0x18),
    TCM_TOO_MANY_TARGET_DESCS		= R(0x19),
    TCM_UNSUPPORTED_TARGET_DESC_TYPE_CODE	= R(0x1a),
    TCM_TOO_MANY_SEGMENT_DESCS		= R(0x1b),
    TCM_UNSUPPORTED_SEGMENT_DESC_TYPE_CODE	= R(0x1c),
    TCM_INSUFFICIENT_REGISTRATION_RESOURCES	= R(0x1d),
    TCM_LUN_BUSY				= R(0x1e),
    TCM_INVALID_FIELD_IN_COMMAND_IU         = R(0x1f),
    TCM_ALUA_TG_PT_STANDBY			= R(0x20),
    TCM_ALUA_TG_PT_UNAVAILABLE		= R(0x21),
    TCM_ALUA_STATE_TRANSITION		= R(0x22),
    TCM_ALUA_OFFLINE			= R(0x23),

}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum target_sc_flags_table {
    TARGET_SCF_BIDI_OP		= 0x01,
    TARGET_SCF_ACK_KREF		= 0x02,
    TARGET_SCF_UNKNOWN_SIZE		= 0x04,
    TARGET_SCF_USE_CPUID		= 0x08,
}

// fabric independent task management function values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tcm_tmreq_table {
    TMR_ABORT_TASK		= 1,
    TMR_ABORT_TASK_SET	= 2,
    TMR_CLEAR_ACA		= 3,
    TMR_CLEAR_TASK_SET	= 4,
    TMR_LUN_RESET		= 5,
    TMR_TARGET_WARM_RESET	= 6,
    TMR_TARGET_COLD_RESET	= 7,
    TMR_LUN_RESET_PRO	= 0x80,
    TMR_UNKNOWN		= 0xff,
}

// fabric independent task management response values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tcm_tmrsp_table {
    TMR_FUNCTION_FAILED		= 0,
    TMR_FUNCTION_COMPLETE		= 1,
    TMR_TASK_DOES_NOT_EXIST		= 2,
    TMR_LUN_DOES_NOT_EXIST		= 3,
    TMR_TASK_MGMT_FUNCTION_NOT_SUPPORTED	= 4,
    TMR_FUNCTION_REJECTED		= 5,
}

//
// Used for target SCSI statistics
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct t10_alua_lba_map_member {
    pub lba_map_mem_list: list_head,
    pub lba_map_mem_alua_state: c_int,
    pub lba_map_mem_alua_pg_id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct t10_alua_lba_map {
    pub lba_map_first_lba: u64,
    pub lba_map_last_lba: u64,
    pub lba_map_list: list_head,
    pub lba_map_mem_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct t10_alua {
// ALUA Target Port Group ID
    pub alua_tg_pt_gps_counter: u16,
    pub alua_tg_pt_gps_count: u32,
// Referrals support
    pub lba_map_lock: spinlock_t,
    pub lba_map_segment_size: u32,
    pub lba_map_segment_multiplier: u32,
    pub lba_map_list: list_head,
    pub tg_pt_gps_lock: spinlock_t,
    pub t10_dev: *mut se_device,
// Used for default ALUA Target Port Group
    pub default_tg_pt_gp: *mut t10_alua_tg_pt_gp,
// Used for default ALUA Target Port Group ConfigFS group
    pub alua_tg_pt_gps_group: config_group,
    pub tg_pt_gps_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct t10_alua_lu_gp {
    pub lu_gp_id: u16,
    pub lu_gp_valid_id: c_int,
    pub lu_gp_members: u32,
    pub lu_gp_ref_cnt: core::sync::atomic::AtomicI32,
    pub lu_gp_lock: spinlock_t,
    pub lu_gp_group: config_group,
    pub lu_gp_node: list_head,
    pub lu_gp_mem_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct t10_alua_lu_gp_member {
    pub lu_gp_assoc: bool,
    pub lu_gp_mem_ref_cnt: core::sync::atomic::AtomicI32,
    pub lu_gp_mem_lock: spinlock_t,
    pub lu_gp: *mut t10_alua_lu_gp,
    pub lu_gp_mem_dev: *mut se_device,
    pub lu_gp_mem_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct t10_alua_tg_pt_gp {
    pub tg_pt_gp_id: u16,
    pub tg_pt_gp_valid_id: c_int,
    pub tg_pt_gp_alua_supported_states: c_int,
    pub tg_pt_gp_alua_access_status: c_int,
    pub tg_pt_gp_alua_access_type: c_int,
    pub tg_pt_gp_nonop_delay_msecs: c_int,
    pub tg_pt_gp_trans_delay_msecs: c_int,
    pub tg_pt_gp_implicit_trans_secs: c_int,
    pub tg_pt_gp_pref: c_int,
    pub tg_pt_gp_write_metadata: c_int,
    pub tg_pt_gp_members: u32,
    pub tg_pt_gp_alua_access_state: c_int,
    pub tg_pt_gp_ref_cnt: core::sync::atomic::AtomicI32,
    pub tg_pt_gp_lock: spinlock_t,
    pub tg_pt_gp_transition_mutex: mutex,
    pub tg_pt_gp_dev: *mut se_device,
    pub tg_pt_gp_group: config_group,
    pub tg_pt_gp_list: list_head,
    pub tg_pt_gp_lun_list: list_head,
    pub tg_pt_gp_alua_lun: *mut se_lun,
    pub tg_pt_gp_alua_nacl: *mut se_node_acl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct t10_vpd {
    pub device_identifier: [c_uchar; INQUIRY_VPD_DEVICE_IDENTIFIER_LEN],
    pub protocol_identifier_set: c_int,
    pub protocol_identifier: u32,
    pub device_identifier_code_set: u32,
    pub association: u32,
    pub device_identifier_type: u32,
    pub vpd_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct t10_wwn {
//
// SCSI left aligned strings may not be null terminated. +1 to ensure a
// null terminator is always present.
//
    pub 1]: char vendor[INQUIRY_VENDOR_LEN +,
    pub 1]: char model[INQUIRY_MODEL_LEN +,
    pub 1]: char revision[INQUIRY_REVISION_LEN +,
    pub unit_serial: [c_char; INQUIRY_VPD_SERIAL_LEN],
    pub company_id: u32,
    pub t10_vpd_lock: spinlock_t,
    pub t10_dev: *mut se_device,
    pub t10_wwn_group: config_group,
    pub t10_vpd_list: list_head,
    pub pd_text_id_info: [c_char; PD_TEXT_ID_INFO_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct t10_pr_registration {
// Used for fabrics that contain WWN+ISID
pub const PR_REG_ISID_LEN: c_int = 16;
// PR_REG_ISID_LEN + ',i,0x'
    pub pr_reg_isid: [c_char; PR_REG_ISID_LEN],
// Used during APTPL metadata reading
pub const PR_APTPL_MAX_IPORT_LEN: c_int = 256;
    pub pr_iport: [c_uchar; PR_APTPL_MAX_IPORT_LEN],
// Used during APTPL metadata reading
pub const PR_APTPL_MAX_TPORT_LEN: c_int = 256;
    pub pr_tport: [c_uchar; PR_APTPL_MAX_TPORT_LEN],
    pub pr_aptpl_rpti: u16,
    pub pr_reg_tpgt: u16,
// Reservation effects all target ports
    pub pr_reg_all_tg_pt: c_int,
// Activate Persistence across Target Power Loss
    pub pr_reg_aptpl: c_int,
    pub pr_res_holder: c_int,
    pub pr_res_type: c_int,
    pub pr_res_scope: c_int,
// Used for fabric initiator WWPNs using a ISID
    pub isid_present_at_reg: bool,
    pub pr_res_mapped_lun: u64,
    pub pr_aptpl_target_lun: u64,
    pub tg_pt_sep_rtpi: u16,
    pub pr_res_generation: u32,
    pub pr_reg_bin_isid: u64,
    pub pr_res_key: u64,
    pub pr_res_holders: core::sync::atomic::AtomicI32,
    pub pr_reg_nacl: *mut se_node_acl,
// Used by ALL_TG_PT=1 registration with deve->pr_ref taken
    pub pr_reg_deve: *mut se_dev_entry,
    pub pr_reg_list: list_head,
    pub pr_reg_abort_list: list_head,
    pub pr_reg_aptpl_list: list_head,
    pub pr_reg_atp_list: list_head,
    pub pr_reg_atp_mem_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct t10_reservation {
// Reservation effects all target ports
    pub pr_all_tg_pt: c_int,
// Activate Persistence across Target Power Loss enabled
// for SCSI device
    pub pr_aptpl_active: c_int,
pub const PR_APTPL_BUF_LEN: c_int = 262144;
    pub pr_generation: u32,
    pub registration_lock: spinlock_t,
    pub aptpl_reg_lock: spinlock_t,
//
// This will always be set by one individual I_T Nexus.
// However with all_tg_pt=1, other I_T Nexus from the
// same initiator can access PR reg/res info on a different
// target port.
//
// There is also the 'All Registrants' case, where there is
// a single *pr_res_holder of the reservation, but all
// registrations are considered reservation holders.
//
    pub pr_res_holder: *mut se_node_acl,
    pub registration_list: list_head,
    pub aptpl_reg_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct se_tmr_req {
// Task Management function to be performed
    pub function: u8,
// Task Management response to send
    pub response: u8,
    pub call_transport: c_int,
// Reference to ITT that Task Mgmt should be performed
    pub ref_task_tag: u64,
    pub fabric_tmr_ptr: *mut c_void,
    pub task_cmd: *mut se_cmd,
    pub tmr_dev: *mut se_device,
    pub tmr_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum target_prot_op {
    TARGET_PROT_NORMAL	= 0,
    TARGET_PROT_DIN_INSERT	= (1 << 0),
    TARGET_PROT_DOUT_INSERT	= (1 << 1),
    TARGET_PROT_DIN_STRIP	= (1 << 2),
    TARGET_PROT_DOUT_STRIP	= (1 << 3),
    TARGET_PROT_DIN_PASS	= (1 << 4),
    TARGET_PROT_DOUT_PASS	= (1 << 5),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum target_prot_type {
    TARGET_DIF_TYPE0_PROT,
    TARGET_DIF_TYPE1_PROT,
    TARGET_DIF_TYPE2_PROT,
    TARGET_DIF_TYPE3_PROT,
}

// Emulation for UNIT ATTENTION Interlock Control
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum target_ua_intlck_ctrl {
    TARGET_UA_INTLCK_CTRL_CLEAR = 0,
    TARGET_UA_INTLCK_CTRL_NO_CLEAR = 1,
    TARGET_UA_INTLCK_CTRL_ESTABLISH_UA = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum target_core_dif_check {
    TARGET_DIF_CHECK_GUARD  = 0x1 << 0,
    TARGET_DIF_CHECK_APPTAG = 0x1 << 1,
    TARGET_DIF_CHECK_REFTAG = 0x1 << 2,
}

// for sam_task_attr
pub const TCM_SIMPLE_TAG: c_uint = 0x20;
pub const TCM_HEAD_TAG: c_uint = 0x21;
pub const TCM_ORDERED_TAG: c_uint = 0x22;
pub const TCM_ACA_TAG: c_uint = 0x24;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct se_cmd {
// Used for fail with specific sense codes
    pub sense_reason: sense_reason_t,
// SAM response code being sent to initiator
    pub scsi_status: u8,
    pub scsi_sense_length: u16,
    pub unknown_data_length:1: unsigned,
    pub state_active:1: bool,
    pub /: *mut *mut u64 tag; / SAM command identifier aka task tag,
// Delay for ALUA Active/NonOptimized state access in milliseconds
    pub alua_nonop_delay: c_int,
// See include/linux/dma-mapping.h
    pub data_direction: dma_data_direction,
// For SAM Task Attribute
    pub sam_task_attr: c_int,
// Used for se_sess->sess_tag_pool
    pub map_tag: c_uint,
    pub map_cpu: c_int,
// Transport protocol dependent state, see transport_state_table
    pub t_state: transport_state_table,
// See se_cmd_flags_table
    pub se_cmd_flags: u32,
// Total size in bytes associated with command
    pub data_length: u32,
    pub residual_count: u32,
    pub orig_fe_lun: u64,
// Persistent Reservation key
    pub pr_res_key: u64,
// Used for sense data
    pub sense_buffer: *mut c_void,
    pub se_delayed_node: list_head,
    pub se_qf_node: list_head,
    pub se_dev: *mut se_device,
    pub se_lun: *mut se_lun,
// Only used for internal passthrough and legacy TCM fabric modules
    pub se_sess: *mut se_session,
    pub cmd_cnt: *mut target_cmd_counter,
    pub se_tmr_req: *mut se_tmr_req,
    pub se_cmd_list: llist_node,
    pub free_compl: *mut completion,
    pub abrt_compl: *mut completion,
    pub se_tfo: *const target_core_fabric_ops,
    pub ): *mut *mut sense_reason_t (execute_cmd)(struct se_cmd,
    pub ): *mut *mut *mut sense_reason_t (transport_complete_callback)(struct se_cmd , bool, int,
    pub protocol_data: *mut c_void,
    pub t_task_cdb: *mut c_uchar,
    pub __t_task_cdb: [c_uchar; TCM_MAX_COMMAND_SIZE],
    pub t_task_lba: c_ulonglong,
    pub t_task_nolb: c_uint,
    pub transport_state: c_uint,

    pub t_state_lock: spinlock_t,
    pub cmd_kref: kref,
    pub t_transport_stop_comp: completion,
    pub work: work_struct,
    pub t_data_sg: *mut scatterlist,
    pub t_data_sg_orig: *mut scatterlist,
    pub t_data_nents: c_uint,
    pub t_data_nents_orig: c_uint,
    pub t_data_vmap: *mut c_void,
    pub t_bidi_data_sg: *mut scatterlist,
    pub t_bidi_data_nents: c_uint,
// Used for lun->lun_ref counting
    pub lun_ref_active: c_int,
    pub state_list: list_head,
// backend private data
    pub priv: *mut c_void,
// DIF related members
    pub prot_op: target_prot_op,
    pub prot_type: target_prot_type,
    pub prot_checks: u8,
    pub prot_pto: bool,
    pub prot_length: u32,
    pub reftag_seed: u32,
    pub t_prot_sg: *mut scatterlist,
    pub t_prot_nents: c_uint,
    pub pi_err: sense_reason_t,
    pub sense_info: u64,
//
// CPU LIO will execute the cmd on. Defaults to the CPU the cmd is
// initialized on. Drivers can override.
//
    pub cpuid: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct se_ua {
    pub ua_asc: u8,
    pub ua_ascq: u8,
    pub ua_nacl_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct se_node_acl {
    pub initiatorname: [c_char; TRANSPORT_IQN_LEN],
// Used to signal demo mode created ACL, disabled by default
    pub dynamic_node_acl: bool,
    pub dynamic_stop: bool,
    pub queue_depth: u32,
    pub acl_index: u32,
    pub saved_prot_type: target_prot_type,
pub const MAX_ACL_TAG_SIZE: c_int = 64;
    pub acl_tag: [c_char; MAX_ACL_TAG_SIZE],
// Used for PR SPEC_I_PT=1 and REGISTER_AND_MOVE
    pub acl_pr_ref_count: core::sync::atomic::AtomicI32,
    pub lun_entry_hlist: hlist_head,
    pub nacl_sess: *mut se_session,
    pub se_tpg: *mut se_portal_group,
    pub lun_entry_mutex: mutex,
    pub nacl_sess_lock: spinlock_t,
    pub acl_group: config_group,
    pub acl_attrib_group: config_group,
    pub acl_auth_group: config_group,
    pub acl_param_group: config_group,
    pub acl_fabric_stat_group: config_group,
    pub acl_list: list_head,
    pub acl_sess_list: list_head,
    pub acl_free_comp: completion,
    pub acl_kref: kref,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct target_cmd_counter {
    pub refcnt: percpu_ref,
    pub refcnt_wq: wait_queue_head_t,
    pub stop_done: completion,
    pub stopped: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct se_session {
    pub sess_bin_isid: u64,
    pub sup_prot_ops: target_prot_op,
    pub sess_prot_type: target_prot_type,
    pub se_node_acl: *mut se_node_acl,
    pub se_tpg: *mut se_portal_group,
    pub fabric_sess_ptr: *mut c_void,
    pub sess_list: list_head,
    pub sess_acl_list: list_head,
    pub sess_cmd_lock: spinlock_t,
    pub sess_cmd_map: *mut c_void,
    pub sess_tag_pool: sbitmap_queue,
    pub cmd_cnt: *mut target_cmd_counter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct se_ml_stat_grps {
    pub stat_group: config_group,
    pub scsi_auth_intr_group: config_group,
    pub scsi_att_intr_port_group: config_group,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct se_lun_acl {
    pub mapped_lun: u64,
    pub se_lun_nacl: *mut se_node_acl,
    pub se_lun: *mut se_lun,
    pub se_lun_group: config_group,
    pub ml_stat_grps: se_ml_stat_grps,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct se_dev_entry_io_stats {
    pub total_cmds: u64,
    pub read_bytes: u64,
    pub write_bytes: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct se_dev_entry {
    pub mapped_lun: u64,
    pub pr_res_key: u64,
    pub creation_time: u64,
    pub lun_access_ro: bool,
    pub attach_count: u32,
    pub stats: *mut se_dev_entry_io_stats __percpu,
// Used for PR SPEC_I_PT=1 and REGISTER_AND_MOVE
    pub pr_kref: kref,
    pub pr_comp: completion,
    pub se_lun_acl: *mut se_lun_acl,
    pub ua_lock: spinlock_t,
    pub se_lun: *mut se_lun,
pub const DEF_PR_REG_ACTIVE: c_int = 1;
    pub deve_flags: c_ulong,
    pub alua_port_list: list_head,
    pub lun_link: list_head,
    pub ua_list: list_head,
    pub link: hlist_node,
    pub rcu_head: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct se_dev_attrib {
    pub emulate_model_alias: bool,
    pub /: *mut *mut bool emulate_dpo; / deprecated,
    pub emulate_fua_write: bool,
    pub /: *mut *mut bool emulate_fua_read; / deprecated,
    pub emulate_write_cache: bool,
    pub emulate_ua_intlck_ctrl: target_ua_intlck_ctrl,
    pub emulate_tas: bool,
    pub emulate_tpu: bool,
    pub emulate_tpws: bool,
    pub emulate_caw: bool,
    pub emulate_3pc: bool,
    pub emulate_pr: bool,
    pub emulate_rsoc: bool,
    pub pi_prot_type: target_prot_type,
    pub hw_pi_prot_type: target_prot_type,
    pub pi_prot_verify: bool,
    pub enforce_pr_isids: bool,
    pub force_pr_aptpl: bool,
    pub is_nonrot: bool,
    pub emulate_rest_reord: bool,
    pub unmap_zeroes_data: bool,
    pub hw_block_size: u32,
    pub block_size: u32,
    pub hw_max_sectors: u32,
    pub optimal_sectors: u32,
    pub hw_queue_depth: u32,
    pub queue_depth: u32,
    pub max_unmap_lba_count: u32,
    pub max_unmap_block_desc_count: u32,
    pub unmap_granularity: u32,
    pub unmap_granularity_alignment: u32,
    pub max_write_same_len: u32,
    pub atomic_max_len: u32,
    pub atomic_alignment: u32,
    pub atomic_granularity: u32,
    pub atomic_max_with_boundary: u32,
    pub atomic_max_boundary: u32,
    pub complete_type: u8,
    pub submit_type: u8,
    pub da_dev: *mut se_device,
    pub da_group: config_group,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct se_port_stat_grps {
    pub stat_group: config_group,
    pub scsi_port_group: config_group,
    pub scsi_tgt_port_group: config_group,
    pub scsi_transport_group: config_group,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_port_stats {
    pub cmd_pdus: u64,
    pub tx_data_octets: u64,
    pub rx_data_octets: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct se_lun {
    pub unpacked_lun: u64,
    pub lun_shutdown: bool,
    pub lun_access_ro: bool,
    pub lun_index: u32,
    pub lun_acl_count: core::sync::atomic::AtomicI32,
    pub lun_se_dev: *mut se_device __rcu,
    pub lun_deve_list: list_head,
    pub lun_deve_lock: spinlock_t,
// ALUA state
    pub lun_tg_pt_secondary_stat: c_int,
    pub lun_tg_pt_secondary_write_md: c_int,
    pub lun_tg_pt_secondary_offline: core::sync::atomic::AtomicI32,
    pub lun_tg_pt_md_mutex: mutex,
// ALUA target port group linkage
    pub lun_tg_pt_gp_link: list_head,
    pub lun_tg_pt_gp: *mut t10_alua_tg_pt_gp __rcu,
    pub lun_tg_pt_gp_lock: spinlock_t,
    pub lun_tpg: *mut se_portal_group,
    pub lun_stats: *mut scsi_port_stats __percpu,
    pub lun_group: config_group,
    pub port_stat_grps: se_port_stat_grps,
    pub lun_shutdown_comp: completion,
    pub lun_ref: percpu_ref,
    pub lun_dev_link: list_head,
    pub link: hlist_node,
    pub rcu_head: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct se_dev_stat_grps {
    pub stat_group: config_group,
    pub scsi_dev_group: config_group,
    pub scsi_tgt_dev_group: config_group,
    pub scsi_lu_group: config_group,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct se_cmd_queue {
    pub cmd_list: llist_head,
    pub work: work_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct se_dev_plug {
    pub se_dev: *mut se_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct se_device_queue {
    pub state_list: list_head,
    pub lock: spinlock_t,
    pub sq: se_cmd_queue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct se_dev_io_stats {
    pub total_cmds: u64,
    pub read_bytes: u64,
    pub write_bytes: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct se_device {
// Used for SAM Task Attribute ordering
    pub dev_cur_ordered_id: u32,
    pub dev_flags: u32,
pub const DF_CONFIGURED: c_uint = 0x00000001;
pub const DF_FIRMWARE_VPD_UNIT_SERIAL: c_uint = 0x00000002;
pub const DF_EMULATED_VPD_UNIT_SERIAL: c_uint = 0x00000004;
pub const DF_USING_UDEV_PATH: c_uint = 0x00000008;
pub const DF_USING_ALIAS: c_uint = 0x00000010;
pub const DF_READ_ONLY: c_uint = 0x00000020;
    pub transport_flags: u8,
// Physical device queue depth
    pub queue_depth: u32,
// Used for SPC-2 reservations enforce of ISIDs
    pub dev_res_bin_isid: u64,
// Pointer to transport specific device structure
    pub dev_index: u32,
    pub creation_time: u64,
    pub num_resets: atomic_long_t,
    pub aborts_complete: atomic_long_t,
    pub aborts_no_task: atomic_long_t,
    pub stats: *mut se_dev_io_stats __percpu,
// Active commands on this virtual SE device
    pub non_ordered: percpu_ref,
    pub ordered_sync_in_progress: bool,
    pub dev_qf_count: core::sync::atomic::AtomicI32,
    pub export_count: u32,
    pub delayed_cmd_lock: spinlock_t,
    pub dev_reservation_lock: spinlock_t,
    pub dev_reservation_flags: c_uint,
pub const DRF_SPC2_RESERVATIONS: c_uint = 0x00000001;
pub const DRF_SPC2_RESERVATIONS_WITH_ISID: c_uint = 0x00000002;
    pub se_port_lock: spinlock_t,
    pub se_tmr_lock: spinlock_t,
    pub qf_cmd_lock: spinlock_t,
    pub caw_sem: semaphore,
// Used for legacy SPC-2 reservations
    pub reservation_holder: *mut se_session,
// Used for ALUA Logical Unit Group membership
    pub dev_alua_lu_gp_mem: *mut t10_alua_lu_gp_member,
// Used for SPC-3 Persistent Reservations
    pub dev_pr_res_holder: *mut t10_pr_registration,
    pub dev_sep_list: list_head,
    pub dev_tmr_list: list_head,
    pub qf_work_queue: work_struct,
    pub delayed_cmd_work: work_struct,
    pub delayed_cmd_list: list_head,
    pub qf_cmd_list: list_head,
// Pointer to associated SE HBA
    pub se_hba: *mut se_hba,
// T10 Inquiry and VPD WWN Information
    pub t10_wwn: t10_wwn,
// T10 Asymmetric Logical Unit Assignment for Target Ports
    pub t10_alua: t10_alua,
// T10 SPC-2 + SPC-3 Reservations
    pub t10_pr: t10_reservation,
    pub dev_attrib: se_dev_attrib,
    pub dev_action_group: config_group,
    pub dev_group: config_group,
    pub dev_pr_group: config_group,
    pub dev_stat_grps: se_dev_stat_grps,
    pub dev_alias: [c_uchar; SE_DEV_ALIAS_LEN],    pub udev_path: [c_uchar; SE_UDEV_PATH_LEN],
// Pointer to template of function pointers for transport
    pub transport: *const target_backend_ops,
    pub xcopy_lun: se_lun,
// Protection Information
    pub prot_length: c_int,
// For se_lun->lun_se_dev RCU read-side critical access
    pub hba_index: u32,
    pub rcu_head: rcu_head,
    pub queue_cnt: c_int,
    pub queues: *mut se_device_queue,
    pub lun_reset_mutex: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct target_opcode_descriptor {
    pub support:3: u8,
    pub serv_action_valid:1: u8,
    pub opcode: u8,
    pub service_action: u16,
    pub cdb_size: u32,
    pub specific_timeout: u8,
    pub nominal_timeout: u16,
    pub recommended_timeout: u16,
    pub cmd): *mut se_cmd,
    pub dev): *mut se_device,
    pub usage_bits: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct se_hba {
    pub hba_tpgt: u16,
    pub hba_id: u32,
// See hba_flags_table
    pub hba_flags: u32,
// Virtual iSCSI devices attached.
    pub dev_count: u32,
    pub hba_index: u32,
// Pointer to transport specific host structure.
    pub hba_ptr: *mut c_void,
    pub hba_node: list_head,
    pub device_lock: spinlock_t,
    pub hba_group: config_group,
    pub hba_access_mutex: mutex,
    pub backend: *mut target_backend,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct se_tpg_np {
    pub tpg_np_parent: *mut se_portal_group,
    pub tpg_np_group: config_group,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct se_portal_group {
//
// PROTOCOL IDENTIFIER value per SPC4, 7.5.1.
//
// Negative values can be used by fabric drivers for internal use TPGs.
//
    pub proto_id: c_int,
    pub enabled: bool,
// RELATIVE TARGET PORT IDENTIFIER
    pub tpg_rtpi: u16,
    pub rtpi_manual: bool,
// Used for PR SPEC_I_PT=1 and REGISTER_AND_MOVE
    pub tpg_pr_ref_count: core::sync::atomic::AtomicI32,
// Spinlock for adding/removing ACLed Nodes
    pub acl_node_mutex: mutex,
// Spinlock for adding/removing sessions
    pub session_lock: spinlock_t,
    pub tpg_lun_mutex: mutex,
// linked list for initiator ACL list
    pub acl_node_list: list_head,
    pub tpg_lun_hlist: hlist_head,
    pub tpg_virt_lun0: *mut se_lun,
// List of TCM sessions associated wth this TPG
    pub tpg_sess_list: list_head,
// Pointer to $FABRIC_MOD dependent code
    pub se_tpg_tfo: *const target_core_fabric_ops,
    pub se_tpg_wwn: *mut se_wwn,
    pub tpg_group: config_group,
    pub tpg_lun_group: config_group,
    pub tpg_np_group: config_group,
    pub tpg_acl_group: config_group,
    pub tpg_attrib_group: config_group,
    pub tpg_auth_group: config_group,
    pub tpg_param_group: config_group,
}

// Use se_cmd's cpuid for completion
// Complete on current CPU
#[repr(C)]
#[derive(Copy, Clone)]
pub struct se_wwn {
    pub wwn_tf: *mut target_fabric_configfs,
    pub priv: *mut c_void,
    pub wwn_group: config_group,
    pub fabric_stat_group: config_group,
    pub param_group: config_group,
    pub cmd_compl_affinity: c_int,
}
