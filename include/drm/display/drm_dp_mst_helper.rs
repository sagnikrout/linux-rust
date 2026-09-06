//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/display/drm_dp_mst_helper.h
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


//
// Copyright © 2014 Red Hat.
//
// Permission to use, copy, modify, distribute, and sell this software and its
// documentation for any purpose is hereby granted without fee, provided that
// the above copyright notice appear in all copies and that both that copyright
// notice and this permission notice appear in supporting documentation, and
// that the name of the copyright holders not be used in advertising or
// publicity pertaining to distribution of the software without specific,
// written prior permission.  The copyright holders make no representations
// about the suitability of this software for any purpose.  It is provided "as
// is" without express or implied warranty.
//
// THE COPYRIGHT HOLDERS DISCLAIM ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
// INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
// EVENT SHALL THE COPYRIGHT HOLDERS BE LIABLE FOR ANY SPECIAL, INDIRECT OR
// CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
// DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
// TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE
// OF THIS SOFTWARE.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_dp_mst_topology_ref_type {
    DRM_DP_MST_TOPOLOGY_REF_GET,
    DRM_DP_MST_TOPOLOGY_REF_PUT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_mst_topology_ref_history {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_mst_topology_ref_entry {
    pub type: drm_dp_mst_topology_ref_type,
    pub count: c_int,
    pub ts_nsec: ktime_t,
    pub backtrace: depot_stack_handle_t,
    pub entries: *mut },
    pub len: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_dp_mst_payload_allocation {
    DRM_DP_MST_PAYLOAD_ALLOCATION_NONE,
    DRM_DP_MST_PAYLOAD_ALLOCATION_LOCAL,
    DRM_DP_MST_PAYLOAD_ALLOCATION_DFP,
    DRM_DP_MST_PAYLOAD_ALLOCATION_REMOTE,
}

//
// struct drm_dp_mst_port - MST port
// @port_num: port number
// @input: if this port is an input port. Protected by
// &drm_dp_mst_topology_mgr.base.lock.
// @mcs: message capability status - DP 1.2 spec. Protected by
// &drm_dp_mst_topology_mgr.base.lock.
// @ddps: DisplayPort Device Plug Status - DP 1.2. Protected by
// &drm_dp_mst_topology_mgr.base.lock.
// @pdt: Peer Device Type. Protected by
// &drm_dp_mst_topology_mgr.base.lock.
// @ldps: Legacy Device Plug Status. Protected by
// &drm_dp_mst_topology_mgr.base.lock.
// @dpcd_rev: DPCD revision of device on this port. Protected by
// &drm_dp_mst_topology_mgr.base.lock.
// @num_sdp_streams: Number of simultaneous streams. Protected by
// &drm_dp_mst_topology_mgr.base.lock.
// @num_sdp_stream_sinks: Number of stream sinks. Protected by
// &drm_dp_mst_topology_mgr.base.lock.
// @full_pbn: Max possible bandwidth for this port. Protected by
// &drm_dp_mst_topology_mgr.base.lock.
// @next: link to next port on this branch device
// @aux: i2c aux transport to talk to device connected to this port, protected
// by &drm_dp_mst_topology_mgr.base.lock.
// @passthrough_aux: parent aux to which DSC pass-through requests should be
// sent, only set if DSC pass-through is possible.
// @parent: branch device parent of this port
// @connector: DRM connector this port is connected to. Protected by
// &drm_dp_mst_topology_mgr.base.lock.
// @mgr: topology manager this port lives under.
//
// This structure represents an MST port endpoint on a device somewhere
// in the MST topology.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_mst_port {
//
// @topology_kref: refcount for this port's lifetime in the topology,
// only the DP MST helpers should need to touch this
//
    pub topology_kref: kref,
//
// @malloc_kref: refcount for the memory allocation containing this
// structure. See drm_dp_mst_get_port_malloc() and
// drm_dp_mst_put_port_malloc().
//
    pub malloc_kref: kref,

//
// @topology_ref_history: A history of each topology
// reference/dereference. See CONFIG_DRM_DEBUG_DP_MST_TOPOLOGY_REFS.
//
    pub topology_ref_history: drm_dp_mst_topology_ref_history,

    pub port_num: u8,
    pub input: bool,
    pub mcs: bool,
    pub ddps: bool,
    pub pdt: u8,
    pub ldps: bool,
    pub dpcd_rev: u8,
    pub num_sdp_streams: u8,
    pub num_sdp_stream_sinks: u8,
    pub full_pbn: u16,
    pub next: list_head,
//
// @mstb: the branch device connected to this port, if there is one.
// This should be considered protected for reading by
// &drm_dp_mst_topology_mgr.lock. There are two exceptions to this:
// &drm_dp_mst_topology_mgr.up_req_work and
// &drm_dp_mst_topology_mgr.work, which do not grab
// &drm_dp_mst_topology_mgr.lock during reads but are the only
// updaters of this list and are protected from writing concurrently
// by &drm_dp_mst_topology_mgr.probe_lock.
//
    pub mstb: *mut drm_dp_mst_branch,
    pub /: *mut *mut drm_dp_aux aux; / i2c bus for this port?,
    pub passthrough_aux: *mut drm_dp_aux,
    pub parent: *mut drm_dp_mst_branch,
    pub connector: *mut drm_connector,
    pub mgr: *mut drm_dp_mst_topology_mgr,
//
// @cached_edid: for DP logical ports - make tiling work by ensuring
// that the EDID for all connectors is read immediately.
//
    pub cached_edid: *const drm_edid,
//
// @fec_capable: bool indicating if FEC can be supported up to that
// point in the MST topology.
//
    pub fec_capable: bool,
}

// sideband msg header - not bit struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_sideband_msg_hdr {
    pub lct: u8,
    pub lcr: u8,
    pub rad: [u8; 8],
    pub broadcast: bool,
    pub path_msg: bool,
    pub msg_len: u8,
    pub somt: bool,
    pub eomt: bool,
    pub seqno: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_sideband_msg_rx {
    pub chunk: [u8; 48],
    pub msg: [u8; 256],
    pub curchunk_len: u8,
    pub /: *mut *mut u8 curchunk_idx; / chunk we are parsing now,
    pub curchunk_hdrlen: u8,
    pub /: *mut *mut u8 curlen; / total length of the msg,
    pub have_somt: bool,
    pub have_eomt: bool,
    pub initial_hdr: drm_dp_sideband_msg_hdr,
}

//
// struct drm_dp_mst_branch - MST branch device.
// @rad: Relative Address to talk to this branch device.
// @lct: Link count total to talk to this branch device.
// @num_ports: number of ports on the branch.
// @port_parent: pointer to the port parent, NULL if toplevel.
// @mgr: topology manager for this branch device.
// @link_address_sent: if a link address message has been sent to this device yet.
// @guid: guid for DP 1.2 branch device. port under this branch can be
// identified by port #.
//
// This structure represents an MST branch device, there is one
// primary branch device at the root, along with any other branches connected
// to downstream port of parent branches.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_mst_branch {
//
// @topology_kref: refcount for this branch device's lifetime in the
// topology, only the DP MST helpers should need to touch this
//
    pub topology_kref: kref,
//
// @malloc_kref: refcount for the memory allocation containing this
// structure. See drm_dp_mst_get_mstb_malloc() and
// drm_dp_mst_put_mstb_malloc().
//
    pub malloc_kref: kref,

//
// @topology_ref_history: A history of each topology
// reference/dereference. See CONFIG_DRM_DEBUG_DP_MST_TOPOLOGY_REFS.
//
    pub topology_ref_history: drm_dp_mst_topology_ref_history,

//
// @destroy_next: linked-list entry used by
// drm_dp_delayed_destroy_work()
//
    pub destroy_next: list_head,
//
// @rad: Relative Address of the MST branch.
// For &drm_dp_mst_topology_mgr.mst_primary, it's rad[8] are all 0,
// unset and unused. For MST branches connected after mst_primary,
// in each element of rad[] the nibbles are ordered by the most
// signifcant 4 bits first and the least significant 4 bits second.
//
    pub rad: [u8; 8],
    pub lct: u8,
    pub num_ports: c_int,
//
// @ports: the list of ports on this branch device. This should be
// considered protected for reading by &drm_dp_mst_topology_mgr.lock.
// There are two exceptions to this:
// &drm_dp_mst_topology_mgr.up_req_work and
// &drm_dp_mst_topology_mgr.work, which do not grab
// &drm_dp_mst_topology_mgr.lock during reads but are the only
// updaters of this list and are protected from updating the list
// concurrently by @drm_dp_mst_topology_mgr.probe_lock
//
    pub ports: list_head,
    pub port_parent: *mut drm_dp_mst_port,
    pub mgr: *mut drm_dp_mst_topology_mgr,
    pub link_address_sent: bool,
// global unique identifier to identify branch devices
    pub guid: guid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_nak_reply {
    pub guid: guid_t,
    pub reason: u8,
    pub nak_data: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_link_address_ack_reply {
    pub guid: guid_t,
    pub nports: u8,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_link_addr_reply_port {
    pub input_port: bool,
    pub peer_device_type: u8,
    pub port_number: u8,
    pub mcs: bool,
    pub ddps: bool,
    pub legacy_device_plug_status: bool,
    pub dpcd_revision: u8,
    pub peer_guid: guid_t,
    pub num_sdp_streams: u8,
    pub num_sdp_stream_sinks: u8,
    pub ports: [}; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_remote_dpcd_read_ack_reply {
    pub port_number: u8,
    pub num_bytes: u8,
    pub bytes: [u8; 255],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_remote_dpcd_write_ack_reply {
    pub port_number: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_remote_dpcd_write_nak_reply {
    pub port_number: u8,
    pub reason: u8,
    pub bytes_written_before_failure: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_remote_i2c_read_ack_reply {
    pub port_number: u8,
    pub num_bytes: u8,
    pub bytes: [u8; 255],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_remote_i2c_read_nak_reply {
    pub port_number: u8,
    pub nak_reason: u8,
    pub i2c_nak_transaction: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_remote_i2c_write_ack_reply {
    pub port_number: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_query_stream_enc_status_ack_reply {
// Bit[23:16]- Stream Id
    pub stream_id: u8,
// Bit[15]- Signed
    pub reply_signed: bool,
// Bit[10:8]- Stream Output Sink Type
    pub unauthorizable_device_present: bool,
    pub legacy_device_present: bool,
    pub query_capable_device_present: bool,
// Bit[12:11]- Stream Output CP Type
    pub hdcp_1x_device_present: bool,
    pub hdcp_2x_device_present: bool,
// Bit[4]- Stream Authentication
    pub auth_completed: bool,
// Bit[3]- Stream Encryption
    pub encryption_enabled: bool,
// Bit[2]- Stream Repeater Function Present
    pub repeater_present: bool,
// Bit[1:0]- Stream State
    pub state: u8,
}

pub const DRM_DP_MAX_SDP_STREAMS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_allocate_payload {
    pub port_number: u8,
    pub number_sdp_streams: u8,
    pub vcpi: u8,
    pub pbn: u16,
    pub sdp_stream_sink: [u8; DRM_DP_MAX_SDP_STREAMS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_allocate_payload_ack_reply {
    pub port_number: u8,
    pub vcpi: u8,
    pub allocated_pbn: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_connection_status_notify {
    pub guid: guid_t,
    pub port_number: u8,
    pub legacy_device_plug_status: bool,
    pub displayport_device_plug_status: bool,
    pub message_capability_status: bool,
    pub input_port: bool,
    pub peer_device_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_remote_dpcd_read {
    pub port_number: u8,
    pub dpcd_address: u32,
    pub num_bytes: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_remote_dpcd_write {
    pub port_number: u8,
    pub dpcd_address: u32,
    pub num_bytes: u8,
    pub bytes: *mut u8,
}

pub const DP_REMOTE_I2C_READ_MAX_TRANSACTIONS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_remote_i2c_read {
    pub num_transactions: u8,
    pub port_number: u8,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_remote_i2c_read_tx {
    pub i2c_dev_id: u8,
    pub num_bytes: u8,
    pub bytes: *mut u8,
    pub no_stop_bit: u8,
    pub i2c_transaction_delay: u8,
    pub transactions: [}; DP_REMOTE_I2C_READ_MAX_TRANSACTIONS],
    pub read_i2c_device_id: u8,
    pub num_bytes_read: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_remote_i2c_write {
    pub port_number: u8,
    pub write_i2c_device_id: u8,
    pub num_bytes: u8,
    pub bytes: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_query_stream_enc_status {
    pub stream_id: u8,
    pub /: *mut *mut u8 client_id[7]; / 56-bit nonce,
    pub stream_event: u8,
    pub valid_stream_event: bool,
    pub stream_behavior: u8,
    pub valid_stream_behavior: u8,
}

// this covers ENUM_RESOURCES, POWER_DOWN_PHY, POWER_UP_PHY
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_port_number_req {
    pub port_number: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_enum_path_resources_ack_reply {
    pub port_number: u8,
    pub fec_capable: bool,
    pub full_payload_bw_number: u16,
    pub avail_payload_bw_number: u16,
}

// covers POWER_DOWN_PHY, POWER_UP_PHY
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_port_number_rep {
    pub port_number: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_query_payload {
    pub port_number: u8,
    pub vcpi: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_resource_status_notify {
    pub port_number: u8,
    pub guid: guid_t,
    pub available_pbn: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_query_payload_ack_reply {
    pub port_number: u8,
    pub allocated_pbn: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_sideband_msg_req_body {
    pub req_type: u8,
#[repr(C)]
#[derive(Copy, Clone)]
pub union ack_req {
    pub conn_stat: drm_dp_connection_status_notify,
    pub port_num: drm_dp_port_number_req,
    pub resource_stat: drm_dp_resource_status_notify,
    pub query_payload: drm_dp_query_payload,
    pub allocate_payload: drm_dp_allocate_payload,
    pub dpcd_read: drm_dp_remote_dpcd_read,
    pub dpcd_write: drm_dp_remote_dpcd_write,
    pub i2c_read: drm_dp_remote_i2c_read,
    pub i2c_write: drm_dp_remote_i2c_write,
    pub enc_status: drm_dp_query_stream_enc_status,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_sideband_msg_reply_body {
    pub reply_type: u8,
    pub req_type: u8,
#[repr(C)]
#[derive(Copy, Clone)]
pub union ack_replies {
    pub nak: drm_dp_nak_reply,
    pub link_addr: drm_dp_link_address_ack_reply,
    pub port_number: drm_dp_port_number_rep,
    pub path_resources: drm_dp_enum_path_resources_ack_reply,
    pub allocate_payload: drm_dp_allocate_payload_ack_reply,
    pub query_payload: drm_dp_query_payload_ack_reply,
    pub remote_dpcd_read_ack: drm_dp_remote_dpcd_read_ack_reply,
    pub remote_dpcd_write_ack: drm_dp_remote_dpcd_write_ack_reply,
    pub remote_dpcd_write_nack: drm_dp_remote_dpcd_write_nak_reply,
    pub remote_i2c_read_ack: drm_dp_remote_i2c_read_ack_reply,
    pub remote_i2c_read_nack: drm_dp_remote_i2c_read_nak_reply,
    pub remote_i2c_write_ack: drm_dp_remote_i2c_write_ack_reply,
    pub enc_status: drm_dp_query_stream_enc_status_ack_reply,
    pub u: },
}

// msg is queued to be put into a slot
pub const DRM_DP_SIDEBAND_TX_QUEUED: c_int = 0;
// msg has started transmitting on a slot - still on msgq
pub const DRM_DP_SIDEBAND_TX_START_SEND: c_int = 1;
// msg has finished transmitting on a slot - removed from msgq only in slot
pub const DRM_DP_SIDEBAND_TX_SENT: c_int = 2;
// msg has received a response - removed from slot
pub const DRM_DP_SIDEBAND_TX_RX: c_int = 3;
pub const DRM_DP_SIDEBAND_TX_TIMEOUT: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_sideband_msg_tx {
    pub msg: [u8; 256],
    pub chunk: [u8; 48],
    pub cur_offset: u8,
    pub cur_len: u8,
    pub dst: *mut drm_dp_mst_branch,
    pub next: list_head,
    pub seqno: c_int,
    pub state: c_int,
    pub path_msg: bool,
    pub reply: drm_dp_sideband_msg_reply_body,
}

// sideband msg handler
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_mst_topology_cbs {
// create a connector for a port
    pub path): *const *const *const *const *const drm_connector (add_connector)(drm_dp_mst_topology_mgr mgr, drm_dp_mst_port port, char,
//
// Checks for any pending MST interrupts, passing them to MST core for
// processing, the same way an HPD IRQ pulse handler would do this.
// If provided MST core calls this callback from a poll-waiting loop
// when waiting for MST down message replies. The driver is expected
// to guard against a race between this callback and the driver's HPD
// IRQ pulse handler.
//
    pub mgr): *mut *mut void (poll_hpd_irq)(struct drm_dp_mst_topology_mgr,
}

//
// struct drm_dp_mst_atomic_payload - Atomic state struct for an MST payload
//
// The primary atomic state structure for a given MST payload. Stores information like current
// bandwidth allocation, intended action for this payload, etc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_mst_atomic_payload {
// @port: The MST port assigned to this payload
    pub port: *mut drm_dp_mst_port,
//
// @vc_start_slot: The time slot that this payload starts on. Because payload start slots
// can't be determined ahead of time, the contents of this value are UNDEFINED at atomic
// check time. This shouldn't usually matter, as the start slot should never be relevant for
// atomic state computations.
//
// Since this value is determined at commit time instead of check time, this value is
// protected by the MST helpers ensuring that async commits operating on the given topology
// never run in parallel. In the event that a driver does need to read this value (e.g. to
// inform hardware of the starting timeslot for a payload), the driver may either:
//
// * Read this field during the atomic commit after
// drm_dp_mst_atomic_wait_for_dependencies() has been called, which will ensure the
// previous MST states payload start slots have been copied over to the new state. Note
// that a new start slot won't be assigned/removed from this payload until
// drm_dp_add_payload_part1()/drm_dp_remove_payload_part2() have been called.
// * Acquire the MST modesetting lock, and then wait for any pending MST-related commits to
// get committed to hardware by calling drm_crtc_commit_wait() on each of the
// &drm_crtc_commit structs in &drm_dp_mst_topology_state.commit_deps.
//
// If neither of the two above solutions suffice (e.g. the driver needs to read the start
// slot in the middle of an atomic commit without waiting for some reason), then drivers
// should cache this value themselves after changing payloads.
//
    pub vc_start_slot: i8,
// @vcpi: The Virtual Channel Payload Identifier
    pub vcpi: u8,
//
// @time_slots:
// The number of timeslots allocated to this payload from the source DP Tx to
// the immediate downstream DP Rx
//
    pub time_slots: c_int,
// @pbn: The payload bandwidth for this payload
    pub pbn: c_int,
// @delete: Whether or not we intend to delete this payload during this atomic commit
    pub 1: bool delete :,
// @dsc_enabled: Whether or not this payload has DSC enabled
    pub 1: bool dsc_enabled :,
// @payload_allocation_status: The allocation status of this payload
    pub payload_allocation_status: drm_dp_mst_payload_allocation,
// @next: The list node for this payload
    pub next: list_head,
}

//
// struct drm_dp_mst_topology_state - DisplayPort MST topology atomic state
//
// This struct represents the atomic state of the toplevel DisplayPort MST manager
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_mst_topology_state {
// @base: Base private state for atomic
    pub base: drm_private_state,
// @mgr: The topology manager
    pub mgr: *mut drm_dp_mst_topology_mgr,
//
// @pending_crtc_mask: A bitmask of all CRTCs this topology state touches, drivers may
// modify this to add additional dependencies if needed.
//
    pub pending_crtc_mask: u32,
//
// @commit_deps: A list of all CRTC commits affecting this topology, this field isn't
// populated until drm_dp_mst_atomic_wait_for_dependencies() is called.
//
    pub commit_deps: *mut drm_crtc_commit,
// @num_commit_deps: The number of CRTC commits in @commit_deps
    pub num_commit_deps: usize,
// @payload_mask: A bitmask of allocated VCPIs, used for VCPI assignments
    pub payload_mask: u32,
// @payloads: The list of payloads being created/destroyed in this state
    pub payloads: list_head,
// @total_avail_slots: The total number of slots this topology can handle (63 or 64)
    pub total_avail_slots: u8,
// @start_slot: The first usable time slot in this topology (1 or 0)
    pub start_slot: u8,
//
// @pbn_div: The current PBN divisor for this topology. The driver is expected to fill this
// out itself.
//
    pub pbn_div: fixed20_12,
}

//
// struct drm_dp_mst_topology_mgr - DisplayPort MST manager
//
// This struct represents the toplevel displayport MST topology manager.
// There should be one instance of this for every MST capable DP connector
// on the GPU.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_mst_topology_mgr {
//
// @base: Base private object for atomic
//
    pub base: drm_private_obj,
//
// @dev: device pointer for adding i2c devices etc.
//
    pub dev: *mut drm_device,
//
// @cbs: callbacks for connector addition and destruction.
//
    pub cbs: *const drm_dp_mst_topology_cbs,
//
// @max_dpcd_transaction_bytes: maximum number of bytes to read/write
// in one go.
//
    pub max_dpcd_transaction_bytes: c_int,
//
// @aux: AUX channel for the DP MST connector this topolgy mgr is
// controlling.
//
    pub aux: *mut drm_dp_aux,
//
// @max_payloads: maximum number of payloads the GPU can generate.
//
    pub max_payloads: c_int,
//
// @conn_base_id: DRM connector ID this mgr is connected to. Only used
// to build the MST connector path value.
//
    pub conn_base_id: c_int,
//
// @up_req_recv: Message receiver state for up requests.
//
    pub up_req_recv: drm_dp_sideband_msg_rx,
//
// @down_rep_recv: Message receiver state for replies to down
// requests.
//
    pub down_rep_recv: drm_dp_sideband_msg_rx,
//
// @lock: protects @mst_state, @mst_primary, @dpcd, and
// @payload_id_table_cleared.
//
    pub lock: mutex,
//
// @probe_lock: Prevents @work and @up_req_work, the only writers of
// &drm_dp_mst_port.mstb and &drm_dp_mst_branch.ports, from racing
// while they update the topology.
//
    pub probe_lock: mutex,
//
// @mst_state: If this manager is enabled for an MST capable port. False
// if no MST sink/branch devices is connected.
//
    pub 1: bool mst_state :,
//
// @payload_id_table_cleared: Whether or not we've cleared the payload
// ID table for @mst_primary. Protected by @lock.
//
    pub 1: bool payload_id_table_cleared :,
//
// @reset_rx_state: The down request's reply and up request message
// receiver state must be reset, after the topology manager got
// removed. Protected by @lock.
//
    pub 1: bool reset_rx_state :,
//
// @payload_count: The number of currently active payloads in hardware. This value is only
// intended to be used internally by MST helpers for payload tracking, and is only safe to
// read/write from the atomic commit (not check) context.
//
    pub payload_count: u8,
//
// @next_start_slot: The starting timeslot to use for new VC payloads. This value is used
// internally by MST helpers for payload tracking, and is only safe to read/write from the
// atomic commit (not check) context.
//
    pub next_start_slot: u8,
//
// @mst_primary: Pointer to the primary/first branch device.
//
    pub mst_primary: *mut drm_dp_mst_branch,
//
// @dpcd: Cache of DPCD for primary port.
//
    pub dpcd: [u8; DP_RECEIVER_CAP_SIZE],
//
// @sink_count: Sink count from DEVICE_SERVICE_IRQ_VECTOR_ESI0.
//
    pub sink_count: u8,
//
// @funcs: Atomic helper callbacks
//
    pub funcs: *const drm_private_state_funcs,
//
// @qlock: protects @tx_msg_downq and &drm_dp_sideband_msg_tx.state
//
    pub qlock: mutex,
//
// @tx_msg_downq: List of pending down requests
//
    pub tx_msg_downq: list_head,
//
// @tx_waitq: Wait to queue stall for the tx worker.
//
    pub tx_waitq: wait_queue_head_t,
//
// @work: Probe work.
//
    pub work: work_struct,
//
// @tx_work: Sideband transmit worker. This can nest within the main
// @work worker for each transaction @work launches.
//
    pub tx_work: work_struct,
//
// @destroy_port_list: List of to be destroyed connectors.
//
    pub destroy_port_list: list_head,
//
// @destroy_branch_device_list: List of to be destroyed branch
// devices.
//
    pub destroy_branch_device_list: list_head,
//
// @delayed_destroy_lock: Protects @destroy_port_list and
// @destroy_branch_device_list.
//
    pub delayed_destroy_lock: mutex,
//
// @delayed_destroy_wq: Workqueue used for delayed_destroy_work items.
// A dedicated WQ makes it possible to drain any requeued work items
// on it.
//
    pub delayed_destroy_wq: *mut workqueue_struct,
//
// @delayed_destroy_work: Work item to destroy MST port and branch
// devices, needed to avoid locking inversion.
//
    pub delayed_destroy_work: work_struct,
//
// @up_req_list: List of pending up requests from the topology that
// need to be processed, in chronological order.
//
    pub up_req_list: list_head,
//
// @up_req_lock: Protects @up_req_list
//
    pub up_req_lock: mutex,
//
// @up_req_work: Work item to process up requests received from the
// topology. Needed to avoid blocking hotplug handling and sideband
// transmissions.
//
    pub up_req_work: work_struct,

//
// @topology_ref_history_lock: protects
// &drm_dp_mst_port.topology_ref_history and
// &drm_dp_mst_branch.topology_ref_history.
//
    pub topology_ref_history_lock: mutex,

}

extern "C" {
    pub fn drm_dp_mst_topology_mgr_destroy(mgr: *mut drm_dp_mst_topology_mgr);
}
//
// enum drm_dp_mst_mode - sink's MST mode capability
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_dp_mst_mode {
//
// @DRM_DP_SST: The sink does not support MST nor single stream sideband
// messaging.
//
    DRM_DP_SST,
//
// @DRM_DP_MST: Sink supports MST, more than one stream and single
// stream sideband messaging.
//
    DRM_DP_MST,
//
// @DRM_DP_SST_SIDEBAND_MSG: Sink supports only one stream and single
// stream sideband messaging.
//
    DRM_DP_SST_SIDEBAND_MSG,
}

extern "C" {
    pub fn drm_dp_read_mst_cap(aux: *mut drm_dp_aux, dpcd[DP_RECEIVER_CAP_SIZE]: u8) -> drm_dp_mst_mode;
}
extern "C" {
    pub fn drm_dp_mst_topology_mgr_set_mst(mgr: *mut drm_dp_mst_topology_mgr, mst_state: bool) -> c_int;
}
extern "C" {
    pub fn drm_dp_mst_hpd_irq_send_new_request(mgr: *mut drm_dp_mst_topology_mgr);
}
extern "C" {
    pub fn drm_dp_get_vc_payload_bw(link_rate: c_int, link_lane_count: c_int) -> fixed20_12;
}
extern "C" {
    pub fn drm_dp_calc_pbn_mode(clock: c_int, bpp: c_int) -> c_int;
}
extern "C" {
    pub fn drm_dp_mst_update_slots(mst_state: *mut drm_dp_mst_topology_state, link_encoding_cap: u8);
}
extern "C" {
    pub fn drm_dp_check_act_status(mgr: *mut drm_dp_mst_topology_mgr) -> c_int;
}
extern "C" {
    pub fn drm_dp_mst_topology_queue_probe(mgr: *mut drm_dp_mst_topology_mgr);
}
extern "C" {
    pub fn drm_dp_mst_topology_mgr_suspend(mgr: *mut drm_dp_mst_topology_mgr);
}
extern "C" {
    pub fn drm_dp_mst_atomic_wait_for_dependencies(state: *mut drm_atomic_commit);
}
extern "C" {
    pub fn drm_dp_mst_atomic_setup_commit(state: *mut drm_atomic_commit) -> int __must_check;
}
extern "C" {
    pub fn drm_dp_mst_atomic_check(state: *mut drm_atomic_commit) -> int __must_check;
}
extern "C" {
    pub fn drm_dp_mst_get_port_malloc(port: *mut drm_dp_mst_port);
}
extern "C" {
    pub fn drm_dp_mst_put_port_malloc(port: *mut drm_dp_mst_port);
}
extern "C" {
    pub fn container_of(_arg: state, drm_dp_mst_topology_state: struct, _arg: base) -> return;
}
//
// __drm_dp_mst_state_iter_get - private atomic state iterator function for
// macro-internal use
// @state: &struct drm_atomic_commit pointer
// @mgr: pointer to the &struct drm_dp_mst_topology_mgr iteration cursor
// @old_state: optional pointer to the old &struct drm_dp_mst_topology_state
// iteration cursor
// @new_state: optional pointer to the new &struct drm_dp_mst_topology_state
// iteration cursor
// @i: int iteration cursor, for macro-internal use
//
// Used by for_each_oldnew_mst_mgr_in_state(),
// for_each_old_mst_mgr_in_state(), and for_each_new_mst_mgr_in_state(). Don't
// call this directly.
//
// Returns:
// True if the current &struct drm_private_obj is a &struct
// drm_dp_mst_topology_mgr, false otherwise.
//
// mgr = to_dp_mst_topology_mgr(objs_state->ptr);
// old_state = to_dp_mst_topology_state(objs_state->old_state);
// new_state = to_dp_mst_topology_state(objs_state->new_state);
//
// for_each_oldnew_mst_mgr_in_state - iterate over all DP MST topology
// managers in an atomic update
// @__state: &struct drm_atomic_commit pointer
// @mgr: &struct drm_dp_mst_topology_mgr iteration cursor
// @old_state: &struct drm_dp_mst_topology_state iteration cursor for the old
// state
// @new_state: &struct drm_dp_mst_topology_state iteration cursor for the new
// state
// @__i: int iteration cursor, for macro-internal use
//
// This iterates over all DRM DP MST topology managers in an atomic update,
// tracking both old and new state. This is useful in places where the state
// delta needs to be considered, for example in atomic check functions.
//

//
// for_each_old_mst_mgr_in_state - iterate over all DP MST topology managers
// in an atomic update
// @__state: &struct drm_atomic_commit pointer
// @mgr: &struct drm_dp_mst_topology_mgr iteration cursor
// @old_state: &struct drm_dp_mst_topology_state iteration cursor for the old
// state
// @__i: int iteration cursor, for macro-internal use
//
// This iterates over all DRM DP MST topology managers in an atomic update,
// tracking only the old state. This is useful in disable functions, where we
// need the old state the hardware is still in.
//

//
// for_each_new_mst_mgr_in_state - iterate over all DP MST topology managers
// in an atomic update
// @__state: &struct drm_atomic_commit pointer
// @mgr: &struct drm_dp_mst_topology_mgr iteration cursor
// @new_state: &struct drm_dp_mst_topology_state iteration cursor for the new
// state
// @__i: int iteration cursor, for macro-internal use
//
// This iterates over all DRM DP MST topology managers in an atomic update,
// tracking only the new state. This is useful in enable functions, where we
// need the new state the hardware should be in when the atomic commit
// operation has completed.
//

