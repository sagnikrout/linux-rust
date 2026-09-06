//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/v4l2-event.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// v4l2-event.h
//
// V4L2 events.
//
// Copyright (C) 2009--2010 Nokia Corporation.
//
// Contact: Sakari Ailus <sakari.ailus@iki.fi>
//

//
// struct v4l2_kevent - Internal kernel event struct.
// @list:	List node for the v4l2_fh->available list.
// @sev:	Pointer to parent v4l2_subscribed_event.
// @event:	The event itself.
// @ts:		The timestamp of the event.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_kevent {
    pub list: list_head,
    pub sev: *mut v4l2_subscribed_event,
    pub event: v4l2_event,
    pub ts: u64,
}

//
// struct v4l2_subscribed_event_ops - Subscribed event operations.
//
// @add:	Optional callback, called when a new listener is added
// @del:	Optional callback, called when a listener stops listening
// @replace:	Optional callback that can replace event 'old' with event 'new'.
// @merge:	Optional callback that can merge event 'old' into event 'new'.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subscribed_event_ops {
    pub elems): *mut *mut *mut int (add)(struct v4l2_subscribed_event sev, unsigned int,
    pub sev): *mut *mut void (del)(struct v4l2_subscribed_event,
    pub new): *const *const *const void (replace)(struct v4l2_event old, struct v4l2_event,
    pub new): *const *const *const void (merge)(struct v4l2_event old, struct v4l2_event,
}

//
// struct v4l2_subscribed_event - Internal struct representing a subscribed
// event.
//
// @list:	List node for the v4l2_fh->subscribed list.
// @type:	Event type.
// @id:	Associated object ID (e.g. control ID). 0 if there isn't any.
// @flags:	Copy of v4l2_event_subscription->flags.
// @fh:	Filehandle that subscribed to this event.
// @node:	List node that hooks into the object's event list
// (if there is one).
// @ops:	v4l2_subscribed_event_ops
// @elems:	The number of elements in the events array.
// @first:	The index of the events containing the oldest available event.
// @in_use:	The number of queued events.
// @events:	An array of @elems events.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subscribed_event {
    pub list: list_head,
    pub type: u32,
    pub id: u32,
    pub flags: u32,
    pub fh: *mut v4l2_fh,
    pub node: list_head,
    pub ops: *const v4l2_subscribed_event_ops,
    pub elems: c_uint,
    pub first: c_uint,
    pub in_use: c_uint,
    pub __counted_by(elems): v4l2_kevent events[],
}

//
// v4l2_event_dequeue - Dequeue events from video device.
//
// @fh: pointer to struct v4l2_fh
// @event: pointer to struct v4l2_event
// @nonblocking: if not zero, waits for an event to arrive
//
// v4l2_event_queue - Queue events to video device.
//
// @vdev: pointer to &struct video_device
// @ev: pointer to &struct v4l2_event
//
// The event will be queued for all &struct v4l2_fh file handlers.
//
// .. note::
// The driver's only responsibility is to fill in the type and the data
// fields. The other fields will be filled in by V4L2.
//
extern "C" {
    pub fn v4l2_event_queue(vdev: *mut video_device, ev: *const v4l2_event);
}
//
// v4l2_event_queue_fh - Queue events to video device.
//
// @fh: pointer to &struct v4l2_fh
// @ev: pointer to &struct v4l2_event
//
// The event will be queued only for the specified &struct v4l2_fh file handler.
//
// .. note::
// The driver's only responsibility is to fill in the type and the data
// fields. The other fields will be filled in by V4L2.
//
extern "C" {
    pub fn v4l2_event_queue_fh(fh: *mut v4l2_fh, ev: *const v4l2_event);
}
//
// v4l2_event_wake_all - Wake all filehandles.
//
// Used when unregistering a video device.
//
// @vdev: pointer to &struct video_device
//
extern "C" {
    pub fn v4l2_event_wake_all(vdev: *mut video_device);
}
//
// v4l2_event_pending - Check if an event is available
//
// @fh: pointer to &struct v4l2_fh
//
// Returns the number of pending events.
//
extern "C" {
    pub fn v4l2_event_pending(fh: *mut v4l2_fh) -> c_int;
}
//
// v4l2_event_subscribe - Subscribes to an event
//
// @fh: pointer to &struct v4l2_fh
// @sub: pointer to &struct v4l2_event_subscription
// @elems: size of the events queue
// @ops: pointer to &v4l2_subscribed_event_ops
//
// .. note::
//
// if @elems is zero, the framework will fill in a default value,
// with is currently 1 element.
//
// v4l2_event_unsubscribe - Unsubscribes to an event
//
// @fh: pointer to &struct v4l2_fh
// @sub: pointer to &struct v4l2_event_subscription
//
// v4l2_event_unsubscribe_all - Unsubscribes to all events
//
// @fh: pointer to &struct v4l2_fh
//
extern "C" {
    pub fn v4l2_event_unsubscribe_all(fh: *mut v4l2_fh);
}
//
// v4l2_event_subdev_unsubscribe - Subdev variant of v4l2_event_unsubscribe()
//
// @sd: pointer to &struct v4l2_subdev
// @fh: pointer to &struct v4l2_fh
// @sub: pointer to &struct v4l2_event_subscription
//
// .. note::
//
// This function should be used for the &struct v4l2_subdev_core_ops
// %unsubscribe_event field.
//
// v4l2_src_change_event_subscribe - helper function that calls
// v4l2_event_subscribe() if the event is %V4L2_EVENT_SOURCE_CHANGE.
//
// @fh: pointer to struct v4l2_fh
// @sub: pointer to &struct v4l2_event_subscription
//
// v4l2_src_change_event_subdev_subscribe - Variant of v4l2_event_subscribe(),
// meant to subscribe only events of the type %V4L2_EVENT_SOURCE_CHANGE.
//
// @sd: pointer to &struct v4l2_subdev
// @fh: pointer to &struct v4l2_fh
// @sub: pointer to &struct v4l2_event_subscription
//
