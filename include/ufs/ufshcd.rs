//! Automatically rewritten from C Header to Rust Module
//! Source: include/ufs/ufshcd.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Universal Flash Storage Host controller driver
// Copyright (C) 2011-2013 Samsung India Software Operations
// Copyright (c) 2013-2016, The Linux Foundation. All rights reserved.
//
// Authors:
// Santosh Yaraganavi <santosh.sy@samsung.com>
// Vinayak Holikatti <h.vinayak@samsung.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dev_cmd_type {
    DEV_CMD_TYPE_NOP		= 0x0,
    DEV_CMD_TYPE_QUERY		= 0x1,
    DEV_CMD_TYPE_RPMB		= 0x2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ufs_event_type {
// uic specific errors
    UFS_EVT_PA_ERR = 0,
    UFS_EVT_DL_ERR,
    UFS_EVT_NL_ERR,
    UFS_EVT_TL_ERR,
    UFS_EVT_DME_ERR,

// fatal errors
    UFS_EVT_AUTO_HIBERN8_ERR,
    UFS_EVT_FATAL_ERR,
    UFS_EVT_LINK_STARTUP_FAIL,
    UFS_EVT_RESUME_ERR,
    UFS_EVT_SUSPEND_ERR,
    UFS_EVT_WL_SUSP_ERR,
    UFS_EVT_WL_RES_ERR,

// abnormal events
    UFS_EVT_DEV_RESET,
    UFS_EVT_HOST_RESET,
    UFS_EVT_ABORT,

    UFS_EVT_CNT,
}

//
// struct uic_command - UIC command structure
// @command: UIC command
// @argument1: UIC command argument 1
// @argument2: UIC command argument 2
// @argument3: UIC command argument 3
// @cmd_active: Indicate if UIC command is outstanding
// @done: UIC command completion
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uic_command {
    pub command: u32,
    pub argument1: u32,
    pub argument2: u32,
    pub argument3: u32,
    pub cmd_active: bool,
    pub done: completion,
}

// Used to differentiate the power management options
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ufs_pm_op {
    UFS_RUNTIME_PM,
    UFS_SYSTEM_PM,
    UFS_SHUTDOWN_PM,
}

// Host <-> Device UniPro Link state
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uic_link_state {
    UIC_LINK_OFF_STATE	= 0, /* Link powered down or disabled */
    UIC_LINK_ACTIVE_STATE	= 1, /* Link is in Fast/Slow/Sleep state */
    UIC_LINK_HIBERN8_STATE	= 2, /* Link is in Hibernate state */
    UIC_LINK_BROKEN_STATE	= 3, /* Link is in broken state */
}

//
// UFS Power management levels.
// Each level is in increasing order of power savings, except DeepSleep
// which is lower than PowerDown with power on but not PowerDown with
// power off.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ufs_pm_level {
    UFS_PM_LVL_0,
    UFS_PM_LVL_1,
    UFS_PM_LVL_2,
    UFS_PM_LVL_3,
    UFS_PM_LVL_4,
    UFS_PM_LVL_5,
    UFS_PM_LVL_6,
    UFS_PM_LVL_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_pm_lvl_states {
    pub dev_state: ufs_dev_pwr_mode,
    pub link_state: uic_link_state,
}

//
// struct ufshcd_lrb - local reference block
// @utr_descriptor_ptr: UTRD address of the command
// @ucd_req_ptr: UCD address of the command
// @ucd_rsp_ptr: Response UPIU address for this command
// @ucd_prdt_ptr: PRDT address of the command
// @utrd_dma_addr: UTRD dma address for debug
// @ucd_prdt_dma_addr: PRDT dma address for debug
// @ucd_rsp_dma_addr: UPIU response dma address for debug
// @ucd_req_dma_addr: UPIU request dma address for debug
// @scsi_status: SCSI status of the command
// @command_type: SCSI, UFS, Query.
// @lun: LUN of the command
// @intr_cmd: Interrupt command (doesn't participate in interrupt aggregation)
// @req_abort_skip: skip request abort task flag
// @issue_time_stamp: time stamp for debug purposes (CLOCK_MONOTONIC)
// @issue_time_stamp_local_clock: time stamp for debug purposes (local_clock)
// @compl_time_stamp: time stamp for statistics (CLOCK_MONOTONIC)
// @compl_time_stamp_local_clock: time stamp for debug purposes (local_clock)
// @crypto_key_slot: the key slot to use for inline crypto (-1 if none)
// @data_unit_num: the data unit number for the first block for inline crypto
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufshcd_lrb {
    pub utr_descriptor_ptr: *mut utp_transfer_req_desc,
    pub ucd_req_ptr: *mut utp_upiu_req,
    pub ucd_rsp_ptr: *mut utp_upiu_rsp,
    pub ucd_prdt_ptr: *mut ufshcd_sg_entry,
    pub utrd_dma_addr: dma_addr_t,
    pub ucd_req_dma_addr: dma_addr_t,
    pub ucd_rsp_dma_addr: dma_addr_t,
    pub ucd_prdt_dma_addr: dma_addr_t,
    pub scsi_status: c_int,
    pub command_type: c_int,
    pub /: *mut *mut u8 lun; / UPIU LUN id field is only 8-bit wide,
    pub intr_cmd: bool,
    pub req_abort_skip: bool,
    pub issue_time_stamp: ktime_t,
    pub issue_time_stamp_local_clock: u64,
    pub compl_time_stamp: ktime_t,
    pub compl_time_stamp_local_clock: u64,

    pub crypto_key_slot: c_int,
    pub data_unit_num: u64,

}

//
// struct ufs_query_req - parameters for building a query request
// @query_func: UPIU header query function
// @upiu_req: the query request data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_query_req {
    pub query_func: u8,
    pub upiu_req: utp_upiu_query,
}

//
// struct ufs_query_res - UPIU QUERY
// @upiu_res: query response data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_query_res {
    pub upiu_res: utp_upiu_query,
}

//
// struct ufs_query - holds relevant data structures for query request
// @request: request upiu and function
// @descriptor: buffer for sending/receiving descriptor
// @response: response upiu and response
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_query {
    pub request: ufs_query_req,
    pub descriptor: *mut u8,
    pub response: ufs_query_res,
}

//
// struct ufs_dev_cmd - all assosiated fields with device management commands
// @type: device management command type - Query, NOP OUT
// @lock: lock to allow one command at a time
// @query: Device management query information
// @tag: tag of the reserved request in use
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_dev_cmd {
    pub type: dev_cmd_type,
    pub lock: mutex,
    pub query: ufs_query,
    pub tag: u8,
}

//
// struct ufs_clk_info - UFS clock related info
// @list: list headed by hba->clk_list_head
// @clk: clock node
// @name: clock name
// @max_freq: maximum frequency supported by the clock
// @min_freq: min frequency that can be used for clock scaling
// @curr_freq: indicates the current frequency that it is set to
// @keep_link_active: indicates that the clk should not be disabled if
// link is active
// @enabled: variable to check against multiple enable/disable
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_clk_info {
    pub list: list_head,
    pub clk: *mut clk,
    pub name: *const c_char,
    pub max_freq: u32,
    pub min_freq: u32,
    pub curr_freq: u32,
    pub keep_link_active: bool,
    pub enabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ufs_notify_change_status {
    PRE_CHANGE,
    POST_CHANGE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_pa_layer_attr {
    pub gear_rx: u32,
    pub gear_tx: u32,
    pub lane_rx: u32,
    pub lane_tx: u32,
    pub pwr_rx: u32,
    pub pwr_tx: u32,
    pub hs_rate: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_pwr_mode_info {
    pub is_valid: bool,
    pub info: ufs_pa_layer_attr,
}

pub const UFS_MAX_LANES: c_int = 2;
//
// struct tx_eqtr_iter - TX Equalization Training iterator
// @preshoot_bitmap: PreShoot bitmap
// @deemphasis_bitmap: DeEmphasis bitmap
// @preshoot: PreShoot value
// @deemphasis: DeEmphasis value
// @fom: Figure-of-Merit read out from RX_FOM
// @is_updated: Flag to indicate if updated since previous iteration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_eqtr_iter {
    pub preshoot_bitmap: c_ulong,
    pub deemphasis_bitmap: c_ulong,
    pub preshoot: u8,
    pub deemphasis: u8,
    pub fom: [u8; UFS_MAX_LANES],
    pub is_updated: bool,
}

//
// struct ufshcd_tx_eq_settings - TX Equalization settings
// @preshoot: PreShoot value
// @deemphasis: DeEmphasis value
// @fom_val: Figure-of-Merit value read out from RX_FOM (Bit[6:0])
// @precode_en: Flag to indicate whether need to enable pre-coding
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufshcd_tx_eq_settings {
    pub preshoot: u8,
    pub deemphasis: u8,
    pub fom_val: u8,
    pub precode_en: bool,
}

//
// struct ufshcd_tx_eqtr_data - Data used during TX Equalization Training procedure
// @host: Optimal TX EQ settings identified for host TX Lanes during TX EQTR
// @device: Optimal TX EQ settings identified for device TX Lanes during TX EQTR
// @host_fom: Host TX EQTR FOM record
// @device_fom: Device TX EQTR FOM record
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufshcd_tx_eqtr_data {
    pub host: [ufshcd_tx_eq_settings; UFS_MAX_LANES],
    pub device: [ufshcd_tx_eq_settings; UFS_MAX_LANES],
    pub host_fom: [u8; UFS_MAX_LANES][TX_HS_NUM_PRESHOOT][TX_HS_NUM_DEEMPHASIS],
    pub device_fom: [u8; UFS_MAX_LANES][TX_HS_NUM_PRESHOOT][TX_HS_NUM_DEEMPHASIS],
}

//
// struct ufshcd_tx_eqtr_record - TX Equalization Training record
// @host_fom: Host TX EQTR FOM record
// @device_fom: Device TX EQTR FOM record
// @last_record_ts: Timestamp of the most recent TX EQTR record
// @last_record_index: Index of the most recent TX EQTR record
// @saved_adapt_eqtr: Saved Adaptation length setting for TX EQTR
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufshcd_tx_eqtr_record {
    pub host_fom: [u8; UFS_MAX_LANES][TX_HS_NUM_PRESHOOT][TX_HS_NUM_DEEMPHASIS],
    pub device_fom: [u8; UFS_MAX_LANES][TX_HS_NUM_PRESHOOT][TX_HS_NUM_DEEMPHASIS],
    pub last_record_ts: ktime_t,
    pub last_record_index: u16,
    pub saved_adapt_eqtr: u16,
}

//
// struct ufshcd_tx_eq_params - TX Equalization parameters structure
// @host: TX EQ settings for host TX Lanes
// @device: TX EQ settings for device TX Lanes
// @eqtr_record: Pointer to TX EQTR record
// @is_valid: True if parameter contains valid TX Equalization settings
// @is_applied: True if settings have been applied to UniPro of both sides
// @is_trained: True if parameters obtained from TX EQTR procedure
// @from_dt: True if settings are from Device Tree
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufshcd_tx_eq_params {
    pub host: [ufshcd_tx_eq_settings; UFS_MAX_LANES],
    pub device: [ufshcd_tx_eq_settings; UFS_MAX_LANES],
    pub eqtr_record: *mut ufshcd_tx_eqtr_record,
    pub is_valid: bool,
    pub is_applied: bool,
    pub is_trained: bool,
    pub from_dt: bool,
}

//
// struct ufs_hba_variant_ops - variant specific callbacks
// @name: variant name
// @init: called when the driver is initialized
// @exit: called to cleanup everything done in init
// @set_dma_mask: For setting another DMA mask than indicated by the 64AS
// capability bit.
// @get_ufs_hci_version: called to get UFS HCI version
// @clk_scale_notify: notifies that clks are scaled up/down
// @setup_clocks: called before touching any of the controller registers
// @hce_enable_notify: called before and after HCE enable bit is set to allow
// variant specific Uni-Pro initialization.
// @link_startup_notify: called before and after Link startup is carried out
// to allow variant specific Uni-Pro initialization.
// @negotiate_pwr_mode: called to negotiate power mode.
// @pwr_change_notify: called before and after a power mode change
// is carried out to allow vendor spesific capabilities
// to be set.
// @setup_xfer_req: called before any transfer request is issued
// to set some things
// @setup_task_mgmt: called before any task management request is issued
// to set some things
// @hibern8_notify: called around hibern8 enter/exit
// @apply_dev_quirks: called to apply device specific quirks
// @fixup_dev_quirks: called to modify device specific quirks
// @suspend: called during host controller PM callback
// @resume: called during host controller PM callback
// @dbg_register_dump: used to dump controller debug information
// @phy_initialization: used to initialize phys
// @device_reset: called to issue a reset pulse on the UFS device
// @config_scaling_param: called to configure clock scaling parameters
// @fill_crypto_prdt: initialize crypto-related fields in the PRDT
// @event_notify: called to notify important events
// @mcq_config_resource: called to configure MCQ platform resources
// @get_hba_mac: reports maximum number of outstanding commands supported by
// the controller. Should be implemented for UFSHCI 4.0 or later
// controllers that are not compliant with the UFSHCI 4.0 specification.
// @op_runtime_config: called to config Operation and runtime regs Pointers
// @get_outstanding_cqs: called to get outstanding completion queues
// @config_esi: called to config Event Specific Interrupt
// @config_scsi_dev: called to configure SCSI device parameters
// @freq_to_gear_speed: called to map clock frequency to the max supported gear speed
// @apply_tx_eqtr_settings: called to apply settings for TX Equalization
// Training settings.
// @get_rx_fom: called to get Figure of Merit (FOM) value.
// @tx_eqtr_notify: called before and after TX Equalization Training procedure
// to allow platform vendor specific configs to take place.
// @get_hba_nortt: called to get maximum number of outstanding RTTs supported by
// the controller.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_hba_variant_ops {
    pub name: *const c_char,
    pub ): *mut *mut int (init)(struct ufs_hba,
    pub ): *mut *mut void (exit)(struct ufs_hba,
    pub ): *mut *mut u32 (get_ufs_hci_version)(struct ufs_hba,
    pub ): *mut *mut int (set_dma_mask)(struct ufs_hba,
    pub ufs_notify_change_status): enum,
    pub ufs_notify_change_status): enum,
    pub ufs_notify_change_status): enum,
    pub ufs_notify_change_status): enum,
    pub final_params): *mut ufs_pa_layer_attr,
    pub final_params): *mut ufs_pa_layer_attr,
    pub is_scsi_cmd): bool,
    pub u8): *mut *mut *mut void (setup_task_mgmt)(struct ufs_hba , int,,
    pub ufs_notify_change_status): enum,
    pub hba): *mut *mut int (apply_dev_quirks)(struct ufs_hba,
    pub hba): *mut *mut void (fixup_dev_quirks)(struct ufs_hba,
    pub ufs_notify_change_status): enum,
    pub ufs_pm_op): *mut *mut *mut int (resume)(struct ufs_hba , enum,
    pub hba): *mut *mut void (dbg_register_dump)(struct ufs_hba,
    pub ): *mut *mut int (phy_initialization)(struct ufs_hba,
    pub hba): *mut *mut int (device_reset)(struct ufs_hba,
    pub data): *mut devfreq_simple_ondemand_data,
    pub num_segments): *mut *mut void prdt, unsigned int,
    pub data): *mut ufs_event_type evt, void,
    pub hba): *mut *mut int (mcq_config_resource)(struct ufs_hba,
    pub hba): *mut *mut int (get_hba_mac)(struct ufs_hba,
    pub hba): *mut *mut int (op_runtime_config)(struct ufs_hba,
    pub ocqs): *mut c_ulong,
    pub hba): *mut *mut int (config_esi)(struct ufs_hba,
    pub sdev): *mut *mut void (config_scsi_dev)(struct scsi_device,
    pub freq): *mut *mut *mut u32 (freq_to_gear_speed)(struct ufs_hba hba, unsigned long,
    pub d_iter): *mut tx_eqtr_iter,
    pub d_iter): *mut tx_eqtr_iter,
    pub pwr_mode): *mut ufs_pa_layer_attr,
    pub hba): *mut *mut int (get_hba_nortt)(struct ufs_hba,
}

// clock gating state
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum clk_gating_state {
    CLKS_OFF,
    CLKS_ON,
    REQ_CLKS_OFF,
    REQ_CLKS_ON,
}

//
// struct ufs_clk_gating - UFS clock gating related info
// @gate_work: worker to turn off clocks after some delay as specified in
// delay_ms
// @ungate_work: worker to turn on clocks that will be used in case of
// interrupt context
// @clk_gating_workq: workqueue for clock gating work.
// @lock: serialize access to some struct ufs_clk_gating members. An outer lock
// relative to the host lock
// @state: the current clocks state
// @delay_ms: gating delay in ms
// @is_suspended: clk gating is suspended when set to 1 which can be used
// during suspend/resume
// @delay_attr: sysfs attribute to control delay_attr
// @enable_attr: sysfs attribute to enable/disable clock gating
// @is_enabled: Indicates the current status of clock gating
// @is_initialized: Indicates whether clock gating is initialized or not
// @active_reqs: number of requests that are pending and should be waited for
// completion before gating clocks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_clk_gating {
    pub gate_work: delayed_work,
    pub ungate_work: work_struct,
    pub clk_gating_workq: *mut workqueue_struct,
    pub lock: spinlock_t,
    pub state: clk_gating_state,
    pub delay_ms: c_ulong,
    pub is_suspended: bool,
    pub delay_attr: device_attribute,
    pub enable_attr: device_attribute,
    pub is_enabled: bool,
    pub is_initialized: bool,
    pub active_reqs: c_int,
}

//
// struct ufs_clk_scaling - UFS clock scaling related data
// @workq: workqueue to schedule devfreq suspend/resume work
// @suspend_work: worker to suspend devfreq
// @resume_work: worker to resume devfreq
// @lock: serialize access to some struct ufs_clk_scaling members
// @active_reqs: number of requests that are pending. If this is zero when
// devfreq ->target() function is called then schedule "suspend_work" to
// suspend devfreq.
// @tot_busy_t: Total busy time in current polling window
// @window_start_t: Start time (in jiffies) of the current polling window
// @busy_start_t: Start time of current busy period
// @enable_attr: sysfs attribute to enable/disable clock scaling
// @saved_pwr_info: UFS power mode may also be changed during scaling and this
// one keeps track of previous power mode.
// @target_freq: frequency requested by devfreq framework
// @min_gear: lowest HS gear to scale down to
// @wb_gear: enable Write Booster when HS gear scales above or equal to it, else
// disable Write Booster
// @is_enabled: tracks if scaling is currently enabled or not, controlled by
// clkscale_enable sysfs node
// @is_allowed: tracks if scaling is currently allowed or not, used to block
// clock scaling which is not invoked from devfreq governor
// @is_initialized: Indicates whether clock scaling is initialized or not
// @is_busy_started: tracks if busy period has started or not
// @is_suspended: tracks if devfreq is suspended or not
// @suspend_on_no_request: suspend clock scaling only when no request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_clk_scaling {
    pub workq: *mut workqueue_struct,
    pub suspend_work: work_struct,
    pub resume_work: work_struct,
    pub lock: spinlock_t,
    pub active_reqs: c_int,
    pub tot_busy_t: c_ulong,
    pub window_start_t: ktime_t,
    pub busy_start_t: ktime_t,
    pub enable_attr: device_attribute,
    pub saved_pwr_info: ufs_pa_layer_attr,
    pub target_freq: c_ulong,
    pub min_gear: u32,
    pub wb_gear: u32,
    pub is_enabled: bool,
    pub is_allowed: bool,
    pub is_initialized: bool,
    pub is_busy_started: bool,
    pub is_suspended: bool,
    pub suspend_on_no_request: bool,
}

pub const UFS_EVENT_HIST_LENGTH: c_int = 8;
//
// struct ufs_event_hist - keeps history of errors
// @pos: index to indicate cyclic buffer position
// @val: cyclic buffer for registers value
// @tstamp: cyclic buffer for time stamp
// @cnt: error counter
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_event_hist {
    pub pos: c_int,
    pub val: [u32; UFS_EVENT_HIST_LENGTH],
    pub tstamp: [u64; UFS_EVENT_HIST_LENGTH],
    pub cnt: c_ulonglong,
}

//
// struct ufs_stats - keeps usage/err statistics
// @hibern8_exit_cnt: Counter to keep track of number of exits,
// reset this after link-startup.
// @last_hibern8_exit_tstamp: Set time after the hibern8 exit.
// Clear after the first successful command completion.
// @event: array with event history.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_stats {
    pub hibern8_exit_cnt: u32,
    pub last_hibern8_exit_tstamp: u64,
    pub event: [ufs_event_hist; UFS_EVT_CNT],
}

//
// enum ufshcd_state - UFS host controller state
// @UFSHCD_STATE_RESET: Link is not operational. Postpone SCSI command
// processing.
// @UFSHCD_STATE_OPERATIONAL: The host controller is operational and can process
// SCSI commands.
// @UFSHCD_STATE_EH_SCHEDULED_NON_FATAL: The error handler has been scheduled.
// SCSI commands may be submitted to the controller.
// @UFSHCD_STATE_EH_SCHEDULED_FATAL: The error handler has been scheduled. Fail
// newly submitted SCSI commands with error code DID_BAD_TARGET.
// @UFSHCD_STATE_ERROR: An unrecoverable error occurred, e.g. link recovery
// failed. Fail all SCSI commands with error code DID_ERROR.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ufshcd_state {
    UFSHCD_STATE_RESET,
    UFSHCD_STATE_OPERATIONAL,
    UFSHCD_STATE_EH_SCHEDULED_NON_FATAL,
    UFSHCD_STATE_EH_SCHEDULED_FATAL,
    UFSHCD_STATE_ERROR,
}

//
// enum ufshcd_pmc_policy - Power Mode change policy
// @UFSHCD_PMC_POLICY_DONT_FORCE: Do not force a Power Mode change.
// @UFSHCD_PMC_POLICY_FORCE: Force a Power Mode change even if current Power
// Mode is same as target Power Mode.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ufshcd_pmc_policy {
    UFSHCD_PMC_POLICY_DONT_FORCE,
    UFSHCD_PMC_POLICY_FORCE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ufshcd_quirks {
// Interrupt aggregation support is broken
    UFSHCD_QUIRK_BROKEN_INTR_AGGR			= 1 << 0,

//
// delay before each dme command is required as the unipro
// layer has shown instabilities
//
    UFSHCD_QUIRK_DELAY_BEFORE_DME_CMDS		= 1 << 1,

//
// If UFS host controller is having issue in processing LCC (Line
// Control Command) coming from device then enable this quirk.
// When this quirk is enabled, host controller driver should disable
// the LCC transmission on UFS device (by clearing TX_LCC_ENABLE
// attribute of device to 0).
//
    UFSHCD_QUIRK_BROKEN_LCC				= 1 << 2,

//
// The attribute PA_RXHSUNTERMCAP specifies whether or not the
// inbound Link supports unterminated line in HS mode. Setting this
// attribute to 1 fixes moving to HS gear.
//
    UFSHCD_QUIRK_BROKEN_PA_RXHSUNTERMCAP		= 1 << 3,

//
// This quirk needs to be enabled if the host controller only allows
// accessing the peer dme attributes in AUTO mode (FAST AUTO or
// SLOW AUTO).
//
    UFSHCD_QUIRK_DME_PEER_ACCESS_AUTO_MODE		= 1 << 4,

//
// This quirk needs to be enabled if the host controller doesn't
// advertise the correct version in UFS_VER register. If this quirk
// is enabled, standard UFS host driver will call the vendor specific
// ops (get_ufs_hci_version) to get the correct version.
//
    UFSHCD_QUIRK_BROKEN_UFS_HCI_VERSION		= 1 << 5,

//
// Clear handling for transfer/task request list is just opposite.
//
    UFSHCI_QUIRK_BROKEN_REQ_LIST_CLR		= 1 << 6,

//
// This quirk needs to be enabled if host controller doesn't allow
// that the interrupt aggregation timer and counter are reset by s/w.
//
    UFSHCI_QUIRK_SKIP_RESET_INTR_AGGR		= 1 << 7,

//
// This quirks needs to be enabled if host controller cannot be
// enabled via HCE register.
//
    UFSHCI_QUIRK_BROKEN_HCE				= 1 << 8,

//
// This quirk needs to be enabled if the host controller regards
// resolution of the values of PRDTO and PRDTL in UTRD as byte.
//
    UFSHCD_QUIRK_PRDT_BYTE_GRAN			= 1 << 9,

//
// This quirk needs to be enabled if the host controller reports
// OCS FATAL ERROR with device error through sense data
//
    UFSHCD_QUIRK_BROKEN_OCS_FATAL_ERROR		= 1 << 10,

//
// This quirk needs to be enabled if the host controller has
// auto-hibernate capability but it doesn't work.
//
    UFSHCD_QUIRK_BROKEN_AUTO_HIBERN8		= 1 << 11,

//
// This quirk needs to disable manual flush for write booster
//
    UFSHCI_QUIRK_SKIP_MANUAL_WB_FLUSH_CTRL		= 1 << 12,

//
// This quirk needs to disable unipro timeout values
// before power mode change
//
    UFSHCD_QUIRK_SKIP_DEF_UNIPRO_TIMEOUT_SETTING = 1 << 13,

//
// This quirk needs to be enabled if the host controller does not
// support UIC command
//
    UFSHCD_QUIRK_BROKEN_UIC_CMD			= 1 << 15,

//
// This quirk needs to be enabled if the host controller cannot
// support physical host configuration.
//
    UFSHCD_QUIRK_SKIP_PH_CONFIGURATION		= 1 << 16,

//
// This quirk needs to be enabled if the host controller has
// auto-hibernate capability but it's FASTAUTO only.
//
    UFSHCD_QUIRK_HIBERN_FASTAUTO			= 1 << 18,

//
// This quirk needs to be enabled if the host controller needs
// to reinit the device after switching to maximum gear.
//
    UFSHCD_QUIRK_REINIT_AFTER_MAX_GEAR_SWITCH       = 1 << 19,

//
// Some host raises interrupt (per queue) in addition to
// CQES (traditional) when ESI is disabled.
// Enable this quirk will disable CQES and use per queue interrupt.
//
    UFSHCD_QUIRK_MCQ_BROKEN_INTR			= 1 << 20,

//
// Some host does not implement SQ Run Time Command (SQRTC) register
// thus need this quirk to skip related flow.
//
    UFSHCD_QUIRK_MCQ_BROKEN_RTC			= 1 << 21,

//
// This quirk needs to be enabled if the host controller supports inline
// encryption but it needs to initialize the crypto capabilities in a
// nonstandard way and/or needs to override blk_crypto_ll_ops.  If
// enabled, the standard code won't initialize the blk_crypto_profile;
// ufs_hba_variant_ops::init() must do it instead.
//
    UFSHCD_QUIRK_CUSTOM_CRYPTO_PROFILE		= 1 << 22,

//
// This quirk needs to be enabled if the host controller supports inline
// encryption but does not support the CRYPTO_GENERAL_ENABLE bit, i.e.
// host controller initialization fails if that bit is set.
//
    UFSHCD_QUIRK_BROKEN_CRYPTO_ENABLE		= 1 << 23,

//
// This quirk needs to be enabled if the host controller driver copies
// cryptographic keys into the PRDT in order to send them to hardware,
// and therefore the PRDT should be zeroized after each request (as per
// the standard best practice for managing keys).
//
    UFSHCD_QUIRK_KEYS_IN_PRDT			= 1 << 24,

//
// This quirk indicates that the controller reports the value 1 (not
// supported) in the Legacy Single DoorBell Support (LSDBS) bit of the
// Controller Capabilities register although it supports the legacy
// single doorbell mode.
//
    UFSHCD_QUIRK_BROKEN_LSDBS_CAP			= 1 << 25,

//
// This quirk indicates that DME_LINKSTARTUP should not be issued a 2nd
// time (refer link_startup_again) after the 1st time was successful,
// because it causes link startup to become unreliable.
//
    UFSHCD_QUIRK_PERFORM_LINK_STARTUP_ONCE		= 1 << 26,

//
// On some platforms, the VCC regulator has a slow ramp-up time. Add a
// delay after enabling VCC to ensure it's stable.
//
    UFSHCD_QUIRK_VCC_ON_DELAY			= 1 << 27,

//
// This quirk indicates that Host supports TX Equalization Training
// (EQTR) using Adapt L0L1L2L3 length which is larger than what is
// allowed by M-PHY spec ver 6.0.
//
    UFSHCD_QUIRK_EXTENDED_TX_EQTR_ADAPT_LENGTH_L0L1L2L3	= 1 << 28,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ufshcd_caps {
// Allow dynamic clk gating
    UFSHCD_CAP_CLK_GATING				= 1 << 0,

// Allow hiberb8 with clk gating
    UFSHCD_CAP_HIBERN8_WITH_CLK_GATING		= 1 << 1,

// Allow dynamic clk scaling
    UFSHCD_CAP_CLK_SCALING				= 1 << 2,

// Allow auto bkops to enabled during runtime suspend
    UFSHCD_CAP_AUTO_BKOPS_SUSPEND			= 1 << 3,

//
// This capability allows host controller driver to use the UFS HCI's
// interrupt aggregation capability.
// CAUTION: Enabling this might reduce overall UFS throughput.
//
    UFSHCD_CAP_INTR_AGGR				= 1 << 4,

//
// This capability allows the device auto-bkops to be always enabled
// except during suspend (both runtime and suspend).
// Enabling this capability means that device will always be allowed
// to do background operation when it's active but it might degrade
// the performance of ongoing read/write operations.
//
    UFSHCD_CAP_KEEP_AUTO_BKOPS_ENABLED_EXCEPT_SUSPEND = 1 << 5,

//
// This capability allows host controller driver to automatically
// enable runtime power management by itself instead of waiting
// for userspace to control the power management.
//
    UFSHCD_CAP_RPM_AUTOSUSPEND			= 1 << 6,

//
// This capability allows the host controller driver to turn-on
// WriteBooster, if the underlying device supports it and is
// provisioned to be used. This would increase the write performance.
//
    UFSHCD_CAP_WB_EN				= 1 << 7,

//
// This capability allows the host controller driver to use the
// inline crypto engine, if it is present
//
    UFSHCD_CAP_CRYPTO				= 1 << 8,

//
// This capability allows the controller regulators to be put into
// lpm mode aggressively during clock gating.
// This would increase power savings.
//
    UFSHCD_CAP_AGGR_POWER_COLLAPSE			= 1 << 9,

//
// This capability allows the host controller driver to use DeepSleep,
// if it is supported by the UFS device. The host controller driver must
// support device hardware reset via the hba->device_reset() callback,
// in order to exit DeepSleep state.
//
    UFSHCD_CAP_DEEPSLEEP				= 1 << 10,

//
// This capability allows the host controller driver to use temperature
// notification if it is supported by the UFS device.
//
    UFSHCD_CAP_TEMP_NOTIF				= 1 << 11,

//
// Enable WriteBooster when scaling up the clock and disable
// WriteBooster when scaling the clock down.
//
    UFSHCD_CAP_WB_WITH_CLK_SCALING			= 1 << 12,

//
// This capability allows the host controller driver to apply TX
// Equalization settings discovered from UFS attributes, variant
// specific operations and TX Equaliztion Training procedure.
//
    UFSHCD_CAP_TX_EQUALIZATION			= 1 << 13,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_hba_variant_params {
    pub devfreq_profile: devfreq_dev_profile,
    pub ondemand_data: devfreq_simple_ondemand_data,
    pub hba_enable_delay_us: u16,
    pub wb_flush_threshold: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_hba_monitor {
    pub chunk_size: c_ulong,
    pub nr_sec_rw: [c_ulong; 2],
    pub total_busy: [ktime_t; 2],
    pub nr_req: [c_ulong; 2],
// latencies
    pub lat_sum: [ktime_t; 2],
    pub lat_max: [ktime_t; 2],
    pub lat_min: [ktime_t; 2],
    pub nr_queued: [u32; 2],
    pub busy_start_ts: [ktime_t; 2],
    pub enabled_ts: ktime_t,
    pub enabled: bool,
}

//
// struct ufshcd_mcq_opr_info_t - Operation and Runtime registers
//
// @offset: Doorbell Address Offset
// @stride: Steps proportional to queue [0...31]
// @base: base address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufshcd_mcq_opr_info_t {
    pub offset: c_ulong,
    pub stride: c_ulong,
    pub base: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ufshcd_mcq_opr {
    OPR_SQD,
    OPR_SQIS,
    OPR_CQD,
    OPR_CQIS,
    OPR_MAX,
}

//
// struct ufs_hba - per adapter private structure
// @mmio_base: UFSHCI base register address
// @ucdl_base_addr: UFS Command Descriptor base address
// @utrdl_base_addr: UTP Transfer Request Descriptor base address
// @utmrdl_base_addr: UTP Task Management Descriptor base address
// @devman_ucd_base_addr: UFS Command Descriptor base address for the reserved
// device management tag (has a larger response area)
// @ucdl_dma_addr: UFS Command Descriptor DMA address
// @utrdl_dma_addr: UTRDL DMA address
// @utmrdl_dma_addr: UTMRDL DMA address
// @devman_ucd_dma_addr: UFS Command Descriptor DMA address for the reserved
// device management tag
// @host: Scsi_Host instance of the driver
// @dev: device handle
// @ufs_device_wlun: WLUN that controls the entire UFS device.
// @ufs_rpmb_wlun: RPMB WLUN SCSI device
// @hwmon_device: device instance registered with the hwmon core.
// @curr_dev_pwr_mode: active UFS device power mode.
// @uic_link_state: active state of the link to the UFS device.
// @rpm_lvl: desired UFS power management level during runtime PM.
// @spm_lvl: desired UFS power management level during system PM.
// @pm_lvl_min: minimum supported power management level.
// @pm_op_in_progress: whether or not a PM operation is in progress.
// @ahit: value of Auto-Hibernate Idle Timer register.
// @outstanding_tasks: Bits representing outstanding task requests
// @outstanding_lock: Protects @outstanding_reqs.
// @outstanding_reqs: Bits representing outstanding transfer requests
// @capabilities: UFS Controller Capabilities
// @mcq_capabilities: UFS Multi Circular Queue capabilities
// @nutrs: Transfer Request Queue depth supported by controller
// @nortt: Max outstanding RTTs supported by controller
// @nutmrs: Task Management Queue depth supported by controller
// @ufs_version: UFS Version to which controller complies
// @vops: pointer to variant specific operations
// @vps: pointer to variant specific parameters
// @priv: pointer to variant specific private data
// @sg_entry_size: size of struct ufshcd_sg_entry (may include variant fields)
// @irq: Irq number of the controller
// @is_irq_enabled: whether or not the UFS controller interrupt is enabled.
// @dev_ref_clk_freq: reference clock frequency
// @quirks: bitmask with information about deviations from the UFSHCI standard.
// @dev_quirks: bitmask with information about deviations from the UFS standard.
// @tmf_tag_set: TMF tag set.
// @tmf_queue: Used to allocate TMF tags.
// @tmf_rqs: array with pointers to TMF requests while these are in progress.
// @active_uic_cmd: pointer to active UIC command.
// @uic_cmd_mutex: mutex used for serializing UIC command processing.
// @uic_async_done: completion used to wait for power mode or hibernation state
// changes.
// @ufshcd_state: UFSHCD state
// @eh_flags: Error handling flags
// @intr_mask: Interrupt Mask Bits
// @ee_ctrl_mask: Exception event control mask
// @ee_drv_mask: Exception event mask for driver
// @ee_usr_mask: Exception event mask for user (set via debugfs)
// @ee_ctrl_mutex: Used to serialize exception event information.
// @is_powered: flag to check if HBA is powered
// @shutting_down: flag to check if shutdown has been invoked
// @host_sem: semaphore used to serialize concurrent contexts
// @eh_wq: Workqueue that eh_work works on
// @eh_work: Worker to handle UFS errors that require s/w attention
// @eeh_work: Worker to handle exception events
// @errors: HBA errors
// @uic_error: UFS interconnect layer error status
// @saved_err: sticky error mask
// @saved_uic_err: sticky UIC error mask
// @ufs_stats: various error counters
// @force_reset: flag to force eh_work perform a full reset
// @silence_err_logs: flag to silence error logs
// @dev_cmd: ufs device management command information
// @last_dme_cmd_tstamp: time stamp of the last completed DME command
// @nop_out_timeout: NOP OUT timeout value
// @dev_info: information about the UFS device
// @auto_bkops_enabled: to track whether bkops is enabled in device
// @vreg_info: UFS device voltage regulator information
// @clk_list_head: UFS host controller clocks list node head
// @use_pm_opp: Indicates whether OPP based scaling is used or not
// @req_abort_count: number of times ufshcd_abort() has been called
// @lanes_per_direction: number of lanes per data direction between the UFS
// controller and the UFS device.
// @pwr_info: holds current power mode
// @max_pwr_info: keeps the device max valid pwm
// @clk_gating: information related to clock gating
// @caps: bitmask with information about UFS controller capabilities
// @devfreq: frequency scaling information owned by the devfreq core
// @clk_scaling: frequency scaling information owned by the UFS driver
// @is_sys_suspended: UFS device has been suspended because of system suspend
// @urgent_bkops_lvl: keeps track of urgent bkops level for device
// @is_urgent_bkops_lvl_checked: keeps track if the urgent bkops level for
// device is known or not.
// @wb_mutex: used to serialize devfreq and sysfs write booster toggling
// @clk_scaling_lock: used to serialize device commands and clock scaling
// @bsg_dev: struct device associated with the BSG queue
// @bsg_queue: BSG queue associated with the UFS controller
// @rpm_dev_flush_recheck_work: used to suspend from RPM (runtime power
// management) after the UFS device has finished a WriteBooster buffer
// flush or auto BKOP.
// @monitor: statistics about UFS commands
// @crypto_capabilities: Content of crypto capabilities register (0x100)
// @crypto_cap_array: Array of crypto capabilities
// @crypto_cfg_register: Start of the crypto cfg array
// @crypto_profile: the crypto profile of this hba (if applicable)
// @debugfs_root: UFS controller debugfs root directory
// @debugfs_ee_work: used to restore ee_ctrl_mask after a delay
// @debugfs_ee_rate_limit_ms: user configurable delay after which to restore
// ee_ctrl_mask
// @trigger_eh_attr: fault attributes for the fault injection trigger EH
// @timeout_attr: fault attributes for the timeout fault injection trigger
// @luns_avail: number of regular and well known LUNs supported by the UFS
// device
// @nr_hw_queues: number of hardware queues configured
// @nr_queues: number of Queues of different queue types
// @complete_put: whether or not to call ufshcd_rpm_put() from inside
// ufshcd_resume_complete()
// @scsi_host_added: indicates that scsi_add_host() has been called
// @mcq_sup: is mcq supported by UFSHC
// @lsdb_sup: LSDBS capability
// @mcq_enabled: is mcq ready to accept requests
// @mcq_esi_enabled: is mcq ESI configured
// @mcq_base: Multi circular queue registers base address
// @uhq: array of supported hardware queues
// @mcq_opr: MCQ operation and runtime registers
// @ufs_rtc_update_work: A work for UFS RTC periodic update
// @pm_qos_req: PM QoS request handle
// @pm_qos_enabled: flag to check if pm qos is enabled
// @pm_qos_mutex: synchronizes PM QoS request and status updates
// @critical_health_count: count of critical health exceptions
// @dev_lvl_exception_count: count of device level exceptions since last reset
// @dev_lvl_exception_id: vendor specific information about the device level exception event.
// @dme_qos_notification: Bitfield of pending DME Quality of Service (QoS)
// events. Bits[3:1] reflect the corresponding bits of UIC DME Error Code
// field within the Host Controller's UECDME register. Bit[0] is a flag
// indicating that the DME QoS Monitor has been reset by the host.
// @dme_qos_sysfs_handle: handle for 'dme_qos_notification' sysfs entry
// @vcc_off_delay_us: length of delay after VCC is powered off
// @rpmbs: list of OP-TEE RPMB devices (one per RPMB region)
// @host_preshoot_cap: a bitfield to indicate supported PreShoot dBs of host's TX lanes, cache of
// host M-PHY TX_HS_PreShoot_Setting_Capability Attribute (ID 0x15)
// @host_deemphasis_cap: a bitfield to indicate supported DeEmphasis dBs of host's TX lanes, cache
// of host M-PHY TX_HS_DeEmphasis_Setting_Capability Attribute (ID 0x12)
// @device_preshoot_cap: a bitfield to indicate supported PreShoot dBs of device's TX lanes, cache
// of device M-PHY TX_HS_PreShoot_Setting_Capability Attribute (ID 0x15)
// @device_deemphasis_cap: a bitfield to indicate supported DeEmphasis dBs of device's TX lanes,
// cache of device M-PHY TX_HS_DeEmphasis_Setting_Capability Attribute (ID 0x12)
// @tx_eq_params: TX Equalization settings
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_hba {
    pub mmio_base: *mut void __iomem,
// Virtual memory reference
    pub ucdl_base_addr: *mut utp_transfer_cmd_desc,
    pub utrdl_base_addr: *mut utp_transfer_req_desc,
    pub utmrdl_base_addr: *mut utp_task_req_desc,
    pub devman_ucd_base_addr: *mut utp_devman_cmd_desc,
// DMA memory reference
    pub ucdl_dma_addr: dma_addr_t,
    pub utrdl_dma_addr: dma_addr_t,
    pub utmrdl_dma_addr: dma_addr_t,
    pub devman_ucd_dma_addr: dma_addr_t,
    pub host: *mut Scsi_Host,
    pub dev: *mut device,
    pub ufs_device_wlun: *mut scsi_device,
    pub ufs_rpmb_wlun: *mut scsi_device,

    pub hwmon_device: *mut device,

    pub curr_dev_pwr_mode: ufs_dev_pwr_mode,
    pub uic_link_state: uic_link_state,
// Desired UFS power management level during runtime PM
    pub rpm_lvl: ufs_pm_level,
// Desired UFS power management level during system PM
    pub spm_lvl: ufs_pm_level,
    pub pm_lvl_min: ufs_pm_level,
    pub pm_op_in_progress: c_int,
// Auto-Hibernate Idle Timer register value
    pub ahit: u32,
    pub outstanding_tasks: c_ulong,
    pub outstanding_lock: spinlock_t,
    pub outstanding_reqs: c_ulong,
    pub capabilities: u32,
    pub nutrs: c_int,
    pub nortt: c_int,
    pub mcq_capabilities: u32,
    pub nutmrs: c_int,
    pub ufs_version: u32,
    pub vops: *const ufs_hba_variant_ops,
    pub vps: *mut ufs_hba_variant_params,
    pub priv: *mut c_void,

    pub sg_entry_size: usize,

    pub irq: c_uint,
    pub is_irq_enabled: bool,
    pub dev_ref_clk_freq: ufs_ref_clk_freq,
    pub /: *mut *mut unsigned int quirks; / Deviations from standard UFSHCI spec.,
// Device deviations from standard UFS device spec.
    pub dev_quirks: c_uint,
    pub tmf_tag_set: blk_mq_tag_set,
    pub tmf_queue: *mut request_queue,
    pub tmf_rqs: *mut request,
    pub active_uic_cmd: *mut uic_command,
    pub uic_cmd_mutex: mutex,
    pub uic_async_done: *mut completion,
    pub ufshcd_state: ufshcd_state,
    pub eh_flags: u32,
    pub intr_mask: u32,
    pub ee_ctrl_mask: u16,
    pub ee_drv_mask: u16,
    pub ee_usr_mask: u16,
    pub ee_ctrl_mutex: mutex,
    pub is_powered: bool,
    pub shutting_down: bool,
    pub host_sem: semaphore,
// Work Queues
    pub eh_wq: *mut workqueue_struct,
    pub eh_work: work_struct,
    pub eeh_work: work_struct,
// HBA Errors
    pub errors: u32,
    pub uic_error: u32,
    pub saved_err: u32,
    pub saved_uic_err: u32,
    pub ufs_stats: ufs_stats,
    pub force_reset: bool,
    pub silence_err_logs: bool,
// Device management request data
    pub dev_cmd: ufs_dev_cmd,
    pub last_dme_cmd_tstamp: ktime_t,
    pub nop_out_timeout: c_int,
// Keeps information of the UFS device connected to this host
    pub dev_info: ufs_dev_info,
    pub auto_bkops_enabled: bool,
    pub vreg_info: ufs_vreg_info,
    pub clk_list_head: list_head,
    pub use_pm_opp: bool,
// Number of requests aborts
    pub req_abort_count: c_int,
// Number of lanes available (1 or 2) for Rx/Tx
    pub lanes_per_direction: u32,
    pub pwr_info: ufs_pa_layer_attr,
    pub max_pwr_info: ufs_pwr_mode_info,
    pub clk_gating: ufs_clk_gating,
// Control to enable/disable host capabilities
    pub caps: u32,
    pub devfreq: *mut devfreq,
    pub clk_scaling: ufs_clk_scaling,
    pub is_sys_suspended: bool,
    pub urgent_bkops_lvl: bkops_status,
    pub is_urgent_bkops_lvl_checked: bool,
    pub wb_mutex: mutex,
    pub clk_scaling_lock: rw_semaphore,
    pub bsg_dev: device,
    pub bsg_queue: *mut request_queue,
    pub rpm_dev_flush_recheck_work: delayed_work,
    pub monitor: ufs_hba_monitor,

    pub crypto_capabilities: ufs_crypto_capabilities,
    pub crypto_cap_array: *mut ufs_crypto_cap_entry,
    pub crypto_cfg_register: u32,
    pub crypto_profile: blk_crypto_profile,

    pub debugfs_root: *mut dentry,
    pub debugfs_ee_work: delayed_work,
    pub debugfs_ee_rate_limit_ms: u32,

    pub trigger_eh_attr: fault_attr,
    pub timeout_attr: fault_attr,

    pub luns_avail: u32,
    pub nr_hw_queues: c_uint,
    pub nr_queues: [c_uint; HCTX_MAX_TYPES],
    pub complete_put: bool,
    pub scsi_host_added: bool,
    pub mcq_sup: bool,
    pub lsdb_sup: bool,
    pub mcq_enabled: bool,
    pub mcq_esi_enabled: bool,
    pub mcq_base: *mut void __iomem,
    pub uhq: *mut ufs_hw_queue,
    pub mcq_opr: [ufshcd_mcq_opr_info_t; OPR_MAX],
    pub ufs_rtc_update_work: delayed_work,
    pub pm_qos_req: pm_qos_request,
    pub pm_qos_enabled: bool,
// synchronizes PM QoS request and status updates
    pub pm_qos_mutex: mutex,
    pub critical_health_count: c_int,
    pub dev_lvl_exception_count: core::sync::atomic::AtomicI32,
    pub dev_lvl_exception_id: u64,
    pub dme_qos_notification: core::sync::atomic::AtomicI32,
    pub dme_qos_sysfs_handle: *mut kernfs_node,
    pub vcc_off_delay_us: u32,
    pub rpmbs: list_head,
    pub host_preshoot_cap: u8,
    pub host_deemphasis_cap: u8,
    pub device_preshoot_cap: u8,
    pub device_deemphasis_cap: u8,
    pub tx_eq_params: [ufshcd_tx_eq_params; UFS_HS_GEAR_MAX],
}

//
// struct ufs_hw_queue - per hardware queue structure
// @mcq_sq_head: base address of submission queue head pointer
// @mcq_sq_tail: base address of submission queue tail pointer
// @mcq_cq_head: base address of completion queue head pointer
// @mcq_cq_tail: base address of completion queue tail pointer
// @sqe_base_addr: submission queue entry base address
// @sqe_dma_addr: submission queue dma address
// @cqe_base_addr: completion queue base address
// @cqe_dma_addr: completion queue dma address
// @max_entries: max number of slots in this hardware queue
// @id: hardware queue ID
// @sq_tail_slot: current slot to which SQ tail pointer is pointing
// @sq_lock: serialize submission queue access
// @cq_tail_slot: current slot to which CQ tail pointer is pointing
// @cq_head_slot: current slot to which CQ head pointer is pointing
// @cq_lock: Synchronize between multiple polling instances
// @sq_mutex: prevent submission queue concurrent access
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_hw_queue {
    pub mcq_sq_head: *mut void __iomem,
    pub mcq_sq_tail: *mut void __iomem,
    pub mcq_cq_head: *mut void __iomem,
    pub mcq_cq_tail: *mut void __iomem,
    pub sqe_base_addr: *mut utp_transfer_req_desc,
    pub sqe_dma_addr: dma_addr_t,
    pub cqe_base_addr: *mut cq_entry,
    pub cqe_dma_addr: dma_addr_t,
    pub max_entries: u32,
    pub id: u32,
    pub sq_tail_slot: u32,
    pub sq_lock: spinlock_t,
    pub cq_tail_slot: u32,
    pub cq_head_slot: u32,
    pub cq_lock: spinlock_t,
// prevent concurrent access to submission queue
    pub sq_mutex: mutex,
}

pub const MCQ_QCFG_SIZE: c_uint = 0x40;

extern "C" {
    pub fn sizeof(ufshcd_sg_entry: struct) -> return;
}

extern "C" {
    pub fn container_of(_arg: profile, ufs_hba: struct, _arg: crypto_profile) -> return;
}

extern "C" {
    pub fn sizeof(ufshcd_sg_entry_size(hba: *mut *mut utp_transfer_cmd_desc) + SG_ALL) -> return;
}
//
// Two entries should be enough for the largest devman PRDT transfer (4 KiB),
// like advanced RPMB.
//
pub const UFSHCD_DEVMAN_SG_ENTRIES: c_int = 2;
extern "C" {
    pub fn ufshcd_sg_entry_size(_arg: hba) -> *mut UFSHCD_DEVMAN_SG_ENTRIES;
}
// Returns true if clocks can be gated. Otherwise false
extern "C" {
    pub fn FIELD_GET(_arg: UFSHCI_AHIBERN8_TIMER_MASK, _arg: hba->ahit) -> return;
}

//
// ufshcd_rmwl - perform read/modify/write for a controller register
// @hba: per adapter instance
// @mask: mask to apply on read value
// @val: actual value to write
// @reg: register address
//
extern "C" {
    pub fn ufshcd_enable_irq(hba: *mut ufs_hba);
}
extern "C" {
    pub fn ufshcd_disable_irq(hba: *mut ufs_hba);
}
extern "C" {
    pub fn ufshcd_alloc_host(: *mut device, : *mut ufs_hba) -> c_int;
}
extern "C" {
    pub fn ufshcd_hba_enable(hba: *mut ufs_hba) -> c_int;
}
extern "C" {
    pub fn ufshcd_init(: *mut ufs_hba, : *mut void __iomem, int: unsigned) -> c_int;
}
extern "C" {
    pub fn ufshcd_link_recovery(hba: *mut ufs_hba) -> c_int;
}
extern "C" {
    pub fn ufshcd_make_hba_operational(hba: *mut ufs_hba) -> c_int;
}
extern "C" {
    pub fn ufshcd_remove(: *mut ufs_hba);
}
extern "C" {
    pub fn ufshcd_uic_hibern8_enter(hba: *mut ufs_hba) -> c_int;
}
extern "C" {
    pub fn ufshcd_uic_hibern8_exit(hba: *mut ufs_hba) -> c_int;
}
extern "C" {
    pub fn ufshcd_delay_us(us: c_ulong, tolerance: c_ulong);
}
extern "C" {
    pub fn ufshcd_parse_dev_ref_clk_freq(hba: *mut ufs_hba, refclk: *mut clk);
}
extern "C" {
    pub fn ufshcd_update_evt_hist(hba: *mut ufs_hba, id: u32, val: u32);
}
extern "C" {
    pub fn ufshcd_hba_stop(hba: *mut ufs_hba);
}
extern "C" {
    pub fn ufshcd_schedule_eh_work(hba: *mut ufs_hba);
}
extern "C" {
    pub fn ufshcd_mcq_config_mac(hba: *mut ufs_hba, max_active_cmds: u32);
}
extern "C" {
    pub fn ufshcd_mcq_queue_cfg_addr(hba: *mut ufs_hba) -> c_uint;
}
extern "C" {
    pub fn ufshcd_mcq_read_cqis(hba: *mut ufs_hba, i: c_int) -> u32;
}
extern "C" {
    pub fn ufshcd_mcq_write_cqis(hba: *mut ufs_hba, val: u32, i: c_int);
}
extern "C" {
    pub fn ufshcd_mcq_make_queues_operational(hba: *mut ufs_hba);
}
extern "C" {
    pub fn ufshcd_mcq_enable(hba: *mut ufs_hba);
}
extern "C" {
    pub fn ufshcd_mcq_enable_esi(hba: *mut ufs_hba);
}
extern "C" {
    pub fn ufshcd_mcq_config_esi(hba: *mut ufs_hba, msg: *mut msi_msg);
}
//
// ufshcd_set_variant - set variant specific data to the hba
// @hba: per adapter instance
// @variant: pointer to variant specific data
//
// ufshcd_get_variant - get variant specific data from the hba
// @hba: per adapter instance
//
// Returns: pointer to the hba's private data area
//
extern "C" {
    pub fn ufshcd_runtime_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn ufshcd_runtime_resume(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn ufshcd_system_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn ufshcd_system_resume(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn ufshcd_system_freeze(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn ufshcd_system_thaw(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn ufshcd_system_restore(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn ufshcd_dme_reset(hba: *mut ufs_hba) -> c_int;
}
extern "C" {
    pub fn ufshcd_dme_enable(hba: *mut ufs_hba) -> c_int;
}
extern "C" {
    pub fn ufshcd_uic_change_pwr_mode(hba: *mut ufs_hba, mode: u8) -> c_int;
}
// UIC command interfaces for DME primitives
pub const DME_LOCAL: c_int = 0;
pub const DME_PEER: c_int = 1;

extern "C" {
    pub fn ufshcd_dme_get_attr(_arg: hba, _arg: attr_sel, _arg: mib_val, _arg: DME_LOCAL) -> return;
}
extern "C" {
    pub fn ufshcd_dme_get_attr(_arg: hba, _arg: attr_sel, _arg: mib_val, _arg: DME_PEER) -> return;
}
extern "C" {
    pub fn ufshcd_dme_set(_arg: hba, _arg: UIC_ARG_MIB(PA_LOCAL_TX_LCC_ENABLE), _arg: 0) -> return;
}
extern "C" {
    pub fn ufshcd_auto_hibern8_update(hba: *mut ufs_hba, ahit: u32);
}
extern "C" {
    pub fn ufshcd_hold(hba: *mut ufs_hba);
}
extern "C" {
    pub fn ufshcd_release(hba: *mut ufs_hba);
}
extern "C" {
    pub fn ufshcd_clkgate_delay_set(dev: *mut device, value: c_ulong);
}
extern "C" {
    pub fn ufshcd_get_vreg(dev: *mut device, vreg: *mut ufs_vreg) -> c_int;
}
extern "C" {
    pub fn ufshcd_send_uic_cmd(hba: *mut ufs_hba, uic_cmd: *mut uic_command) -> c_int;
}
extern "C" {
    pub fn ufshcd_wb_toggle(hba: *mut ufs_hba, enable: bool) -> c_int;
}
extern "C" {
    pub fn ufshcd_wb_toggle_buf_flush(hba: *mut ufs_hba, enable: bool) -> c_int;
}
extern "C" {
    pub fn ufshcd_wb_set_resize_en(hba: *mut ufs_hba, en_mode: wb_resize_en) -> c_int;
}
extern "C" {
    pub fn ufshcd_suspend_prepare(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn __ufshcd_suspend_prepare(dev: *mut device, rpm_ok_for_spm: bool) -> c_int;
}
extern "C" {
    pub fn ufshcd_resume_complete(dev: *mut device);
}
extern "C" {
    pub fn ufshcd_is_hba_active(hba: *mut ufs_hba) -> bool;
}
extern "C" {
    pub fn ufshcd_pm_qos_init(hba: *mut ufs_hba);
}
extern "C" {
    pub fn ufshcd_pm_qos_exit(hba: *mut ufs_hba);
}
extern "C" {
    pub fn ufshcd_dme_rmw(hba: *mut ufs_hba, mask: u32, val: u32, attr: u32) -> c_int;
}
// Wrapper functions for safely calling variant operations
extern "C" {
    pub fn __ufshcd_write_ee_control(hba: *mut ufs_hba, ee_ctrl_mask: u32) -> c_int;
}
extern "C" {
    pub fn ufshcd_write_ee_control(hba: *mut ufs_hba) -> c_int;
}
extern "C" {
    pub fn ufshcd_force_error_recovery(hba: *mut ufs_hba);
}
extern "C" {
    pub fn ufshcd_pm_qos_update(hba: *mut ufs_hba, on: bool);
}
extern "C" {
    pub fn ufshcd_us_to_ahit(timer: c_uint) -> u32;
}
