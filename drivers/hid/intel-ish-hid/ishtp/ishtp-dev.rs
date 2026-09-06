//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/intel-ish-hid/ishtp/ishtp-dev.h
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
// Most ISHTP provider device and ISHTP logic declarations
//
// Copyright (c) 2003-2016, Intel Corporation.
//

pub const IPC_PAYLOAD_SIZE: c_int = 128;

pub const IPC_FULL_MSG_SIZE: c_int = 132;
// Number of messages to be held in ISR->BH FIFO
pub const RD_INT_FIFO_SIZE: c_int = 64;
//
// Number of IPC messages to be held in Tx FIFO, to be sent by ISR -
// Tx complete interrupt or RX_COMPLETE handler
//
pub const IPC_TX_FIFO_SIZE: c_int = 512;
//
// Number of Maximum ISHTP Clients
//
pub const ISHTP_CLIENTS_MAX: c_int = 256;
//
// Number of File descriptors/handles
// that can be opened to the driver.
//
// Limit to 255: 256 Total Clients
// minus internal client for ISHTP Bus Messages
//

// Internal Clients Number

pub const ISHTP_HBM_HOST_CLIENT_ID: c_int = 0;
pub const MAX_DMA_DELAY: c_int = 20;
// 300ms to get resume response
pub const WAIT_FOR_RESUME_ACK_MS: c_int = 300;
// ISHTP device states
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ishtp_dev_state {
    ISHTP_DEV_INITIALIZING = 0,
    ISHTP_DEV_INIT_CLIENTS,
    ISHTP_DEV_ENABLED,
    ISHTP_DEV_RESETTING,
    ISHTP_DEV_DISABLED,
    ISHTP_DEV_POWER_DOWN,
    ISHTP_DEV_POWER_UP
}

//
// struct ishtp_fw_client - representation of fw client
//
// @props - client properties
// @client_id - fw client id
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ishtp_fw_client {
    pub props: ishtp_client_properties,
    pub client_id: u8,
}

//
// Control info for IPC messages ISHTP/IPC sending FIFO -
// list with inline data buffer
// This structure will be filled with parameters submitted
// by the caller glue layer
// 'buf' may be pointing to the external buffer or to 'inline_data'
// 'offset' will be initialized to 0 by submitting
//
// 'ipc_send_compl' is intended for use by clients that send fragmented
// messages. When a fragment is sent down to IPC msg regs,
// it will be called.
// If it has more fragments to send, it will do it. With last fragment
// it will send appropriate ISHTP "message-complete" flag.
// It will remove the outstanding message
// (mark outstanding buffer as available).
// If counting flow control is in work and there are more flow control
// credits, it can put the next client message queued in cl.
// structure for IPC processing.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wr_msg_ctl_info {
// Will be called with 'ipc_send_compl_prm' as parameter
    pub ): *mut *mut void (ipc_send_compl)(void,
    pub ipc_send_compl_prm: *mut c_void,
    pub length: usize,
    pub link: list_head,
    pub inline_data: [c_uchar; IPC_FULL_MSG_SIZE],
}

//
// The ISHTP layer talks to hardware IPC message using the following
// callbacks
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ishtp_hw_ops {
    pub dev): *mut *mut int (hw_reset)(struct ishtp_device,
    pub dev): *mut *mut int (ipc_reset)(struct ishtp_device,
    pub busy): c_int,
    pub length): *mut *mut unsigned char msg, int,
    pub dev): *const *const uint32_t (ishtp_read_hdr)(struct ishtp_device,
    pub buffer_length): c_ulong,
    pub dev): *mut *mut uint32_t (get_fw_status)(struct ishtp_device,
    pub dev): *mut *mut void (sync_fw_clock)(struct ishtp_device,
    pub dev): *mut *mut bool (dma_no_cache_snooping)(struct ishtp_device,
}

//
// struct ishtp_driver_data - Driver-specific data for ISHTP devices
//
// This structure holds driver-specific data that can be associated with each
// ISHTP device instance. It allows for the storage of data that is unique to
// a particular driver or hardware variant.
//
// @fw_generation: The generation name associated with a specific hardware
// variant of the Intel Integrated Sensor Hub (ISH). This allows
// the driver to load the correct firmware based on the device's
// hardware variant. For example, "lnlm" for the Lunar Lake-M
// platform. The generation name must not exceed 8 characters
// in length.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ishtp_driver_data {
    pub fw_generation: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ish_version {
    pub major: u16,
    pub minor: u16,
    pub hotfix: u16,
    pub build: u16,
}

//
// struct ishtp_device - ISHTP private device struct
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ishtp_device {
    pub /: *mut *mut *mut device devc; / pointer to lowest device,
    pub /: *mut *mut *mut pci_dev pdev; / PCI device to get device ids,
    pub /: *mut *mut *mut ishtp_driver_data driver_data; / pointer to driver-specific data,
// waitq for waiting for suspend response
    pub suspend_wait: wait_queue_head_t,
    pub /: *mut *mut bool suspend_flag; / Suspend is active,
// waitq for waiting for resume response
    pub resume_wait: wait_queue_head_t,
    pub /: *mut *mut bool resume_flag; /Resume is active,
//
// lock for the device, for everything that doesn't have
// a dedicated spinlock
//
    pub device_lock: spinlock_t,
    pub recvd_hw_ready: bool,
    pub version: hbm_version,
    pub /: *mut *mut int transfer_path; / Choice of transfer path: IPC or DMA,
// Alloc a dedicated unbound workqueue for ishtp device
    pub unbound_wq: *mut workqueue_struct,
// work structure for scheduling firmware loading tasks
    pub work_fw_loader: work_struct,
// waitq for waiting for command response from the firmware loader
    pub wait_loader_recvd_msg: wait_queue_head_t,
// indicating whether a message from the firmware loader has been received
    pub fw_loader_received: bool,
// pointer to a buffer for receiving messages from the firmware loader
    pub fw_loader_rx_buf: *mut c_void,
// size of the buffer pointed to by fw_loader_rx_buf
    pub fw_loader_rx_size: c_int,
// ishtp device states
    pub dev_state: ishtp_dev_state,
    pub hbm_state: ishtp_hbm_state,
// driver read queue
    pub read_list: ishtp_cl_rb,
    pub read_list_spinlock: spinlock_t,
// list of ishtp_cl's
    pub cl_list: list_head,
    pub cl_list_lock: spinlock_t,
    pub open_handle_count: c_long,
// List of bus devices
    pub device_list: list_head,
    pub device_list_lock: spinlock_t,
// waiting queues for receive message from FW
    pub wait_hw_ready: wait_queue_head_t,
    pub wait_hbm_recvd_msg: wait_queue_head_t,
// FIFO for input messages for BH processing
    pub IPC_PAYLOAD_SIZE]: *mut *mut unsigned char rd_msg_fifo[RD_INT_FIFO_SIZE,
    pub rd_msg_fifo_tail: unsigned int rd_msg_fifo_head,,
    pub rd_msg_spinlock: spinlock_t,
    pub bh_hbm_work: work_struct,
// IPC write queue
    pub wr_free_list: list_head wr_processing_list,,
// For both processing list  and free list
    pub wr_processing_spinlock: spinlock_t,
    pub allocated*/: *mut *mut *mut ishtp_fw_client fw_clients; /Note:memory has to be,
    pub ISHTP_CLIENTS_MAX): DECLARE_BITMAP(fw_clients_map,,
    pub ISHTP_CLIENTS_MAX): DECLARE_BITMAP(host_clients_map,,
    pub fw_clients_num: u8,
    pub fw_client_presentation_num: u8,
    pub fw_client_index: u8,
    pub fw_clients_lock: spinlock_t,
// TX DMA buffers and slots
    pub ishtp_host_dma_enabled: c_int,
    pub ishtp_host_dma_tx_buf: *mut c_void,
    pub ishtp_host_dma_tx_buf_size: c_uint,
    pub ishtp_host_dma_tx_buf_phys: u64,
    pub ishtp_dma_num_slots: c_int,
// map of 4k blocks in Tx dma buf: 0-free, 1-used
    pub ishtp_dma_tx_map: *mut u8,
    pub ishtp_dma_tx_lock: spinlock_t,
// RX DMA buffers and slots
    pub ishtp_host_dma_rx_buf: *mut c_void,
    pub ishtp_host_dma_rx_buf_size: c_uint,
    pub ishtp_host_dma_rx_buf_phys: u64,
// Dump to trace buffers if enabled
    pub print_log: ishtp_print_log,
// Base version of Intel's released firmware
    pub base_ver: ish_version,
// Vendor-customized project version
    pub prj_ver: ish_version,
// Debug stats
    pub ipc_rx_cnt: c_uint,
    pub ipc_rx_bytes_cnt: c_ulonglong,
    pub ipc_tx_cnt: c_uint,
    pub ipc_tx_bytes_cnt: c_ulonglong,
// Time of the last clock sync
    pub prev_sync: c_ulong,
    pub ops: *const ishtp_hw_ops,
    pub mtu: usize,
    pub ishtp_msg_hdr: u32,
    pub )): *mut char hw[] __aligned(sizeof(void,
}

extern "C" {
    pub fn msecs_to_jiffies(MSEC_PER_SEC: *mut *mut sec) -> return;
}
//
// Register Access Function
//
// Exported function
extern "C" {
    pub fn ishtp_device_init(dev: *mut ishtp_device);
}
extern "C" {
    pub fn ishtp_start(dev: *mut ishtp_device) -> c_int;
}
