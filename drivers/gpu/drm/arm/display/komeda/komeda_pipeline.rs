//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/arm/display/komeda/komeda_pipeline.h
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
// (C) COPYRIGHT 2018 ARM Limited. All rights reserved.
// Author: James.Qian.Wang <james.qian.wang@arm.com>
//

pub const KOMEDA_MAX_PIPELINES: c_int = 2;
pub const KOMEDA_PIPELINE_MAX_LAYERS: c_int = 4;
pub const KOMEDA_PIPELINE_MAX_SCALERS: c_int = 2;
pub const KOMEDA_COMPONENT_N_INPUTS: c_int = 5;
// pipeline component IDs

// komeda_component_funcs - component control functions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct komeda_component_funcs {
// @validate: optional,
// component may has special requirements or limitations, this function
// supply HW the ability to do the further HW specific check.
//
    pub state): *mut komeda_component_state,
// @update: update is a active update
    pub state): *mut komeda_component_state,
// @disable: disable component
    pub c): *mut *mut void (disable)(struct komeda_component,
// @dump_register: Optional, dump registers to seq_file
    pub seq): *mut *mut *mut void (dump_register)(struct komeda_component c, struct seq_file,
}

//
// struct komeda_component
//
// struct komeda_component describe the data flow capabilities for how to link a
// component into the display pipeline.
// all specified components are subclass of this structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct komeda_component {
// @obj: treat component as private obj
    pub obj: drm_private_obj,
// @pipeline: the komeda pipeline this component belongs to
    pub pipeline: *mut komeda_pipeline,
// @name: component name
    pub name: [c_char; 32],
//
// @reg:
// component register base,
// which is initialized by chip and used by chip only
//
    pub reg: *mut u32 __iomem,
// @id: component id
    pub id: u32,
//
// @hw_id: component hw id,
// which is initialized by chip and used by chip only
//
    pub hw_id: u32,
//
// @max_active_inputs:
// @max_active_outputs:
//
// maximum number of inputs/outputs that can be active at the same time
// Note:
// the number isn't the bit number of @supported_inputs or
// @supported_outputs, but may be less than it, since component may not
// support enabling all @supported_inputs/outputs at the same time.
//
    pub max_active_inputs: u8,
// @max_active_outputs: maximum number of outputs
    pub max_active_outputs: u8,
//
// @supported_inputs:
// @supported_outputs:
//
// bitmask of BIT(component->id) for the supported inputs/outputs,
// describes the possibilities of how a component is linked into a
// pipeline.
//
    pub supported_inputs: u32,
// @supported_outputs: bitmask of supported output componenet ids
    pub supported_outputs: u32,
//
// @funcs: chip functions to access HW
//
    pub funcs: *const komeda_component_funcs,
}

//
// struct komeda_component_output
//
// a component has multiple outputs, if want to know where the data
// comes from, only know the component is not enough, we still need to know
// its output port
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct komeda_component_output {
// @component: indicate which component the data comes from
    pub component: *mut komeda_component,
//
// @output_port:
// the output port of the &komeda_component_output.component
//
    pub output_port: u8,
}

//
// struct komeda_component_state
//
// component_state is the data flow configuration of the component, and it's
// the superclass of all specific component_state like @komeda_layer_state,
// @komeda_scaler_state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct komeda_component_state {
// @obj: tracking component_state by drm_atomic_commit
    pub obj: drm_private_state,
// @component: backpointer to the component
    pub component: *mut komeda_component,
//
// @binding_user:
// currently bound user, the user can be @crtc, @plane or @wb_conn,
// which is valid decided by @component and @inputs
//
// -  Layer: its user always is plane.
// -  compiz/improc/timing_ctrlr: the user is crtc.
// -  wb_layer: wb_conn;
// -  scaler: plane when input is layer, wb_conn if input is compiz.
//
// @crtc: backpointer for user crtc
    pub crtc: *mut drm_crtc,
// @plane: backpointer for user plane
    pub plane: *mut drm_plane,
// @wb_conn: backpointer for user wb_connector
    pub wb_conn: *mut drm_connector,
    pub binding_user: *mut c_void,
}

//
// @active_inputs:
//
// active_inputs is bitmask of @inputs index
//
// -  active_inputs = changed_active_inputs | unchanged_active_inputs
// -  affected_inputs = old->active_inputs | new->active_inputs;
// -  disabling_inputs = affected_inputs ^ active_inputs;
// -  changed_inputs = disabling_inputs | changed_active_inputs;
//
// NOTE:
// changed_inputs doesn't include all active_input but only
// @changed_active_inputs, and this bitmask can be used in chip
// level for dirty update.
//
// @changed_active_inputs: bitmask of the changed @active_inputs
// @affected_inputs: bitmask for affected @inputs
//
// @inputs:
//
// the specific inputs[i] only valid on BIT(i) has been set in
// @active_inputs, if not the inputs[i] is undefined.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct komeda_layer {
    pub base: komeda_component,
// accepted h/v input range before rotation
    pub vsize_in: malidp_range hsize_in,,
    pub /: *mut *mut u32 layer_type; / RICH, SIMPLE or WB,
    pub line_sz: u32,
    pub /: *mut *mut u32 yuv_line_sz; / maximum line size for YUV422 and YUV420,
    pub supported_rots: u32,
// komeda supports layer split which splits a whole image to two parts
// left and right and handle them by two individual layer processors
// Note: left/right are always according to the final display rect,
// not the source buffer.
//
    pub right: *mut komeda_layer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct komeda_layer_state {
    pub base: komeda_component_state,
// layer specific configuration state
    pub vsize: u16 hsize,,
    pub rot: u32,
    pub afbc_crop_l: u16,
    pub afbc_crop_r: u16,
    pub afbc_crop_t: u16,
    pub afbc_crop_b: u16,
    pub addr: [dma_addr_t; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct komeda_scaler {
    pub base: komeda_component,
    pub vsize: malidp_range hsize,,
    pub max_upscaling: u32,
    pub max_downscaling: u32,
    pub /: *mut *mut u8 scaling_split_overlap; / split overlap for scaling,
    pub /: *mut *mut u8 enh_split_overlap; / split overlap for image enhancement,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct komeda_scaler_state {
    pub base: komeda_component_state,
    pub vsize_in: u16 hsize_in,,
    pub vsize_out: u16 hsize_out,,
    pub total_vsize_in: u16 total_hsize_in,,
    pub /: *mut *mut u16 total_hsize_out; / total_xxxx are size before split,
    pub right_crop: u16 left_crop,,
    pub /: *mut *mut right_part : 1; / right part of split image,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct komeda_compiz {
    pub base: komeda_component,
    pub vsize: malidp_range hsize,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct komeda_compiz_input_cfg {
    pub vsize: u16 hsize,,
    pub voffset: u16 hoffset,,
    pub layer_alpha: u8 pixel_blend_mode,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct komeda_compiz_state {
    pub base: komeda_component_state,
// composition size
    pub vsize: u16 hsize,,
    pub cins: [komeda_compiz_input_cfg; KOMEDA_COMPONENT_N_INPUTS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct komeda_merger {
    pub base: komeda_component,
    pub hsize_merged: malidp_range,
    pub vsize_merged: malidp_range,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct komeda_merger_state {
    pub base: komeda_component_state,
    pub hsize_merged: u16,
    pub vsize_merged: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct komeda_splitter {
    pub base: komeda_component,
    pub vsize: malidp_range hsize,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct komeda_splitter_state {
    pub base: komeda_component_state,
    pub vsize: u16 hsize,,
    pub overlap: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct komeda_improc {
    pub base: komeda_component,
    pub /: *mut *mut u32 supported_color_formats; / BIT(DRM_OUTPUT_COLOR_FORMAT_RGB444/YUV444/YUV420),
    pub BIT(10)*/: *mut *mut u32 supported_color_depths; / BIT(8) |,
    pub 1: u8 supports_degamma :,
    pub 1: u8 supports_csc :,
    pub 1: u8 supports_gamma :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct komeda_improc_state {
    pub base: komeda_component_state,
    pub color_format: drm_output_color_format,
    pub color_depth: u8,
    pub vsize: u16 hsize,,
    pub fgamma_coeffs: [u32; KOMEDA_N_GAMMA_COEFFS],
    pub ctm_coeffs: [u32; KOMEDA_N_CTM_COEFFS],
}

// display timing controller
#[repr(C)]
#[derive(Copy, Clone)]
pub struct komeda_timing_ctrlr {
    pub base: komeda_component,
    pub 1: u8 supports_dual_link :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct komeda_timing_ctrlr_state {
    pub base: komeda_component_state,
}

// Why define A separated structure but not use plane_state directly ?
// 1. Komeda supports layer_split which means a plane_state can be split and
// handled by two layers, one layer only handle half of plane image.
// 2. Fix up the user properties according to HW's capabilities, like user
// set rotation to R180, but HW only supports REFLECT_X+Y. the rot here is
// after drm_rotation_simplify()
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct komeda_data_flow_cfg {
    pub input: komeda_component_output,
    pub in_h: u16 in_x, in_y, in_w,,
    pub out_h: u32 out_x, out_y, out_w,,
    pub total_in_w: u16 total_in_h,,
    pub total_out_w: u16,
    pub overlap: u16 left_crop, right_crop,,
    pub rot: u32,
    pub blending_zorder: c_int,
    pub layer_alpha: u8 pixel_blend_mode,,
    pub /: *mut *mut right_part : 1; / right part of display image if split enabled,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct komeda_pipeline_funcs {
// check if the aclk (main engine clock) can satisfy the clock
// requirements of the downscaling that specified by dflow
//
    pub dflow): *mut komeda_data_flow_cfg,
// dump_register: Optional, dump registers to seq_file
    pub sf): *mut seq_file,
}

//
// struct komeda_pipeline
//
// Represent a complete display pipeline and hold all functional components.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct komeda_pipeline {
// @obj: link pipeline as private obj of drm_atomic_commit
    pub obj: drm_private_obj,
// @mdev: the parent komeda_dev
    pub mdev: *mut komeda_dev,
// @pxlclk: pixel clock
    pub pxlclk: *mut clk,
// @id: pipeline id
    pub id: c_int,
// @avail_comps: available components mask of pipeline
    pub avail_comps: u32,
//
// @standalone_disabled_comps:
//
// When disable the pipeline, some components can not be disabled
// together with others, but need a sparated and standalone disable.
// The standalone_disabled_comps are the components which need to be
// disabled standalone, and this concept also introduce concept of
// two phase.
// phase 1: for disabling the common components.
// phase 2: for disabling the standalong_disabled_comps.
//
    pub standalone_disabled_comps: u32,
// @n_layers: the number of layer on @layers
    pub n_layers: c_int,
// @layers: the pipeline layers
    pub layers: [*mut komeda_layer; KOMEDA_PIPELINE_MAX_LAYERS],
// @n_scalers: the number of scaler on @scalers
    pub n_scalers: c_int,
// @scalers: the pipeline scalers
    pub scalers: [*mut komeda_scaler; KOMEDA_PIPELINE_MAX_SCALERS],
// @compiz: compositor
    pub compiz: *mut komeda_compiz,
// @splitter: for split the compiz output to two half data flows
    pub splitter: *mut komeda_splitter,
// @merger: merger
    pub merger: *mut komeda_merger,
// @wb_layer: writeback layer
    pub wb_layer: *mut komeda_layer,
// @improc: post image processor
    pub improc: *mut komeda_improc,
// @ctrlr: timing controller
    pub ctrlr: *mut komeda_timing_ctrlr,
// @funcs: chip private pipeline functions
    pub funcs: *const komeda_pipeline_funcs,
// @of_node: pipeline dt node
    pub of_node: *mut device_node,
// @of_output_port: pipeline output port
    pub of_output_port: *mut device_node,
// @of_output_links: output connector device nodes
    pub of_output_links: [*mut device_node; 2],
// @dual_link: true if of_output_links[0] and [1] are both valid
    pub dual_link: bool,
}

//
// struct komeda_pipeline_state
//
// NOTE:
// Unlike the pipeline, pipeline_state doesn’t gather any component_state
// into it. It because all component will be managed by drm_atomic_commit.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct komeda_pipeline_state {
// @obj: tracking pipeline_state by drm_atomic_commit
    pub obj: drm_private_state,
// @pipe: backpointer to the pipeline
    pub pipe: *mut komeda_pipeline,
// @crtc: currently bound crtc
    pub crtc: *mut drm_crtc,
//
// @active_comps:
//
// bitmask - BIT(component->id) of active components
//
    pub active_comps: u32,
}

// pipeline APIs
extern "C" {
    pub fn komeda_assemble_pipelines(mdev: *mut komeda_dev) -> c_int;
}
// component APIs
extern "C" {
    pub fn komeda_pipeline_get_first_component(_arg: c->pipeline, _arg: avail_inputs) -> return;
}
