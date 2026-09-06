//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/amazon/ena/ena_com.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// Copyright 2015-2020 Amazon.com, Inc. or its affiliates. All rights reserved.
//

// Macro flag: #define ENA_COM

// We need to queues for each IO (on for Tx and one for Rx)

pub const ENA_MAX_HANDLERS: c_int = 256;
pub const ENA_MAX_PHYS_ADDR_SIZE_BITS: c_int = 48;
// Unit in usec
pub const ENA_REG_READ_TIMEOUT: c_int = 200000;

pub const ENA_CUSTOMER_METRICS_BUFFER_SIZE: c_int = 512;
//
// ENA adaptive interrupt moderation settings
pub const ENA_INTR_INITIAL_TX_INTERVAL_USECS: c_int = 64;
pub const ENA_INTR_INITIAL_RX_INTERVAL_USECS: c_int = 20;
pub const ENA_DEFAULT_INTR_DELAY_RESOLUTION: c_int = 1;
pub const ENA_HASH_KEY_SIZE: c_int = 40;
pub const ENA_HW_HINTS_NO_TIMEOUT: c_uint = 0xFFFF;
pub const ENA_FEATURE_MAX_QUEUE_EXT_VER: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_llq_configurations {
    pub llq_header_location: ena_admin_llq_header_location,
    pub llq_ring_entry_size: ena_admin_llq_ring_entry_size,
    pub llq_stride_ctrl: ena_admin_llq_stride_ctrl,
    pub llq_num_decs_before_header: ena_admin_llq_num_descs_before_header,
    pub llq_ring_entry_size_value: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum queue_direction {
    ENA_COM_IO_QUEUE_DIRECTION_TX,
    ENA_COM_IO_QUEUE_DIRECTION_RX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_com_buf {
    pub /: *mut *mut *mut dma_addr_t paddr; /< Buffer physical address,
    pub /: *mut *mut *mut u16 len; /< Buffer length in bytes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_com_rx_buf_info {
    pub len: u16,
    pub req_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_com_io_desc_addr {
    pub /: *mut *mut *mut u8 __iomem pbuf_dev_addr; / LLQ address,
    pub virt_addr: *mut u8,
    pub phys_addr: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_com_tx_meta {
    pub mss: u16,
    pub l3_hdr_len: u16,
    pub l3_hdr_offset: u16,
    pub /: *mut *mut u16 l4_hdr_len; / In words,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_com_llq_info {
    pub header_location_ctrl: u16,
    pub desc_stride_ctrl: u16,
    pub desc_list_entry_size_ctrl: u16,
    pub desc_list_entry_size: u16,
    pub descs_num_before_header: u16,
    pub descs_per_entry: u16,
    pub max_entries_in_tx_burst: u16,
    pub disable_meta_caching: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_com_io_cq {
    pub cdesc_addr: ena_com_io_desc_addr,
// Interrupt unmask register
    pub unmask_reg: *mut u32 __iomem,
// numa configuration register (for TPH)
    pub numa_node_cfg_reg: *mut u32 __iomem,
// The value to write to the above register to unmask
// the interrupt of this queue
//
    pub ____cacheline_aligned: u32 msix_vector,
    pub direction: queue_direction,
// holds the number of cdesc of the current packet
    pub cur_rx_pkt_cdesc_count: u16,
// save the first cdesc idx of the current packet
    pub cur_rx_pkt_cdesc_start_idx: u16,
    pub q_depth: u16,
// Caller qid
    pub qid: u16,
// Device queue index
    pub idx: u16,
    pub head: u16,
    pub phase: u8,
    pub cdesc_entry_size_in_bytes: u8,
    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_com_io_bounce_buffer_control {
    pub base_buffer: *mut u8,
    pub next_to_use: u16,
    pub buffer_size: u16,
    pub /: *mut *mut u16 buffers_num; / Must be a power of 2,
}

// This struct is to keep tracking the current location of the next llq entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_com_llq_pkt_ctrl {
    pub curr_bounce_buf: *mut u8,
    pub idx: u16,
    pub descs_left_in_line: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_com_io_sq {
    pub desc_addr: ena_com_io_desc_addr,
    pub db_addr: *mut u32 __iomem,
    pub direction: queue_direction,
    pub mem_queue_type: ena_admin_placement_policy_type,
    pub disable_meta_caching: bool,
    pub msix_vector: u32,
    pub cached_tx_meta: ena_com_tx_meta,
    pub llq_info: ena_com_llq_info,
    pub llq_buf_ctrl: ena_com_llq_pkt_ctrl,
    pub bounce_buf_ctrl: ena_com_io_bounce_buffer_control,
    pub q_depth: u16,
    pub qid: u16,
    pub idx: u16,
    pub tail: u16,
    pub next_to_comp: u16,
    pub llq_last_copy_tail: u16,
    pub tx_max_header_size: u32,
    pub phase: u8,
    pub desc_entry_size: u8,
    pub dma_addr_bits: u8,
    pub entries_in_tx_burst_left: u16,
    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_com_admin_cq {
    pub entries: *mut ena_admin_acq_entry,
    pub dma_addr: dma_addr_t,
    pub head: u16,
    pub phase: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_com_admin_sq {
    pub entries: *mut ena_admin_aq_entry,
    pub dma_addr: dma_addr_t,
    pub db_addr: *mut u32 __iomem,
    pub head: u16,
    pub tail: u16,
    pub phase: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_com_stats_admin {
    pub aborted_cmd: u64,
    pub submitted_cmd: u64,
    pub completed_cmd: u64,
    pub out_of_space: u64,
    pub no_completion: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_com_stats_phc {
    pub phc_cnt: u64,
    pub phc_exp: u64,
    pub phc_skp: u64,
    pub phc_err_dv: u64,
    pub phc_err_ts: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_com_admin_queue {
    pub q_dmadev: *mut c_void,
    pub ena_dev: *mut ena_com_dev,
    pub /: *mut *mut spinlock_t q_lock; / spinlock for the admin queue,
    pub comp_ctx: *mut ena_comp_ctx,
    pub completion_timeout: u32,
    pub q_depth: u16,
    pub cq: ena_com_admin_cq,
    pub sq: ena_com_admin_sq,
// Indicate if the admin queue should poll for completion
    pub polling: bool,
    pub curr_cmd_id: u16,
// Indicate that the ena was initialized and can
// process new admin commands
//
    pub running_state: bool,
// Count the number of outstanding admin commands
    pub outstanding_cmds: core::sync::atomic::AtomicI32,
    pub stats: ena_com_stats_admin,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_com_aenq {
    pub head: u16,
    pub phase: u8,
    pub entries: *mut ena_admin_aenq_entry,
    pub dma_addr: dma_addr_t,
    pub q_depth: u16,
    pub aenq_handlers: *mut ena_aenq_handlers,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_com_mmio_read {
    pub read_resp: *mut ena_admin_ena_mmio_req_read_less_resp,
    pub read_resp_dma_addr: dma_addr_t,
    pub /: *mut *mut u32 reg_read_to; / in us,
    pub seq_num: u16,
    pub readless_supported: bool,
// spin lock to ensure a single outstanding read
    pub lock: spinlock_t,
}

// PTP hardware clock (PHC) MMIO read data info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_com_phc_info {
// Internal PHC statistics
    pub stats: ena_com_stats_phc,
// PHC shared memory - virtual address
    pub virt_addr: *mut ena_admin_phc_resp,
// System time of last PHC request
    pub system_time: ktime_t,
// Spin lock to ensure a single outstanding PHC read
    pub lock: spinlock_t,
// PHC doorbell address as an offset to PCIe MMIO REG BAR
    pub doorbell_offset: u32,
// Shared memory read expire timeout (usec)
// Max time for valid PHC retrieval, passing this threshold will fail
// the get time request and block new PHC requests for block_timeout_usec
// in order to prevent floods on busy device
//
    pub expire_timeout_usec: u32,
// Shared memory read abort timeout (usec)
// PHC requests block period, blocking starts once PHC request expired
// in order to prevent floods on busy device,
// any PHC requests during block period will be skipped
//
    pub block_timeout_usec: u32,
// PHC shared memory - physical address
    pub phys_addr: dma_addr_t,
// Request id sent to the device
    pub req_id: u16,
// True if PHC is active in the device
    pub active: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_rss {
// Indirect table
    pub host_rss_ind_tbl: *mut u16,
    pub rss_ind_tbl: *mut ena_admin_rss_ind_table_entry,
    pub rss_ind_tbl_dma_addr: dma_addr_t,
    pub tbl_log_size: u16,
// Hash key
    pub hash_func: ena_admin_hash_functions,
    pub hash_key: *mut ena_admin_feature_rss_flow_hash_control,
    pub hash_key_dma_addr: dma_addr_t,
    pub hash_init_val: u32,
// Flow Control
    pub hash_ctrl: *mut ena_admin_feature_rss_hash_control,
    pub hash_ctrl_dma_addr: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_customer_metrics {
// in correlation with ENA_ADMIN_CUSTOMER_METRICS_SUPPORT_MASK
// and ena_admin_customer_metrics_id
//
    pub supported_metrics: u64,
    pub buffer_dma_addr: dma_addr_t,
    pub buffer_virt_addr: *mut c_void,
    pub buffer_len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_host_attribute {
// Debug area
    pub debug_area_virt_addr: *mut u8,
    pub debug_area_dma_addr: dma_addr_t,
    pub debug_area_size: u32,
// Host information
    pub host_info: *mut ena_admin_host_info,
    pub host_info_dma_addr: dma_addr_t,
}

// Each ena_dev is a PCI function.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_com_dev {
    pub admin_queue: ena_com_admin_queue,
    pub aenq: ena_com_aenq,
    pub io_cq_queues: [ena_com_io_cq; ENA_TOTAL_NUM_QUEUES],
    pub io_sq_queues: [ena_com_io_sq; ENA_TOTAL_NUM_QUEUES],
    pub reg_bar: *mut u8 __iomem,
    pub mem_bar: *mut void __iomem,
    pub dmadev: *mut c_void,
    pub net_device: *mut net_device,
    pub tx_mem_queue_type: ena_admin_placement_policy_type,
    pub tx_max_header_size: u32,
    pub /: *mut *mut u16 stats_func; / Selected function for extended statistic dump,
    pub /: *mut *mut u16 stats_queue; / Selected queue for extended statistic dump,
    pub ena_min_poll_delay_us: u32,
    pub mmio_read: ena_com_mmio_read,
    pub phc: ena_com_phc_info,
    pub rss: ena_rss,
    pub supported_features: u32,
    pub capabilities: u32,
    pub dma_addr_bits: u32,
    pub host_attr: ena_host_attribute,
    pub adaptive_coalescing: bool,
    pub intr_delay_resolution: u16,
// interrupt moderation intervals are in usec divided by
// intr_delay_resolution, which is supplied by the device.
//
    pub intr_moder_tx_interval: u32,
    pub intr_moder_rx_interval: u32,
    pub intr_moder_tbl: *mut ena_intr_moder_entry,
    pub llq_info: ena_com_llq_info,
    pub customer_metrics: ena_customer_metrics,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_com_dev_get_features_ctx {
    pub max_queues: ena_admin_queue_feature_desc,
    pub max_queue_ext: ena_admin_queue_ext_feature_desc,
    pub dev_attr: ena_admin_device_attr_feature_desc,
    pub aenq: ena_admin_feature_aenq_desc,
    pub offload: ena_admin_feature_offload_desc,
    pub hw_hints: ena_admin_ena_hw_hints,
    pub llq: ena_admin_feature_llq_desc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_com_create_io_ctx {
    pub mem_queue_type: ena_admin_placement_policy_type,
    pub direction: queue_direction,
    pub numa_node: c_int,
    pub msix_vector: u32,
    pub queue_size: u16,
    pub qid: u16,
}

// Holds aenq handlers. Indexed by AENQ event group
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_aenq_handlers {
    pub handlers: [ena_aenq_handler; ENA_MAX_HANDLERS],
    pub unimplemented_handler: ena_aenq_handler,
}

//
// ena_com_mmio_reg_read_request_init - Init the mmio reg read mechanism
// @ena_dev: ENA communication layer struct
//
// Initialize the register read mechanism.
//
// @note: This method must be the first stage in the initialization sequence.
//
// @return - 0 on success, negative value on failure.
//
extern "C" {
    pub fn ena_com_mmio_reg_read_request_init(ena_dev: *mut ena_com_dev) -> c_int;
}
// ena_com_phc_init - Allocate and initialize PHC feature
// @ena_dev: ENA communication layer struct
// @note: This method assumes PHC is supported by the device
// @return - 0 on success, negative value on failure
//
extern "C" {
    pub fn ena_com_phc_init(ena_dev: *mut ena_com_dev) -> c_int;
}
// ena_com_phc_supported - Return if PHC feature is supported by the device
// @ena_dev: ENA communication layer struct
// @note: This method must be called after getting supported features
// @return - supported or not
//
extern "C" {
    pub fn ena_com_phc_supported(ena_dev: *mut ena_com_dev) -> bool;
}
// ena_com_phc_config - Configure PHC feature
// @ena_dev: ENA communication layer struct
// Configure PHC feature in driver and device
// @note: This method assumes PHC is supported by the device
// @return - 0 on success, negative value on failure
//
extern "C" {
    pub fn ena_com_phc_config(ena_dev: *mut ena_com_dev) -> c_int;
}
// ena_com_phc_destroy - Destroy PHC feature
// @ena_dev: ENA communication layer struct
//
extern "C" {
    pub fn ena_com_phc_destroy(ena_dev: *mut ena_com_dev);
}
// ena_com_phc_get_timestamp - Retrieve PHC timestamp
// @ena_dev: ENA communication layer struct
// @timestamp: Retrieved PHC timestamp
// @return - 0 on success, negative value on failure
//
extern "C" {
    pub fn ena_com_phc_get_timestamp(ena_dev: *mut ena_com_dev, timestamp: *mut u64) -> c_int;
}
// ena_com_set_mmio_read_mode - Enable/disable the indirect mmio reg read mechanism
// @ena_dev: ENA communication layer struct
// @readless_supported: readless mode (enable/disable)
//
// ena_com_mmio_reg_read_request_write_dev_addr - Write the mmio reg read return
// value physical address.
// @ena_dev: ENA communication layer struct
//
extern "C" {
    pub fn ena_com_mmio_reg_read_request_write_dev_addr(ena_dev: *mut ena_com_dev);
}
// ena_com_mmio_reg_read_request_destroy - Destroy the mmio reg read mechanism
// @ena_dev: ENA communication layer struct
//
extern "C" {
    pub fn ena_com_mmio_reg_read_request_destroy(ena_dev: *mut ena_com_dev);
}
// ena_com_admin_init - Init the admin and the async queues
// @ena_dev: ENA communication layer struct
// @aenq_handlers: Those handlers to be called upon event.
//
// Initialize the admin submission and completion queues.
// Initialize the asynchronous events notification queues.
//
// @return - 0 on success, negative value on failure.
//
// ena_com_admin_destroy - Destroy the admin and the async events queues.
// @ena_dev: ENA communication layer struct
//
// @note: Before calling this method, the caller must validate that the device
// won't send any additional admin completions/aenq.
// To achieve that, a FLR is recommended.
//
extern "C" {
    pub fn ena_com_admin_destroy(ena_dev: *mut ena_com_dev);
}
// ena_com_dev_reset - Perform device FLR to the device.
// @ena_dev: ENA communication layer struct
// @reset_reason: Specify what is the trigger for the reset in case of an error.
//
// @return - 0 on success, negative value on failure.
//
// ena_com_create_io_queue - Create io queue.
// @ena_dev: ENA communication layer struct
// @ctx - create context structure
//
// Create the submission and the completion queues.
//
// @return - 0 on success, negative value on failure.
//
// ena_com_destroy_io_queue - Destroy IO queue with the queue id - qid.
// @ena_dev: ENA communication layer struct
// @qid - the caller virtual queue id.
//
extern "C" {
    pub fn ena_com_destroy_io_queue(ena_dev: *mut ena_com_dev, qid: u16);
}
// ena_com_get_io_handlers - Return the io queue handlers
// @ena_dev: ENA communication layer struct
// @qid - the caller virtual queue id.
// @io_sq - IO submission queue handler
// @io_cq - IO completion queue handler.
//
// @return - 0 on success, negative value on failure.
//
// ena_com_admin_aenq_enable - ENAble asynchronous event notifications
// @ena_dev: ENA communication layer struct
//
// After this method, aenq event can be received via AENQ.
//
extern "C" {
    pub fn ena_com_admin_aenq_enable(ena_dev: *mut ena_com_dev);
}
// ena_com_set_admin_running_state - Set the state of the admin queue
// @ena_dev: ENA communication layer struct
//
// Change the state of the admin queue (enable/disable)
//
extern "C" {
    pub fn ena_com_set_admin_running_state(ena_dev: *mut ena_com_dev, state: bool);
}
// ena_com_get_admin_running_state - Get the admin queue state
// @ena_dev: ENA communication layer struct
//
// Retrieve the state of the admin queue (enable/disable)
//
// @return - current polling mode (enable/disable)
//
extern "C" {
    pub fn ena_com_get_admin_running_state(ena_dev: *mut ena_com_dev) -> bool;
}
// ena_com_set_admin_polling_mode - Set the admin completion queue polling mode
// @ena_dev: ENA communication layer struct
// @polling: ENAble/Disable polling mode
//
// Set the admin completion mode.
//
extern "C" {
    pub fn ena_com_set_admin_polling_mode(ena_dev: *mut ena_com_dev, polling: bool);
}
// ena_com_admin_q_comp_intr_handler - admin queue interrupt handler
// @ena_dev: ENA communication layer struct
//
// This method goes over the admin completion queue and wakes up all the pending
// threads that wait on the commands wait event.
//
// @note: Should be called after MSI-X interrupt.
//
extern "C" {
    pub fn ena_com_admin_q_comp_intr_handler(ena_dev: *mut ena_com_dev);
}
// ena_com_aenq_intr_handler - AENQ interrupt handler
// @ena_dev: ENA communication layer struct
//
// This method goes over the async event notification queue and calls the proper
// aenq handler.
//
extern "C" {
    pub fn ena_com_aenq_intr_handler(ena_dev: *mut ena_com_dev, data: *mut c_void);
}
// ena_com_abort_admin_commands - Abort all the outstanding admin commands.
// @ena_dev: ENA communication layer struct
//
// This method aborts all the outstanding admin commands.
// The caller should then call ena_com_wait_for_abort_completion to make sure
// all the commands were completed.
//
extern "C" {
    pub fn ena_com_abort_admin_commands(ena_dev: *mut ena_com_dev);
}
// ena_com_wait_for_abort_completion - Wait for admin commands abort.
// @ena_dev: ENA communication layer struct
//
// This method waits until all the outstanding admin commands are completed.
//
extern "C" {
    pub fn ena_com_wait_for_abort_completion(ena_dev: *mut ena_com_dev);
}
// ena_com_validate_version - Validate the device parameters
// @ena_dev: ENA communication layer struct
//
// This method verifies the device parameters are the same as the saved
// parameters in ena_dev.
// This method is useful after device reset, to validate the device mac address
// and the device offloads are the same as before the reset.
//
// @return - 0 on success negative value otherwise.
//
extern "C" {
    pub fn ena_com_validate_version(ena_dev: *mut ena_com_dev) -> c_int;
}
// ena_com_get_link_params - Retrieve physical link parameters.
// @ena_dev: ENA communication layer struct
// @resp: Link parameters
//
// Retrieve the physical link parameters,
// like speed, auto-negotiation and full duplex support.
//
// @return - 0 on Success negative value otherwise.
//
// ena_com_get_dma_width - Retrieve physical dma address width the device
// supports.
// @ena_dev: ENA communication layer struct
//
// Retrieve the maximum physical address bits the device can handle.
//
// @return: > 0 on Success and negative value otherwise.
//
extern "C" {
    pub fn ena_com_get_dma_width(ena_dev: *mut ena_com_dev) -> c_int;
}
// ena_com_set_aenq_config - Set aenq groups configurations
// @ena_dev: ENA communication layer struct
// @groups flag: bit fields flags of enum ena_admin_aenq_group.
//
// Configure which aenq event group the driver would like to receive.
//
// @return: 0 on Success and negative value otherwise.
//
extern "C" {
    pub fn ena_com_set_aenq_config(ena_dev: *mut ena_com_dev, groups_flag: u32) -> c_int;
}
// ena_com_get_dev_attr_feat - Get device features
// @ena_dev: ENA communication layer struct
// @get_feat_ctx: returned context that contain the get features.
//
// @return: 0 on Success and negative value otherwise.
//
// ena_com_get_eni_stats - Get extended network interface statistics
// @ena_dev: ENA communication layer struct
// @stats: stats return value
//
// @return: 0 on Success and negative value otherwise.
//
// ena_com_get_ena_srd_info - Get ENA SRD network interface statistics
// @ena_dev: ENA communication layer struct
// @info: ena srd stats and flags
//
// @return: 0 on Success and negative value otherwise.
//
// ena_com_get_customer_metrics - Get customer metrics for network interface
// @ena_dev: ENA communication layer struct
// @buffer: buffer for returned customer metrics
// @len: size of the buffer
//
// @return: 0 on Success and negative value otherwise.
//
extern "C" {
    pub fn ena_com_get_customer_metrics(ena_dev: *mut ena_com_dev, buffer: *mut c_char, len: u32) -> c_int;
}
// ena_com_set_dev_mtu - Configure the device mtu.
// @ena_dev: ENA communication layer struct
// @mtu: mtu value
//
// @return: 0 on Success and negative value otherwise.
//
extern "C" {
    pub fn ena_com_set_dev_mtu(ena_dev: *mut ena_com_dev, mtu: u32) -> c_int;
}
// ena_com_rss_init - Init RSS
// @ena_dev: ENA communication layer struct
// @log_size: indirection log size
//
// Allocate RSS/RFS resources.
// The caller then can configure rss using ena_com_set_hash_function,
// ena_com_set_hash_ctrl and ena_com_indirect_table_set.
//
// @return: 0 on Success and negative value otherwise.
//
extern "C" {
    pub fn ena_com_rss_init(ena_dev: *mut ena_com_dev, log_size: u16) -> c_int;
}
// ena_com_rss_destroy - Destroy rss
// @ena_dev: ENA communication layer struct
//
// Free all the RSS/RFS resources.
//
extern "C" {
    pub fn ena_com_rss_destroy(ena_dev: *mut ena_com_dev);
}
// ena_com_get_current_hash_function - Get RSS hash function
// @ena_dev: ENA communication layer struct
//
// Return the current hash function.
// @return: 0 or one of the ena_admin_hash_functions values.
//
extern "C" {
    pub fn ena_com_get_current_hash_function(ena_dev: *mut ena_com_dev) -> c_int;
}
// ena_com_fill_hash_function - Fill RSS hash function
// @ena_dev: ENA communication layer struct
// @func: The hash function (Toeplitz or crc)
// @key: Hash key (for toeplitz hash)
// @key_len: key length (max length 10 DW)
// @init_val: initial value for the hash function
//
// Fill the ena_dev resources with the desire hash function, hash key, key_len
// and key initial value (if needed by the hash function).
// To flush the key into the device the caller should call
// ena_com_set_hash_function.
//
// @return: 0 on Success and negative value otherwise.
//
// ena_com_set_hash_function - Flush the hash function and it dependencies to
// the device.
// @ena_dev: ENA communication layer struct
//
// Flush the hash function and it dependencies (key, key length and
// initial value) if needed.
//
// @note: Prior to this method the caller should call ena_com_fill_hash_function
//
// @return: 0 on Success and negative value otherwise.
//
extern "C" {
    pub fn ena_com_set_hash_function(ena_dev: *mut ena_com_dev) -> c_int;
}
// ena_com_get_hash_function - Retrieve the hash function from the device.
// @ena_dev: ENA communication layer struct
// @func: hash function
//
// Retrieve the hash function from the device.
//
// @note: If the caller called ena_com_fill_hash_function but didn't flush
// it to the device, the new configuration will be lost.
//
// @return: 0 on Success and negative value otherwise.
//
// ena_com_get_hash_key - Retrieve the hash key
// @ena_dev: ENA communication layer struct
// @key: hash key
//
// Retrieve the hash key.
//
// @note: If the caller called ena_com_fill_hash_key but didn't flush
// it to the device, the new configuration will be lost.
//
// @return: 0 on Success and negative value otherwise.
//
extern "C" {
    pub fn ena_com_get_hash_key(ena_dev: *mut ena_com_dev, key: *mut u8) -> c_int;
}
// ena_com_fill_hash_ctrl - Fill RSS hash control
// @ena_dev: ENA communication layer struct.
// @proto: The protocol to configure.
// @hash_fields: bit mask of ena_admin_flow_hash_fields
//
// Fill the ena_dev resources with the desire hash control (the ethernet
// fields that take part of the hash) for a specific protocol.
// To flush the hash control to the device, the caller should call
// ena_com_set_hash_ctrl.
//
// @return: 0 on Success and negative value otherwise.
//
// ena_com_set_hash_ctrl - Flush the hash control resources to the device.
// @ena_dev: ENA communication layer struct
//
// Flush the hash control (the ethernet fields that take part of the hash)
//
// @note: Prior to this method the caller should call ena_com_fill_hash_ctrl.
//
// @return: 0 on Success and negative value otherwise.
//
extern "C" {
    pub fn ena_com_set_hash_ctrl(ena_dev: *mut ena_com_dev) -> c_int;
}
// ena_com_get_hash_ctrl - Retrieve the hash control from the device.
// @ena_dev: ENA communication layer struct
// @proto: The protocol to retrieve.
// @fields: bit mask of ena_admin_flow_hash_fields.
//
// Retrieve the hash control from the device.
//
// @note: If the caller called ena_com_fill_hash_ctrl but didn't flush
// it to the device, the new configuration will be lost.
//
// @return: 0 on Success and negative value otherwise.
//
// ena_com_set_default_hash_ctrl - Set the hash control to a default
// configuration.
// @ena_dev: ENA communication layer struct
//
// Fill the ena_dev resources with the default hash control configuration.
// To flush the hash control to the device, the caller should call
// ena_com_set_hash_ctrl.
//
// @return: 0 on Success and negative value otherwise.
//
extern "C" {
    pub fn ena_com_set_default_hash_ctrl(ena_dev: *mut ena_com_dev) -> c_int;
}
// ena_com_indirect_table_fill_entry - Fill a single entry in the RSS
// indirection table
// @ena_dev: ENA communication layer struct.
// @entry_idx - indirection table entry.
// @entry_value - redirection value
//
// Fill a single entry of the RSS indirection table in the ena_dev resources.
// To flush the indirection table to the device, the called should call
// ena_com_indirect_table_set.
//
// @return: 0 on Success and negative value otherwise.
//
// ena_com_indirect_table_set - Flush the indirection table to the device.
// @ena_dev: ENA communication layer struct
//
// Flush the indirection hash control to the device.
// Prior to this method the caller should call ena_com_indirect_table_fill_entry
//
// @return: 0 on Success and negative value otherwise.
//
extern "C" {
    pub fn ena_com_indirect_table_set(ena_dev: *mut ena_com_dev) -> c_int;
}
// ena_com_indirect_table_get - Retrieve the indirection table from the device.
// @ena_dev: ENA communication layer struct
// @ind_tbl: indirection table
//
// Retrieve the RSS indirection table from the device.
//
// @note: If the caller called ena_com_indirect_table_fill_entry but didn't flush
// it to the device, the new configuration will be lost.
//
// @return: 0 on Success and negative value otherwise.
//
extern "C" {
    pub fn ena_com_indirect_table_get(ena_dev: *mut ena_com_dev, ind_tbl: *mut u32) -> c_int;
}
// ena_com_allocate_host_info - Allocate host info resources.
// @ena_dev: ENA communication layer struct
//
// @return: 0 on Success and negative value otherwise.
//
extern "C" {
    pub fn ena_com_allocate_host_info(ena_dev: *mut ena_com_dev) -> c_int;
}
// ena_com_allocate_debug_area - Allocate debug area.
// @ena_dev: ENA communication layer struct
// @debug_area_size - debug area size.
//
// @return: 0 on Success and negative value otherwise.
//
// ena_com_allocate_customer_metrics_buffer - Allocate customer metrics resources.
// @ena_dev: ENA communication layer struct
//
// @return: 0 on Success and negative value otherwise.
//
extern "C" {
    pub fn ena_com_allocate_customer_metrics_buffer(ena_dev: *mut ena_com_dev) -> c_int;
}
// ena_com_delete_debug_area - Free the debug area resources.
// @ena_dev: ENA communication layer struct
//
// Free the allocated debug area.
//
extern "C" {
    pub fn ena_com_delete_debug_area(ena_dev: *mut ena_com_dev);
}
// ena_com_delete_host_info - Free the host info resources.
// @ena_dev: ENA communication layer struct
//
// Free the allocated host info.
//
extern "C" {
    pub fn ena_com_delete_host_info(ena_dev: *mut ena_com_dev);
}
// ena_com_delete_customer_metrics_buffer - Free the customer metrics resources.
// @ena_dev: ENA communication layer struct
//
// Free the allocated customer metrics area.
//
extern "C" {
    pub fn ena_com_delete_customer_metrics_buffer(ena_dev: *mut ena_com_dev);
}
// ena_com_set_host_attributes - Update the device with the host
// attributes (debug area and host info) base address.
// @ena_dev: ENA communication layer struct
//
// @return: 0 on Success and negative value otherwise.
//
extern "C" {
    pub fn ena_com_set_host_attributes(ena_dev: *mut ena_com_dev) -> c_int;
}
// ena_com_create_io_cq - Create io completion queue.
// @ena_dev: ENA communication layer struct
// @io_cq - io completion queue handler
// Create IO completion queue.
//
// @return - 0 on success, negative value on failure.
//
// ena_com_destroy_io_cq - Destroy io completion queue.
// @ena_dev: ENA communication layer struct
// @io_cq - io completion queue handler
// Destroy IO completion queue.
//
// @return - 0 on success, negative value on failure.
//
// ena_com_execute_admin_command - Execute admin command
// @admin_queue: admin queue.
// @cmd: the admin command to execute.
// @cmd_size: the command size.
// @cmd_completion: command completion return value.
// @cmd_comp_size: command completion size.
// Submit an admin command and then wait until the device returns a
// completion.
// The completion will be copied into cmd_comp.
//
// @return - 0 on success, negative value on failure.
//
// ena_com_init_interrupt_moderation - Init interrupt moderation
// @ena_dev: ENA communication layer struct
//
// @return - 0 on success, negative value on failure.
//
extern "C" {
    pub fn ena_com_init_interrupt_moderation(ena_dev: *mut ena_com_dev) -> c_int;
}
// ena_com_interrupt_moderation_supported - Return if interrupt moderation
// capability is supported by the device.
//
// @return - supported or not.
//
extern "C" {
    pub fn ena_com_interrupt_moderation_supported(ena_dev: *mut ena_com_dev) -> bool;
}
// ena_com_update_nonadaptive_moderation_interval_tx - Update the
// non-adaptive interval in Tx direction.
// @ena_dev: ENA communication layer struct
// @tx_coalesce_usecs: Interval in usec.
//
// @return - 0 on success, negative value on failure.
//
// ena_com_update_nonadaptive_moderation_interval_rx - Update the
// non-adaptive interval in Rx direction.
// @ena_dev: ENA communication layer struct
// @rx_coalesce_usecs: Interval in usec.
//
// @return - 0 on success, negative value on failure.
//
// ena_com_get_nonadaptive_moderation_interval_tx - Retrieve the
// non-adaptive interval in Tx direction.
// @ena_dev: ENA communication layer struct
//
// @return - interval in usec
//
extern "C" {
    pub fn ena_com_get_nonadaptive_moderation_interval_tx(ena_dev: *mut ena_com_dev) -> c_uint;
}
// ena_com_get_nonadaptive_moderation_interval_rx - Retrieve the
// non-adaptive interval in Rx direction.
// @ena_dev: ENA communication layer struct
//
// @return - interval in usec
//
extern "C" {
    pub fn ena_com_get_nonadaptive_moderation_interval_rx(ena_dev: *mut ena_com_dev) -> c_uint;
}
// ena_com_config_dev_mode - Configure the placement policy of the device.
// @ena_dev: ENA communication layer struct
// @llq_features: LLQ feature descriptor, retrieve via
// ena_com_get_dev_attr_feat.
// @ena_llq_config: The default driver LLQ parameters configurations
//
// ena_com_io_sq_to_ena_dev - Extract ena_com_dev using contained field io_sq.
// @io_sq: IO submit queue struct
//
// @return - ena_com_dev struct extracted from io_sq
//
extern "C" {
    pub fn container_of(_arg: io_sq, ena_com_dev: struct, _arg: io_sq_queues[io_sq->qid]) -> return;
}
// ena_com_io_cq_to_ena_dev - Extract ena_com_dev using contained field io_cq.
// @io_sq: IO submit queue struct
//
// @return - ena_com_dev struct extracted from io_sq
//
extern "C" {
    pub fn container_of(_arg: io_cq, ena_com_dev: struct, _arg: io_cq_queues[io_cq->qid]) -> return;
}
// ena_com_get_cap - query whether device supports a capability.
// @ena_dev: ENA communication layer struct
// @cap_id: enum value representing the capability
//
// @return - true if capability is supported or false otherwise
//
// ena_com_get_customer_metric_support - query whether device supports a given customer metric.
// @ena_dev: ENA communication layer struct
// @metric_id: enum value representing the customer metric
//
// @return - true if customer metric is supported or false otherwise
//
// ena_com_get_customer_metric_count - return the number of supported customer metrics.
// @ena_dev: ENA communication layer struct
//
// @return - the number of supported customer metrics
//
extern "C" {
    pub fn hweight64(_arg: ena_dev->customer_metrics.supported_metrics) -> return;
}
// ena_com_update_intr_reg - Prepare interrupt register
// @intr_reg: interrupt register to update.
// @rx_delay_interval: Rx interval in usecs
// @tx_delay_interval: Tx interval in usecs
// @unmask: unmask enable/disable
//
// Prepare interrupt update register with the supplied parameters.
//
