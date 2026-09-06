//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/interface/io/netif.h
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
// xen_netif.h
//
// Unified network-device I/O interface for Xen guest OSes.
//
// Copyright (c) 2003-2004, Keir Fraser
//

//
// Older implementation of Xen network frontend / backend has an
// implicit dependency on the MAX_SKB_FRAGS as the maximum number of
// ring slots a skb can use. Netfront / netback may not work as
// expected when frontend and backend have different MAX_SKB_FRAGS.
//
// A better approach is to add mechanism for netfront / netback to
// negotiate this value. However we cannot fix all possible
// frontends, so we need to define a value which states the minimum
// slots backend must support.
//
// The minimum value derives from older Linux kernel's MAX_SKB_FRAGS
// (18), which is proved to work with most frontends. Any new backend
// which doesn't negotiate with frontend should expect frontend to
// send a valid packet using slots up to this value.
//
pub const XEN_NETIF_NR_SLOTS_MIN: c_int = 18;
//
// Notifications after enqueuing any type of message should be conditional on
// the appropriate req_event or rsp_event field in the shared ring.
// If the client sends notification for rx requests then it should specify
// feature 'feature-rx-notify' via xenbus. Otherwise the backend will assume
// that it cannot safely queue packets (as it may not be kicked to send them).
//
// "feature-split-event-channels" is introduced to separate guest TX
// and RX notification. Backend either doesn't support this feature or
// advertises it via xenstore as 0 (disabled) or 1 (enabled).
//
// To make use of this feature, frontend should allocate two event
// channels for TX and RX, advertise them to backend as
// "event-channel-tx" and "event-channel-rx" respectively. If frontend
// doesn't want to use this feature, it just writes "event-channel"
// node as before.
//
// Multiple transmit and receive queues:
// If supported, the backend will write the key "multi-queue-max-queues" to
// the directory for that vif, and set its value to the maximum supported
// number of queues.
// Frontends that are aware of this feature and wish to use it can write the
// key "multi-queue-num-queues", set to the number they wish to use, which
// must be greater than zero, and no more than the value reported by the backend
// in "multi-queue-max-queues".
//
// Queues replicate the shared rings and event channels.
// "feature-split-event-channels" may optionally be used when using
// multiple queues, but is not mandatory.
//
// Each queue consists of one shared ring pair, i.e. there must be the same
// number of tx and rx rings.
//
// For frontends requesting just one queue, the usual event-channel and
// ring-ref keys are written as before, simplifying the backend processing
// to avoid distinguishing between a frontend that doesn't understand the
// multi-queue feature, and one that does, but requested only one queue.
//
// Frontends requesting two or more queues must not write the toplevel
// event-channel (or event-channel-{tx,rx}) and {tx,rx}-ring-ref keys,
// instead writing those keys under sub-keys having the name "queue-N" where
// N is the integer ID of the queue for which those keys belong. Queues
// are indexed from zero. For example, a frontend with two queues and split
// event channels must write the following set of queue-related keys:
//
// /local/domain/1/device/vif/0/multi-queue-num-queues = "2"
// /local/domain/1/device/vif/0/queue-0 = ""
// /local/domain/1/device/vif/0/queue-0/tx-ring-ref = "<ring-ref-tx0>"
// /local/domain/1/device/vif/0/queue-0/rx-ring-ref = "<ring-ref-rx0>"
// /local/domain/1/device/vif/0/queue-0/event-channel-tx = "<evtchn-tx0>"
// /local/domain/1/device/vif/0/queue-0/event-channel-rx = "<evtchn-rx0>"
// /local/domain/1/device/vif/0/queue-1 = ""
// /local/domain/1/device/vif/0/queue-1/tx-ring-ref = "<ring-ref-tx1>"
// /local/domain/1/device/vif/0/queue-1/rx-ring-ref = "<ring-ref-rx1"
// /local/domain/1/device/vif/0/queue-1/event-channel-tx = "<evtchn-tx1>"
// /local/domain/1/device/vif/0/queue-1/event-channel-rx = "<evtchn-rx1>"
//
// If there is any inconsistency in the XenStore data, the backend may
// choose not to connect any queues, instead treating the request as an
// error. This includes scenarios where more (or fewer) queues were
// requested than the frontend provided details for.
//
// Mapping of packets to queues is considered to be a function of the
// transmitting system (backend or frontend) and is not negotiated
// between the two. Guests are free to transmit packets on any queue
// they choose, provided it has been set up correctly. Guests must be
// prepared to receive packets on any queue they have requested be set up.
//
// "feature-no-csum-offload" should be used to turn IPv4 TCP/UDP checksum
// offload off or on. If it is missing then the feature is assumed to be on.
// "feature-ipv6-csum-offload" should be used to turn IPv6 TCP/UDP checksum
// offload on or off. If it is missing then the feature is assumed to be off.
//
// "feature-gso-tcpv4" and "feature-gso-tcpv6" advertise the capability to
// handle large TCP packets (in IPv4 or IPv6 form respectively). Neither
// frontends nor backends are assumed to be capable unless the flags are
// present.
//
// "feature-multicast-control" and "feature-dynamic-multicast-control"
// advertise the capability to filter ethernet multicast packets in the
// backend. If the frontend wishes to take advantage of this feature then
// it may set "request-multicast-control". If the backend only advertises
// "feature-multicast-control" then "request-multicast-control" must be set
// before the frontend moves into the connected state. The backend will
// sample the value on this state transition and any subsequent change in
// value will have no effect. However, if the backend also advertises
// "feature-dynamic-multicast-control" then "request-multicast-control"
// may be set by the frontend at any time. In this case, the backend will
// watch the value and re-sample on watch events.
//
// If the sampled value of "request-multicast-control" is set then the
// backend transmit side should no longer flood multicast packets to the
// frontend, it should instead drop any multicast packet that does not
// match in a filter list.
// The list is amended by the frontend by sending dummy transmit requests
// containing XEN_NETIF_EXTRA_TYPE_MCAST_{ADD,DEL} extra-info fragments as
// specified below.
// Note that the filter list may be amended even if the sampled value of
// "request-multicast-control" is not set, however the filter should only
// be applied if it is set.
//
// "xdp-headroom" is used to request that extra space is added
// for XDP processing.  The value is measured in bytes and passed by
// the frontend to be consistent between both ends.
// If the value is greater than zero that means that
// an RX response is going to be passed to an XDP program for processing.
// XEN_NETIF_MAX_XDP_HEADROOM defines the maximum headroom offset in bytes
//
// "feature-xdp-headroom" is set to "1" by the netback side like other features
// so a guest can check if an XDP program can be processed.
//
pub const XEN_NETIF_MAX_XDP_HEADROOM: c_uint = 0x7FFF;
//
// Control ring
// ============
//
// Some features, such as hashing (detailed below), require a
// significant amount of out-of-band data to be passed from frontend to
// backend. Use of xenstore is not suitable for large quantities of data
// because of quota limitations and so a dedicated 'control ring' is used.
// The ability of the backend to use a control ring is advertised by
// setting:
//
// /local/domain/X/backend/<domid>/<vif>/feature-ctrl-ring = "1"
//
// The frontend provides a control ring to the backend by setting:
//
// /local/domain/<domid>/device/vif/<vif>/ctrl-ring-ref = <gref>
// /local/domain/<domid>/device/vif/<vif>/event-channel-ctrl = <port>
//
// where <gref> is the grant reference of the shared page used to
// implement the control ring and <port> is an event channel to be used
// as a mailbox interrupt. These keys must be set before the frontend
// moves into the connected state.
//
// The control ring uses a fixed request/response message size and is
// balanced (i.e. one request to one response), so operationally it is much
// the same as a transmit or receive ring.
// Note that there is no requirement that responses are issued in the same
// order as requests.
//
// Hash types
// ==========
//
// For the purposes of the definitions below, 'Packet[]' is an array of
// octets containing an IP packet without options, 'Array[X..Y]' means a
// sub-array of 'Array' containing bytes X thru Y inclusive, and '+' is
// used to indicate concatenation of arrays.
//
// A hash calculated over an IP version 4 header as follows:
//
// Buffer[0..8] = Packet[12..15] (source address) +
// Packet[16..19] (destination address)
//
// Result = Hash(Buffer, 8)
//
pub const _XEN_NETIF_CTRL_HASH_TYPE_IPV4: c_int = 0;

//
// A hash calculated over an IP version 4 header and TCP header as
// follows:
//
// Buffer[0..12] = Packet[12..15] (source address) +
// Packet[16..19] (destination address) +
// Packet[20..21] (source port) +
// Packet[22..23] (destination port)
//
// Result = Hash(Buffer, 12)
//
pub const _XEN_NETIF_CTRL_HASH_TYPE_IPV4_TCP: c_int = 1;

//
// A hash calculated over an IP version 6 header as follows:
//
// Buffer[0..32] = Packet[8..23]  (source address ) +
// Packet[24..39] (destination address)
//
// Result = Hash(Buffer, 32)
//
pub const _XEN_NETIF_CTRL_HASH_TYPE_IPV6: c_int = 2;

//
// A hash calculated over an IP version 6 header and TCP header as
// follows:
//
// Buffer[0..36] = Packet[8..23]  (source address) +
// Packet[24..39] (destination address) +
// Packet[40..41] (source port) +
// Packet[42..43] (destination port)
//
// Result = Hash(Buffer, 36)
//
pub const _XEN_NETIF_CTRL_HASH_TYPE_IPV6_TCP: c_int = 3;

//
// Hash algorithms
// ===============
//
pub const XEN_NETIF_CTRL_HASH_ALGORITHM_NONE: c_int = 0;
//
// Toeplitz hash:
//
pub const XEN_NETIF_CTRL_HASH_ALGORITHM_TOEPLITZ: c_int = 1;
//
// This algorithm uses a 'key' as well as the data buffer itself.
// (Buffer[] and Key[] are treated as shift-registers where the MSB of
// Buffer/Key[0] is considered 'left-most' and the LSB of Buffer/Key[N-1]
// is the 'right-most').
//
// Value = 0
// For number of bits in Buffer[]
// If (left-most bit of Buffer[] is 1)
// Value ^= left-most 32 bits of Key[]
// Key[] << 1
// Buffer[] << 1
//
// The code below is provided for convenience where an operating system
// does not already provide an implementation.
//

// Pre-load prefix with the first 8 bytes of the key
//
// 'prefix' has now been left-shifted by 8, so
// OR in the next byte.
//
// The valid part of the hash is in the upper 32 bits.

//
// Control requests (struct xen_netif_ctrl_request)
// ================================================
//
// All requests have the following format:
//
// 0     1     2     3     4     5     6     7  octet
// +-----+-----+-----+-----+-----+-----+-----+-----+
// |    id     |   type    |         data[0]       |
// +-----+-----+-----+-----+-----+-----+-----+-----+
// |         data[1]       |         data[2]       |
// +-----+-----+-----+-----+-----------------------+
//
// id: the request identifier, echoed in response.
// type: the type of request (see below)
// data[]: any data associated with the request (determined by type)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_netif_ctrl_request {
    pub id: u16,
    pub type: u16,
pub const XEN_NETIF_CTRL_TYPE_INVALID: c_int = 0;
pub const XEN_NETIF_CTRL_TYPE_GET_HASH_FLAGS: c_int = 1;
pub const XEN_NETIF_CTRL_TYPE_SET_HASH_FLAGS: c_int = 2;
pub const XEN_NETIF_CTRL_TYPE_SET_HASH_KEY: c_int = 3;
pub const XEN_NETIF_CTRL_TYPE_GET_HASH_MAPPING_SIZE: c_int = 4;
pub const XEN_NETIF_CTRL_TYPE_SET_HASH_MAPPING_SIZE: c_int = 5;
pub const XEN_NETIF_CTRL_TYPE_SET_HASH_MAPPING: c_int = 6;
pub const XEN_NETIF_CTRL_TYPE_SET_HASH_ALGORITHM: c_int = 7;
    pub data: [u32; 3],
}

//
// Control responses (struct xen_netif_ctrl_response)
// ==================================================
//
// All responses have the following format:
//
// 0     1     2     3     4     5     6     7  octet
// +-----+-----+-----+-----+-----+-----+-----+-----+
// |    id     |   type    |         status        |
// +-----+-----+-----+-----+-----+-----+-----+-----+
// |         data          |
// +-----+-----+-----+-----+
//
// id: the corresponding request identifier
// type: the type of the corresponding request
// status: the status of request processing
// data: any data associated with the response (determined by type and
// status)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_netif_ctrl_response {
    pub id: u16,
    pub type: u16,
    pub status: u32,
pub const XEN_NETIF_CTRL_STATUS_SUCCESS: c_int = 0;
pub const XEN_NETIF_CTRL_STATUS_NOT_SUPPORTED: c_int = 1;
pub const XEN_NETIF_CTRL_STATUS_INVALID_PARAMETER: c_int = 2;
pub const XEN_NETIF_CTRL_STATUS_BUFFER_OVERFLOW: c_int = 3;
    pub data: u32,
}

//
// Control messages
// ================
//
// XEN_NETIF_CTRL_TYPE_SET_HASH_ALGORITHM
// --------------------------------------
//
// This is sent by the frontend to set the desired hash algorithm.
//
// Request:
//
// type    = XEN_NETIF_CTRL_TYPE_SET_HASH_ALGORITHM
// data[0] = a XEN_NETIF_CTRL_HASH_ALGORITHM_* value
// data[1] = 0
// data[2] = 0
//
// Response:
//
// status = XEN_NETIF_CTRL_STATUS_NOT_SUPPORTED     - Operation not
// supported
// XEN_NETIF_CTRL_STATUS_INVALID_PARAMETER - The algorithm is not
// supported
// XEN_NETIF_CTRL_STATUS_SUCCESS           - Operation successful
//
// NOTE: Setting data[0] to XEN_NETIF_CTRL_HASH_ALGORITHM_NONE disables
// hashing and the backend is free to choose how it steers packets
// to queues (which is the default behaviour).
//
// XEN_NETIF_CTRL_TYPE_GET_HASH_FLAGS
// ----------------------------------
//
// This is sent by the frontend to query the types of hash supported by
// the backend.
//
// Request:
//
// type    = XEN_NETIF_CTRL_TYPE_GET_HASH_FLAGS
// data[0] = 0
// data[1] = 0
// data[2] = 0
//
// Response:
//
// status = XEN_NETIF_CTRL_STATUS_NOT_SUPPORTED - Operation not supported
// XEN_NETIF_CTRL_STATUS_SUCCESS       - Operation successful
// data   = supported hash types (if operation was successful)
//
// NOTE: A valid hash algorithm must be selected before this operation can
// succeed.
//
// XEN_NETIF_CTRL_TYPE_SET_HASH_FLAGS
// ----------------------------------
//
// This is sent by the frontend to set the types of hash that the backend
// should calculate. (See above for hash type definitions).
// Note that the 'maximal' type of hash should always be chosen. For
// example, if the frontend sets both IPV4 and IPV4_TCP hash types then
// the latter hash type should be calculated for any TCP packet and the
// former only calculated for non-TCP packets.
//
// Request:
//
// type    = XEN_NETIF_CTRL_TYPE_SET_HASH_FLAGS
// data[0] = bitwise OR of XEN_NETIF_CTRL_HASH_TYPE_* values
// data[1] = 0
// data[2] = 0
//
// Response:
//
// status = XEN_NETIF_CTRL_STATUS_NOT_SUPPORTED     - Operation not
// supported
// XEN_NETIF_CTRL_STATUS_INVALID_PARAMETER - One or more flag
// value is invalid or
// unsupported
// XEN_NETIF_CTRL_STATUS_SUCCESS           - Operation successful
// data   = 0
//
// NOTE: A valid hash algorithm must be selected before this operation can
// succeed.
// Also, setting data[0] to zero disables hashing and the backend
// is free to choose how it steers packets to queues.
//
// XEN_NETIF_CTRL_TYPE_SET_HASH_KEY
// --------------------------------
//
// This is sent by the frontend to set the key of the hash if the algorithm
// requires it. (See hash algorithms above).
//
// Request:
//
// type    = XEN_NETIF_CTRL_TYPE_SET_HASH_KEY
// data[0] = grant reference of page containing the key (assumed to
// start at beginning of grant)
// data[1] = size of key in octets
// data[2] = 0
//
// Response:
//
// status = XEN_NETIF_CTRL_STATUS_NOT_SUPPORTED     - Operation not
// supported
// XEN_NETIF_CTRL_STATUS_INVALID_PARAMETER - Key size is invalid
// XEN_NETIF_CTRL_STATUS_BUFFER_OVERFLOW   - Key size is larger
// than the backend
// supports
// XEN_NETIF_CTRL_STATUS_SUCCESS           - Operation successful
// data   = 0
//
// NOTE: Any key octets not specified are assumed to be zero (the key
// is assumed to be empty by default) and specifying a new key
// invalidates any previous key, hence specifying a key size of
// zero will clear the key (which ensures that the calculated hash
// will always be zero).
// The maximum size of key is algorithm and backend specific, but
// is also limited by the single grant reference.
// The grant reference may be read-only and must remain valid until
// the response has been processed.
//
// XEN_NETIF_CTRL_TYPE_GET_HASH_MAPPING_SIZE
// -----------------------------------------
//
// This is sent by the frontend to query the maximum size of mapping
// table supported by the backend. The size is specified in terms of
// table entries.
//
// Request:
//
// type    = XEN_NETIF_CTRL_TYPE_GET_HASH_MAPPING_SIZE
// data[0] = 0
// data[1] = 0
// data[2] = 0
//
// Response:
//
// status = XEN_NETIF_CTRL_STATUS_NOT_SUPPORTED - Operation not supported
// XEN_NETIF_CTRL_STATUS_SUCCESS       - Operation successful
// data   = maximum number of entries allowed in the mapping table
// (if operation was successful) or zero if a mapping table is
// not supported (i.e. hash mapping is done only by modular
// arithmetic).
//
// XEN_NETIF_CTRL_TYPE_SET_HASH_MAPPING_SIZE
// -------------------------------------
//
// This is sent by the frontend to set the actual size of the mapping
// table to be used by the backend. The size is specified in terms of
// table entries.
// Any previous table is invalidated by this message and any new table
// is assumed to be zero filled.
//
// Request:
//
// type    = XEN_NETIF_CTRL_TYPE_SET_HASH_MAPPING_SIZE
// data[0] = number of entries in mapping table
// data[1] = 0
// data[2] = 0
//
// Response:
//
// status = XEN_NETIF_CTRL_STATUS_NOT_SUPPORTED     - Operation not
// supported
// XEN_NETIF_CTRL_STATUS_INVALID_PARAMETER - Table size is invalid
// XEN_NETIF_CTRL_STATUS_SUCCESS           - Operation successful
// data   = 0
//
// NOTE: Setting data[0] to 0 means that hash mapping should be done
// using modular arithmetic.
//
// XEN_NETIF_CTRL_TYPE_SET_HASH_MAPPING
// ------------------------------------
//
// This is sent by the frontend to set the content of the table mapping
// hash value to queue number. The backend should calculate the hash from
// the packet header, use it as an index into the table (modulo the size
// of the table) and then steer the packet to the queue number found at
// that index.
//
// Request:
//
// type    = XEN_NETIF_CTRL_TYPE_SET_HASH_MAPPING
// data[0] = grant reference of page containing the mapping (sub-)table
// (assumed to start at beginning of grant)
// data[1] = size of (sub-)table in entries
// data[2] = offset, in entries, of sub-table within overall table
//
// Response:
//
// status = XEN_NETIF_CTRL_STATUS_NOT_SUPPORTED     - Operation not
// supported
// XEN_NETIF_CTRL_STATUS_INVALID_PARAMETER - Table size or content
// is invalid
// XEN_NETIF_CTRL_STATUS_BUFFER_OVERFLOW   - Table size is larger
// than the backend
// supports
// XEN_NETIF_CTRL_STATUS_SUCCESS           - Operation successful
// data   = 0
//
// NOTE: The overall table has the following format:
//
// 0     1     2     3     4     5     6     7  octet
// +-----+-----+-----+-----+-----+-----+-----+-----+
// |       mapping[0]      |       mapping[1]      |
// +-----+-----+-----+-----+-----+-----+-----+-----+
// |                       .                       |
// +-----+-----+-----+-----+-----+-----+-----+-----+
// |      mapping[N-2]     |      mapping[N-1]     |
// +-----+-----+-----+-----+-----+-----+-----+-----+
//
// where N is specified by a XEN_NETIF_CTRL_TYPE_SET_HASH_MAPPING_SIZE
// message and each  mapping must specifies a queue between 0 and
// "multi-queue-num-queues" (see above).
// The backend may support a mapping table larger than can be
// mapped by a single grant reference. Thus sub-tables within a
// larger table can be individually set by sending multiple messages
// with differing offset values. Specifying a new sub-table does not
// invalidate any table data outside that range.
// The grant reference may be read-only and must remain valid until
// the response has been processed.
//
// Guest transmit
// ==============
//
// This is the 'wire' format for transmit (frontend -> backend) packets:
//
// Fragment 1: xen_netif_tx_request_t  - flags = XEN_NETTXF_
// size = total packet size
// [Extra 1: xen_netif_extra_info_t]    - (only if fragment 1 flags include
// XEN_NETTXF_extra_info)
// ...
// [Extra N: xen_netif_extra_info_t]    - (only if extra N-1 flags include
// XEN_NETIF_EXTRA_MORE)
// ...
// Fragment N: xen_netif_tx_request_t  - (only if fragment N-1 flags include
// XEN_NETTXF_more_data - flags on preceding
// extras are not relevant here)
// flags = 0
// size = fragment size
//
// NOTE:
//
// This format slightly is different from that used for receive
// (backend -> frontend) packets. Specifically, in a multi-fragment
// packet the actual size of fragment 1 can only be determined by
// subtracting the sizes of fragments 2..N from the total packet size.
//
// Ring slot size is 12 octets, however not all request/response
// structs use the full size.
//
// tx request data (xen_netif_tx_request_t)
// ------------------------------------
//
// 0     1     2     3     4     5     6     7  octet
// +-----+-----+-----+-----+-----+-----+-----+-----+
// | grant ref             | offset    | flags     |
// +-----+-----+-----+-----+-----+-----+-----+-----+
// | id        | size      |
// +-----+-----+-----+-----+
//
// grant ref: Reference to buffer page.
// offset: Offset within buffer page.
// flags: XEN_NETTXF_*.
// id: request identifier, echoed in response.
// size: packet size in bytes.
//
// tx response (xen_netif_tx_response_t)
// ---------------------------------
//
// 0     1     2     3     4     5     6     7  octet
// +-----+-----+-----+-----+-----+-----+-----+-----+
// | id        | status    | unused                |
// +-----+-----+-----+-----+-----+-----+-----+-----+
// | unused                |
// +-----+-----+-----+-----+
//
// id: reflects id in transmit request
// status: XEN_NETIF_RSP_
//
// Guest receive
// =============
//
// This is the 'wire' format for receive (backend -> frontend) packets:
//
// Fragment 1: xen_netif_rx_request_t  - flags = XEN_NETRXF_
// size = fragment size
// [Extra 1: xen_netif_extra_info_t]    - (only if fragment 1 flags include
// XEN_NETRXF_extra_info)
// ...
// [Extra N: xen_netif_extra_info_t]    - (only if extra N-1 flags include
// XEN_NETIF_EXTRA_MORE)
// ...
// Fragment N: xen_netif_rx_request_t  - (only if fragment N-1 flags include
// XEN_NETRXF_more_data - flags on preceding
// extras are not relevant here)
// flags = 0
// size = fragment size
//
// NOTE:
//
// This format slightly is different from that used for transmit
// (frontend -> backend) packets. Specifically, in a multi-fragment
// packet the size of the packet can only be determined by summing the
// sizes of fragments 1..N.
//
// Ring slot size is 8 octets.
//
// rx request (xen_netif_rx_request_t)
// -------------------------------
//
// 0     1     2     3     4     5     6     7  octet
// +-----+-----+-----+-----+-----+-----+-----+-----+
// | id        | pad       | gref                  |
// +-----+-----+-----+-----+-----+-----+-----+-----+
//
// id: request identifier, echoed in response.
// gref: reference to incoming granted frame.
//
// rx response (xen_netif_rx_response_t)
// ---------------------------------
//
// 0     1     2     3     4     5     6     7  octet
// +-----+-----+-----+-----+-----+-----+-----+-----+
// | id        | offset    | flags     | status    |
// +-----+-----+-----+-----+-----+-----+-----+-----+
//
// id: reflects id in receive request
// offset: offset in page of start of received packet
// flags: XEN_NETRXF_
// status: -ve: XEN_NETIF_RSP_*; +ve: Rx'ed pkt size.
//
// NOTE: Historically, to support GSO on the frontend receive side, Linux
// netfront does not make use of the rx response id (because, as
// described below, extra info structures overlay the id field).
// Instead it assumes that responses always appear in the same ring
// slot as their corresponding request. Thus, to maintain
// compatibility, backends must make sure this is the case.
//
// Extra Info
// ==========
//
// Can be present if initial request or response has NET{T,R}XF_extra_info,
// or previous extra request has XEN_NETIF_EXTRA_MORE.
//
// The struct therefore needs to fit into either a tx or rx slot and
// is therefore limited to 8 octets.
//
// NOTE: Because extra info data overlays the usual request/response
// structures, there is no id information in the opposite direction.
// So, if an extra info overlays an rx response the frontend can
// assume that it is in the same ring slot as the request that was
// consumed to make the slot available, and the backend must ensure
// this assumption is true.
//
// extra info (xen_netif_extra_info_t)
// -------------------------------
//
// General format:
//
// 0     1     2     3     4     5     6     7  octet
// +-----+-----+-----+-----+-----+-----+-----+-----+
// |type |flags| type specific data                |
// +-----+-----+-----+-----+-----+-----+-----+-----+
// | padding for tx        |
// +-----+-----+-----+-----+
//
// type: XEN_NETIF_EXTRA_TYPE_
// flags: XEN_NETIF_EXTRA_FLAG_
// padding for tx: present only in the tx case due to 8 octet limit
// from rx case. Not shown in type specific entries
// below.
//
// XEN_NETIF_EXTRA_TYPE_GSO:
//
// 0     1     2     3     4     5     6     7  octet
// +-----+-----+-----+-----+-----+-----+-----+-----+
// |type |flags| size      |type | pad | features  |
// +-----+-----+-----+-----+-----+-----+-----+-----+
//
// type: Must be XEN_NETIF_EXTRA_TYPE_GSO
// flags: XEN_NETIF_EXTRA_FLAG_
// size: Maximum payload size of each segment. For example,
// for TCP this is just the path MSS.
// type: XEN_NETIF_GSO_TYPE_*: This determines the protocol of
// the packet and any extra features required to segment the
// packet properly.
// features: EN_XEN_NETIF_GSO_FEAT_*: This specifies any extra GSO
// features required to process this packet, such as ECN
// support for TCPv4.
//
// XEN_NETIF_EXTRA_TYPE_MCAST_{ADD,DEL}:
//
// 0     1     2     3     4     5     6     7  octet
// +-----+-----+-----+-----+-----+-----+-----+-----+
// |type |flags| addr                              |
// +-----+-----+-----+-----+-----+-----+-----+-----+
//
// type: Must be XEN_NETIF_EXTRA_TYPE_MCAST_{ADD,DEL}
// flags: XEN_NETIF_EXTRA_FLAG_
// addr: address to add/remove
//
// XEN_NETIF_EXTRA_TYPE_HASH:
//
// A backend that supports teoplitz hashing is assumed to accept
// this type of extra info in transmit packets.
// A frontend that enables hashing is assumed to accept
// this type of extra info in receive packets.
//
// 0     1     2     3     4     5     6     7  octet
// +-----+-----+-----+-----+-----+-----+-----+-----+
// |type |flags|htype| alg |LSB ---- value ---- MSB|
// +-----+-----+-----+-----+-----+-----+-----+-----+
//
// type: Must be XEN_NETIF_EXTRA_TYPE_HASH
// flags: XEN_NETIF_EXTRA_FLAG_
// htype: Hash type (one of _XEN_NETIF_CTRL_HASH_TYPE_* - see above)
// alg: The algorithm used to calculate the hash (one of
// XEN_NETIF_CTRL_HASH_TYPE_ALGORITHM_* - see above)
// value: Hash value
//
// Protocol checksum field is blank in the packet (hardware offload)?

// Packet data has been validated against protocol checksum.

// Packet continues in the next request descriptor.

// Packet to be followed by extra descriptor(s).

pub const XEN_NETIF_MAX_TX_SIZE: c_uint = 0xFFFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_netif_tx_request {
    pub gref: grant_ref_t,
    pub offset: u16,
    pub flags: u16,
    pub id: u16,
    pub size: u16,
}

// Types of xen_netif_extra_info descriptors.

// xen_netif_extra_info_t flags.

// GSO types

//
// This structure needs to fit within both xen_netif_tx_request_t and
// xen_netif_rx_response_t for compatibility.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_netif_extra_info {
    pub type: u8,
    pub flags: u8,
    pub size: u16,
    pub type: u8,
    pub pad: u8,
    pub features: u16,
    pub gso: },
    pub addr: [u8; 6],
    pub mcast: },
    pub type: u8,
    pub algorithm: u8,
    pub value: [u8; 4],
    pub hash: },
    pub headroom: u16,
    pub pad: [u16; 2],
    pub xdp: },
    pub pad: [u16; 3],
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_netif_tx_response {
    pub id: u16,
    pub status: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_netif_rx_request {
    pub /: *mut *mut uint16_t id; / Echoed in response message.,
    pub pad: u16,
    pub gref: grant_ref_t,
}

// Packet data has been validated against protocol checksum.

// Protocol checksum field is blank in the packet (hardware offload)?

// Packet continues in the next request descriptor.

// Packet to be followed by extra descriptor(s).

// Packet has GSO prefix. Deprecated but included for compatibility

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_netif_rx_response {
    pub id: u16,
    pub offset: u16,
    pub flags: u16,
    pub status: i16,
}

//
// Generate xen_netif ring structures and types.
//

pub const XEN_NETIF_RSP_OKAY: c_int = 0;
// No response: used for auxiliary requests (e.g., xen_netif_extra_info_t).
pub const XEN_NETIF_RSP_NULL: c_int = 1;
