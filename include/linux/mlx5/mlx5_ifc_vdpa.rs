//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mlx5/mlx5_ifc_vdpa.h
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
// Copyright (c) 2020 Mellanox Technologies Ltd.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_virtio_q_bits {
    pub virtio_q_type: [u8; 0x8],
    pub reserved_at_8: [u8; 0x5],
    pub event_mode: [u8; 0x3],
    pub queue_index: [u8; 0x10],
    pub full_emulation: [u8; 0x1],
    pub virtio_version_1_0: [u8; 0x1],
    pub reserved_at_22: [u8; 0x2],
    pub offload_type: [u8; 0x4],
    pub event_qpn_or_msix: [u8; 0x18],
    pub doorbell_stride_index: [u8; 0x10],
    pub queue_size: [u8; 0x10],
    pub device_emulation_id: [u8; 0x20],
    pub desc_addr: [u8; 0x40],
    pub used_addr: [u8; 0x40],
    pub available_addr: [u8; 0x40],
    pub virtio_q_mkey: [u8; 0x20],
    pub max_tunnel_desc: [u8; 0x10],
    pub reserved_at_170: [u8; 0x8],
    pub error_type: [u8; 0x8],
    pub umem_1_id: [u8; 0x20],
    pub umem_1_size: [u8; 0x20],
    pub umem_1_offset: [u8; 0x40],
    pub umem_2_id: [u8; 0x20],
    pub umem_2_size: [u8; 0x20],
    pub umem_2_offset: [u8; 0x40],
    pub umem_3_id: [u8; 0x20],
    pub umem_3_size: [u8; 0x20],
    pub umem_3_offset: [u8; 0x40],
    pub counter_set_id: [u8; 0x20],
    pub reserved_at_320: [u8; 0x8],
    pub pd: [u8; 0x18],
    pub reserved_at_340: [u8; 0x20],
    pub desc_group_mkey: [u8; 0x20],
    pub reserved_at_380: [u8; 0x80],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_virtio_net_q_object_bits {
    pub modify_field_select: [u8; 0x40],
    pub reserved_at_40: [u8; 0x20],
    pub vhca_id: [u8; 0x10],
    pub reserved_at_70: [u8; 0x10],
    pub queue_feature_bit_mask_12_3: [u8; 0xa],
    pub dirty_bitmap_dump_enable: [u8; 0x1],
    pub vhost_log_page: [u8; 0x5],
    pub reserved_at_90: [u8; 0xc],
    pub state: [u8; 0x4],
    pub reserved_at_a0: [u8; 0x5],
    pub queue_feature_bit_mask_2_0: [u8; 0x3],
    pub tisn_or_qpn: [u8; 0x18],
    pub dirty_bitmap_mkey: [u8; 0x20],
    pub dirty_bitmap_size: [u8; 0x20],
    pub dirty_bitmap_addr: [u8; 0x40],
    pub hw_available_index: [u8; 0x10],
    pub hw_used_index: [u8; 0x10],
    pub reserved_at_160: [u8; 0xa0],
    pub virtio_q_context: mlx5_ifc_virtio_q_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_virtio_net_q_in_bits {
    pub general_obj_in_cmd_hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
    pub obj_context: mlx5_ifc_virtio_net_q_object_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_virtio_net_q_out_bits {
    pub general_obj_out_cmd_hdr: mlx5_ifc_general_obj_out_cmd_hdr_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_virtio_net_q_in_bits {
    pub general_obj_out_cmd_hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_virtio_net_q_out_bits {
    pub general_obj_out_cmd_hdr: mlx5_ifc_general_obj_out_cmd_hdr_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_virtio_net_q_in_bits {
    pub general_obj_in_cmd_hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_virtio_net_q_out_bits {
    pub general_obj_out_cmd_hdr: mlx5_ifc_general_obj_out_cmd_hdr_bits,
    pub obj_context: mlx5_ifc_virtio_net_q_object_bits,
}

// This indicates that the object was not created or has already
// been desroyed. It is very safe to assume that this object will never
// have so many states
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_virtio_net_q_in_bits {
    pub general_obj_in_cmd_hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
    pub obj_context: mlx5_ifc_virtio_net_q_object_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_virtio_net_q_out_bits {
    pub general_obj_out_cmd_hdr: mlx5_ifc_general_obj_out_cmd_hdr_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_virtio_q_counters_bits {
    pub modify_field_select: [u8; 0x40],
    pub reserved_at_40: [u8; 0x40],
    pub received_desc: [u8; 0x40],
    pub completed_desc: [u8; 0x40],
    pub error_cqes: [u8; 0x20],
    pub bad_desc_errors: [u8; 0x20],
    pub exceed_max_chain: [u8; 0x20],
    pub invalid_buffer: [u8; 0x20],
    pub reserved_at_180: [u8; 0x280],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_virtio_q_counters_in_bits {
    pub hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
    pub virtio_q_counters: mlx5_ifc_virtio_q_counters_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_virtio_q_counters_out_bits {
    pub hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
    pub virtio_q_counters: mlx5_ifc_virtio_q_counters_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_virtio_q_counters_in_bits {
    pub hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_virtio_q_counters_out_bits {
    pub hdr: mlx5_ifc_general_obj_out_cmd_hdr_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_virtio_q_counters_in_bits {
    pub hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_virtio_q_counters_out_bits {
    pub hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
    pub counters: mlx5_ifc_virtio_q_counters_bits,
}
