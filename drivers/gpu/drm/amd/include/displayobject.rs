//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/displayobject.h
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


// \
//
// Module Name    displayobjectsoc15.h
// Project
// Device
//
// Description    Contains the common definitions for display objects for SoC15 products.
//
// Copyright 2014 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software
// and associated documentation files (the "Software"), to deal in the Software without restriction,
// including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial
// portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//

//
// Display Object Type Definition
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum display_object_type {
    DISPLAY_OBJECT_TYPE_NONE						=0x00,
    DISPLAY_OBJECT_TYPE_GPU							=0x01,
    DISPLAY_OBJECT_TYPE_ENCODER						=0x02,
    DISPLAY_OBJECT_TYPE_CONNECTOR					=0x03
}

//
// Encorder Object Type Definition
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum encoder_object_type {
    ENCODER_OBJECT_ID_NONE							 =0x00,
    ENCODER_OBJECT_ID_INTERNAL_UNIPHY				 =0x01,
    ENCODER_OBJECT_ID_INTERNAL_UNIPHY1				 =0x02,
    ENCODER_OBJECT_ID_INTERNAL_UNIPHY2				 =0x03,
}

//
// Connector Object ID Definition
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum connector_object_type {
    CONNECTOR_OBJECT_ID_NONE						  =0x00,
    CONNECTOR_OBJECT_ID_SINGLE_LINK_DVI_D			  =0x01,
    CONNECTOR_OBJECT_ID_DUAL_LINK_DVI_D				  =0x02,
    CONNECTOR_OBJECT_ID_HDMI_TYPE_A					  =0x03,
    CONNECTOR_OBJECT_ID_LVDS						  =0x04,
    CONNECTOR_OBJECT_ID_DISPLAYPORT					  =0x05,
    CONNECTOR_OBJECT_ID_eDP							  =0x06,
    CONNECTOR_OBJECT_ID_OPM							  =0x07
}

//
// Protection Object ID Definition
//
// No need
//
// Object ENUM ID Definition
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum object_enum_id {
    OBJECT_ENUM_ID1									  =0x01,
    OBJECT_ENUM_ID2									  =0x02,
    OBJECT_ENUM_ID3									  =0x03,
    OBJECT_ENUM_ID4									  =0x04,
    OBJECT_ENUM_ID5									  =0x05,
    OBJECT_ENUM_ID6									  =0x06
}

//
// Object ID Bit definition
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum object_id_bit {
    OBJECT_ID_MASK									  =0x00FF,
    ENUM_ID_MASK									  =0x0F00,
    OBJECT_TYPE_MASK								  =0xF000,
    OBJECT_ID_SHIFT									  =0x00,
    ENUM_ID_SHIFT									  =0x08,
    OBJECT_TYPE_SHIFT								  =0x0C
}

//
// GPU Object definition - Shared with BIOS
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gpu_objet_def {
    GPU_ENUM_ID1                            =( DISPLAY_OBJECT_TYPE_GPU << OBJECT_TYPE_SHIFT | OBJECT_ENUM_ID1 << ENUM_ID_SHIFT)
}

//
// Encoder Object definition - Shared with BIOS
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum encoder_objet_def {
    ENCODER_INTERNAL_UNIPHY_ENUM_ID1         =( DISPLAY_OBJECT_TYPE_ENCODER << OBJECT_TYPE_SHIFT |\
    OBJECT_ENUM_ID1 << ENUM_ID_SHIFT |\
    ENCODER_OBJECT_ID_INTERNAL_UNIPHY << OBJECT_ID_SHIFT),

    ENCODER_INTERNAL_UNIPHY_ENUM_ID2         =( DISPLAY_OBJECT_TYPE_ENCODER << OBJECT_TYPE_SHIFT |\
    OBJECT_ENUM_ID2 << ENUM_ID_SHIFT |\
    ENCODER_OBJECT_ID_INTERNAL_UNIPHY << OBJECT_ID_SHIFT),

    ENCODER_INTERNAL_UNIPHY1_ENUM_ID1        =( DISPLAY_OBJECT_TYPE_ENCODER << OBJECT_TYPE_SHIFT |\
    OBJECT_ENUM_ID1 << ENUM_ID_SHIFT |\
    ENCODER_OBJECT_ID_INTERNAL_UNIPHY1 << OBJECT_ID_SHIFT),

    ENCODER_INTERNAL_UNIPHY1_ENUM_ID2        =( DISPLAY_OBJECT_TYPE_ENCODER << OBJECT_TYPE_SHIFT |\
    OBJECT_ENUM_ID2 << ENUM_ID_SHIFT |\
    ENCODER_OBJECT_ID_INTERNAL_UNIPHY1 << OBJECT_ID_SHIFT),

    ENCODER_INTERNAL_UNIPHY2_ENUM_ID1        =( DISPLAY_OBJECT_TYPE_ENCODER << OBJECT_TYPE_SHIFT |\
    OBJECT_ENUM_ID1 << ENUM_ID_SHIFT |\
    ENCODER_OBJECT_ID_INTERNAL_UNIPHY2 << OBJECT_ID_SHIFT),

    ENCODER_INTERNAL_UNIPHY2_ENUM_ID2        =( DISPLAY_OBJECT_TYPE_ENCODER << OBJECT_TYPE_SHIFT |\
    OBJECT_ENUM_ID2 << ENUM_ID_SHIFT |\
    ENCODER_OBJECT_ID_INTERNAL_UNIPHY2 << OBJECT_ID_SHIFT)
}

//
// Connector Object definition - Shared with BIOS
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum connector_objet_def {
    CONNECTOR_LVDS_ENUM_ID1							=( DISPLAY_OBJECT_TYPE_CONNECTOR << OBJECT_TYPE_SHIFT |\
    OBJECT_ENUM_ID1 << ENUM_ID_SHIFT |\
    CONNECTOR_OBJECT_ID_LVDS << OBJECT_ID_SHIFT),


    CONNECTOR_eDP_ENUM_ID1							=( DISPLAY_OBJECT_TYPE_CONNECTOR << OBJECT_TYPE_SHIFT |\
    OBJECT_ENUM_ID1 << ENUM_ID_SHIFT |\
    CONNECTOR_OBJECT_ID_eDP << OBJECT_ID_SHIFT),

    CONNECTOR_SINGLE_LINK_DVI_D_ENUM_ID1			=( DISPLAY_OBJECT_TYPE_CONNECTOR << OBJECT_TYPE_SHIFT |\
    OBJECT_ENUM_ID1 << ENUM_ID_SHIFT |\
    CONNECTOR_OBJECT_ID_SINGLE_LINK_DVI_D << OBJECT_ID_SHIFT),

    CONNECTOR_SINGLE_LINK_DVI_D_ENUM_ID2			=( DISPLAY_OBJECT_TYPE_CONNECTOR << OBJECT_TYPE_SHIFT |\
    OBJECT_ENUM_ID2 << ENUM_ID_SHIFT |\
    CONNECTOR_OBJECT_ID_SINGLE_LINK_DVI_D << OBJECT_ID_SHIFT),


    CONNECTOR_DUAL_LINK_DVI_D_ENUM_ID1				=( DISPLAY_OBJECT_TYPE_CONNECTOR << OBJECT_TYPE_SHIFT |\
    OBJECT_ENUM_ID1 << ENUM_ID_SHIFT |\
    CONNECTOR_OBJECT_ID_DUAL_LINK_DVI_D << OBJECT_ID_SHIFT),

    CONNECTOR_DUAL_LINK_DVI_D_ENUM_ID2				=( DISPLAY_OBJECT_TYPE_CONNECTOR << OBJECT_TYPE_SHIFT |\
    OBJECT_ENUM_ID2 << ENUM_ID_SHIFT |\
    CONNECTOR_OBJECT_ID_DUAL_LINK_DVI_D << OBJECT_ID_SHIFT),

    CONNECTOR_HDMI_TYPE_A_ENUM_ID1					=( DISPLAY_OBJECT_TYPE_CONNECTOR << OBJECT_TYPE_SHIFT |\
    OBJECT_ENUM_ID1 << ENUM_ID_SHIFT |\
    CONNECTOR_OBJECT_ID_HDMI_TYPE_A << OBJECT_ID_SHIFT),

    CONNECTOR_HDMI_TYPE_A_ENUM_ID2					=( DISPLAY_OBJECT_TYPE_CONNECTOR << OBJECT_TYPE_SHIFT |\
    OBJECT_ENUM_ID2 << ENUM_ID_SHIFT |\
    CONNECTOR_OBJECT_ID_HDMI_TYPE_A << OBJECT_ID_SHIFT),

    CONNECTOR_DISPLAYPORT_ENUM_ID1					=( DISPLAY_OBJECT_TYPE_CONNECTOR << OBJECT_TYPE_SHIFT |\
    OBJECT_ENUM_ID1 << ENUM_ID_SHIFT |\
    CONNECTOR_OBJECT_ID_DISPLAYPORT << OBJECT_ID_SHIFT),

    CONNECTOR_DISPLAYPORT_ENUM_ID2					=( DISPLAY_OBJECT_TYPE_CONNECTOR << OBJECT_TYPE_SHIFT |\
    OBJECT_ENUM_ID2 << ENUM_ID_SHIFT |\
    CONNECTOR_OBJECT_ID_DISPLAYPORT << OBJECT_ID_SHIFT),

    CONNECTOR_DISPLAYPORT_ENUM_ID3					=( DISPLAY_OBJECT_TYPE_CONNECTOR << OBJECT_TYPE_SHIFT |\
    OBJECT_ENUM_ID3 << ENUM_ID_SHIFT |\
    CONNECTOR_OBJECT_ID_DISPLAYPORT << OBJECT_ID_SHIFT),

    CONNECTOR_DISPLAYPORT_ENUM_ID4					=( DISPLAY_OBJECT_TYPE_CONNECTOR << OBJECT_TYPE_SHIFT |\
    OBJECT_ENUM_ID4 << ENUM_ID_SHIFT |\
    CONNECTOR_OBJECT_ID_DISPLAYPORT << OBJECT_ID_SHIFT),

    CONNECTOR_OPM_ENUM_ID1							=( DISPLAY_OBJECT_TYPE_CONNECTOR << OBJECT_TYPE_SHIFT |\
    OBJECT_ENUM_ID1 << ENUM_ID_SHIFT |\
    CONNECTOR_OBJECT_ID_OPM << OBJECT_ID_SHIFT),          //Mapping to MXM_DP_A

    CONNECTOR_OPM_ENUM_ID2							=( DISPLAY_OBJECT_TYPE_CONNECTOR << OBJECT_TYPE_SHIFT |\
    OBJECT_ENUM_ID2 << ENUM_ID_SHIFT |\
    CONNECTOR_OBJECT_ID_OPM << OBJECT_ID_SHIFT),          //Mapping to MXM_DP_B

    CONNECTOR_OPM_ENUM_ID3							=( DISPLAY_OBJECT_TYPE_CONNECTOR << OBJECT_TYPE_SHIFT |\
    OBJECT_ENUM_ID3 << ENUM_ID_SHIFT |\
    CONNECTOR_OBJECT_ID_OPM << OBJECT_ID_SHIFT),          //Mapping to MXM_DP_C

    CONNECTOR_OPM_ENUM_ID4							=( DISPLAY_OBJECT_TYPE_CONNECTOR << OBJECT_TYPE_SHIFT |\
    OBJECT_ENUM_ID4 << ENUM_ID_SHIFT |\
    CONNECTOR_OBJECT_ID_OPM << OBJECT_ID_SHIFT),          //Mapping to MXM_DP_D

    CONNECTOR_OPM_ENUM_ID5							=( DISPLAY_OBJECT_TYPE_CONNECTOR << OBJECT_TYPE_SHIFT |\
    OBJECT_ENUM_ID5 << ENUM_ID_SHIFT |\
    CONNECTOR_OBJECT_ID_OPM << OBJECT_ID_SHIFT),          //Mapping to MXM_LVDS_TXxx


    CONNECTOR_OPM_ENUM_ID6							=( DISPLAY_OBJECT_TYPE_CONNECTOR << OBJECT_TYPE_SHIFT |\
    OBJECT_ENUM_ID6 << ENUM_ID_SHIFT |\
    CONNECTOR_OBJECT_ID_OPM << OBJECT_ID_SHIFT)         //Mapping to MXM_LVDS_TXxx
}

//
// Router Object ID definition - Shared with BIOS
//
// No Need, in future we ever need, we can define a record in atomfirwareSoC15.h associated with an object that has this router
//
// PROTECTION Object ID definition - Shared with BIOS
//
// No need,in future we ever need, all display path are capable of protection now.
//
// Generic Object ID definition - Shared with BIOS
//
// No need, in future we ever need like GLsync, we can define a record in atomfirwareSoC15.h associated with an object.

