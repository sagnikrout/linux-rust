//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/inc/resource.h
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
// Copyright 2015 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: AMD
//

pub const MEMORY_TYPE_MULTIPLIER_CZ: c_int = 4;
pub const MEMORY_TYPE_HBM: c_int = 2;
pub const MAX_MCACHES: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct resource_caps {
    pub num_timing_generator: c_int,
    pub num_opp: c_int,
    pub num_dpp: c_int,
    pub num_video_plane: c_int,
    pub num_audio: c_int,
    pub num_stream_encoder: c_int,
    pub num_analog_stream_encoder: c_int,
    pub num_pll: c_int,
    pub num_dwb: c_int,
    pub num_ddc: c_int,
    pub num_vmid: c_int,
    pub num_dsc: c_int,
    pub Input/Output).: unsigned int num_dig_link_enc; // Total number of DIGs (digital encoders) in DIO (Display,
    pub Adapters).: unsigned int num_usb4_dpia; // Total number of USB4 DPIA (DisplayPort Input,
    pub num_hpo_frl: c_int,
    pub num_hpo_dp_stream_encoder: c_int,
    pub num_hpo_dp_link_encoder: c_int,
    pub num_mpc_3dlut: c_int,
    pub num_mpc: c_int,
    pub num_rmcm: c_int,
    pub num_aux: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct resource_straps {
    pub hdmi_disable: u32,
    pub dc_pinstraps_audio: u32,
    pub audio_stream_number: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_mcache_allocations {
    pub 1]: int global_mcache_ids_plane0[MAX_MCACHES +,
    pub 1]: int global_mcache_ids_plane1[MAX_MCACHES +,
    pub 1]: int global_mcache_ids_mall_plane0[MAX_MCACHES +,
    pub 1]: int global_mcache_ids_mall_plane1[MAX_MCACHES +,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct resource_create_funcs {
    pub straps): *mut *mut dc_context ctx, resource_straps,
    pub inst): *mut *mut dc_context ctx, unsigned int,
    pub ctx): *mut engine_id eng_id, struct dc_context,
    pub ctx): *mut engine_id eng_id, struct dc_context,
    pub ctx): *mut engine_id eng_id, struct dc_context,
    pub ctx): *mut engine_id eng_id, struct dc_context,
    pub ctx): *mut dc_context,
    pub ctx): *mut dc_context,
}

extern "C" {
    pub fn dc_destroy_resource_pool(dc: *mut dc);
}
extern "C" {
    pub fn resource_is_upsp_required(format: surface_pixel_format) -> upsp_mode;
}
extern "C" {
    pub fn resource_build_scaling_params(pipe_ctx: *mut pipe_ctx) -> bool;
}
extern "C" {
    pub fn resource_build_info_frame(pipe_ctx: *mut pipe_ctx);
}
extern "C" {
    pub fn resource_can_pipe_disable_cursor(pipe_ctx: *mut pipe_ctx) -> bool;
}

//
// pipe types are identified based on MUXes in DCN front end that are capable
// of taking input from one DCN pipeline to another DCN pipeline. The name is
// in a form of XXXX_YYYY, where XXXX is the DCN front end hardware block the
// pipeline ends with and YYYY is the rendering role that the pipe is in.
//
// For instance OTG_MASTER is a pipe ending with OTG hardware block in its
// pipeline and it is in a role of a master pipe for timing generation.
//
// For quick reference a diagram of each pipe type's areas of responsibility
// for outputting timings on the screen is shown below:
//
// Timing Active for Stream 0
// __________________________________________________
// |OTG master 0 (OPP head 0)|OPP head 2 (DPP pipe 2) |
// |             (DPP pipe 0)|                        |
// | Top Plane 0             |                        |
// |           ______________|____                    |
// |          |DPP pipe 1    |DPP |                   |
// |          |              |pipe|                   |
// |          |  Bottom      |3   |                   |
// |          |  Plane 1     |    |                   |
// |          |              |    |                   |
// |          |______________|____|                   |
// |                         |                        |
// | ODM slice 0             | ODM slice 1            |
// |_________________________|________________________|
//
// Timing Active for Stream 1
// __________________________________________________
// |OTG master 4 (OPP head 4)                         |
// |                                                  |
// |               Blank Pixel Data                   |
// |              (generated by DPG4)                 |
// |                                                  |
// |__________________________________________________|
//
// Inter-pipe Relation
// __________________________________________________
// |PIPE IDX|   DPP PIPES   | OPP HEADS | OTG MASTER  |
// |        |  plane 0      | slice 0   |             |
// |   0    | -------------MPC---------ODM----------- |
// |        |  plane 1    | |         | |             |
// |   1    | ------------- |         | |             |
// |        |  plane 0      | slice 1 | |             |
// |   2    | -------------MPC--------- |             |
// |        |  plane 1    | |           |             |
// |   3    | ------------- |           |             |
// |        |               | blank     |             |
// |   4    |               | ----------------------- |
// |        |               |           |             |
// |   5    |  (FREE)       |           |             |
// |________|_______________|___________|_____________|
//
// The following is a quick reference of the class relation:
//
// DC state            ---1--------0..N---           streams
//
// stream              ---1-----------1---           OTG Master pipe
//
// OTG Master pipe     ---1--------1..N---           OPP Head pipes
//
// OPP Head pipe       ---1--------0..N---           DPP pipes
//
// stream              ---1--------0..N---           Planes
//
// Plane               ---1--------1..N---           DPP pipes
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pipe_type {
// free pipe - free pipe is an uninitialized pipe without a stream
// associated with it. It is a free DCN pipe resource. It can be
// acquired as any type of pipe.
//
    FREE_PIPE,

// OTG master pipe - the master pipe of its OPP head pipes with a
// functional OTG. It merges all its OPP head pipes pixel data in ODM
// block and output to back end DIG. OTG master pipe is responsible for
// generating entire CRTC timing to back end DIG. An OTG master pipe may
// or may not have a plane. If it has a plane it blends it as the left
// most MPC slice of the top most layer. If it doesn't have a plane it
// can output pixel data from its OPP head pipes' test pattern
// generators (DPG) such as solid black pixel data to blank the screen.
//
    OTG_MASTER,

// OPP head pipe - the head pipe of an MPC blending tree with a
// functional OPP outputting to an OTG. OPP head pipe is responsible for
// processing output pixels in its own ODM slice. It may or may not have
// a plane. If it has a plane it blends it as the top most layer within
// its own ODM slice. If it doesn't have a plane it can output pixel
// data from its DPG such as solid black pixel data to blank the pixel
// data in its own ODM slice. OTG master pipe is also an OPP head pipe
// but with more responsibility.
//
    OPP_HEAD,

// DPP pipe - the pipe with a functional DPP outputting to an OPP head
// pipe's MPC. DPP pipe is responsible for processing pixel data from
// its own MPC slice of a plane. It must be connected to an OPP head
// pipe and it must have a plane associated with it.
//
    DPP_PIPE,
}

//
// Determine if the input pipe_ctx is of a pipe type.
// return - true if pipe_ctx is of the input type.
//
extern "C" {
    pub fn resource_is_pipe_type(pipe_ctx: *const pipe_ctx, type: pipe_type) -> bool;
}
//
// Acquire a pipe as OTG master pipe and allocate pipe resources required to
// enable stream output.
//
// Release pipe resources and the OTG master pipe associated with the stream
// The stream must have all planes removed and ODM/MPC slice counts are reset
// to 1 before invoking this interface.
//
// Add plane to the bottom most layer in plane composition and allocate DPP pipe
// resources as needed.
// return - true if plane is added in plane composition, false otherwise.
//
// Add plane to the bottom most layer in plane composition and allocate DPP pipe
// resources as needed.
// return - true if plane is added in plane composition, false otherwise.
//
// Update ODM slice count by acquiring or releasing pipes. If new slices need
// to be added, it is going to add them to the last ODM index. If existing
// slices need to be removed, it is going to remove them from the last ODM
// index.
//
// return - true if ODM slices are updated and required pipes are acquired. All
// affected pipe parameters are updated.
//
// false if resource fails to complete this update. The function is not designed
// to recover the creation of invalid topologies. Returning false is typically
// an indication of insufficient validation in caller's stack. new_ctx will be
// invalid. Caller may attempt to restore new_ctx by calling this function
// again with original slice count.
//
// Update MPC slice count by acquiring or releasing DPP pipes. If new slices
// need to be added it is going to add to the last MPC index. If existing
// slices need to be removed, it is going to remove them from the last MPC
// index.
//
// @dpp_pipe - top most dpp pipe for MPCC combine.
//
// return - true if MPC slices are updated and required pipes are acquired. All
// affected pipe parameters are updated.
//
// false if resource fails to complete this update. The function is not designed
// to recover the creation of invalid topologies. Returning false is typically
// an indication of insufficient validation in caller's stack. new_ctx will be
// invalid. Caller may attempt to restore new_ctx by calling this function
// again with original slice count.
//
// Get the OTG master pipe in resource context associated with the stream.
// return - NULL if not found. Otherwise the OTG master pipe associated with the
// stream.
//
// Get an array of OPP heads in opp_heads ordered with index low to high for OTG
// master pipe in res_ctx.
// return - number of OPP heads in the array. If otg_master passed in is not
// an OTG master, the function returns 0.
//
// Get an array of DPP pipes in dpp_pipes ordered with index low to high for OPP
// head pipe in res_ctx.
// return - number of DPP pipes in the array. If opp_head passed in is not
// an OPP pipe, the function returns 0.
//
// Get an array of DPP pipes in dpp_pipes ordered with index low to high for
// plane in res_ctx.
// return - number of DPP pipes in the array.
//
// Get the OTG master pipe for the input pipe context.
// return - the OTG master pipe for the input pipe
// context.
//
// Get the OPP head pipe for the input pipe context.
// return - the OPP head pipe for the input pipe
// context.
//
// Get the DPP pipe allocated for MPC slice 0 and ODM slice 0 of the plane
// associated with dpp_pipe.
//
// Get the MPC slice index counting from 0 from left most slice
// For example, if a DPP pipe is used as a secondary pipe in MPCC combine, MPC
// split index is greater than 0.
//
extern "C" {
    pub fn resource_get_mpc_slice_index(dpp_pipe: *const pipe_ctx) -> c_int;
}
//
// Get the number of MPC slices associated with the pipe.
// The function returns 0 if the pipe is not associated with an MPC combine
// pipe topology.
//
extern "C" {
    pub fn resource_get_mpc_slice_count(pipe: *const pipe_ctx) -> c_int;
}
//
// Get the number of ODM slices associated with the pipe.
// The function returns 0 if the pipe is not associated with an ODM combine
// pipe topology.
//
extern "C" {
    pub fn resource_get_odm_slice_count(pipe: *const pipe_ctx) -> c_int;
}
// Get the ODM slice index counting from 0 from left most slice
extern "C" {
    pub fn resource_get_odm_slice_index(opp_head: *const pipe_ctx) -> c_int;
}
// Get ODM slice source rect in timing active as input to OPP block
extern "C" {
    pub fn resource_get_odm_slice_src_rect(pipe_ctx: *mut pipe_ctx) -> rect;
}
// Get ODM slice destination rect in timing active as output from OPP block
extern "C" {
    pub fn resource_get_odm_slice_dst_rect(pipe_ctx: *mut pipe_ctx) -> rect;
}
// Get ODM slice destination width in timing active as output from OPP block
// determine if pipe topology is changed between state a and state b
//
// determine if the two OTG master pipes have the same ODM topology
// return
// false - if pipes passed in are not OTG masters or ODM topology is
// changed.
// true - otherwise
//
// log the pipe topology update in state
extern "C" {
    pub fn resource_log_pipe_topology_update(dc: *mut dc, state: *mut dc_state);
}
//
// Look for a free pipe in new resource context that is used as a secondary OPP
// head by cur_otg_master.
//
// return - FREE_PIPE_INDEX_NOT_FOUND if free pipe is not found, otherwise
// pipe idx of the free pipe
//
// Look for a free pipe in new resource context that is used as a secondary DPP
// pipe in MPC blending tree associated with input OPP head pipe.
//
// return - FREE_PIPE_INDEX_NOT_FOUND if free pipe is not found, otherwise
// pipe idx of the free pipe
//
// Look for a free pipe in new resource context that is not used in current
// resource context.
//
// return - FREE_PIPE_INDEX_NOT_FOUND if free pipe is not found, otherwise
// pipe idx of the free pipe
//
// Look for a free pipe in new resource context that is used in current resource
// context as an OTG master pipe.
//
// return - FREE_PIPE_INDEX_NOT_FOUND if free pipe is not found, otherwise
// pipe idx of the free pipe
//
// Look for a free pipe in new resource context that is used as a secondary DPP
// pipe in current resource context.
// return - FREE_PIPE_INDEX_NOT_FOUND if free pipe is not found, otherwise
// pipe idx of the free pipe
//
// Look for a free pipe in new resource context that is used as a secondary DPP
// pipe in any MPCC combine in current resource context.
// return - FREE_PIPE_INDEX_NOT_FOUND if free pipe is not found, otherwise
// pipe idx of the free pipe
//
// Look for any free pipe in new resource context.
// return - FREE_PIPE_INDEX_NOT_FOUND if free pipe is not found, otherwise
// pipe idx of the free pipe
//
// Legacy find free secondary pipe logic deprecated for newer DCNs as it doesn't
// find the most optimal free pipe to prevent from time consuming hardware state
// transitions.
//
extern "C" {
    pub fn resource_pixel_format_to_bpp(format: surface_pixel_format) -> c_uint;
}
extern "C" {
    pub fn resource_transmitter_to_phy_idx(dc: *const dc, transmitter: transmitter) -> u8;
}
extern "C" {
    pub fn is_h_timing_divisible_by_2(stream: *mut dc_stream_state) -> bool;
}
// A test harness interface that modifies dp encoder resources in the given dc
// state and bypasses the need to revalidate. The interface assumes that the
// test harness interface is called with pre-validated link config stored in the
// pipe_ctx and updates dp encoder resources according to the link config.
//
// Get hw programming parameters container from pipe context
// @pipe_ctx: pipe context
// @dscl_prog_data: struct to hold programmable hw reg values
//
// Setup dc callbacks for dml2
// @dc: the display core structure
// @dml2_options: struct to hold callbacks
//
extern "C" {
    pub fn resource_init_common_dml2_callbacks(dc: *mut dc, dml2_options: *mut dml2_configuration_options);
}
//
// Calculate total DET allocated for all pipes for a given OTG_MASTER pipe
//
extern "C" {
    pub fn resource_calculate_det_for_stream(state: *mut dc_state, otg_master: *mut pipe_ctx) -> c_int;
}
extern "C" {
    pub fn resource_is_hpo_acquired(context: *mut dc_state) -> bool;
}
