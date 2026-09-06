//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/vmwgfx/vmwgfx_binding.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Copyright 2015 VMware, Inc., Palo Alto, CA., USA
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sub license, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
// THE COPYRIGHT HOLDERS, AUTHORS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
// OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
// USE OR OTHER DEALINGS IN THE SOFTWARE.
//

pub const VMW_MAX_VIEW_BINDINGS: c_int = 128;
pub const VMW_MAX_UAV_BIND_TYPE: c_int = 2;
//
// enum vmw_ctx_binding_type - abstract resource to context binding types
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vmw_ctx_binding_type {
    vmw_ctx_binding_shader,
    vmw_ctx_binding_rt,
    vmw_ctx_binding_tex,
    vmw_ctx_binding_cb,
    vmw_ctx_binding_dx_shader,
    vmw_ctx_binding_dx_rt,
    vmw_ctx_binding_sr,
    vmw_ctx_binding_ds,
    vmw_ctx_binding_so_target,
    vmw_ctx_binding_vb,
    vmw_ctx_binding_ib,
    vmw_ctx_binding_uav,
    vmw_ctx_binding_cs_uav,
    vmw_ctx_binding_so,
    vmw_ctx_binding_max
}

//
// struct vmw_ctx_bindinfo - single binding metadata
//
// @ctx_list: List head for the context's list of bindings.
// @res_list: List head for a resource's list of bindings.
// @ctx: Non-refcounted pointer to the context that owns the binding. NULL
// indicates no binding present.
// @res: Non-refcounted pointer to the resource the binding points to. This
// is typically a surface or a view.
// @bt: Binding type.
// @scrubbed: Whether the binding has been scrubbed from the context.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_ctx_bindinfo {
    pub ctx_list: list_head,
    pub res_list: list_head,
    pub ctx: *mut vmw_resource,
    pub res: *mut vmw_resource,
    pub bt: vmw_ctx_binding_type,
    pub scrubbed: bool,
}

//
// struct vmw_ctx_bindinfo_tex - texture stage binding metadata
//
// @bi: struct vmw_ctx_bindinfo we derive from.
// @texture_stage: Device data used to reconstruct binding command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_ctx_bindinfo_tex {
    pub bi: vmw_ctx_bindinfo,
    pub texture_stage: uint32,
}

//
// struct vmw_ctx_bindinfo_shader - Shader binding metadata
//
// @bi: struct vmw_ctx_bindinfo we derive from.
// @shader_slot: Device data used to reconstruct binding command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_ctx_bindinfo_shader {
    pub bi: vmw_ctx_bindinfo,
    pub shader_slot: SVGA3dShaderType,
}

//
// struct vmw_ctx_bindinfo_cb - Constant buffer binding metadata
//
// @bi: struct vmw_ctx_bindinfo we derive from.
// @shader_slot: Device data used to reconstruct binding command.
// @offset: Device data used to reconstruct binding command.
// @size: Device data used to reconstruct binding command.
// @slot: Device data used to reconstruct binding command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_ctx_bindinfo_cb {
    pub bi: vmw_ctx_bindinfo,
    pub shader_slot: SVGA3dShaderType,
    pub offset: uint32,
    pub size: uint32,
    pub slot: uint32,
}

//
// struct vmw_ctx_bindinfo_view - View binding metadata
//
// @bi: struct vmw_ctx_bindinfo we derive from.
// @shader_slot: Device data used to reconstruct binding command.
// @slot: Device data used to reconstruct binding command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_ctx_bindinfo_view {
    pub bi: vmw_ctx_bindinfo,
    pub shader_slot: SVGA3dShaderType,
    pub slot: uint32,
}

//
// struct vmw_ctx_bindinfo_so_target - StreamOutput binding metadata
//
// @bi: struct vmw_ctx_bindinfo we derive from.
// @offset: Device data used to reconstruct binding command.
// @size: Device data used to reconstruct binding command.
// @slot: Device data used to reconstruct binding command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_ctx_bindinfo_so_target {
    pub bi: vmw_ctx_bindinfo,
    pub offset: uint32,
    pub size: uint32,
    pub slot: uint32,
}

//
// struct vmw_ctx_bindinfo_vb - Vertex buffer binding metadata
//
// @bi: struct vmw_ctx_bindinfo we derive from.
// @offset: Device data used to reconstruct binding command.
// @stride: Device data used to reconstruct binding command.
// @slot: Device data used to reconstruct binding command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_ctx_bindinfo_vb {
    pub bi: vmw_ctx_bindinfo,
    pub offset: uint32,
    pub stride: uint32,
    pub slot: uint32,
}

//
// struct vmw_ctx_bindinfo_ib - StreamOutput binding metadata
//
// @bi: struct vmw_ctx_bindinfo we derive from.
// @offset: Device data used to reconstruct binding command.
// @format: Device data used to reconstruct binding command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_ctx_bindinfo_ib {
    pub bi: vmw_ctx_bindinfo,
    pub offset: uint32,
    pub format: uint32,
}

//
// struct vmw_dx_shader_bindings - per shader type context binding state
//
// @shader: The shader binding for this shader type
// @const_buffer: Const buffer bindings for this shader type.
// @shader_res: Shader resource view bindings for this shader type.
// @dirty_sr: Bitmap tracking individual shader resource bindings changes
// that have not yet been emitted to the device.
// @dirty: Bitmap tracking per-binding type binding changes that have not
// yet been emitted to the device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_dx_shader_bindings {
    pub shader: vmw_ctx_bindinfo_shader,
    pub const_buffers: [vmw_ctx_bindinfo_cb; SVGA3D_DX_MAX_CONSTBUFFERS],
    pub shader_res: [vmw_ctx_bindinfo_view; SVGA3D_DX_MAX_SRVIEWS],
    pub SVGA3D_DX_MAX_SRVIEWS): DECLARE_BITMAP(dirty_sr,,
    pub dirty: c_ulong,
}

//
// struct vmw_ctx_bindinfo_uav - UAV context binding state.
// @views: UAV view bindings.
// @splice_index: The device splice index set by user-space.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_ctx_bindinfo_uav {
    pub views: [vmw_ctx_bindinfo_view; SVGA3D_DX11_1_MAX_UAVIEWS],
    pub index: uint32,
}

//
// struct vmw_ctx_bindinfo_so - Stream output binding metadata.
// @bi: struct vmw_ctx_bindinfo we derive from.
// @slot: Device data used to reconstruct binding command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_ctx_bindinfo_so {
    pub bi: vmw_ctx_bindinfo,
    pub slot: uint32,
}

extern "C" {
    pub fn vmw_binding_res_list_kill(head: *mut list_head);
}
extern "C" {
    pub fn vmw_binding_res_list_scrub(head: *mut list_head);
}
extern "C" {
    pub fn vmw_binding_rebind_all(cbs: *mut vmw_ctx_binding_state) -> c_int;
}
extern "C" {
    pub fn vmw_binding_state_kill(cbs: *mut vmw_ctx_binding_state);
}
extern "C" {
    pub fn vmw_binding_state_scrub(cbs: *mut vmw_ctx_binding_state);
}
extern "C" {
    pub fn vmw_binding_state_free(cbs: *mut vmw_ctx_binding_state);
}
extern "C" {
    pub fn vmw_binding_state_reset(cbs: *mut vmw_ctx_binding_state);
}
extern "C" {
    pub fn vmw_binding_dirtying(binding_type: vmw_ctx_binding_type) -> u32;
}
