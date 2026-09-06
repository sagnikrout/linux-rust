//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/vmwgfx/device_include/svga3d_dx.h
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
// Copyright 2012-2021 VMware, Inc.
//
// Permission is hereby granted, free of charge, to any person
// obtaining a copy of this software and associated documentation
// files (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy,
// modify, merge, publish, distribute, sublicense, and/or sell copies
// of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
// svga3d_dx.h --
//
// SVGA 3d hardware definitions for DX10 support.
//

pub const SVGA3D_INPUT_MIN: c_int = 0;
pub const SVGA3D_INPUT_PER_VERTEX_DATA: c_int = 0;
pub const SVGA3D_INPUT_PER_INSTANCE_DATA: c_int = 1;
pub const SVGA3D_INPUT_MAX: c_int = 2;
pub type SVGA3dInputClassification = uint32;

pub type SVGA3dColorWriteEnable = uint8;
pub const SVGA3D_DEPTH_WRITE_MASK_ZERO: c_int = 0;
pub const SVGA3D_DEPTH_WRITE_MASK_ALL: c_int = 1;
pub type SVGA3dDepthWriteMask = uint8;

pub type SVGA3dFilter = uint32;
pub const SVGA3D_CULL_INVALID: c_int = 0;
pub const SVGA3D_CULL_MIN: c_int = 1;
pub const SVGA3D_CULL_NONE: c_int = 1;
pub const SVGA3D_CULL_FRONT: c_int = 2;
pub const SVGA3D_CULL_BACK: c_int = 3;
pub const SVGA3D_CULL_MAX: c_int = 4;
pub type SVGA3dCullMode = uint8;
pub const SVGA3D_COMPARISON_INVALID: c_int = 0;
pub const SVGA3D_COMPARISON_MIN: c_int = 1;
pub const SVGA3D_COMPARISON_NEVER: c_int = 1;
pub const SVGA3D_COMPARISON_LESS: c_int = 2;
pub const SVGA3D_COMPARISON_EQUAL: c_int = 3;
pub const SVGA3D_COMPARISON_LESS_EQUAL: c_int = 4;
pub const SVGA3D_COMPARISON_GREATER: c_int = 5;
pub const SVGA3D_COMPARISON_NOT_EQUAL: c_int = 6;
pub const SVGA3D_COMPARISON_GREATER_EQUAL: c_int = 7;
pub const SVGA3D_COMPARISON_ALWAYS: c_int = 8;
pub const SVGA3D_COMPARISON_MAX: c_int = 9;
pub type SVGA3dComparisonFunc = uint8;
pub const SVGA3D_MULTISAMPLE_RAST_DISABLE: c_int = 0;
pub const SVGA3D_MULTISAMPLE_RAST_ENABLE: c_int = 1;
pub const SVGA3D_MULTISAMPLE_RAST_DX_MAX: c_int = 1;
pub const SVGA3D_MULTISAMPLE_RAST_DISABLE_LINE: c_int = 2;
pub const SVGA3D_MULTISAMPLE_RAST_MAX: c_int = 2;
pub type SVGA3dMultisampleRastEnable = uint8;
pub const SVGA3D_DX_MAX_VERTEXBUFFERS: c_int = 32;
pub const SVGA3D_DX_MAX_VERTEXINPUTREGISTERS: c_int = 16;
pub const SVGA3D_DX_SM41_MAX_VERTEXINPUTREGISTERS: c_int = 32;
pub const SVGA3D_DX_MAX_SOTARGETS: c_int = 4;
pub const SVGA3D_DX_MAX_SRVIEWS: c_int = 128;
pub const SVGA3D_DX_MAX_CONSTBUFFERS: c_int = 16;
pub const SVGA3D_DX_MAX_SAMPLERS: c_int = 16;
pub const SVGA3D_DX_MAX_CLASS_INSTANCES: c_int = 253;

pub type SVGA3dShaderResourceViewId = uint32;
pub type SVGA3dRenderTargetViewId = uint32;
pub type SVGA3dDepthStencilViewId = uint32;
pub type SVGA3dUAViewId = uint32;
pub type SVGA3dShaderId = uint32;
pub type SVGA3dElementLayoutId = uint32;
pub type SVGA3dSamplerId = uint32;
pub type SVGA3dBlendStateId = uint32;
pub type SVGA3dDepthStencilStateId = uint32;
pub type SVGA3dRasterizerStateId = uint32;
pub type SVGA3dQueryId = uint32;
pub type SVGA3dStreamOutputId = uint32;

pub type SVGA3dDXQueryFlags = uint32;

pub const SVGADX_QDSTATE_MIN: c_int = 0;
pub const SVGADX_QDSTATE_IDLE: c_int = 0;
pub const SVGADX_QDSTATE_ACTIVE: c_int = 1;
pub const SVGADX_QDSTATE_PENDING: c_int = 2;
pub const SVGADX_QDSTATE_FINISHED: c_int = 3;
pub const SVGADX_QDSTATE_MAX: c_int = 4;
pub type SVGADXQueryDeviceState = uint8;

pub const SVGA3D_DX_MAX_VIEWPORTS: c_int = 16;

pub const SVGA3D_DX_MAX_SCISSORRECTS: c_int = 16;

pub type SVGA3dDXPresentBltMode = uint32;

pub type SVGA3dTransferToBufferFlags = uint32;

pub type SVGADXHintId = uint32;
pub const SVGA_DX_HINT_NONE: c_int = 0;
pub const SVGA_DX_HINT_PREFETCH_OBJECT: c_int = 1;
pub const SVGA_DX_HINT_PREEVICT_OBJECT: c_int = 2;
pub const SVGA_DX_HINT_PREFETCH_COBJECT: c_int = 3;
pub const SVGA_DX_HINT_PREEVICT_COBJECT: c_int = 4;
pub const SVGA_DX_HINT_MAX: c_int = 5;

pub type SVGA3dCmdDXSetVSConstantBufferOffset = SVGA3dCmdDXSetConstantBufferOffset;
pub type SVGA3dCmdDXSetPSConstantBufferOffset = SVGA3dCmdDXSetConstantBufferOffset;
pub type SVGA3dCmdDXSetGSConstantBufferOffset = SVGA3dCmdDXSetConstantBufferOffset;
pub type SVGA3dCmdDXSetHSConstantBufferOffset = SVGA3dCmdDXSetConstantBufferOffset;
pub type SVGA3dCmdDXSetDSConstantBufferOffset = SVGA3dCmdDXSetConstantBufferOffset;
pub type SVGA3dCmdDXSetCSConstantBufferOffset = SVGA3dCmdDXSetConstantBufferOffset;

pub type SVGA3dBufferExFlags = uint32;

pub const SVGA3D_DXDSVIEW_CREATE_READ_ONLY_DEPTH: c_uint = 0x01;
pub const SVGA3D_DXDSVIEW_CREATE_READ_ONLY_STENCIL: c_uint = 0x02;
pub const SVGA3D_DXDSVIEW_CREATE_FLAG_MASK: c_uint = 0x03;
pub type SVGA3DCreateDSViewFlags = uint8;

pub type SVGA3dUABufferFlags = uint32;

pub const SVGA3D_DX_MAX_RENDER_TARGETS: c_int = 8;

pub const SVGADX_SIGNATURE_SEMANTIC_NAME_UNDEFINED: c_int = 0;
pub const SVGADX_SIGNATURE_SEMANTIC_NAME_POSITION: c_int = 1;
pub const SVGADX_SIGNATURE_SEMANTIC_NAME_CLIP_DISTANCE: c_int = 2;
pub const SVGADX_SIGNATURE_SEMANTIC_NAME_CULL_DISTANCE: c_int = 3;
pub const SVGADX_SIGNATURE_SEMANTIC_NAME_RENDER_TARGET_ARRAY_INDEX: c_int = 4;
pub const SVGADX_SIGNATURE_SEMANTIC_NAME_VIEWPORT_ARRAY_INDEX: c_int = 5;
pub const SVGADX_SIGNATURE_SEMANTIC_NAME_VERTEX_ID: c_int = 6;
pub const SVGADX_SIGNATURE_SEMANTIC_NAME_PRIMITIVE_ID: c_int = 7;
pub const SVGADX_SIGNATURE_SEMANTIC_NAME_INSTANCE_ID: c_int = 8;
pub const SVGADX_SIGNATURE_SEMANTIC_NAME_IS_FRONT_FACE: c_int = 9;
pub const SVGADX_SIGNATURE_SEMANTIC_NAME_SAMPLE_INDEX: c_int = 10;
pub const SVGADX_SIGNATURE_SEMANTIC_NAME_FINAL_QUAD_U_EQ_0_EDGE_TESSFACTOR: c_int = 11;
pub const SVGADX_SIGNATURE_SEMANTIC_NAME_FINAL_QUAD_V_EQ_0_EDGE_TESSFACTOR: c_int = 12;
pub const SVGADX_SIGNATURE_SEMANTIC_NAME_FINAL_QUAD_U_EQ_1_EDGE_TESSFACTOR: c_int = 13;
pub const SVGADX_SIGNATURE_SEMANTIC_NAME_FINAL_QUAD_V_EQ_1_EDGE_TESSFACTOR: c_int = 14;
pub const SVGADX_SIGNATURE_SEMANTIC_NAME_FINAL_QUAD_U_INSIDE_TESSFACTOR: c_int = 15;
pub const SVGADX_SIGNATURE_SEMANTIC_NAME_FINAL_QUAD_V_INSIDE_TESSFACTOR: c_int = 16;
pub const SVGADX_SIGNATURE_SEMANTIC_NAME_FINAL_TRI_U_EQ_0_EDGE_TESSFACTOR: c_int = 17;
pub const SVGADX_SIGNATURE_SEMANTIC_NAME_FINAL_TRI_V_EQ_0_EDGE_TESSFACTOR: c_int = 18;
pub const SVGADX_SIGNATURE_SEMANTIC_NAME_FINAL_TRI_W_EQ_0_EDGE_TESSFACTOR: c_int = 19;
pub const SVGADX_SIGNATURE_SEMANTIC_NAME_FINAL_TRI_INSIDE_TESSFACTOR: c_int = 20;
pub const SVGADX_SIGNATURE_SEMANTIC_NAME_FINAL_LINE_DETAIL_TESSFACTOR: c_int = 21;
pub const SVGADX_SIGNATURE_SEMANTIC_NAME_FINAL_LINE_DENSITY_TESSFACTOR: c_int = 22;
pub const SVGADX_SIGNATURE_SEMANTIC_NAME_MAX: c_int = 23;
pub type SVGA3dDXSignatureSemanticName = uint32;
pub const SVGADX_SIGNATURE_REGISTER_COMPONENT_UNKNOWN: c_int = 0;
pub type SVGA3dDXSignatureRegisterComponentType = uint32;
pub const SVGADX_SIGNATURE_MIN_PRECISION_DEFAULT: c_int = 0;
pub type SVGA3dDXSignatureMinPrecision = uint32;

pub const SVGADX_SIGNATURE_HEADER_VERSION_0: c_uint = 0x08a92d12;

pub const SVGA3D_MAX_DX10_STREAMOUT_DECLS: c_int = 64;
pub const SVGA3D_MAX_STREAMOUT_DECLS: c_int = 512;

pub const SVGA3D_DX_SO_NO_RASTERIZED_STREAM: c_uint = 0xFFFFFFFF;

pub const SVGA3D_DX_MAX_CLASS_INSTANCES_PADDED: c_int = 256;

