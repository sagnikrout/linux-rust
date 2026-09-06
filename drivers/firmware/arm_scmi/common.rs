//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/firmware/arm_scmi/common.h
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
// System Control and Management Interface (SCMI) Message Protocol
// driver common header file containing some definitions, structures
// and function prototypes used in all the different SCMI protocols.
//
// Copyright (C) 2018-2024 ARM Ltd.
//

pub const SCMI_MAX_CHANNELS: c_int = 256;

pub const SCMI_SHMEM_MAX_PAYLOAD_SIZE: c_int = 104;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scmi_error_codes {
    SCMI_SUCCESS = 0,	/* Success */
    SCMI_ERR_SUPPORT = -1,	/* Not supported */
    SCMI_ERR_PARAMS = -2,	/* Invalid Parameters */
    SCMI_ERR_ACCESS = -3,	/* Invalid access/permission denied */
    SCMI_ERR_ENTRY = -4,	/* Not found */
    SCMI_ERR_RANGE = -5,	/* Value out of range */
    SCMI_ERR_BUSY = -6,	/* Device busy */
    SCMI_ERR_COMMS = -7,	/* Communication Error */
    SCMI_ERR_GENERIC = -8,	/* Generic Error */
    SCMI_ERR_HARDWARE = -9,	/* Hardware Error */
    SCMI_ERR_PROTOCOL = -10,/* Protocol Error */
}

// better than switch case as long as return value is continuous

pub const MSG_TYPE_COMMAND: c_int = 0;
pub const MSG_TYPE_DELAYED_RESP: c_int = 2;
pub const MSG_TYPE_NOTIFICATION: c_int = 3;

//
// Size of @pending_xfers hashtable included in @scmi_xfers_info; ideally, in
// order to minimize space and collisions, this should equal max_msg, i.e. the
// maximum number of in-flight messages on a specific platform, but such value
// is only available at runtime while kernel hashtables are statically sized:
// pick instead as a fixed static size the maximum number of entries that can
// fit the whole table into one 4k page.
//
pub const SCMI_PENDING_XFERS_HT_ORDER_SZ: c_int = 9;
//
// pack_scmi_header() - packs and returns 32-bit header
//
// @hdr: pointer to header containing all the information on message id,
// protocol id, sequence id and type.
//
// Return: 32-bit packed message header to be sent to the platform.
//
// unpack_scmi_header() - unpacks and records message and protocol id
//
// @msg_hdr: 32-bit packed message header sent from the platform
// @hdr: pointer to header to fetch message and protocol id.
//
// An helper macro to lookup an xfer from the @pending_xfers hashtable
// using the message sequence number token as a key.
//

pub const SCMI_BUS_NOTIFY_DEVICE_REQUEST: c_int = 0;
pub const SCMI_BUS_NOTIFY_DEVICE_UNREQUEST: c_int = 1;
extern "C" {
    pub fn scmi_device_destroy(parent: *mut device, protocol: c_int, name: *const c_char);
}
extern "C" {
    pub fn scmi_protocol_acquire(handle: *const scmi_handle, protocol_id: u8) -> c_int;
}
extern "C" {
    pub fn scmi_protocol_release(handle: *const scmi_handle, protocol_id: u8);
}
// SCMI Transport
//
// struct scmi_chan_info - Structure representing a SCMI channel information
//
// @id: An identifier for this channel: this matches the protocol number
// used to initialize this channel
// @dev: Reference to device in the SCMI hierarchy corresponding to this
// channel
// @is_p2a: A flag to identify a channel as P2A (RX)
// @rx_timeout_ms: The configured RX timeout in milliseconds.
// @max_msg_size: Maximum size of message payload.
// @handle: Pointer to SCMI entity handle
// @no_completion_irq: Flag to indicate that this channel has no completion
// interrupt mechanism for synchronous commands.
// This can be dynamically set by transports at run-time
// inside their provided .chan_setup().
// @transport_info: Transport layer related information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_chan_info {
    pub id: c_int,
    pub dev: *mut device,
    pub is_p2a: bool,
    pub rx_timeout_ms: c_uint,
    pub max_msg_size: c_uint,
    pub handle: *mut scmi_handle,
    pub no_completion_irq: bool,
    pub transport_info: *mut c_void,
}

//
// struct scmi_transport_ops - Structure representing a SCMI transport ops
//
// @chan_available: Callback to check if channel is available or not
// @chan_setup: Callback to allocate and setup a channel
// @chan_free: Callback to free a channel
// @get_max_msg: Optional callback to provide max_msg dynamically
// Returns the maximum number of messages for the channel type
// (tx or rx) that can be pending simultaneously in the system
// @send_message: Callback to send a message
// @mark_txdone: Callback to mark tx as done
// @fetch_response: Callback to fetch response
// @fetch_notification: Callback to fetch notification
// @clear_channel: Callback to clear a channel
// @poll_done: Callback to poll transfer status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_transport_ops {
    pub idx): *mut *mut *mut bool (chan_available)(struct device_node of_node, int,
    pub tx): bool,
    pub data): *mut *mut *mut int (chan_free)(int id, void p, void,
    pub base_cinfo): *mut *mut unsigned int (get_max_msg)(struct scmi_chan_info,
    pub xfer): *mut scmi_xfer,
    pub xfer): *mut scmi_xfer,
    pub xfer): *mut scmi_xfer,
    pub xfer): *mut size_t max_len, struct scmi_xfer,
    pub cinfo): *mut *mut void (clear_channel)(struct scmi_chan_info,
    pub xfer): *mut *mut *mut bool (poll_done)(struct scmi_chan_info cinfo, struct scmi_xfer,
}

//
// struct scmi_desc - Description of SoC integration
//
// @ops: Pointer to the transport specific ops structure
// @max_rx_timeout_ms: Timeout for communication with SoC (in Milliseconds)
// @max_msg: Maximum number of messages for a channel type (tx or rx) that can
// be pending simultaneously in the system. May be overridden by the
// get_max_msg op.
// @max_msg_size: Maximum size of data payload per message that can be handled.
// @atomic_threshold: Optional system wide DT-configured threshold, expressed
// in microseconds, for atomic operations.
// Only SCMI synchronous commands reported by the platform
// to have an execution latency lesser-equal to the threshold
// should be considered for atomic mode operation: such
// decision is finally left up to the SCMI drivers.
// @no_completion_irq: Flag to indicate that this transport has no completion
// interrupt and has to be polled. This is similar to the
// force_polling below, except this is set via DT property.
// @force_polling: Flag to force this whole transport to use SCMI core polling
// mechanism instead of completion interrupts even if available.
// @sync_cmds_completed_on_ret: Flag to indicate that the transport assures
// synchronous-command messages are atomically
// completed on .send_message: no need to poll
// actively waiting for a response.
// Used by core internally only when polling is
// selected as a waiting for reply method: i.e.
// if a completion irq was found use that anyway.
// @atomic_enabled: Flag to indicate that this transport, which is assured not
// to sleep anywhere on the TX path, can be used in atomic mode
// when requested.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_desc {
    pub ops: *const scmi_transport_ops,
    pub max_rx_timeout_ms: c_int,
    pub max_msg: c_int,
    pub max_msg_size: c_int,
    pub atomic_threshold: c_uint,
    pub no_completion_irq: bool,
    pub force_polling: bool,
    pub sync_cmds_completed_on_ret: bool,
    pub atomic_enabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum debug_counters {
    SENT_OK,
    SENT_FAIL,
    SENT_FAIL_POLLING_UNSUPPORTED,
    SENT_FAIL_CHANNEL_NOT_FOUND,
    RESPONSE_OK,
    NOTIFICATION_OK,
    DELAYED_RESPONSE_OK,
    XFERS_RESPONSE_TIMEOUT,
    XFERS_RESPONSE_POLLED_TIMEOUT,
    RESPONSE_POLLED_OK,
    ERR_MSG_UNEXPECTED,
    ERR_MSG_INVALID,
    ERR_MSG_NOMEM,
    ERR_PROTOCOL,
    XFERS_INFLIGHT,
    SCMI_DEBUG_COUNTERS_LAST
}

//
// struct scmi_debug_info  - Debug common info
// @top_dentry: A reference to the top debugfs dentry
// @name: Name of this SCMI instance
// @type: Type of this SCMI instance
// @is_atomic: Flag to state if the transport of this instance is atomic
// @counters: An array of atomic_c's used for tracking statistics (if enabled)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_debug_info {
    pub top_dentry: *mut dentry,
    pub name: *const c_char,
    pub type: *const c_char,
    pub is_atomic: bool,
    pub counters: [core::sync::atomic::AtomicI32; SCMI_DEBUG_COUNTERS_LAST],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scmi_bad_msg {
    MSG_UNEXPECTED = -1,
    MSG_INVALID = -2,
    MSG_UNKNOWN = -3,
    MSG_NOMEM = -4,
    MSG_MBOX_SPURIOUS = -5,
}

// Used for compactness and signature validation of the function pointers being
// passed.
//
// struct scmi_shmem_io_ops  - I/O operations to read from/write to
// Shared Memory
//
// @toio: Copy data to the shared memory area
// @fromio: Copy data from the shared memory area
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_shmem_io_ops {
    pub fromio: shmem_copy_fromio_t,
    pub toio: shmem_copy_toio_t,
}

// shmem related declarations
//
// struct scmi_shared_mem_operations  - Transport core operations for
// Shared Memory
//
// @tx_prepare: Prepare the @xfer message for transmission on the chosen @shmem
// @read_header: Read header of the message currently hold in @shmem
// @fetch_response: Copy the message response from @shmem into @xfer
// @fetch_notification: Copy the message notification from @shmem into @xfer
// @clear_channel: Clear the @shmem channel busy flag
// @poll_done: Check if poll has completed for @xfer on @shmem
// @channel_free: Check if @shmem channel is marked as free
// @channel_intr_enabled: Check is @shmem channel has requested a completion irq
// @setup_iomap: Setup IO shared memory for channel @cinfo
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_shared_mem_operations {
    pub toio): shmem_copy_toio_t,
    pub shmem): *mut *mut u32 (read_header)(struct scmi_shared_mem __iomem,
    pub fromio): shmem_copy_fromio_t,
    pub fromio): shmem_copy_fromio_t,
    pub shmem): *mut *mut void (clear_channel)(struct scmi_shared_mem __iomem,
    pub xfer): *mut scmi_xfer,
    pub shmem): *mut *mut bool (channel_free)(struct scmi_shared_mem __iomem,
    pub shmem): *mut *mut bool (channel_intr_enabled)(struct scmi_shared_mem __iomem,
    pub ops): *mut scmi_shmem_io_ops,
}

// declarations for message passing transports
// Maximum overhead of message w.r.t. struct scmi_desc.max_msg_size

//
// struct scmi_message_operations  - Transport core operations for Message
//
// @response_size: Get calculated response size for @xfer
// @command_size: Get calculated command size for @xfer
// @tx_prepare: Prepare the @xfer message for transmission on the provided @msg
// @read_header: Read header of the message currently hold in @msg
// @fetch_response: Copy the message response from @msg into @xfer
// @fetch_notification: Copy the message notification from @msg into @xfer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_message_operations {
    pub xfer): *mut *mut size_t (response_size)(struct scmi_xfer,
    pub xfer): *mut *mut size_t (command_size)(struct scmi_xfer,
    pub xfer): *mut *mut *mut void (tx_prepare)(struct scmi_msg_payld msg, struct scmi_xfer,
    pub msg): *mut *mut u32 (read_header)(struct scmi_msg_payld,
    pub xfer): *mut scmi_xfer,
    pub xfer): *mut size_t max_len, struct scmi_xfer,
}

//
// struct scmi_transport_core_operations  - Transpoert core operations
//
// @bad_message_trace: An helper to report a malformed/unexpected message
// @rx_callback: Callback to report received messages
// @shmem: Datagram operations for shared memory based transports
// @msg: Datagram operations for message based transports
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_transport_core_operations {
    pub err): u32 msg_hdr, enum scmi_bad_msg,
    pub priv): *mut c_void,
    pub shmem: *const scmi_shared_mem_operations,
    pub msg: *const scmi_message_operations,
}

//
// struct scmi_transport_handle  - Transport instance handle
// @supplier_get: A helper to retrieve the device descriptor, identifying the
// transport driver serving this SCMI instance, which will be
// used as a supplier for the core SCMI driver: returning an
// error here causes the probe sequence to be interrupted and
// return that same error code, so that each transport can decide
// which policy to implement by choosing an appropriate error.
// @supplier_put: A helper to signal that the specified transport supplier is
// no more being used and it is made available again.
//
// Note that these helpers are needed and provided only by those transports
// whose initialization relies on some other subsystem and whose relations to
// the core SCMI driver is not tracked by firmware descriptions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_transport_handle {
    pub th): *const (struct scmi_transport_handle,
    pub dev): *mut device,
}

//
// struct scmi_transport  - A structure representing a configured transport
//
// @supplier: Device representing the transport and acting as a supplier for
// the core SCMI stack
// @desc: Transport descriptor
// @core_ops: A pointer to a pointer used by the core SCMI stack to make the
// core transport operations accessible to the transports.
// @th: An optional pointer to the transport handle
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_transport {
    pub supplier: *mut device,
    pub desc: scmi_desc,
    pub core_ops: *mut scmi_transport_core_operations,
    pub th: *const scmi_transport_handle,
}

//
// struct scmi_transport_supplier  - Transport descriptor
// @mtx: A mutex to protect @available
// @available: A reference to an initialized transport device, when available.
// This reference is implicitly used to track the status of the
// supplier and it can cycle through the following 3 states:
// 1. NOT_READY - PTR_ERR(-EPROBE_DEFER): no supplier available;
// this is the transport initial state.
// 2. AVAILABLE - <supplier_dev>: a transport supplier has been
// initialized and it is available, ready to use.
// 3. BUSY _ PTR_ERR(-EBUSY): transport supplier is currently in use.
// @th: An embedded transport handle object that embeds the helpers
// implementing the above mentioned logic
//
// Note that this transport driver enforces single instance probing.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_transport_supplier {
// Protect @available
    pub mtx: mutex,
    pub available: *mut device,
    pub th: scmi_transport_handle,
}

//
// scmi_transport_supplier_put  - A helper to dispose of a supplier
// @th: A reference to the transport handle to use
// @supplier: A reference to the device supplier to manage, cannot be NULL
// or ERR_PTR.
//
// Note that putting a supplier will have different effect based on the
// current state of scmi_transport_supplier.available:
// - NOT_READY/BUSY: @supplier will be set as the new available device: this
// can be used to made available a supplier OR stop using one.
// - AVAILABLE: if the @supplier we are disposing of matches the currently
// available one, roll back to NOT_READY state.
// Any other attempt to override an available supplier with a
// new one is rejected, effectively enforcing one single supplier.
//
// Return: 0 on Success, errno otherwise.
//
// Nothing to do when the provided supplier was never real
// Putting a supplier when in the AVAILABLE state causes a
// transition back to the NOT_READY state, BUT only if the
// supplier we are disposing of was exactly the device that was
// previously made readily available.
//
// scmi_transport_supplier_get  - A helper to get hold of a supplier
// @th: A reference to the transport handle to use
//
// Note that, trying to get a supplier device can return:
// - a ready to use supplier device, (subsequently made unavailable)
// - PTR_ERR(-EPROBE_DEFER): no supplier is available
// - PTR_ERR(-BUSY): supplier was already taken by a previous get
//
// This allows the probe to defer and wait when a possible device can
// be reasonably expected to appear.
//
// Return: a usable supplier device on Success or PTR_ERR on Failure.
//

extern "C" {
    pub fn scmi_inflight_count(handle: *const scmi_handle) -> c_int;
}
