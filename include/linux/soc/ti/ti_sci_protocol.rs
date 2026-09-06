//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soc/ti/ti_sci_protocol.h
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
// Texas Instruments System Control Interface Protocol
//
// Copyright (C) 2015-2016 Texas Instruments Incorporated - https://www.ti.com
// Nishanth Menon
//
// struct ti_sci_version_info - version information structure
// @abi_major:	Major ABI version. Change here implies risk of backward
// compatibility break.
// @abi_minor:	Minor ABI version. Change here implies new feature addition,
// or compatible change in ABI.
// @firmware_revision:	Firmware revision (not usually used).
// @firmware_description: Firmware description (not usually used).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_sci_version_info {
    pub abi_major: u8,
    pub abi_minor: u8,
    pub firmware_revision: u16,
    pub firmware_description: [c_char; 32],
}

//
// struct ti_sci_core_ops - SoC Core Operations
// @reboot_device: Reboot the SoC
// Returns 0 for successful request(ideally should never return),
// else returns corresponding error value.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_sci_core_ops {
    pub handle): *const *const int (reboot_device)(struct ti_sci_handle,
}

//
// struct ti_sci_dev_ops - Device control operations
// @get_device: Command to request for device managed by TISCI
// Returns 0 for successful exclusive request, else returns
// corresponding error message.
// @idle_device: Command to idle a device managed by TISCI
// Returns 0 for successful exclusive request, else returns
// corresponding error message.
// @put_device:	Command to release a device managed by TISCI
// Returns 0 for successful release, else returns corresponding
// error message.
// @is_valid:	Check if the device ID is a valid ID.
// Returns 0 if the ID is valid, else returns corresponding error.
// @get_context_loss_count: Command to retrieve context loss counter - this
// increments every time the device looses context. Overflow
// is possible.
// - count: pointer to u32 which will retrieve counter
// Returns 0 for successful information request and count has
// proper data, else returns corresponding error message.
// @is_idle:	Reports back about device idle state
// - req_state: Returns requested idle state
// Returns 0 for successful information request and req_state and
// current_state has proper data, else returns corresponding error
// message.
// @is_stop:	Reports back about device stop state
// - req_state: Returns requested stop state
// - current_state: Returns current stop state
// Returns 0 for successful information request and req_state and
// current_state has proper data, else returns corresponding error
// message.
// @is_on:	Reports back about device ON(or active) state
// - req_state: Returns requested ON state
// - current_state: Returns current ON state
// Returns 0 for successful information request and req_state and
// current_state has proper data, else returns corresponding error
// message.
// @is_transitioning: Reports back if the device is in the middle of transition
// of state.
// -current_state: Returns 'true' if currently transitioning.
// @set_device_resets: Command to configure resets for device managed by TISCI.
// -reset_state: Device specific reset bit field
// Returns 0 for successful request, else returns
// corresponding error message.
// @get_device_resets: Command to read state of resets for device managed
// by TISCI.
// -reset_state: pointer to u32 which will retrieve resets
// Returns 0 for successful request, else returns
// corresponding error message.
//
// NOTE: for all these functions, the following parameters are generic in
// nature:
// -handle:	Pointer to TISCI handle as retrieved by *ti_sci_get_handle
// -id:		Device Identifier
//
// Request for the device - NOTE: the client MUST maintain integrity of
// usage count by balancing get_device with put_device. No refcounting is
// managed by driver for that purpose.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_sci_dev_ops {
    pub id): *const *const *const int (get_device)(struct ti_sci_handle handle, u32,
    pub id): *const *const *const int (get_device_exclusive)(struct ti_sci_handle handle, u32,
    pub id): *const *const *const int (idle_device)(struct ti_sci_handle handle, u32,
    pub id): u32,
    pub id): *const *const *const int (put_device)(struct ti_sci_handle handle, u32,
    pub id): *const *const *const int (is_valid)(struct ti_sci_handle handle, u32,
    pub count): *mut u32 id, u32,
    pub requested_state): *mut bool,
    pub current_state): *mut *mut bool req_state, bool,
    pub current_state): *mut *mut bool req_state, bool,
    pub current_state): *mut bool,
    pub reset_state): u32,
    pub reset_state): *mut u32,
}

//
// struct ti_sci_clk_ops - Clock control operations
// @get_clock:	Request for activation of clock and manage by processor
// - needs_ssc: 'true' if Spread Spectrum clock is desired.
// - can_change_freq: 'true' if frequency change is desired.
// - enable_input_term: 'true' if input termination is desired.
// @idle_clock:	Request for Idling a clock managed by processor
// @put_clock:	Release the clock to be auto managed by TISCI
// @is_auto:	Is the clock being auto managed
// - req_state: state indicating if the clock is auto managed
// @is_on:	Is the clock ON
// - req_state: if the clock is requested to be forced ON
// - current_state: if the clock is currently ON
// @is_off:	Is the clock OFF
// - req_state: if the clock is requested to be forced OFF
// - current_state: if the clock is currently Gated
// @set_parent:	Set the clock source of a specific device clock
// - parent_id: Parent clock identifier to set.
// @get_parent:	Get the current clock source of a specific device clock
// - parent_id: Parent clock identifier which is the parent.
// @get_num_parents: Get the number of parents of the current clock source
// - num_parents: returns the number of parent clocks.
// @get_best_match_freq: Find a best matching frequency for a frequency
// range.
// - match_freq: Best matching frequency in Hz.
// @set_freq:	Set the Clock frequency
// @get_freq:	Get the Clock frequency
// - current_freq: Frequency in Hz that the clock is at.
//
// NOTE: for all these functions, the following parameters are generic in
// nature:
// -handle:	Pointer to TISCI handle as retrieved by *ti_sci_get_handle
// -did:	Device identifier this request is for
// -cid:	Clock identifier for the device for this request.
// Each device has it's own set of clock inputs. This indexes
// which clock input to modify.
// -min_freq:	The minimum allowable frequency in Hz. This is the minimum
// allowable programmed frequency and does not account for clock
// tolerances and jitter.
// -target_freq: The target clock frequency in Hz. A frequency will be
// processed as close to this target frequency as possible.
// -max_freq:	The maximum allowable frequency in Hz. This is the maximum
// allowable programmed frequency and does not account for clock
// tolerances and jitter.
//
// Request for the clock - NOTE: the client MUST maintain integrity of
// usage count by balancing get_clock with put_clock. No refcounting is
// managed by driver for that purpose.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_sci_clk_ops {
    pub enable_input_term): bool,
    pub cid): *const *const *const int (idle_clock)(struct ti_sci_handle handle, u32 did, u32,
    pub cid): *const *const *const int (put_clock)(struct ti_sci_handle handle, u32 did, u32,
    pub req_state): *mut bool,
    pub current_state): *mut *mut bool req_state, bool,
    pub current_state): *mut *mut bool req_state, bool,
    pub parent_id): u32,
    pub parent_id): *mut u32,
    pub num_parents): *mut u32 cid, u32,
    pub match_freq): *mut u64 max_freq, u64,
    pub max_freq): u64 min_freq, u64 target_freq, u64,
    pub current_freq): *mut u64,
}

// TISCI LPM IO isolation control values
pub const TISCI_MSG_VALUE_IO_ENABLE: c_int = 1;
pub const TISCI_MSG_VALUE_IO_DISABLE: c_int = 0;
// TISCI LPM constraint state values
pub const TISCI_MSG_CONSTRAINT_SET: c_int = 1;
pub const TISCI_MSG_CONSTRAINT_CLR: c_int = 0;
//
// struct ti_sci_pm_ops - Low Power Mode (LPM) control operations
// @lpm_wake_reason: Get the wake up source that woke the SoC from LPM
// - source: The wake up source that woke soc from LPM.
// - timestamp: Timestamp at which soc woke.
// @set_device_constraint: Set LPM constraint on behalf of a device
// - id: Device Identifier
// - state: The desired state of device constraint: set or clear.
// @set_latency_constraint: Set LPM resume latency constraint
// - latency: maximum acceptable latency to wake up from low power mode
// - state: The desired state of latency constraint: set or clear.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_sci_pm_ops {
    pub mode): *mut *mut *mut *mut u32 source, u64 timestamp, u8 pin, u8,
    pub state): u32 id, u8,
    pub state): u16 latency, u8,
}

//
// struct ti_sci_resource_desc - Description of TI SCI resource instance range.
// @start:	Start index of the first resource range.
// @num:	Number of resources in the first range.
// @start_sec:	Start index of the second resource range.
// @num_sec:	Number of resources in the second range.
// @res_map:	Bitmap to manage the allocation of these resources.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_sci_resource_desc {
    pub start: u16,
    pub num: u16,
    pub start_sec: u16,
    pub num_sec: u16,
    pub res_map: *mut c_ulong,
}

//
// struct ti_sci_rm_core_ops - Resource management core operations
// @get_range:		Get a range of resources belonging to ti sci host.
// @get_rage_from_shost:	Get a range of resources belonging to
// specified host id.
// - s_host: Host processing entity to which the
// resources are allocated
//
// NOTE: for these functions, all the parameters are consolidated and defined
// as below:
// - handle:	Pointer to TISCI handle as retrieved by *ti_sci_get_handle
// - dev_id:	TISCI device ID.
// - subtype:	Resource assignment subtype that is being requested
// from the given device.
// - desc:	Pointer to ti_sci_resource_desc to be updated with the resource
// range start index and number of resources
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_sci_rm_core_ops {
    pub desc): *mut u8 subtype, struct ti_sci_resource_desc,
    pub desc): *mut ti_sci_resource_desc,
}

pub const TI_SCI_RESASG_SUBTYPE_IR_OUTPUT: c_int = 0;
pub const TI_SCI_RESASG_SUBTYPE_IA_VINT: c_uint = 0xa;
pub const TI_SCI_RESASG_SUBTYPE_GLOBAL_EVENT_SEVT: c_uint = 0xd;
//
// struct ti_sci_rm_irq_ops: IRQ management operations
// @set_irq:		Set an IRQ route between the requested source
// and destination
// @set_event_map:	Set an Event based peripheral irq to Interrupt
// Aggregator.
// @free_irq:		Free an IRQ route between the requested source
// and destination.
// @free_event_map:	Free an event based peripheral irq to Interrupt
// Aggregator.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_sci_rm_irq_ops {
    pub dst_host_irq): u16 src_index, u16 dst_id, u16,
    pub vint_status_bit): u16 global_event, u8,
    pub dst_host_irq): u16 src_index, u16 dst_id, u16,
    pub vint_status_bit): u16 global_event, u8,
}

// RA config.addr_lo parameter is valid for RM ring configure TI_SCI message

// RA config.addr_hi parameter is valid for RM ring configure TI_SCI message

// RA config.count parameter is valid for RM ring configure TI_SCI message

// RA config.mode parameter is valid for RM ring configure TI_SCI message

// RA config.size parameter is valid for RM ring configure TI_SCI message

// RA config.order_id parameter is valid for RM ring configure TISCI message

// RA config.virtid parameter is valid for RM ring configure TISCI message

// RA config.asel parameter is valid for RM ring configure TISCI message

//
// struct ti_sci_msg_rm_ring_cfg - Ring configuration
//
// Parameters for Navigator Subsystem ring configuration
// See @ti_sci_msg_rm_ring_cfg_req
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_sci_msg_rm_ring_cfg {
    pub valid_params: u32,
    pub nav_id: u16,
    pub index: u16,
    pub addr_lo: u32,
    pub addr_hi: u32,
    pub count: u32,
    pub mode: u8,
    pub size: u8,
    pub order_id: u8,
    pub virtid: u16,
    pub asel: u8,
}

//
// struct ti_sci_rm_ringacc_ops - Ring Accelerator Management operations
// @set_cfg: configure the SoC Navigator Subsystem Ring Accelerator ring
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_sci_rm_ringacc_ops {
    pub params): *const ti_sci_msg_rm_ring_cfg,
}

//
// struct ti_sci_rm_psil_ops - PSI-L thread operations
// @pair: pair PSI-L source thread to a destination thread.
// If the src_thread is mapped to UDMA tchan, the corresponding channel's
// TCHAN_THRD_ID register is updated.
// If the dst_thread is mapped to UDMA rchan, the corresponding channel's
// RCHAN_THRD_ID register is updated.
// @unpair: unpair PSI-L source thread from a destination thread.
// If the src_thread is mapped to UDMA tchan, the corresponding channel's
// TCHAN_THRD_ID register is cleared.
// If the dst_thread is mapped to UDMA rchan, the corresponding channel's
// RCHAN_THRD_ID register is cleared.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_sci_rm_psil_ops {
    pub dst_thread): u32 src_thread, u32,
    pub dst_thread): u32 src_thread, u32,
}

// UDMAP channel types
pub const TI_SCI_RM_UDMAP_CHAN_TYPE_PKT_PBRR: c_int = 2;

pub const TI_SCI_RM_UDMAP_CHAN_TYPE_3RDP_PBRR: c_int = 10;
pub const TI_SCI_RM_UDMAP_CHAN_TYPE_3RDP_PBVR: c_int = 11;
pub const TI_SCI_RM_UDMAP_CHAN_TYPE_3RDP_BCOPY_PBRR: c_int = 12;
pub const TI_SCI_RM_UDMAP_CHAN_TYPE_3RDP_BCOPY_PBVR: c_int = 13;
pub const TI_SCI_RM_UDMAP_RX_FLOW_DESC_HOST: c_int = 0;
pub const TI_SCI_RM_UDMAP_RX_FLOW_DESC_MONO: c_int = 2;
pub const TI_SCI_RM_UDMAP_CHAN_BURST_SIZE_64_BYTES: c_int = 1;
pub const TI_SCI_RM_UDMAP_CHAN_BURST_SIZE_128_BYTES: c_int = 2;
pub const TI_SCI_RM_UDMAP_CHAN_BURST_SIZE_256_BYTES: c_int = 3;
pub const TI_SCI_RM_BCDMA_EXTENDED_CH_TYPE_TCHAN: c_int = 0;
pub const TI_SCI_RM_BCDMA_EXTENDED_CH_TYPE_BCHAN: c_int = 1;
// UDMAP TX/RX channel valid_params common declarations

//
// Configures a Navigator Subsystem UDMAP transmit channel
//
// Configures a Navigator Subsystem UDMAP transmit channel registers.
// See @ti_sci_msg_rm_udmap_tx_ch_cfg_req
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_sci_msg_rm_udmap_tx_ch_cfg {
    pub valid_params: u32,

    pub nav_id: u16,
    pub index: u16,
    pub tx_pause_on_err: u8,
    pub tx_filt_einfo: u8,
    pub tx_filt_pswords: u8,
    pub tx_atype: u8,
    pub tx_chan_type: u8,
    pub tx_supr_tdpkt: u8,
    pub tx_fetch_size: u16,
    pub tx_credit_count: u8,
    pub txcq_qnum: u16,
    pub tx_priority: u8,
    pub tx_qos: u8,
    pub tx_orderid: u8,
    pub fdepth: u16,
    pub tx_sched_priority: u8,
    pub tx_burst_size: u8,
    pub tx_tdtype: u8,
    pub extended_ch_type: u8,
}

//
// Configures a Navigator Subsystem UDMAP receive channel
//
// Configures a Navigator Subsystem UDMAP receive channel registers.
// See @ti_sci_msg_rm_udmap_rx_ch_cfg_req
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_sci_msg_rm_udmap_rx_ch_cfg {
    pub valid_params: u32,

    pub nav_id: u16,
    pub index: u16,
    pub rx_fetch_size: u16,
    pub rxcq_qnum: u16,
    pub rx_priority: u8,
    pub rx_qos: u8,
    pub rx_orderid: u8,
    pub rx_sched_priority: u8,
    pub flowid_start: u16,
    pub flowid_cnt: u16,
    pub rx_pause_on_err: u8,
    pub rx_atype: u8,
    pub rx_chan_type: u8,
    pub rx_ignore_short: u8,
    pub rx_ignore_long: u8,
    pub rx_burst_size: u8,
}

//
// Configures a Navigator Subsystem UDMAP receive flow
//
// Configures a Navigator Subsystem UDMAP receive flow's registers.
// See @tis_ci_msg_rm_udmap_flow_cfg_req
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_sci_msg_rm_udmap_flow_cfg {
    pub valid_params: u32,

    pub nav_id: u16,
    pub flow_index: u16,
    pub rx_einfo_present: u8,
    pub rx_psinfo_present: u8,
    pub rx_error_handling: u8,
    pub rx_desc_type: u8,
    pub rx_sop_offset: u16,
    pub rx_dest_qnum: u16,
    pub rx_src_tag_hi: u8,
    pub rx_src_tag_lo: u8,
    pub rx_dest_tag_hi: u8,
    pub rx_dest_tag_lo: u8,
    pub rx_src_tag_hi_sel: u8,
    pub rx_src_tag_lo_sel: u8,
    pub rx_dest_tag_hi_sel: u8,
    pub rx_dest_tag_lo_sel: u8,
    pub rx_fdq0_sz0_qnum: u16,
    pub rx_fdq1_qnum: u16,
    pub rx_fdq2_qnum: u16,
    pub rx_fdq3_qnum: u16,
    pub rx_ps_location: u8,
}

//
// struct ti_sci_rm_udmap_ops - UDMA Management operations
// @tx_ch_cfg: configure SoC Navigator Subsystem UDMA transmit channel.
// @rx_ch_cfg: configure SoC Navigator Subsystem UDMA receive channel.
// @rx_flow_cfg1: configure SoC Navigator Subsystem UDMA receive flow.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_sci_rm_udmap_ops {
    pub params): *const ti_sci_msg_rm_udmap_tx_ch_cfg,
    pub params): *const ti_sci_msg_rm_udmap_rx_ch_cfg,
    pub params): *const ti_sci_msg_rm_udmap_flow_cfg,
}

//
// struct ti_sci_proc_ops - Processor Control operations
// @request:	Request to control a physical processor. The requesting host
// should be in the processor access list
// @release:	Relinquish a physical processor control
// @handover:	Handover a physical processor control to another host
// in the permitted list
// @set_config:	Set base configuration of a processor
// @set_control: Setup limited control flags in specific cases
// @get_status: Get the state of physical processor
//
// NOTE: The following paramteres are generic in nature for all these ops,
// -handle:	Pointer to TI SCI handle as retrieved by *ti_sci_get_handle
// -pid:	Processor ID
// -hid:	Host ID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_sci_proc_ops {
    pub pid): *const *const *const int (request)(struct ti_sci_handle handle, u8,
    pub pid): *const *const *const int (release)(struct ti_sci_handle handle, u8,
    pub hid): *const *const *const int (handover)(struct ti_sci_handle handle, u8 pid, u8,
    pub cfg_clr): u64 boot_vector, u32 cfg_set, u32,
    pub ctrl_clr): u32 ctrl_set, u32,
    pub status_flags): *mut u32,
}

//
// struct ti_sci_ops - Function support for TI SCI
// @dev_ops:	Device specific operations
// @clk_ops:	Clock specific operations
// @rm_core_ops:	Resource management core operations.
// @rm_irq_ops:		IRQ management specific operations
// @proc_ops:	Processor Control specific operations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_sci_ops {
    pub core_ops: ti_sci_core_ops,
    pub dev_ops: ti_sci_dev_ops,
    pub clk_ops: ti_sci_clk_ops,
    pub pm_ops: ti_sci_pm_ops,
    pub rm_core_ops: ti_sci_rm_core_ops,
    pub rm_irq_ops: ti_sci_rm_irq_ops,
    pub rm_ring_ops: ti_sci_rm_ringacc_ops,
    pub rm_psil_ops: ti_sci_rm_psil_ops,
    pub rm_udmap_ops: ti_sci_rm_udmap_ops,
    pub proc_ops: ti_sci_proc_ops,
}

//
// struct ti_sci_handle - Handle returned to TI SCI clients for usage.
// @version:	structure containing version information
// @ops:	operations that are made available to TI SCI clients
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_sci_handle {
    pub version: ti_sci_version_info,
    pub ops: ti_sci_ops,
}

pub const TI_SCI_RESOURCE_NULL: c_uint = 0xffff;
//
// struct ti_sci_resource - Structure representing a resource assigned
// to a device.
// @sets:	Number of sets available from this resource type
// @lock:	Lock to guard the res map in each set.
// @desc:	Array of resource descriptors.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_sci_resource {
    pub sets: u16,
    pub lock: raw_spinlock_t,
    pub desc: *mut ti_sci_resource_desc,
}

extern "C" {
    pub fn ti_sci_put_handle(handle: *const ti_sci_handle) -> c_int;
}
extern "C" {
    pub fn ti_sci_get_free_resource(res: *mut ti_sci_resource) -> u16;
}
extern "C" {
    pub fn ti_sci_release_resource(res: *mut ti_sci_resource, id: u16);
}
extern "C" {
    pub fn ti_sci_get_num_resources(res: *mut ti_sci_resource) -> u32;
}

extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}

