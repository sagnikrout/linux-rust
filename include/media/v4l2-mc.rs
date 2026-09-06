//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/v4l2-mc.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// v4l2-mc.h - Media Controller V4L2 types and prototypes
//
// Copyright (C) 2016 Mauro Carvalho Chehab <mchehab@kernel.org>
// Copyright (C) 2006-2010 Nokia Corporation
// Copyright (c) 2016 Intel Corporation.
//

// We don't need to include pci.h or usb.h here

//
// v4l2_mc_create_media_graph() - create Media Controller links at the graph.
//
// @mdev:	pointer to the &media_device struct.
//
// Add links between the entities commonly found on PC customer's hardware at
// the V4L2 side: camera sensors, audio and video PLL-IF decoders, tuners,
// analog TV decoder and I/O entities (video, VBI and Software Defined Radio).
//
// .. note::
//
// Webcams are modelled on a very simple way: the sensor is
// connected directly to the I/O entity. All dirty details, like
// scaler and crop HW are hidden. While such mapping is enough for v4l2
// interface centric PC-consumer's hardware, V4L2 subdev centric camera
// hardware should not use this routine, as it will not build the right graph.
//
extern "C" {
    pub fn v4l2_mc_create_media_graph(mdev: *mut media_device) -> c_int;
}
//
// v4l_enable_media_source() -	Hold media source for exclusive use
// if free
//
// @vdev:	pointer to struct video_device
//
// This interface calls enable_source handler to determine if
// media source is free for use. The enable_source handler is
// responsible for checking is the media source is free and
// start a pipeline between the media source and the media
// entity associated with the video device. This interface
// should be called from v4l2-core and dvb-core interfaces
// that change the source configuration.
//
// Return: returns zero on success or a negative error code.
//
extern "C" {
    pub fn v4l_enable_media_source(vdev: *mut video_device) -> c_int;
}
//
// v4l_disable_media_source() -	Release media source
//
// @vdev:	pointer to struct video_device
//
// This interface calls disable_source handler to release
// the media source. The disable_source handler stops the
// active media pipeline between the media source and the
// media entity associated with the video device.
//
// Return: returns zero on success or a negative error code.
//
extern "C" {
    pub fn v4l_disable_media_source(vdev: *mut video_device);
}
//
// v4l_vb2q_enable_media_tuner -  Hold media source for exclusive use
// if free.
// @q - pointer to struct vb2_queue
//
// Wrapper for v4l_enable_media_source(). This function should
// be called from v4l2-core to enable the media source with
// pointer to struct vb2_queue as the input argument. Some
// v4l2-core interfaces don't have access to video device and
// this interface finds the struct video_device for the q and
// calls v4l_enable_media_source().
//
extern "C" {
    pub fn v4l_vb2q_enable_media_source(q: *mut vb2_queue) -> c_int;
}
//
// v4l2_create_fwnode_links_to_pad - Create fwnode-based links from a
// source subdev to a sink pad.
//
// @src_sd: pointer to a source subdev
// @sink:  pointer to a sink pad
// @flags: the link flags
//
// This function searches for fwnode endpoint connections from a source
// subdevice to a single sink pad, and if suitable connections are found,
// translates them into media links to that pad. The function can be
// called by the sink, in its v4l2-async notifier bound callback, to create
// links from a bound source subdevice.
//
// The @flags argument specifies the link flags. The caller shall ensure that
// the flags are valid regardless of the number of links that may be created.
// For instance, setting the MEDIA_LNK_FL_ENABLED flag will cause all created
// links to be enabled, which isn't valid if more than one link is created.
//
// .. note::
//
// Any sink subdevice that calls this function must implement the
// .get_fwnode_pad media operation in order to verify endpoints passed
// to the sink are owned by the sink.
//
// Return 0 on success or a negative error code on failure.
//
// v4l2_create_fwnode_links - Create fwnode-based links from a source
// subdev to a sink subdev.
//
// @src_sd: pointer to a source subdevice
// @sink_sd: pointer to a sink subdevice
//
// This function searches for any and all fwnode endpoint connections
// between source and sink subdevices, and translates them into media
// links. The function can be called by the sink subdevice, in its
// v4l2-async notifier subdev bound callback, to create all links from
// a bound source subdevice.
//
// .. note::
//
// Any sink subdevice that calls this function must implement the
// .get_fwnode_pad media operation in order to verify endpoints passed
// to the sink are owned by the sink.
//
// Return 0 on success or a negative error code on failure.
//
// v4l2_pipeline_pm_get - Increase the use count of a pipeline
// @entity: The root entity of a pipeline
//
// THIS FUNCTION IS DEPRECATED. DO NOT USE IN NEW DRIVERS. USE RUNTIME PM
// ON SUB-DEVICE DRIVERS INSTEAD.
//
// Update the use count of all entities in the pipeline and power entities on.
//
// This function is intended to be called in video node open. It uses
// struct media_entity.use_count to track the power status. The use
// of this function should be paired with v4l2_pipeline_link_notify().
//
// Return 0 on success or a negative error code on failure.
//
extern "C" {
    pub fn v4l2_pipeline_pm_get(entity: *mut media_entity) -> c_int;
}
//
// v4l2_pipeline_pm_put - Decrease the use count of a pipeline
// @entity: The root entity of a pipeline
//
// THIS FUNCTION IS DEPRECATED. DO NOT USE IN NEW DRIVERS. USE RUNTIME PM
// ON SUB-DEVICE DRIVERS INSTEAD.
//
// Update the use count of all entities in the pipeline and power entities off.
//
// This function is intended to be called in video node release. It uses
// struct media_entity.use_count to track the power status. The use
// of this function should be paired with v4l2_pipeline_link_notify().
//
extern "C" {
    pub fn v4l2_pipeline_pm_put(entity: *mut media_entity);
}
//
// v4l2_pipeline_link_notify - Link management notification callback
// @link: The link
// @flags: New link flags that will be applied
// @notification: The link's state change notification type (MEDIA_DEV_NOTIFY_*)
//
// THIS FUNCTION IS DEPRECATED. DO NOT USE IN NEW DRIVERS. USE RUNTIME PM
// ON SUB-DEVICE DRIVERS INSTEAD.
//
// React to link management on powered pipelines by updating the use count of
// all entities in the source and sink sides of the link. Entities are powered
// on or off accordingly. The use of this function should be paired
// with v4l2_pipeline_pm_{get,put}().
//
// Return 0 on success or a negative error code on failure. Powering entities
// off is assumed to never fail. This function will not fail for disconnection
// events.
//

