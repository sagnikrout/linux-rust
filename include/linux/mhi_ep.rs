//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mhi_ep.h
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
// Copyright (c) 2022, Linaro Ltd.
//

pub const MHI_EP_DEFAULT_MTU: c_uint = 0x8000;
//
// struct mhi_ep_channel_config - Channel configuration structure for controller
// @name: The name of this channel
// @num: The number assigned to this channel
// @num_elements: The number of elements that can be queued to this channel
// @dir: Direction that data may flow on this channel
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhi_ep_channel_config {
    pub name: *mut c_char,
    pub num: u32,
    pub num_elements: u32,
    pub dir: dma_data_direction,
}

//
// struct mhi_ep_cntrl_config - MHI Endpoint controller configuration
// @mhi_version: MHI spec version supported by the controller
// @max_channels: Maximum number of channels supported
// @num_channels: Number of channels defined in @ch_cfg
// @ch_cfg: Array of defined channels
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhi_ep_cntrl_config {
    pub mhi_version: u32,
    pub max_channels: u32,
    pub num_channels: u32,
    pub ch_cfg: *const mhi_ep_channel_config,
}

//
// struct mhi_ep_db_info - MHI Endpoint doorbell info
// @mask: Mask of the doorbell interrupt
// @status: Status of the doorbell interrupt
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhi_ep_db_info {
    pub mask: u32,
    pub status: u32,
}

//
// struct mhi_ep_buf_info - MHI Endpoint transfer buffer info
// @mhi_dev: MHI device associated with this buffer
// @dev_addr: Address of the buffer in endpoint
// @host_addr: Address of the bufffer in host
// @size: Size of the buffer
// @code: Transfer completion code
// @cb: Callback to be executed by controller drivers after transfer completion (async)
// @cb_buf: Opaque buffer to be passed to the callback
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhi_ep_buf_info {
    pub mhi_dev: *mut mhi_ep_device,
    pub dev_addr: *mut c_void,
    pub host_addr: u64,
    pub size: usize,
    pub code: c_int,
    pub buf_info): *mut *mut void (cb)(struct mhi_ep_buf_info,
    pub cb_buf: *mut c_void,
}

//
// struct mhi_ep_cntrl - MHI Endpoint controller structure
// @cntrl_dev: Pointer to the struct device of physical bus acting as the MHI
// Endpoint controller
// @mhi_dev: MHI Endpoint device instance for the controller
// @mmio: MMIO region containing the MHI registers
// @mhi_chan: Points to the channel configuration table
// @mhi_event: Points to the event ring configurations table
// @mhi_cmd: Points to the command ring configurations table
// @sm: MHI Endpoint state machine
// @ch_ctx_cache: Cache of host channel context data structure
// @ev_ctx_cache: Cache of host event context data structure
// @cmd_ctx_cache: Cache of host command context data structure
// @ch_ctx_host_pa: Physical address of host channel context data structure
// @ev_ctx_host_pa: Physical address of host event context data structure
// @cmd_ctx_host_pa: Physical address of host command context data structure
// @ch_ctx_cache_phys: Physical address of the host channel context cache
// @ev_ctx_cache_phys: Physical address of the host event context cache
// @cmd_ctx_cache_phys: Physical address of the host command context cache
// @chdb: Array of channel doorbell interrupt info
// @event_lock: Lock for protecting event rings
// @state_lock: Lock for protecting state transitions
// @list_lock: Lock for protecting state transition and channel doorbell lists
// @st_transition_list: List of state transitions
// @ch_db_list: List of queued channel doorbells
// @wq: Dedicated workqueue for handling rings and state changes
// @state_work: State transition worker
// @reset_work: Worker for MHI Endpoint reset
// @cmd_ring_work: Worker for processing command rings
// @ch_ring_work: Worker for processing channel rings
// @raise_irq: CB function for raising IRQ to the host
// @alloc_map: CB function for allocating memory in endpoint for storing host context and mapping it
// @unmap_free: CB function to unmap and free the allocated memory in endpoint for storing host context
// @read_sync: CB function for reading from host memory synchronously
// @write_sync: CB function for writing to host memory synchronously
// @read_async: CB function for reading from host memory asynchronously
// @write_async: CB function for writing to host memory asynchronously
// @flush_async: CB function for flushing asynchronous read/writes
// @mhi_state: MHI Endpoint state
// @max_chan: Maximum channels supported by the endpoint controller
// @mru: MRU (Maximum Receive Unit) value of the endpoint controller
// @event_rings: Number of event rings supported by the endpoint controller
// @hw_event_rings: Number of hardware event rings supported by the endpoint controller
// @chdb_offset: Channel doorbell offset set by the host
// @erdb_offset: Event ring doorbell offset set by the host
// @index: MHI Endpoint controller index
// @irq: IRQ used by the endpoint controller
// @enabled: Check if the endpoint controller is enabled or not
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhi_ep_cntrl {
    pub cntrl_dev: *mut device,
    pub mhi_dev: *mut mhi_ep_device,
    pub mmio: *mut void __iomem,
    pub mhi_chan: *mut mhi_ep_chan,
    pub mhi_event: *mut mhi_ep_event,
    pub mhi_cmd: *mut mhi_ep_cmd,
    pub sm: *mut mhi_ep_sm,
    pub ch_ctx_cache: *mut mhi_chan_ctxt,
    pub ev_ctx_cache: *mut mhi_event_ctxt,
    pub cmd_ctx_cache: *mut mhi_cmd_ctxt,
    pub ch_ctx_host_pa: u64,
    pub ev_ctx_host_pa: u64,
    pub cmd_ctx_host_pa: u64,
    pub ch_ctx_cache_phys: phys_addr_t,
    pub ev_ctx_cache_phys: phys_addr_t,
    pub cmd_ctx_cache_phys: phys_addr_t,
    pub chdb: [mhi_ep_db_info; 4],
    pub event_lock: mutex,
    pub state_lock: mutex,
    pub list_lock: spinlock_t,
    pub st_transition_list: list_head,
    pub ch_db_list: list_head,
    pub wq: *mut workqueue_struct,
    pub state_work: work_struct,
    pub reset_work: work_struct,
    pub cmd_ring_work: work_struct,
    pub ch_ring_work: work_struct,
    pub ring_item_cache: *mut kmem_cache,
    pub ev_ring_el_cache: *mut kmem_cache,
    pub tre_buf_cache: *mut kmem_cache,
    pub vector): *mut *mut *mut void (raise_irq)(struct mhi_ep_cntrl mhi_cntrl, u32,
    pub size): *mut *mut *mut void __iomem virt, size_t,
    pub size): *mut *mut void __iomem virt, size_t,
    pub buf_info): *mut *mut *mut int (read_sync)(struct mhi_ep_cntrl mhi_cntrl, struct mhi_ep_buf_info,
    pub buf_info): *mut *mut *mut int (write_sync)(struct mhi_ep_cntrl mhi_cntrl, struct mhi_ep_buf_info,
    pub buf_info): *mut *mut *mut int (read_async)(struct mhi_ep_cntrl mhi_cntrl, struct mhi_ep_buf_info,
    pub buf_info): *mut *mut *mut int (write_async)(struct mhi_ep_cntrl mhi_cntrl, struct mhi_ep_buf_info,
    pub mhi_cntrl): *mut *mut void (flush_async)(struct mhi_ep_cntrl,
    pub mhi_state: mhi_state,
    pub max_chan: u32,
    pub mru: u32,
    pub event_rings: u32,
    pub hw_event_rings: u32,
    pub chdb_offset: u32,
    pub erdb_offset: u32,
    pub index: u32,
    pub irq: c_int,
    pub enabled: bool,
}

//
// struct mhi_ep_device - Structure representing an MHI Endpoint device that binds
// to channels or is associated with controllers
// @dev: Driver model device node for the MHI Endpoint device
// @mhi_cntrl: Controller the device belongs to
// @id: Pointer to MHI Endpoint device ID struct
// @name: Name of the associated MHI Endpoint device
// @ul_chan: UL (from host to endpoint) channel for the device
// @dl_chan: DL (from endpoint to host) channel for the device
// @dev_type: MHI device type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhi_ep_device {
    pub dev: device,
    pub mhi_cntrl: *mut mhi_ep_cntrl,
    pub id: *const mhi_device_id,
    pub name: *const c_char,
    pub ul_chan: *mut mhi_ep_chan,
    pub dl_chan: *mut mhi_ep_chan,
    pub dev_type: mhi_device_type,
}

//
// struct mhi_ep_driver - Structure representing a MHI Endpoint client driver
// @id_table: Pointer to MHI Endpoint device ID table
// @driver: Device driver model driver
// @probe: CB function for client driver probe function
// @remove: CB function for client driver remove function
// @ul_xfer_cb: CB function for UL (from host to endpoint) data transfer
// @dl_xfer_cb: CB function for DL (from endpoint to host) data transfer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhi_ep_driver {
    pub id_table: *const mhi_device_id,
    pub driver: device_driver,
    pub id): *const mhi_device_id,
    pub mhi_ep): *mut *mut void (remove)(struct mhi_ep_device,
    pub result): *mut mhi_result,
    pub result): *mut mhi_result,
}

//
// module_mhi_ep_driver() - Helper macro for drivers that don't do
// anything special other than using default mhi_ep_driver_register() and
// mhi_ep_driver_unregister().  This eliminates a lot of boilerplate.
// Each module may only use this macro once.
//

//
// Macro to avoid include chaining to get THIS_MODULE
//

//
// __mhi_ep_driver_register - Register a driver with MHI Endpoint bus
// @mhi_drv: Driver to be associated with the device
// @owner: The module owner
//
// Return: 0 if driver registrations succeeds, a negative error code otherwise.
//
extern "C" {
    pub fn __mhi_ep_driver_register(mhi_drv: *mut mhi_ep_driver, owner: *mut module) -> c_int;
}
//
// mhi_ep_driver_unregister - Unregister a driver from MHI Endpoint bus
// @mhi_drv: Driver associated with the device
//
extern "C" {
    pub fn mhi_ep_driver_unregister(mhi_drv: *mut mhi_ep_driver);
}
//
// mhi_ep_register_controller - Register MHI Endpoint controller
// @mhi_cntrl: MHI Endpoint controller to register
// @config: Configuration to use for the controller
//
// Return: 0 if controller registrations succeeds, a negative error code otherwise.
//
// mhi_ep_unregister_controller - Unregister MHI Endpoint controller
// @mhi_cntrl: MHI Endpoint controller to unregister
//
extern "C" {
    pub fn mhi_ep_unregister_controller(mhi_cntrl: *mut mhi_ep_cntrl);
}
//
// mhi_ep_power_up - Power up the MHI endpoint stack
// @mhi_cntrl: MHI Endpoint controller
//
// Return: 0 if power up succeeds, a negative error code otherwise.
//
extern "C" {
    pub fn mhi_ep_power_up(mhi_cntrl: *mut mhi_ep_cntrl) -> c_int;
}
//
// mhi_ep_power_down - Power down the MHI endpoint stack
// @mhi_cntrl: MHI controller
//
extern "C" {
    pub fn mhi_ep_power_down(mhi_cntrl: *mut mhi_ep_cntrl);
}
//
// mhi_ep_queue_is_empty - Determine whether the transfer queue is empty
// @mhi_dev: Device associated with the channels
// @dir: DMA direction for the channel
//
// Return: true if the queue is empty, false otherwise.
//
extern "C" {
    pub fn mhi_ep_queue_is_empty(mhi_dev: *mut mhi_ep_device, dir: dma_data_direction) -> bool;
}
//
// mhi_ep_queue_skb - Send SKBs to host over MHI Endpoint
// @mhi_dev: Device associated with the DL channel
// @skb: SKBs to be queued
//
// Return: 0 if the SKBs has been sent successfully, a negative error code otherwise.
//
extern "C" {
    pub fn mhi_ep_queue_skb(mhi_dev: *mut mhi_ep_device, skb: *mut sk_buff) -> c_int;
}
