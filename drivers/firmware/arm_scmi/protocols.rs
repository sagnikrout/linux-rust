//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/firmware/arm_scmi/protocols.h
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
// protocols common header file containing some definitions, structures
// and function prototypes used in all the different SCMI protocols.
//
// Copyright (C) 2022 ARM Ltd.
//

pub const SCMI_PROTOCOL_VENDOR_BASE: c_uint = 0x80;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scmi_common_cmd {
    PROTOCOL_VERSION = 0x0,
    PROTOCOL_ATTRIBUTES = 0x1,
    PROTOCOL_MESSAGE_ATTRIBUTES = 0x2,
    NEGOTIATE_PROTOCOL_VERSION = 0x10,
}

//
// struct scmi_msg_resp_prot_version - Response for a message
//
// @minor_version: Minor version of the ABI that firmware supports
// @major_version: Major version of the ABI that firmware supports
//
// In general, ABI version changes follow the rule that minor version increments
// are backward compatible. Major revision changes in ABI may not be
// backward compatible.
//
// Response to a generic message with message type SCMI_MSG_VERSION
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_msg_resp_prot_version {
    pub minor_version: __le16,
    pub major_version: __le16,
}

//
// struct scmi_msg - Message(Tx/Rx) structure
//
// @buf: Buffer pointer
// @len: Length of data in the Buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_msg {
    pub buf: *mut c_void,
    pub len: usize,
}

//
// struct scmi_msg_hdr - Message(Tx/Rx) header
//
// @id: The identifier of the message being sent
// @protocol_id: The identifier of the protocol used to send @id message
// @type: The SCMI type for this message
// @seq: The token to identify the message. When a message returns, the
// platform returns the whole message header unmodified including the
// token
// @status: Status of the transfer once it's complete
// @poll_completion: Indicate if the transfer needs to be polled for
// completion or interrupt mode is used
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_msg_hdr {
    pub id: u8,
    pub protocol_id: u8,
    pub type: u8,
    pub seq: u16,
    pub status: u32,
    pub poll_completion: bool,
}

//
// struct scmi_xfer - Structure representing a message flow
//
// @transfer_id: Unique ID for debug & profiling purpose
// @hdr: Transmit message header
// @tx: Transmit message
// @rx: Receive message, the buffer should be pre-allocated to store
// message. If request-ACK protocol is used, we can reuse the same
// buffer for the rx path as we use for the tx path.
// @done: command message transmit completion event
// @async_done: pointer to delayed response message received event completion
// @pending: True for xfers added to @pending_xfers hashtable
// @node: An hlist_node reference used to store this xfer, alternatively, on
// the free list @free_xfers or in the @pending_xfers hashtable
// @users: A refcount to track the active users for this xfer.
// This is meant to protect against the possibility that, when a command
// transaction times out concurrently with the reception of a valid
// response message, the xfer could be finally put on the TX path, and
// so vanish, while on the RX path scmi_rx_callback() is still
// processing it: in such a case this refcounting will ensure that, even
// though the timed-out transaction will anyway cause the command
// request to be reported as failed by time-out, the underlying xfer
// cannot be discarded and possibly reused until the last one user on
// the RX path has released it.
// @busy: An atomic flag to ensure exclusive write access to this xfer
// @state: The current state of this transfer, with states transitions deemed
// valid being:
// - SCMI_XFER_SENT_OK -> SCMI_XFER_RESP_OK [ -> SCMI_XFER_DRESP_OK ]
// - SCMI_XFER_SENT_OK -> SCMI_XFER_DRESP_OK
// (Missing synchronous response is assumed OK and ignored)
// @flags: Optional flags associated to this xfer.
// @lock: A spinlock to protect state and busy fields.
// @priv: A pointer for transport private usage.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_xfer {
    pub transfer_id: c_int,
    pub hdr: scmi_msg_hdr,
    pub tx: scmi_msg,
    pub rx: scmi_msg,
    pub done: completion,
    pub async_done: *mut completion,
    pub pending: bool,
    pub node: hlist_node,
    pub users: refcount_t,
pub const SCMI_XFER_FREE: c_int = 0;
pub const SCMI_XFER_BUSY: c_int = 1;
    pub busy: core::sync::atomic::AtomicI32,
pub const SCMI_XFER_SENT_OK: c_int = 0;
pub const SCMI_XFER_RESP_OK: c_int = 1;
pub const SCMI_XFER_DRESP_OK: c_int = 2;
    pub state: c_int,

    pub flags: c_int,
// A lock to protect state and busy fields
    pub lock: spinlock_t,
    pub priv: *mut c_void,
}

//
// struct scmi_protocol_handle  - Reference to an initialized protocol instance
//
// @dev: A reference to the associated SCMI instance device (handle->dev).
// @version: The protocol version currently effectively in use by this
// initialized instance of the protocol as determined at the end of
// any possibly needed negotiations performed by the core.
// @xops: A reference to a struct holding refs to the core xfer operations that
// can be used by the protocol implementation to generate SCMI messages.
// @set_priv: A method to set protocol private data for this instance.
// @get_priv: A method to get protocol private data previously set.
//
// This structure represents a protocol initialized against specific SCMI
// instance and it will be used as follows:
// - as a parameter fed from the core to the protocol initialization code so
// that it can access the core xfer operations to build and generate SCMI
// messages exclusively for the specific underlying protocol instance.
// - as an opaque handle fed by an SCMI driver user when it tries to access
// this protocol through its own protocol operations.
// In this case this handle will be returned as an opaque object together
// with the related protocol operations when the SCMI driver tries to access
// the protocol.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_protocol_handle {
    pub dev: *mut device,
    pub version: c_uint,
    pub xops: *const scmi_xfer_ops,
    pub hops: *const scmi_proto_helpers_ops,
    pub priv): *const *const *const int (set_priv)(struct scmi_protocol_handle ph, void,
    pub ph): *const *const *const void (get_priv)(struct scmi_protocol_handle,
}

//
// struct scmi_iterator_state  - Iterator current state descriptor
// @desc_index: Starting index for the current multi-part request.
// @num_returned: Number of returned items in the last multi-part reply.
// @num_remaining: Number of remaining items in the multi-part message.
// @max_resources: Maximum acceptable number of items, configured by the caller
// depending on the underlying resources that it is querying.
// @loop_idx: The iterator loop index in the current multi-part reply.
// @rx_len: Size in bytes of the currently processed message; it can be used by
// the user of the iterator to verify a reply size.
// @priv: Optional pointer to some additional state-related private data setup
// by the caller during the iterations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_iterator_state {
    pub desc_index: c_uint,
    pub num_returned: c_uint,
    pub num_remaining: c_uint,
    pub max_resources: c_uint,
    pub loop_idx: c_uint,
    pub rx_len: usize,
    pub priv: *mut c_void,
}

//
// struct scmi_iterator_ops  - Custom iterator operations
// @prepare_message: An operation to provide the custom logic to fill in the
// SCMI command request pointed by @message. @desc_index is
// a reference to the next index to use in the multi-part
// request.
// @update_state: An operation to provide the custom logic to update the
// iterator state from the actual message response.
// @process_response: An operation to provide the custom logic needed to process
// each chunk of the multi-part message.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_iterator_ops {
    pub priv): *const c_void,
    pub priv): *const *const void response, void,
    pub priv): *mut *mut scmi_iterator_state st, void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_fc_db_info {
    pub width: c_int,
    pub set: u64,
    pub mask: u64,
    pub addr: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_fc_info {
    pub set_addr: *mut void __iomem,
    pub get_addr: *mut void __iomem,
    pub set_db: *mut scmi_fc_db_info,
    pub rate_limit: u32,
}

//
// struct scmi_proto_helpers_ops  - References to common protocol helpers
// @extended_name_get: A common helper function to retrieve extended naming
// for the specified resource using the specified command.
// Result is returned as a NULL terminated string in the
// pre-allocated area pointed to by @name with maximum
// capacity of @len bytes.
// @iter_response_init: A common helper to initialize a generic iterator to
// parse multi-message responses: when run the iterator
// will take care to send the initial command request as
// specified by @msg_id and @tx_size and then to parse the
// multi-part responses using the custom operations
// provided in @ops.
// @iter_response_run: A common helper to trigger the run of a previously
// initialized iterator. Note that unbound iterators are
// automatically cleaned up.
// @iter_response_run_bound: A common helper to trigger the run of a previously
// initialized iterator, but only within the
// specified, optional, @start and @end resource
// indexes. Note that these bound-iterators need
// explicit cleanup via @iter_response_bound_cleanup.
// @iter_response_bound_cleanup: A common helper to finally release the iterator
// for bound iterators.
// @protocol_msg_check: A common helper to check is a specific protocol message
// is supported.
// @fastchannel_init: A common helper used to initialize FC descriptors by
// gathering FC descriptions from the SCMI platform server.
// @fastchannel_db_ring: A common helper to ring a FC doorbell.
// @get_max_msg_size: A common helper to get the maximum message size.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_proto_helpers_ops {
    pub len): usize,
    pub priv): *mut size_t tx_size, void,
    pub iter): *mut *mut int (iter_response_run)(void,
    pub end): *mut *mut unsigned int start, unsigned int,
    pub iter): *mut *mut void (iter_response_bound_cleanup)(void,
    pub attributes): *mut u32 message_id, u32,
    pub rate_limit): *mut u32,
    pub db): *mut *mut void (fastchannel_db_ring)(struct scmi_fc_db_info,
    pub ph): *const *const int (get_max_msg_size)(struct scmi_protocol_handle,
}

//
// struct scmi_xfer_ops  - References to the core SCMI xfer operations.
// @xfer_get_init: Initialize one struct xfer if any xfer slot is free.
// @reset_rx_to_maxsz: Reset rx size to max transport size.
// @do_xfer: Do the SCMI transfer.
// @do_xfer_with_response: Do the SCMI transfer waiting for a response.
// @xfer_put: Free the xfer slot.
//
// Note that all this operations expect a protocol handle as first parameter;
// they then internally use it to infer the underlying protocol number: this
// way is not possible for a protocol implementation to forge messages for
// another protocol.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_xfer_ops {
    pub p): *mut scmi_xfer,
    pub xfer): *mut scmi_xfer,
    pub xfer): *mut scmi_xfer,
    pub xfer): *mut scmi_xfer,
    pub xfer): *mut scmi_xfer,
}

extern "C" {
    pub fn int(: *const *const scmi_prot_init_ph_fn_t)(struct scmi_protocol_handle) -> typedef;
}
//
// struct scmi_protocol  - Protocol descriptor
// @id: Protocol ID.
// @owner: Module reference if any.
// @instance_init: Mandatory protocol initialization function.
// @instance_deinit: Optional protocol de-initialization function.
// @ops: Optional reference to the operations provided by the protocol and
// exposed in scmi_protocol.h.
// @events: An optional reference to the events supported by this protocol.
// @supported_version: The highest version currently supported for this
// protocol by the agent. Each protocol implementation
// in the agent is supposed to downgrade to match the
// protocol version supported by the platform.
// @vendor_id: A firmware vendor string for vendor protocols matching.
// Ignored when @id identifies a standard protocol, cannot be NULL
// otherwise.
// @sub_vendor_id: A firmware sub_vendor string for vendor protocols matching.
// Ignored if NULL or when @id identifies a standard protocol.
// @impl_ver: A firmware implementation version for vendor protocols matching.
// Ignored if zero or if @id identifies a standard protocol.
//
// Note that vendor protocols matching at load time is performed by attempting
// the closest match first against the tuple (vendor, sub_vendor, impl_ver)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_protocol {
    pub id: u8,
    pub owner: *mut module,
    pub instance_init: scmi_prot_init_ph_fn_t,
    pub instance_deinit: scmi_prot_init_ph_fn_t,
    pub ops: *const c_void,
    pub events: *const scmi_protocol_events,
    pub supported_version: c_uint,
    pub vendor_id: *mut c_char,
    pub sub_vendor_id: *mut c_char,
    pub impl_ver: u32,
}

