//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/qla2xxx/qla_fw.h
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

pub const MBS_CHECKSUM_ERROR: c_uint = 0x4010;
pub const MBS_INVALID_PRODUCT_KEY: c_uint = 0x4020;
//
// Firmware Options.
//

//
// Port Database structure definition for ISP 24xx.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct buffer_credit_24xx {
    pub parameter: [u32; 28],
}

pub const PORT_DATABASE_24XX_SIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct port_database_24xx {
    pub flags: u16,

//
// for NVMe, the login_state field has been
// split into nibbles.
// The lower nibble is for FCP.
// The upper nibble is for NVMe.
//
    pub current_login_state: u8,
    pub last_login_state: u8,
pub const PDS_PLOGI_PENDING: c_uint = 0x03;
pub const PDS_PLOGI_COMPLETE: c_uint = 0x04;
pub const PDS_PRLI_PENDING: c_uint = 0x05;
pub const PDS_PRLI_COMPLETE: c_uint = 0x06;
pub const PDS_PORT_UNAVAILABLE: c_uint = 0x07;
pub const PDS_PRLO_PENDING: c_uint = 0x09;
pub const PDS_LOGO_PENDING: c_uint = 0x11;
pub const PDS_PRLI2_PENDING: c_uint = 0x12;
    pub hard_address: [u8; 3],
    pub reserved_1: u8,
    pub port_id: [u8; 3],
    pub sequence_id: u8,
    pub port_timer: u16,
    pub /: *mut *mut uint16_t nport_handle; / N_PORT handle.,
    pub receive_data_size: u16,
    pub reserved_2: u16,
    pub /: *mut *mut uint8_t prli_svc_param_word_0[2]; / Big endian,
// Bits 15-0 of word 0
    pub /: *mut *mut uint8_t prli_svc_param_word_3[2]; / Big endian,
// Bits 15-0 of word 3
    pub port_name: [u8; WWN_SIZE],
    pub node_name: [u8; WWN_SIZE],
    pub reserved_3: [u8; 2],
    pub nvme_first_burst_size: u16,
    pub /: *mut *mut uint16_t prli_nvme_svc_param_word_0; / Bits 15-0 of word 0,
    pub /: *mut *mut uint16_t prli_nvme_svc_param_word_3; / Bits 15-0 of word 3,
    pub secure_login: u8,
    pub reserved_4: [u8; 14],
}

//
// MB 75h returns a list of DB entries similar to port_database_24xx(64B).
// However, in this case it returns 1st 40 bytes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_name_list_extended {
    pub flags: __le16,
    pub current_login_state: u8,
    pub last_login_state: u8,
    pub hard_address: [u8; 3],
    pub reserved_1: u8,
    pub port_id: [u8; 3],
    pub sequence_id: u8,
    pub port_timer: __le16,
    pub /: *mut *mut __le16 nport_handle; / N_PORT handle.,
    pub receive_data_size: __le16,
    pub reserved_2: __le16,
// PRLI SVC Param are Big endian
    pub /: *mut *mut u8 prli_svc_param_word_0[2]; / Bits 15-0 of word 0,
    pub /: *mut *mut u8 prli_svc_param_word_3[2]; / Bits 15-0 of word 3,
    pub port_name: [u8; WWN_SIZE],
    pub node_name: [u8; WWN_SIZE],
}

// MB 75h: This is the short version of the database
#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_name_list {
    pub /: *mut *mut u8 port_node_name[WWN_SIZE]; / B7 most sig, B0 least sig,
    pub nport_handle: __le16,
    pub reserved: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vp_database_24xx {
    pub vp_status: u16,
    pub options: u8,
    pub id: u8,
    pub port_name: [u8; WWN_SIZE],
    pub node_name: [u8; WWN_SIZE],
    pub port_id_low: u16,
    pub port_id_high: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvram_24xx {
// NVRAM header.
    pub id: [u8; 4],
    pub nvram_version: __le16,
    pub reserved_0: u16,
// Firmware Initialization Control Block.
    pub version: __le16,
    pub reserved_1: u16,
    pub frame_payload_size: __le16,
    pub execution_throttle: __le16,
    pub exchange_count: __le16,
    pub hard_address: __le16,
    pub port_name: [u8; WWN_SIZE],
    pub node_name: [u8; WWN_SIZE],
    pub login_retry_count: __le16,
    pub link_down_on_nos: __le16,
    pub interrupt_delay_timer: __le16,
    pub login_timeout: __le16,
    pub firmware_options_1: __le32,
    pub firmware_options_2: __le32,
    pub firmware_options_3: __le32,
// Offset 56.
//
// BIT 0     = Control Enable
// BIT 1-15  =
//
// BIT 0-7   = Reserved
// BIT 8-10  = Output Swing 1G
// BIT 11-13 = Output Emphasis 1G
// BIT 14-15 = Reserved
//
// BIT 0-7   = Reserved
// BIT 8-10  = Output Swing 2G
// BIT 11-13 = Output Emphasis 2G
// BIT 14-15 = Reserved
//
// BIT 0-7   = Reserved
// BIT 8-10  = Output Swing 4G
// BIT 11-13 = Output Emphasis 4G
// BIT 14-15 = Reserved
//
    pub seriallink_options: [__le16; 4],
    pub reserved_2: [u16; 16],
// Offset 96.
    pub reserved_3: [u16; 16],
// PCIe table entries.
    pub reserved_4: [u16; 16],
// Offset 160.
    pub reserved_5: [u16; 16],
// Offset 192.
    pub reserved_6: [u16; 16],
// Offset 224.
    pub reserved_7: [u16; 16],
//
// BIT 0  = Enable spinup delay
// BIT 1  = Disable BIOS
// BIT 2  = Enable Memory Map BIOS
// BIT 3  = Enable Selectable Boot
// BIT 4  = Disable RISC code load
// BIT 5  = Disable Serdes
// BIT 6  =
// BIT 7  =
//
// BIT 8  =
// BIT 9  =
// BIT 10 = Enable lip full login
// BIT 11 = Enable target reset
// BIT 12 =
// BIT 13 =
// BIT 14 =
// BIT 15 = Enable alternate WWN
//
// BIT 16-31 =
//
    pub host_p: __le32,
    pub alternate_port_name: [u8; WWN_SIZE],
    pub alternate_node_name: [u8; WWN_SIZE],
    pub boot_port_name: [u8; WWN_SIZE],
    pub boot_lun_number: __le16,
    pub reserved_8: u16,
    pub alt1_boot_port_name: [u8; WWN_SIZE],
    pub alt1_boot_lun_number: __le16,
    pub reserved_9: u16,
    pub alt2_boot_port_name: [u8; WWN_SIZE],
    pub alt2_boot_lun_number: __le16,
    pub reserved_10: u16,
    pub alt3_boot_port_name: [u8; WWN_SIZE],
    pub alt3_boot_lun_number: __le16,
    pub reserved_11: u16,
//
// BIT 0 = Selective Login
// BIT 1 = Alt-Boot Enable
// BIT 2 = Reserved
// BIT 3 = Boot Order List
// BIT 4 = Reserved
// BIT 5 = Selective LUN
// BIT 6 = Reserved
// BIT 7-31 =
//
    pub efi_parameters: __le32,
    pub reset_delay: u8,
    pub reserved_12: u8,
    pub reserved_13: u16,
    pub boot_id_number: __le16,
    pub reserved_14: u16,
    pub max_luns_per_target: __le16,
    pub reserved_15: u16,
    pub port_down_retry_count: __le16,
    pub link_down_timeout: __le16,
// FCode parameters.
    pub fcode_parameter: __le16,
    pub reserved_16: [u16; 3],
// Offset 352.
    pub prev_drv_ver_major: u8,
    pub prev_drv_ver_submajob: u8,
    pub prev_drv_ver_minor: u8,
    pub prev_drv_ver_subminor: u8,
    pub prev_bios_ver_major: __le16,
    pub prev_bios_ver_minor: __le16,
    pub prev_efi_ver_major: __le16,
    pub prev_efi_ver_minor: __le16,
    pub prev_fw_ver_major: __le16,
    pub prev_fw_ver_minor: u8,
    pub prev_fw_ver_subminor: u8,
    pub reserved_17: [u16; 8],
// Offset 384.
    pub reserved_18: [u16; 16],
// Offset 416.
    pub reserved_19: [u16; 16],
// Offset 448.
    pub reserved_20: [u16; 16],
// Offset 480.
    pub model_name: [u8; 16],
    pub reserved_21: [u16; 2],
// Offset 500.
// HW Parameter Block.
    pub pcie_table_sig: u16,
    pub pcie_table_offset: u16,
    pub subsystem_vendor_id: u16,
    pub subsystem_device_id: u16,
    pub checksum: __le32,
}

//
// ISP Initialization Control Block.
// Little endian except where noted.
//
pub const ICB_VERSION: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct init_cb_24xx {
    pub version: __le16,
    pub reserved_1: u16,
    pub frame_payload_size: __le16,
    pub execution_throttle: __le16,
    pub exchange_count: __le16,
    pub hard_address: __le16,
    pub /: *mut *mut uint8_t port_name[WWN_SIZE]; / Big endian.,
    pub /: *mut *mut uint8_t node_name[WWN_SIZE]; / Big endian.,
    pub response_q_inpointer: __le16,
    pub request_q_outpointer: __le16,
    pub login_retry_count: __le16,
    pub prio_request_q_outpointer: __le16,
    pub response_q_length: __le16,
    pub request_q_length: __le16,
    pub /: *mut *mut __le16 link_down_on_nos; / Milliseconds.,
    pub prio_request_q_length: __le16,
    pub __packed: __le64 request_q_address,
    pub __packed: __le64 response_q_address,
    pub __packed: __le64 prio_request_q_address,
    pub msix: __le16,
    pub msix_atio: __le16,
    pub reserved_2: [u8; 4],
    pub atio_q_inpointer: __le16,
    pub atio_q_length: __le16,
    pub __packed: __le64 atio_q_address,
    pub /: *mut *mut __le16 interrupt_delay_timer; / 100us increments.,
    pub login_timeout: __le16,
//
// BIT 0  = Enable Hard Loop Id
// BIT 1  = Enable Fairness
// BIT 2  = Enable Full-Duplex
// BIT 3  = Reserved
// BIT 4  = Enable Target Mode
// BIT 5  = Disable Initiator Mode
// BIT 6  = Acquire FA-WWN
// BIT 7  = Enable D-port Diagnostics
//
// BIT 8  = Reserved
// BIT 9  = Non Participating LIP
// BIT 10 = Descending Loop ID Search
// BIT 11 = Acquire Loop ID in LIPA
// BIT 12 = Reserved
// BIT 13 = Full Login after LIP
// BIT 14 = Node Name Option
// BIT 15-31 = Reserved
//
    pub firmware_options_1: __le32,
//
// BIT 0  = Operation Mode bit 0
// BIT 1  = Operation Mode bit 1
// BIT 2  = Operation Mode bit 2
// BIT 3  = Operation Mode bit 3
// BIT 4  = Connection Options bit 0
// BIT 5  = Connection Options bit 1
// BIT 6  = Connection Options bit 2
// BIT 7  = Enable Non part on LIHA failure
//
// BIT 8  = Enable Class 2
// BIT 9  = Enable ACK0
// BIT 10 = Reserved
// BIT 11 = Enable FC-SP Security
// BIT 12 = FC Tape Enable
// BIT 13 = Reserved
// BIT 14 = Enable Target PRLI Control
// BIT 15-31 = Reserved
//
    pub firmware_options_2: __le32,
//
// BIT 0  = Reserved
// BIT 1  = Soft ID only
// BIT 2  = Reserved
// BIT 3  = Reserved
// BIT 4  = FCP RSP Payload bit 0
// BIT 5  = FCP RSP Payload bit 1
// BIT 6  = Enable Receive Out-of-Order data frame handling
// BIT 7  = Disable Automatic PLOGI on Local Loop
//
// BIT 8  = Reserved
// BIT 9  = Enable Out-of-Order FCP_XFER_RDY relative offset handling
// BIT 10 = Reserved
// BIT 11 = Reserved
// BIT 12 = Reserved
// BIT 13 = Data Rate bit 0
// BIT 14 = Data Rate bit 1
// BIT 15 = Data Rate bit 2
// BIT 16 = Enable 75 ohm Termination Select
// BIT 17-28 = Reserved
// BIT 29 = Enable response queue 0 in index shadowing
// BIT 30 = Enable request queue 0 out index shadowing
// BIT 31 = Reserved
//
    pub firmware_options_3: __le32,
    pub qos: __le16,
    pub rid: __le16,
    pub reserved_3: [u8; 20],
}

//
// ISP queue - command entry structure definition.
//
pub const COMMAND_BIDIRECTIONAL: c_uint = 0x75;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_bidir {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined,
    pub /: *mut *mut uint8_t entry_status; / Entry status.,
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub /: *mut *mut __le16 nport_handle; / N_PORT handle.,
    pub /: *mut *mut __le16 timeout; / Command timeout.,
    pub /: *mut *mut __le16 wr_dseg_count; / Write Data segment count.,
    pub /: *mut *mut __le16 rd_dseg_count; / Read Data segment count.,
    pub /: *mut *mut scsi_lun lun; / FCP LUN (BE).,
    pub /: *mut *mut __le16 control_flags; / Control flags.,

    pub /: *mut *mut __le16 fcp_cmnd_dseg_len; / Data segment length.,
    pub /: *mut *mut __le64 fcp_cmnd_dseg_address __packed;/ Data segment address.,
    pub /: *mut *mut uint16_t reserved[2]; / Reserved,
    pub /: *mut *mut __le32 rd_byte_count; / Total Byte count Read.,
    pub /: *mut *mut __le32 wr_byte_count; / Total Byte count write.,
    pub port.*/: *mut *mut uint8_t port_id[3]; / PortID of destination,
    pub vp_index: u8,
    pub fcp_dsd: dsd64,
}

pub const COMMAND_TYPE_6: c_uint = 0x48		/* Command Type 6 entry */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_type_6 {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub /: *mut *mut __le16 nport_handle; / N_PORT handle.,
    pub /: *mut *mut __le16 timeout; / Command timeout.,
    pub /: *mut *mut __le16 dseg_count; / Data segment count.,
    pub /: *mut *mut __le16 fcp_rsp_dsd_len; / FCP_RSP DSD length.,
    pub /: *mut *mut scsi_lun lun; / FCP LUN (BE).,
    pub /: *mut *mut __le16 control_flags; / Control flags.,

    pub /: *mut *mut __le16 fcp_cmnd_dseg_len; / Data segment length.,
// Data segment address.
    pub __packed: __le64 fcp_cmnd_dseg_address,
// Data segment address.
    pub __packed: __le64 fcp_rsp_dseg_address,
    pub /: *mut *mut __le32 byte_count; / Total byte count.,
    pub /: *mut *mut uint8_t port_id[3]; / PortID of destination port.,
    pub vp_index: u8,
    pub fcp_dsd: dsd64,
}

pub const COMMAND_TYPE_7: c_uint = 0x18		/* Command Type 7 entry */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_type_7 {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub /: *mut *mut __le16 nport_handle; / N_PORT handle.,
    pub /: *mut *mut __le16 timeout; / Command timeout.,
pub const FW_MAX_TIMEOUT: c_uint = 0x1999;
    pub /: *mut *mut __le16 dseg_count; / Data segment count.,
    pub reserved_1: u16,
    pub /: *mut *mut scsi_lun lun; / FCP LUN (BE).,
    pub /: *mut *mut __le16 task_mgmt_flags; / Task management flags.,

    pub task: u8,
pub const TSK_SIMPLE: c_int = 0;
pub const TSK_HEAD_OF_QUEUE: c_int = 1;
pub const TSK_ORDERED: c_int = 2;
pub const TSK_ACA: c_int = 4;
pub const TSK_UNTAGGED: c_int = 5;
    pub crn: u8,
    pub /: *mut *mut uint8_t fcp_cdb[MAX_CMDSZ]; / SCSI command words.,
    pub /: *mut *mut __le32 byte_count; / Total byte count.,
    pub /: *mut *mut uint8_t port_id[3]; / PortID of destination port.,
    pub vp_index: u8,
    pub dsd: dsd64,
}

pub const COMMAND_TYPE_CRC_2: c_uint = 0x6A	/* Command Type CRC_2 (Type 6);
// (T10-DIF)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_type_crc_2 {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub /: *mut *mut __le16 nport_handle; / N_PORT handle.,
    pub /: *mut *mut __le16 timeout; / Command timeout.,
    pub /: *mut *mut __le16 dseg_count; / Data segment count.,
    pub /: *mut *mut __le16 fcp_rsp_dseg_len; / FCP_RSP DSD length.,
    pub /: *mut *mut scsi_lun lun; / FCP LUN (BE).,
    pub /: *mut *mut __le16 control_flags; / Control flags.,
    pub /: *mut *mut __le16 fcp_cmnd_dseg_len; / Data segment length.,
    pub __packed: __le64 fcp_cmnd_dseg_address,
// Data segment address.
    pub __packed: __le64 fcp_rsp_dseg_address,
    pub /: *mut *mut __le32 byte_count; / Total byte count.,
    pub /: *mut *mut uint8_t port_id[3]; / PortID of destination port.,
    pub vp_index: u8,
    pub /: *mut *mut __le64 crc_context_address __packed; / Data segment address.,
    pub /: *mut *mut __le16 crc_context_len; / Data segment length.,
    pub /: *mut *mut uint16_t reserved_1; / MUST be set to 0.,
}

//
// ISP queue - status entry structure definition.
//
pub const STATUS_TYPE: c_uint = 0x03		/* Status entry. */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sts_entry_24xx {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub /: *mut *mut __le16 comp_status; / Completion status.,
    pub /: *mut *mut __le16 ox_id; / OX_ID used by the firmware.,
    pub /: *mut *mut __le32 residual_len; / FW calc residual transfer length.,
    pub reserved_1: __le16,
    pub nvme_rsp_pyld_len: __le16,
    pub /: *mut *mut __le16 edif_sa_index; / edif sa_index used for initiator read data,
}

//
// If DIF Error is set in comp_status, these additional fields are
// defined:
//
// !!! NOTE: Firmware sends expected/actual DIF data in big endian
// format; but all of the "data" field gets swab32-d in the beginning
// of qla2x00_status_entry().
//
// &data[10] : uint8_t report_runt_bg[2];	- computed guard
// &data[12] : uint8_t actual_dif[8];		- DIF Data received
// &data[20] : uint8_t expected_dif[8];		- DIF Data computed
//
// Status entry completion status
//
pub const CS_DATA_REASSEMBLY_ERROR: c_uint = 0x11	/* Data Reassembly Error.. */;
pub const CS_ABTS_BY_TARGET: c_uint = 0x13	/* Target send ABTS to abort IOCB. */;
pub const CS_FW_RESOURCE: c_uint = 0x2C	/* Firmware Resource Unavailable. */;
pub const CS_TASK_MGMT_OVERRUN: c_uint = 0x30	/* Task management overrun (8+). */;
pub const CS_ABORT_BY_TARGET: c_uint = 0x47	/* Abort By Target. */;
//
// ISP queue - marker entry structure definition.
//
pub const MARKER_TYPE: c_uint = 0x04		/* Marker entry. */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrk_entry_24xx {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t handle_count; / Handle count.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub /: *mut *mut __le16 nport_handle; / N_PORT handle.,
    pub /: *mut *mut uint8_t modifier; / Modifier (7-0).,

    pub reserved_1: u8,
    pub reserved_2: u8,
    pub vp_index: u8,
    pub reserved_3: u16,
    pub /: *mut *mut uint8_t lun[8]; / FCP LUN (BE).,
    pub reserved_4: [u8; 40],
}

//
// ISP queue - CT Pass-Through entry structure definition.
//
pub const CT_IOCB_TYPE: c_uint = 0x29	/* CT Pass-Through IOCB entry */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ct_entry_24xx {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System Defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub /: *mut *mut __le16 comp_status; / Completion status.,
    pub /: *mut *mut __le16 nport_handle; / N_PORT handle.,
    pub cmd_dsd_count: __le16,
    pub vp_index: u8,
    pub reserved_1: u8,
    pub /: *mut *mut __le16 timeout; / Command timeout.,
    pub reserved_2: u16,
    pub rsp_dsd_count: __le16,
    pub reserved_3: [u8; 10],
    pub rsp_byte_count: __le32,
    pub cmd_byte_count: __le32,
    pub dsd: [dsd64; 2],
}

pub const PURX_ELS_HEADER_SIZE: c_uint = 0x18;
//
// ISP queue - PUREX IOCB entry structure definition
//
pub const PUREX_IOCB_TYPE: c_uint = 0x51	/* CT Pass Through IOCB entry */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct purex_entry_24xx {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub reserved1: __le16,
    pub vp_idx: u8,
    pub reserved2: u8,
    pub status_flags: __le16,
    pub nport_handle: __le16,
    pub frame_size: __le16,
    pub trunc_frame_size: __le16,
    pub rx_xchg_addr: __le32,
    pub d_id: [u8; 3],
    pub r_ctl: u8,
    pub s_id: [u8; 3],
    pub cs_ctl: u8,
    pub f_ctl: [u8; 3],
    pub type: u8,
    pub seq_cnt: __le16,
    pub df_ctl: u8,
    pub seq_id: u8,
    pub rx_id: __le16,
    pub ox_id: __le16,
    pub param: __le32,
    pub els_frame_payload: [u8; 20],
}

//
// ISP queue - ELS Pass-Through entry structure definition.
//
pub const ELS_IOCB_TYPE: c_uint = 0x53	/* ELS Pass-Through IOCB entry */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct els_entry_24xx {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System Defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub /: *mut *mut __le16 comp_status; / response only,
    pub nport_handle: __le16,
    pub tx_dsd_count: __le16,
    pub vp_index: u8,
    pub sof_type: u8,

    pub /: *mut *mut __le32 rx_xchg_address; / Receive exchange address.,
    pub rx_dsd_count: __le16,
    pub opcode: u8,
    pub reserved_2: u8,
    pub d_id: [u8; 3],
    pub s_id: [u8; 3],
    pub /: *mut *mut __le16 control_flags; / Control flags.,

    pub rx_byte_count: __le32,
    pub tx_byte_count: __le32,
    pub /: *mut *mut __le64 tx_address __packed; / DSD 0 address.,
    pub /: *mut *mut __le32 tx_len; / DSD 0 length.,
    pub /: *mut *mut __le64 rx_address __packed; / DSD 1 address.,
    pub /: *mut *mut __le32 rx_len; / DSD 1 length.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct els_sts_entry_24xx {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System Defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut __le32 handle; / System handle.,
    pub comp_status: __le16,
    pub /: *mut *mut __le16 nport_handle; / N_PORT handle.,
    pub reserved_1: __le16,
    pub vp_index: u8,
    pub sof_type: u8,
    pub /: *mut *mut __le32 rx_xchg_address; / Receive exchange address.,
    pub reserved_2: __le16,
    pub opcode: u8,
    pub reserved_3: u8,
    pub d_id: [u8; 3],
    pub s_id: [u8; 3],
    pub /: *mut *mut __le16 control_flags; / Control flags.,
    pub total_byte_count: __le32,
    pub error_subcode_1: __le32,
    pub error_subcode_2: __le32,
    pub error_subcode_3: __le32,
    pub reserved_4: [__le32; 4],
}

//
// ISP queue - Mailbox Command entry structure definition.
//
pub const MBX_IOCB_TYPE: c_uint = 0x39;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbx_entry_24xx {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t handle_count; / Handle count.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub mbx: [u16; 28],
}

pub const LOGINOUT_PORT_IOCB_TYPE: c_uint = 0x52	/* Login/Logout Port entry. */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct logio_entry_24xx {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub /: *mut *mut __le16 comp_status; / Completion status.,
pub const CS_LOGIO_ERROR: c_uint = 0x31	/* Login/Logout IOCB error. */;
    pub /: *mut *mut __le16 nport_handle; / N_PORT handle.,
    pub /: *mut *mut __le16 control_flags; / Control flags.,
// Modifiers.

// Commands.
pub const LCF_COMMAND_PLOGI: c_uint = 0x00	/* PLOGI. */;
pub const LCF_COMMAND_PRLI: c_uint = 0x01	/* PRLI. */;
pub const LCF_COMMAND_PDISC: c_uint = 0x02	/* PDISC. */;
pub const LCF_COMMAND_ADISC: c_uint = 0x03	/* ADISC. */;
pub const LCF_COMMAND_LOGO: c_uint = 0x08	/* LOGO. */;
pub const LCF_COMMAND_PRLO: c_uint = 0x09	/* PRLO. */;
pub const LCF_COMMAND_TPRLO: c_uint = 0x0A	/* TPRLO. */;
    pub vp_index: u8,
    pub reserved_1: u8,
    pub /: *mut *mut uint8_t port_id[3]; / PortID of destination port.,
    pub /: *mut *mut uint8_t rsp_size; / Response size in 32bit words.,
    pub /: *mut *mut __le32 io_parameter[11]; / General I/O parameters.,

pub const LSC_SCODE_NOLINK: c_uint = 0x01;
pub const LSC_SCODE_NOIOCB: c_uint = 0x02;
pub const LSC_SCODE_NOXCB: c_uint = 0x03;
pub const LSC_SCODE_CMD_FAILED: c_uint = 0x04;
pub const LSC_SCODE_NOFABRIC: c_uint = 0x05;
pub const LSC_SCODE_FW_NOT_READY: c_uint = 0x07;
pub const LSC_SCODE_NOT_LOGGED_IN: c_uint = 0x09;
pub const LSC_SCODE_NOPCB: c_uint = 0x0A;
pub const LSC_SCODE_ELS_REJECT: c_uint = 0x18;
pub const LSC_SCODE_CMD_PARAM_ERR: c_uint = 0x19;
pub const LSC_SCODE_PORTID_USED: c_uint = 0x1A;
pub const LSC_SCODE_NPORT_USED: c_uint = 0x1B;
pub const LSC_SCODE_NONPORT: c_uint = 0x1C;
pub const LSC_SCODE_LOGGED_IN: c_uint = 0x1D;
pub const LSC_SCODE_NOFLOGI_ACC: c_uint = 0x1F;
}

pub const TSK_MGMT_IOCB_TYPE: c_uint = 0x14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsk_mgmt_entry {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t handle_count; / Handle count.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub /: *mut *mut __le16 nport_handle; / N_PORT handle.,
    pub reserved_1: u16,
    pub /: *mut *mut __le16 delay; / Activity delay in seconds.,
    pub /: *mut *mut __le16 timeout; / Command timeout.,
    pub /: *mut *mut scsi_lun lun; / FCP LUN (BE).,
    pub /: *mut *mut __le32 control_flags; / Control Flags.,
    pub reserved_2: [u8; 20],
    pub /: *mut *mut uint8_t port_id[3]; / PortID of destination port.,
    pub vp_index: u8,
    pub reserved_3: [u8; 12],
}

pub const ABORT_IOCB_TYPE: c_uint = 0x33;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct abort_entry_24xx {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t handle_count; / Handle count.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub /: *mut *mut __le16 nport_handle; / N_PORT handle.,
    pub /: *mut *mut __le16 comp_status; / Completion status.,
}

pub const ABTS_RCV_TYPE: c_uint = 0x54;
pub const ABTS_RSP_TYPE: c_uint = 0x55;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct abts_entry_24xx {
    pub entry_type: u8,
    pub entry_count: u8,
    pub handle_count: u8,
    pub entry_status: u8,
    pub /: *mut *mut __le32 handle; / type 0x55 only,
    pub /: *mut *mut __le16 comp_status; / type 0x55 only,
    pub /: *mut *mut __le16 nport_handle; / type 0x54 only,
    pub /: *mut *mut __le16 control_flags; / type 0x55 only,
    pub vp_idx: u8,
    pub /: *mut *mut uint8_t sof_type; / sof_type is upper nibble,
    pub rx_xch_addr: __le32,
    pub d_id: [u8; 3],
    pub r_ctl: u8,
    pub s_id: [u8; 3],
    pub cs_ctl: u8,
    pub f_ctl: [u8; 3],
    pub type: u8,
    pub seq_cnt: __le16,
    pub df_ctl: u8,
    pub seq_id: u8,
    pub rx_id: __le16,
    pub ox_id: __le16,
    pub param: __le32,
    pub subcode3: __le32,
    pub rsvd: __le32,
    pub subcode1: __le32,
    pub subcode2: __le32,
    pub error: },
    pub rsrvd1: __le16,
    pub last_seq_id: u8,
    pub seq_id_valid: u8,
    pub aborted_rx_id: __le16,
    pub aborted_ox_id: __le16,
    pub high_seq_cnt: __le16,
    pub low_seq_cnt: __le16,
    pub ba_acc: },
    pub vendor_unique: u8,
    pub explanation: u8,
    pub reason: u8,
    pub ba_rjt: },
    pub payload: },
    pub rx_xch_addr_to_abort: __le32,
    pub __packed: },
// ABTS payload explanation values
pub const BA_RJT_EXP_NO_ADDITIONAL: c_int = 0;
pub const BA_RJT_EXP_INV_OX_RX_ID: c_int = 3;
pub const BA_RJT_EXP_SEQ_ABORTED: c_int = 5;
// ABTS payload reason values
pub const BA_RJT_RSN_INV_CMD_CODE: c_int = 1;
pub const BA_RJT_RSN_LOGICAL_ERROR: c_int = 3;
pub const BA_RJT_RSN_LOGICAL_BUSY: c_int = 5;
pub const BA_RJT_RSN_PROTOCOL_ERROR: c_int = 7;
pub const BA_RJT_RSN_UNABLE_TO_PERFORM: c_int = 9;
pub const BA_RJT_RSN_VENDOR_SPECIFIC: c_uint = 0xff;
// FC_F values
pub const FC_TYPE_BLD: c_uint = 0x000		/* Basic link data */;
pub const FC_F_CTL_RSP_CNTXT: c_uint = 0x800000	/* Responder of exchange */;
pub const FC_F_CTL_LAST_SEQ: c_uint = 0x100000	/* Last sequence */;
pub const FC_F_CTL_END_SEQ: c_uint = 0x80000		/* Last sequence */;
pub const FC_F_CTL_SEQ_INIT: c_uint = 0x010000	/* Sequence initiative */;
pub const FC_ROUTING_BLD: c_uint = 0x80		/* Basic link data frame */;
pub const FC_R_CTL_BLD_BA_ACC: c_uint = 0x04		/* BA_ACC (basic accept) */;
//
// ISP I/O Register Set structure definitions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_reg_24xx {
    pub /: *mut *mut __le32 flash_addr; / Flash/NVRAM BIOS address.,

pub const FARX_ACCESS_FLASH_CONF: c_uint = 0x7FFD0000;
pub const FARX_ACCESS_FLASH_DATA: c_uint = 0x7FF00000;
pub const FARX_ACCESS_NVRAM_CONF: c_uint = 0x7FFF0000;
pub const FARX_ACCESS_NVRAM_DATA: c_uint = 0x7FFE0000;
pub const FA_NVRAM_FUNC0_ADDR: c_uint = 0x80;
pub const FA_NVRAM_FUNC1_ADDR: c_uint = 0x180;
pub const FA_NVRAM_VPD_SIZE: c_uint = 0x200;
pub const FA_NVRAM_VPD0_ADDR: c_uint = 0x00;
pub const FA_NVRAM_VPD1_ADDR: c_uint = 0x100;
pub const FA_BOOT_CODE_ADDR: c_uint = 0x00000;
//
// RISC code begins at offset 512KB
// within flash. Consisting of two
// contiguous RISC code segments.
//
pub const FA_RISC_CODE_ADDR: c_uint = 0x20000;
pub const FA_RISC_CODE_SEGMENTS: c_int = 2;
pub const FA_FLASH_DESCR_ADDR_24: c_uint = 0x11000;
pub const FA_FLASH_LAYOUT_ADDR_24: c_uint = 0x11400;
pub const FA_NPIV_CONF0_ADDR_24: c_uint = 0x16000;
pub const FA_NPIV_CONF1_ADDR_24: c_uint = 0x17000;
pub const FA_FW_AREA_ADDR: c_uint = 0x40000;
pub const FA_VPD_NVRAM_ADDR: c_uint = 0x48000;
pub const FA_FEATURE_ADDR: c_uint = 0x4C000;
pub const FA_FLASH_DESCR_ADDR: c_uint = 0x50000;
pub const FA_FLASH_LAYOUT_ADDR: c_uint = 0x50400;
pub const FA_HW_EVENT0_ADDR: c_uint = 0x54000;
pub const FA_HW_EVENT1_ADDR: c_uint = 0x54400;
pub const FA_HW_EVENT_SIZE: c_uint = 0x200;
pub const FA_HW_EVENT_ENTRY_SIZE: c_int = 4;
pub const FA_NPIV_CONF0_ADDR: c_uint = 0x5C000;
pub const FA_NPIV_CONF1_ADDR: c_uint = 0x5D000;
pub const FA_FCP_PRIO0_ADDR: c_uint = 0x10000;
pub const FA_FCP_PRIO1_ADDR: c_uint = 0x12000;
//
// Flash Error Log Event Codes.
//
pub const HW_EVENT_RESET_ERR: c_uint = 0xF00B;
pub const HW_EVENT_ISP_ERR: c_uint = 0xF020;
pub const HW_EVENT_PARITY_ERR: c_uint = 0xF022;
pub const HW_EVENT_NVRAM_CHKSUM_ERR: c_uint = 0xF023;
pub const HW_EVENT_FLASH_FW_ERR: c_uint = 0xF024;
    pub /: *mut *mut __le32 flash_data; / Flash/NVRAM BIOS data.,
    pub /: *mut *mut __le32 ctrl_status; / Control/Status.,

// PCI-X Bus Mode.

// Max Write Burst byte count.

    pub /: *mut *mut __le32 ictrl; / Interrupt control.,

    pub /: *mut *mut __le32 istatus; / Interrupt status.,

    pub /: *mut *mut __le32 unused_1[2]; / Gap.,
// Request Queue.
    pub /: *mut *mut __le32 req_q_in; / In-Pointer.,
    pub /: *mut *mut __le32 req_q_out; / Out-Pointer.,
// Response Queue.
    pub /: *mut *mut __le32 rsp_q_in; / In-Pointer.,
    pub /: *mut *mut __le32 rsp_q_out; / Out-Pointer.,
// Priority Request Queue.
    pub /: *mut *mut __le32 preq_q_in; / In-Pointer.,
    pub /: *mut *mut __le32 preq_q_out; / Out-Pointer.,
    pub /: *mut *mut __le32 unused_2[2]; / Gap.,
// ATIO Queue.
    pub /: *mut *mut __le32 atio_q_in; / In-Pointer.,
    pub /: *mut *mut __le32 atio_q_out; / Out-Pointer.,
    pub host_status: __le32,

    pub /: *mut *mut __le32 hccr; / Host command & control register.,
// HCCR statuses.

// HCCR commands.
// NOOP.
pub const HCCRX_NOOP: c_uint = 0x00000000;
// Set RISC Reset.
pub const HCCRX_SET_RISC_RESET: c_uint = 0x10000000;
// Clear RISC Reset.
pub const HCCRX_CLR_RISC_RESET: c_uint = 0x20000000;
// Set RISC Pause.
pub const HCCRX_SET_RISC_PAUSE: c_uint = 0x30000000;
// Releases RISC Pause.
pub const HCCRX_REL_RISC_PAUSE: c_uint = 0x40000000;
// Set HOST to RISC interrupt.
pub const HCCRX_SET_HOST_INT: c_uint = 0x50000000;
// Clear HOST to RISC interrupt.
pub const HCCRX_CLR_HOST_INT: c_uint = 0x60000000;
// Clear RISC to PCI interrupt.
pub const HCCRX_CLR_RISC_INT: c_uint = 0xA0000000;
    pub /: *mut *mut __le32 gpiod; / GPIO Data register.,
// LED update mask.

// Data update mask.

// Data update mask.

// LED control mask.

// LED bit values. Color names as
// referenced in fw spec.
//

// Data in/out.

    pub /: *mut *mut __le32 gpioe; / GPIO Enable register.,
// Enable update mask.

// Enable update mask.

// Enable.

    pub /: *mut *mut __le32 iobase_addr; / I/O Bus Base Address register.,
    pub /: *mut *mut __le32 unused_3[10]; / Gap.,
    pub mailbox0: __le16,
    pub mailbox1: __le16,
    pub mailbox2: __le16,
    pub mailbox3: __le16,
    pub mailbox4: __le16,
    pub mailbox5: __le16,
    pub mailbox6: __le16,
    pub mailbox7: __le16,
    pub mailbox8: __le16,
    pub mailbox9: __le16,
    pub mailbox10: __le16,
    pub mailbox11: __le16,
    pub mailbox12: __le16,
    pub mailbox13: __le16,
    pub mailbox14: __le16,
    pub mailbox15: __le16,
    pub mailbox16: __le16,
    pub mailbox17: __le16,
    pub mailbox18: __le16,
    pub mailbox19: __le16,
    pub mailbox20: __le16,
    pub mailbox21: __le16,
    pub mailbox22: __le16,
    pub mailbox23: __le16,
    pub mailbox24: __le16,
    pub mailbox25: __le16,
    pub mailbox26: __le16,
    pub mailbox27: __le16,
    pub mailbox28: __le16,
    pub mailbox29: __le16,
    pub mailbox30: __le16,
    pub mailbox31: __le16,
    pub iobase_window: __le32,
    pub iobase_c4: __le32,
    pub iobase_c8: __le32,
    pub /: *mut *mut __le32 unused_4_1[6]; / Gap.,
    pub iobase_q: __le32,
    pub /: *mut *mut __le32 unused_5[2]; / Gap.,
    pub iobase_select: __le32,
    pub /: *mut *mut __le32 unused_6[2]; / Gap.,
    pub iobase_sdata: __le32,
}

// RISC-RISC semaphore register PCI offet
pub const RISC_REGISTER_BASE_OFFSET: c_uint = 0x7010;
pub const RISC_REGISTER_WINDOW_OFFSET: c_uint = 0x6;
// RISC-RISC semaphore/flag register (risc address 0x7016)
pub const RISC_SEMAPHORE: c_uint = 0x1UL;

pub const RISC_SEMAPHORE_FORCE: c_uint = 0x8000UL;

// RISC semaphore timeouts (ms)
pub const TIMEOUT_SEMAPHORE: c_int = 2500;
pub const TIMEOUT_SEMAPHORE_FORCE: c_int = 2000;
pub const TIMEOUT_TOTAL_ELAPSED: c_int = 4500;
// Trace Control
pub const TC_AEN_DISABLE: c_int = 0;
pub const TC_EFT_ENABLE: c_int = 4;
pub const TC_EFT_DISABLE: c_int = 5;
pub const TC_FCE_ENABLE: c_int = 8;
pub const TC_FCE_OPTIONS: c_int = 0;
pub const TC_FCE_DEFAULT_RX_SIZE: c_int = 2112;
pub const TC_FCE_DEFAULT_TX_SIZE: c_int = 2112;
pub const TC_FCE_DISABLE: c_int = 9;

// MID Support

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mid_conf_entry_24xx {
    pub reserved_1: u16,
//
// BIT 0  = Enable Hard Loop Id
// BIT 1  = Acquire Loop ID in LIPA
// BIT 2  = ID not Acquired
// BIT 3  = Enable VP
// BIT 4  = Enable Initiator Mode
// BIT 5  = Disable Target Mode
// BIT 6-7 = Reserved
//
    pub options: u8,
    pub hard_address: u8,
    pub port_name: [u8; WWN_SIZE],
    pub node_name: [u8; WWN_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mid_init_cb_24xx {
    pub init_cb: init_cb_24xx,
    pub count: __le16,
    pub options: __le16,
    pub entries: [mid_conf_entry_24xx; MAX_MULTI_ID_FABRIC],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mid_db_entry_24xx {
    pub status: u16,

    pub options: u8,
    pub hard_address: u8,
    pub port_name: [u8; WWN_SIZE],
    pub node_name: [u8; WWN_SIZE],
    pub port_id: [u8; 3],
    pub reserved_1: u8,
}

//
// Virtual Port Control IOCB
//
pub const VP_CTRL_IOCB_TYPE: c_uint = 0x30	/* Virtual Port Control entry. */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vp_ctrl_entry_24xx {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub vp_idx_failed: __le16,
    pub /: *mut *mut __le16 comp_status; / Completion status.,
pub const CS_VCE_IOCB_ERROR: c_uint = 0x01    /* Error processing IOCB */;
pub const CS_VCE_ACQ_ID_ERROR: c_uint = 0x02	/* Error while acquireing ID. */;
pub const CS_VCE_BUSY: c_uint = 0x05	/* Firmware not ready to accept cmd. */;
    pub command: __le16,
pub const VCE_COMMAND_ENABLE_VPS: c_uint = 0x00	/* Enable VPs. */;
pub const VCE_COMMAND_DISABLE_VPS: c_uint = 0x08	/* Disable VPs. */;
pub const VCE_COMMAND_DISABLE_VPS_REINIT: c_uint = 0x09 /* Disable VPs and reinit link. */;
pub const VCE_COMMAND_DISABLE_VPS_LOGO: c_uint = 0x0a /* Disable VPs and LOGO ports. */;
pub const VCE_COMMAND_DISABLE_VPS_LOGO_ALL: c_uint = 0x0b /* Disable VPs and LOGO ports. */;
    pub vp_count: __le16,
    pub vp_idx_map: [u8; 16],
    pub flags: __le16,
    pub id: __le16,
    pub reserved_4: u16,
    pub hopct: __le16,
    pub reserved_5: [u8; 24],
}

// vp_idx_map is a 128-bit (16-byte) bitmap selecting target VPs.

//
// Modify Virtual Port Configuration IOCB
//
pub const VP_CONFIG_IOCB_TYPE: c_uint = 0x31	/* Virtual Port Config entry. */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vp_config_entry_24xx {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub handle_count: u8,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub flags: __le16,

    pub /: *mut *mut __le16 comp_status; / Completion status.,
pub const CS_VCT_STS_ERROR: c_uint = 0x01	/* Specified VPs were not disabled. */;
pub const CS_VCT_CNT_ERROR: c_uint = 0x02	/* Invalid VP count. */;
pub const CS_VCT_ERROR: c_uint = 0x03	/* Unknown error. */;
pub const CS_VCT_IDX_ERROR: c_uint = 0x02	/* Invalid VP index. */;
pub const CS_VCT_BUSY: c_uint = 0x05	/* Firmware not ready to accept cmd. */;
    pub command: u8,
pub const VCT_COMMAND_MOD_VPS: c_uint = 0x00    /* Modify VP configurations. */;
pub const VCT_COMMAND_MOD_ENABLE_VPS: c_uint = 0x01 /* Modify configuration & enable VPs. */;
    pub vp_count: u8,
    pub vp_index1: u8,
    pub vp_index2: u8,
    pub options_idx1: u8,
    pub hard_address_idx1: u8,
    pub reserved_vp1: u16,
    pub port_name_idx1: [u8; WWN_SIZE],
    pub node_name_idx1: [u8; WWN_SIZE],
    pub options_idx2: u8,
    pub hard_address_idx2: u8,
    pub reserved_vp2: u16,
    pub port_name_idx2: [u8; WWN_SIZE],
    pub node_name_idx2: [u8; WWN_SIZE],
    pub id: __le16,
    pub reserved_4: u16,
    pub hopct: __le16,
    pub reserved_5: [u8; 2],
}

pub const VP_RPT_ID_IOCB_TYPE: c_uint = 0x32	/* Report ID Acquisition entry. */;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum VP_STATUS {
    VP_STAT_COMPL,
    VP_STAT_FAIL,
    VP_STAT_ID_CHG,
    VP_STAT_SNS_TO,				/* timeout */
    VP_STAT_SNS_RJT,
    VP_STAT_SCR_TO,				/* timeout */
    VP_STAT_SCR_RJT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum VP_FLAGS {
    VP_FLAGS_CON_FLOOP = 1,
    VP_FLAGS_CON_P2P = 2,
    VP_FLAGS_CON_FABRIC = 3,
    VP_FLAGS_NAME_VALID = BIT_5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vp_rpt_id_entry_24xx {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub resv1: __le32,
    pub vp_acquired: u8,
    pub vp_setup: u8,
    pub /: *mut *mut uint8_t vp_idx; / Format 0=reserved,
    pub /: *mut *mut uint8_t vp_status; / Format 0=reserved,
    pub port_id: [u8; 3],
    pub format: u8,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _f0 {
// format 0 loop
    pub vp_idx_map: [u8; 16],
    pub reserved_4: [u8; 32],
    pub f0: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _f1 {
// format 1 fabric
    pub /: *mut *mut uint8_t vpstat1_subcode; / vp_status=1 subcode,
    pub flags: u8,
pub const TOPO_MASK: c_uint = 0xE;
pub const TOPO_FL: c_uint = 0x2;
pub const TOPO_N2N: c_uint = 0x4;
pub const TOPO_F: c_uint = 0x6;
    pub fip_flags: __le16,
    pub rsv2: [u8; 12],
    pub ls_rjt_vendor: u8,
    pub ls_rjt_explanation: u8,
    pub ls_rjt_reason: u8,
    pub rsv3: [u8; 5],
    pub port_name: [u8; 8],
    pub node_name: [u8; 8],
    pub bbcr: __le16,
    pub reserved_5: [u8; 6],
    pub f1: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _f2 {
    pub vpstat1_subcode: u8,
    pub flags: u8,
    pub fip_flags: __le16,
    pub rsv2: [u8; 12],
    pub ls_rjt_vendor: u8,
    pub ls_rjt_explanation: u8,
    pub ls_rjt_reason: u8,
    pub rsv3: [u8; 5],
    pub port_name: [u8; 8],
    pub node_name: [u8; 8],
    pub bbcr: __le16,
    pub reserved_5: [u8; 2],
    pub remote_nport_id: [u8; 4],
    pub f2: },
    pub u: },
}

pub const VF_EVFP_IOCB_TYPE: c_uint = 0x26    /* Exchange Virtual Fabric Parameters entry. */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vf_evfp_entry_24xx {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub /: *mut *mut __le16 comp_status; / Completion status.,
    pub /: *mut *mut __le16 timeout; / timeout,
    pub adim_tagging_mode: __le16,
    pub vfport_id: __le16,
    pub exch_addr: u32,
    pub /: *mut *mut __le16 nport_handle; / N_PORT handle.,
    pub control_flags: __le16,
    pub io_parameter_0: u32,
    pub io_parameter_1: u32,
    pub /: *mut *mut __le64 tx_address __packed; / Data segment 0 address.,
    pub /: *mut *mut uint32_t tx_len; / Data segment 0 length.,
    pub /: *mut *mut __le64 rx_address __packed; / Data segment 1 address.,
    pub /: *mut *mut uint32_t rx_len; / Data segment 1 length.,
}

// END MID Support
// Flash Description Table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_fdt_layout {
    pub sig: [u8; 4],
    pub version: __le16,
    pub len: __le16,
    pub checksum: __le16,
    pub unused1: [u8; 2],
    pub model: [u8; 16],
    pub man_id: __le16,
    pub id: __le16,
    pub flags: u8,
    pub erase_cmd: u8,
    pub alt_erase_cmd: u8,
    pub wrt_enable_cmd: u8,
    pub wrt_enable_bits: u8,
    pub wrt_sts_reg_cmd: u8,
    pub unprotect_sec_cmd: u8,
    pub read_man_id_cmd: u8,
    pub block_size: __le32,
    pub alt_block_size: __le32,
    pub flash_size: __le32,
    pub wrt_enable_data: __le32,
    pub read_id_addr_len: u8,
    pub wrt_disable_bits: u8,
    pub read_dev_id_len: u8,
    pub chip_erase_cmd: u8,
    pub read_timeout: __le16,
    pub protect_sec_cmd: u8,
    pub unused2: [u8; 65],
}

// Flash Layout Table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_flt_location {
    pub sig: [u8; 4],
    pub start_lo: __le16,
    pub start_hi: __le16,
    pub version: u8,
    pub unused: [u8; 5],
    pub checksum: __le16,
}

pub const FLT_REG_FW: c_uint = 0x01;
pub const FLT_REG_BOOT_CODE: c_uint = 0x07;
pub const FLT_REG_VPD_0: c_uint = 0x14;
pub const FLT_REG_NVRAM_0: c_uint = 0x15;
pub const FLT_REG_VPD_1: c_uint = 0x16;
pub const FLT_REG_NVRAM_1: c_uint = 0x17;
pub const FLT_REG_VPD_2: c_uint = 0xD4;
pub const FLT_REG_NVRAM_2: c_uint = 0xD5;
pub const FLT_REG_VPD_3: c_uint = 0xD6;
pub const FLT_REG_NVRAM_3: c_uint = 0xD7;
pub const FLT_REG_FDT: c_uint = 0x1a;
pub const FLT_REG_FLT: c_uint = 0x1c;
pub const FLT_REG_HW_EVENT_0: c_uint = 0x1d;
pub const FLT_REG_HW_EVENT_1: c_uint = 0x1f;
pub const FLT_REG_NPIV_CONF_0: c_uint = 0x29;
pub const FLT_REG_NPIV_CONF_1: c_uint = 0x2a;
pub const FLT_REG_GOLD_FW: c_uint = 0x2f;
pub const FLT_REG_FCP_PRIO_0: c_uint = 0x87;
pub const FLT_REG_FCP_PRIO_1: c_uint = 0x88;
pub const FLT_REG_CNA_FW: c_uint = 0x97;
pub const FLT_REG_BOOT_CODE_8044: c_uint = 0xA2;
pub const FLT_REG_FCOE_FW: c_uint = 0xA4;
pub const FLT_REG_FCOE_NVRAM_0: c_uint = 0xAA;
pub const FLT_REG_FCOE_NVRAM_1: c_uint = 0xAC;
// 27xx
pub const FLT_REG_IMG_PRI_27XX: c_uint = 0x95;
pub const FLT_REG_IMG_SEC_27XX: c_uint = 0x96;
pub const FLT_REG_FW_SEC_27XX: c_uint = 0x02;
pub const FLT_REG_BOOTLOAD_SEC_27XX: c_uint = 0x9;
pub const FLT_REG_VPD_SEC_27XX_0: c_uint = 0x50;
pub const FLT_REG_VPD_SEC_27XX_1: c_uint = 0x52;
pub const FLT_REG_VPD_SEC_27XX_2: c_uint = 0xD8;
pub const FLT_REG_VPD_SEC_27XX_3: c_uint = 0xDA;
pub const FLT_REG_NVME_PARAMS_27XX: c_uint = 0x21;
pub const FLT_REG_FMB_PRI: c_uint = 0xDF;
pub const FLT_REG_FMB_SEC: c_uint = 0x124;
// 28xx
pub const FLT_REG_AUX_IMG_PRI_28XX: c_uint = 0x125;
pub const FLT_REG_AUX_IMG_SEC_28XX: c_uint = 0x126;
pub const FLT_REG_VPD_SEC_28XX_0: c_uint = 0x10C;
pub const FLT_REG_VPD_SEC_28XX_1: c_uint = 0x10E;
pub const FLT_REG_VPD_SEC_28XX_2: c_uint = 0x110;
pub const FLT_REG_VPD_SEC_28XX_3: c_uint = 0x112;
pub const FLT_REG_NVRAM_SEC_28XX_0: c_uint = 0x10D;
pub const FLT_REG_NVRAM_SEC_28XX_1: c_uint = 0x10F;
pub const FLT_REG_NVRAM_SEC_28XX_2: c_uint = 0x111;
pub const FLT_REG_NVRAM_SEC_28XX_3: c_uint = 0x113;
pub const FLT_REG_MPI_PRI_28XX: c_uint = 0xD3;
pub const FLT_REG_MPI_SEC_28XX: c_uint = 0xF0;
pub const FLT_REG_PEP_PRI_28XX: c_uint = 0xD1;
pub const FLT_REG_PEP_SEC_28XX: c_uint = 0xF1;
pub const FLT_REG_NVME_PARAMS_PRI_28XX: c_uint = 0x14E;
pub const FLT_REG_NVME_PARAMS_SEC_28XX: c_uint = 0x179;
// 29xx
pub const FLT_REG_MINI_FLT: c_uint = 0x201;
pub const FLT_REG_FW_DUMP_TMPLT: c_uint = 0x1A0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_flt_region {
    pub code: __le16,
    pub attribute: u8,
    pub reserved: u8,
    pub size: __le32,
    pub start: __le32,
    pub end: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_flt_header {
    pub version: __le16,
    pub length: __le16,
    pub checksum: __le16,
    pub unused: __le16,
    pub region: [qla_flt_region; ],
}

pub const FLT_REGION_SIZE: c_int = 16;
pub const FLT_MAX_REGIONS: c_uint = 0xFF;

// 29xx
pub const FLT_HDR_VERSION: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_flt_region_header {
    pub signature: __le32,
    pub version: __le32,
    pub length: __le32,
    pub checksum: __le32,
    pub region_count: __le16,
    pub region_size: __le16,
    pub segment_size: __le32,
    pub res3: __le32,
    pub res4: __le32,
    pub res5: __le32,
    pub res6: __le32,
    pub res7: __le32,
    pub res8: __le32,
    pub res9: __le32,
    pub res10: __le32,
    pub res11: __le32,
    pub res12: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_flt_region_data {
    pub region_code: __le16,
    pub reserved: __le16,
    pub attribute: __le32,
    pub image_length: __le32,
    pub mbi_offset: __le32,
    pub version: __le32,
    pub card_type: __le32,
    pub chip_revision: __le32,
    pub res4: __le32,
    pub res5: __le32,
    pub res6: __le32,
    pub res7: __le32,
    pub res8: __le32,
    pub res9: __le32,
    pub res10: __le32,
    pub res11: __le32,
    pub res12: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_flash_layout {
    pub flt_header: qla_flt_region_header,
    pub region: [qla_flt_region_data; ],
}

pub const FLT_DATA_MAX_REGIONS: c_uint = 0xFF;
// Flash NPIV Configuration Table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_npiv_header {
    pub sig: [u8; 2],
    pub version: __le16,
    pub entries: __le16,
    pub unused: [__le16; 4],
    pub checksum: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_npiv_entry {
    pub flags: __le16,
    pub vf_id: __le16,
    pub q_qos: u8,
    pub f_qos: u8,
    pub unused1: __le16,
    pub port_name: [u8; WWN_SIZE],
    pub node_name: [u8; WWN_SIZE],
}

// 84XX Support
pub const MBA_ISP84XX_ALERT: c_uint = 0x800f  /* Alert Notification. */;
pub const A84_PANIC_RECOVERY: c_uint = 0x1;
pub const A84_OP_LOGIN_COMPLETE: c_uint = 0x2;
pub const A84_DIAG_LOGIN_COMPLETE: c_uint = 0x3;
pub const A84_GOLD_LOGIN_COMPLETE: c_uint = 0x4;
pub const MBC_ISP84XX_RESET: c_uint = 0x3a    /* Reset. */;

pub const VERIFY_CHIP_IOCB_TYPE: c_uint = 0x1B;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct verify_chip_entry_84xx {
    pub entry_type: u8,
    pub entry_count: u8,
    pub sys_defined: u8,
    pub entry_status: u8,
    pub handle: u32,
    pub options: __le16,

    pub reserved_1: __le16,
    pub data_seg_cnt: __le16,
    pub reserved_2: [__le16; 3],
    pub fw_ver: __le32,
    pub exchange_address: __le32,
    pub reserved_3: [__le32; 3],
    pub fw_size: __le32,
    pub fw_seq_size: __le32,
    pub relative_offset: __le32,
    pub dsd: dsd64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct verify_chip_rsp_84xx {
    pub entry_type: u8,
    pub entry_count: u8,
    pub sys_defined: u8,
    pub entry_status: u8,
    pub handle: u32,
    pub comp_status: __le16,
pub const CS_VCS_CHIP_FAILURE: c_uint = 0x3;
pub const CS_VCS_BAD_EXCHANGE: c_uint = 0x8;
pub const CS_VCS_SEQ_COMPLETEi: c_uint = 0x40;
    pub failure_code: __le16,
pub const VFC_CHECKSUM_ERROR: c_uint = 0x1;
pub const VFC_INVALID_LEN: c_uint = 0x2;
pub const VFC_ALREADY_IN_PROGRESS: c_uint = 0x8;
    pub reserved_1: [__le16; 4],
    pub fw_ver: __le32,
    pub exchange_address: __le32,
    pub reserved_2: [__le32; 6],
}

pub const ACCESS_CHIP_IOCB_TYPE: c_uint = 0x2B;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct access_chip_84xx {
    pub entry_type: u8,
    pub entry_count: u8,
    pub sys_defined: u8,
    pub entry_status: u8,
    pub handle: u32,
    pub options: __le16,
pub const ACO_DUMP_MEMORY: c_uint = 0x0;
pub const ACO_LOAD_MEMORY: c_uint = 0x1;
pub const ACO_CHANGE_CONFIG_PARAM: c_uint = 0x2;
pub const ACO_REQUEST_INFO: c_uint = 0x3;
    pub reserved1: __le16,
    pub dseg_count: __le16,
    pub reserved2: [__le16; 3],
    pub parameter1: __le32,
    pub parameter2: __le32,
    pub parameter3: __le32,
    pub reserved3: [__le32; 3],
    pub total_byte_cnt: __le32,
    pub reserved4: __le32,
    pub dsd: dsd64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct access_chip_rsp_84xx {
    pub entry_type: u8,
    pub entry_count: u8,
    pub sys_defined: u8,
    pub entry_status: u8,
    pub handle: u32,
    pub comp_status: __le16,
    pub failure_code: __le16,
    pub residual_count: __le32,
    pub reserved: [__le32; 12],
}

// 81XX Support
pub const MBA_DCBX_START: c_uint = 0x8016;
pub const MBA_DCBX_COMPLETE: c_uint = 0x8030;
pub const MBA_FCF_CONF_ERR: c_uint = 0x8031;
pub const MBA_DCBX_PARAM_UPDATE: c_uint = 0x8032;
pub const MBA_IDC_COMPLETE: c_uint = 0x8100;
pub const MBA_IDC_NOTIFY: c_uint = 0x8101;
pub const MBA_IDC_TIME_EXT: c_uint = 0x8102;
pub const MBC_IDC_ACK: c_uint = 0x101;
pub const MBC_RESTART_MPI_FW: c_uint = 0x3d;
pub const MBC_FLASH_ACCESS_CTRL: c_uint = 0x3e	/* Control flash access. */;
pub const MBC_GET_XGMAC_STATS: c_uint = 0x7a;
pub const MBC_GET_DCBX_PARAMS: c_uint = 0x51;
//
// ISP83xx mailbox commands
//
pub const MBC_WRITE_REMOTE_REG: c_uint = 0x0001 /* Write remote register */;
pub const MBC_READ_REMOTE_REG: c_uint = 0x0009 /* Read remote register */;
pub const MBC_RESTART_NIC_FIRMWARE: c_uint = 0x003d /* Restart NIC firmware */;
pub const MBC_SET_ACCESS_CONTROL: c_uint = 0x003e /* Access control command */;
// Flash access control option field bit definitions

pub const FAC_OPT_CMD_SUBCODE: c_uint = 0xff;
// Flash access control command subcodes
pub const FAC_OPT_CMD_WRITE_PROTECT: c_uint = 0x00;
pub const FAC_OPT_CMD_WRITE_ENABLE: c_uint = 0x01;
pub const FAC_OPT_CMD_ERASE_SECTOR: c_uint = 0x02;
pub const FAC_OPT_CMD_LOCK_SEMAPHORE: c_uint = 0x03;
pub const FAC_OPT_CMD_UNLOCK_SEMAPHORE: c_uint = 0x04;
pub const FAC_OPT_CMD_GET_SECTOR_SIZE: c_uint = 0x05;
// enhanced features bit definitions

// LR Distance bit positions
pub const LR_DIST_NV_POS: c_int = 2;
pub const LR_DIST_NV_MASK: c_uint = 0xf;
pub const LR_DIST_FW_POS: c_int = 12;
// FAC semaphore defines
pub const FAC_SEMAPHORE_UNLOCK: c_int = 0;
pub const FAC_SEMAPHORE_LOCK: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvram_81xx {
// NVRAM header.
    pub id: [u8; 4],
    pub nvram_version: __le16,
    pub reserved_0: __le16,
// Firmware Initialization Control Block.
    pub version: __le16,
    pub reserved_1: __le16,
    pub frame_payload_size: __le16,
    pub execution_throttle: __le16,
    pub exchange_count: __le16,
    pub reserved_2: __le16,
    pub port_name: [u8; WWN_SIZE],
    pub node_name: [u8; WWN_SIZE],
    pub login_retry_count: __le16,
    pub reserved_3: __le16,
    pub interrupt_delay_timer: __le16,
    pub login_timeout: __le16,
    pub firmware_options_1: __le32,
    pub firmware_options_2: __le32,
    pub firmware_options_3: __le32,
    pub reserved_4: [__le16; 4],
// Offset 64.
    pub enode_mac: [u8; 6],
    pub reserved_5: [__le16; 5],
// Offset 80.
    pub reserved_6: [__le16; 24],
// Offset 128.
    pub ex_version: __le16,
    pub prio_fcf_matching_flags: u8,
    pub reserved_6_1: [u8; 3],
    pub pri_fcf_vlan_id: __le16,
    pub pri_fcf_fabric_name: [u8; 8],
    pub reserved_6_2: [__le16; 7],
    pub spma_mac_addr: [u8; 6],
    pub reserved_6_3: [__le16; 14],
// Offset 192.
    pub min_supported_speed: u8,
    pub reserved_7_0: u8,
    pub reserved_7: [__le16; 31],
//
// BIT 0  = Enable spinup delay
// BIT 1  = Disable BIOS
// BIT 2  = Enable Memory Map BIOS
// BIT 3  = Enable Selectable Boot
// BIT 4  = Disable RISC code load
// BIT 5  = Disable Serdes
// BIT 6  = Opt boot mode
// BIT 7  = Interrupt enable
//
// BIT 8  = EV Control enable
// BIT 9  = Enable lip reset
// BIT 10 = Enable lip full login
// BIT 11 = Enable target reset
// BIT 12 = Stop firmware
// BIT 13 = Enable nodename option
// BIT 14 = Default WWPN valid
// BIT 15 = Enable alternate WWN
//
// BIT 16 = CLP LUN string
// BIT 17 = CLP Target string
// BIT 18 = CLP BIOS enable string
// BIT 19 = CLP Serdes string
// BIT 20 = CLP WWPN string
// BIT 21 = CLP WWNN string
// BIT 22 =
// BIT 23 =
// BIT 24 = Keep WWPN
// BIT 25 = Temp WWPN
// BIT 26-31 =
//
    pub host_p: __le32,
    pub alternate_port_name: [u8; WWN_SIZE],
    pub alternate_node_name: [u8; WWN_SIZE],
    pub boot_port_name: [u8; WWN_SIZE],
    pub boot_lun_number: __le16,
    pub reserved_8: __le16,
    pub alt1_boot_port_name: [u8; WWN_SIZE],
    pub alt1_boot_lun_number: __le16,
    pub reserved_9: __le16,
    pub alt2_boot_port_name: [u8; WWN_SIZE],
    pub alt2_boot_lun_number: __le16,
    pub reserved_10: __le16,
    pub alt3_boot_port_name: [u8; WWN_SIZE],
    pub alt3_boot_lun_number: __le16,
    pub reserved_11: __le16,
//
// BIT 0 = Selective Login
// BIT 1 = Alt-Boot Enable
// BIT 2 = Reserved
// BIT 3 = Boot Order List
// BIT 4 = Reserved
// BIT 5 = Selective LUN
// BIT 6 = Reserved
// BIT 7-31 =
//
    pub efi_parameters: __le32,
    pub reset_delay: u8,
    pub reserved_12: u8,
    pub reserved_13: __le16,
    pub boot_id_number: __le16,
    pub reserved_14: __le16,
    pub max_luns_per_target: __le16,
    pub reserved_15: __le16,
    pub port_down_retry_count: __le16,
    pub link_down_timeout: __le16,
// FCode parameters.
    pub fcode_parameter: __le16,
    pub reserved_16: [__le16; 3],
// Offset 352.
    pub reserved_17: [u8; 4],
    pub reserved_18: [__le16; 5],
    pub reserved_19: [u8; 2],
    pub reserved_20: [__le16; 8],
// Offset 384.
    pub reserved_21: [u8; 16],
    pub reserved_22: [__le16; 3],
// Offset 406 (0x196) Enhanced Features
// BIT 0    = Extended BB credits for LR
// BIT 1    = Virtual Fabric Enable
// BIT 2-5  = Distance Support if BIT 0 is on
// BIT 6    = Prefer FCP
// BIT 7    = SCM Disabled if BIT is set (1)
// BIT 8-15 = Unused
//
    pub enhanced_features: __le16,
    pub reserved_24: [u16; 4],
// Offset 416.
    pub reserved_25: [__le16; 32],
// Offset 480.
    pub model_name: [u8; 16],
// Offset 496.
    pub feature_mask_l: __le16,
    pub feature_mask_h: __le16,
    pub reserved_26: [__le16; 2],
    pub subsystem_vendor_id: __le16,
    pub subsystem_device_id: __le16,
    pub checksum: __le32,
}

//
// ISP Initialization Control Block.
// Little endian except where noted.
//
pub const ICB_VERSION: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct init_cb_81xx {
    pub version: __le16,
    pub reserved_1: __le16,
    pub frame_payload_size: __le16,
    pub execution_throttle: __le16,
    pub exchange_count: __le16,
    pub reserved_2: __le16,
    pub /: *mut *mut uint8_t port_name[WWN_SIZE]; / Big endian.,
    pub /: *mut *mut uint8_t node_name[WWN_SIZE]; / Big endian.,
    pub response_q_inpointer: __le16,
    pub request_q_outpointer: __le16,
    pub login_retry_count: __le16,
    pub prio_request_q_outpointer: __le16,
    pub response_q_length: __le16,
    pub request_q_length: __le16,
    pub reserved_3: __le16,
    pub prio_request_q_length: __le16,
    pub __packed: __le64 request_q_address,
    pub __packed: __le64 response_q_address,
    pub __packed: __le64 prio_request_q_address,
    pub reserved_4: [u8; 8],
    pub atio_q_inpointer: __le16,
    pub atio_q_length: __le16,
    pub __packed: __le64 atio_q_address,
    pub /: *mut *mut __le16 interrupt_delay_timer; / 100us increments.,
    pub login_timeout: __le16,
//
// BIT 0-3 = Reserved
// BIT 4  = Enable Target Mode
// BIT 5  = Disable Initiator Mode
// BIT 6  = Reserved
// BIT 7  = Reserved
//
// BIT 8-13 = Reserved
// BIT 14 = Node Name Option
// BIT 15-31 = Reserved
//
    pub firmware_options_1: __le32,
//
// BIT 0  = Operation Mode bit 0
// BIT 1  = Operation Mode bit 1
// BIT 2  = Operation Mode bit 2
// BIT 3  = Operation Mode bit 3
// BIT 4-7 = Reserved
//
// BIT 8  = Enable Class 2
// BIT 9  = Enable ACK0
// BIT 10 = Reserved
// BIT 11 = Enable FC-SP Security
// BIT 12 = FC Tape Enable
// BIT 13 = Reserved
// BIT 14 = Enable Target PRLI Control
// BIT 15-31 = Reserved
//
    pub firmware_options_2: __le32,
//
// BIT 0-3 = Reserved
// BIT 4  = FCP RSP Payload bit 0
// BIT 5  = FCP RSP Payload bit 1
// BIT 6  = Enable Receive Out-of-Order data frame handling
// BIT 7  = Reserved
//
// BIT 8  = Reserved
// BIT 9  = Enable Out-of-Order FCP_XFER_RDY relative offset handling
// BIT 10-16 = Reserved
// BIT 17 = Enable multiple FCFs
// BIT 18-20 = MAC addressing mode
// BIT 21-25 = Ethernet data rate
// BIT 26 = Enable ethernet header rx IOCB for ATIO q
// BIT 27 = Enable ethernet header rx IOCB for response q
// BIT 28 = SPMA selection bit 0
// BIT 28 = SPMA selection bit 1
// BIT 30-31 = Reserved
//
    pub firmware_options_3: __le32,
    pub reserved_5: [u8; 8],
    pub enode_mac: [u8; 6],
    pub reserved_6: [u8; 10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mid_init_cb_81xx {
    pub init_cb: init_cb_81xx,
    pub count: u16,
    pub options: u16,
    pub entries: [mid_conf_entry_24xx; MAX_MULTI_ID_FABRIC],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ex_init_cb_81xx {
    pub ex_version: u16,
    pub prio_fcf_matching_flags: u8,
    pub reserved_1: [u8; 3],
    pub pri_fcf_vlan_id: u16,
    pub pri_fcf_fabric_name: [u8; 8],
    pub reserved_2: [u16; 7],
    pub spma_mac_addr: [u8; 6],
    pub reserved_3: [u16; 14],
}

pub const FARX_ACCESS_FLASH_CONF_81XX: c_uint = 0x7FFD0000;
pub const FARX_ACCESS_FLASH_DATA_81XX: c_uint = 0x7F800000;
pub const FARX_ACCESS_FLASH_CONF_28XX: c_uint = 0x7FFD0000;
pub const FARX_ACCESS_FLASH_DATA_28XX: c_uint = 0x7F7D0000;
// FCP priority config defines
// operations
pub const QLFC_FCP_PRIO_DISABLE: c_uint = 0x0;
pub const QLFC_FCP_PRIO_ENABLE: c_uint = 0x1;
pub const QLFC_FCP_PRIO_GET_CONFIG: c_uint = 0x2;
pub const QLFC_FCP_PRIO_SET_CONFIG: c_uint = 0x3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_fcp_prio_entry {
    pub /: *mut *mut uint16_t flags; / Describes parameter(s) in FCP,
// priority entry that are valid
pub const FCP_PRIO_ENTRY_VALID: c_uint = 0x1;
pub const FCP_PRIO_ENTRY_TAG_VALID: c_uint = 0x2;
pub const FCP_PRIO_ENTRY_SPID_VALID: c_uint = 0x4;
pub const FCP_PRIO_ENTRY_DPID_VALID: c_uint = 0x8;
pub const FCP_PRIO_ENTRY_LUNB_VALID: c_uint = 0x10;
pub const FCP_PRIO_ENTRY_LUNE_VALID: c_uint = 0x20;
pub const FCP_PRIO_ENTRY_SWWN_VALID: c_uint = 0x40;
pub const FCP_PRIO_ENTRY_DWWN_VALID: c_uint = 0x80;
    pub /: *mut *mut uint8_t tag; / Priority value,
    pub /: *mut *mut uint8_t reserved; / Reserved for future use,
    pub /: *mut *mut uint32_t src_pid; / Src port id. high order byte,
// unused; -1 (wild card)
    pub /: *mut *mut uint32_t dst_pid; / Src port id. high order byte,
// unused; -1 (wild card)
    pub /: *mut *mut uint16_t lun_beg; / 1st lun num of lun range.,
// -1 (wild card)
    pub /: *mut *mut uint16_t lun_end; / 2nd lun num of lun range.,
// -1 (wild card)
    pub /: *mut *mut uint8_t src_wwpn[8]; / Source WWPN: -1 (wild card),
    pub /: *mut *mut uint8_t dst_wwpn[8]; / Destination WWPN: -1 (wild card),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_fcp_prio_cfg {
    pub /: *mut *mut uint8_t signature[4]; / "HQOS" signature of config data,
    pub /: *mut *mut uint16_t version; / 1: Initial version,
    pub /: *mut *mut uint16_t length; / config data size in num bytes,
    pub /: *mut *mut uint16_t checksum; / config data bytes checksum,
    pub /: *mut *mut uint16_t num_entries; / Number of entries,
    pub /: *mut *mut uint16_t size_of_entry; / Size of each entry in num bytes,
    pub /: *mut *mut uint8_t attributes; / enable/disable, persistence,
pub const FCP_PRIO_ATTR_DISABLE: c_uint = 0x0;
pub const FCP_PRIO_ATTR_ENABLE: c_uint = 0x1;
pub const FCP_PRIO_ATTR_PERSIST: c_uint = 0x2;
    pub /: *mut *mut uint8_t reserved; / Reserved for future use,

    pub /: *mut *mut qla_fcp_prio_entry entry[1023]; / fcp priority entries,
    pub reserved2: [u8; 16],
}

// 25XX Support
pub const FA_FCP_PRIO0_ADDR_25: c_uint = 0x3C000;
pub const FA_FCP_PRIO1_ADDR_25: c_uint = 0x3E000;
// 81XX Flash locations -- occupies second 2MB region.
pub const FA_BOOT_CODE_ADDR_81: c_uint = 0x80000;
pub const FA_RISC_CODE_ADDR_81: c_uint = 0xA0000;
pub const FA_FW_AREA_ADDR_81: c_uint = 0xC0000;
pub const FA_VPD_NVRAM_ADDR_81: c_uint = 0xD0000;
pub const FA_VPD0_ADDR_81: c_uint = 0xD0000;
pub const FA_VPD1_ADDR_81: c_uint = 0xD0400;
pub const FA_NVRAM0_ADDR_81: c_uint = 0xD0080;
pub const FA_NVRAM1_ADDR_81: c_uint = 0xD0180;
pub const FA_FEATURE_ADDR_81: c_uint = 0xD4000;
pub const FA_FLASH_DESCR_ADDR_81: c_uint = 0xD8000;
pub const FA_FLASH_LAYOUT_ADDR_81: c_uint = 0xD8400;
pub const FA_HW_EVENT0_ADDR_81: c_uint = 0xDC000;
pub const FA_HW_EVENT1_ADDR_81: c_uint = 0xDC400;
pub const FA_NPIV_CONF0_ADDR_81: c_uint = 0xD1000;
pub const FA_NPIV_CONF1_ADDR_81: c_uint = 0xD2000;
// 83XX Flash locations -- occupies second 8MB region.

pub const NVRAM_DUAL_FCP_NVME_FLAG_OFFSET: c_uint = 0x196;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_fmb_version {
    pub major: u8,
    pub minor: u8,
    pub sub: u8,
    pub build: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_fmb_upd_time {
    pub year: __le16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub reserved: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_flash_memo_block {
    pub /: *mut *mut __le32 signature; / "FMBS",

    pub length: __le32,
    pub version: __le32,
pub const QLFC_FMB_VERSION: c_int = 3;
    pub checksum: __le32,
    pub ffv_ver: qla_fmb_version,
    pub mbi_ver: qla_fmb_version,
    pub year: __le16,
    pub month: u8,
    pub day: u8,
    pub reserve: [u8; 4],
    pub bld_time: },
    pub tool_id: [u8; 4],
    pub upd_time: qla_fmb_upd_time,
    pub tool_version: qla_fmb_version,
}

pub const TIM_DEST_ADDR: c_uint = 0xffffffff;
pub const CHUNK_SIZE: c_uint = 0x10000;
pub const TIM: c_int = 0;
pub const ARR1: c_int = 1;
pub const ARR2: c_int = 2;
pub const ARR3: c_int = 3;
pub const ARR4: c_int = 4;
pub const LD_FL_HEADER_SIGNATURE: c_uint = 0x46434F50;
pub const LD_FL_HEADER_VERSION: c_uint = 0x01;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcop_header {
    pub signature: u32,
    pub header_length: u32,
    pub header_version: u32,
    pub segment_size: u32,
    pub tim_length: u32,
    pub fc_major_version: u32,
    pub fc_minor_version: u32,
    pub fc_subminor_version: u32,
    pub array1_length: u32,
    pub array1_destination_addr: u32,
    pub array2_length: u32,
    pub array2_destination_addr: u32,
    pub array3_length: u32,
    pub array4_length: u32,
    pub attribute: u32,
    pub extended_attribute: u32,
    pub reserved0: u32,
    pub reserved1: u32,
    pub reserved2: u32,
    pub image_checksum: u32,
}
