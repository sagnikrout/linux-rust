//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/surface_aggregator/controller.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Surface System Aggregator Module (SSAM) controller interface.
//
// Main communication interface for the SSAM EC. Provides a controller
// managing access and communication to and from the SSAM EC, as well as main
// communication structures and definitions.
//
// Copyright (C) 2019-2021 Maximilian Luz <luzmaximilian@gmail.com>
//

// -- Main data types and definitions ---------------------------------------
//
// enum ssam_event_flags - Flags for enabling/disabling SAM events
// @SSAM_EVENT_SEQUENCED: The event will be sent via a sequenced data frame.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ssam_event_flags {
    SSAM_EVENT_SEQUENCED = BIT(0),
}

//
// struct ssam_event - SAM event sent from the EC to the host.
// @target_category: Target category of the event source. See &enum ssam_ssh_tc.
// @target_id:       Target ID of the event source.
// @command_id:      Command ID of the event.
// @instance_id:     Instance ID of the event source.
// @length:          Length of the event payload in bytes.
// @data:            Event payload data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssam_event {
    pub target_category: u8,
    pub target_id: u8,
    pub command_id: u8,
    pub instance_id: u8,
    pub length: u16,
    pub __counted_by(length): u8 data[],
}

//
// enum ssam_request_flags - Flags for SAM requests.
//
// @SSAM_REQUEST_HAS_RESPONSE:
// Specifies that the request expects a response. If not set, the request
// will be directly completed after its underlying packet has been
// transmitted. If set, the request transport system waits for a response
// of the request.
//
// @SSAM_REQUEST_UNSEQUENCED:
// Specifies that the request should be transmitted via an unsequenced
// packet. If set, the request must not have a response, meaning that this
// flag and the %SSAM_REQUEST_HAS_RESPONSE flag are mutually exclusive.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ssam_request_flags {
    SSAM_REQUEST_HAS_RESPONSE = BIT(0),
    SSAM_REQUEST_UNSEQUENCED  = BIT(1),
}

//
// struct ssam_request - SAM request description.
// @target_category: Category of the request's target. See &enum ssam_ssh_tc.
// @target_id:       ID of the request's target.
// @command_id:      Command ID of the request.
// @instance_id:     Instance ID of the request's target.
// @flags:           Flags for the request. See &enum ssam_request_flags.
// @length:          Length of the request payload in bytes.
// @payload:         Request payload data.
//
// This struct fully describes a SAM request with payload. It is intended to
// help set up the actual transport struct, e.g. &struct ssam_request_sync,
// and specifically its raw message data via ssam_request_write_data().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssam_request {
    pub target_category: u8,
    pub target_id: u8,
    pub command_id: u8,
    pub instance_id: u8,
    pub flags: u16,
    pub length: u16,
    pub payload: *const u8,
}

//
// struct ssam_response - Response buffer for SAM request.
// @capacity: Capacity of the buffer, in bytes.
// @length:   Length of the actual data stored in the memory pointed to by
// @pointer, in bytes. Set by the transport system.
// @pointer:  Pointer to the buffer's memory, storing the response payload data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssam_response {
    pub capacity: usize,
    pub length: usize,
    pub pointer: *mut u8,
}

extern "C" {
    pub fn ssam_client_link(ctrl: *mut ssam_controller, client: *mut device) -> c_int;
}
extern "C" {
    pub fn ssam_controller_put(c: *mut ssam_controller);
}
extern "C" {
    pub fn ssam_controller_statelock(c: *mut ssam_controller);
}
extern "C" {
    pub fn ssam_controller_stateunlock(c: *mut ssam_controller);
}
// -- Synchronous request interface. ----------------------------------------
//
// struct ssam_request_sync - Synchronous SAM request struct.
// @base:   Underlying SSH request.
// @comp:   Completion used to signal full completion of the request. After the
// request has been submitted, this struct may only be modified or
// deallocated after the completion has been signaled.
// request has been submitted,
// @resp:   Buffer to store the response.
// @status: Status of the request, set after the base request has been
// completed or has failed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssam_request_sync {
    pub base: ssh_request,
    pub comp: completion,
    pub resp: *mut ssam_response,
    pub status: c_int,
}

extern "C" {
    pub fn ssam_request_sync_free(rqst: *mut ssam_request_sync);
}
//
// ssam_request_sync_set_data - Set message data of a synchronous request.
// @rqst: The request.
// @ptr:  Pointer to the request message data.
// @len:  Length of the request message data.
//
// Set the request message data of a synchronous request. The provided buffer
// needs to live until the request has been completed.
//
// ssam_request_sync_set_resp - Set response buffer of a synchronous request.
// @rqst: The request.
// @resp: The response buffer.
//
// Sets the response buffer of a synchronous request. This buffer will store
// the response of the request after it has been completed. May be %NULL if no
// response is expected.
//
// ssam_request_sync_wait - Wait for completion of a synchronous request.
// @rqst: The request to wait for.
//
// Wait for completion and release of a synchronous request. After this
// function terminates, the request is guaranteed to have left the transport
// system. After successful submission of a request, this function must be
// called before accessing the response of the request, freeing the request,
// or freeing any of the buffers associated with the request.
//
// This function must not be called if the request has not been submitted yet
// and may lead to a deadlock/infinite wait if a subsequent request submission
// fails in that case, due to the completion never triggering.
//
// Return: Returns the status of the given request, which is set on completion
// of the packet. This value is zero on success and negative on failure.
//
// ssam_request_do_sync_onstack - Execute a synchronous request on the stack.
// @ctrl: The controller via which the request is submitted.
// @rqst: The request specification.
// @rsp:  The response buffer.
// @payload_len: The (maximum) request payload length.
//
// Allocates a synchronous request with specified payload length on the stack,
// fully initializes it via the provided request specification, submits it,
// and finally waits for its completion before returning its status. This
// helper macro essentially allocates the request message buffer on the stack
// and then calls ssam_request_do_sync_with_buffer().
//
// Note: The @payload_len parameter specifies the maximum payload length, used
// for buffer allocation. The actual payload length may be smaller.
//
// Return: Returns the status of the request or any failure during setup, i.e.
// zero on success and a negative value on failure.
//

//
// __ssam_retry - Retry request in case of I/O errors or timeouts.
// @request: The request function to execute. Must return an integer.
// @n:       Number of tries.
// @args:    Arguments for the request function.
//
// Executes the given request function, i.e. calls @request. In case the
// request returns %-EREMOTEIO (indicates I/O error) or %-ETIMEDOUT (request
// or underlying packet timed out), @request will be re-executed again, up to
// @n times in total.
//
// Return: Returns the return value of the last execution of @request.
//

//
// ssam_retry - Retry request in case of I/O errors or timeouts up to three
// times in total.
// @request: The request function to execute. Must return an integer.
// @args:    Arguments for the request function.
//
// Executes the given request function, i.e. calls @request. In case the
// request returns %-EREMOTEIO (indicates I/O error) or -%ETIMEDOUT (request
// or underlying packet timed out), @request will be re-executed again, up to
// three times in total.
//
// See __ssam_retry() for a more generic macro for this purpose.
//
// Return: Returns the return value of the last execution of @request.
//

//
// struct ssam_request_spec - Blue-print specification of SAM request.
// @target_category: Category of the request's target. See &enum ssam_ssh_tc.
// @target_id:       ID of the request's target.
// @command_id:      Command ID of the request.
// @instance_id:     Instance ID of the request's target.
// @flags:           Flags for the request. See &enum ssam_request_flags.
//
// Blue-print specification for a SAM request. This struct describes the
// unique static parameters of a request (i.e. type) without specifying any of
// its instance-specific data (e.g. payload). It is intended to be used as base
// for defining simple request functions via the
// ``SSAM_DEFINE_SYNC_REQUEST_x()`` family of macros.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssam_request_spec {
    pub target_category: u8,
    pub target_id: u8,
    pub command_id: u8,
    pub instance_id: u8,
    pub flags: u8,
}

//
// struct ssam_request_spec_md - Blue-print specification for multi-device SAM
// request.
// @target_category: Category of the request's target. See &enum ssam_ssh_tc.
// @command_id:      Command ID of the request.
// @flags:           Flags for the request. See &enum ssam_request_flags.
//
// Blue-print specification for a multi-device SAM request, i.e. a request
// that is applicable to multiple device instances, described by their
// individual target and instance IDs. This struct describes the unique static
// parameters of a request (i.e. type) without specifying any of its
// instance-specific data (e.g. payload) and without specifying any of its
// device specific IDs (i.e. target and instance ID). It is intended to be
// used as base for defining simple multi-device request functions via the
// ``SSAM_DEFINE_SYNC_REQUEST_MD_x()`` and ``SSAM_DEFINE_SYNC_REQUEST_CL_x()``
// families of macros.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssam_request_spec_md {
    pub target_category: u8,
    pub command_id: u8,
    pub flags: u8,
}

//
// SSAM_DEFINE_SYNC_REQUEST_N() - Define synchronous SAM request function
// with neither argument nor return value.
// @name: Name of the generated function.
// @spec: Specification (&struct ssam_request_spec) defining the request.
//
// Defines a function executing the synchronous SAM request specified by
// @spec, with the request having neither argument nor return value. The
// generated function takes care of setting up the request struct and buffer
// allocation, as well as execution of the request itself, returning once the
// request has been fully completed. The required transport buffer will be
// allocated on the stack.
//
// The generated function is defined as ``static int name(struct
// ssam_controller *ctrl)``, returning the status of the request, which is
// zero on success and negative on failure. The ``ctrl`` parameter is the
// controller via which the request is being sent.
//
// Refer to ssam_request_do_sync_onstack() for more details on the behavior of
// the generated function.
//

//
// SSAM_DEFINE_SYNC_REQUEST_W() - Define synchronous SAM request function with
// argument.
// @name:  Name of the generated function.
// @atype: Type of the request's argument.
// @spec:  Specification (&struct ssam_request_spec) defining the request.
//
// Defines a function executing the synchronous SAM request specified by
// @spec, with the request taking an argument of type @atype and having no
// return value. The generated function takes care of setting up the request
// struct, buffer allocation, as well as execution of the request itself,
// returning once the request has been fully completed. The required transport
// buffer will be allocated on the stack.
//
// The generated function is defined as ``static int name(struct
// ssam_controller *ctrl, const atype *arg)``, returning the status of the
// request, which is zero on success and negative on failure. The ``ctrl``
// parameter is the controller via which the request is sent. The request
// argument is specified via the ``arg`` pointer.
//
// Refer to ssam_request_do_sync_onstack() for more details on the behavior of
// the generated function.
//

//
// SSAM_DEFINE_SYNC_REQUEST_R() - Define synchronous SAM request function with
// return value.
// @name:  Name of the generated function.
// @rtype: Type of the request's return value.
// @spec:  Specification (&struct ssam_request_spec) defining the request.
//
// Defines a function executing the synchronous SAM request specified by
// @spec, with the request taking no argument but having a return value of
// type @rtype. The generated function takes care of setting up the request
// and response structs, buffer allocation, as well as execution of the
// request itself, returning once the request has been fully completed. The
// required transport buffer will be allocated on the stack.
//
// The generated function is defined as ``static int name(struct
// ssam_controller *ctrl, rtype *ret)``, returning the status of the request,
// which is zero on success and negative on failure. The ``ctrl`` parameter is
// the controller via which the request is sent. The request's return value is
// written to the memory pointed to by the ``ret`` parameter.
//
// Refer to ssam_request_do_sync_onstack() for more details on the behavior of
// the generated function.
//

//
// SSAM_DEFINE_SYNC_REQUEST_WR() - Define synchronous SAM request function with
// both argument and return value.
// @name:  Name of the generated function.
// @atype: Type of the request's argument.
// @rtype: Type of the request's return value.
// @spec:  Specification (&struct ssam_request_spec) defining the request.
//
// Defines a function executing the synchronous SAM request specified by @spec,
// with the request taking an argument of type @atype and having a return value
// of type @rtype. The generated function takes care of setting up the request
// and response structs, buffer allocation, as well as execution of the request
// itself, returning once the request has been fully completed. The required
// transport buffer will be allocated on the stack.
//
// The generated function is defined as ``static int name(struct
// ssam_controller *ctrl, const atype *arg, rtype *ret)``, returning the status
// of the request, which is zero on success and negative on failure. The
// ``ctrl`` parameter is the controller via which the request is sent. The
// request argument is specified via the ``arg`` pointer. The request's return
// value is written to the memory pointed to by the ``ret`` parameter.
//
// Refer to ssam_request_do_sync_onstack() for more details on the behavior of
// the generated function.
//

//
// SSAM_DEFINE_SYNC_REQUEST_MD_N() - Define synchronous multi-device SAM
// request function with neither argument nor return value.
// @name: Name of the generated function.
// @spec: Specification (&struct ssam_request_spec_md) defining the request.
//
// Defines a function executing the synchronous SAM request specified by
// @spec, with the request having neither argument nor return value. Device
// specifying parameters are not hard-coded, but instead must be provided to
// the function. The generated function takes care of setting up the request
// struct, buffer allocation, as well as execution of the request itself,
// returning once the request has been fully completed. The required transport
// buffer will be allocated on the stack.
//
// The generated function is defined as ``static int name(struct
// ssam_controller *ctrl, u8 tid, u8 iid)``, returning the status of the
// request, which is zero on success and negative on failure. The ``ctrl``
// parameter is the controller via which the request is sent, ``tid`` the
// target ID for the request, and ``iid`` the instance ID.
//
// Refer to ssam_request_do_sync_onstack() for more details on the behavior of
// the generated function.
//

//
// SSAM_DEFINE_SYNC_REQUEST_MD_W() - Define synchronous multi-device SAM
// request function with argument.
// @name:  Name of the generated function.
// @atype: Type of the request's argument.
// @spec:  Specification (&struct ssam_request_spec_md) defining the request.
//
// Defines a function executing the synchronous SAM request specified by
// @spec, with the request taking an argument of type @atype and having no
// return value. Device specifying parameters are not hard-coded, but instead
// must be provided to the function. The generated function takes care of
// setting up the request struct, buffer allocation, as well as execution of
// the request itself, returning once the request has been fully completed.
// The required transport buffer will be allocated on the stack.
//
// The generated function is defined as ``static int name(struct
// ssam_controller *ctrl, u8 tid, u8 iid, const atype *arg)``, returning the
// status of the request, which is zero on success and negative on failure.
// The ``ctrl`` parameter is the controller via which the request is sent,
// ``tid`` the target ID for the request, and ``iid`` the instance ID. The
// request argument is specified via the ``arg`` pointer.
//
// Refer to ssam_request_do_sync_onstack() for more details on the behavior of
// the generated function.
//

//
// SSAM_DEFINE_SYNC_REQUEST_MD_R() - Define synchronous multi-device SAM
// request function with return value.
// @name:  Name of the generated function.
// @rtype: Type of the request's return value.
// @spec:  Specification (&struct ssam_request_spec_md) defining the request.
//
// Defines a function executing the synchronous SAM request specified by
// @spec, with the request taking no argument but having a return value of
// type @rtype. Device specifying parameters are not hard-coded, but instead
// must be provided to the function. The generated function takes care of
// setting up the request and response structs, buffer allocation, as well as
// execution of the request itself, returning once the request has been fully
// completed. The required transport buffer will be allocated on the stack.
//
// The generated function is defined as ``static int name(struct
// ssam_controller *ctrl, u8 tid, u8 iid, rtype *ret)``, returning the status
// of the request, which is zero on success and negative on failure. The
// ``ctrl`` parameter is the controller via which the request is sent, ``tid``
// the target ID for the request, and ``iid`` the instance ID. The request's
// return value is written to the memory pointed to by the ``ret`` parameter.
//
// Refer to ssam_request_do_sync_onstack() for more details on the behavior of
// the generated function.
//

//
// SSAM_DEFINE_SYNC_REQUEST_MD_WR() - Define synchronous multi-device SAM
// request function with both argument and return value.
// @name:  Name of the generated function.
// @atype: Type of the request's argument.
// @rtype: Type of the request's return value.
// @spec:  Specification (&struct ssam_request_spec_md) defining the request.
//
// Defines a function executing the synchronous SAM request specified by @spec,
// with the request taking an argument of type @atype and having a return value
// of type @rtype. Device specifying parameters are not hard-coded, but instead
// must be provided to the function. The generated function takes care of
// setting up the request and response structs, buffer allocation, as well as
// execution of the request itself, returning once the request has been fully
// completed. The required transport buffer will be allocated on the stack.
//
// The generated function is defined as ``static int name(struct
// ssam_controller *ctrl, u8 tid, u8 iid, const atype *arg, rtype *ret)``,
// returning the status of the request, which is zero on success and negative
// on failure. The ``ctrl`` parameter is the controller via which the request
// is sent, ``tid`` the target ID for the request, and ``iid`` the instance ID.
// The request argument is specified via the ``arg`` pointer. The request's
// return value is written to the memory pointed to by the ``ret`` parameter.
//
// Refer to ssam_request_do_sync_onstack() for more details on the behavior of
// the generated function.
//

// -- Event notifier/callbacks. ---------------------------------------------
pub const SSAM_NOTIF_STATE_SHIFT: c_int = 2;

//
// enum ssam_notif_flags - Flags used in return values from SSAM notifier
// callback functions.
//
// @SSAM_NOTIF_HANDLED:
// Indicates that the notification has been handled. This flag should be
// set by the handler if the handler can act/has acted upon the event
// provided to it. This flag should not be set if the handler is not a
// primary handler intended for the provided event.
//
// If this flag has not been set by any handler after the notifier chain
// has been traversed, a warning will be emitted, stating that the event
// has not been handled.
//
// @SSAM_NOTIF_STOP:
// Indicates that the notifier traversal should stop. If this flag is
// returned from a notifier callback, notifier chain traversal will
// immediately stop and any remaining notifiers will not be called. This
// flag is automatically set when ssam_notifier_from_errno() is called
// with a negative error value.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ssam_notif_flags {
    SSAM_NOTIF_HANDLED = BIT(0),
    SSAM_NOTIF_STOP    = BIT(1),
}

//
// struct ssam_notifier_block - Base notifier block for SSAM event
// notifications.
// @node:     The node for the list of notifiers.
// @fn:       The callback function of this notifier. This function takes the
// respective notifier block and event as input and should return
// a notifier value, which can either be obtained from the flags
// provided in &enum ssam_notif_flags, converted from a standard
// error value via ssam_notifier_from_errno(), or a combination of
// both (e.g. ``ssam_notifier_from_errno(e) | SSAM_NOTIF_HANDLED``).
// @priority: Priority value determining the order in which notifier callbacks
// will be called. A higher value means higher priority, i.e. the
// associated callback will be executed earlier than other (lower
// priority) callbacks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssam_notifier_block {
    pub node: list_head,
    pub fn: ssam_notifier_fn_t,
    pub priority: c_int,
}

//
// ssam_notifier_from_errno() - Convert standard error value to notifier
// return code.
// @err: The error code to convert, must be negative (in case of failure) or
// zero (in case of success).
//
// Return: Returns the notifier return value obtained by converting the
// specified @err value. In case @err is negative, the %SSAM_NOTIF_STOP flag
// will be set, causing notifier call chain traversal to abort.
//
// ssam_notifier_to_errno() - Convert notifier return code to standard error
// value.
// @ret: The notifier return value to convert.
//
// Return: Returns the negative error value encoded in @ret or zero if @ret
// indicates success.
//
// -- Event/notification registry. ------------------------------------------
//
// struct ssam_event_registry - Registry specification used for enabling events.
// @target_category: Target category for the event registry requests.
// @target_id:       Target ID for the event registry requests.
// @cid_enable:      Command ID for the event-enable request.
// @cid_disable:     Command ID for the event-disable request.
//
// This struct describes a SAM event registry via the minimal collection of
// SAM IDs specifying the requests to use for enabling and disabling an event.
// The individual event to be enabled/disabled itself is specified via &struct
// ssam_event_id.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssam_event_registry {
    pub target_category: u8,
    pub target_id: u8,
    pub cid_enable: u8,
    pub cid_disable: u8,
}

//
// struct ssam_event_id - Unique event ID used for enabling events.
// @target_category: Target category of the event source.
// @instance:        Instance ID of the event source.
//
// This struct specifies the event to be enabled/disabled via an externally
// provided registry. It does not specify the registry to be used itself, this
// is done via &struct ssam_event_registry.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssam_event_id {
    pub target_category: u8,
    pub instance: u8,
}

//
// enum ssam_event_mask - Flags specifying how events are matched to notifiers.
//
// @SSAM_EVENT_MASK_NONE:
// Run the callback for any event with matching target category. Do not
// do any additional filtering.
//
// @SSAM_EVENT_MASK_TARGET:
// In addition to filtering by target category, only execute the notifier
// callback for events with a target ID matching to the one of the
// registry used for enabling/disabling the event.
//
// @SSAM_EVENT_MASK_INSTANCE:
// In addition to filtering by target category, only execute the notifier
// callback for events with an instance ID matching to the instance ID
// used when enabling the event.
//
// @SSAM_EVENT_MASK_STRICT:
// Do all the filtering above.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ssam_event_mask {
    SSAM_EVENT_MASK_TARGET   = BIT(0),
    SSAM_EVENT_MASK_INSTANCE = BIT(1),

    SSAM_EVENT_MASK_NONE = 0,
    SSAM_EVENT_MASK_STRICT =
    SSAM_EVENT_MASK_TARGET
    | SSAM_EVENT_MASK_INSTANCE,
}

//
// SSAM_EVENT_REGISTRY() - Define a new event registry.
// @tc:      Target category for the event registry requests.
// @tid:     Target ID for the event registry requests.
// @cid_en:  Command ID for the event-enable request.
// @cid_dis: Command ID for the event-disable request.
//
// Return: Returns the &struct ssam_event_registry specified by the given
// parameters.
//

// Macro flag: #define SSAM_EVENT_REGISTRY_REG(tid)\
//
// enum ssam_event_notifier_flags - Flags for event notifiers.
// @SSAM_EVENT_NOTIFIER_OBSERVER:
// The corresponding notifier acts as observer. Registering a notifier
// with this flag set will not attempt to enable any event. Equally,
// unregistering will not attempt to disable any event. Note that a
// notifier with this flag may not even correspond to a certain event at
// all, only to a specific event target category. Event matching will not
// be influenced by this flag.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ssam_event_notifier_flags {
    SSAM_EVENT_NOTIFIER_OBSERVER = BIT(0),
}

//
// struct ssam_event_notifier - Notifier block for SSAM events.
// @base:        The base notifier block with callback function and priority.
// @event:       The event for which this block will receive notifications.
// @event.reg:   Registry via which the event will be enabled/disabled.
// @event.id:    ID specifying the event.
// @event.mask:  Flags determining how events are matched to the notifier.
// @event.flags: Flags used for enabling the event.
// @flags:       Notifier flags (see &enum ssam_event_notifier_flags).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssam_event_notifier {
    pub base: ssam_notifier_block,
    pub reg: ssam_event_registry,
    pub id: ssam_event_id,
    pub mask: ssam_event_mask,
    pub flags: u8,
    pub event: },
    pub flags: c_ulong,
}

//
// ssam_notifier_unregister() - Unregister an event notifier.
// @ctrl:    The controller the notifier has been registered on.
// @n:       The event notifier to unregister.
//
// Unregister an event notifier. Decrement the usage counter of the associated
// SAM event if the notifier is not marked as an observer. If the usage counter
// reaches zero, the event will be disabled.
//
// Return: Returns zero on success, %-ENOENT if the given notifier block has
// not been registered on the controller. If the given notifier block was the
// last one associated with its specific event, returns the status of the
// event-disable EC-command.
//
extern "C" {
    pub fn __ssam_notifier_unregister(_arg: ctrl, _arg: n, _arg: true) -> return;
}
