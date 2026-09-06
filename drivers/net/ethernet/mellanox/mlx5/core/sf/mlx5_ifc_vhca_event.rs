//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/sf/mlx5_ifc_vhca_event.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
// Copyright (c) 2020 Mellanox Technologies Ltd
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_vhca_state_context_bits {
    pub arm_change_event: [u8; 0x1],
    pub reserved_at_1: [u8; 0xb],
    pub vhca_state: [u8; 0x4],
    pub reserved_at_10: [u8; 0x10],
    pub sw_function_id: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_vhca_state_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
    pub vhca_state_context: mlx5_ifc_vhca_state_context_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_vhca_state_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub embedded_cpu_function: [u8; 0x1],
    pub reserved_at_41: [u8; 0xf],
    pub function_id: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_vhca_state_field_select_bits {
    pub reserved_at_0: [u8; 0x1e],
    pub sw_function_id: [u8; 0x1],
    pub arm_change_event: [u8; 0x1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_vhca_state_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_vhca_state_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub embedded_cpu_function: [u8; 0x1],
    pub reserved_at_41: [u8; 0xf],
    pub function_id: [u8; 0x10],
    pub vhca_state_field_select: mlx5_ifc_vhca_state_field_select_bits,
    pub vhca_state_context: mlx5_ifc_vhca_state_context_bits,
}
