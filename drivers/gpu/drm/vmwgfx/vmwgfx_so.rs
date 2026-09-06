//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/vmwgfx/vmwgfx_so.h
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
// Copyright 2014-2015 VMware, Inc., Palo Alto, CA., USA
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
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vmw_view_type {
    vmw_view_sr,
    vmw_view_rt,
    vmw_view_ds,
    vmw_view_ua,
    vmw_view_max,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vmw_so_type {
    vmw_so_el,
    vmw_so_bs,
    vmw_so_ds,
    vmw_so_rs,
    vmw_so_ss,
    vmw_so_so,
    vmw_so_max,
}

//
// union vmw_view_destroy - view destruction command body
//
// @rtv: RenderTarget view destruction command body
// @srv: ShaderResource view destruction command body
// @dsv: DepthStencil view destruction command body
// @view_id: A single u32 view id.
//
// The assumption here is that all union members are really represented by a
// single u32 in the command stream. If that's not the case,
// the size of this union will not equal the size of an u32, and the
// assumption is invalid, and we detect that at compile time in the
// vmw_so_build_asserts() function.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union vmw_view_destroy {
    pub rtv: SVGA3dCmdDXDestroyRenderTargetView,
    pub srv: SVGA3dCmdDXDestroyShaderResourceView,
    pub dsv: SVGA3dCmdDXDestroyDepthStencilView,
    pub uav: SVGA3dCmdDXDestroyUAView,
    pub view_id: u32,
}

// Map enum vmw_view_type to view destroy command ids
// Map enum vmw_view_type to SVGACOTableType
// Map enum vmw_so_type to SVGACOTableType
//
// vmw_view_cmd_to_type - Return the view type for a create or destroy command
//
// @id: The SVGA3D command id.
//
// For a given view create or destroy command id, return the corresponding
// enum vmw_view_type. If the command is unknown, return vmw_view_max.
// The validity of the simplified calculation is verified in the
// vmw_so_build_asserts() function.
//
// vmw_so_cmd_to_type - Return the state object type for a
// create or destroy command
//
// @id: The SVGA3D command id.
//
// For a given state object create or destroy command id,
// return the corresponding enum vmw_so_type. If the command is uknown,
// return vmw_so_max. We should perhaps optimize this function using
// a similar strategy as vmw_view_cmd_to_type().
//
// View management - vmwgfx_so.c
//
extern "C" {
    pub fn vmw_view_dirtying(res: *mut vmw_resource) -> u32;
}
