//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/renesas/vsp1/vsp1_entity.h
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
// vsp1_entity.h  --  R-Car VSP1 Base Entity
//
// Copyright (C) 2013-2014 Renesas Electronics Corporation
//
// Contact: Laurent Pinchart (laurent.pinchart@ideasonboard.com)
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vsp1_entity_type {
    VSP1_ENTITY_BRS,
    VSP1_ENTITY_BRU,
    VSP1_ENTITY_CLU,
    VSP1_ENTITY_HGO,
    VSP1_ENTITY_HGT,
    VSP1_ENTITY_HSI,
    VSP1_ENTITY_HST,
    VSP1_ENTITY_IIF,
    VSP1_ENTITY_LIF,
    VSP1_ENTITY_LUT,
    VSP1_ENTITY_RPF,
    VSP1_ENTITY_SRU,
    VSP1_ENTITY_UDS,
    VSP1_ENTITY_UIF,
    VSP1_ENTITY_WPF,
}

//
// struct vsp1_route - Entity routing configuration
// @type: Entity type this routing entry is associated with
// @index: Entity index this routing entry is associated with
// @reg: Output routing configuration register
// @inputs: Target node value for each input
// @output: Target node value for entity output
//
// Each $vsp1_route entry describes routing configuration for the entity
// specified by the entry's @type and @index. @reg indicates the register that
// holds output routing configuration for the entity, and the @inputs array
// store the target node value for each input of the entity. The @output field
// stores the target node value of the entity output when used as a source for
// histogram generation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsp1_route {
    pub type: vsp1_entity_type,
    pub index: c_uint,
    pub reg: c_uint,
    pub inputs: [c_uint; VSP1_ENTITY_MAX_INPUTS],
    pub output: c_uint,
}

//
// struct vsp1_entity_operations - Entity operations
// @destroy:	Destroy the entity.
// @configure_stream:	Setup the hardware parameters for the stream which do
// not vary between frames (pipeline, formats). Note that
// the vsp1_dl_list argument is only valid for display
// pipeline and will be NULL for mem-to-mem pipelines.
// @configure_frame:	Configure the runtime parameters for each frame.
// @configure_partition: Configure partition specific parameters.
// @max_width:	Return the max supported width of data that the entity can
// process in a single operation.
// @partition:	Process the partition construction based on this entity's
// configuration.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsp1_entity_operations {
    pub entity): *mut *mut void (destroy)(struct vsp1_entity,
    pub dlb): *mut vsp1_dl_body,
    pub dlb): *mut vsp1_dl_body,
    pub dlb): *mut vsp1_dl_body,
    pub pipe): *mut vsp1_pipeline,
    pub window): *mut v4l2_rect,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsp1_entity {
    pub vsp1: *mut vsp1_device,
    pub ops: *const vsp1_entity_operations,
    pub type: vsp1_entity_type,
    pub index: c_uint,
    pub route: *const vsp1_route,
    pub codes: *const u32,
    pub num_codes: c_uint,
    pub min_width: c_uint,
    pub min_height: c_uint,
    pub max_width: c_uint,
    pub max_height: c_uint,
    pub pipe: *mut vsp1_pipeline,
    pub list_dev: list_head,
    pub list_pipe: list_head,
    pub pads: *mut media_pad,
    pub source_pad: c_uint,
    pub sources: *mut vsp1_entity,
    pub sink: *mut vsp1_entity,
    pub sink_pad: c_uint,
    pub subdev: v4l2_subdev,
    pub state: *mut v4l2_subdev_state,
    pub /: *mut *mut mutex lock; / Protects the state,
}

extern "C" {
    pub fn container_of(_arg: subdev, vsp1_entity: struct, _arg: subdev) -> return;
}
extern "C" {
    pub fn vsp1_entity_destroy(entity: *mut vsp1_entity);
}
extern "C" {
    pub fn vsp1_entity_adjust_color_space(format: *mut v4l2_mbus_framefmt);
}
