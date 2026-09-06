//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/interface/io/ring.h
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
// ring.h
//
// Shared producer-consumer ring macros.
//
// Tim Deegan and Andrew Warfield November 2004.
//
// When #include'ing this header, you need to provide the following
// declaration upfront:
// - standard integers types (uint8_t, uint16_t, etc)
// They are provided by stdint.h of the standard headers.
//
// In addition, if you intend to use the FLEX macros, you also need to
// provide the following, before invoking the FLEX macros:
// - size_t
// - memcpy
// - grant_ref_t
// These declarations are provided by string.h of the standard headers,
// and grant_table.h from the Xen public headers.
//

pub type RING_IDX = c_uint;
// Round a 32-bit unsigned constant down to the nearest power of two.

//
// Calculate size of a shared ring, given the total available space for the
// ring and indexes (_sz), and the name tag of the request/response structure.
// A ring contains as many entries as will fit, rounded down to the nearest
// power of two (so we can mask with (size-1) to loop around).
//

//
// The same for passing in an actual pointer instead of a name tag.
//

//
// Macros to make the correct C datatypes for a new kind of ring.
//
// To make a new ring datatype, you need to have two message structures,
// let's say request_t, and response_t already defined.
//
// In a header where you want the ring datatype declared, you then do:
//
// DEFINE_RING_TYPES(mytag, request_t, response_t);
//
// These expand out to give you a set of types, as you can see below.
// The most important of these are:
//
// mytag_sring_t      - The shared ring.
// mytag_front_ring_t - The 'front' half of the ring.
// mytag_back_ring_t  - The 'back' half of the ring.
//
// To initialize a ring in your code you need to know the location and size
// of the shared memory area (PAGE_SIZE, for instance). To initialise
// the front half:
//
// mytag_front_ring_t ring;
// XEN_FRONT_RING_INIT(&ring, (mytag_sring_t *)shared_page, PAGE_SIZE);
//
// Initializing the back follows similarly (note that only the front
// initializes the shared ring):
//
// mytag_back_ring_t back_ring;
// BACK_RING_INIT(&back_ring, (mytag_sring_t *)shared_page, PAGE_SIZE);
//

// Shared ring entry */                                                 \
// Shared ring page */                                                  \
// "Front" end's private variables */                                   \
// "Back" end's private variables */                                    \
//
// Macros for manipulating rings.
//
// FRONT_RING_whatever works on the "front end" of a ring: here
// requests are pushed on to the ring and responses taken off it.
//
// BACK_RING_whatever works on the "back end" of a ring: here
// requests are taken off the ring and responses put on.
//
// N.B. these macros do NO INTERLOCKS OR FLOW CONTROL.
// This is OK in 1-for-1 request-response situations where the
// requestor (front end) never has more than RING_SIZE()-1
// outstanding requests.
//
// Initialising empty rings

// How big is this ring?

// Number of free requests (for use on front side only).

// Test if there is an empty slot available on the front ring.
// (This is only meaningful from the front. )
//

// Test if there are outstanding messages to be processed on a ring.

// Direct access to individual ring elements, by index.

//
// Get a local copy of a request/response.
//
// Use this in preference to RING_GET_{REQUEST,RESPONSE}() so all processing is
// done on a local copy that cannot be modified by the other end.
//
// Note that https://gcc.gnu.org/bugzilla/show_bug.cgi?id=58145 may cause this
// to be ineffective where dest is a struct which consists of only bitfields.
//

// Use volatile to force the copy into dest. */			\
// (dest) = *(volatile typeof(dest))RING_GET_##type(r, idx);	\

// Loop termination condition: Would the specified index overflow the ring?

// Ill-behaved frontend determination: Can there be this many requests?

// Ill-behaved backend determination: Can there be this many responses?

//
// Notification hold-off (req_event and rsp_event):
//
// When queueing requests or responses on a shared ring, it may not always be
// necessary to notify the remote end. For example, if requests are in flight
// in a backend, the front may be able to queue further requests without
// notifying the back (if the back checks for new requests when it queues
// responses).
//
// When enqueuing requests or responses:
//
// Use RING_PUSH_{REQUESTS,RESPONSES}_AND_CHECK_NOTIFY(). The second argument
// is a boolean return value. True indicates that the receiver requires an
// asynchronous notification.
//
// After dequeuing requests or responses (before sleeping the connection):
//
// Use RING_FINAL_CHECK_FOR_REQUESTS() or RING_FINAL_CHECK_FOR_RESPONSES().
// The second argument is a boolean return value. True indicates that there
// are pending messages on the ring (i.e., the connection should not be put
// to sleep).
//
// These macros will set the req_event/rsp_event field to trigger a
// notification on the very next message that is enqueued. If you want to
// create batches of work (i.e., only receive a notification after several
// messages have been enqueued) then you will need to create a customised
// version of the FINAL_CHECK macro in your own code, which sets the event
// field appropriately.
//

//
// DEFINE_XEN_FLEX_RING_AND_INTF defines two monodirectional rings and
// functions to check if there is data on the ring, and to read and
// write to them.
//
// DEFINE_XEN_FLEX_RING is similar to DEFINE_XEN_FLEX_RING_AND_INTF, but
// does not define the indexes page. As different protocols can have
// extensions to the basic format, this macro allow them to define their
// own struct.
//
// XEN_FLEX_RING_SIZE
// Convenience macro to calculate the size of one of the two rings
// from the overall order.
//
// $NAME_mask
// Function to apply the size mask to an index, to reduce the index
// within the range [0-size].
//
// $NAME_read_packet
// Function to read data from the ring. The amount of data to read is
// specified by the "size" argument.
//
// $NAME_write_packet
// Function to write data to the ring. The amount of data to write is
// specified by the "size" argument.
//
// $NAME_get_ring_ptr
// Convenience function that returns a pointer to read/write to the
// ring at the right location.
//
// $NAME_data_intf
// Indexes page, shared between frontend and backend. It also
// contains the array of grant refs.
//
// $NAME_queued
// Function to calculate how many bytes are currently on the ring,
// ready to be read. It can also be used to calculate how much free
// space is currently on the ring (XEN_FLEX_RING_SIZE() -
// $NAME_queued()).
//

// The PAGE_SIZE for ring protocols and hypercall interfaces is always
// 4K, regardless of the architecture, and page granularity chosen by
// operating systems.
//
pub const XEN_PAGE_SHIFT: c_int = 12;

// masked_cons = name##_mask(*masked_cons + size, ring_size);               \
// masked_prod = name##_mask(*masked_prod + size, ring_size);               \

