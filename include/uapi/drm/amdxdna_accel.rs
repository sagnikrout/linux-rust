//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/drm/amdxdna_accel.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Copyright (C) 2022-2024, Advanced Micro Devices, Inc.
//

pub const AMDXDNA_INVALID_CTX_HANDLE: c_int = 0;
pub const AMDXDNA_INVALID_BO_HANDLE: c_int = 0;
pub const AMDXDNA_INVALID_FENCE_HANDLE: c_int = 0;

//
// Define hardware context priority
//
pub const AMDXDNA_QOS_REALTIME_PRIORITY: c_uint = 0x100;
pub const AMDXDNA_QOS_HIGH_PRIORITY: c_uint = 0x180;
pub const AMDXDNA_QOS_NORMAL_PRIORITY: c_uint = 0x200;
pub const AMDXDNA_QOS_LOW_PRIORITY: c_uint = 0x280;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdxdna_device_type {
    AMDXDNA_DEV_TYPE_UNKNOWN = -1,
    AMDXDNA_DEV_TYPE_KMQ = 0,
    AMDXDNA_DEV_TYPE_UMQ = 1,
    AMDXDNA_DEV_TYPE_PF = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdxdna_drm_ioctl_id {
    DRM_AMDXDNA_CREATE_HWCTX,
    DRM_AMDXDNA_DESTROY_HWCTX,
    DRM_AMDXDNA_CONFIG_HWCTX,
    DRM_AMDXDNA_CREATE_BO,
    DRM_AMDXDNA_GET_BO_INFO,
    DRM_AMDXDNA_SYNC_BO,
    DRM_AMDXDNA_EXEC_CMD,
    DRM_AMDXDNA_GET_INFO,
    DRM_AMDXDNA_SET_STATE,
    DRM_AMDXDNA_WAIT_CMD,
    DRM_AMDXDNA_GET_ARRAY,
}

//
// struct qos_info - QoS information for driver.
// @gops: Giga operations per second.
// @fps: Frames per second.
// @dma_bandwidth: DMA bandwidtha.
// @latency: Frame response latency.
// @frame_exec_time: Frame execution time.
// @priority: Request priority.
//
// User program can provide QoS hints to driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_qos_info {
    pub gops: __u32,
    pub fps: __u32,
    pub dma_bandwidth: __u32,
    pub latency: __u32,
    pub frame_exec_time: __u32,
    pub priority: __u32,
}

//
// struct amdxdna_drm_create_hwctx - Create hardware context.
// @ext: MBZ.
// @ext_flags: MBZ.
// @qos_p: Address of QoS info.
// @umq_bo: BO handle for user mode queue(UMQ).
// @log_buf_bo: BO handle for log buffer.
// @max_opc: Maximum operations per cycle.
// @num_tiles: Number of AIE tiles.
// @mem_size: Size of AIE tile memory.
// @umq_doorbell: Returned offset of doorbell associated with UMQ.
// @handle: Returned hardware context handle.
// @syncobj_handle: Returned syncobj handle for command completion.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_create_hwctx {
    pub ext: __u64,
    pub ext_flags: __u64,
    pub qos_p: __u64,
    pub umq_bo: __u32,
    pub log_buf_bo: __u32,
    pub max_opc: __u32,
    pub num_tiles: __u32,
    pub mem_size: __u32,
    pub umq_doorbell: __u32,
    pub handle: __u32,
    pub syncobj_handle: __u32,
}

//
// struct amdxdna_drm_destroy_hwctx - Destroy hardware context.
// @handle: Hardware context handle.
// @pad: MBZ.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_destroy_hwctx {
    pub handle: __u32,
    pub pad: __u32,
}

//
// struct amdxdna_cu_config - configuration for one CU
// @cu_bo: CU configuration buffer bo handle.
// @cu_func: Function of a CU.
// @pad: MBZ.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_cu_config {
    pub cu_bo: __u32,
    pub cu_func: __u8,
    pub pad: [__u8; 3],
}

//
// struct amdxdna_hwctx_param_config_cu - configuration for CUs in hardware context
// @num_cus: Number of CUs to configure.
// @pad: MBZ.
// @cu_configs: Array of CU configurations of struct amdxdna_cu_config.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_hwctx_param_config_cu {
    pub num_cus: __u16,
    pub pad: [__u16; 3],
    pub __counted_by(num_cus): amdxdna_cu_config cu_configs[],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdxdna_drm_config_hwctx_param {
    DRM_AMDXDNA_HWCTX_CONFIG_CU,
    DRM_AMDXDNA_HWCTX_ASSIGN_DBG_BUF,
    DRM_AMDXDNA_HWCTX_REMOVE_DBG_BUF,
}

//
// struct amdxdna_drm_config_hwctx - Configure hardware context.
// @handle: hardware context handle.
// @param_type: Value in enum amdxdna_drm_config_hwctx_param. Specifies the
// structure passed in via param_val.
// @param_val: A structure specified by the param_type struct member.
// @param_val_size: Size of the parameter buffer pointed to by the param_val.
// If param_val is not a pointer, driver can ignore this.
// @pad: MBZ.
//
// Note: if the param_val is a pointer pointing to a buffer, the maximum size
// of the buffer is 4KiB(PAGE_SIZE).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_config_hwctx {
    pub handle: __u32,
    pub param_type: __u32,
    pub param_val: __u64,
    pub param_val_size: __u32,
    pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdxdna_bo_type {
    AMDXDNA_BO_INVALID = 0,
    AMDXDNA_BO_SHMEM = 1, /* Be compatible with legacy application code. */
    AMDXDNA_BO_SHARE = 1,
    AMDXDNA_BO_DEV_HEAP = 2,
    AMDXDNA_BO_DEV = 3,
    AMDXDNA_BO_CMD = 4,
}

//
// struct amdxdna_drm_va_entry
// @vaddr: Virtual address.
// @len: Size of entry.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_va_entry {
    pub vaddr: __u64,
    pub len: __u64,
}

//
// struct amdxdna_drm_va_tbl
// @dmabuf_fd: The fd of dmabuf.
// @num_entries: Number of va entries.
// @va_entries: Array of va entries.
//
// The input can be either a dmabuf fd or a virtual address entry table.
// When dmabuf_fd is used, num_entries must be zero.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_va_tbl {
    pub dmabuf_fd: __s32,
    pub num_entries: __u32,
    pub va_entries: [amdxdna_drm_va_entry; ],
}

//
// struct amdxdna_drm_create_bo - Create a buffer object.
// @flags: Buffer flags. MBZ.
// @vaddr: User VA of buffer if applied. MBZ.
// @size: Size in bytes.
// @type: Buffer type.
// @handle: Returned DRM buffer object handle.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_create_bo {
    pub flags: __u64,
    pub vaddr: __u64,
    pub size: __u64,
    pub type: __u32,
    pub handle: __u32,
}

//
// struct amdxdna_drm_get_bo_info - Get buffer object information.
// @ext: MBZ.
// @ext_flags: MBZ.
// @handle: DRM buffer object handle.
// @pad: MBZ.
// @map_offset: Returned DRM fake offset for mmap().
// @vaddr: Returned user VA of buffer. 0 in case user needs mmap().
// @xdna_addr: Returned XDNA device virtual address.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_get_bo_info {
    pub ext: __u64,
    pub ext_flags: __u64,
    pub handle: __u32,
    pub pad: __u32,
    pub map_offset: __u64,
    pub vaddr: __u64,
    pub xdna_addr: __u64,
}

//
// struct amdxdna_drm_sync_bo - Sync buffer object.
// @handle: Buffer object handle.
// @direction: Direction of sync, can be from device or to device.
// @offset: Offset in the buffer to sync.
// @size: Size in bytes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_sync_bo {
    pub handle: __u32,

    pub direction: __u32,
    pub offset: __u64,
    pub size: __u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdxdna_cmd_type {
    AMDXDNA_CMD_SUBMIT_EXEC_BUF = 0,
    AMDXDNA_CMD_SUBMIT_DEPENDENCY,
    AMDXDNA_CMD_SUBMIT_SIGNAL,
}

//
// struct amdxdna_drm_exec_cmd - Execute command.
// @ext: MBZ.
// @ext_flags: MBZ.
// @hwctx: Hardware context handle.
// @type: One of command type in enum amdxdna_cmd_type.
// @cmd_handles: Array of command handles or the command handle itself
// in case of just one.
// @args: Array of arguments for all command handles.
// @cmd_count: Number of command handles in the cmd_handles array.
// @arg_count: Number of arguments in the args array.
// @seq: Returned sequence number for this command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_exec_cmd {
    pub ext: __u64,
    pub ext_flags: __u64,
    pub hwctx: __u32,
    pub type: __u32,
    pub cmd_handles: __u64,
    pub args: __u64,
    pub cmd_count: __u32,
    pub arg_count: __u32,
    pub seq: __u64,
}

//
// struct amdxdna_drm_wait_cmd - Wait execution command.
//
// @hwctx: Context handle.
// @timeout: timeout in ms, 0 implies infinite wait.
// @seq: sequence number of the command returned by execute command.
//
// Wait a command specified by seq to be completed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_wait_cmd {
    pub hwctx: __u32,
    pub timeout: __u32,
    pub seq: __u64,
}

//
// struct amdxdna_drm_query_aie_status - Query the status of the AIE hardware
// @buffer: The user space buffer that will return the AIE status.
// @buffer_size: The size of the user space buffer.
// @cols_filled: A bitmap of AIE columns whose data has been returned in the buffer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_query_aie_status {
    pub /: *mut *mut __u64 buffer; / out,
    pub /: *mut *mut __u32 buffer_size; / in,
    pub /: *mut *mut __u32 cols_filled; / out,
}

//
// struct amdxdna_drm_query_aie_version - Query the version of the AIE hardware
// @major: The major version number.
// @minor: The minor version number.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_query_aie_version {
    pub /: *mut *mut __u32 major; / out,
    pub /: *mut *mut __u32 minor; / out,
}

//
// struct amdxdna_drm_query_aie_tile_metadata - Query the metadata of AIE tile (core, mem, shim)
// @row_count: The number of rows.
// @row_start: The starting row number.
// @dma_channel_count: The number of dma channels.
// @lock_count: The number of locks.
// @event_reg_count: The number of events.
// @pad: Structure padding.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_query_aie_tile_metadata {
    pub row_count: __u16,
    pub row_start: __u16,
    pub dma_channel_count: __u16,
    pub lock_count: __u16,
    pub event_reg_count: __u16,
    pub pad: [__u16; 3],
}

//
// struct amdxdna_drm_query_aie_metadata - Query the metadata of the AIE hardware
// @col_size: The size of a column in bytes.
// @cols: The total number of columns.
// @rows: The total number of rows.
// @version: The version of the AIE hardware.
// @core: The metadata for all core tiles.
// @mem: The metadata for all mem tiles.
// @shim: The metadata for all shim tiles.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_query_aie_metadata {
    pub col_size: __u32,
    pub cols: __u16,
    pub rows: __u16,
    pub version: amdxdna_drm_query_aie_version,
    pub core: amdxdna_drm_query_aie_tile_metadata,
    pub mem: amdxdna_drm_query_aie_tile_metadata,
    pub shim: amdxdna_drm_query_aie_tile_metadata,
}

//
// struct amdxdna_drm_query_clock - Metadata for a clock
// @name: The clock name.
// @freq_mhz: The clock frequency.
// @pad: Structure padding.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_query_clock {
    pub name: [__u8; 16],
    pub freq_mhz: __u32,
    pub pad: __u32,
}

//
// struct amdxdna_drm_query_clock_metadata - Query metadata for clocks
// @mp_npu_clock: The metadata for MP-NPU clock.
// @h_clock: The metadata for H clock.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_query_clock_metadata {
    pub mp_npu_clock: amdxdna_drm_query_clock,
    pub h_clock: amdxdna_drm_query_clock,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdxdna_sensor_type {
    AMDXDNA_SENSOR_TYPE_POWER,
    AMDXDNA_SENSOR_TYPE_COLUMN_UTILIZATION
}

//
// struct amdxdna_drm_query_sensor - The data for single sensor.
// @label: The name for a sensor.
// @input: The current value of the sensor.
// @max: The maximum value possible for the sensor.
// @average: The average value of the sensor.
// @highest: The highest recorded sensor value for this driver load for the sensor.
// @status: The sensor status.
// @units: The sensor units.
// @unitm: Translates value member variables into the correct unit via (pow(10, unitm) * value).
// @type: The sensor type from enum amdxdna_sensor_type.
// @pad: Structure padding.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_query_sensor {
    pub label: [__u8; 64],
    pub input: __u32,
    pub max: __u32,
    pub average: __u32,
    pub highest: __u32,
    pub status: [__u8; 64],
    pub units: [__u8; 16],
    pub unitm: __s8,
    pub type: __u8,
    pub pad: [__u8; 6],
}

//
// struct amdxdna_drm_query_hwctx - The data for single context.
// @context_id: The ID for this context.
// @start_col: The starting column for the partition assigned to this context.
// @num_col: The number of columns in the partition assigned to this context.
// @pad: Structure padding.
// @pid: The Process ID of the process that created this context.
// @command_submissions: The number of commands submitted to this context.
// @command_completions: The number of commands completed by this context.
// @migrations: The number of times this context has been moved to a different partition.
// @preemptions: The number of times this context has been preempted by another context in the
// same partition.
// @errors: The errors for this context.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_query_hwctx {
    pub context_id: __u32,
    pub start_col: __u32,
    pub num_col: __u32,
    pub pad: __u32,
    pub pid: __s64,
    pub command_submissions: __u64,
    pub command_completions: __u64,
    pub migrations: __u64,
    pub preemptions: __u64,
    pub errors: __u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdxdna_power_mode_type {
    POWER_MODE_DEFAULT, /* Fallback to calculated DPM */
    POWER_MODE_LOW,     /* Set frequency to lowest DPM */
    POWER_MODE_MEDIUM,  /* Set frequency to medium DPM */
    POWER_MODE_HIGH,    /* Set frequency to highest DPM */
    POWER_MODE_TURBO,   /* Maximum power */
}

//
// struct amdxdna_drm_get_power_mode - Get the configured power mode
// @power_mode: The mode type from enum amdxdna_power_mode_type
// @pad: Structure padding.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_get_power_mode {
    pub power_mode: __u8,
    pub pad: [__u8; 7],
}

//
// struct amdxdna_drm_query_firmware_version - Query the firmware version
// @major: The major version number
// @minor: The minor version number
// @patch: The patch level version number
// @build: The build ID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_query_firmware_version {
    pub /: *mut *mut __u32 major; / out,
    pub /: *mut *mut __u32 minor; / out,
    pub /: *mut *mut __u32 patch; / out,
    pub /: *mut *mut __u32 build; / out,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdxdna_drm_get_param {
    DRM_AMDXDNA_QUERY_AIE_STATUS,
    DRM_AMDXDNA_QUERY_AIE_METADATA,
    DRM_AMDXDNA_QUERY_AIE_VERSION,
    DRM_AMDXDNA_QUERY_CLOCK_METADATA,
    DRM_AMDXDNA_QUERY_SENSORS,
    DRM_AMDXDNA_QUERY_HW_CONTEXTS,
    DRM_AMDXDNA_QUERY_FIRMWARE_VERSION = 8,
    DRM_AMDXDNA_GET_POWER_MODE,
    DRM_AMDXDNA_QUERY_TELEMETRY,
    DRM_AMDXDNA_GET_FORCE_PREEMPT_STATE,
    DRM_AMDXDNA_QUERY_RESOURCE_INFO,
    DRM_AMDXDNA_GET_FRAME_BOUNDARY_PREEMPT_STATE,
}

//
// struct amdxdna_drm_get_resource_info - Get resource information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_get_resource_info {
// @npu_clk_max: max H-Clocks
    pub npu_clk_max: __u64,
// @npu_tops_max: max TOPs
    pub npu_tops_max: __u64,
// @npu_task_max: max number of tasks
    pub npu_task_max: __u64,
// @npu_tops_curr: current TOPs
    pub npu_tops_curr: __u64,
// @npu_task_curr: current number of tasks
    pub npu_task_curr: __u64,
}

//
// struct amdxdna_drm_attribute_state - State of an attribute
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_attribute_state {
// @state: enabled or disabled
    pub state: __u8,
// @pad: MBZ
    pub pad: [__u8; 7],
}

//
// struct amdxdna_drm_query_telemetry_header - Telemetry data header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_query_telemetry_header {
// @major: Firmware telemetry interface major version number
    pub major: __u32,
// @minor: Firmware telemetry interface minor version number
    pub minor: __u32,
// @type: Telemetry query type
    pub type: __u32,
// @map_num_elements: Total number of elements in the map table
    pub map_num_elements: __u32,
// @map: Element map
    pub map: [__u32; ],
}

//
// struct amdxdna_drm_get_info - Get some information from the AIE hardware.
// @param: Value in enum amdxdna_drm_get_param. Specifies the structure passed in the buffer.
// @buffer_size: Size of the input buffer. Size needed/written by the kernel.
// @buffer: A structure specified by the param struct member.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_get_info {
    pub /: *mut *mut __u32 param; / in,
    pub /: *mut *mut __u32 buffer_size; / in/out,
    pub /: *mut *mut __u64 buffer; / in/out,
}

pub const AMDXDNA_HWCTX_STATE_IDLE: c_int = 0;
pub const AMDXDNA_HWCTX_STATE_ACTIVE: c_int = 1;
//
// struct amdxdna_drm_hwctx_entry - The hardware context array entry
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_hwctx_entry {
// @context_id: Context ID.
    pub context_id: __u32,
// @start_col: Start AIE array column assigned to context.
    pub start_col: __u32,
// @num_col: Number of AIE array columns assigned to context.
    pub num_col: __u32,
// @hwctx_id: The real hardware context id.
    pub hwctx_id: __u32,
// @pid: ID of process which created this context.
    pub pid: __s64,
// @command_submissions: Number of commands submitted.
    pub command_submissions: __u64,
// @command_completions: Number of commands completed.
    pub command_completions: __u64,
// @migrations: Number of times been migrated.
    pub migrations: __u64,
// @preemptions: Number of times been preempted.
    pub preemptions: __u64,
// @errors: Number of errors happened.
    pub errors: __u64,
// @priority: Context priority.
    pub priority: __u64,
// @heap_usage: Usage of device heap buffer.
    pub heap_usage: __u64,
// @suspensions: Number of times been suspended.
    pub suspensions: __u64,
//
// @state: Context state.
// %AMDXDNA_HWCTX_STATE_IDLE
// %AMDXDNA_HWCTX_STATE_ACTIVE
//
    pub state: __u32,
// @pasid: PASID been bound.
    pub pasid: __u32,
// @gops: Giga operations per second.
    pub gops: __u32,
// @fps: Frames per second.
    pub fps: __u32,
// @dma_bandwidth: DMA bandwidth.
    pub dma_bandwidth: __u32,
// @latency: Frame response latency.
    pub latency: __u32,
// @frame_exec_time: Frame execution time.
    pub frame_exec_time: __u32,
// @txn_op_idx: Index of last control code executed.
    pub txn_op_idx: __u32,
// @ctx_pc: Program counter.
    pub ctx_pc: __u32,
// @fatal_error_type: Fatal error type if context crashes.
    pub fatal_error_type: __u32,
// @fatal_error_exception_type: Firmware exception type.
    pub fatal_error_exception_type: __u32,
// @fatal_error_exception_pc: Firmware exception program counter.
    pub fatal_error_exception_pc: __u32,
// @fatal_error_app_module: Exception module name.
    pub fatal_error_app_module: __u32,
// @pad: Structure pad.
    pub pad: __u32,
}

//
// struct amdxdna_async_error - XDNA async error structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_async_error {
// @err_code: Error code.
    pub err_code: __u64,
// @ts_us: Timestamp.
    pub ts_us: __u64,
// @ex_err_code: Extra error code
    pub ex_err_code: __u64,
}

//
// struct amdxdna_drm_bo_usage - all types of BO usage
// BOs managed by XRT/SHIM/driver is counted as internal.
// Others are counted as external which are managed by applications.
//
// Among all types of BOs:
// AMDXDNA_BO_DEV_HEAP - is counted for internal.
// AMDXDNA_BO_SHARE    - is counted for external.
// AMDXDNA_BO_CMD      - is counted for internal.
// AMDXDNA_BO_DEV      - is counted by heap_usage only, not internal
// or external. It does not add to the total memory
// footprint since its mem comes from heap which is
// already counted as internal.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_bo_usage {
// @pid: The ID of the process to query from.
    pub pid: __s64,
// @total_usage: Total BO size used by process.
    pub total_usage: __u64,
// @internal_usage: Total internal BO size used by process.
    pub internal_usage: __u64,
// @heap_usage: Total device BO size used by process.
    pub heap_usage: __u64,
}

//
// Supported params in struct amdxdna_drm_get_array
//
pub const DRM_AMDXDNA_HW_CONTEXT_ALL: c_int = 0;
pub const DRM_AMDXDNA_HW_LAST_ASYNC_ERR: c_int = 2;
pub const DRM_AMDXDNA_BO_USAGE: c_int = 6;
//
// struct amdxdna_drm_get_array - Get information array.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_get_array {
//
// @param:
//
// Supported params:
//
// %DRM_AMDXDNA_HW_CONTEXT_ALL:
// Returns all created hardware contexts.
//
// %DRM_AMDXDNA_HW_LAST_ASYNC_ERR:
// Returns last async error.
//
// %DRM_AMDXDNA_BO_USAGE:
// Returns usage of heap/internal/external BOs.
//
    pub param: __u32,
//
// @element_size:
//
// Specifies maximum element size and returns the actual element size.
//
    pub element_size: __u32,
//
// @num_element:
//
// Specifies maximum number of elements and returns the actual number
// of elements.
//
    pub /: *mut *mut __u32 num_element; / in/out,
// @pad: MBZ
    pub pad: __u32,
//
// @buffer:
//
// Specifies the match conditions and returns the matched information
// array.
//
    pub buffer: __u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdxdna_drm_set_param {
    DRM_AMDXDNA_SET_POWER_MODE,
    DRM_AMDXDNA_WRITE_AIE_MEM,
    DRM_AMDXDNA_WRITE_AIE_REG,
    DRM_AMDXDNA_SET_FORCE_PREEMPT,
    DRM_AMDXDNA_SET_FRAME_BOUNDARY_PREEMPT,
}

//
// struct amdxdna_drm_set_state - Set the state of the AIE hardware.
// @param: Value in enum amdxdna_drm_set_param.
// @buffer_size: Size of the input param.
// @buffer: Pointer to the input param.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_set_state {
    pub /: *mut *mut __u32 param; / in,
    pub /: *mut *mut __u32 buffer_size; / in,
    pub /: *mut *mut __u64 buffer; / in,
}

//
// struct amdxdna_drm_set_power_mode - Set the power mode of the AIE hardware
// @power_mode: The sensor type from enum amdxdna_power_mode_type
// @pad: MBZ.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_set_power_mode {
    pub power_mode: __u8,
    pub pad: [__u8; 7],
}

