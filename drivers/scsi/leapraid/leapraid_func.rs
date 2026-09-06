//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/leapraid/leapraid_func.h
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
// Copyright (C) 2026 LeapIO Tech Inc.
//
// LeapRAID storage and RAID controller driver.
//

// Macro flag: #define LEAPRAID_FUNC_H_INCLUDED

// Request and reply buffer size.
pub const LEAPRAID_REQUEST_SIZE: c_int = 128;
pub const LEAPRAID_REPLY_SIZE: c_int = 128;
pub const LEAPRAID_CHAIN_SEG_SIZE: c_int = 128;
pub const LEAPRAID_MAX_SGES_IN_CHAIN: c_int = 7;
pub const LEAPRAID_DEFAULT_CHAINS_PER_IO: c_int = 19;

pub const LEAPRAID_IEEE_SGE64_ENTRY_SIZE: c_int = 16;
pub const LEAPRAID_REP_DESC_CHUNK_SIZE: c_int = 16;
pub const LEAPRAID_REP_DESC_ENTRY_SIZE: c_int = 8;
pub const LEAPRAID_REP_MSG_ADDR_SIZE: c_int = 4;
pub const LEAPRAID_REP_RQ_CNT_SIZE: c_int = 16;
pub const LEAPRAID_SYS_LOG_BUF_SIZE: c_uint = 0x200000;
pub const LEAPRAID_SYS_LOG_BUF_RESERVE: c_uint = 0x1000;
// Driver version and name.

pub const LEAPRAID_NAME_LENGTH: c_int = 48;
pub const LEAPRAID_BOARD_NAME_LENGTH: c_int = 17;

pub const LEAPRAID_MAJOR_VERSION: c_int = 2;
pub const LEAPRAID_MINOR_VERSION: c_int = 0;
pub const LEAPRAID_BUILD_VERSION: c_int = 1;
pub const LEAPRAID_RELEASE_VERSION: c_int = 9;
pub const LEAPRAID_MSG_VERSION: c_uint = 0x1021;
pub const LEAPRAID_HEADER_VERSION: c_uint = 0x0000;
// Device ID.
pub const LEAPRAID_VENDOR_ID: c_uint = 0xD405;
pub const LEAPRAID_DEVID_HBA: c_uint = 0x8200;
pub const LEAPRAID_SUBVENDOR_ID: c_uint = 0xD405;
pub const LEAPRAID_SUBDEVID_HBA: c_uint = 0x8200;
pub const LEAPRAID_PCI_VENDOR_ID_MASK: c_uint = 0xFFFF;
// RAID virtual channel ID.
pub const RAID_CHANNEL: c_int = 1;
// Scatter-gather (SG) segment limits.

pub const LEAPRAID_KDUMP_MIN_PHYS_SEGMENTS: c_int = 32;

// Firmware and config page operations.
pub const LEAPRAID_CFG_REQ_RETRY_TIMES: c_int = 2;
// Hardware access helpers.

// Polling intervals.
pub const LEAPRAID_PCIE_LOG_POLLING_INTERVAL: c_int = 1;
pub const LEAPRAID_FAULT_POLLING_INTERVAL: c_int = 1000;
// Init mask.
pub const LEAPRAID_RESET_IRQ_MASK: c_uint = 0x40000000;
pub const LEAPRAID_REPLY_INT_MASK: c_uint = 0x00000008;
pub const LEAPRAID_TO_SYS_DB_MASK: c_uint = 0x00000001;
// Queue depth.
pub const LEAPRAID_SATA_QUEUE_DEPTH: c_int = 32;
pub const LEAPRAID_SAS_QUEUE_DEPTH: c_int = 64;
pub const LEAPRAID_RAID_QUEUE_DEPTH: c_int = 64;
// Target probe flag.
pub const LEAPRAID_NO_ULD_ATTACH_FLAG: c_int = 1;
// SCSI device and queue limits.
pub const LEAPRAID_MAX_SECTORS: c_int = 2048;
pub const LEAPRAID_MAX_CDB_LEN: c_int = 32;
pub const LEAPRAID_MAX_LUNS: c_int = 16384;
pub const LEAPRAID_CAN_QUEUE_MIN: c_int = 1;

pub const LEAPRAID_CMD_PER_LUN: c_int = 128;
pub const LEAPRAID_MAX_SEGMENT_SIZE: c_uint = 0xffffffff;
// SCSI sense and ASC/ASCQ and disk geometry configuration.
pub const DESC_FORMAT_THRESHOLD: c_uint = 0x72;
pub const SENSE_KEY_MASK: c_uint = 0x0F;
pub const SCSI_SENSE_RESPONSE_CODE_MASK: c_uint = 0x7F;
pub const ASC_FAILURE_PREDICTION_THRESHOLD_EXCEEDED: c_uint = 0x5D;
pub const LEAPRAID_LARGE_DISK_THRESHOLD: c_uint = 0x200000UL;
pub const LEAPRAID_LARGE_DISK_HEADS: c_int = 255;
pub const LEAPRAID_LARGE_DISK_SECTORS: c_int = 63;
pub const LEAPRAID_SMALL_DISK_HEADS: c_int = 64;
pub const LEAPRAID_SMALL_DISK_SECTORS: c_int = 32;
// SMP (Serial Management Protocol).
pub const LEAPRAID_SMP_PT_FLAG_SGL_PTR: c_uint = 0x80;
pub const LEAPRAID_SMP_FRAME_HEADER_SIZE: c_int = 4;
pub const LEAPRAID_SCSI_HOST_SHIFT: c_int = 16;
pub const LEAPRAID_SCSI_DRIVER_SHIFT: c_int = 24;
// SCSI ASC/ASCQ definitions.
pub const LEAPRAID_SCSI_ASCQ_DEFAULT: c_uint = 0x00;
pub const LEAPRAID_SCSI_ASC_POWER_ON_RESET: c_uint = 0x29;
pub const LEAPRAID_SCSI_ASC_INVALID_CMD_CODE: c_uint = 0x20;
pub const LEAPRAID_SCSI_ASCQ_POWER_ON_RESET: c_uint = 0x07;
// VPD Page 0x89 (ATA Information).
pub const LEAPRAID_VPD_PAGE_ATA_INFO: c_uint = 0x89;
pub const LEAPRAID_VPD_PG89_MAX_LEN: c_int = 255;
pub const LEAPRAID_VPD_PG89_MIN_LEN: c_int = 214;
// Byte index for NCQ support flag in VPD page 0x89.
pub const LEAPRAID_VPD_PG89_NCQ_BYTE_IDX: c_int = 213;
pub const LEAPRAID_VPD_PG89_NCQ_BIT_SHIFT: c_int = 4;
pub const LEAPRAID_VPD_PG89_NCQ_BIT_MASK: c_uint = 0x1;
// Readiness polling: max retries, sleep µs between.
pub const LEAPRAID_ADAPTER_READY_MAX_RETRY: c_int = 15000;
pub const LEAPRAID_ADAPTER_READY_SLEEP_MIN_US: c_int = 1000;
pub const LEAPRAID_ADAPTER_READY_SLEEP_MAX_US: c_int = 1100;
// Doorbell wait parameters.
pub const LEAPRAID_DB_WAIT_MAX_RETRY: c_int = 20000;
pub const LEAPRAID_DB_WAIT_DELAY_US: c_int = 500;
// Basic data size definitions.
pub const LEAPRAID_DWORDS_BYTE_SIZE: c_int = 4;
pub const LEAPRAID_WORD_BYTE_SIZE: c_int = 2;
// SGL threshold and chain offset.
pub const LEAPRAID_SGL_INLINE_THRESHOLD: c_int = 2;
pub const LEAPRAID_CHAIN_OFFSET_DWORDS: c_int = 7;
// MSI-X group size and mask.
pub const LEAPRAID_MSIX_GROUP_SIZE: c_int = 8;
pub const LEAPRAID_MSIX_GROUP_MASK: c_int = 7;
// Basic constants and limits.
pub const LEAPRAID_BUSY_LIMIT: c_int = 1;
pub const LEAPRAID_INVALID_HOST_DIAG_VAL: c_uint = 0xFFFFFFFF;
// Retry/Sleep configuration.
pub const LEAPRAID_WRSEQ_FLUSH_KEY_VALUE: c_uint = 0x0;
pub const LEAPRAID_WRSEQ_1ST_KEY_VALUE: c_uint = 0xF;
pub const LEAPRAID_WRSEQ_2ND_KEY_VALUE: c_uint = 0x4;
pub const LEAPRAID_WRSEQ_3RD_KEY_VALUE: c_uint = 0xB;
pub const LEAPRAID_WRSEQ_4TH_KEY_VALUE: c_uint = 0x2;
pub const LEAPRAID_WRSEQ_5TH_KEY_VALUE: c_uint = 0x7;
pub const LEAPRAID_WRSEQ_6TH_KEY_VALUE: c_uint = 0xD;
pub const LEAPRAID_UNLOCK_RETRY_LIMIT: c_int = 20;
pub const LEAPRAID_UNLOCK_SLEEP_MS: c_int = 100;
pub const LEAPRAID_MSLEEP_NORMAL_MS: c_int = 100;
pub const LEAPRAID_MSLEEP_EXTRA_LONG_MS: c_int = 500;
pub const LEAPRAID_IO_POLL_DELAY_US: c_int = 500;
// Device/Volume configuration.
pub const LEAPRAID_MAX_VOLUMES_DEFAULT: c_int = 32;
pub const LEAPRAID_MAX_DEV_HANDLE_DEFAULT: c_int = 2048;
pub const LEAPRAID_INVALID_DEV_HANDLE: c_uint = 0xFFFF;
// Commands queue depth.
pub const LEAPRAID_DEFAULT_CMD_QD_OFFSET: c_int = 64;
pub const LEAPRAID_REPLY_QD_ALIGNMENT: c_int = 16;
// Task ID offset.
pub const LEAPRAID_TASKID_OFFSET_CTRL_CMD: c_int = 1;
pub const LEAPRAID_TASKID_OFFSET_CFG_OP_CMD: c_int = 1;
pub const LEAPRAID_TASKID_OFFSET_TRANSPORT_CMD: c_int = 2;
pub const LEAPRAID_TASKID_OFFSET_ENC_CMD: c_int = 3;
pub const LEAPRAID_TASKID_OFFSET_NOTIFY_EVENT_CMD: c_int = 4;
// Task ID offset for high-priority.
pub const LEAPRAID_HP_TASKID_OFFSET_CTL_CMD: c_int = 0;
pub const LEAPRAID_HP_TASKID_OFFSET_TM_CMD: c_int = 1;
// Event/Boot configuration.
pub const LEAPRAID_EVT_MASK_COUNT: c_int = 4;
pub const LEAPRAID_BOOT_DEV_SIZE: c_int = 24;
// Commands timeout.
pub const LEAPRAID_UNIFIED_TIMEOUT: c_int = 30;

pub const LEAPRAID_SCAN_DEV_CMD_TIMEOUT: c_int = 300;

// Host DMA cap.
pub const DMA_32_BITS: c_int = 32;
pub const DMA_64_BITS: c_int = 64;
pub const LEAPRAID_DMA_ALIGN: c_int = 16;
// Values used to represent an invalid.

// Version shift and mask.
pub const LEAPRAID_VER_MAJOR_SHIFT: c_int = 24;
pub const LEAPRAID_VER_MINOR_SHIFT: c_int = 16;
pub const LEAPRAID_VER_BUILD_SHIFT: c_int = 8;
pub const LEAPRAID_VER_MASK: c_uint = 0xFF;
// Interrupt type.
pub const LEAPRAID_INTERRUPT_MODE_MSIX: c_int = 0;
pub const LEAPRAID_INTERRUPT_MODE_MSI: c_int = 1;
pub const LEAPRAID_INTERRUPT_MODE_LEGACY: c_int = 2;
// SMP link reset.
pub const SMP_PHY_CONTROL_LINK_RESET: c_uint = 0x01;
pub const SMP_PHY_CONTROL_HARD_RESET: c_uint = 0x02;
pub const SMP_PHY_CONTROL_DISABLE: c_uint = 0x03;
//
// struct leapraid_adapter_features - Adapter features/capabilities.
//
// @req_slot: Number of request slots supported by the adapter.
// @hp_slot: Number of high-priority slots supported by the adapter.
// @adapter_caps: Adapter capabilities.
// @fw_version: Firmware version of the adapter.
// @max_dev_handle: Maximum device handle supported by the adapter.
// @min_dev_handle: Minimum device handle supported by the adapter.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_adapter_features {
    pub req_slot: u16,
    pub hp_slot: u16,
    pub adapter_caps: u32,
    pub fw_version: u32,
    pub max_msix_vectors: u8,
    pub max_volumes: u8,
    pub max_dev_handle: u16,
    pub min_dev_handle: u16,
    pub msg_ver: u16,
    pub product_id: u16,
}

//
// struct leapraid_adapter_attr - Adapter attributes and capabilities
//
// @id: Adapter identifier.
// @raid_support: Indicates if RAID is supported.
// @bios_version: Version of the adapter BIOS.
// @enable_mp: Indicates if multipath (MP) support is enabled.
// @wideport_max_queue_depth: Maximum queue depth for wide ports.
// @narrowport_max_queue_depth: Maximum queue depth for narrow ports.
// @sata_max_queue_depth: Maximum queue depth for SATA.
// @raid_volume_max_queue_depth: Maximum queue depth for RAID volumes.
// @features: Detailed features of the adapter.
// @adapter_total_qd: Total queue depth available on the adapter.
// @io_qd: Queue depth allocated for I/O operations.
// @rep_msg_qd: Queue depth for reply messages.
// @rep_desc_qd: Queue depth for reply descriptors.
// @rep_desc_q_seg_cnt: Number of segments in a reply descriptor queue.
// @rq_cnt: Number of request queues.
// @task_desc_dma_size: Size of task descriptor DMA memory.
// @use_32_dma_mask: Indicates if 32-bit DMA mask is used.
// @name: Adapter name string.
// @board_name: Board name retrieved from manufacturing page 0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_adapter_attr {
    pub id: u8,
    pub raid_support: u8,
    pub bios_version: u32,
    pub enable_mp: u8,
    pub wideport_max_queue_depth: u32,
    pub narrowport_max_queue_depth: u32,
    pub sata_max_queue_depth: u32,
    pub raid_volume_max_queue_depth: u32,
    pub features: leapraid_adapter_features,
    pub adapter_total_qd: u32,
    pub io_qd: u32,
    pub rep_msg_qd: u32,
    pub rep_desc_qd: u32,
    pub rep_desc_q_seg_cnt: u32,
    pub rq_cnt: u16,
    pub task_desc_dma_size: u32,
    pub use_32_dma_mask: u8,
    pub name: [c_char; LEAPRAID_NAME_LENGTH],
    pub board_name: [c_char; LEAPRAID_BOARD_NAME_LENGTH],
}

//
// struct leapraid_io_req_tracker - Track a SCSI I/O request for the adapter
//
// @taskid: Unique task ID for this I/O request.
// @scmd: Pointer to the associated SCSI command.
// @chain_list: List of chain frames associated with this request.
// @msix_io: MSI-X vector assigned to this I/O request.
// @chain: Pointer to the chain memory for this request.
// @chain_dma: DMA address of the chain memory.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_io_req_tracker {
    pub taskid: u16,
    pub scmd: *mut scsi_cmnd,
    pub chain_list: list_head,
    pub msix_io: u16,
    pub chain: *mut c_void,
    pub chain_dma: dma_addr_t,
}

//
// struct leapraid_task_tracker - Tracks a task in the adapter
//
// @taskid: Unique task ID for this tracker.
// @cb_idx: Callback index associated with this task.
// @tracker_list: Linked list node to chain this tracker in lists.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_task_tracker {
    pub taskid: u16,
    pub cb_idx: u8,
    pub tracker_list: list_head,
}

//
// struct leapraid_rep_desc_maint - Maintains reply descriptor memory
//
// @rep_desc: Pointer to the reply descriptor.
// @rep_desc_dma: DMA address of the reply descriptor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_rep_desc_maint {
    pub rep_desc: *mut leapraid_rep_desc_union,
    pub rep_desc_dma: dma_addr_t,
}

//
// struct leapraid_rep_desc_seg_maint -
// Maintains reply descriptor segment memory
//
// @rep_desc_seg: Pointer to the reply descriptor segment.
// @rep_desc_seg_dma: DMA address of the reply descriptor segment.
// @rep_desc_maint: Pointer to the main reply descriptor structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_rep_desc_seg_maint {
    pub rep_desc_seg: *mut c_void,
    pub rep_desc_seg_dma: dma_addr_t,
    pub rep_desc_maint: *mut leapraid_rep_desc_maint,
}

//
// struct leapraid_mem_desc - Memory descriptor for LeapRAID adapter
//
// @task_desc: Pointer to task descriptor.
// @task_desc_dma: DMA address of task descriptor.
// @sg_chain_pool: DMA pool for SGL chain allocations.
// @sg_chain_pool_size: Size of the sg_chain_pool.
// @taskid_to_uniq_tag: Mapping from task ID to unique tag.
// @sense_data: Buffer for SCSI sense data.
// @sense_data_dma: DMA address of sense_data buffer.
// @rep_msg: Buffer for reply message.
// @rep_msg_dma: DMA address of reply message buffer.
// @rep_msg_addr: Pointer to reply message address.
// @rep_msg_addr_dma: DMA address of reply message address.
// @rep_desc_seg_maint: Pointer to reply descriptor segment.
// @rep_desc_q_arr: Pointer to reply descriptor queue array.
// @rep_desc_q_arr_dma: DMA address of reply descriptor queue array.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_mem_desc {
    pub task_desc: *mut c_void,
    pub task_desc_dma: dma_addr_t,
    pub sg_chain_pool: *mut dma_pool,
    pub sg_chain_pool_size: u16,
    pub taskid_to_uniq_tag: *mut u16,
    pub sense_data: *mut u8,
    pub sense_data_dma: dma_addr_t,
    pub rep_msg: *mut u8,
    pub rep_msg_dma: dma_addr_t,
    pub rep_msg_addr: *mut __le32,
    pub rep_msg_addr_dma: dma_addr_t,
    pub rep_desc_seg_maint: *mut leapraid_rep_desc_seg_maint,
    pub rep_desc_q_arr: *mut leapraid_rep_desc_q_arr,
    pub rep_desc_q_arr_dma: dma_addr_t,
}

// internal cmd description
pub const LEAPRAID_FIXED_INTER_CMDS: c_int = 5;
pub const LEAPRAID_FIXED_HP_CMDS: c_int = 2;
pub const LEAPRAID_CMD_NOT_USED: c_uint = 0x8000;
pub const LEAPRAID_CMD_DONE: c_uint = 0x0001;
pub const LEAPRAID_CMD_PENDING: c_uint = 0x0002;
pub const LEAPRAID_CMD_REPLY_VALID: c_uint = 0x0004;
pub const LEAPRAID_CMD_RESET: c_uint = 0x0008;
//
// enum LEAPRAID_CB_INDEX - Callback index for LeapRAID driver
//
// @LEAPRAID_SCAN_DEV_CB_IDX: Scan device callback index.
// @LEAPRAID_CONFIG_CB_IDX: Configuration callback index.
// @LEAPRAID_TRANSPORT_CB_IDX: Transport callback index.
// @LEAPRAID_SAS_CTRL_CB_IDX: SAS controller callback index.
// @LEAPRAID_ENC_CB_IDX: Encryption callback index.
// @LEAPRAID_NOTIFY_EVENT_CB_IDX: Notify event callback index.
// @LEAPRAID_CTL_CB_IDX: Control callback index.
// @LEAPRAID_TM_CB_IDX: Task management callback index.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LEAPRAID_CB_INDEX {
    LEAPRAID_SCAN_DEV_CB_IDX	= 0x1,
    LEAPRAID_CONFIG_CB_IDX		= 0x2,
    LEAPRAID_TRANSPORT_CB_IDX	= 0x3,
    LEAPRAID_SAS_CTRL_CB_IDX	= 0x5,
    LEAPRAID_ENC_CB_IDX		= 0x6,
    LEAPRAID_NOTIFY_EVENT_CB_IDX	= 0x7,
    LEAPRAID_CTL_CB_IDX		= 0x8,
    LEAPRAID_TM_CB_IDX		= 0x9,
    LEAPRAID_NUM_CB_IDXS
}

//
// struct leapraid_default_reply - Default reply frame buffer
// @pad: Raw reply data buffer returned by firmware.
//
// This structure represents a generic reply frame used by the firmware
// when no specific reply format is required. The buffer size is defined
// by LEAPRAID_REPLY_SIZE and stores the raw reply data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_default_reply {
    pub pad: [u8; LEAPRAID_REPLY_SIZE],
}

//
// struct leapraid_sense_buffer - SCSI sense data buffer
// @pad: Buffer used to store sense data returned by a SCSI command.
//
// This buffer holds the sense data provided by a device.The size is
// defined by SCSI_SENSE_BUFFERSIZE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_sense_buffer {
    pub pad: [u8; SCSI_SENSE_BUFFERSIZE],
}

//
// struct leapraid_driver_cmd - Driver command tracking structure
//
// @reply: Default reply structure returned by the adapter.
// @done: Completion object used to signal command completion.
// @status: Status code returned by the firmware.
// @taskid: Unique task identifier for this command.
// @hp_taskid: Task identifier for high-priority commands.
// @inter_taskid: Task identifier for internal commands.
// @cb_idx: Callback index used to identify completion context.
// @async_scan_dev: True if this command is for asynchronous device scan.
// @sense: Sense buffer holding error information from device.
// @mutex: Mutex to protect access to this command structure.
// @list: List node for linking driver commands into lists.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_driver_cmd {
    pub reply: leapraid_default_reply,
    pub done: completion,
    pub status: u16,
    pub taskid: u16,
    pub hp_taskid: u16,
    pub inter_taskid: u16,
    pub cb_idx: u8,
    pub async_scan_dev: u8,
    pub sense: leapraid_sense_buffer,
    pub /: *mut *mut mutex mutex; / Mutex for driver cmd,
    pub list: list_head,
}

//
// struct leapraid_driver_cmds - Collection of driver command objects
//
// @special_cmd_list: List head for tracking special driver commands.
// @scan_dev_cmd: Command used for asynchronous device scan operations.
// @cfg_op_cmd: Command for configuration operations.
// @transport_cmd: Command for transport-level operations.
// @enc_cmd: Command for enclosure management operations.
// @notify_event_cmd: Command for asynchronous event notification handling.
// @ctl_cmd: Command for generic control or maintenance operations.
// @tm_cmd: Task management command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_driver_cmds {
    pub special_cmd_list: list_head,
    pub scan_dev_cmd: leapraid_driver_cmd,
    pub cfg_op_cmd: leapraid_driver_cmd,
    pub transport_cmd: leapraid_driver_cmd,
    pub enc_cmd: leapraid_driver_cmd,
    pub notify_event_cmd: leapraid_driver_cmd,
    pub ctl_cmd: leapraid_driver_cmd,
    pub tm_cmd: leapraid_driver_cmd,
}

//
// struct leapraid_dynamic_task_desc - Dynamic task descriptor
//
// @task_lock: Spinlock to protect concurrent access.
// @hp_taskid: Current high-priority task ID.
// @hp_cmd_qd: Fixed command queue depth for high-priority tasks.
// @inter_taskid: Current internal task ID.
// @inter_cmd_qd: Fixed command queue depth for internal tasks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_dynamic_task_desc {
    pub /: *mut *mut spinlock_t task_lock; / protects dynamic task,
    pub hp_taskid: u16,
    pub hp_cmd_qd: u16,
    pub inter_taskid: u16,
    pub inter_cmd_qd: u16,
}

//
// struct leapraid_fw_evt_work - Firmware event work structure
//
// @list: Linked list node for queuing the work.
// @adapter: Pointer to the associated LeapRAID adapter.
// @work: Work structure used by the kernel workqueue.
// @refcnt: Reference counter for managing the lifetime of this work.
// @evt_data: Pointer to firmware event data.
// @dev_handle: Device handle associated with the event.
// @evt_type: Type of firmware event.
// @ignore: Flag indicating whether the event should be ignored.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_fw_evt_work {
    pub list: list_head,
    pub adapter: *mut leapraid_adapter,
    pub work: work_struct,
    pub refcnt: kref,
    pub evt_data: *mut c_void,
    pub dev_handle: u16,
    pub evt_type: u16,
    pub ignore: u8,
}

//
// struct leapraid_fw_evt_struct - Firmware event handling structure
//
// @fw_evt_name: Name of the firmware event.
// @fw_evt_thread: Workqueue used for processing firmware events.
// @fw_evt_lock: Spinlock protecting access to the firmware event list.
// @fw_evt_list: Linked list of pending firmware events.
// @cur_evt: Pointer to the currently processing firmware event.
// @cur_evt_task: Task currently executing @cur_evt work.
// @fw_evt_cleanup: Flag indicating whether cleanup of events is in progress.
// @leapraid_evt_masks: Array of event masks for filtering firmware events.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_fw_evt_struct {
    pub leapraid_evt_masks: [u32; 4],
    pub fw_evt_name: [c_char; 48],
    pub fw_evt_thread: *mut workqueue_struct,
    pub /: *mut *mut spinlock_t fw_evt_lock; / protects firmware event,
    pub fw_evt_list: list_head,
    pub cur_evt: *mut leapraid_fw_evt_work,
    pub cur_evt_task: *mut task_struct,
    pub fw_evt_cleanup: u8,
}

//
// struct leapraid_rq - Represents a LeapRAID request queue
//
// @adapter: Pointer to the associated LeapRAID adapter.
// @msix_idx: MSI-X vector index used by this queue.
// @rep_post_host_idx: Index of the last processed reply descriptor.
// @rep_desc: Pointer to the reply descriptor associated with this queue.
// @name: Name of the request queue.
// @busy: Atomic counter indicating if the queue is busy.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_rq {
    pub adapter: *mut leapraid_adapter,
    pub msix_idx: u8,
    pub rep_post_host_idx: u32,
    pub rep_desc: *mut leapraid_rep_desc_union,
    pub name: [c_char; LEAPRAID_NAME_LENGTH],
    pub busy: core::sync::atomic::AtomicI32,
}

//
// struct leapraid_int_rq - Internal request queue for a CPU
//
// @affinity_hint: CPU affinity mask for the queue.
// @rq: Underlying LeapRAID request queue structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_int_rq {
    pub affinity_hint: cpumask_var_t,
    pub rq: leapraid_rq,
}

//
// struct leapraid_blk_mq_poll_rq - Polling request for LeapRAID blk-mq
//
// @busy: Atomic flag indicating request is being processed.
// @pause: Atomic flag to temporarily suspend polling.
// @rq: The underlying LeapRAID request structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_blk_mq_poll_rq {
    pub busy: core::sync::atomic::AtomicI32,
    pub pause: core::sync::atomic::AtomicI32,
    pub rq: leapraid_rq,
}

//
// struct leapraid_notification_desc - Notification descriptor for LeapRAID
//
// @iopoll_qdex: Index of the I/O polling queue.
// @iopoll_qcnt: Count of I/O polling queues.
// @msix_enable: Flag indicating MSI-X is enabled.
// @irq_vectors_allocated: Flag indicating PCI IRQ vectors are allocated.
// @irq_mode: Actual interrupt mode used for allocated PCI IRQ vectors.
// @msix_cpu_map: CPU-ID indexed MSI-X queue map.
// @msix_cpu_map_sz: Number of CPU-ID slots allocated in @msix_cpu_map.
// @int_rqs: Array of interrupt request queues.
// @int_rqs_allocated: Count of allocated interrupt request queues.
// @blk_mq_poll_rqs: Array of blk-mq polling requests.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_notification_desc {
    pub iopoll_qdex: u32,
    pub iopoll_qcnt: u32,
    pub msix_enable: u8,
    pub irq_vectors_allocated: u8,
    pub irq_mode: u8,
    pub msix_cpu_map: *mut u8,
    pub msix_cpu_map_sz: u32,
    pub int_rqs: *mut leapraid_int_rq,
    pub int_rqs_allocated: u32,
    pub blk_mq_poll_rqs: *mut leapraid_blk_mq_poll_rq,
}

//
// struct leapraid_overheat_desc - Overheat descriptor for LeapRAID
//
// @fault_overheat_wq: Workqueue for adapter thermal overheat operations.
// @fault_overheat_work: Work structure for thermal overheat.
// @thermal_alert: Flag indicating if adapter thermal alert is active.
// @fault_overheat_wq_name: Name of the fault overheat workqueue.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_overheat_desc {
    pub fault_overheat_wq: *mut workqueue_struct,
    pub fault_overheat_work: work_struct,
    pub thermal_alert: core::sync::atomic::AtomicI32,
    pub fault_overheat_wq_name: [c_char; 48],
}

//
// struct leapraid_reset_desc - Reset descriptor for LeapRAID
//
// @fault_reset_wq: Workqueue for fault reset operations.
// @fault_reset_work: Delayed work structure for fault reset.
// @fault_reset_wq_name: Name of the fault reset workqueue.
// @host_diag_mutex: Mutex for host diagnostic operations.
// @adapter_reset_lock: Spinlock for adapter reset operations.
// @adapter_reset_mutex: Mutex for adapter reset operations.
// @adapter_link_resetting: Flag indicating if adapter link is resetting.
// @adapter_reset_results: Results of the adapter reset operation.
// @pending_io_cnt: Count of pending I/O operations.
// @reset_wait_queue: Wait queue for reset operations.
// @reset_cnt: Counter for reset operations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_reset_desc {
    pub fault_reset_wq: *mut workqueue_struct,
    pub fault_reset_work: delayed_work,
    pub fault_reset_wq_name: [c_char; 48],
    pub /: *mut *mut mutex host_diag_mutex; / Mutex for operations.,
    pub /: *mut *mut spinlock_t adapter_reset_lock; / Protects adapter reset state.,
    pub /: *mut *mut mutex adapter_reset_mutex; / Serializes adapter reset.,
    pub adapter_link_resetting: u8,
    pub adapter_reset_results: c_int,
    pub pending_io_cnt: c_int,
    pub reset_wait_queue: wait_queue_head_t,
    pub reset_cnt: u32,
}

//
// struct leapraid_scan_dev_desc - Scan device descriptor for LeapRAID
//
// @wait_scan_dev_done: Flag indicating if scan device operation is done.
// @driver_loading: Flag indicating if driver is loading.
// @first_scan_dev_fired: Flag indicating if first scan device operation fired.
// @scan_dev_failed: Flag indicating if scan device operation failed.
// @scan_start: Flag indicating if scan operation started.
// @scan_start_failed: Count of failed scan start operations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_scan_dev_desc {
    pub wait_scan_dev_done: u8,
    pub driver_loading: u8,
    pub wait_driver_loading: wait_queue_head_t,
    pub first_scan_dev_fired: u8,
    pub scan_dev_failed: u8,
    pub scan_start: u8,
    pub scan_start_failed: u16,
}

//
// struct leapraid_access_ctrl - Access control structure for LeapRAID
//
// @pci_access_lock: Mutex for PCI access control.
// @shost_recovering: Flag indicating if host is recovering.
// @shost_recover_async: Flag indicating if host is recovering
// and the 0xFFFF event.
// @shost_recover_wq: Wait queue for threads waiting on shost_recover_async.
// @host_removing: Flag indicating if host is being removed.
// @pcie_recovering: Flag indicating if PCIe is recovering.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_access_ctrl {
    pub /: *mut *mut mutex pci_access_lock; / serializes PCI register access.,
    pub shost_recovering: u8,
    pub recovery_waitq: wait_queue_head_t,
    pub shost_recover_async: u8,
    pub shost_recover_wq: wait_queue_head_t,
    pub host_removing: u8,
    pub pcie_recovering: u8,
}

//
// struct leapraid_fw_log_desc - Firmware log descriptor for LeapRAID
//
// @fw_log_buffer: Buffer for firmware log data.
// @fw_log_buffer_dma: DMA address of the firmware log buffer.
// @fw_log_wq_name: Name of the firmware log workqueue.
// @fw_log_wq: Workqueue for firmware log operations.
// @fw_log_work: Delayed work structure for firmware log.
// @open_pcie_trace: Flag indicating if PCIe tracing is open.
// @fw_log_init_flag: Flag indicating if firmware log is initialized.
// @mmap_refcnt: Number of active user VMAs on fw_log_buffer.
// @mmap_waitq: Waitqueue for fw_log_buffer VMA teardown.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_fw_log_desc {
    pub fw_log_buffer: *mut u8,
    pub fw_log_buffer_dma: dma_addr_t,
    pub fw_log_wq_name: [c_char; 48],
    pub fw_log_wq: *mut workqueue_struct,
    pub fw_log_work: delayed_work,
    pub open_pcie_trace: c_int,
    pub fw_log_init_flag: c_int,
    pub mmap_refcnt: core::sync::atomic::AtomicI32,
    pub mmap_waitq: wait_queue_head_t,
}

pub const LEAPRAID_CARD_PORT_FLG_DIRTY: c_uint = 0x01;
pub const LEAPRAID_CARD_PORT_FLG_NEW: c_uint = 0x02;
pub const LEAPRAID_DISABLE_MP_PORT_ID: c_uint = 0xFF;
//
// struct leapraid_card_port - Card port structure for LeapRAID
//
// @list: List head for card port.
// @vphys_list: List head for virtual PHY list.
// @port_id: Port ID.
// @sas_address: SAS address.
// @phy_mask: Mask of PHY.
// @vphys_mask: Mask of virtual PHY.
// @flg: Flags for the port.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_card_port {
    pub list: list_head,
    pub vphys_list: list_head,
    pub port_id: u8,
    pub sas_address: u64,
    pub phy_mask: u32,
    pub vphys_mask: u32,
    pub flg: u8,
}

//
// struct leapraid_card_phy - Card PHY structure for LeapRAID
//
// @port_siblings: List head for port siblings.
// @card_port: Pointer to the card port.
// @identify: SAS identify structure.
// @remote_identify: Remote SAS identify structure.
// @phy: SAS PHY structure.
// @phy_id: PHY ID.
// @hdl: Handle for the port.
// @attached_hdl: Handle for the attached port.
// @phy_is_assigned: Flag indicating if PHY is assigned.
// @vphy: Flag indicating if virtual PHY.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_card_phy {
    pub port_siblings: list_head,
    pub card_port: *mut leapraid_card_port,
    pub identify: sas_identify,
    pub remote_identify: sas_identify,
    pub phy: *mut sas_phy,
    pub phy_id: u8,
    pub hdl: u16,
    pub attached_hdl: u16,
    pub phy_is_assigned: u8,
    pub vphy: u8,
}

//
// struct leapraid_topo_node - SAS topology node for LeapRAID
//
// @list: List head for linking nodes.
// @sas_port_list: List of SAS ports.
// @card_port: Associated card port.
// @card_phy: Associated card PHY.
// @rphy: SAS remote PHY device.
// @parent_dev: Parent device pointer.
// @sas_address: SAS address of this node.
// @sas_address_parent: Parent node's SAS address.
// @phys_num: Number of physical links.
// @hdl: Handle identifier.
// @enc_hdl: Enclosure handle.
// @enc_lid: Enclosure logical identifier.
// @resp: Response status flag.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_topo_node {
    pub list: list_head,
    pub sas_port_list: list_head,
    pub card_port: *mut leapraid_card_port,
    pub card_phy: *mut leapraid_card_phy,
    pub rphy: *mut sas_rphy,
    pub parent_dev: *mut device,
    pub sas_address: u64,
    pub sas_address_parent: u64,
    pub phys_num: u8,
    pub hdl: u16,
    pub enc_hdl: u16,
    pub enc_lid: u64,
    pub resp: u8,
}

//
// struct leapraid_dev_topo - LeapRAID device topology management structure
//
// @topo_node_lock: Spinlock for protecting topology node operations.
// @sas_dev_lock: Spinlock for SAS device list access.
// @raid_volume_lock: Spinlock for RAID volume list access.
// @enc_lock: Spinlock for enclosure device list access.
// @sas_id: SAS domain identifier.
// @card: Main card topology node.
// @exp_list: List of expander devices.
// @enc_list: List of enclosure devices.
// @sas_dev_list: List of SAS devices.
// @sas_dev_init_list: List of SAS devices being initialized.
// @raid_volume_list: List of RAID volumes.
// @card_port_list: List of card ports.
// @pd_hdls: Array of physical disk handles.
// @blocking_hdls: Array of blocking handles.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_dev_topo {
    pub /: *mut *mut spinlock_t topo_node_lock; / Protects topology node.,
    pub /: *mut *mut spinlock_t sas_dev_lock; / Protects SAS device list access.,
    pub /: *mut *mut spinlock_t raid_volume_lock; / Protects RAID volume list access.,
    pub /: *mut *mut spinlock_t enc_lock; / Protects enclosure device list access.,
    pub sas_id: c_int,
    pub card: leapraid_topo_node,
    pub exp_list: list_head,
    pub enc_list: list_head,
    pub sas_dev_list: list_head,
    pub sas_dev_init_list: list_head,
    pub raid_volume_list: list_head,
    pub card_port_list: list_head,
    pub pd_hdls_sz: u16,
    pub pd_hdls: *mut c_ulong,
    pub blocking_hdls: *mut c_ulong,
}

//
// struct leapraid_boot_dev - Boot device structure for LeapRAID
//
// @dev: Device pointer.
// @chnl: Channel number.
// @form: Form factor.
// @pg_dev: Config page device content.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_boot_dev {
    pub dev: *mut c_void,
    pub chnl: u8,
    pub form: u8,
    pub pg_dev: [u8; 24],
}

//
// struct leapraid_boot_devs - Boot device management structure
//
// @lock: Spinlock protecting boot device cache access.
// @requested_boot_dev: Requested primary boot device.
// @requested_alt_boot_dev: Requested alternate boot device.
// @current_boot_dev: Currently active boot device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_boot_devs {
    pub /: *mut *mut spinlock_t lock; / protects boot dev,
    pub requested_boot_dev: leapraid_boot_dev,
    pub requested_alt_boot_dev: leapraid_boot_dev,
    pub current_boot_dev: leapraid_boot_dev,
}

//
// struct leapraid_adapter - Main LeapRAID adapter structure
//
// @list: List head for adapter management.
// @shost: SCSI host structure.
// @pdev: PCI device structure.
// @iomem_base: I/O memory mapped base address.
// @rep_msg_host_idx: Host index for reply messages.
// @mask_int: Interrupt masking flag.
// @adapter_attr: Adapter attributes.
// @mem_desc: Memory descriptor.
// @driver_cmds: Driver commands.
// @dynamic_task_desc: Dynamic task descriptor.
// @fw_evt_s: Firmware event structure.
// @notification_desc: Notification descriptor.
// @reset_desc: Reset descriptor.
// @scan_dev_desc: Device scan descriptor.
// @access_ctrl: Access control.
// @fw_log_desc: Firmware log descriptor.
// @dev_topo: Device topology.
// @boot_devs: Boot devices.
// @overheat_desc: Overheat processing descriptor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_adapter {
    pub list: list_head,
    pub shost: *mut Scsi_Host,
    pub pdev: *mut pci_dev,
    pub iomem_base: *mut leapraid_reg_base __iomem,
    pub rep_msg_host_idx: u32,
    pub mask_int: u8,
    pub adapter_attr: leapraid_adapter_attr,
    pub mem_desc: leapraid_mem_desc,
    pub driver_cmds: leapraid_driver_cmds,
    pub dynamic_task_desc: leapraid_dynamic_task_desc,
    pub fw_evt_s: leapraid_fw_evt_struct,
    pub notification_desc: leapraid_notification_desc,
    pub reset_desc: leapraid_reset_desc,
    pub scan_dev_desc: leapraid_scan_dev_desc,
    pub access_ctrl: leapraid_access_ctrl,
    pub fw_log_desc: leapraid_fw_log_desc,
    pub dev_topo: leapraid_dev_topo,
    pub boot_devs: leapraid_boot_devs,
    pub overheat_desc: leapraid_overheat_desc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cfg_param_1 {
    pub form: u32,
    pub size: u32,
    pub phy_number: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cfg_param_2 {
    pub handle: u32,
    pub form_specific: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum config_page_action {
    GET_BIOS_PG2,
    GET_BIOS_PG3,
    GET_MANUFACTURING_PG0,
    GET_SAS_DEVICE_PG0,
    GET_SAS_IOUNIT_PG0,
    GET_SAS_IOUNIT_PG1,
    GET_SAS_EXPANDER_PG0,
    GET_SAS_EXPANDER_PG1,
    GET_SAS_ENCLOSURE_PG0,
    GET_PHY_PG0,
    GET_RAID_VOLUME_PG0,
    GET_RAID_VOLUME_PG1,
    GET_PHY_DISK_PG0,
}

//
// struct leapraid_enc_node - Enclosure node structure
//
// @list: List head for enclosure management.
// @pg0: Enclosure page 0 data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_enc_node {
    pub list: list_head,
    pub pg0: leapraid_enc_p0,
}

//
// struct leapraid_raid_volume - RAID volume structure
//
// @list: List head for volume management.
// @refcnt: Reference count.
// @starget: SCSI target structure.
// @sdev: SCSI device structure.
// @id: Volume ID.
// @channel: SCSI channel.
// @wwid: World wide Identifier.
// @hdl: Volume handle.
// @vol_type: Volume type.
// @pd_num: Number of physical disks.
// @resp: Response status.
// @dev_info: Device information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_raid_volume {
    pub list: list_head,
    pub refcnt: kref,
    pub starget: *mut scsi_target,
    pub sdev: *mut scsi_device,
    pub id: c_uint,
    pub channel: c_uint,
    pub wwid: u64,
    pub hdl: u16,
    pub vol_type: u8,
    pub pd_num: u8,
    pub resp: u8,
    pub dev_info: u32,
}

pub const LEAPRAID_TGT_FLG_RAID_MEMBER: c_uint = 0x01;
pub const LEAPRAID_TGT_FLG_VOLUME: c_uint = 0x02;
pub const LEAPRAID_NO_ULD_ATTACH: c_int = 1;
//
// struct leapraid_starget_priv - SCSI target private data
//
// @starget: SCSI target structure.
// @sas_address: SAS address.
// @hdl: Device handle.
// @num_luns: Number of LUNs.
// @flg: Flags.
// @deleted: Deletion flag.
// @tm_busy: Task management busy flag.
// @card_port: Associated card port.
// @sas_dev: SAS device structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_starget_priv {
    pub starget: *mut scsi_target,
    pub sas_address: u64,
    pub hdl: u16,
    pub num_luns: c_int,
    pub flg: u32,
    pub deleted: u8,
    pub tm_busy: u8,
    pub card_port: *mut leapraid_card_port,
    pub sas_dev: *mut leapraid_sas_dev,
}

pub const LEAPRAID_DEVICE_FLG_INIT: c_uint = 0x01;
//
// struct leapraid_sdev_priv - SCSI device private data
//
// @starget_priv: Associated target private data.
// @lun: Logical Unit Number.
// @flg: Flags.
// @ncq_prio_enable: Enables NCQ command priority for RT I/O.
// @block: Block flag.
// @deleted: Deletion flag.
// @sep: SEP flag.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_sdev_priv {
    pub starget_priv: *mut leapraid_starget_priv,
    pub lun: c_uint,
    pub flg: u32,
    pub ncq_prio_enable: u8,
    pub block: u8,
    pub deleted: u8,
    pub sep: u8,
}

//
// struct leapraid_sas_dev - SAS device structure
//
// @list: List head for device management.
// @starget: SCSI target structure.
// @card_port: Associated card port.
// @rphy: SAS remote PHY.
// @refcnt: Reference count.
// @id: Device ID.
// @channel: SCSI channel.
// @slot: Slot number.
// @phy: PHY identifier.
// @resp: Response status.
// @led_on: LED state.
// @sas_addr: SAS address.
// @dev_name: Device name.
// @hdl: Device handle.
// @parent_sas_addr: Parent SAS address.
// @enc_hdl: Enclosure handle.
// @enc_lid: Enclosure logical ID.
// @volume_hdl: Volume handle.
// @volume_wwid: Volume WWID.
// @dev_info: Device information.
// @pend_sas_rphy_add: Pending SAS remote PHY addition flag.
// @enc_level: Enclosure level.
// @port_connection: Port connection.
// @connector_name: Connector name.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_sas_dev {
    pub list: list_head,
    pub starget: *mut scsi_target,
    pub card_port: *mut leapraid_card_port,
    pub rphy: *mut sas_rphy,
    pub refcnt: kref,
    pub id: c_uint,
    pub channel: c_uint,
    pub slot: u16,
    pub phy: u8,
    pub resp: u8,
    pub led_on: u8,
    pub sas_addr: u64,
    pub dev_name: u64,
    pub hdl: u16,
    pub parent_sas_addr: u64,
    pub enc_hdl: u16,
    pub enc_lid: u64,
    pub volume_hdl: u16,
    pub volume_wwid: u64,
    pub dev_info: u32,
    pub pend_sas_rphy_add: u8,
    pub enc_level: u8,
    pub port_connection: u8,
    pub 1]: u8 connector_name[LEAPRAID_SAS_DEV_P0_CON_NAME_LEN +,
}

extern "C" {
    pub fn leapraid_boot_dev_get(dev: *mut c_void, chnl: u32);
}
extern "C" {
    pub fn leapraid_boot_dev_put(dev: *mut c_void, chnl: u32);
}
//
// struct leapraid_sas_port - SAS port structure
//
// @port_list: List head for port management.
// @phy_list: List of PHYs in this port.
// @port: SAS port structure.
// @card_port: Associated card port.
// @remote_identify: Remote device identification.
// @rphy: SAS remote PHY.
// @phys_num: Number of PHYs in this port.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_sas_port {
    pub port_list: list_head,
    pub phy_list: list_head,
    pub port: *mut sas_port,
    pub card_port: *mut leapraid_card_port,
    pub remote_identify: sas_identify,
    pub rphy: *mut sas_rphy,
    pub phys_num: u8,
}

pub const LEAPRAID_VPHY_FLG_DIRTY: c_uint = 0x01;
//
// struct leapraid_vphy - Virtual PHY structure
//
// @list: List head for PHY management.
// @sas_address: SAS address.
// @phy_mask: PHY mask.
// @flg: Flags.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_vphy {
    pub list: list_head,
    pub sas_address: u64,
    pub phy_mask: u32,
    pub flg: u8,
}

//
// struct leapraid_tgt_rst_list - Target reset tracking entry
// @list: List node used to link reset entries.
// @handle: Device handle of the target being reset.
// @state: Current state of the target reset operation.
//
// Each entry represents a target device that is undergoing or has
// undergone a target reset operation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_tgt_rst_list {
    pub list: list_head,
    pub handle: u16,
    pub state: u16,
}

//
// struct sense_info - Parsed SCSI sense information
// @sense_key: Sense key extracted from sense data.
// @asc: Additional Sense Code (ASC).
// @ascq: Additional Sense Code Qualifier (ASCQ).
//
// This structure stores key fields parsed from SCSI sense data to
// simplify error handling and reporting.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sense_info {
    pub sense_key: u8,
    pub asc: u8,
    pub ascq: u8,
}

//
// struct leapraid_fw_log_info - Firmware log position information
// @user_position: Current log read position used by the driver.
// @adapter_position: Current log write position maintained by firmware.
//
// This structure tracks firmware log buffer positions used for retrieving
// log entries from the adapter.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_fw_log_info {
    pub user_position: u32,
    pub adapter_position: u32,
}

//
// enum reset_type - Reset type enumeration
//
// @FULL_RESET: Full hardware reset.
// @PART_RESET: Partial reset.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reset_type {
    FULL_RESET,
    PART_RESET,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum leapraid_card_port_checking_flg {
    CARD_PORT_FURTHER_CHECKING_NEEDED = 0,
    CARD_PORT_SKIP_CHECKING,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum leapraid_port_checking_state {
    NEW_CARD_PORT = 0,
    SAME_PORT_WITH_NOTHING_CHANGED,
    SAME_PORT_WITH_PARTIALLY_CHANGED_PHYS,
    SAME_ADDR_WITH_PARTIALLY_CHANGED_PHYS,
    SAME_ADDR_ONLY,
}

//
// struct leapraid_card_port_feature - Card port feature
//
// @dirty_flg: Dirty flag indicator.
// @same_addr: Same address flag.
// @exact_phy: Exact PHY match flag.
// @phy_overlap: PHY overlap bitmap.
// @same_port: Same port flag.
// @cur_chking_old_port: Current checking old port.
// @expected_old_port: Expected old port.
// @same_addr_port_count: Same address port count.
// @checking_state: Port checking state.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_card_port_feature {
    pub dirty_flg: u8,
    pub same_addr: u8,
    pub exact_phy: u8,
    pub phy_overlap: u32,
    pub same_port: u8,
    pub cur_chking_old_port: *mut leapraid_card_port,
    pub expected_old_port: *mut leapraid_card_port,
    pub same_addr_port_count: c_int,
    pub checking_state: leapraid_port_checking_state,
}

pub const SMP_REPORT_MANUFACTURER_INFORMATION_FRAME_TYPE: c_uint = 0x40;
pub const SMP_REPORT_MANUFACTURER_INFORMATION_FUNC: c_uint = 0x01;
//
// ref: SAS-2(INCITS 457-2010) 10.4.3.5
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_rep_manu_request {
    pub smp_frame_type: u8,
    pub function: u8,
    pub allocated_response_length: u8,
    pub request_length: u8,
}

//
// ref: SAS-2(INCITS 457-2010) 10.4.3.5
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_rep_manu_reply {
    pub smp_frame_type: u8,
    pub function: u8,
    pub function_result: u8,
    pub response_length: u8,
    pub expander_change_count: u16,
    pub r1: [u8; 2],
    pub sas_format: u8,
    pub r2: [u8; 3],
    pub vendor_identification: [u8; SAS_EXPANDER_VENDOR_ID_LEN],
    pub product_identification: [u8; SAS_EXPANDER_PRODUCT_ID_LEN],
    pub product_revision_level: [u8; SAS_EXPANDER_PRODUCT_REV_LEN],
    pub comp_vendor_identification: [u8; SAS_EXPANDER_COMPONENT_VENDOR_ID_LEN],
    pub component_id: u16,
    pub component_revision_level: u8,
    pub r3: u8,
    pub vendor_specific: [u8; 8],
}

extern "C" {
    pub fn leapraid_ctrl_init(adapter: *mut leapraid_adapter) -> c_int;
}
extern "C" {
    pub fn leapraid_remove_ctrl(adapter: *mut leapraid_adapter);
}
extern "C" {
    pub fn leapraid_check_scheduled_fault_start(adapter: *mut leapraid_adapter);
}
extern "C" {
    pub fn leapraid_check_scheduled_fault_stop(adapter: *mut leapraid_adapter);
}
extern "C" {
    pub fn leapraid_fw_log_start(adapter: *mut leapraid_adapter);
}
extern "C" {
    pub fn leapraid_fw_log_stop(adapter: *mut leapraid_adapter);
}
extern "C" {
    pub fn leapraid_set_pcie_and_notification(adapter: *mut leapraid_adapter) -> c_int;
}
extern "C" {
    pub fn leapraid_disable_controller(adapter: *mut leapraid_adapter);
}
extern "C" {
    pub fn leapraid_mask_int(adapter: *mut leapraid_adapter);
}
extern "C" {
    pub fn leapraid_unmask_int(adapter: *mut leapraid_adapter);
}
extern "C" {
    pub fn leapraid_get_adapter_state(adapter: *mut leapraid_adapter) -> u32;
}
extern "C" {
    pub fn leapraid_pci_removed(adapter: *mut leapraid_adapter) -> bool;
}
extern "C" {
    pub fn leapraid_free_taskid(adapter: *mut leapraid_adapter, taskid: u16);
}
extern "C" {
    pub fn leapraid_scan_dev(adapter: *mut leapraid_adapter, async_scan_dev: bool) -> c_int;
}
extern "C" {
    pub fn leapraid_scan_dev_done(adapter: *mut leapraid_adapter);
}
extern "C" {
    pub fn leapraid_cleanup_lists(adapter: *mut leapraid_adapter);
}
extern "C" {
    pub fn leapraid_wait_cmds_done(adapter: *mut leapraid_adapter);
}
extern "C" {
    pub fn leapraid_clean_active_scsi_cmds(adapter: *mut leapraid_adapter);
}
extern "C" {
    pub fn leapraid_sync_irqs(adapter: *mut leapraid_adapter, poll: bool);
}
extern "C" {
    pub fn leapraid_rep_queue_handler(rq: *mut leapraid_rq) -> c_int;
}
extern "C" {
    pub fn leapraid_blk_mq_poll(shost: *mut Scsi_Host, queue_num: c_uint) -> c_int;
}
extern "C" {
    pub fn leapraid_mq_polling_pause(adapter: *mut leapraid_adapter);
}
extern "C" {
    pub fn leapraid_mq_polling_resume(adapter: *mut leapraid_adapter);
}
extern "C" {
    pub fn leapraid_set_tm_flg(adapter: *mut leapraid_adapter, handle: u16);
}
extern "C" {
    pub fn leapraid_clear_tm_flg(adapter: *mut leapraid_adapter, handle: u16);
}
extern "C" {
    pub fn leapraid_async_turn_on_led(adapter: *mut leapraid_adapter, handle: u16);
}
extern "C" {
    pub fn leapraid_clean_active_fw_evt(adapter: *mut leapraid_adapter);
}
extern "C" {
    pub fn leapraid_fire_task(adapter: *mut leapraid_adapter, taskid: u16);
}
extern "C" {
    pub fn leapraid_change_queue_depth(sdev: *mut scsi_device, qdepth: c_int) -> c_int;
}
extern "C" {
    pub fn leapraid_ctl_release(inode: *mut inode, filep: *mut file) -> c_int;
}
extern "C" {
    pub fn leapraid_ctl_init() -> c_int;
}
extern "C" {
    pub fn leapraid_ctl_exit();
}
extern "C" {
    pub fn leapraid_overheat_cleanup(adapter: *mut leapraid_adapter);
}
extern "C" {
    pub fn leapraid_smart_fault_detect(adapter: *mut leapraid_adapter, hdl: u16);
}
