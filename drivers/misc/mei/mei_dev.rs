//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/mei/mei_dev.h
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
// Copyright (c) 2003-2022, Intel Corporation. All rights reserved.
// Intel Management Engine Interface (Intel MEI) Linux driver
//

extern "C" {
    pub fn memcmp(_arg: &u1, _arg: &u2, _arg: sizeof(uuid_le)) -> return;
}

//
// Number of Maximum MEI Clients
//
pub const MEI_CLIENTS_MAX: c_int = 256;
//
// maximum number of consecutive resets
//
pub const MEI_MAX_CONSEC_RESET: c_int = 3;
//
// Number of File descriptors/handles
// that can be opened to the driver.
//
// Limit to 255: 256 Total Clients
// minus internal client for MEI Bus Messages
//

// File state
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum file_state {
    MEI_FILE_UNINITIALIZED = 0,
    MEI_FILE_INITIALIZING,
    MEI_FILE_CONNECTING,
    MEI_FILE_CONNECTED,
    MEI_FILE_DISCONNECTING,
    MEI_FILE_DISCONNECT_REPLY,
    MEI_FILE_DISCONNECT_REQUIRED,
    MEI_FILE_DISCONNECTED,
}

// MEI device states
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mei_dev_state {
    MEI_DEV_UNINITIALIZED = 0,
    MEI_DEV_INITIALIZING,
    MEI_DEV_INIT_CLIENTS,
    MEI_DEV_ENABLED,
    MEI_DEV_RESETTING,
    MEI_DEV_DISABLED,
    MEI_DEV_POWERING_DOWN,
    MEI_DEV_POWER_DOWN,
    MEI_DEV_POWER_UP
}

//
// enum mei_dev_pxp_mode - MEI PXP mode state
//
// @MEI_DEV_PXP_DEFAULT: PCH based device, no initialization required
// @MEI_DEV_PXP_INIT:    device requires initialization, send setup message to firmware
// @MEI_DEV_PXP_SETUP:   device is in setup stage, waiting for firmware response
// @MEI_DEV_PXP_READY:   device initialized
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mei_dev_pxp_mode {
    MEI_DEV_PXP_DEFAULT = 0,
    MEI_DEV_PXP_INIT    = 1,
    MEI_DEV_PXP_SETUP   = 2,
    MEI_DEV_PXP_READY   = 3,
}

//
// enum mei_dev_reset_to_pxp - reset to PXP mode performed
//
// @MEI_DEV_RESET_TO_PXP_DEFAULT: before reset
// @MEI_DEV_RESET_TO_PXP_PERFORMED: reset performed
// @MEI_DEV_RESET_TO_PXP_DONE: reset processed
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mei_dev_reset_to_pxp {
    MEI_DEV_RESET_TO_PXP_DEFAULT = 0,
    MEI_DEV_RESET_TO_PXP_PERFORMED = 1,
    MEI_DEV_RESET_TO_PXP_DONE = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mei_file_transaction_states {
    MEI_IDLE,
    MEI_WRITING,
    MEI_WRITE_COMPLETE,
}

//
// enum mei_cb_file_ops  - file operation associated with the callback
// @MEI_FOP_READ:       read
// @MEI_FOP_WRITE:      write
// @MEI_FOP_CONNECT:    connect
// @MEI_FOP_DISCONNECT: disconnect
// @MEI_FOP_DISCONNECT_RSP: disconnect response
// @MEI_FOP_NOTIFY_START:   start notification
// @MEI_FOP_NOTIFY_STOP:    stop notification
// @MEI_FOP_DMA_MAP:   request client dma map
// @MEI_FOP_DMA_UNMAP: request client dma unmap
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mei_cb_file_ops {
    MEI_FOP_READ = 0,
    MEI_FOP_WRITE,
    MEI_FOP_CONNECT,
    MEI_FOP_DISCONNECT,
    MEI_FOP_DISCONNECT_RSP,
    MEI_FOP_NOTIFY_START,
    MEI_FOP_NOTIFY_STOP,
    MEI_FOP_DMA_MAP,
    MEI_FOP_DMA_UNMAP,
}

//
// enum mei_cl_io_mode - io mode between driver and fw
//
// @MEI_CL_IO_TX_BLOCKING: send is blocking
// @MEI_CL_IO_TX_INTERNAL: internal communication between driver and FW
//
// @MEI_CL_IO_RX_NONBLOCK: recv is non-blocking
//
// @MEI_CL_IO_SGL: send command with sgl list.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mei_cl_io_mode {
    MEI_CL_IO_TX_BLOCKING = BIT(0),
    MEI_CL_IO_TX_INTERNAL = BIT(1),

    MEI_CL_IO_RX_NONBLOCK = BIT(2),

    MEI_CL_IO_SGL         = BIT(3),
}

//
// Intel MEI message data struct
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_msg_data {
    pub size: usize,
    pub data: *mut c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_dma_data {
    pub buffer_id: u8,
    pub vaddr: *mut c_void,
    pub daddr: dma_addr_t,
    pub size: usize,
}

//
// struct mei_dma_dscr - dma address descriptor
//
// @vaddr: dma buffer virtual address
// @daddr: dma buffer physical address
// @size : dma buffer size
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_dma_dscr {
    pub vaddr: *mut c_void,
    pub daddr: dma_addr_t,
    pub size: usize,
}

// Maximum number of processed FW status registers
pub const MEI_FW_STATUS_MAX: c_int = 6;
// Minimal  buffer for FW status string (8 bytes in dw + space or '\0')

//
// struct mei_fw_status - storage of FW status data
//
// @count: number of actually available elements in array
// @status: FW status registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_fw_status {
    pub count: c_int,
    pub status: [u32; MEI_FW_STATUS_MAX],
}

//
// struct mei_me_client - representation of me (fw) client
//
// @list: link in me client list
// @refcnt: struct reference count
// @props: client properties
// @client_id: me client id
// @tx_flow_ctrl_creds: flow control credits
// @connect_count: number connections to this client
// @bus_added: added to bus
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_me_client {
    pub list: list_head,
    pub refcnt: kref,
    pub props: mei_client_properties,
    pub client_id: u8,
    pub tx_flow_ctrl_creds: u8,
    pub connect_count: u8,
    pub bus_added: u8,
}

//
// struct mei_cl_cb - file operation callback structure
//
// @list: link in callback queue
// @cl: file client who is running this operation
// @fop_type: file operation type
// @buf: buffer for data associated with the callback
// @buf_idx: last read index
// @vtag: virtual tag
// @fp: pointer to file structure
// @status: io status of the cb
// @internal: communication between driver and FW flag
// @blocking: transmission blocking mode
// @ext_hdr: extended header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_cl_cb {
    pub list: list_head,
    pub cl: *mut mei_cl,
    pub fop_type: mei_cb_file_ops,
    pub buf: mei_msg_data,
    pub buf_idx: usize,
    pub vtag: u8,
    pub fp: *const file,
    pub status: c_int,
    pub internal:1: u32,
    pub blocking:1: u32,
    pub ext_hdr: *mut mei_ext_hdr,
}

//
// struct mei_cl_vtag - file pointer to vtag mapping structure
//
// @list: link in map queue
// @fp: file pointer
// @vtag: corresponding vtag
// @pending_read: the read is pending on this file
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_cl_vtag {
    pub list: list_head,
    pub fp: *const file,
    pub vtag: u8,
    pub pending_read:1: u8,
}

//
// struct mei_cl - me client host representation
// carried in file->private_data
//
// @link: link in the clients list
// @dev: mei parent device
// @state: file operation state
// @tx_wait: wait queue for tx completion
// @rx_wait: wait queue for rx completion
// @wait:  wait queue for management operation
// @ev_wait: notification wait queue
// @ev_async: event async notification
// @status: connection status
// @me_cl: fw client connected
// @fp: file associated with client
// @host_client_id: host id
// @vtag_map: vtag map
// @tx_flow_ctrl_creds: transmit flow credentials
// @rx_flow_ctrl_creds: receive flow credentials
// @timer_count:  watchdog timer for operation completion
// @notify_en: notification - enabled/disabled
// @notify_ev: pending notification event
// @tx_cb_queued: number of tx callbacks in queue
// @writing_state: state of the tx
// @rd_pending: pending read credits
// @rd_completed_lock: protects rd_completed queue
// @rd_completed: completed read
// @dma: dma settings
// @dma_mapped: dma buffer is currently mapped.
//
// @cldev: device on the mei client bus
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_cl {
    pub link: list_head,
    pub dev: *mut mei_device,
    pub state: file_state,
    pub tx_wait: wait_queue_head_t,
    pub rx_wait: wait_queue_head_t,
    pub wait: wait_queue_head_t,
    pub ev_wait: wait_queue_head_t,
    pub ev_async: *mut fasync_struct,
    pub status: c_int,
    pub me_cl: *mut mei_me_client,
    pub fp: *const file,
    pub host_client_id: u8,
    pub vtag_map: list_head,
    pub tx_flow_ctrl_creds: u8,
    pub rx_flow_ctrl_creds: u8,
    pub timer_count: u8,
    pub notify_en: u8,
    pub notify_ev: u8,
    pub tx_cb_queued: u8,
    pub writing_state: mei_file_transaction_states,
    pub rd_pending: list_head,
    pub /: *mut *mut spinlock_t rd_completed_lock; / protects rd_completed queue,
    pub rd_completed: list_head,
    pub dma: mei_dma_data,
    pub dma_mapped: u8,
    pub cldev: *mut mei_cl_device,
}

pub const MEI_TX_QUEUE_LIMIT_DEFAULT: c_int = 50;
pub const MEI_TX_QUEUE_LIMIT_MAX: c_int = 255;
pub const MEI_TX_QUEUE_LIMIT_MIN: c_int = 30;
//
// struct mei_hw_ops - hw specific ops
//
// @host_is_ready    : query for host readiness
//
// @hw_is_ready      : query if hw is ready
// @hw_reset         : reset hw
// @hw_start         : start hw after reset
// @hw_config        : configure hw
//
// @fw_status        : get fw status registers
// @trc_status       : get trc status register
// @pg_state         : power gating state of the device
// @pg_in_transition : is device now in pg transition
// @pg_is_enabled    : is power gating enabled
//
// @intr_clear       : clear pending interrupts
// @intr_enable      : enable interrupts
// @intr_disable     : disable interrupts
// @synchronize_irq  : synchronize irqs
//
// @hbuf_free_slots  : query for write buffer empty slots
// @hbuf_is_ready    : query if write buffer is empty
// @hbuf_depth       : query for write buffer depth
//
// @write            : write a message to FW
//
// @rdbuf_full_slots : query how many slots are filled
//
// @read_hdr         : get first 4 bytes (header)
// @read             : read a buffer from the FW
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_hw_ops {
    pub dev): *mut *mut bool (host_is_ready)(struct mei_device,
    pub dev): *mut *mut bool (hw_is_ready)(struct mei_device,
    pub enable): *mut *mut *mut int (hw_reset)(struct mei_device dev, bool,
    pub dev): *mut *mut int (hw_start)(struct mei_device,
    pub dev): *mut *mut int (hw_config)(struct mei_device,
    pub fw_sts): *mut *mut *mut int (fw_status)(struct mei_device dev, struct mei_fw_status,
    pub trc): *mut *mut *mut int (trc_status)(struct mei_device dev, u32,
    pub dev): *mut *mut mei_pg_state (pg_state)(struct mei_device,
    pub dev): *mut *mut bool (pg_in_transition)(struct mei_device,
    pub dev): *mut *mut bool (pg_is_enabled)(struct mei_device,
    pub dev): *mut *mut void (intr_clear)(struct mei_device,
    pub dev): *mut *mut void (intr_enable)(struct mei_device,
    pub dev): *mut *mut void (intr_disable)(struct mei_device,
    pub dev): *mut *mut void (synchronize_irq)(struct mei_device,
    pub dev): *mut *mut int (hbuf_free_slots)(struct mei_device,
    pub dev): *mut *mut bool (hbuf_is_ready)(struct mei_device,
    pub dev): *const *const u32 (hbuf_depth)(struct mei_device,
    pub data_len): *const *const void data, size_t,
    pub dev): *mut *mut int (rdbuf_full_slots)(struct mei_device,
    pub dev): *const *const u32 (read_hdr)(struct mei_device,
    pub len): *mut *mut unsigned char buf, unsigned long,
}

// MEI bus API
extern "C" {
    pub fn mei_cl_bus_rescan_work(work: *mut work_struct);
}
extern "C" {
    pub fn mei_cl_bus_dev_fixup(dev: *mut mei_cl_device);
}
extern "C" {
    pub fn mei_cl_bus_rx_event(cl: *mut mei_cl) -> bool;
}
extern "C" {
    pub fn mei_cl_bus_notify_event(cl: *mut mei_cl) -> bool;
}
extern "C" {
    pub fn mei_cl_bus_remove_devices(bus: *mut mei_device);
}
extern "C" {
    pub fn mei_cl_bus_init() -> c_int;
}
extern "C" {
    pub fn mei_cl_bus_exit();
}
//
// enum mei_pg_event - power gating transition events
//
// @MEI_PG_EVENT_IDLE: the driver is not in power gating transition
// @MEI_PG_EVENT_WAIT: the driver is waiting for a pg event to complete
// @MEI_PG_EVENT_RECEIVED: the driver received pg event
// @MEI_PG_EVENT_INTR_WAIT: the driver is waiting for a pg event interrupt
// @MEI_PG_EVENT_INTR_RECEIVED: the driver received pg event interrupt
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mei_pg_event {
    MEI_PG_EVENT_IDLE,
    MEI_PG_EVENT_WAIT,
    MEI_PG_EVENT_RECEIVED,
    MEI_PG_EVENT_INTR_WAIT,
    MEI_PG_EVENT_INTR_RECEIVED,
}

//
// enum mei_pg_state - device internal power gating state
//
// @MEI_PG_OFF: device is not power gated - it is active
// @MEI_PG_ON:  device is power gated - it is in lower power state
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mei_pg_state {
    MEI_PG_OFF = 0,
    MEI_PG_ON =  1,
}

//
// struct mei_fw_version - MEI FW version struct
//
// @platform: platform identifier
// @major: major version field
// @minor: minor version field
// @buildno: build number version field
// @hotfix: hotfix number version field
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_fw_version {
    pub platform: u8,
    pub major: u8,
    pub minor: u16,
    pub buildno: u16,
    pub hotfix: u16,
}

pub const MEI_MAX_FW_VER_BLOCKS: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_dev_timeouts {
    pub /: *mut *mut unsigned long hw_ready; / Timeout on ready message, in jiffies,
    pub /: *mut *mut int connect; / HPS: at least 2 seconds, in seconds,
    pub /: *mut *mut unsigned long cl_connect; / HPS: Client Connect Timeout, in jiffies,
    pub /: *mut *mut int client_init; / HPS: Clients Enumeration Timeout, in seconds,
    pub /: *mut *mut unsigned long pgi; / PG Isolation time response, in jiffies,
    pub /: *mut *mut unsigned int d0i3; / D0i3 set/unset max response time, in jiffies,
    pub /: *mut *mut unsigned long hbm; / HBM operation timeout, in jiffies,
    pub /: *mut *mut unsigned long mkhi_recv; / receive timeout, in jiffies,
    pub /: *mut *mut unsigned long link_reset_wait; / link reset wait timeout, in jiffies,
}

//
// enum mei_dev_kind - device type
//
// @MEI_DEV_KIND_MEI: basic device
// @MEI_DEV_KIND_ITOUCH: itouch support
// @MEI_DEV_KIND_GSC: discete graphics content protection
// @MEI_DEV_KIND_GSCFI: discete graphics chassis controller
// @MEI_DEV_KIND_IVSC: visual sensing controller
// @MEI_DEV_KIND_IOE: IO extender
// @MEI_DEV_KIND_MAX: sentinel
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mei_dev_kind {
    MEI_DEV_KIND_MEI,
    MEI_DEV_KIND_ITOUCH,
    MEI_DEV_KIND_GSC,
    MEI_DEV_KIND_GSCFI,
    MEI_DEV_KIND_IVSC,
    MEI_DEV_KIND_IOE,
    MEI_DEV_KIND_MAX
}

//
// struct mei_device -  MEI private device struct
//
// @parent      : device on a bus
// @dev         : device object
// @cdev        : character device pointer
// @minor       : minor number allocated for device
//
// @write_list  : write pending list
// @write_waiting_list : write completion list
// @ctrl_wr_list : pending control write list
// @ctrl_rd_list : pending control read list
// @tx_queue_limit: tx queues per client linit
//
// @file_list   : list of opened handles
// @open_handle_count: number of opened handles
//
// @device_lock : big device lock
// @timer_work  : MEI timer delayed work (timeouts)
//
// @recvd_hw_ready : hw ready message received flag
// @pg_blocked  : low power mode is not allowed
// @read_fws_need_resume: the FW status handler needs HW woken from sleep
//
// @wait_hw_ready : wait queue for receive HW ready message form FW
// @wait_pg     : wait queue for receive PG message from FW
// @wait_hbm_start : wait queue for receive HBM start message from FW
//
// @reset_count : number of consecutive resets
// @dev_state   : device state
// @wait_dev_state: wait queue for device state change
// @hbm_state   : state of host bus message protocol
// @pxp_mode    : PXP device mode
// @init_clients_timer : HBM init handshake timeout
//
// @pg_event    : power gating event
// @pg_domain   : runtime PM domain
//
// @rd_msg_buf  : control messages buffer
// @rd_msg_hdr  : read message header storage
// @rd_msg_hdr_count : how many dwords were already read from header
//
// @hbuf_is_ready : query if the host host/write buffer is ready
// @dr_dscr: DMA ring descriptors: TX, RX, and CTRL
//
// @version     : HBM protocol version in use
// @hbm_f_pg_supported  : hbm feature pgi protocol
// @hbm_f_dc_supported  : hbm feature dynamic clients
// @hbm_f_dot_supported : hbm feature disconnect on timeout
// @hbm_f_ev_supported  : hbm feature event notification
// @hbm_f_fa_supported  : hbm feature fixed address client
// @hbm_f_ie_supported  : hbm feature immediate reply to enum request
// @hbm_f_os_supported  : hbm feature support OS ver message
// @hbm_f_dr_supported  : hbm feature dma ring supported
// @hbm_f_vt_supported  : hbm feature vtag supported
// @hbm_f_cap_supported : hbm feature capabilities message supported
// @hbm_f_cd_supported  : hbm feature client dma supported
// @hbm_f_gsc_supported : hbm feature gsc supported
//
// @fw_ver : FW versions
//
// @fw_f_fw_ver_supported : fw feature: fw version supported
// @fw_ver_received : fw version received
//
// @me_clients_rwsem: rw lock over me_clients list
// @me_clients  : list of FW clients
// @me_clients_map : FW clients bit map
// @host_clients_map : host clients id pool
//
// @allow_fixed_address: allow user space to connect a fixed client
// @override_fixed_address: force allow fixed address behavior
//
// @timeouts: actual timeout values
//
// @reset_work  : work item for the device reset
// @bus_rescan_work : work item for the bus rescan
//
// @device_list : mei client bus list
// @cl_bus_lock : client bus list lock
//
// @kind        : kind of mei device
//
// @dbgfs_dir   : debugfs mei root directory
//
// @gsc_reset_to_pxp     : state of reset to the PXP mode
//
// @ops:        : hw specific operations
// @hw          : hw specific data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_device {
    pub parent: *mut device,
    pub dev: device,
    pub cdev: *mut cdev,
    pub minor: c_int,
    pub write_list: list_head,
    pub write_waiting_list: list_head,
    pub ctrl_wr_list: list_head,
    pub ctrl_rd_list: list_head,
    pub tx_queue_limit: u8,
    pub file_list: list_head,
    pub open_handle_count: c_long,
    pub device_lock: mutex,
    pub timer_work: delayed_work,
    pub recvd_hw_ready: bool,
    pub pg_blocked: bool,
    pub read_fws_need_resume: bool,
//
// waiting queue for receive message from FW
//
    pub wait_hw_ready: wait_queue_head_t,
    pub wait_pg: wait_queue_head_t,
    pub wait_hbm_start: wait_queue_head_t,
//
// mei device  states
//
    pub reset_count: c_ulong,
    pub dev_state: mei_dev_state,
    pub wait_dev_state: wait_queue_head_t,
    pub hbm_state: mei_hbm_state,
    pub pxp_mode: mei_dev_pxp_mode,
    pub init_clients_timer: u16,
//
// Power Gating support
//
    pub pg_event: mei_pg_event,

    pub pg_domain: dev_pm_domain,
    pub rd_msg_buf: [c_uchar; MEI_RD_MSG_BUF_SIZE],
    pub rd_msg_hdr: [u32; MEI_RD_MSG_BUF_SIZE],
    pub rd_msg_hdr_count: c_int,
// write buffer
    pub hbuf_is_ready: bool,
    pub dr_dscr: [mei_dma_dscr; DMA_DSCR_NUM],
    pub version: hbm_version,
    pub hbm_f_pg_supported:1: c_uint,
    pub hbm_f_dc_supported:1: c_uint,
    pub hbm_f_dot_supported:1: c_uint,
    pub hbm_f_ev_supported:1: c_uint,
    pub hbm_f_fa_supported:1: c_uint,
    pub hbm_f_ie_supported:1: c_uint,
    pub hbm_f_os_supported:1: c_uint,
    pub hbm_f_dr_supported:1: c_uint,
    pub hbm_f_vt_supported:1: c_uint,
    pub hbm_f_cap_supported:1: c_uint,
    pub hbm_f_cd_supported:1: c_uint,
    pub hbm_f_gsc_supported:1: c_uint,
    pub fw_ver: [mei_fw_version; MEI_MAX_FW_VER_BLOCKS],
    pub fw_f_fw_ver_supported:1: c_uint,
    pub fw_ver_received:1: c_uint,
    pub me_clients_rwsem: rw_semaphore,
    pub me_clients: list_head,
    pub MEI_CLIENTS_MAX): DECLARE_BITMAP(me_clients_map,,
    pub MEI_CLIENTS_MAX): DECLARE_BITMAP(host_clients_map,,
    pub allow_fixed_address: bool,
    pub override_fixed_address: bool,
    pub timeouts: mei_dev_timeouts,
    pub reset_work: work_struct,
    pub bus_rescan_work: work_struct,
// List of bus devices
    pub device_list: list_head,
    pub cl_bus_lock: mutex,
    pub kind: mei_dev_kind,

    pub dbgfs_dir: *mut dentry,

    pub gsc_reset_to_pxp: mei_dev_reset_to_pxp,
    pub ops: *const mei_hw_ops,
    pub )): *mut char hw[] __aligned(sizeof(void,
}

extern "C" {
    pub fn msecs_to_jiffies(MSEC_PER_SEC: *mut *mut sec) -> return;
}
//
// mei_data2slots - get slots number from a message length
//
// @length: size of the messages in bytes
//
// Return: number of slots
//
extern "C" {
    pub fn DIV_ROUND_UP(_arg: length, _arg: MEI_SLOT_SIZE) -> return;
}
//
// mei_hbm2slots - get slots number from a hbm message length
// length + size of the mei message header
//
// @length: size of the messages in bytes
//
// Return: number of slots
//
extern "C" {
    pub fn DIV_ROUND_UP(length: sizeof(struct mei_msg_hdr) +, _arg: MEI_SLOT_SIZE) -> return;
}
//
// mei_slots2data - get data in slots - bytes from slots
//
// @slots: number of available slots
//
// Return: number of bytes in slots
//
// mei init function prototypes
//
extern "C" {
    pub fn mei_reset(dev: *mut mei_device) -> c_int;
}
extern "C" {
    pub fn mei_start(dev: *mut mei_device) -> c_int;
}
extern "C" {
    pub fn mei_restart(dev: *mut mei_device) -> c_int;
}
extern "C" {
    pub fn mei_stop(dev: *mut mei_device);
}
extern "C" {
    pub fn mei_cancel_work(dev: *mut mei_device);
}
extern "C" {
    pub fn mei_set_devstate(dev: *mut mei_device, state: mei_dev_state);
}
extern "C" {
    pub fn mei_dmam_ring_alloc(dev: *mut mei_device) -> c_int;
}
extern "C" {
    pub fn mei_dmam_ring_free(dev: *mut mei_device);
}
extern "C" {
    pub fn mei_dma_ring_is_allocated(dev: *mut mei_device) -> bool;
}
extern "C" {
    pub fn mei_dma_ring_reset(dev: *mut mei_device);
}
extern "C" {
    pub fn mei_dma_ring_read(dev: *mut mei_device, buf: *mut c_uchar, len: u32);
}
extern "C" {
    pub fn mei_dma_ring_write(dev: *mut mei_device, buf: *mut c_uchar, len: u32);
}
extern "C" {
    pub fn mei_dma_ring_empty_slots(dev: *mut mei_device) -> u32;
}
//
// MEI interrupt functions prototype
//
extern "C" {
    pub fn mei_timer(work: *mut work_struct);
}
extern "C" {
    pub fn mei_schedule_stall_timer(dev: *mut mei_device);
}
extern "C" {
    pub fn mei_irq_write_handler(dev: *mut mei_device, cmpl_list: *mut list_head) -> c_int;
}
extern "C" {
    pub fn mei_irq_compl_handler(dev: *mut mei_device, cmpl_list: *mut list_head);
}
//
// Register Access Function
//
extern "C" {
    pub fn mei_hbuf_acquire(dev: *mut mei_device) -> bool;
}
extern "C" {
    pub fn mei_write_is_idle(dev: *mut mei_device) -> bool;
}

extern "C" {
    pub fn mei_dbgfs_register(dev: *mut mei_device, name: *const c_char);
}
extern "C" {
    pub fn mei_dbgfs_deregister(dev: *mut mei_device);
}

extern "C" {
    pub fn mei_register(dev: *mut mei_device, parent: *mut device) -> c_int;
}
extern "C" {
    pub fn mei_deregister(dev: *mut mei_device);
}

extern "C" {
    pub fn mei_fw_status2str(fw_sts: *mut mei_fw_status, buf: *mut c_char, len: usize) -> isize;
}
//
// mei_fw_status_str - fetch and convert fw status registers to printable string
//
// @dev: the device structure
// @buf: string buffer at minimal size MEI_FW_STATUS_STR_SZ
// @len: buffer len must be >= MEI_FW_STATUS_STR_SZ
//
// Return: number of bytes written or < 0 on failure
//
// kind_is_gsc - checks whether the device is gsc
//
// @dev: the device structure
//
// Return: whether the device is gsc
//
// kind_is_gscfi - checks whether the device is gscfi
//
// @dev: the device structure
//
// Return: whether the device is gscfi
//
