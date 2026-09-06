//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/iwl-trans.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (C) 2005-2014, 2018-2026 Intel Corporation
// Copyright (C) 2013-2015 Intel Mobile Communications GmbH
// Copyright (C) 2016-2017 Intel Deutschland GmbH
//

// Macro flag: #define __iwl_trans_h__

//
// DOC: Transport layer - what is it ?
//
// The transport layer is the layer that deals with the HW directly. It provides
// the PCIe access to the underlying hardwarwe. The transport layer doesn't
// provide any policy, algorithm or anything of this kind, but only mechanisms
// to make the HW do something. It is not completely stateless but close to it.
//
// DOC: Life cycle of the transport layer
//
// The transport layer has a very precise life cycle.
//
// 1) A helper function is called during the module initialization and
// registers the bus driver's ops with the transport's alloc function.
// 2) Bus's probe calls to the transport layer's allocation functions.
// Of course this function is bus specific.
// 3) This allocation functions will spawn the upper layer which will
// register mac80211.
//
// 4) At some point (i.e. mac80211's start call), the op_mode will call
// the following sequence:
// start_hw
// start_fw
//
// 5) Then when finished (or reset):
// stop_device
//
// 6) Eventually, the free function will be called.
//
// default preset 0 (start from bit 16)
pub const IWL_FW_DBG_DOMAIN_POS: c_int = 16;

pub const FH_RSCSR_FRAME_SIZE_MSK: c_uint = 0x00003FFF	/* bits 0-13 */;
pub const FH_RSCSR_FRAME_INVALID: c_uint = 0x55550000;
pub const FH_RSCSR_FRAME_ALIGN: c_uint = 0x40;

pub const FH_RSCSR_RXQ_POS: c_int = 16;
pub const FH_RSCSR_RXQ_MASK: c_uint = 0x3F0000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_rx_packet {
//
// The first 4 bytes of the RX frame header contain both the RX frame
// size and some flags.
// Bit fields:
// 31:    flag flush RB request
// 30:    flag ignore TC (terminal counter) request
// 29:    flag fast IRQ request
// 28-27: Reserved
// 26:    RADA enabled
// 25:    Offload enabled
// 24:    RPF enabled
// 23:    RSS enabled
// 22:    Checksum enabled
// 21-16: RX queue
// 15-14: Reserved
// 13-00: RX frame size
//
    pub len_n_flags: __le32,
    pub hdr: iwl_cmd_header,
    pub data: [u8; ],
    pub __packed: },
    pub FH_RSCSR_FRAME_SIZE_MSK: return le32_to_cpu(pkt->len_n_flags) &,
    pub sizeof(pkt->hdr): return iwl_rx_packet_len(pkt) -,
//
// enum CMD_MODE - how to send the host commands ?
//
// @CMD_ASYNC: Return right away and don't wait for the response
// @CMD_WANT_SKB: Not valid with CMD_ASYNC. The caller needs the buffer of
// the response. The caller needs to call iwl_free_resp when done.
// @CMD_SEND_IN_RFKILL: Send the command even if the NIC is in RF-kill.
// @CMD_BLOCK_TXQS: Block TXQs while the comment is executing.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CMD_MODE {
    CMD_ASYNC		= BIT(0),
    CMD_WANT_SKB		= BIT(1),
    CMD_SEND_IN_RFKILL	= BIT(2),
    CMD_BLOCK_TXQS		= BIT(3),
}

pub const CMD_MODE_BITS: c_int = 5;
pub const DEF_CMD_PAYLOAD_SIZE: c_int = 320;
//
// struct iwl_device_cmd - device command structure
//
// For allocation of the command and tx queues, this establishes the overall
// size of the largest command we send to uCode, except for commands that
// aren't fully copied and use other TFD space.
//
// @hdr: command header
// @payload: payload for the command
// @hdr_wide: wide command header
// @payload_wide: payload for the wide command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_device_cmd {
    pub /: *mut *mut iwl_cmd_header hdr; / uCode API,
    pub payload: [u8; DEF_CMD_PAYLOAD_SIZE],
}

//
// struct iwl_device_tx_cmd - buffer for TX command
// @hdr: the header
// @payload: the payload placeholder
//
// The actual structure is sized dynamically according to need.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_device_tx_cmd {
    pub hdr: iwl_cmd_header,
    pub payload: [u8; ],
    pub __packed: },

// Maximum payload size for a non-NOCOPY host command (excluding the header)

//
// number of transfer buffers (fragments) per transmit frame descriptor;
// this is just the driver's idea, the hardware supports 20
//
pub const IWL_MAX_CMD_TBS_PER_TFD: c_int = 2;
//
// enum iwl_hcmd_dataflag - flag for each one of the chunks of the command
//
// @IWL_HCMD_DFL_NOCOPY: By default, the command is copied to the host command's
// ring. The transport layer doesn't map the command's buffer to DMA, but
// rather copies it to a previously allocated DMA buffer. This flag tells
// the transport layer not to copy the command, but to map the existing
// buffer (that is passed in) instead. This saves the memcpy and allows
// commands that are bigger than the fixed buffer to be submitted.
// Note that a TFD entry after a NOCOPY one cannot be a normal copied one.
// @IWL_HCMD_DFL_DUP: Only valid without NOCOPY, duplicate the memory for this
// chunk internally and free it again after the command completes. This
// can (currently) be used only once per command.
// Note that a TFD entry after a DUP one cannot be a normal copied one.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_hcmd_dataflag {
    IWL_HCMD_DFL_NOCOPY	= BIT(0),
    IWL_HCMD_DFL_DUP	= BIT(1),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_error_event_table_status {
    IWL_ERROR_EVENT_TABLE_LMAC1 = BIT(0),
    IWL_ERROR_EVENT_TABLE_LMAC2 = BIT(1),
    IWL_ERROR_EVENT_TABLE_UMAC = BIT(2),
    IWL_ERROR_EVENT_TABLE_TCM1 = BIT(3),
    IWL_ERROR_EVENT_TABLE_TCM2 = BIT(4),
    IWL_ERROR_EVENT_TABLE_RCM1 = BIT(5),
    IWL_ERROR_EVENT_TABLE_RCM2 = BIT(6),
}

//
// struct iwl_host_cmd - Host command to the uCode
//
// @data: array of chunks that composes the data of the host command
// @resp_pkt: response packet, if %CMD_WANT_SKB was set
// @_rx_page_order: (internally used to free response packet)
// @_rx_page_addr: (internally used to free response packet)
// @flags: can be CMD_
// @len: array of the lengths of the chunks in data
// @dataflags: IWL_HCMD_DFL_
// @id: command id of the host command, for wide commands encoding the
// version and group as well
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_host_cmd {
    pub data: [*const c_void; IWL_MAX_CMD_TBS_PER_TFD],
    pub resp_pkt: *mut iwl_rx_packet,
    pub _rx_page_addr: c_ulong,
    pub _rx_page_order: u32,
    pub flags: u32,
    pub id: u32,
    pub len: [u16; IWL_MAX_CMD_TBS_PER_TFD],
    pub dataflags: [u8; IWL_MAX_CMD_TBS_PER_TFD],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_rx_cmd_buffer {
    pub _page: *mut page,
    pub _offset: c_int,
    pub _page_stolen: bool,
    pub _rx_page_order: u32,
    pub truesize: c_uint,
}

pub const MAX_NO_RECLAIM_CMDS: c_int = 6;

//
// Maximum number of HW queues the transport layer
// currently supports
//
pub const IWL_MAX_HW_QUEUES: c_int = 32;
pub const IWL_MAX_TVQM_QUEUES: c_int = 512;
pub const IWL_MAX_TID_COUNT: c_int = 8;
pub const IWL_MGMT_TID: c_int = 15;
pub const IWL_FRAME_LIMIT: c_int = 64;
pub const IWL_MAX_RX_HW_QUEUES: c_int = 16;
pub const IWL_9000_MAX_RX_HW_QUEUES: c_int = 1;
//
// enum iwl_trans_status: transport status flags
// @STATUS_SYNC_HCMD_ACTIVE: a SYNC command is being processed
// @STATUS_DEVICE_ENABLED: APM is enabled
// @STATUS_TPOWER_PMI: the device might be asleep (need to wake it up)
// @STATUS_INT_ENABLED: interrupts are enabled
// @STATUS_RFKILL_HW: the actual HW state of the RF-kill switch
// @STATUS_RFKILL_OPMODE: RF-kill state reported to opmode
// @STATUS_FW_ERROR: the fw is in error state
// @STATUS_TRANS_DEAD: trans is dead - avoid any read/write operation
// @STATUS_IN_SW_RESET: device is undergoing reset, cleared by opmode
// via iwl_trans_finish_sw_reset()
// @STATUS_RESET_PENDING: reset worker was scheduled, but didn't dump
// the firmware state yet
// @STATUS_TRANS_RESET_IN_PROGRESS: reset is still in progress, don't
// attempt another reset yet
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_trans_status {
    STATUS_SYNC_HCMD_ACTIVE,
    STATUS_DEVICE_ENABLED,
    STATUS_TPOWER_PMI,
    STATUS_INT_ENABLED,
    STATUS_RFKILL_HW,
    STATUS_RFKILL_OPMODE,
    STATUS_FW_ERROR,
    STATUS_TRANS_DEAD,
    STATUS_IN_SW_RESET,
    STATUS_RESET_PENDING,
    STATUS_TRANS_RESET_IN_PROGRESS,
}

extern "C" {
    pub fn get_order(1024: *mut *mut 2) -> return;
}
extern "C" {
    pub fn get_order(1024: *mut *mut 4) -> return;
}
extern "C" {
    pub fn get_order(1024: *mut *mut 8) -> return;
}
extern "C" {
    pub fn get_order(1024: *mut *mut 16) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_hcmd_names {
    pub cmd_id: u8,
    pub cmd_name: *const *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_hcmd_arr {
    pub arr: *const iwl_hcmd_names,
    pub size: c_int,
}

//
// struct iwl_dump_sanitize_ops - dump sanitization operations
// @frob_txf: Scrub the TX FIFO data
// @frob_hcmd: Scrub a host command, the %hcmd pointer is to the header
// but that might be short or long (&struct iwl_cmd_header or
// &struct iwl_cmd_header_wide)
// @frob_mem: Scrub memory data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_dump_sanitize_ops {
    pub buflen): *mut *mut *mut *mut void (frob_txf)(void ctx, void buf, size_t,
    pub buflen): *mut *mut *mut *mut void (frob_hcmd)(void ctx, void hcmd, size_t,
    pub buflen): *mut *mut *mut *mut void (frob_mem)(void ctx, u32 mem_addr, void mem, size_t,
}

//
// struct iwl_trans_config - transport configuration
//
// These values should be set before iwl_trans_op_mode_enter().
//
// @cmd_queue: the index of the command queue.
// Must be set before start_fw.
// @cmd_fifo: the fifo for host commands
// @no_reclaim_cmds: Some devices erroneously don't set the
// SEQ_RX_FRAME bit on some notifications, this is the
// list of such notifications to filter. Max length is
// %MAX_NO_RECLAIM_CMDS.
// @n_no_reclaim_cmds: # of commands in list
// @rx_buf_size: RX buffer size needed for A-MSDUs
// if unset 4k will be the RX buffer size
// @scd_set_active: should the transport configure the SCD for HCMD queue
// @command_groups: array of command groups, each member is an array of the
// commands in the group; for debugging only
// @command_groups_size: number of command groups, to avoid illegal access
// @cb_data_offs: offset inside skb->cb to store transport data at, must have
// space for at least two pointers
// @fw_reset_handshake: firmware supports reset flow handshake
// @queue_alloc_cmd_ver: queue allocation command version, set to 0
// for using the older SCD_QUEUE_CFG, set to the version of
// SCD_QUEUE_CONFIG_CMD otherwise.
// @wide_cmd_header: true when ucode supports wide command header format
// @rx_mpdu_cmd: MPDU RX command ID, must be assigned by opmode before
// starting the firmware, used for tracing
// @rx_mpdu_cmd_hdr_size: used for tracing, amount of data before the
// start of the 802.11 header in the @rx_mpdu_cmd
// @dsbr_urm_fw_dependent: switch to URM based on fw settings
// @dsbr_urm_permanent: switch to URM permanently
// @mbx_addr_0_step: step address data 0
// @mbx_addr_1_step: step address data 1
// @ext_32khz_clock_valid: if true, the external 32 KHz clock can be used
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_trans_config {
    pub cmd_queue: u8,
    pub cmd_fifo: u8,
    pub n_no_reclaim_cmds: u8,
    pub no_reclaim_cmds: [u8; MAX_NO_RECLAIM_CMDS],
    pub rx_buf_size: iwl_amsdu_size,
    pub scd_set_active: bool,
    pub command_groups: *const iwl_hcmd_arr,
    pub command_groups_size: c_int,
    pub cb_data_offs: u8,
    pub fw_reset_handshake: bool,
    pub queue_alloc_cmd_ver: u8,
    pub wide_cmd_header: bool,
    pub rx_mpdu_cmd_hdr_size: u8 rx_mpdu_cmd,,
    pub mbx_addr_0_step: u32,
    pub mbx_addr_1_step: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_trans_dump_data {
    pub len: u32,
    pub data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_trans_txq_scd_cfg {
    pub fifo: u8,
    pub sta_id: u8,
    pub tid: u8,
    pub aggregate: bool,
    pub frame_limit: c_int,
}

//
// struct iwl_trans_rxq_dma_data - RX queue DMA data
// @fr_bd_cb: DMA address of free BD cyclic buffer
// @fr_bd_wid: Initial write index of the free BD cyclic buffer
// @urbd_stts_wrptr: DMA address of urbd_stts_wrptr
// @ur_bd_cb: DMA address of used BD cyclic buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_trans_rxq_dma_data {
    pub fr_bd_cb: u64,
    pub fr_bd_wid: u32,
    pub urbd_stts_wrptr: u64,
    pub ur_bd_cb: u64,
}

// maximal number of DRAM MAP entries supported by FW
pub const IPC_DRAM_MAP_ENTRY_NUM_MAX: c_int = 64;
//
// struct iwl_pnvm_image - contains info about the parsed pnvm image
// @chunks: array of pointers to pnvm payloads and their sizes
// @n_chunks: the number of the pnvm payloads.
// @version: the version of the loaded PNVM image
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_pnvm_image {
    pub data: *const c_void,
    pub len: u32,
    pub chunks: [}; IPC_DRAM_MAP_ENTRY_NUM_MAX],
    pub n_chunks: u32,
    pub version: u32,
}

//
// enum iwl_trans_state - state of the transport layer
//
// @IWL_TRANS_NO_FW: firmware wasn't started yet, or crashed
// @IWL_TRANS_FW_STARTED: FW was started, but not alive yet
// @IWL_TRANS_FW_ALIVE: FW has sent an alive response
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_trans_state {
    IWL_TRANS_NO_FW,
    IWL_TRANS_FW_STARTED,
    IWL_TRANS_FW_ALIVE,
}

//
// DOC: Platform power management
//
// In system-wide power management the entire platform goes into a low
// power state (e.g. idle or suspend to RAM) at the same time and the
// device is configured as a wakeup source for the entire platform.
// This is usually triggered by userspace activity (e.g. the user
// presses the suspend button or a power management daemon decides to
// put the platform in low power mode).  The device's behavior in this
// mode is dictated by the wake-on-WLAN configuration.
//
// The terms used for the device's behavior are as follows:
//
// - D0: the device is fully powered and the host is awake;
// - D3: the device is in low power mode and only reacts to
// specific events (e.g. magic-packet received or scan
// results found);
//
// These terms reflect the power modes in the firmware and are not to
// be confused with the physical device power state.
//
// enum iwl_ini_cfg_state - debug config state
// @IWL_INI_CFG_STATE_NOT_LOADED: no debug cfg was given
// @IWL_INI_CFG_STATE_LOADED: debug cfg was found and loaded
// @IWL_INI_CFG_STATE_CORRUPTED: debug cfg was found and some of the TLVs
// are corrupted. The rest of the debug TLVs will still be used
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_ini_cfg_state {
    IWL_INI_CFG_STATE_NOT_LOADED,
    IWL_INI_CFG_STATE_LOADED,
    IWL_INI_CFG_STATE_CORRUPTED,
}

// Max time to wait for nmi interrupt

//
// struct iwl_dram_data - DRAM data descriptor
// @physical: page phy pointer
// @block: pointer to the allocated block/page
// @size: size of the block/page
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_dram_data {
    pub physical: dma_addr_t,
    pub block: *mut c_void,
    pub size: c_int,
}

//
// struct iwl_dram_regions - DRAM regions container structure
// @drams: array of several DRAM areas that contains the pnvm and power
// reduction table payloads.
// @n_regions: number of DRAM regions that were allocated
// @prph_scratch_mem_desc: points to a structure allocated in dram,
// designed to show FW where all the payloads are.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_dram_regions {
    pub drams: [iwl_dram_data; IPC_DRAM_MAP_ENTRY_NUM_MAX],
    pub prph_scratch_mem_desc: iwl_dram_data,
    pub n_regions: u8,
}

//
// struct iwl_fw_mon - fw monitor per allocation id
// @num_frags: number of fragments
// @frags: an array of DRAM buffer fragments
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_fw_mon {
    pub num_frags: u32,
    pub frags: *mut iwl_dram_data,
}

//
// struct iwl_self_init_dram - dram data used by self init process
// @fw: lmac and umac dram data
// @fw_cnt: total number of items in array
// @paging: paging dram data
// @paging_cnt: total number of items in array
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_self_init_dram {
    pub fw: *mut iwl_dram_data,
    pub fw_cnt: c_int,
    pub paging: *mut iwl_dram_data,
    pub paging_cnt: c_int,
}

//
// struct iwl_imr_data - imr dram data used during debug process
// @imr_enable: imr enable status received from fw
// @imr_size: imr dram size received from fw
// @sram_addr: sram address from debug tlv
// @sram_size: sram size from debug tlv
// @imr2sram_remainbyte: size remained after each dma transfer
// @imr_curr_addr: current dst address used during dma transfer
// @imr_base_addr: imr address received from fw
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_imr_data {
    pub imr_enable: u32,
    pub imr_size: u32,
    pub sram_addr: u32,
    pub sram_size: u32,
    pub imr2sram_remainbyte: u32,
    pub imr_curr_addr: u64,
    pub imr_base_addr: __le64,
}

pub const IWL_TRANS_CURRENT_PC_NAME_MAX_BYTES: c_int = 32;
//
// struct iwl_pc_data - program counter details
// @pc_name: cpu name
// @pc_address: cpu program counter
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_pc_data {
    pub pc_name: [u8; IWL_TRANS_CURRENT_PC_NAME_MAX_BYTES],
    pub pc_address: u32,
}

//
// struct iwl_trans_debug - transport debug related data
//
// @n_dest_reg: num of reg_ops in %dbg_dest_tlv
// @rec_on: true iff there is a fw debug recording currently active
// @dest_tlv: points to the destination TLV for debug
// @lmac_error_event_table: addrs of lmacs error tables
// @umac_error_event_table: addr of umac error table
// @tcm_error_event_table: address(es) of TCM error table(s)
// @rcm_error_event_table: address(es) of RCM error table(s)
// @error_event_table_tlv_status: bitmap that indicates what error table
// pointers was recevied via TLV. uses enum &iwl_error_event_table_status
// @internal_ini_cfg: internal debug cfg state. Uses &enum iwl_ini_cfg_state
// @external_ini_cfg: external debug cfg state. Uses &enum iwl_ini_cfg_state
// @fw_mon_cfg: debug buffer allocation configuration
// @fw_mon_ini: DRAM buffer fragments per allocation id
// @fw_mon: DRAM buffer for firmware monitor
// @hw_error: equals true if hw error interrupt was received from the FW
// @ini_dest: debug monitor destination uses &enum iwl_fw_ini_buffer_location
// @unsupported_region_msk: unsupported regions out of active_regions
// @active_regions: active regions
// @debug_info_tlv_list: list of debug info TLVs
// @time_point: array of debug time points
// @periodic_trig_list: periodic triggers list
// @domains_bitmap: bitmap of active domains other than &IWL_FW_INI_DOMAIN_ALWAYS_ON
// @ucode_preset: preset based on ucode
// @restart_required: indicates debug restart is required
// @last_tp_resetfw: last handling of reset during debug timepoint
// @imr_data: IMR debug data allocation
// @num_pc: number of program counter for cpu
// @pc_data: details of the program counter
// @yoyo_bin_loaded: tells if a yoyo debug file has been loaded
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_trans_debug {
    pub n_dest_reg: u8,
    pub rec_on: bool,
    pub dest_tlv: *const iwl_fw_dbg_dest_tlv_v1,
    pub lmac_error_event_table: [u32; 2],
    pub umac_error_event_table: u32,
    pub tcm_error_event_table: [u32; 2],
    pub rcm_error_event_table: [u32; 2],
    pub error_event_table_tlv_status: c_uint,
    pub internal_ini_cfg: iwl_ini_cfg_state,
    pub external_ini_cfg: iwl_ini_cfg_state,
    pub fw_mon_cfg: [iwl_fw_ini_allocation_tlv; IWL_FW_INI_ALLOCATION_NUM],
    pub fw_mon_ini: [iwl_fw_mon; IWL_FW_INI_ALLOCATION_NUM],
    pub fw_mon: iwl_dram_data,
    pub hw_error: bool,
    pub ini_dest: iwl_fw_ini_buffer_location,
    pub unsupported_region_msk: u64,
    pub active_regions: [*mut iwl_ucode_tlv; IWL_FW_INI_MAX_REGION_ID],
    pub debug_info_tlv_list: list_head,
    pub time_point: [iwl_dbg_tlv_time_point_data; IWL_FW_INI_TIME_POINT_NUM],
    pub periodic_trig_list: list_head,
    pub domains_bitmap: u32,
    pub ucode_preset: u32,
    pub restart_required: bool,
    pub last_tp_resetfw: u32,
    pub imr_data: iwl_imr_data,
    pub num_pc: u32,
    pub pc_data: *mut iwl_pc_data,
    pub yoyo_bin_loaded: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_dma_ptr {
    pub dma: dma_addr_t,
    pub addr: *mut c_void,
    pub size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_cmd_meta {
// only for SYNC commands, iff the reply skb is wanted
    pub source: *mut iwl_host_cmd,
    pub CMD_MODE_BITS: u32 flags:,
// sg_offset is valid if it is non-zero
    pub PAGE_SHIFT: u32 sg_offset:,
    pub tbs: u32,
}

//
// struct iwl_trans_info - transport info for outside use
// @name: the device name
// @max_skb_frags: maximum number of fragments an SKB can have when transmitted.
// 0 indicates that frag SKBs (NETIF_F_SG) aren't supported.
// @hw_rev: the revision data of the HW
// @hw_rev_step: The mac step of the HW
// @hw_rf_id: the device RF ID
// @hw_cnv_id: the device CNV ID
// @hw_crf_id: the device CRF ID
// @hw_id: the ID of the device / sub-device
// Bits 0:15 represent the sub-device ID
// Bits 16:31 represent the device ID.
// @pcie_link_speed: current PCIe link speed (%PCI_EXP_LNKSTA_CLS_*),
// only valid for discrete (not integrated) NICs
// @num_rxqs: number of RX queues allocated by the transport
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_trans_info {
    pub name: *const c_char,
    pub max_skb_frags: u32,
    pub hw_rev: u32,
    pub hw_rev_step: u32,
    pub hw_rf_id: u32,
    pub hw_crf_id: u32,
    pub hw_cnv_id: u32,
    pub hw_id: u32,
    pub pcie_link_speed: u8,
    pub num_rxqs: u8,
}

//
// struct iwl_trans - transport common data
//
// @csme_own: true if we couldn't get ownership on the device
// @op_mode: pointer to the op_mode
// @mac_cfg: the trans-specific configuration part
// @cfg: pointer to the configuration
// @drv: pointer to iwl_drv
// @conf: configuration set by the opmode before enter
// @state: current device state
// @status: a bit-mask of transport status flags
// @dev: pointer to struct device * that represents the device
// @info: device information for use by other layers
// @pnvm_loaded: indicates PNVM was loaded
// @suppress_cmd_error_once: suppress "FW error in SYNC CMD" once,
// e.g. for testing
// @fail_to_parse_pnvm_image: set to true if pnvm parsing failed
// @reduce_power_loaded: indicates reduced power section was loaded
// @failed_to_load_reduce_power_image: set to true if pnvm loading failed
// @dbgfs_dir: iwlwifi debugfs base dir for this device
// @sync_cmd_lockdep_map: lockdep map for checking sync commands
// @dbg: additional debug data, see &struct iwl_trans_debug
// @init_dram: FW initialization DMA data
// @reduced_cap_sku: reduced capability supported SKU
// @step_urm: STEP is in URM, no support for MCS>9 in 320 MHz
// @restart: restart worker data
// @restart.wk: restart worker
// @restart.mode: reset/restart error mode information
// @restart.during_reset: error occurred during previous software reset
// @trans_specific: data for the specific transport this is allocated for/with
// @request_top_reset: TOP reset was requested, used by the reset
// worker that should be scheduled (with appropriate reason)
// @do_top_reset: indication to the (PCIe) transport/context-info
// to do the TOP reset
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_trans {
    pub csme_own: bool,
    pub op_mode: *mut iwl_op_mode,
    pub mac_cfg: *const iwl_mac_cfg,
    pub cfg: *const iwl_rf_cfg,
    pub drv: *mut iwl_drv,
    pub conf: iwl_trans_config,
    pub state: iwl_trans_state,
    pub status: c_ulong,
    pub dev: *mut device,
    pub info: iwl_trans_info,
    pub reduced_cap_sku: bool,
    pub step_urm: bool,
    pub suppress_cmd_error_once: bool,
    pub pnvm_loaded:1: u8,
    pub fail_to_parse_pnvm_image:1: u8,
    pub reduce_power_loaded:1: u8,
    pub failed_to_load_reduce_power_image:1: u8,
    pub dbgfs_dir: *mut dentry,

    pub sync_cmd_lockdep_map: lockdep_map,

    pub dbg: iwl_trans_debug,
    pub init_dram: iwl_self_init_dram,
    pub wk: delayed_work,
    pub mode: iwl_fw_error_dump_mode,
    pub during_reset: bool,
    pub restart: },
// pointer to trans specific struct
// Ensure that this pointer will always be aligned to sizeof pointer
    pub )): *mut char trans_specific[] __aligned(sizeof(void,
}

extern "C" {
    pub fn iwl_trans_start_hw(trans: *mut iwl_trans) -> c_int;
}
extern "C" {
    pub fn iwl_trans_op_mode_leave(trans: *mut iwl_trans);
}
extern "C" {
    pub fn iwl_trans_fw_alive(trans: *mut iwl_trans);
}
extern "C" {
    pub fn iwl_trans_stop_device(trans: *mut iwl_trans);
}
extern "C" {
    pub fn iwl_trans_d3_suspend(trans: *mut iwl_trans, reset: bool) -> c_int;
}
extern "C" {
    pub fn iwl_trans_d3_resume(trans: *mut iwl_trans, reset: bool) -> c_int;
}
extern "C" {
    pub fn iwl_trans_send_cmd(trans: *mut iwl_trans, cmd: *mut iwl_host_cmd) -> c_int;
}
extern "C" {
    pub fn iwl_trans_set_q_ptrs(trans: *mut iwl_trans, queue: c_int, ptr: c_int);
}
extern "C" {
    pub fn iwl_trans_txq_free(trans: *mut iwl_trans, queue: c_int);
}
extern "C" {
    pub fn iwl_trans_wait_tx_queues_empty(trans: *mut iwl_trans, txqs: u32) -> c_int;
}
extern "C" {
    pub fn iwl_trans_wait_txq_empty(trans: *mut iwl_trans, queue: c_int) -> c_int;
}
extern "C" {
    pub fn iwl_trans_write8(trans: *mut iwl_trans, ofs: u32, val: u8);
}
extern "C" {
    pub fn iwl_trans_write32(trans: *mut iwl_trans, ofs: u32, val: u32);
}
extern "C" {
    pub fn iwl_trans_read32(trans: *mut iwl_trans, ofs: u32) -> u32;
}
extern "C" {
    pub fn iwl_trans_read_prph(trans: *mut iwl_trans, ofs: u32) -> u32;
}
extern "C" {
    pub fn iwl_trans_write_prph(trans: *mut iwl_trans, ofs: u32, val: u32);
}
//
// Note the special calling convention - it's allowed to drop the
// internal transport lock and re-enable BHs temporarily, but will
// not release NIC access.
//

extern "C" {
    pub fn iwl_trans_debugfs_cleanup(trans: *mut iwl_trans);
}

extern "C" {
    pub fn iwl_trans_write_mem(_arg: trans, _arg: addr, _arg: &val, _arg: 1) -> return;
}
extern "C" {
    pub fn iwl_trans_set_pmi(trans: *mut iwl_trans, state: bool);
}
extern "C" {
    pub fn iwl_trans_sw_reset(trans: *mut iwl_trans) -> c_int;
}
extern "C" {
    pub fn iwl_trans_grab_nic_access(trans: *mut iwl_trans) -> bool;
}
//
// iwl_trans_resched_with_nic_access - reschedule while holding NIC access
// @trans: the transport pointer
//
// This can be called while holding NIC access, during periods where
// the lock itself isn't interesting, the NIC should remain active,
// but a lot of processing is happening and the CPU may need to be
// released. In practice, this is only during FW dump.
//
extern "C" {
    pub fn iwl_trans_resched_with_nic_access(trans: *mut iwl_trans);
}
// clear this on device init, not cleared on any unbind/reprobe
//
// keep track of whether or not this happened while resetting,
// by the timer the worker runs it might have finished
//
// prevent double restarts due to the same erroneous FW
extern "C" {
    pub fn iwl_trans_sync_nmi(trans: *mut iwl_trans);
}
extern "C" {
    pub fn iwl_trans_interrupts(trans: *mut iwl_trans, enable: bool);
}
extern "C" {
    pub fn iwl_trans_activate_nic(trans: *mut iwl_trans) -> c_int;
}
//
// transport helper functions
//
extern "C" {
    pub fn iwl_trans_free(trans: *mut iwl_trans);
}
extern "C" {
    pub fn iwl_trans_free_restart_list();
}
//
// Since AX210 family (So/Ty) the device cannot put mutliple
// frames into the same buffer, so double the value for them.
//
extern "C" {
    pub fn test_bit(_arg: STATUS_DEVICE_ENABLED, _arg: &trans->status) -> return;
}
extern "C" {
    pub fn test_bit(_arg: STATUS_TRANS_DEAD, _arg: &trans->status) -> return;
}
extern "C" {
    pub fn test_bit(_arg: STATUS_FW_ERROR, _arg: &trans->status) -> return;
}
//
// This function notifies the transport layer of firmware error, the recovery
// will be handled by the op mode
//
// PCIe handling
//
extern "C" {
    pub fn iwl_pci_register_driver() -> int __must_check;
}
extern "C" {
    pub fn iwl_pci_unregister_driver();
}
// Note: order matters
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_reset_mode {
// upper level modes:
    IWL_RESET_MODE_SW_RESET,
    IWL_RESET_MODE_REPROBE,
// TOP reset doesn't require PCIe remove
    IWL_RESET_MODE_TOP_RESET,
// PCIE level modes:
    IWL_RESET_MODE_REMOVE_ONLY,
    IWL_RESET_MODE_RESCAN,
    IWL_RESET_MODE_FUNC_RESET,
    IWL_RESET_MODE_PROD_RESET,

// keep last - special backoff value
    IWL_RESET_MODE_BACKOFF,
}

extern "C" {
    pub fn iwl_trans_pcie_reset(trans: *mut iwl_trans, mode: iwl_reset_mode);
}
extern "C" {
    pub fn iwl_trans_pcie_fw_reset_handshake(trans: *mut iwl_trans);
}
// Internal helper
// write = *info;
extern "C" {
    pub fn u32_get_bits(_arg: trans->info.hw_id, _arg: GENMASK(31, _arg: 16)) -> return;
}
extern "C" {
    pub fn iwl_trans_is_pm_supported(trans: *mut iwl_trans) -> bool;
}
extern "C" {
    pub fn iwl_trans_is_ltr_enabled(trans: *mut iwl_trans) -> bool;
}
// not supported before Sc family
// for Sc family only supported for Sc2/Sc2f
// so far these numbers are increasing - not before Pe
