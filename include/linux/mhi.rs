//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mhi.h
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
// Copyright (c) 2018-2020, The Linux Foundation. All rights reserved.
//

pub const MHI_MAX_OEM_PK_HASH_SEGMENTS: c_int = 16;
//
// enum mhi_callback - MHI callback
// @MHI_CB_IDLE: MHI entered idle state
// @MHI_CB_PENDING_DATA: New data available for client to process
// @MHI_CB_LPM_ENTER: MHI host entered low power mode
// @MHI_CB_LPM_EXIT: MHI host about to exit low power mode
// @MHI_CB_EE_RDDM: MHI device entered RDDM exec env
// @MHI_CB_EE_MISSION_MODE: MHI device entered Mission Mode exec env
// @MHI_CB_SYS_ERROR: MHI device entered error state (may recover)
// @MHI_CB_FATAL_ERROR: MHI device entered fatal error state
// @MHI_CB_BW_REQ: Received a bandwidth switch request from device
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mhi_callback {
    MHI_CB_IDLE,
    MHI_CB_PENDING_DATA,
    MHI_CB_LPM_ENTER,
    MHI_CB_LPM_EXIT,
    MHI_CB_EE_RDDM,
    MHI_CB_EE_MISSION_MODE,
    MHI_CB_SYS_ERROR,
    MHI_CB_FATAL_ERROR,
    MHI_CB_BW_REQ,
}

//
// enum mhi_flags - Transfer flags
// @MHI_EOB: End of buffer for bulk transfer
// @MHI_EOT: End of transfer
// @MHI_CHAIN: Linked transfer
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mhi_flags {
    MHI_EOB = BIT(0),
    MHI_EOT = BIT(1),
    MHI_CHAIN = BIT(2),
}

//
// enum mhi_device_type - Device types
// @MHI_DEVICE_XFER: Handles data transfer
// @MHI_DEVICE_CONTROLLER: Control device
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mhi_device_type {
    MHI_DEVICE_XFER,
    MHI_DEVICE_CONTROLLER,
}

//
// enum mhi_ch_type - Channel types
// @MHI_CH_TYPE_INVALID: Invalid channel type
// @MHI_CH_TYPE_OUTBOUND: Outbound channel to the device
// @MHI_CH_TYPE_INBOUND: Inbound channel from the device
// @MHI_CH_TYPE_INBOUND_COALESCED: Coalesced channel for the device to combine
// multiple packets and send them as a single
// large packet to reduce CPU consumption
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mhi_ch_type {
    MHI_CH_TYPE_INVALID = 0,
    MHI_CH_TYPE_OUTBOUND = DMA_TO_DEVICE,
    MHI_CH_TYPE_INBOUND = DMA_FROM_DEVICE,
    MHI_CH_TYPE_INBOUND_COALESCED = 3,
}

//
// struct mhi_buf - MHI Buffer description
// @buf: Virtual address of the buffer
// @name: Buffer label. For offload channel, configurations name must be:
// ECA - Event context array data
// CCA - Channel context array data
// @dma_addr: IOMMU address of the buffer
// @len: # of bytes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhi_buf {
    pub buf: *mut c_void,
    pub name: *const c_char,
    pub dma_addr: dma_addr_t,
    pub len: usize,
}

//
// struct image_info - Firmware and RDDM table
// @mhi_buf: Buffer for firmware and RDDM table
// @entries: # of entries in table
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct image_info {
// private: from internal.h
    pub bhi_vec: *mut bhi_vec_entry,
// public:
    pub entries: u32,
    pub __counted_by(entries): mhi_buf mhi_buf[],
}

//
// struct mhi_link_info - BW requirement
// @target_link_speed: Link speed as defined by TLS bits in LinkControl reg
// @target_link_width: Link width as defined by NLW bits in LinkStatus reg
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhi_link_info {
    pub target_link_speed: c_uint,
    pub target_link_width: c_uint,
}

//
// enum mhi_ee_type - Execution environment types
// @MHI_EE_PBL: Primary Bootloader
// @MHI_EE_SBL: Secondary Bootloader
// @MHI_EE_AMSS: Modem, aka the primary runtime EE
// @MHI_EE_RDDM: Ram dump download mode
// @MHI_EE_WFW: WLAN firmware mode
// @MHI_EE_PTHRU: Passthrough
// @MHI_EE_EDL: Embedded downloader
// @MHI_EE_FP: Flash Programmer Environment
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mhi_ee_type {
    MHI_EE_PBL,
    MHI_EE_SBL,
    MHI_EE_AMSS,
    MHI_EE_RDDM,
    MHI_EE_WFW,
    MHI_EE_PTHRU,
    MHI_EE_EDL,
    MHI_EE_FP,
    MHI_EE_MAX_SUPPORTED = MHI_EE_FP,
    MHI_EE_DISABLE_TRANSITION, /* local EE, not related to mhi spec */
    MHI_EE_NOT_SUPPORTED,
    MHI_EE_MAX,
}

//
// enum mhi_state - MHI states
// @MHI_STATE_RESET: Reset state
// @MHI_STATE_READY: Ready state
// @MHI_STATE_M0: M0 state
// @MHI_STATE_M1: M1 state
// @MHI_STATE_M2: M2 state
// @MHI_STATE_M3: M3 state
// @MHI_STATE_M3_FAST: M3 Fast state
// @MHI_STATE_BHI: BHI state
// @MHI_STATE_SYS_ERR: System Error state
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mhi_state {
    MHI_STATE_RESET = 0x0,
    MHI_STATE_READY = 0x1,
    MHI_STATE_M0 = 0x2,
    MHI_STATE_M1 = 0x3,
    MHI_STATE_M2 = 0x4,
    MHI_STATE_M3 = 0x5,
    MHI_STATE_M3_FAST = 0x6,
    MHI_STATE_BHI = 0x7,
    MHI_STATE_SYS_ERR = 0xFF,
// private:
    MHI_STATE_MAX,
}

//
// enum mhi_ch_ee_mask - Execution environment mask for channel
// @MHI_CH_EE_PBL: Allow channel to be used in PBL EE
// @MHI_CH_EE_SBL: Allow channel to be used in SBL EE
// @MHI_CH_EE_AMSS: Allow channel to be used in AMSS EE
// @MHI_CH_EE_RDDM: Allow channel to be used in RDDM EE
// @MHI_CH_EE_PTHRU: Allow channel to be used in PTHRU EE
// @MHI_CH_EE_WFW: Allow channel to be used in WFW EE
// @MHI_CH_EE_EDL: Allow channel to be used in EDL EE
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mhi_ch_ee_mask {
    MHI_CH_EE_PBL = BIT(MHI_EE_PBL),
    MHI_CH_EE_SBL = BIT(MHI_EE_SBL),
    MHI_CH_EE_AMSS = BIT(MHI_EE_AMSS),
    MHI_CH_EE_RDDM = BIT(MHI_EE_RDDM),
    MHI_CH_EE_PTHRU = BIT(MHI_EE_PTHRU),
    MHI_CH_EE_WFW = BIT(MHI_EE_WFW),
    MHI_CH_EE_EDL = BIT(MHI_EE_EDL),
}

//
// enum mhi_er_data_type - Event ring data types
// @MHI_ER_DATA: Only client data over this ring
// @MHI_ER_CTRL: MHI control data and client data
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mhi_er_data_type {
    MHI_ER_DATA,
    MHI_ER_CTRL,
}

//
// enum mhi_db_brst_mode - Doorbell mode
// @MHI_DB_BRST_DISABLE: Burst mode disable
// @MHI_DB_BRST_ENABLE: Burst mode enable
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mhi_db_brst_mode {
    MHI_DB_BRST_DISABLE = 0x2,
    MHI_DB_BRST_ENABLE = 0x3,
}

//
// struct mhi_channel_config - Channel configuration structure for controller
// @name: The name of this channel
// @num: The number assigned to this channel
// @num_elements: The number of elements that can be queued to this channel
// @local_elements: The local ring length of the channel
// @event_ring: The event ring index that services this channel
// @dir: Direction that data may flow on this channel
// @type: Channel type
// @ee_mask: Execution Environment mask for this channel
// @pollcfg: Polling configuration for burst mode.  0 is default.  milliseconds
// for UL channels, multiple of 8 ring elements for DL channels
// @doorbell: Doorbell mode
// @lpm_notify: The channel master requires low power mode notifications
// @offload_channel: The client manages the channel completely
// @doorbell_mode_switch: Channel switches to doorbell mode on M0 transition
// @wake_capable: Channel capable of waking up the system
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhi_channel_config {
    pub name: *mut c_char,
    pub num: u32,
    pub num_elements: u32,
    pub local_elements: u32,
    pub event_ring: u32,
    pub dir: dma_data_direction,
    pub type: mhi_ch_type,
    pub ee_mask: u32,
    pub pollcfg: u32,
    pub doorbell: mhi_db_brst_mode,
    pub lpm_notify: bool,
    pub offload_channel: bool,
    pub doorbell_mode_switch: bool,
    pub wake_capable: bool,
}

//
// struct mhi_event_config - Event ring configuration structure for controller
// @num_elements: The number of elements that can be queued to this ring
// @irq_moderation_ms: Delay irq for additional events to be aggregated
// @irq: IRQ associated with this ring
// @channel: Dedicated channel number. U32_MAX indicates a non-dedicated ring
// @priority: Priority of this ring. Use 1 for now
// @mode: Doorbell mode
// @data_type: Type of data this ring will process
// @hardware_event: This ring is associated with hardware channels
// @client_managed: This ring is client managed
// @offload_channel: This ring is associated with an offloaded channel
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhi_event_config {
    pub num_elements: u32,
    pub irq_moderation_ms: u32,
    pub irq: u32,
    pub channel: u32,
    pub priority: u32,
    pub mode: mhi_db_brst_mode,
    pub data_type: mhi_er_data_type,
    pub hardware_event: bool,
    pub client_managed: bool,
    pub offload_channel: bool,
}

//
// struct mhi_controller_config - Root MHI controller configuration
// @max_channels: Maximum number of channels supported
// @timeout_ms: Timeout value for operations. 0 means use default
// @ready_timeout_ms: Timeout value for waiting device to be ready (optional)
// @buf_len: Size of automatically allocated buffers. 0 means use default
// @num_channels: Number of channels defined in @ch_cfg
// @ch_cfg: Array of defined channels
// @num_events: Number of event rings defined in @event_cfg
// @event_cfg: Array of defined event rings
// @use_bounce_buf: Use a bounce buffer pool due to limited DDR access
// @m2_no_db: Host is not allowed to ring DB in M2 state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhi_controller_config {
    pub max_channels: u32,
    pub timeout_ms: u32,
    pub ready_timeout_ms: u32,
    pub buf_len: u32,
    pub num_channels: u32,
    pub ch_cfg: *const mhi_channel_config,
    pub num_events: u32,
    pub event_cfg: *mut mhi_event_config,
    pub use_bounce_buf: bool,
    pub m2_no_db: bool,
}

//
// struct mhi_controller - Master MHI controller structure
// @name: Device name of the MHI controller
// @cntrl_dev: Pointer to the struct device of physical bus acting as the MHI
// controller (required)
// @mhi_dev: MHI device instance for the controller
// @debugfs_dentry: MHI controller debugfs directory
// @regs: Base address of MHI MMIO register space (required)
// @bhi: Points to base of MHI BHI register space
// @bhie: Points to base of MHI BHIe register space
// @wake_db: MHI WAKE doorbell register address
// @iova_start: IOMMU starting address for data (required)
// @iova_stop: IOMMU stop address for data (required)
// @fw_image: Firmware image name for normal booting (optional)
// @fw_data: Firmware image data content for normal booting, used only
// if fw_image is NULL and fbc_download is true (optional)
// @fw_sz: Firmware image data size for normal booting, used only if fw_image
// is NULL and fbc_download is true (optional)
// @edl_image: Firmware image name for emergency download mode (optional)
// @rddm_size: RAM dump size that host should allocate for debugging purpose
// @sbl_size: SBL image size downloaded through BHIe (optional)
// @seg_len: BHIe vector size (optional)
// @reg_len: Length of the MHI MMIO region (required)
// @fbc_image: Points to firmware image buffer
// @rddm_image: Points to RAM dump buffer
// @mhi_chan: Points to the channel configuration table
// @lpm_chans: List of channels that require LPM notifications
// @irq: base irq # to request (required)
// @max_chan: Maximum number of channels the controller supports
// @total_ev_rings: Total # of event rings allocated
// @hw_ev_rings: Number of hardware event rings
// @sw_ev_rings: Number of software event rings
// @nr_irqs: Number of IRQ allocated by bus master (required)
// @serial_number: MHI controller serial number obtained from BHI
// @mhi_event: MHI event ring configurations table
// @mhi_cmd: MHI command ring configurations table
// @mhi_ctxt: MHI device context, shared memory between host and device
// @pm_mutex: Mutex for suspend/resume operation
// @pm_lock: Lock for protecting MHI power management state
// @timeout_ms: Timeout in ms for state transitions
// @ready_timeout_ms: Timeout in ms for waiting device to be ready (optional)
// @pm_state: MHI power management state
// @db_access: DB access states
// @ee: MHI device execution environment
// @dev_state: MHI device state
// @dev_wake: Device wakeup count
// @pending_pkts: Pending packets for the controller
// @M0: Counter to track number of device MHI state changes
// @M2: Counter to track number of device MHI state changes
// @M3: Counter to track number of device MHI state changes
// @transition_list: List of MHI state transitions
// @transition_lock: Lock for protecting MHI state transition list
// @wlock: Lock for protecting device wakeup
// @mhi_link_info: Device bandwidth info
// @st_worker: State transition worker
// @hiprio_wq: High priority workqueue for MHI work such as state transitions
// @state_event: State change event
// @status_cb: CB function to notify power states of the device (required)
// @wake_get: CB function to assert device wake (optional)
// @wake_put: CB function to de-assert device wake (optional)
// @wake_toggle: CB function to assert and de-assert device wake (optional)
// @runtime_get: CB function to controller runtime resume (required)
// @runtime_put: CB function to decrement pm usage (required)
// @map_single: CB function to create TRE buffer
// @unmap_single: CB function to destroy TRE buffer
// @read_reg: Read a MHI register via the physical link (required)
// @write_reg: Write a MHI register via the physical link (required)
// @reset: Controller specific reset function (optional)
// @edl_trigger: CB function to trigger EDL mode (optional)
// @buffer_len: Bounce buffer length
// @index: Index of the MHI controller instance
// @bounce_buf: Use of bounce buffer
// @fbc_download: MHI host needs to do complete image transfer (optional)
// @wake_set: Device wakeup set flag
// @no_m3: Device doesn't support M3 state
// @irq_flags: irq flags passed to request_irq (optional)
// @mru: the default MRU for the MHI device
//
// Fields marked as (required) need to be populated by the controller driver
// before calling mhi_register_controller(). For the fields marked as (optional)
// they can be populated depending on the usecase.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhi_controller {
    pub name: *const c_char,
    pub cntrl_dev: *mut device,
    pub mhi_dev: *mut mhi_device,
    pub debugfs_dentry: *mut dentry,
    pub regs: *mut void __iomem,
    pub bhi: *mut void __iomem,
    pub bhie: *mut void __iomem,
    pub wake_db: *mut void __iomem,
    pub iova_start: dma_addr_t,
    pub iova_stop: dma_addr_t,
    pub fw_image: *const c_char,
    pub fw_data: *const u8,
    pub fw_sz: usize,
    pub edl_image: *const c_char,
    pub rddm_size: usize,
    pub sbl_size: usize,
    pub seg_len: usize,
    pub reg_len: usize,
    pub fbc_image: *mut image_info,
    pub rddm_image: *mut image_info,
    pub mhi_chan: *mut mhi_chan,
    pub lpm_chans: list_head,
    pub irq: *mut c_int,
    pub max_chan: u32,
    pub total_ev_rings: u32,
    pub hw_ev_rings: u32,
    pub sw_ev_rings: u32,
    pub nr_irqs: u32,
    pub serial_number: u32,
    pub mhi_event: *mut mhi_event,
    pub mhi_cmd: *mut mhi_cmd,
    pub mhi_ctxt: *mut mhi_ctxt,
    pub pm_mutex: mutex,
    pub pm_lock: rwlock_t,
    pub timeout_ms: u32,
    pub ready_timeout_ms: u32,
    pub pm_state: u32,
    pub db_access: u32,
    pub ee: mhi_ee_type,
    pub dev_state: mhi_state,
    pub dev_wake: core::sync::atomic::AtomicI32,
    pub pending_pkts: core::sync::atomic::AtomicI32,
    pub M3: u32 M0, M2,,
    pub transition_list: list_head,
    pub transition_lock: spinlock_t,
    pub wlock: spinlock_t,
    pub mhi_link_info: mhi_link_info,
    pub st_worker: work_struct,
    pub hiprio_wq: *mut workqueue_struct,
    pub state_event: wait_queue_head_t,
    pub cb): mhi_callback,
    pub override): *mut *mut *mut void (wake_get)(struct mhi_controller mhi_cntrl, bool,
    pub override): *mut *mut *mut void (wake_put)(struct mhi_controller mhi_cntrl, bool,
    pub mhi_cntrl): *mut *mut void (wake_toggle)(struct mhi_controller,
    pub mhi_cntrl): *mut *mut int (runtime_get)(struct mhi_controller,
    pub mhi_cntrl): *mut *mut void (runtime_put)(struct mhi_controller,
    pub buf): *mut mhi_buf_info,
    pub buf): *mut mhi_buf_info,
    pub out): *mut u32,
    pub val): u32,
    pub mhi_cntrl): *mut *mut void (reset)(struct mhi_controller,
    pub mhi_cntrl): *mut *mut int (edl_trigger)(struct mhi_controller,
    pub buffer_len: usize,
    pub index: c_int,
    pub bounce_buf: bool,
    pub fbc_download: bool,
    pub wake_set: bool,
    pub no_m3: bool,
    pub irq_flags: c_ulong,
    pub mru: u32,
}

//
// struct mhi_device - Structure representing an MHI device which binds
// to channels or is associated with controllers
// @id: Pointer to MHI device ID struct
// @name: Name of the associated MHI device
// @mhi_cntrl: Controller the device belongs to
// @ul_chan: UL channel for the device
// @dl_chan: DL channel for the device
// @dev: Driver model device node for the MHI device
// @dev_type: MHI device type
// @ul_chan_id: MHI channel id for UL transfer
// @dl_chan_id: MHI channel id for DL transfer
// @dev_wake: Device wakeup counter
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhi_device {
    pub id: *const mhi_device_id,
    pub name: *const c_char,
    pub mhi_cntrl: *mut mhi_controller,
    pub ul_chan: *mut mhi_chan,
    pub dl_chan: *mut mhi_chan,
    pub dev: device,
    pub dev_type: mhi_device_type,
    pub ul_chan_id: c_int,
    pub dl_chan_id: c_int,
    pub dev_wake: u32,
}

//
// struct mhi_result - Completed buffer information
// @buf_addr: Address of data buffer
// @bytes_xferd: # of bytes transferred
// @dir: Channel direction
// @transaction_status: Status of last transaction
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhi_result {
    pub buf_addr: *mut c_void,
    pub bytes_xferd: usize,
    pub dir: dma_data_direction,
    pub transaction_status: c_int,
}

//
// struct mhi_driver - Structure representing a MHI client driver
// @id_table: table of MHI channel names that a driver supports
// @probe: CB function for client driver probe function
// @remove: CB function for client driver remove function
// @ul_xfer_cb: CB function for UL data transfer
// @dl_xfer_cb: CB function for DL data transfer
// @status_cb: CB functions for asynchronous status
// @driver: Device driver model driver
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhi_driver {
    pub id_table: *const mhi_device_id,
    pub id): *const mhi_device_id,
    pub mhi_dev): *mut *mut void (remove)(struct mhi_device,
    pub result): *mut mhi_result,
    pub result): *mut mhi_result,
    pub mhi_cb): *mut *mut *mut void (status_cb)(struct mhi_device mhi_dev, enum mhi_callback,
    pub driver: device_driver,
}

//
// mhi_alloc_controller - Allocate the MHI Controller structure
// Allocate the mhi_controller structure using zero initialized memory
//
// mhi_free_controller - Free the MHI Controller structure
// @mhi_cntrl: MHI controller to free
// Free the mhi_controller structure which was previously allocated
//
extern "C" {
    pub fn mhi_free_controller(mhi_cntrl: *mut mhi_controller);
}
//
// mhi_register_controller - Register MHI controller
// @mhi_cntrl: MHI controller to register
// @config: Configuration to use for the controller
//
// mhi_unregister_controller - Unregister MHI controller
// @mhi_cntrl: MHI controller to unregister
//
extern "C" {
    pub fn mhi_unregister_controller(mhi_cntrl: *mut mhi_controller);
}
//
// module_mhi_driver() - Helper macro for drivers that don't do
// anything special other than using default mhi_driver_register() and
// mhi_driver_unregister().  This eliminates a lot of boilerplate.
// Each module may only use this macro once.
//

//
// Macro to avoid include chaining to get THIS_MODULE
//

//
// __mhi_driver_register - Register driver with MHI framework
// @mhi_drv: Driver associated with the device
// @owner: The module owner
//
extern "C" {
    pub fn __mhi_driver_register(mhi_drv: *mut mhi_driver, owner: *mut module) -> c_int;
}
//
// mhi_driver_unregister - Unregister a driver for mhi_devices
// @mhi_drv: Driver associated with the device
//
extern "C" {
    pub fn mhi_driver_unregister(mhi_drv: *mut mhi_driver);
}
//
// mhi_set_mhi_state - Set MHI device state
// @mhi_cntrl: MHI controller
// @state: State to set
//
// mhi_notify - Notify the MHI client driver about client device status
// @mhi_dev: MHI device instance
// @cb_reason: MHI callback reason
//
extern "C" {
    pub fn mhi_notify(mhi_dev: *mut mhi_device, cb_reason: mhi_callback);
}
//
// mhi_get_free_desc_count - Get transfer ring length
// Get # of TD available to queue buffers
// @mhi_dev: Device associated with the channels
// @dir: Direction of the channel
//
// mhi_prepare_for_power_up - Do pre-initialization before power up.
// This is optional, call this before power up if
// the controller does not want bus framework to
// automatically free any allocated memory during
// shutdown process.
// @mhi_cntrl: MHI controller
//
extern "C" {
    pub fn mhi_prepare_for_power_up(mhi_cntrl: *mut mhi_controller) -> c_int;
}
//
// mhi_async_power_up - Start MHI power up sequence
// @mhi_cntrl: MHI controller
//
extern "C" {
    pub fn mhi_async_power_up(mhi_cntrl: *mut mhi_controller) -> c_int;
}
//
// mhi_sync_power_up - Start MHI power up sequence and wait till the device
// enters valid EE state
// @mhi_cntrl: MHI controller
//
extern "C" {
    pub fn mhi_sync_power_up(mhi_cntrl: *mut mhi_controller) -> c_int;
}
//
// mhi_power_down - Power down the MHI device and also destroy the
// 'struct device' for the channels associated with it.
// See also mhi_power_down_keep_dev() which is a variant
// of this API that keeps the 'struct device' for channels
// (useful during suspend/hibernation).
// @mhi_cntrl: MHI controller
// @graceful: Link is still accessible, so do a graceful shutdown process
//
extern "C" {
    pub fn mhi_power_down(mhi_cntrl: *mut mhi_controller, graceful: bool);
}
//
// mhi_power_down_keep_dev - Power down the MHI device but keep the 'struct
// device' for the channels associated with it.
// This is a variant of 'mhi_power_down()' and
// useful in scenarios such as suspend/hibernation
// where destroying of the 'struct device' is not
// needed.
// @mhi_cntrl: MHI controller
// @graceful: Link is still accessible, so do a graceful shutdown process
//
extern "C" {
    pub fn mhi_power_down_keep_dev(mhi_cntrl: *mut mhi_controller, graceful: bool);
}
//
// mhi_unprepare_after_power_down - Free any allocated memory after power down
// @mhi_cntrl: MHI controller
//
extern "C" {
    pub fn mhi_unprepare_after_power_down(mhi_cntrl: *mut mhi_controller);
}
//
// mhi_pm_suspend - Move MHI into a suspended state
// @mhi_cntrl: MHI controller
//
extern "C" {
    pub fn mhi_pm_suspend(mhi_cntrl: *mut mhi_controller) -> c_int;
}
//
// mhi_pm_resume - Resume MHI from suspended state
// @mhi_cntrl: MHI controller
//
extern "C" {
    pub fn mhi_pm_resume(mhi_cntrl: *mut mhi_controller) -> c_int;
}
//
// mhi_pm_resume_force - Force resume MHI from suspended state
// @mhi_cntrl: MHI controller
//
// Resume the device irrespective of its MHI state. As per the MHI spec, devices
// has to be in M3 state during resume. But some devices seem to be in a
// different MHI state other than M3 but they continue working fine if allowed.
// This API is intented to be used for such devices.
//
// Return: 0 if the resume succeeds, a negative error code otherwise
//
extern "C" {
    pub fn mhi_pm_resume_force(mhi_cntrl: *mut mhi_controller) -> c_int;
}
//
// mhi_download_rddm_image - Download ramdump image from device for
// debugging purpose.
// @mhi_cntrl: MHI controller
// @in_panic: Download rddm image during kernel panic
//
extern "C" {
    pub fn mhi_download_rddm_image(mhi_cntrl: *mut mhi_controller, in_panic: bool) -> c_int;
}
//
// mhi_force_rddm_mode - Force device into rddm mode
// @mhi_cntrl: MHI controller
//
extern "C" {
    pub fn mhi_force_rddm_mode(mhi_cntrl: *mut mhi_controller) -> c_int;
}
//
// mhi_get_exec_env - Get BHI execution environment of the device
// @mhi_cntrl: MHI controller
//
extern "C" {
    pub fn mhi_get_exec_env(mhi_cntrl: *mut mhi_controller) -> mhi_ee_type;
}
//
// mhi_get_mhi_state - Get MHI state of the device
// @mhi_cntrl: MHI controller
//
extern "C" {
    pub fn mhi_get_mhi_state(mhi_cntrl: *mut mhi_controller) -> mhi_state;
}
//
// mhi_soc_reset - Trigger a device reset. This can be used as a last resort
// to reset and recover a device.
// @mhi_cntrl: MHI controller
//
extern "C" {
    pub fn mhi_soc_reset(mhi_cntrl: *mut mhi_controller);
}
//
// mhi_device_get_sync - Disable device low power mode. Synchronously
// take the controller out of suspended state
// @mhi_dev: Device associated with the channel
//
extern "C" {
    pub fn mhi_device_get_sync(mhi_dev: *mut mhi_device) -> c_int;
}
//
// mhi_device_put - Re-enable device low power mode
// @mhi_dev: Device associated with the channel
//
extern "C" {
    pub fn mhi_device_put(mhi_dev: *mut mhi_device);
}
//
// mhi_prepare_for_transfer - Setup UL and DL channels for data transfer.
// @mhi_dev: Device associated with the channels
//
// Allocate and initialize the channel context and also issue the START channel
// command to both channels. Channels can be started only if both host and
// device execution environments match and channels are in a DISABLED state.
//
extern "C" {
    pub fn mhi_prepare_for_transfer(mhi_dev: *mut mhi_device) -> c_int;
}
//
// mhi_unprepare_from_transfer - Reset UL and DL channels for data transfer.
// Issue the RESET channel command and let the
// device clean-up the context so no incoming
// transfers are seen on the host. Free memory
// associated with the context on host. If device
// is unresponsive, only perform a host side
// clean-up. Channels can be reset only if both
// host and device execution environments match
// and channels are in an ENABLED, STOPPED or
// SUSPENDED state.
// @mhi_dev: Device associated with the channels
//
extern "C" {
    pub fn mhi_unprepare_from_transfer(mhi_dev: *mut mhi_device);
}
//
// mhi_queue_buf - Send or receive raw buffers from client device over MHI
// channel
// @mhi_dev: Device associated with the channels
// @dir: DMA direction for the channel
// @buf: Buffer for holding the data
// @len: Buffer length
// @mflags: MHI transfer flags used for the transfer
//
// mhi_queue_skb - Send or receive SKBs from client device over MHI channel
// @mhi_dev: Device associated with the channels
// @dir: DMA direction for the channel
// @skb: Buffer for holding SKBs
// @len: Buffer length
// @mflags: MHI transfer flags used for the transfer
//
// mhi_queue_is_full - Determine whether queueing new elements is possible
// @mhi_dev: Device associated with the channels
// @dir: DMA direction for the channel
//
extern "C" {
    pub fn mhi_queue_is_full(mhi_dev: *mut mhi_device, dir: dma_data_direction) -> bool;
}
//
// mhi_get_channel_doorbell_offset - Get the channel doorbell offset
// @mhi_cntrl: MHI controller
// @chdb_offset: Read channel doorbell offset
//
// Return: 0 if the read succeeds, a negative error code otherwise
//
extern "C" {
    pub fn mhi_get_channel_doorbell_offset(mhi_cntrl: *mut mhi_controller, chdb_offset: *mut u32) -> c_int;
}
