//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qlogic/qed/qed_mcp.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
// QLogic qed NIC Driver
// Copyright (c) 2015-2017  QLogic Corporation
// Copyright (c) 2019-2020 Marvell International Ltd.
//

pub const QED_MFW_REPORT_STR_SIZE: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_mcp_link_speed_params {
    pub autoneg: bool,
    pub advertised_speeds: u32,
pub const QED_EXT_SPEED_MASK_RES: c_uint = 0x1;
pub const QED_EXT_SPEED_MASK_1G: c_uint = 0x2;
pub const QED_EXT_SPEED_MASK_10G: c_uint = 0x4;
pub const QED_EXT_SPEED_MASK_20G: c_uint = 0x8;
pub const QED_EXT_SPEED_MASK_25G: c_uint = 0x10;
pub const QED_EXT_SPEED_MASK_40G: c_uint = 0x20;
pub const QED_EXT_SPEED_MASK_50G_R: c_uint = 0x40;
pub const QED_EXT_SPEED_MASK_50G_R2: c_uint = 0x80;
pub const QED_EXT_SPEED_MASK_100G_R2: c_uint = 0x100;
pub const QED_EXT_SPEED_MASK_100G_R4: c_uint = 0x200;
pub const QED_EXT_SPEED_MASK_100G_P4: c_uint = 0x400;
    pub /: *mut *mut u32 forced_speed; / In Mb/s,
pub const QED_EXT_SPEED_1G: c_uint = 0x1;
pub const QED_EXT_SPEED_10G: c_uint = 0x2;
pub const QED_EXT_SPEED_20G: c_uint = 0x4;
pub const QED_EXT_SPEED_25G: c_uint = 0x8;
pub const QED_EXT_SPEED_40G: c_uint = 0x10;
pub const QED_EXT_SPEED_50G_R: c_uint = 0x20;
pub const QED_EXT_SPEED_50G_R2: c_uint = 0x40;
pub const QED_EXT_SPEED_100G_R2: c_uint = 0x80;
pub const QED_EXT_SPEED_100G_R4: c_uint = 0x100;
pub const QED_EXT_SPEED_100G_P4: c_uint = 0x200;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_mcp_link_pause_params {
    pub autoneg: bool,
    pub forced_rx: bool,
    pub forced_tx: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_mcp_eee_mode {
    QED_MCP_EEE_DISABLED,
    QED_MCP_EEE_ENABLED,
    QED_MCP_EEE_UNSUPPORTED
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_mcp_link_params {
    pub speed: qed_mcp_link_speed_params,
    pub pause: qed_mcp_link_pause_params,
    pub loopback_mode: u32,
    pub eee: qed_link_eee_params,
    pub fec: u32,
    pub ext_speed: qed_mcp_link_speed_params,
    pub ext_fec_mode: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_mcp_link_capabilities {
    pub speed_capabilities: u32,
    pub default_speed_autoneg: bool,
    pub fec_default: u32,
    pub default_eee: qed_mcp_eee_mode,
    pub eee_lpi_timer: u32,
    pub eee_speed_caps: u8,
    pub default_ext_speed_caps: u32,
    pub default_ext_autoneg: u32,
    pub default_ext_speed: u32,
    pub default_ext_fec: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_mcp_link_state {
    pub link_up: bool,
    pub min_pf_rate: u32,
// Actual link speed in Mb/s
    pub line_speed: u32,
// PF max speed in Mb/s, deduced from line_speed
// according to PF max bandwidth configuration.
//
    pub speed: u32,
    pub full_duplex: bool,
    pub an: bool,
    pub an_complete: bool,
    pub parallel_detection: bool,
    pub pfc_enabled: bool,
    pub partner_adv_speed: u32,

    pub partner_tx_flow_ctrl_en: bool,
    pub partner_rx_flow_ctrl_en: bool,
    pub partner_adv_pause: u8,
pub const QED_LINK_PARTNER_SYMMETRIC_PAUSE: c_uint = 0x1;
pub const QED_LINK_PARTNER_ASYMMETRIC_PAUSE: c_uint = 0x2;
pub const QED_LINK_PARTNER_BOTH_PAUSE: c_uint = 0x3;
    pub sfp_tx_fault: bool,
    pub eee_active: bool,
    pub eee_adv_caps: u8,
    pub eee_lp_adv_caps: u8,
    pub fec_active: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_mcp_function_info {
    pub pause_on_host: u8,
    pub protocol: qed_pci_personality,
    pub bandwidth_min: u8,
    pub bandwidth_max: u8,
    pub mac: [u8; ETH_ALEN],
    pub wwn_port: u64,
    pub wwn_node: u64,

    pub ovlan: u16,
    pub mtu: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_mcp_nvm_common {
    pub offset: u32,
    pub param: u32,
    pub resp: u32,
    pub cmd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_mcp_drv_version {
    pub version: u32,
    pub 4]: u8 name[MCP_DRV_VER_STR_SIZE -,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_mcp_lan_stats {
    pub ucast_rx_pkts: u64,
    pub ucast_tx_pkts: u64,
    pub fcs_err: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_mcp_fcoe_stats {
    pub rx_pkts: u64,
    pub tx_pkts: u64,
    pub fcs_err: u32,
    pub login_failure: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_mcp_iscsi_stats {
    pub rx_pdus: u64,
    pub tx_pdus: u64,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_mcp_rdma_stats {
    pub rx_pkts: u64,
    pub tx_pkts: u64,
    pub rx_bytes: u64,
    pub tx_byts: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_mcp_protocol_type {
    QED_MCP_LAN_STATS,
    QED_MCP_FCOE_STATS,
    QED_MCP_ISCSI_STATS,
    QED_MCP_RDMA_STATS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union qed_mcp_protocol_stats {
    pub lan_stats: qed_mcp_lan_stats,
    pub fcoe_stats: qed_mcp_fcoe_stats,
    pub iscsi_stats: qed_mcp_iscsi_stats,
    pub rdma_stats: qed_mcp_rdma_stats,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_ov_eswitch {
    QED_OV_ESWITCH_NONE,
    QED_OV_ESWITCH_VEB,
    QED_OV_ESWITCH_VEPA
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_ov_client {
    QED_OV_CLIENT_DRV,
    QED_OV_CLIENT_USER,
    QED_OV_CLIENT_VENDOR_SPEC
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_ov_driver_state {
    QED_OV_DRIVER_STATE_NOT_LOADED,
    QED_OV_DRIVER_STATE_DISABLED,
    QED_OV_DRIVER_STATE_ACTIVE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_ov_wol {
    QED_OV_WOL_DEFAULT,
    QED_OV_WOL_DISABLED,
    QED_OV_WOL_ENABLED
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_mfw_tlv_type {
    QED_MFW_TLV_GENERIC = 0x1,	/* Core driver TLVs */
    QED_MFW_TLV_ETH = 0x2,		/* L2 driver TLVs */
    QED_MFW_TLV_FCOE = 0x4,		/* FCoE protocol TLVs */
    QED_MFW_TLV_ISCSI = 0x8,	/* SCSI protocol TLVs */
    QED_MFW_TLV_MAX = 0x16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_mfw_tlv_generic {
pub const QED_MFW_TLV_FLAGS_SIZE: c_int = 2;
    pub ipv4_csum_offload: u8,
    pub lso_supported: u8,
    pub b_set: bool,
    pub flags: },
pub const QED_MFW_TLV_MAC_COUNT: c_int = 3;
// First entry for primary MAC, 2 secondary MACs possible
    pub mac: [u8; QED_MFW_TLV_MAC_COUNT][6],
    pub mac_set: [bool; QED_MFW_TLV_MAC_COUNT],
    pub rx_frames: u64,
    pub rx_frames_set: bool,
    pub rx_bytes: u64,
    pub rx_bytes_set: bool,
    pub tx_frames: u64,
    pub tx_frames_set: bool,
    pub tx_bytes: u64,
    pub tx_bytes_set: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union qed_mfw_tlv_data {
    pub generic: qed_mfw_tlv_generic,
    pub eth: qed_mfw_tlv_eth,
    pub fcoe: qed_mfw_tlv_fcoe,
    pub iscsi: qed_mfw_tlv_iscsi,
}

//
// qed_mcp_get_link_params(): Returns the link params of the hw function.
//
// @p_hwfn: HW device data.
//
// Returns: Pointer to link params.
//
// qed_mcp_get_link_state(): Return the link state of the hw function.
//
// @p_hwfn: HW device data.
//
// Returns: Pointer to link state.
//
// qed_mcp_get_link_capabilities(): Return the link capabilities of the
// hw function.
//
// @p_hwfn: HW device data.
//
// Returns: Pointer to link capabilities.
//
// qed_mcp_get_link_capabilities(struct qed_hwfn *p_hwfn);
//
// qed_mcp_set_link(): Request the MFW to set the link according
// to 'link_input'.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @b_up: Raise link if `true'. Reset link if `false'.
//
// Return: Int.
//
// qed_mcp_get_mfw_ver(): Get the management firmware version value.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @p_mfw_ver: MFW version value.
// @p_running_bundle_id: Image id in nvram; Optional.
//
// Return: Int - 0 - operation was successful.
//
// qed_mcp_get_mbi_ver(): Get the MBI version value.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @p_mbi_ver: A pointer to a variable to be filled with the MBI version.
//
// Return: Int - 0 - operation was successful.
//
// qed_mcp_get_media_type(): Get media type value of the port.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @media_type: Media type value
//
// Return: Int - 0 - Operation was successul.
// -EBUSY - Operation failed
//
// qed_mcp_get_transceiver_data(): Get transceiver data of the port.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @p_transceiver_state: Transceiver state.
// @p_tranceiver_type: Media type value.
//
// Return: Int - 0 - Operation was successul.
// -EBUSY - Operation failed
//
// qed_mcp_trans_speed_mask(): Get transceiver supported speed mask.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @p_speed_mask: Bit mask of all supported speeds.
//
// Return: Int - 0 - Operation was successul.
// -EBUSY - Operation failed
//
// qed_mcp_get_board_config(): Get board configuration.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @p_board_config: Board config.
//
// Return: Int - 0 - Operation was successul.
// -EBUSY - Operation failed
//
// qed_mcp_cmd(): Sleepable function for sending commands to the MCP
// mailbox. It acquire mutex lock for the entire
// operation, from sending the request until the MCP
// response. Waiting for MCP response will be checked up
// to 5 seconds every 10ms. Should not be called from atomic
// context.
//
// @p_hwfn: HW device data.
// @p_ptt: PTT required for register access.
// @cmd: command to be sent to the MCP.
// @param: Optional param
// @o_mcp_resp: The MCP response code (exclude sequence).
// @o_mcp_param: Optional parameter provided by the MCP
// response
//
// Return: Int - 0 - Operation was successul.
//
// qed_mcp_cmd_nosleep(): Function for sending commands to the MCP
// mailbox. It acquire mutex lock for the entire
// operation, from sending the request until the MCP
// response. Waiting for MCP response will be checked up
// to 5 seconds every 10us. Should be called when sleep
// is not allowed.
//
// @p_hwfn: HW device data.
// @p_ptt: PTT required for register access.
// @cmd: command to be sent to the MCP.
// @param: Optional param
// @o_mcp_resp: The MCP response code (exclude sequence).
// @o_mcp_param: Optional parameter provided by the MCP
// response
//
// Return: Int - 0 - Operation was successul.
//
// qed_mcp_drain(): drains the nig, allowing completion to pass in
// case of pauses.
// (Should be called only from sleepable context)
//
// @p_hwfn: HW device data.
// @p_ptt: PTT required for register access.
//
// Return: Int.
//
// qed_mcp_get_flash_size(): Get the flash size value.
//
// @p_hwfn: HW device data.
// @p_ptt: PTT required for register access.
// @p_flash_size: Flash size in bytes to be filled.
//
// Return: Int - 0 - Operation was successul.
//
// qed_mcp_send_drv_version(): Send driver version to MFW.
//
// @p_hwfn: HW device data.
// @p_ptt: PTT required for register access.
// @p_ver: Version value.
//
// Return: Int - 0 - Operation was successul.
//
// qed_get_process_kill_counter(): Read the MFW process kill counter.
//
// @p_hwfn: HW device data.
// @p_ptt: PTT required for register access.
//
// Return: u32.
//
// qed_start_recovery_process(): Trigger a recovery process.
//
// @p_hwfn: HW device data.
// @p_ptt: PTT required for register access.
//
// Return: Int.
//
extern "C" {
    pub fn qed_start_recovery_process(p_hwfn: *mut qed_hwfn, p_ptt: *mut qed_ptt) -> c_int;
}
//
// qed_recovery_prolog(): A recovery handler must call this function
// as its first step.
// It is assumed that the handler is not run from
// an interrupt context.
//
// @cdev: Qed dev pointer.
//
// Return: int.
//
extern "C" {
    pub fn qed_recovery_prolog(cdev: *mut qed_dev) -> c_int;
}
//
// qed_mcp_ov_update_current_config(): Notify MFW about the change in base
// device properties
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @client: Qed client type.
//
// Return: Int - 0 - Operation was successul.
//
// qed_mcp_ov_update_driver_state(): Notify MFW about the driver state.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @drv_state: Driver state.
//
// Return: Int - 0 - Operation was successul.
//
// qed_mcp_ov_update_mtu(): Send MTU size to MFW.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @mtu: MTU size.
//
// Return: Int - 0 - Operation was successul.
//
// qed_mcp_ov_update_mac(): Send MAC address to MFW.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @mac: MAC address.
//
// Return: Int - 0 - Operation was successul.
//
// qed_mcp_ov_update_wol(): Send WOL mode to MFW.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @wol: WOL mode.
//
// Return: Int - 0 - Operation was successul.
//
// qed_mcp_set_led(): Set LED status.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @mode: LED mode.
//
// Return: Int - 0 - Operation was successul.
//
// qed_mcp_nvm_read(): Read from NVM.
//
// @cdev: Qed dev pointer.
// @addr: NVM offset.
// @p_buf: NVM read buffer.
// @len: Buffer len.
//
// Return: Int - 0 - Operation was successul.
//
extern "C" {
    pub fn qed_mcp_nvm_read(cdev: *mut qed_dev, addr: u32, p_buf: *mut u8, len: u32) -> c_int;
}
//
// qed_mcp_nvm_write(): Write to NVM.
//
// @cdev: Qed dev pointer.
// @addr: NVM offset.
// @cmd: NVM command.
// @p_buf: NVM write buffer.
// @len: Buffer len.
//
// Return: Int - 0 - Operation was successul.
//
// qed_mcp_nvm_resp(): Check latest response.
//
// @cdev: Qed dev pointer.
// @p_buf: NVM write buffer.
//
// Return: Int - 0 - Operation was successul.
//
extern "C" {
    pub fn qed_mcp_nvm_resp(cdev: *mut qed_dev, p_buf: *mut u8) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_nvm_image_att {
    pub start_addr: u32,
    pub length: u32,
}

//
// qed_mcp_get_nvm_image_att(): Allows reading a whole nvram image.
//
// @p_hwfn: HW device data.
// @image_id: Image to get attributes for.
// @p_image_att: Image attributes structure into which to fill data.
//
// Return: Int - 0 - Operation was successul.
//
// qed_mcp_get_nvm_image(): Allows reading a whole nvram image.
//
// @p_hwfn: HW device data.
// @image_id: image requested for reading.
// @p_buffer: allocated buffer into which to fill data.
// @buffer_len: length of the allocated buffer.
//
// Return: 0 if p_buffer now contains the nvram image.
//
// qed_mcp_bist_register_test(): Bist register test.
//
// @p_hwfn: HW device data.
// @p_ptt: PTT required for register access.
//
// Return: Int - 0 - Operation was successul.
//
// qed_mcp_bist_clock_test(): Bist clock test.
//
// @p_hwfn: HW device data.
// @p_ptt: PTT required for register access.
//
// Return: Int - 0 - Operation was successul.
//
// qed_mcp_bist_nvm_get_num_images(): Bist nvm test - get number of images.
//
// @p_hwfn: HW device data.
// @p_ptt: PTT required for register access.
// @num_images: number of images if operation was
// successful. 0 if not.
//
// Return: Int - 0 - Operation was successul.
//
// qed_mcp_bist_nvm_get_image_att(): Bist nvm test - get image attributes
// by index.
//
// @p_hwfn: HW device data.
// @p_ptt: PTT required for register access.
// @p_image_att: Attributes of image.
// @image_index: Index of image to get information for.
//
// Return: Int - 0 - Operation was successul.
//
// qed_mfw_process_tlv_req(): Processes the TLV request from MFW i.e.,
// get the required TLV info
// from the qed client and send it to the MFW.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
//
// Return: 0 upon success.
//
extern "C" {
    pub fn qed_mfw_process_tlv_req(p_hwfn: *mut qed_hwfn, p_ptt: *mut qed_ptt) -> c_int;
}
//
// qed_mcp_send_raw_debug_data(): Send raw debug data to the MFW
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @p_buf: raw debug data buffer.
// @size: Buffer size.
//
// Return : Int.
//
// Using hwfn number (and not pf_num) is required since in CMT mode,
// same pf_num may be used by two different hwfn
// TODO - this shouldn't really be in .h file, but until all fields
// required during hw-init will be placed in their correct place in shmem
// we need it in qed_dev.c [for readin the nvram reflection in shmem].
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_mcp_info {
// List for mailbox commands which were sent and wait for a response
    pub cmd_list: list_head,
// Spinlock used for protecting the access to the mailbox commands list
// and the sending of the commands.
//
    pub cmd_lock: spinlock_t,
// Flag to indicate whether sending a MFW mailbox command is blocked
    pub b_block_cmd: bool,
// Spinlock used for syncing SW link-changes and link-changes
// originating from attention context.
//
    pub link_lock: spinlock_t,
    pub public_base: u32,
    pub drv_mb_addr: u32,
    pub mfw_mb_addr: u32,
    pub port_addr: u32,
    pub drv_mb_seq: u16,
    pub drv_pulse_seq: u16,
    pub link_input: qed_mcp_link_params,
    pub link_output: qed_mcp_link_state,
    pub link_capabilities: qed_mcp_link_capabilities,
    pub func_info: qed_mcp_function_info,
    pub mfw_mb_cur: *mut u8,
    pub mfw_mb_shadow: *mut u8,
    pub mfw_mb_length: u16,
    pub mcp_hist: u32,
// Capabilties negotiated with the MFW
    pub capabilities: u32,
// S/N for debug data mailbox commands
    pub dbg_data_seq: core::sync::atomic::AtomicI32,
// Spinlock used to sync the flag mcp_handling_status with
// the mfw events handler
//
    pub unload_lock: spinlock_t,
    pub mcp_handling_status: c_ulong,
pub const QED_MCP_BYPASS_PROC_BIT: c_int = 0;
pub const QED_MCP_IN_PROCESSING_BIT: c_int = 1;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_mcp_mb_params {
    pub cmd: u32,
    pub param: u32,
    pub p_data_src: *mut c_void,
    pub p_data_dst: *mut c_void,
    pub data_src_size: u8,
    pub data_dst_size: u8,
    pub mcp_resp: u32,
    pub mcp_param: u32,
    pub flags: u32,

    pub \: ({ typeof(params) __params = (params);,
    pub }): (__params && (__params->flags & QED_MB_FLAG_ ## flag));,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_drv_tlv_hdr {
    pub tlv_type: u8,
    pub /: *mut *mut u8 tlv_length; / In dwords - not including this header,
    pub tlv_reserved: u8,
pub const QED_DRV_TLV_FLAGS_CHANGED: c_uint = 0x01;
    pub tlv_flags: u8,
}

//
// qed_mcp_is_ext_speed_supported() - Check if management firmware supports
// extended speeds.
// @p_hwfn: HW device data.
//
// Return: true if supported, false otherwise.
//
// qed_mcp_cmd_init(): Initialize the interface with the MCP.
//
// @p_hwfn: HW device data.
// @p_ptt: PTT required for register access.
//
// Return: Int.
//
// qed_mcp_cmd_port_init(): Initialize the port interface with the MCP
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
//
// Return: Void.
//
// Can only be called after `num_ports_in_engines' is set
//
// qed_mcp_free(): Releases resources allocated during the init process.
//
// @p_hwfn: HW function.
//
// Return: Int.
//
extern "C" {
    pub fn qed_mcp_free(p_hwfn: *mut qed_hwfn) -> c_int;
}
//
// qed_mcp_handle_events(): This function is called from the DPC context.
// After pointing PTT to the mfw mb, check for events sent by
// the MCP to the driver and ack them. In case a critical event
// detected, it will be handled here, otherwise the work will be
// queued to a sleepable work-queue.
//
// @p_hwfn: HW function.
// @p_ptt: PTT required for register access.
//
// Return: Int - 0 - Operation was successul.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_drv_role {
    QED_DRV_ROLE_OS,
    QED_DRV_ROLE_KDUMP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_load_req_params {
// Input params
    pub drv_role: qed_drv_role,
    pub timeout_val: u8,
    pub avoid_eng_reset: bool,
    pub override_force_load: qed_override_force_load,
// Output params
    pub load_code: u32,
}

//
// qed_mcp_load_req(): Sends a LOAD_REQ to the MFW, and in case the
// operation succeeds, returns whether this PF is
// the first on the engine/port or function.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @p_params: Params.
//
// Return: Int - 0 - Operation was successul.
//
// qed_mcp_load_done(): Sends a LOAD_DONE message to the MFW.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
//
// Return: Int - 0 - Operation was successul.
//
extern "C" {
    pub fn qed_mcp_load_done(p_hwfn: *mut qed_hwfn, p_ptt: *mut qed_ptt) -> c_int;
}
//
// qed_mcp_unload_req(): Sends a UNLOAD_REQ message to the MFW.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
//
// Return: Int - 0 - Operation was successul.
//
extern "C" {
    pub fn qed_mcp_unload_req(p_hwfn: *mut qed_hwfn, p_ptt: *mut qed_ptt) -> c_int;
}
//
// qed_mcp_unload_done(): Sends a UNLOAD_DONE message to the MFW
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
//
// Return: Int - 0 - Operation was successul.
//
extern "C" {
    pub fn qed_mcp_unload_done(p_hwfn: *mut qed_hwfn, p_ptt: *mut qed_ptt) -> c_int;
}
//
// qed_mcp_read_mb(): Read the MFW mailbox into Current buffer.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
//
// Return: Void.
//
// qed_mcp_ack_vf_flr(): Ack to mfw that driver finished FLR process for VFs
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @vfs_to_ack: bit mask of all engine VFs for which the PF acks.
//
// Return: Int - 0 - Operation was successul.
//
// qed_mcp_fill_shmem_func_info(): Calls during init to read shmem of
// all function-related info.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
//
// Return: 0 upon success.
//
// qed_mcp_reset(): Reset the MCP using mailbox command.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
//
// Return: 0 upon success.
//
// qed_mcp_nvm_rd_cmd(): Sends an NVM read command request to the MFW to get
// a buffer.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @cmd: (Command) DRV_MSG_CODE_NVM_GET_FILE_DATA or
// DRV_MSG_CODE_NVM_READ_NVRAM commands.
// @param: [0:23] - Offset [24:31] - Size.
// @o_mcp_resp: MCP response.
// @o_mcp_param: MCP response param.
// @o_txn_size: Buffer size output.
// @o_buf: Pointer to the buffer returned by the MFW.
// @b_can_sleep: Can sleep.
//
// Return: 0 upon success.
//
// qed_mcp_phy_sfp_read(): Read from sfp.
//
// @p_hwfn: HW device data.
// @p_ptt: PTT required for register access.
// @port: transceiver port.
// @addr: I2C address.
// @offset: offset in sfp.
// @len: buffer length.
// @p_buf: buffer to read into.
//
// Return: Int - 0 - Operation was successul.
//
// qed_mcp_is_init(): indicates whether the MFW objects [under mcp_info]
// are accessible
//
// @p_hwfn: HW device data.
//
// Return: true if MFW is running and mcp_info is initialized.
//
extern "C" {
    pub fn qed_mcp_is_init(p_hwfn: *mut qed_hwfn) -> bool;
}
//
// qed_mcp_config_vf_msix(): Request MFW to configure MSI-X for a VF.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @vf_id: absolute inside engine.
// @num: number of entries to request.
//
// Return: Int.
//
// qed_mcp_halt(): Halt the MCP.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
//
// Return: 0 upon success.
//
extern "C" {
    pub fn qed_mcp_halt(p_hwfn: *mut qed_hwfn, p_ptt: *mut qed_ptt) -> c_int;
}
//
// qed_mcp_resume: Wake up the MCP.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
//
// Return: 0 upon success.
//
extern "C" {
    pub fn qed_mcp_resume(p_hwfn: *mut qed_hwfn, p_ptt: *mut qed_ptt) -> c_int;
}
extern "C" {
    pub fn qed_configure_pf_min_bandwidth(cdev: *mut qed_dev, min_bw: u8) -> c_int;
}
extern "C" {
    pub fn qed_configure_pf_max_bandwidth(cdev: *mut qed_dev, max_bw: u8) -> c_int;
}
// qed_mcp_mdump_get_retain(): Gets the mdump retained data from the MFW.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @p_mdump_retain: mdump retain.
//
// Return: Int - 0 - Operation was successul.
//
// qed_mcp_set_resc_max_val(): Sets the MFW's max value for the given resource.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @res_id: RES ID.
// @resc_max_val: Resec max val.
// @p_mcp_resp: MCP Resp
//
// Return: Int - 0 - Operation was successul.
//
// qed_mcp_get_resc_info(): Gets the MFW allocation info for the given
// resource.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @res_id: Res ID.
// @p_mcp_resp: MCP resp.
// @p_resc_num: Resc num.
// @p_resc_start: Resc start.
//
// Return: Int - 0 - Operation was successul.
//
// qed_mcp_ov_update_eswitch(): Send eswitch mode to MFW.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @eswitch: eswitch mode.
//
// Return: Int - 0 - Operation was successul.
//

pub const QED_MCP_RESC_LOCK_MAX_VAL: c_int = 31;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_resc_lock {
    QED_RESC_LOCK_DBG_DUMP = QED_MCP_RESC_LOCK_MIN_VAL,
    QED_RESC_LOCK_PTP_PORT0,
    QED_RESC_LOCK_PTP_PORT1,
    QED_RESC_LOCK_PTP_PORT2,
    QED_RESC_LOCK_PTP_PORT3,
    QED_RESC_LOCK_RESC_ALLOC = QED_MCP_RESC_LOCK_MAX_VAL,
    QED_RESC_LOCK_RESC_INVALID
}

//
// qed_mcp_initiate_pf_flr(): Initiates PF FLR.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
//
// Return: Int - 0 - Operation was successul.
//
extern "C" {
    pub fn qed_mcp_initiate_pf_flr(p_hwfn: *mut qed_hwfn, p_ptt: *mut qed_ptt) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_resc_lock_params {
// Resource number [valid values are 0..31]
    pub resource: u8,
// Lock timeout value in seconds [default, none or 1..254]
    pub timeout: u8,
pub const QED_MCP_RESC_LOCK_TO_DEFAULT: c_int = 0;
pub const QED_MCP_RESC_LOCK_TO_NONE: c_int = 255;
// Number of times to retry locking
    pub retry_num: u8,
pub const QED_MCP_RESC_LOCK_RETRY_CNT_DFLT: c_int = 10;
// The interval in usec between retries
    pub retry_interval: u16,
pub const QED_MCP_RESC_LOCK_RETRY_VAL_DFLT: c_int = 10000;
// Use sleep or delay between retries
    pub sleep_b4_retry: bool,
// Will be set as true if the resource is free and granted
    pub b_granted: bool,
// Will be filled with the resource owner.
// [0..15 = PF0-15, 16 = MFW]
//
    pub owner: u8,
}

//
// qed_mcp_resc_lock(): Acquires MFW generic resource lock.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @p_params: Params.
//
// Return: Int - 0 - Operation was successul.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_resc_unlock_params {
// Resource number [valid values are 0..31]
    pub resource: u8,
// Allow to release a resource even if belongs to another PF
    pub b_force: bool,
// Will be set as true if the resource is released
    pub b_released: bool,
}

//
// qed_mcp_resc_unlock(): Releases MFW generic resource lock.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @p_params: Params.
//
// Return: Int - 0 - Operation was successul.
//
// qed_mcp_resc_lock_default_init(): Default initialization for
// lock/unlock resource structs.
//
// @p_lock: lock params struct to be initialized; Can be NULL.
// @p_unlock: unlock params struct to be initialized; Can be NULL.
// @resource: the requested resource.
// @b_is_permanent: disable retries & aging when set.
//
// Return: Void.
//
// qed_mcp_is_smart_an_supported(): Return whether management firmware
// support smart AN
//
// @p_hwfn: HW device data.
//
// Return: bool true if feature is supported.
//
extern "C" {
    pub fn qed_mcp_is_smart_an_supported(p_hwfn: *mut qed_hwfn) -> bool;
}
//
// qed_mcp_get_capabilities(): Learn of supported MFW features;
// To be done during early init.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
//
// Return: Int.
//
extern "C" {
    pub fn qed_mcp_get_capabilities(p_hwfn: *mut qed_hwfn, p_ptt: *mut qed_ptt) -> c_int;
}
//
// qed_mcp_set_capabilities(): Inform MFW of set of features supported
// by driver. Should be done inside the content
// of the LOAD_REQ.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
//
// Return: Int.
//
extern "C" {
    pub fn qed_mcp_set_capabilities(p_hwfn: *mut qed_hwfn, p_ptt: *mut qed_ptt) -> c_int;
}
//
// qed_mcp_read_ufp_config(): Read ufp config from the shared memory.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
//
// Return: Void.
//
extern "C" {
    pub fn qed_mcp_read_ufp_config(p_hwfn: *mut qed_hwfn, p_ptt: *mut qed_ptt);
}
//
// qed_mcp_nvm_info_populate(): Populate the nvm info shadow in the given
// hardware function.
//
// @p_hwfn: HW device data.
//
// Return: Int.
//
extern "C" {
    pub fn qed_mcp_nvm_info_populate(p_hwfn: *mut qed_hwfn) -> c_int;
}
//
// qed_mcp_nvm_info_free(): Delete nvm info shadow in the given
// hardware function.
//
// @p_hwfn: HW device data.
//
// Return: Void.
//
extern "C" {
    pub fn qed_mcp_nvm_info_free(p_hwfn: *mut qed_hwfn);
}
//
// qed_mcp_get_engine_config(): Get the engine affinity configuration.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
//
// Return: Int.
//
extern "C" {
    pub fn qed_mcp_get_engine_config(p_hwfn: *mut qed_hwfn, p_ptt: *mut qed_ptt) -> c_int;
}
//
// qed_mcp_get_ppfid_bitmap(): Get the PPFID bitmap.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
//
// Return: Int.
//
extern "C" {
    pub fn qed_mcp_get_ppfid_bitmap(p_hwfn: *mut qed_hwfn, p_ptt: *mut qed_ptt) -> c_int;
}
//
// qed_mcp_nvm_get_cfg(): Get NVM config attribute value.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @option_id: Option ID.
// @entity_id: Entity ID.
// @flags: Flags.
// @p_buf: Buf.
// @p_len: Len.
//
// Return: Int.
//
// qed_mcp_nvm_set_cfg(): Set NVM config attribute value.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @option_id: Option ID.
// @entity_id: Entity ID.
// @flags: Flags.
// @p_buf: Buf.
// @len: Len.
//
// Return: Int.
//
// qed_mcp_is_esl_supported(): Return whether management firmware support ESL or not.
//
// @p_hwfn: hw function pointer
//
// Return: true if esl is supported, otherwise return false
//
extern "C" {
    pub fn qed_mcp_is_esl_supported(p_hwfn: *mut qed_hwfn) -> bool;
}
//
// qed_mcp_get_esl_status(): Get enhanced system lockdown status
//
// @p_hwfn: hw function pointer
// @p_ptt: ptt resource pointer
// @active: ESL active status data pointer
//
// Return: 0 with esl status info on success, otherwise return error
//
extern "C" {
    pub fn qed_mcp_get_esl_status(p_hwfn: *mut qed_hwfn, p_ptt: *mut qed_ptt, active: *mut bool) -> c_int;
}
