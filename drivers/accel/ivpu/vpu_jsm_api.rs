//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/ivpu/vpu_jsm_api.h
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


// SPDX-License-Identifier: MIT
//
// Copyright (c) 2020-2025, Intel Corporation.
//
// @addtogroup Jsm
// @{
//
// @file
// @brief JSM shared definitions
//
// Major version changes that break backward compatibility
//
pub const VPU_JSM_API_VER_MAJOR: c_int = 3;
//
// Minor version changes when API backward compatibility is preserved.
//
pub const VPU_JSM_API_VER_MINOR: c_int = 34;
//
// API header changed (field names, documentation, formatting) but API itself has not been changed
//
pub const VPU_JSM_API_VER_PATCH: c_int = 0;
//
// Index in the API version table
//
pub const VPU_JSM_API_VER_INDEX: c_int = 4;
//
// Number of Priority Bands for Hardware Scheduling
// Bands: Idle(0), Normal(1), Focus(2), RealTime(3)
//
pub const VPU_HWS_NUM_PRIORITY_BANDS: c_int = 4;
// Max number of impacted contexts that can be dealt with the engine reset command
pub const VPU_MAX_ENGINE_RESET_IMPACTED_CONTEXTS: c_int = 3;
//
// Pack the API structures to enforce binary compatibility
// Align to 8 bytes for optimal performance
//

//
// Engine indexes.
//
pub const VPU_ENGINE_COMPUTE: c_int = 0;
pub const VPU_ENGINE_NB: c_int = 1;
//
// VPU status values.
//
pub const VPU_JSM_STATUS_SUCCESS: c_uint = 0x0U;
pub const VPU_JSM_STATUS_PARSING_ERR: c_uint = 0x1U;
pub const VPU_JSM_STATUS_PROCESSING_ERR: c_uint = 0x2U;
pub const VPU_JSM_STATUS_PREEMPTED: c_uint = 0x3U;
pub const VPU_JSM_STATUS_ABORTED: c_uint = 0x4U;
pub const VPU_JSM_STATUS_USER_CTX_VIOL_ERR: c_uint = 0x5U;
pub const VPU_JSM_STATUS_GLOBAL_CTX_VIOL_ERR: c_uint = 0x6U;
pub const VPU_JSM_STATUS_MVNCI_WRONG_INPUT_FORMAT: c_uint = 0x7U;
pub const VPU_JSM_STATUS_MVNCI_UNSUPPORTED_NETWORK_ELEMENT: c_uint = 0x8U;
pub const VPU_JSM_STATUS_MVNCI_INVALID_HANDLE: c_uint = 0x9U;
pub const VPU_JSM_STATUS_MVNCI_OUT_OF_RESOURCES: c_uint = 0xAU;
pub const VPU_JSM_STATUS_MVNCI_NOT_IMPLEMENTED: c_uint = 0xBU;
pub const VPU_JSM_STATUS_MVNCI_INTERNAL_ERROR: c_uint = 0xCU;
// @deprecated (use VPU_JSM_STATUS_PREEMPTED_MID_COMMAND instead)
pub const VPU_JSM_STATUS_PREEMPTED_MID_INFERENCE: c_uint = 0xDU;
// Job status returned when the job was preempted mid-command
pub const VPU_JSM_STATUS_PREEMPTED_MID_COMMAND: c_uint = 0xDU;
// Range of status codes that require engine reset
pub const VPU_JSM_STATUS_ENGINE_RESET_REQUIRED_MIN: c_uint = 0xEU;
pub const VPU_JSM_STATUS_MVNCI_CONTEXT_VIOLATION_HW: c_uint = 0xEU;
pub const VPU_JSM_STATUS_MVNCI_PREEMPTION_TIMED_OUT: c_uint = 0xFU;
pub const VPU_JSM_STATUS_ENGINE_RESET_REQUIRED_MAX: c_uint = 0x1FU;
//
// Host <-> VPU IPC channels.
// ASYNC commands use a high priority channel, other messages use low-priority ones.
//
pub const VPU_IPC_CHAN_ASYNC_CMD: c_int = 0;
pub const VPU_IPC_CHAN_GEN_CMD: c_int = 10;
pub const VPU_IPC_CHAN_JOB_RET: c_int = 11;
//
// Job flags bit masks.
//
// Null submission mask.
// When set, batch buffer's commands are not processed but returned as
// successful immediately, except fences and timestamps.
// When cleared, batch buffer's commands are processed normally.
// Used for testing and profiling purposes.
//
// Inline command mask.
// When set, the object in job queue is an inline command (see struct vpu_inline_cmd below).
// When cleared, the object in job queue is a job (see struct vpu_job_queue_entry below).
//
// VPU private data mask.
// Reserved for the VPU to store private data about the job (or inline command)
// while being processed.
//
// Job queue flags bit masks.
//
// No job done notification mask.
// When set, indicates that no job done notification should be sent for any
// job from this queue. When cleared, indicates that job done notification
// should be sent for every job completed from this queue.
//
// Native fence usage mask.
// When set, indicates that job queue uses native fences (as inline commands
// in job queue). Such queues may also use legacy fences (as commands in batch buffers).
// When cleared, indicates the job queue only uses legacy fences.
// NOTES:
// 1. For queues using native fences, VPU expects that all jobs in the queue
// are immediately followed by an inline command object. This object is expected
// to be a fence signal command in most cases, but can also be a NOP in case the host
// does not need per-job fence signalling. Other inline commands objects can be
// inserted between "job and inline command" pairs.
// 2. Native fence queues are only supported on VPU 40xx onwards.
//
// Enable turbo mode for testing NPU performance; not recommended for regular usage.
//
// Queue error detection mode flag
// For 'interactive' queues (this bit not set), the FW will identify queues that have not
// completed a job inside the TDR timeout as in error as part of engine reset sequence.
// For 'non-interactive' queues (this bit set), the FW will identify queues that have not
// progressed the heartbeat inside the non-interactive no-progress timeout as in error as
// part of engine reset sequence. Additionally, there is an upper limit applied to these
// queues: even if they progress the heartbeat, if they run longer than non-interactive
// timeout, then the FW will also identify them as in error.
//
// Max length (including trailing NULL char) of trace entity name (e.g., the
// name of a logging destination or a loggable HW component).
//
pub const VPU_TRACE_ENTITY_NAME_MAX_LEN: c_int = 32;
//
// Max length (including trailing NULL char) of a dyndbg command.
//
// NOTE: 96 is used so that the size of 'struct vpu_ipc_msg' in the JSM API is
// 128 bytes (multiple of 64 bytes, the cache line size).
//
pub const VPU_DYNDBG_CMD_MAX_LEN: c_int = 96;
//
// For HWS command queue scheduling, we can prioritise command queues inside the
// same process with a relative in-process priority. Valid values for relative
// priority are given below - max and min.
//
pub const VPU_HWS_COMMAND_QUEUE_MAX_IN_PROCESS_PRIORITY: c_int = 7;

//
// For HWS priority scheduling, we can have multiple realtime priority bands.
// They are numbered 0 to a MAX.
//

//
// vpu_jsm_engine_reset_context flag definitions
//

pub const VPU_ENGINE_RESET_CONTEXT_HANG_PRIMARY_CAUSE: c_int = 0;
pub const VPU_ENGINE_RESET_CONTEXT_COLLATERAL_DAMAGE: c_int = 1;
//
// Invalid command queue handle identifier. Applies to cmdq_id and cmdq_group
// in this API.
//

//
// Inline commands types.
//
// NOP.
// VPU does nothing other than consuming the inline command object.
//
pub const VPU_INLINE_CMD_TYPE_NOP: c_uint = 0x0;
//
// Fence wait.
// VPU waits for the fence current value to reach monitored value.
// Fence wait operations are executed upon job dispatching. While waiting for
// the fence to be satisfied, VPU blocks fetching of the next objects in the queue.
// Jobs present in the queue prior to the fence wait object may be processed
// concurrently.
//
pub const VPU_INLINE_CMD_TYPE_FENCE_WAIT: c_uint = 0x1;
//
// Fence signal.
// VPU sets the fence current value to the provided value. If new current value
// is equal to or higher than monitored value, VPU sends fence signalled notification
// to the host. Fence signal operations are executed upon completion of all the jobs
// present in the queue prior to them, and in-order relative to each other in the queue.
// But jobs in-between them may be processed concurrently and may complete out-of-order.
//
pub const VPU_INLINE_CMD_TYPE_FENCE_SIGNAL: c_uint = 0x2;
//
// Job scheduling priority bands for both hardware scheduling and OS scheduling.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vpu_job_scheduling_priority_band {
    VPU_JOB_SCHEDULING_PRIORITY_BAND_IDLE = 0,
    VPU_JOB_SCHEDULING_PRIORITY_BAND_NORMAL = 1,
    VPU_JOB_SCHEDULING_PRIORITY_BAND_FOCUS = 2,
    VPU_JOB_SCHEDULING_PRIORITY_BAND_REALTIME = 3,
    VPU_JOB_SCHEDULING_PRIORITY_BAND_COUNT = 4,
}

//
// Job format.
// Jobs defines the actual workloads to be executed by a given engine.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_job_queue_entry {
// Address of VPU commands batch buffer
    pub batch_buf_addr: u64,
// Job ID
    pub job_id: u32,
// Flags bit field, see VPU_JOB_FLAGS_* above
    pub flags: u32,
//
// Doorbell ring timestamp taken by KMD from SoC's global system clock, in
// microseconds. NPU can convert this value to its own fixed clock's timebase,
// to match other profiling timestamps.
//
    pub doorbell_timestamp: u64,
// Extra id for job tracking, used only in the firmware perf traces
    pub host_tracking_id: u64,
// Address of the primary preemption buffer to use for this job
    pub primary_preempt_buf_addr: u64,
// Size of the primary preemption buffer to use for this job
    pub primary_preempt_buf_size: u32,
// Size of secondary preemption buffer to use for this job
    pub secondary_preempt_buf_size: u32,
// Address of secondary preemption buffer to use for this job
    pub secondary_preempt_buf_addr: u64,
    pub reserved_0: u64,
}

//
// Inline command format.
// Inline commands are the commands executed at scheduler level (typically,
// synchronization directives). Inline command and job objects must be of
// the same size and have flags field at same offset.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_inline_cmd {
    pub reserved_0: u64,
// Inline command type, see VPU_INLINE_CMD_TYPE_* defines.
    pub type: u32,
// Flags bit field, see VPU_JOB_FLAGS_* above.
    pub flags: u32,
// Inline command payload. Depends on inline command type.
#[repr(C)]
#[derive(Copy, Clone)]
pub union payload {
// Fence (wait and signal) commands' payload.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fence {
// Fence object handle.
    pub fence_handle: u64,
// User VA of the current fence value.
    pub current_value_va: u64,
// User VA of the monitored fence value (read-only).
    pub monitored_value_va: u64,
// Value to wait for or write in fence location.
    pub value: u64,
// User VA of the log buffer in which to add log entry on completion.
    pub log_buffer_va: u64,
// NPU private data.
    pub npu_private_data: u64,
    pub fence: },
//
// Other commands do not have a payload:
// Payload definition for future inline commands can be inserted here.
//
    pub reserved_1: [u64; 6],
    pub payload: },
}

//
// Job queue slots can be populated either with job objects or inline command objects.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union vpu_jobq_slot {
    pub job: vpu_job_queue_entry,
    pub inline_cmd: vpu_inline_cmd,
}

//
// Job queue control registers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_job_queue_header {
    pub engine_idx: u32,
    pub head: u32,
    pub tail: u32,
    pub flags: u32,
// Set to 1 to indicate priority_band field is valid
    pub priority_band_valid: u32,
//
// Priority for the work of this job queue, valid only if the HWS is NOT used
// and the @ref priority_band_valid is set to 1. It is applied only during
// the @ref VPU_JSM_MSG_REGISTER_DB message processing.
// The device firmware might use the priority_band to optimize the power
// management logic, but it will not affect the order of jobs.
// Available priority bands: @see enum vpu_job_scheduling_priority_band
//
    pub priority_band: u32,
// Inside realtime band assigns a further priority, limited to 0..31 range
    pub realtime_priority_level: u32,
    pub reserved_0: [u32; 9],
}

//
// Job queue format.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_job_queue {
    pub header: vpu_job_queue_header,
    pub slot: [vpu_jobq_slot; ],
}

//
// Logging entity types.
//
// This enum defines the different types of entities involved in logging.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vpu_trace_entity_type {
// Logging destination (entity where logs can be stored / printed).
    VPU_TRACE_ENTITY_TYPE_DESTINATION = 1,
// Loggable HW component (HW entity that can be logged).
    VPU_TRACE_ENTITY_TYPE_HW_COMPONENT = 2,
}

//
// HWS specific log buffer header details.
// Total size is 32 bytes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_hws_log_buffer_header {
// Written by VPU after adding a log entry. Initialised by host to 0.
    pub first_free_entry_index: u32,
// Incremented by VPU every time the VPU writes the 0th entry; initialised by host to 0.
    pub wraparound_count: u32,
//
// This is the number of buffers that can be stored in the log buffer provided by the host.
// It is written by host before passing buffer to VPU. VPU should consider it read-only.
//
    pub num_of_entries: u64,
    pub reserved: [u64; 2],
}

//
// HWS specific log buffer entry details.
// Total size is 32 bytes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_hws_log_buffer_entry {
// VPU timestamp must be an invariant timer tick (not impacted by DVFS)
    pub vpu_timestamp: u64,
//
// Operation type:
// 0 - context state change
// 1 - queue new work
// 2 - queue unwait sync object
// 3 - queue no more work
// 4 - queue wait sync object
//
    pub operation_type: u32,
    pub reserved: u32,
// Operation data depends on operation type
    pub operation_data: [u64; 2],
}

// Native fence log buffer types.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vpu_hws_native_fence_log_type {
    VPU_HWS_NATIVE_FENCE_LOG_TYPE_WAITS = 1,
    VPU_HWS_NATIVE_FENCE_LOG_TYPE_SIGNALS = 2
}

// HWS native fence log buffer header.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_hws_native_fence_log_header {
// Index of the first free entry in buffer.
    pub first_free_entry_idx: u32,
//
// Incremented whenever the NPU wraps around the buffer and writes
// to the first entry again.
//
    pub wraparound_count: u32,
}

// Field allowing atomic update of both fields above.
// Log buffer type, see enum vpu_hws_native_fence_log_type.
// Allocated number of entries in the log buffer.
// Native fence log operation types.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vpu_hws_native_fence_log_op {
    VPU_HWS_NATIVE_FENCE_LOG_OP_SIGNAL_EXECUTED = 0,
    VPU_HWS_NATIVE_FENCE_LOG_OP_WAIT_UNBLOCKED = 1
}

// HWS native fence log entry.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_hws_native_fence_log_entry {
// Newly signaled/unblocked fence value.
    pub fence_value: u64,
// Native fence object handle to which this operation belongs.
    pub fence_handle: u64,
// Operation type, see enum vpu_hws_native_fence_log_op.
    pub op_type: u64,
    pub reserved_0: u64,
//
// VPU_HWS_NATIVE_FENCE_LOG_OP_WAIT_UNBLOCKED only: Timestamp at which fence
// wait was started (in NPU SysTime).
//
    pub fence_wait_start_ts: u64,
    pub reserved_1: u64,
// Timestamp at which fence operation was completed (in NPU SysTime).
    pub fence_end_ts: u64,
}

// Native fence log buffer.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_hws_native_fence_log_buffer {
    pub header: vpu_hws_native_fence_log_header,
    pub entry: [vpu_hws_native_fence_log_entry; ],
}

//
// Host <-> VPU IPC messages types.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vpu_ipc_msg_type {
// Unsupported command
    VPU_JSM_MSG_UNKNOWN = 0xFFFFFFFF,

// IPC Host -> Device, base id for async commands
    VPU_JSM_MSG_ASYNC_CMD = 0x1100,
//
// Reset engine. The NPU cancels all the jobs currently executing on the target
// engine making the engine become idle and then does a HW reset, before returning
// to the host.
// @see struct vpu_ipc_msg_payload_engine_reset
//
    VPU_JSM_MSG_ENGINE_RESET = VPU_JSM_MSG_ASYNC_CMD,
//
// Preempt engine. The NPU stops (preempts) all the jobs currently
// executing on the target engine making the engine become idle and ready to
// execute new jobs.
// NOTE: The NPU does not remove unstarted jobs (if any) from job queues of
// the target engine, but it stops processing them (until the queue doorbell
// is rung again); the host is responsible to reset the job queue, either
// after preemption or when resubmitting jobs to the queue.
// @see vpu_ipc_msg_payload_engine_preempt
//
    VPU_JSM_MSG_ENGINE_PREEMPT = 0x1101,
//
// OS scheduling doorbell register command
// @see vpu_ipc_msg_payload_register_db
//
    VPU_JSM_MSG_REGISTER_DB = 0x1102,
//
// OS scheduling doorbell unregister command
// @see vpu_ipc_msg_payload_unregister_db
//
    VPU_JSM_MSG_UNREGISTER_DB = 0x1103,
//
// Query engine heartbeat. Heartbeat is expected to increase monotonically
// and increase while work is being progressed by NPU.
// @see vpu_ipc_msg_payload_query_engine_hb
//
    VPU_JSM_MSG_QUERY_ENGINE_HB = 0x1104,
    VPU_JSM_MSG_GET_POWER_LEVEL_COUNT = 0x1105,
    VPU_JSM_MSG_GET_POWER_LEVEL = 0x1106,
    VPU_JSM_MSG_SET_POWER_LEVEL = 0x1107,
// @deprecated
    VPU_JSM_MSG_METRIC_STREAMER_OPEN = 0x1108,
// @deprecated
    VPU_JSM_MSG_METRIC_STREAMER_CLOSE = 0x1109,
// Configure logging (used to modify configuration passed in boot params).
    VPU_JSM_MSG_TRACE_SET_CONFIG = 0x110a,
// Return current logging configuration.
    VPU_JSM_MSG_TRACE_GET_CONFIG = 0x110b,
//
// Get masks of destinations and HW components supported by the firmware
// (may vary between HW generations and FW compile
// time configurations)
//
    VPU_JSM_MSG_TRACE_GET_CAPABILITY = 0x110c,
// Get the name of a destination or HW component.
    VPU_JSM_MSG_TRACE_GET_NAME = 0x110d,
//
// Release resource associated with host ssid . All jobs that belong to the host_ssid
// aborted and removed from internal scheduling queues. All doorbells assigned
// to the host_ssid are unregistered and any internal FW resources belonging to
// the host_ssid are released.
// @see vpu_ipc_msg_payload_ssid_release
//
    VPU_JSM_MSG_SSID_RELEASE = 0x110e,
//
// Start collecting metric data.
// @see vpu_jsm_metric_streamer_start
//
    VPU_JSM_MSG_METRIC_STREAMER_START = 0x110f,
//
// Stop collecting metric data. This command will return success if it is called
// for a metric stream that has already been stopped or was never started.
// @see vpu_jsm_metric_streamer_stop
//
    VPU_JSM_MSG_METRIC_STREAMER_STOP = 0x1110,
//
// Update current and next buffer for metric data collection. This command can
// also be used to request information about the number of collected samples
// and the amount of data written to the buffer.
// @see vpu_jsm_metric_streamer_update
//
    VPU_JSM_MSG_METRIC_STREAMER_UPDATE = 0x1111,
//
// Request description of selected metric groups and metric counters within
// each group. The VPU will write the description of groups and counters to
// the buffer specified in the command structure.
// @see vpu_jsm_metric_streamer_start
//
    VPU_JSM_MSG_METRIC_STREAMER_INFO = 0x1112,
//
// Control command: Priority band setup
// @see vpu_ipc_msg_payload_hws_priority_band_setup
//
    VPU_JSM_MSG_SET_PRIORITY_BAND_SETUP = 0x1113,
//
// Control command: Create command queue
// @see vpu_ipc_msg_payload_hws_create_cmdq
//
    VPU_JSM_MSG_CREATE_CMD_QUEUE = 0x1114,
//
// Control command: Destroy command queue
// @see vpu_ipc_msg_payload_hws_destroy_cmdq
//
    VPU_JSM_MSG_DESTROY_CMD_QUEUE = 0x1115,
//
// Control command: Set context scheduling properties
// @see vpu_ipc_msg_payload_hws_set_context_sched_properties
//
    VPU_JSM_MSG_SET_CONTEXT_SCHED_PROPERTIES = 0x1116,
//
// Register a doorbell to notify VPU of new work. The doorbell may later be
// deallocated or reassigned to another context.
// @see vpu_jsm_hws_register_db
//
    VPU_JSM_MSG_HWS_REGISTER_DB = 0x1117,
//
// Control command: Log buffer setting
// @see vpu_ipc_msg_payload_hws_set_scheduling_log
//
    VPU_JSM_MSG_HWS_SET_SCHEDULING_LOG = 0x1118,
//
// Control command: Suspend command queue.
// @see vpu_ipc_msg_payload_hws_suspend_cmdq
//
    VPU_JSM_MSG_HWS_SUSPEND_CMDQ = 0x1119,
//
// Control command: Resume command queue
// @see vpu_ipc_msg_payload_hws_resume_cmdq
//
    VPU_JSM_MSG_HWS_RESUME_CMDQ = 0x111a,
//
// Control command: Resume engine after reset
// @see vpu_ipc_msg_payload_hws_resume_engine
//
    VPU_JSM_MSG_HWS_ENGINE_RESUME = 0x111b,
//
// Control command: Enable survivability/DCT mode
// @see vpu_ipc_msg_payload_pwr_dct_control
//
    VPU_JSM_MSG_DCT_ENABLE = 0x111c,
//
// Control command: Disable survivability/DCT mode
// This command has no payload
//
    VPU_JSM_MSG_DCT_DISABLE = 0x111d,
//
// Reserved command ID to ensure that the following command requests
// responses have the same lower byte value.
//
    VPU_JSM_MSG_RESERVED_111E = 0x111e,
//
// Control command: Configure VPU frequency scaling parameters.
// @see vpu_ipc_msg_payload_freq_config
//
    VPU_JSM_MSG_FREQ_CONFIG = 0x111f,
//
// Dump VPU state. To be used for debug purposes only.
// This command has no payload.
// NOTE: Please introduce new ASYNC commands before this one.
//
    VPU_JSM_MSG_STATE_DUMP = 0x11FF,

// IPC Host -> Device, base id for general commands
    VPU_JSM_MSG_GENERAL_CMD = 0x1200,
// Unsupported command
    VPU_JSM_MSG_BLOB_DEINIT_DEPRECATED = VPU_JSM_MSG_GENERAL_CMD,
//
// Control dyndbg behavior by executing a dyndbg command; equivalent to
// Linux command:
// @verbatim echo '<dyndbg_cmd>' > <debugfs>/dynamic_debug/control @endverbatim
// @see vpu_ipc_msg_payload_dyndbg_control
//
    VPU_JSM_MSG_DYNDBG_CONTROL = 0x1201,
//
// Perform the save procedure for the D0i3 entry
//
    VPU_JSM_MSG_PWR_D0I3_ENTER = 0x1202,

//
// IPC Device -> Host, Job completion
// @see struct vpu_ipc_msg_payload_job_done
//
    VPU_JSM_MSG_JOB_DONE = 0x2100,
//
// IPC Device -> Host, Fence signalled
// @see vpu_ipc_msg_payload_native_fence_signalled
//
    VPU_JSM_MSG_NATIVE_FENCE_SIGNALLED = 0x2101,

// IPC Device -> Host, Async command completion
    VPU_JSM_MSG_ASYNC_CMD_DONE = 0x2200,
//
// IPC Device -> Host, engine reset complete
// @see vpu_ipc_msg_payload_engine_reset_done
//
    VPU_JSM_MSG_ENGINE_RESET_DONE = VPU_JSM_MSG_ASYNC_CMD_DONE,
//
// Preempt complete message
// @see vpu_ipc_msg_payload_engine_preempt_done
//
    VPU_JSM_MSG_ENGINE_PREEMPT_DONE = 0x2201,
    VPU_JSM_MSG_REGISTER_DB_DONE = 0x2202,
    VPU_JSM_MSG_UNREGISTER_DB_DONE = 0x2203,
//
// Response to query engine heartbeat.
// @see vpu_ipc_msg_payload_query_engine_hb_done
//
    VPU_JSM_MSG_QUERY_ENGINE_HB_DONE = 0x2204,
    VPU_JSM_MSG_GET_POWER_LEVEL_COUNT_DONE = 0x2205,
    VPU_JSM_MSG_GET_POWER_LEVEL_DONE = 0x2206,
    VPU_JSM_MSG_SET_POWER_LEVEL_DONE = 0x2207,
// @deprecated
    VPU_JSM_MSG_METRIC_STREAMER_OPEN_DONE = 0x2208,
// @deprecated
    VPU_JSM_MSG_METRIC_STREAMER_CLOSE_DONE = 0x2209,
// Response to VPU_JSM_MSG_TRACE_SET_CONFIG.
    VPU_JSM_MSG_TRACE_SET_CONFIG_RSP = 0x220a,
// Response to VPU_JSM_MSG_TRACE_GET_CONFIG.
    VPU_JSM_MSG_TRACE_GET_CONFIG_RSP = 0x220b,
// Response to VPU_JSM_MSG_TRACE_GET_CAPABILITY.
    VPU_JSM_MSG_TRACE_GET_CAPABILITY_RSP = 0x220c,
// Response to VPU_JSM_MSG_TRACE_GET_NAME.
    VPU_JSM_MSG_TRACE_GET_NAME_RSP = 0x220d,
//
// Response to VPU_JSM_MSG_SSID_RELEASE.
// @see vpu_ipc_msg_payload_ssid_release
//
    VPU_JSM_MSG_SSID_RELEASE_DONE = 0x220e,
//
// Response to VPU_JSM_MSG_METRIC_STREAMER_START.
// VPU will return an error result if metric collection cannot be started,
// e.g. when the specified metric mask is invalid.
// @see vpu_jsm_metric_streamer_done
//
    VPU_JSM_MSG_METRIC_STREAMER_START_DONE = 0x220f,
//
// Response to VPU_JSM_MSG_METRIC_STREAMER_STOP.
// Returns information about collected metric data.
// @see vpu_jsm_metric_streamer_done
//
    VPU_JSM_MSG_METRIC_STREAMER_STOP_DONE = 0x2210,
//
// Response to VPU_JSM_MSG_METRIC_STREAMER_UPDATE.
// Returns information about collected metric data.
// @see vpu_jsm_metric_streamer_done
//
    VPU_JSM_MSG_METRIC_STREAMER_UPDATE_DONE = 0x2211,
//
// Response to VPU_JSM_MSG_METRIC_STREAMER_INFO.
// Returns a description of the metric groups and metric counters.
// @see vpu_jsm_metric_streamer_done
//
    VPU_JSM_MSG_METRIC_STREAMER_INFO_DONE = 0x2212,
//
// Asynchronous event sent from the VPU to the host either when the current
// metric buffer is full or when the VPU has collected a multiple of
// @ref vpu_jsm_metric_streamer_start::notify_sample_count samples as indicated
// through the start command (VPU_JSM_MSG_METRIC_STREAMER_START). Returns
// information about collected metric data.
// @see vpu_jsm_metric_streamer_done
//
    VPU_JSM_MSG_METRIC_STREAMER_NOTIFICATION = 0x2213,
//
// Response to control command: Priority band setup
// @see vpu_ipc_msg_payload_hws_priority_band_setup
//
    VPU_JSM_MSG_SET_PRIORITY_BAND_SETUP_RSP = 0x2214,
//
// Response to control command: Create command queue
// @see vpu_ipc_msg_payload_hws_create_cmdq_rsp
//
    VPU_JSM_MSG_CREATE_CMD_QUEUE_RSP = 0x2215,
//
// Response to control command: Destroy command queue
// @see vpu_ipc_msg_payload_hws_destroy_cmdq
//
    VPU_JSM_MSG_DESTROY_CMD_QUEUE_RSP = 0x2216,
//
// Response to control command: Set context scheduling properties
// @see vpu_ipc_msg_payload_hws_set_context_sched_properties
//
    VPU_JSM_MSG_SET_CONTEXT_SCHED_PROPERTIES_RSP = 0x2217,
//
// Response to control command: Log buffer setting
// @see vpu_ipc_msg_payload_hws_set_scheduling_log
//
    VPU_JSM_MSG_HWS_SET_SCHEDULING_LOG_RSP = 0x2218,
//
// IPC Device -> Host, HWS notify index entry of log buffer written
// @see vpu_ipc_msg_payload_hws_scheduling_log_notification
//
    VPU_JSM_MSG_HWS_SCHEDULING_LOG_NOTIFICATION = 0x2219,
//
// IPC Device -> Host, HWS completion of a context suspend request
// @see vpu_ipc_msg_payload_hws_suspend_cmdq
//
    VPU_JSM_MSG_HWS_SUSPEND_CMDQ_DONE = 0x221a,
//
// Response to control command: Resume command queue
// @see vpu_ipc_msg_payload_hws_resume_cmdq
//
    VPU_JSM_MSG_HWS_RESUME_CMDQ_RSP = 0x221b,
//
// Response to control command: Resume engine command response
// @see vpu_ipc_msg_payload_hws_resume_engine
//
    VPU_JSM_MSG_HWS_RESUME_ENGINE_DONE = 0x221c,
//
// Response to control command: Enable survivability/DCT mode
// This command has no payload
//
    VPU_JSM_MSG_DCT_ENABLE_DONE = 0x221d,
//
// Response to control command: Disable survivability/DCT mode
// This command has no payload
//
    VPU_JSM_MSG_DCT_DISABLE_DONE = 0x221e,
//
// Response to control command: Configure VPU frequency scaling parameters.
// @see vpu_ipc_msg_payload_freq_config
//
    VPU_JSM_MSG_FREQ_CONFIG_RSP = 0x221f,
//
// Response to state dump control command.
// This command has no payload.
// NOTE: Please introduce new ASYNC responses before this one.
//
    VPU_JSM_MSG_STATE_DUMP_RSP = 0x22FF,

// IPC Device -> Host, General command completion
    VPU_JSM_MSG_GENERAL_CMD_DONE = 0x2300,
    VPU_JSM_MSG_BLOB_DEINIT_DONE = VPU_JSM_MSG_GENERAL_CMD_DONE,
// Response to VPU_JSM_MSG_DYNDBG_CONTROL.
    VPU_JSM_MSG_DYNDBG_CONTROL_RSP = 0x2301,
//
// Acknowledgment of completion of the save procedure initiated by
// VPU_JSM_MSG_PWR_D0I3_ENTER
//
    VPU_JSM_MSG_PWR_D0I3_ENTER_DONE = 0x2302,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vpu_ipc_msg_status {

//
// Engine reset request payload
// @see VPU_JSM_MSG_ENGINE_RESET
//
    struct vpu_ipc_msg_payload_engine_reset {
// Engine to be reset.
    u32 engine_idx;
// Reserved
    u32 reserved_0;
}

//
// Engine preemption request struct
// @see VPU_JSM_MSG_ENGINE_PREEMPT
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_ipc_msg_payload_engine_preempt {
// Engine to be preempted.
    pub engine_idx: u32,
// ID of the preemption request.
    pub preempt_id: u32,
}

//
// Register doorbell command structure.
// This structure supports doorbell registration for only OS scheduling.
// @see VPU_JSM_MSG_REGISTER_DB
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_ipc_msg_payload_register_db {
// Index of the doorbell to register.
    pub db_idx: u32,
// Reserved
    pub reserved_0: u32,
// Virtual address in Global GTT pointing to the start of job queue.
    pub jobq_base: u64,
// Size of the job queue in bytes.
    pub jobq_size: u32,
// Host sub-stream ID for the context assigned to the doorbell.
    pub host_ssid: u32,
}

//
// Unregister doorbell command structure.
// Request structure to unregister a doorbell for both HW and OS scheduling.
// @see VPU_JSM_MSG_UNREGISTER_DB
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_ipc_msg_payload_unregister_db {
// Index of the doorbell to unregister.
    pub db_idx: u32,
// Reserved
    pub reserved_0: u32,
}

//
// Heartbeat request structure
// @see VPU_JSM_MSG_QUERY_ENGINE_HB
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_ipc_msg_payload_query_engine_hb {
// Engine to return heartbeat value.
    pub engine_idx: u32,
// Reserved
    pub reserved_0: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_ipc_msg_payload_power_level {
//
// Requested power level. The power level value is in the
// range [0, power_level_count-1] where power_level_count
// is the number of available power levels as returned by
// the get power level count command. A power level of 0
// corresponds to the maximum possible power level, while
// power_level_count-1 corresponds to the minimum possible
// power level. Values outside of this range are not
// considered to be valid.
//
    pub power_level: u32,
// Reserved
    pub reserved_0: u32,
}

//
// Structure for requesting ssid release
// @see VPU_JSM_MSG_SSID_RELEASE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_ipc_msg_payload_ssid_release {
// Host sub-stream ID for the context to be released.
    pub host_ssid: u32,
// Reserved
    pub reserved_0: u32,
}

//
// @brief Metric streamer start command structure.
// This structure is also used with VPU_JSM_MSG_METRIC_STREAMER_INFO to request metric
// groups and metric counters description from the firmware.
// @see VPU_JSM_MSG_METRIC_STREAMER_START
// @see VPU_JSM_MSG_METRIC_STREAMER_INFO
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_jsm_metric_streamer_start {
//
// Bitmask to select the desired metric groups.
// A metric group can belong only to one metric streamer instance at a time.
// Since each metric streamer instance has a unique set of metric groups, it
// can also identify a metric streamer instance if more than one instance was
// started. If the VPU device does not support multiple metric streamer instances,
// then VPU_JSM_MSG_METRIC_STREAMER_START will return an error even if the second
// instance has different groups to the first.
//
    pub metric_group_mask: u64,
// Sampling rate in nanoseconds.
    pub sampling_rate: u64,
//
// If > 0 the VPU will send a VPU_JSM_MSG_METRIC_STREAMER_NOTIFICATION message
// after every @ref notify_sample_count samples is collected or dropped by the VPU.
// If set to UINT_MAX the VPU will only generate a notification when the metric
// buffer is full. If set to 0 the VPU will never generate a notification.
//
    pub notify_sample_count: u32,
    pub reserved_0: u32,
//
// Address and size of the buffer where the VPU will write metric data. The
// VPU writes all counters from enabled metric groups one after another. If
// there is no space left to write data at the next sample period the VPU
// will switch to the next buffer (@ref next_buffer_addr) and will optionally
// send a notification to the host driver if @ref notify_sample_count is non-zero.
// If @ref next_buffer_addr is NULL the VPU will stop collecting metric data.
//
    pub buffer_addr: u64,
    pub buffer_size: u64,
//
// Address and size of the next buffer to write metric data to after the initial
// buffer is full. If the address is NULL the VPU will stop collecting metric
// data.
//
    pub next_buffer_addr: u64,
    pub next_buffer_size: u64,
}

//
// @brief Metric streamer stop command structure.
// @see VPU_JSM_MSG_METRIC_STREAMER_STOP
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_jsm_metric_streamer_stop {
// Bitmask to select the desired metric groups.
    pub metric_group_mask: u64,
}

//
// Provide VPU FW with buffers to write metric data.
// @see VPU_JSM_MSG_METRIC_STREAMER_UPDATE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_jsm_metric_streamer_update {
// Metric group mask that identifies metric streamer instance.
    pub metric_group_mask: u64,
//
// Address and size of the buffer where the VPU will write metric data.
// This member dictates how the update operation should perform:
// 1. client needs information about the number of collected samples and the
// amount of data written to the current buffer
// 2. client wants to switch to a new buffer
//
// Case 1. is identified by the buffer address being 0 or the same as the
// currently used buffer address. In this case the buffer size is ignored and
// the size of the current buffer is unchanged. The VPU will return an update
// in the vpu_jsm_metric_streamer_done structure. The internal writing position
// into the buffer is not changed.
//
// Case 2. is identified by the address being non-zero and differs from the
// current buffer address. The VPU will immediately switch data collection to
// the new buffer. Then the VPU will return an update in the
// vpu_jsm_metric_streamer_done structure.
//
    pub buffer_addr: u64,
    pub buffer_size: u64,
//
// Address and size of the next buffer to write metric data after the initial
// buffer is full. If the address is NULL the VPU will stop collecting metric
// data but will continue to record dropped samples.
//
// Note that there is a hazard possible if both buffer_addr and the next_buffer_addr
// are non-zero in same update request. It is the host's responsibility to ensure
// that both addresses make sense even if the VPU just switched to writing samples
// from the current to the next buffer.
//
    pub next_buffer_addr: u64,
    pub next_buffer_size: u64,
}

//
// Device -> host job completion message.
// @see VPU_JSM_MSG_JOB_DONE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_ipc_msg_payload_job_done {
// Engine to which the job was submitted.
    pub engine_idx: u32,
// Index of the doorbell to which the job was submitted
    pub db_idx: u32,
// ID of the completed job
    pub job_id: u32,
// Status of the completed job
    pub job_status: u32,
// Host SSID
    pub host_ssid: u32,
// Zero Padding
    pub reserved_0: u32,
// Command queue id
    pub cmdq_id: u64,
}

//
// Notification message upon native fence signalling.
// @see VPU_JSM_MSG_NATIVE_FENCE_SIGNALLED
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_ipc_msg_payload_native_fence_signalled {
// Engine ID.
    pub engine_idx: u32,
// Host SSID.
    pub host_ssid: u32,
// CMDQ ID
    pub cmdq_id: u64,
// Fence object handle.
    pub fence_handle: u64,
}

//
// vpu_ipc_msg_payload_engine_reset_done will contain an array of this structure
// which contains which queues caused reset if FW was able to detect any error.
// @see vpu_ipc_msg_payload_engine_reset_done
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_jsm_engine_reset_context {
// Host SSID
    pub host_ssid: u32,
// Zero Padding
    pub reserved_0: u32,
// Command queue id
    pub cmdq_id: u64,
// See VPU_ENGINE_RESET_CONTEXT_* defines
    pub flags: u64,
}

//
// Engine reset response.
// @see VPU_JSM_MSG_ENGINE_RESET_DONE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_ipc_msg_payload_engine_reset_done {
// Engine ordinal
    pub engine_idx: u32,
// Number of impacted contexts
    pub num_impacted_contexts: u32,
// Array of impacted command queue ids and their flags
}

//
// Preemption response struct
// @see VPU_JSM_MSG_ENGINE_PREEMPT_DONE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_ipc_msg_payload_engine_preempt_done {
// Engine preempted.
    pub engine_idx: u32,
// ID of the preemption request.
    pub preempt_id: u32,
}

//
// Response structure for register doorbell command for both OS
// and HW scheduling.
// @see VPU_JSM_MSG_REGISTER_DB
// @see VPU_JSM_MSG_HWS_REGISTER_DB
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_ipc_msg_payload_register_db_done {
// Index of the registered doorbell.
    pub db_idx: u32,
// Reserved
    pub reserved_0: u32,
}

//
// Response structure for unregister doorbell command for both OS
// and HW scheduling.
// @see VPU_JSM_MSG_UNREGISTER_DB
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_ipc_msg_payload_unregister_db_done {
// Index of the unregistered doorbell.
    pub db_idx: u32,
// Reserved
    pub reserved_0: u32,
}

//
// Structure for heartbeat response
// @see VPU_JSM_MSG_QUERY_ENGINE_HB_DONE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_ipc_msg_payload_query_engine_hb_done {
// Engine returning heartbeat value.
    pub engine_idx: u32,
// Reserved
    pub reserved_0: u32,
// Heartbeat value.
    pub heartbeat: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_ipc_msg_payload_get_power_level_count_done {
//
// Number of supported power levels. The maximum possible
// value of power_level_count is 16 but this may vary across
// implementations.
//
    pub power_level_count: u32,
// Reserved
    pub reserved_0: u32,
//
// Power consumption limit for each supported power level in
// [0-100%] range relative to power level 0.
//
    pub power_limit: [u8; 16],
}

//
// HWS priority band setup request / response
// @see VPU_JSM_MSG_SET_PRIORITY_BAND_SETUP
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_ipc_msg_payload_hws_priority_band_setup {
//
// Grace period in 100ns units when preempting another priority band for
// this priority band
//
    pub grace_period: [u32; VPU_HWS_NUM_PRIORITY_BANDS],
//
// Default quantum in 100ns units for scheduling across processes
// within a priority band
// Minimum value supported by NPU is 1ms (10000 in 100ns units).
//
    pub process_quantum: [u32; VPU_HWS_NUM_PRIORITY_BANDS],
//
// Default grace period in 100ns units for processes that preempt each
// other within a priority band
//
    pub process_grace_period: [u32; VPU_HWS_NUM_PRIORITY_BANDS],
//
// For normal priority band, specifies the target VPU percentage
// in situations when it's starved by the focus band.
//
    pub normal_band_percentage: u32,
//
// TDR timeout value in milliseconds. Default value of 0 meaning no timeout.
//
    pub tdr_timeout: u32,
// Non-interactive queue timeout for no progress of heartbeat in milliseconds.
// Default value of 0 meaning no timeout.
//
    pub non_interactive_no_progress_timeout: u32,
//
// Non-interactive queue upper limit timeout value in milliseconds. Default
// value of 0 meaning no timeout.
//
    pub non_interactive_timeout: u32,
}

//
// @brief HWS create command queue request.
// Host will create a command queue via this command.
// Note: Cmdq group is a handle of an object which
// may contain one or more command queues.
// @see VPU_JSM_MSG_CREATE_CMD_QUEUE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_ipc_msg_payload_hws_create_cmdq {
// Process id
    pub process_id: u64,
// Host SSID
    pub host_ssid: u32,
// Engine for which queue is being created
    pub engine_idx: u32,
// Cmdq group: only used for HWS logging of state changes
    pub cmdq_group: u64,
// Command queue id
    pub cmdq_id: u64,
// Command queue base
    pub cmdq_base: u64,
// Command queue size
    pub cmdq_size: u32,
// Zero padding
    pub reserved_0: u32,
}

//
// HWS create command queue response.
// @see VPU_JSM_MSG_CREATE_CMD_QUEUE_RSP
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_ipc_msg_payload_hws_create_cmdq_rsp {
// Process id
    pub process_id: u64,
// Host SSID
    pub host_ssid: u32,
// Engine for which queue is being created
    pub engine_idx: u32,
// Command queue group
    pub cmdq_group: u64,
// Command queue id
    pub cmdq_id: u64,
}

//
// HWS destroy command queue request / response
// @see VPU_JSM_MSG_DESTROY_CMD_QUEUE
// @see VPU_JSM_MSG_DESTROY_CMD_QUEUE_RSP
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_ipc_msg_payload_hws_destroy_cmdq {
// Host SSID
    pub host_ssid: u32,
// Zero Padding
    pub reserved: u32,
// Command queue id
    pub cmdq_id: u64,
}

//
// HWS set context scheduling properties request / response
// @see VPU_JSM_MSG_SET_CONTEXT_SCHED_PROPERTIES
// @see VPU_JSM_MSG_SET_CONTEXT_SCHED_PROPERTIES_RSP
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_ipc_msg_payload_hws_set_context_sched_properties {
// Host SSID
    pub host_ssid: u32,
// Zero Padding
    pub reserved_0: u32,
// Command queue id
    pub cmdq_id: u64,
//
// Priority band to assign to work of this context.
// Available priority bands: @see enum vpu_job_scheduling_priority_band
//
    pub priority_band: u32,
// Inside realtime band assigns a further priority
    pub realtime_priority_level: u32,
// Priority relative to other contexts in the same process
    pub in_process_priority: i32,
// Zero padding / Reserved
    pub reserved_1: u32,
//
// Context quantum relative to other contexts of same priority in the same process
// Minimum value supported by NPU is 1ms (10000 in 100ns units).
//
    pub context_quantum: u64,
// Grace period when preempting context of the same priority within the same process
    pub grace_period_same_priority: u64,
// Grace period when preempting context of a lower priority within the same process
    pub grace_period_lower_priority: u64,
}

//
// Register doorbell command structure.
// This structure supports doorbell registration for both HW and OS scheduling.
// Note: Queue base and size are added here so that the same structure can be used for
// OS scheduling and HW scheduling. For OS scheduling, cmdq_id will be ignored
// and cmdq_base and cmdq_size will be used. For HW scheduling, cmdq_base and cmdq_size will be
// ignored and cmdq_id is used.
// @see VPU_JSM_MSG_HWS_REGISTER_DB
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_jsm_hws_register_db {
// Index of the doorbell to register.
    pub db_id: u32,
// Host sub-stream ID for the context assigned to the doorbell.
    pub host_ssid: u32,
// ID of the command queue associated with the doorbell.
    pub cmdq_id: u64,
// Virtual address pointing to the start of command queue.
    pub cmdq_base: u64,
// Size of the command queue in bytes.
    pub cmdq_size: u64,
}

//
// Structure to set another buffer to be used for scheduling-related logging.
// The size of the logging buffer and the number of entries is defined as part of the
// buffer itself as described next.
// The log buffer received from the host is made up of;
// - header:     32 bytes in size, as shown in @ref vpu_hws_log_buffer_header.
// The header contains the number of log entries in the buffer.
// - log entry:  0 to n-1, each log entry is 32 bytes in size, as shown in
// @ref vpu_hws_log_buffer_entry.
// The entry contains the VPU timestamp, operation type and data.
// The host should provide the notify index value of log buffer to VPU. This is a
// value defined within the log buffer and when written to will generate the
// scheduling log notification.
// The host should set engine_idx and vpu_log_buffer_va to 0 to disable logging
// for a particular engine.
// VPU will handle one log buffer for each of supported engines.
// VPU should allow the logging to consume one host_ssid.
// @see VPU_JSM_MSG_HWS_SET_SCHEDULING_LOG
// @see VPU_JSM_MSG_HWS_SET_SCHEDULING_LOG_RSP
// @see VPU_JSM_MSG_HWS_SCHEDULING_LOG_NOTIFICATION
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_ipc_msg_payload_hws_set_scheduling_log {
// Engine ordinal
    pub engine_idx: u32,
// Host SSID
    pub host_ssid: u32,
//
// VPU log buffer virtual address.
// Set to 0 to disable logging for this engine.
//
    pub vpu_log_buffer_va: u64,
//
// Notify index of log buffer. VPU_JSM_MSG_HWS_SCHEDULING_LOG_NOTIFICATION
// is generated when an event log is written to this index.
//
    pub notify_index: u64,
//
// Field is now deprecated, will be removed when KMD is updated to support removal
//
    pub enable_extra_events: u32,
// Zero Padding
    pub reserved_0: u32,
}

//
// The scheduling log notification is generated by VPU when it writes
// an event into the log buffer at the notify_index. VPU notifies host with
// VPU_JSM_MSG_HWS_SCHEDULING_LOG_NOTIFICATION. This is an asynchronous
// message from VPU to host.
// @see VPU_JSM_MSG_HWS_SCHEDULING_LOG_NOTIFICATION
// @see VPU_JSM_MSG_HWS_SET_SCHEDULING_LOG
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_ipc_msg_payload_hws_scheduling_log_notification {
// Engine ordinal
    pub engine_idx: u32,
// Zero Padding
    pub reserved_0: u32,
}

//
// HWS suspend command queue request and done structure.
// Host will request the suspend of contexts and VPU will;
// - Suspend all work on this context
// - Preempt any running work
// - Asynchronously perform the above and return success immediately once
// all items above are started successfully
// - Notify the host of completion of these operations via
// VPU_JSM_MSG_HWS_SUSPEND_CMDQ_DONE
// - Reject any other context operations on a context with an in-flight
// suspend request running
// Same structure used when VPU notifies host of completion of a context suspend
// request. The ids and suspend fence value reported in this command will match
// the one in the request from the host to suspend the context. Once suspend is
// complete, VPU will not access any data relating to this command queue until
// it is resumed.
// @see VPU_JSM_MSG_HWS_SUSPEND_CMDQ
// @see VPU_JSM_MSG_HWS_SUSPEND_CMDQ_DONE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_ipc_msg_payload_hws_suspend_cmdq {
// Host SSID
    pub host_ssid: u32,
// Zero Padding
    pub reserved_0: u32,
// Command queue id
    pub cmdq_id: u64,
//
// Suspend fence value - reported by the VPU suspend context
// completed once suspend is complete.
//
    pub suspend_fence_value: u64,
}

//
// HWS Resume command queue request / response structure.
// Host will request the resume of a context;
// - VPU will resume all work on this context
// - Scheduler will allow this context to be scheduled
// @see VPU_JSM_MSG_HWS_RESUME_CMDQ
// @see VPU_JSM_MSG_HWS_RESUME_CMDQ_RSP
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_ipc_msg_payload_hws_resume_cmdq {
// Host SSID
    pub host_ssid: u32,
// Zero Padding
    pub reserved_0: u32,
// Command queue id
    pub cmdq_id: u64,
}

//
// HWS Resume engine request / response structure.
// After a HWS engine reset, all scheduling is stopped on VPU until an engine resume.
// Host shall send this command to resume scheduling of any valid queue.
// @see VPU_JSM_MSG_HWS_ENGINE_RESUME
// @see VPU_JSM_MSG_HWS_RESUME_ENGINE_DONE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_ipc_msg_payload_hws_resume_engine {
// Engine to be resumed
    pub engine_idx: u32,
// Reserved
    pub reserved_0: u32,
}

//
// Payload for VPU_JSM_MSG_TRACE_SET_CONFIG[_RSP] and
// VPU_JSM_MSG_TRACE_GET_CONFIG_RSP messages.
//
// The payload is interpreted differently depending on the type of message:
//
// - For VPU_JSM_MSG_TRACE_SET_CONFIG, the payload specifies the desired
// logging configuration to be set.
//
// - For VPU_JSM_MSG_TRACE_SET_CONFIG_RSP, the payload reports the logging
// configuration that was set after a VPU_JSM_MSG_TRACE_SET_CONFIG request.
// The host can compare this payload with the one it sent in the
// VPU_JSM_MSG_TRACE_SET_CONFIG request to check whether or not the
// configuration was set as desired.
//
// - VPU_JSM_MSG_TRACE_GET_CONFIG_RSP, the payload reports the current logging
// configuration.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_ipc_msg_payload_trace_config {
//
// Logging level (currently set or to be set); see 'mvLog_t' enum for
// acceptable values. The specified logging level applies to all
// destinations and HW components
//
    pub trace_level: u32,
//
// Bitmask of logging destinations (currently enabled or to be enabled);
// bitwise OR of values defined in logging_destination enum.
//
    pub trace_destination_mask: u32,
//
// Bitmask of loggable HW components (currently enabled or to be enabled);
// bitwise OR of values defined in loggable_hw_component enum.
//
    pub trace_hw_component_mask: u64,
    pub /: *mut *mut *mut u64 reserved_0; /< Reserved for future extensions.,
}

//
// Payload for VPU_JSM_MSG_TRACE_GET_CAPABILITY_RSP messages.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_ipc_msg_payload_trace_capability_rsp {
    pub /: *mut *mut *mut u32 trace_destination_mask; /< Bitmask of supported logging destinations.,
    pub reserved_0: u32,
    pub /: *mut *mut *mut u64 trace_hw_component_mask; /< Bitmask of supported loggable HW components.,
    pub /: *mut *mut *mut u64 reserved_1; /< Reserved for future extensions.,
}

//
// Payload for VPU_JSM_MSG_TRACE_GET_NAME requests.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_ipc_msg_payload_trace_get_name {
//
// The type of the entity to query name for; see logging_entity_type for
// possible values.
//
    pub entity_type: u32,
    pub reserved_0: u32,
//
// The ID of the entity to query name for; possible values depends on the
// entity type.
//
    pub entity_id: u64,
}

//
// Payload for VPU_JSM_MSG_TRACE_GET_NAME_RSP responses.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_ipc_msg_payload_trace_get_name_rsp {
//
// The type of the entity whose name was queried; see logging_entity_type
// for possible values.
//
    pub entity_type: u32,
    pub reserved_0: u32,
//
// The ID of the entity whose name was queried; possible values depends on
// the entity type.
//
    pub entity_id: u64,
// Reserved for future extensions.
    pub reserved_1: u64,
// The name of the entity.
    pub entity_name: [c_char; VPU_TRACE_ENTITY_NAME_MAX_LEN],
}

//
// Data sent from the VPU to the host in all metric streamer response messages
// and in asynchronous notification.
// @see VPU_JSM_MSG_METRIC_STREAMER_START_DONE
// @see VPU_JSM_MSG_METRIC_STREAMER_STOP_DONE
// @see VPU_JSM_MSG_METRIC_STREAMER_UPDATE_DONE
// @see VPU_JSM_MSG_METRIC_STREAMER_INFO_DONE
// @see VPU_JSM_MSG_METRIC_STREAMER_NOTIFICATION
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_jsm_metric_streamer_done {
// Metric group mask that identifies metric streamer instance.
    pub metric_group_mask: u64,
//
// Size in bytes of single sample - total size of all enabled counters.
// Some VPU implementations may align sample_size to more than 8 bytes.
//
    pub sample_size: u32,
    pub reserved_0: u32,
//
// Number of samples collected since the metric streamer was started.
// This will be 0 if the metric streamer was not started.
//
    pub samples_collected: u32,
//
// Number of samples dropped since the metric streamer was started. This
// is incremented every time the metric streamer is not able to write
// collected samples because the current buffer is full and there is no
// next buffer to switch to.
//
    pub samples_dropped: u32,
// Address of the buffer that contains the latest metric data.
    pub buffer_addr: u64,
//
// Number of bytes written into the metric data buffer. In response to the
// VPU_JSM_MSG_METRIC_STREAMER_INFO request this field contains the size of
// all group and counter descriptors. The size is updated even if the buffer
// in the request was NULL or too small to hold descriptors of all counters
//
    pub bytes_written: u64,
}

//
// Metric group description placed in the metric buffer after successful completion
// of the VPU_JSM_MSG_METRIC_STREAMER_INFO command. This is followed by one or more
// @ref vpu_jsm_metric_counter_descriptor records.
// @see VPU_JSM_MSG_METRIC_STREAMER_INFO
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_jsm_metric_group_descriptor {
//
// Offset to the next metric group (8-byte aligned). If this offset is 0 this
// is the last descriptor. The value of metric_info_size must be greater than
// or equal to sizeof(struct vpu_jsm_metric_group_descriptor) + name_string_size
// + description_string_size and must be 8-byte aligned.
//
    pub next_metric_group_info_offset: u32,
//
// Offset to the first metric counter description record (8-byte aligned).
// @see vpu_jsm_metric_counter_descriptor
//
    pub next_metric_counter_info_offset: u32,
// Index of the group. This corresponds to bit index in metric_group_mask.
    pub group_id: u32,
// Number of counters in the metric group.
    pub num_counters: u32,
// Data size for all counters, must be a multiple of 8 bytes.
    pub metric_group_data_size: u32,
//
// Metric group domain number. Cannot use multiple, simultaneous metric groups
// from the same domain.
//
    pub domain: u32,
//
// Counter name string size. The string must include a null termination character.
// The FW may use a fixed size name or send a different name for each counter.
// If the VPU uses fixed size strings, all characters from the end of the name
// to the of the fixed size character array must be zeroed.
//
    pub name_string_size: u32,
// Counter description string size, @see name_string_size
    pub description_string_size: u32,
    pub reserved_0: u64,
//
// Right after this structure, the VPU writes name and description of
// the metric group.
//
}

//
// Metric counter description, placed in the buffer after vpu_jsm_metric_group_descriptor.
// @see VPU_JSM_MSG_METRIC_STREAMER_INFO
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_jsm_metric_counter_descriptor {
//
// Offset to the next counter in a group (8-byte aligned). If this offset is
// 0 this is the last counter in the group.
//
    pub next_metric_counter_info_offset: u32,
//
// Offset to the counter data from the start of samples in this metric group.
// Note that metric_data_offset % metric_data_size must be 0.
//
    pub metric_data_offset: u32,
// Size of the metric counter data in bytes.
    pub metric_data_size: u32,
// Metric type, see Level Zero API for definitions.
    pub tier: u32,
// Metric type, see set_metric_type_t for definitions.
    pub metric_type: u32,
// Metric type, see set_value_type_t for definitions.
    pub metric_value_type: u32,
//
// Counter name string size. The string must include a null termination character.
// The FW may use a fixed size name or send a different name for each counter.
// If the VPU uses fixed size strings, all characters from the end of the name
// to the of the fixed size character array must be zeroed.
//
    pub name_string_size: u32,
// Counter description string size, @see name_string_size
    pub description_string_size: u32,
// Counter component name string size, @see name_string_size
    pub component_string_size: u32,
// Counter string size, @see name_string_size
    pub units_string_size: u32,
    pub reserved_0: u64,
//
// Right after this structure, the VPU writes name, description
// component and unit strings.
//
}

//
// Payload for @ref VPU_JSM_MSG_DYNDBG_CONTROL requests.
//
// VPU_JSM_MSG_DYNDBG_CONTROL requests are used to control the VPU FW dynamic debug
// feature, which allows developers to selectively enable/disable code to obtain
// additional FW information. This is equivalent to the dynamic debug functionality
// provided by Linux. The host can control dynamic debug behavior by sending dyndbg
// commands, using the same syntax as for Linux dynamic debug commands.
//
// @see https://www.kernel.org/doc/html/latest/admin-guide/dynamic-debug-howto.html.
//
// NOTE:
// As the dynamic debug feature uses MVLOG messages to provide information, the host
// must first set the logging level to MVLOG_DEBUG, using the @ref VPU_JSM_MSG_TRACE_SET_CONFIG
// command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_ipc_msg_payload_dyndbg_control {
//
// Dyndbg command to be executed.
//
    pub dyndbg_cmd: [c_char; VPU_DYNDBG_CMD_MAX_LEN],
}

//
// Payload for VPU_JSM_MSG_PWR_D0I3_ENTER
//
// This is a bi-directional payload.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_ipc_msg_payload_pwr_d0i3_enter {
//
// 0: VPU_JSM_MSG_PWR_D0I3_ENTER_DONE is not sent to the host driver
// The driver will poll for D0i2 Idle state transitions.
// 1: VPU_JSM_MSG_PWR_D0I3_ENTER_DONE is sent after VPU state save is complete
//
    pub send_response: u32,
    pub reserved_0: u32,
}

//
// Payload for @ref VPU_JSM_MSG_DCT_ENABLE message.
//
// Default values for DCT active/inactive times are 5.3ms and 30ms respectively,
// corresponding to a 85% duty cycle. This payload allows the host to tune these
// values according to application requirements.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_ipc_msg_payload_pwr_dct_control {
// Duty cycle active time in microseconds
    pub dct_active_us: u32,
// Duty cycle inactive time in microseconds
    pub dct_inactive_us: u32,
}

//
// Payload for @ref VPU_JSM_MSG_FREQ_CONFIG message.
//
// This payload allows the host to configure the VPU frequency scaling parameters.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_ipc_msg_payload_freq_config {
// Minimum frequency PLL ratio
    pub min_freq_pll_ratio: u32,
// Efficiency frequency PLL ratio
    pub pn_freq_pll_ratio: u32,
// Maximum frequency PLL ratio
    pub max_freq_pll_ratio: u32,
// Reserved for 64-bit alignment
    pub reserved_0: u32,
}

//
// Payloads union, used to define complete message format.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union vpu_ipc_msg_payload {
    pub engine_reset: vpu_ipc_msg_payload_engine_reset,
    pub engine_preempt: vpu_ipc_msg_payload_engine_preempt,
    pub register_db: vpu_ipc_msg_payload_register_db,
    pub unregister_db: vpu_ipc_msg_payload_unregister_db,
    pub query_engine_hb: vpu_ipc_msg_payload_query_engine_hb,
    pub power_level: vpu_ipc_msg_payload_power_level,
    pub metric_streamer_start: vpu_jsm_metric_streamer_start,
    pub metric_streamer_stop: vpu_jsm_metric_streamer_stop,
    pub metric_streamer_update: vpu_jsm_metric_streamer_update,
    pub ssid_release: vpu_ipc_msg_payload_ssid_release,
    pub hws_register_db: vpu_jsm_hws_register_db,
    pub job_done: vpu_ipc_msg_payload_job_done,
    pub native_fence_signalled: vpu_ipc_msg_payload_native_fence_signalled,
    pub engine_reset_done: vpu_ipc_msg_payload_engine_reset_done,
    pub engine_preempt_done: vpu_ipc_msg_payload_engine_preempt_done,
    pub register_db_done: vpu_ipc_msg_payload_register_db_done,
    pub unregister_db_done: vpu_ipc_msg_payload_unregister_db_done,
    pub query_engine_hb_done: vpu_ipc_msg_payload_query_engine_hb_done,
    pub get_power_level_count_done: vpu_ipc_msg_payload_get_power_level_count_done,
    pub metric_streamer_done: vpu_jsm_metric_streamer_done,
    pub trace_config: vpu_ipc_msg_payload_trace_config,
    pub trace_capability: vpu_ipc_msg_payload_trace_capability_rsp,
    pub trace_get_name: vpu_ipc_msg_payload_trace_get_name,
    pub trace_get_name_rsp: vpu_ipc_msg_payload_trace_get_name_rsp,
    pub dyndbg_control: vpu_ipc_msg_payload_dyndbg_control,
    pub hws_priority_band_setup: vpu_ipc_msg_payload_hws_priority_band_setup,
    pub hws_create_cmdq: vpu_ipc_msg_payload_hws_create_cmdq,
    pub hws_create_cmdq_rsp: vpu_ipc_msg_payload_hws_create_cmdq_rsp,
    pub hws_destroy_cmdq: vpu_ipc_msg_payload_hws_destroy_cmdq,
    pub hws_set_scheduling_log: vpu_ipc_msg_payload_hws_set_scheduling_log,
    pub hws_scheduling_log_notification: vpu_ipc_msg_payload_hws_scheduling_log_notification,
    pub hws_suspend_cmdq: vpu_ipc_msg_payload_hws_suspend_cmdq,
    pub hws_resume_cmdq: vpu_ipc_msg_payload_hws_resume_cmdq,
    pub hws_resume_engine: vpu_ipc_msg_payload_hws_resume_engine,
    pub pwr_d0i3_enter: vpu_ipc_msg_payload_pwr_d0i3_enter,
    pub pwr_dct_control: vpu_ipc_msg_payload_pwr_dct_control,
    pub freq_config: vpu_ipc_msg_payload_freq_config,
}

//
// Host <-> NPU IPC message base structure.
//
// NOTE: All instances of this object must be aligned on a 64B boundary
// to allow proper handling of VPU cache operations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_jsm_msg {
// Reserved
    pub reserved_0: u64,
// Message type, see @ref vpu_ipc_msg_type.
    pub type: u32,
// Buffer status, see @ref vpu_ipc_msg_status.
    pub status: u32,
//
// Request ID, provided by the host in a request message and passed
// back by VPU in the response message.
//
    pub request_id: u32,
// Request return code set by the VPU, see VPU_JSM_STATUS_* defines.
    pub result: u32,
    pub reserved_1: u64,
// Message payload depending on message type, see vpu_ipc_msg_payload union.
    pub payload: vpu_ipc_msg_payload,
}

// @}
