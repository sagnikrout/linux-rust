//! Automatically rewritten from C Header to Rust Module
//! Source: include/rdma/rdma_cm.h
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
// Copyright (c) 2005 Voltaire Inc.  All rights reserved.
// Copyright (c) 2005 Intel Corporation.  All rights reserved.
//

//
// Upon receiving a device removal event, users must destroy the associated
// RDMA identifier and release all resources allocated with the device.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdma_cm_event_type {
    RDMA_CM_EVENT_ADDR_RESOLVED,
    RDMA_CM_EVENT_ADDR_ERROR,
    RDMA_CM_EVENT_ROUTE_RESOLVED,
    RDMA_CM_EVENT_ROUTE_ERROR,
    RDMA_CM_EVENT_CONNECT_REQUEST,
    RDMA_CM_EVENT_CONNECT_RESPONSE,
    RDMA_CM_EVENT_CONNECT_ERROR,
    RDMA_CM_EVENT_UNREACHABLE,
    RDMA_CM_EVENT_REJECTED,
    RDMA_CM_EVENT_ESTABLISHED,
    RDMA_CM_EVENT_DISCONNECTED,
    RDMA_CM_EVENT_DEVICE_REMOVAL,
    RDMA_CM_EVENT_MULTICAST_JOIN,
    RDMA_CM_EVENT_MULTICAST_ERROR,
    RDMA_CM_EVENT_ADDR_CHANGE,
    RDMA_CM_EVENT_TIMEWAIT_EXIT,
    RDMA_CM_EVENT_ADDRINFO_RESOLVED,
    RDMA_CM_EVENT_ADDRINFO_ERROR,
    RDMA_CM_EVENT_USER,
    RDMA_CM_EVENT_INTERNAL,
}

extern "C" {
    pub fn rdma_event_msg(event: rdma_cm_event_type) -> *const char __attribute_const__;
}
pub const RDMA_IB_IP_PS_MASK: c_uint = 0xFFFFFFFFFFFF0000ULL;
pub const RDMA_IB_IP_PS_TCP: c_uint = 0x0000000001060000ULL;
pub const RDMA_IB_IP_PS_UDP: c_uint = 0x0000000001110000ULL;
pub const RDMA_IB_IP_PS_IB: c_uint = 0x00000000013F0000ULL;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_addr {
    pub src_addr: sockaddr_storage,
    pub dst_addr: sockaddr_storage,
    pub dev_addr: rdma_dev_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_route {
    pub addr: rdma_addr,
    pub path_rec: *mut sa_path_rec,
// Optional path records of primary path
    pub path_rec_inbound: *mut sa_path_rec,
    pub path_rec_outbound: *mut sa_path_rec,
//
// 0 - No primary nor alternate path is available
// 1 - Only primary path is available
// 2 - Both primary and alternate path are available
//
    pub num_pri_alt_paths: c_int,
    pub num_service_recs: c_uint,
    pub service_recs: *mut sa_service_rec,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_conn_param {
    pub private_data: *const c_void,
    pub private_data_len: u8,
    pub responder_resources: u8,
    pub initiator_depth: u8,
    pub flow_control: u8,
    pub /: *mut *mut u8 retry_count; / ignored when accepting,
    pub rnr_retry_count: u8,
// Fields below ignored if a QP is created on the rdma_cm_id.
    pub srq: u8,
    pub qp_num: u32,
    pub qkey: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ud_param {
    pub private_data: *const c_void,
    pub private_data_len: u8,
    pub ah_attr: rdma_ah_attr,
    pub qp_num: u32,
    pub qkey: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_cm_event {
    pub event: rdma_cm_event_type,
    pub status: c_int,
    pub conn: rdma_conn_param,
    pub ud: rdma_ud_param,
    pub arg: u64,
    pub param: },
    pub ece: rdma_ucm_ece,
}

//
// rdma_cm_event_handler - Callback used to report user events.
//
// Notes: Users may not call rdma_destroy_id from this callback to destroy
// the passed in id, or a corresponding listen id.  Returning a
// non-zero value from the callback will destroy the passed in id.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_cm_id {
    pub device: *mut ib_device,
    pub context: *mut c_void,
    pub qp: *mut ib_qp,
    pub event_handler: rdma_cm_event_handler,
    pub route: rdma_route,
    pub ps: rdma_ucm_port_space,
    pub qp_type: ib_qp_type,
    pub port_num: u32,
    pub net_work: work_struct,
}

//
// rdma_create_id - Create an RDMA identifier.
//
// @net: The network namespace in which to create the new id.
// @event_handler: User callback invoked to report events associated with the
// returned rdma_id.
// @context: User specified context associated with the id.
// @ps: RDMA port space.
// @qp_type: type of queue pair associated with the id.
//
// Returns a new rdma_cm_id. The id holds a reference on the network
// namespace until it is destroyed.
//
// The event handler callback serializes on the id's mutex and is
// allowed to sleep.
//

//
// rdma_destroy_id - Destroys an RDMA identifier.
//
// @id: RDMA identifier.
//
// Note: calling this function has the effect of canceling in-flight
// asynchronous operations associated with the id.
//
extern "C" {
    pub fn rdma_destroy_id(id: *mut rdma_cm_id);
}
//
// rdma_restrict_node_type - Restrict an RDMA identifier to specific
// RDMA device node type.
//
// @id: RDMA identifier.
// @node_type: The device node type. Only RDMA_NODE_UNSPECIFIED (default),
// RDMA_NODE_RNIC and RDMA_NODE_IB_CA are allowed
//
// This allows the caller to restrict the possible devices
// used to iWarp (RDMA_NODE_RNIC) or InfiniBand/RoCEv1/RoCEv2 (RDMA_NODE_IB_CA).
//
// It needs to be called before the RDMA identifier is bound
// to an device, which mean it should be called before
// rdma_bind_addr(), rdma_resolve_addr() and rdma_listen().
//
extern "C" {
    pub fn rdma_restrict_node_type(id: *mut rdma_cm_id, node_type: u8) -> c_int;
}
//
// rdma_bind_addr - Bind an RDMA identifier to a source address and
// associated RDMA device, if needed.
//
// @id: RDMA identifier.
// @addr: Local address information.  Wildcard values are permitted.
//
// This associates a source address with the RDMA identifier before calling
// rdma_listen.  If a specific local address is given, the RDMA identifier will
// be bound to a local RDMA device.
//
extern "C" {
    pub fn rdma_bind_addr(id: *mut rdma_cm_id, addr: *mut sockaddr) -> c_int;
}
//
// rdma_resolve_addr - Resolve destination and optional source addresses
// from IP addresses to an RDMA address.  If successful, the specified
// rdma_cm_id will be bound to a local device.
//
// @id: RDMA identifier.
// @src_addr: Source address information.  This parameter may be NULL.
// @dst_addr: Destination address information.
// @timeout_ms: Time to wait for resolution to complete.
//
// rdma_resolve_route - Resolve the RDMA address bound to the RDMA identifier
// into route information needed to establish a connection.
//
// This is called on the client side of a connection.
// Users must have first called rdma_resolve_addr to resolve a dst_addr
// into an RDMA address before calling this routine.
//
extern "C" {
    pub fn rdma_resolve_route(id: *mut rdma_cm_id, timeout_ms: c_ulong) -> c_int;
}
//
// rdma_resolve_ib_service - Resolve the IB service record of the
// service with the given service ID or name.
//
// This function is optional in the rdma cm flow. It is called on the client
// side of a connection, before calling rdma_resolve_route. The resolution
// can be done once per rdma_cm_id.
//
// rdma_create_qp - Allocate a QP and associate it with the specified RDMA
// identifier.
//
// QPs allocated to an rdma_cm_id will automatically be transitioned by the CMA
// through their states.
//
// rdma_destroy_qp - Deallocate the QP associated with the specified RDMA
// identifier.
//
// Users must destroy any QP associated with an RDMA identifier before
// destroying the RDMA ID.
//
extern "C" {
    pub fn rdma_destroy_qp(id: *mut rdma_cm_id);
}
//
// rdma_init_qp_attr - Initializes the QP attributes for use in transitioning
// to a specified QP state.
// @id: Communication identifier associated with the QP attributes to
// initialize.
// @qp_attr: On input, specifies the desired QP state.  On output, the
// mandatory and desired optional attributes will be set in order to
// modify the QP to the specified state.
// @qp_attr_mask: The QP attribute mask that may be used to transition the
// QP to the specified state.
//
// Users must set the @qp_attr->qp_state to the desired QP state.  This call
// will set all required attributes for the given transition, along with
// known optional attributes.  Users may override the attributes returned from
// this call before calling ib_modify_qp.
//
// Users that wish to have their QP automatically transitioned through its
// states can associate a QP with the rdma_cm_id by calling rdma_create_qp().
//
extern "C" {
    pub fn rdma_connect(id: *mut rdma_cm_id, conn_param: *mut rdma_conn_param) -> c_int;
}
//
// rdma_listen - This function is called by the passive side to
// listen for incoming connection requests.
//
// Users must have bound the rdma_cm_id to a local address by calling
// rdma_bind_addr before calling this routine.
//
extern "C" {
    pub fn rdma_listen(id: *mut rdma_cm_id, backlog: c_int) -> c_int;
}
extern "C" {
    pub fn rdma_accept(id: *mut rdma_cm_id, conn_param: *mut rdma_conn_param) -> c_int;
}
extern "C" {
    pub fn rdma_lock_handler(id: *mut rdma_cm_id);
}
extern "C" {
    pub fn rdma_unlock_handler(id: *mut rdma_cm_id);
}
//
// rdma_notify - Notifies the RDMA CM of an asynchronous event that has
// occurred on the connection.
// @id: Connection identifier to transition to established.
// @event: Asynchronous event.
//
// This routine should be invoked by users to notify the CM of relevant
// communication events.  Events that should be reported to the CM and
// when to report them are:
//
// IB_EVENT_COMM_EST - Used when a message is received on a connected
// QP before an RTU has been received.
//
extern "C" {
    pub fn rdma_notify(id: *mut rdma_cm_id, event: ib_event_type) -> c_int;
}
//
// rdma_reject - Called to reject a connection request or response.
//
// rdma_disconnect - This function disconnects the associated QP and
// transitions it into the error state.
//
extern "C" {
    pub fn rdma_disconnect(id: *mut rdma_cm_id) -> c_int;
}
//
// rdma_join_multicast - Join the multicast group specified by the given
// address.
// @id: Communication identifier associated with the request.
// @addr: Multicast address identifying the group to join.
// @join_state: Multicast JoinState bitmap requested by port.
// Bitmap is based on IB_SA_MCMEMBER_REC_JOIN_STATE bits.
// @context: User-defined context associated with the join request, returned
// to the user through the private_data pointer in multicast events.
//
// rdma_leave_multicast - Leave the multicast group specified by the given
// address.
//
extern "C" {
    pub fn rdma_leave_multicast(id: *mut rdma_cm_id, addr: *mut sockaddr);
}
//
// rdma_set_service_type - Set the type of service associated with a
// connection identifier.
// @id: Communication identifier to associated with service type.
// @tos: Type of service.
//
// The type of service is interpretted as a differentiated service
// field (RFC 2474).  The service type should be specified before
// performing route resolution, as existing communication on the
// connection identifier may be unaffected.  The type of service
// requested may not be supported by the network to all destinations.
//
extern "C" {
    pub fn rdma_set_service_type(id: *mut rdma_cm_id, tos: c_int);
}
//
// rdma_set_reuseaddr - Allow the reuse of local addresses when binding
// the rdma_cm_id.
// @id: Communication identifier to configure.
// @reuse: Value indicating if the bound address is reusable.
//
// Reuse must be set before an address is bound to the id.
//
extern "C" {
    pub fn rdma_set_reuseaddr(id: *mut rdma_cm_id, reuse: c_int) -> c_int;
}
//
// rdma_set_afonly - Specify that listens are restricted to the
// bound address family only.
// @id: Communication identifer to configure.
// @afonly: Value indicating if listens are restricted.
//
// Must be set before identifier is in the listening state.
//
extern "C" {
    pub fn rdma_set_afonly(id: *mut rdma_cm_id, afonly: c_int) -> c_int;
}
extern "C" {
    pub fn rdma_set_ack_timeout(id: *mut rdma_cm_id, timeout: u8) -> c_int;
}
extern "C" {
    pub fn rdma_set_min_rnr_timer(id: *mut rdma_cm_id, min_rnr_timer: u8) -> c_int;
}
//
// rdma_get_service_id - Return the IB service ID for a specified address.
// @id: Communication identifier associated with the address.
// @addr: Address for the service ID.
//
extern "C" {
    pub fn rdma_get_service_id(id: *mut rdma_cm_id, addr: *mut sockaddr) -> __be64;
}
//
// rdma_reject_msg - return a pointer to a reject message string.
// @id: Communication identifier that received the REJECT event.
// @reason: Value returned in the REJECT event status field.
//
// rdma_consumer_reject_data - return the consumer reject private data and
// length, if any.
// @id: Communication identifier that received the REJECT event.
// @ev: RDMA CM reject event.
// @data_len: Pointer to the resulting length of the consumer data.
//
// rdma_read_gids - Return the SGID and DGID used for establishing
// connection. This can be used after rdma_resolve_addr()
// on client side. This can be use on new connection
// on server side. This is applicable to IB, RoCE, iWarp.
// If cm_id is not bound yet to the RDMA device, it doesn't
// copy and SGID or DGID to the given pointers.
// @id: Communication identifier whose GIDs are queried.
// @sgid: Pointer to SGID where SGID will be returned. It is optional.
// @dgid: Pointer to DGID where DGID will be returned. It is optional.
// Note: This API should not be used by any new ULPs or new code.
// Instead, users interested in querying GIDs should refer to path record
// of the rdma_cm_id to query the GIDs.
// This API is provided for compatibility for existing users.
//
