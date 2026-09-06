//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/gsp/rm/r570/nvrm/fifo.h
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


// SPDX-License-Identifier: MIT
// Copyright (c) 2025, NVIDIA CORPORATION. All rights reserved.

// Excerpt of RM headers from https://github.com/NVIDIA/open-gpu-kernel-modules/tree/570.144
pub const NV_MAX_SUBDEVICES: c_int = 8;

// handle to UserD memory object for channel, ignored if hUserdMemory[0]=0
// offset to beginning of UserD within hUserdMemory[x]
// engine type(NV2080_ENGINE_TYPE_*) with which this channel is associated
// Channel identifier that is unique for the duration of a RM session
// One-hot encoded bitmask to match SET_SUBDEVICE_MASK methods
// IV used for CPU-side encryption / GPU-side decryption.
// IV used for CPU-side decryption / GPU-side encryption.
// Nonce used CPU-side signing / GPU-side signature verification.
pub type NV_CHANNELGPFIFO_ALLOCATION_PARAMETERS = NV_CHANNEL_ALLOC_PARAMS;

pub const NVOS04_FLAGS_CHANNEL_TYPE_PHYSICAL: c_uint = 0x00000000;
pub const NVOS04_FLAGS_CHANNEL_TYPE_VIRTUAL: c_uint = 0x00000001  // OBSOLETE;
pub const NVOS04_FLAGS_CHANNEL_TYPE_PHYSICAL_FOR_VIRTUAL: c_uint = 0x00000002  // OBSOLETE;

pub const NVOS04_FLAGS_VPR_FALSE: c_uint = 0x00000000;
pub const NVOS04_FLAGS_VPR_TRUE: c_uint = 0x00000001;

pub const NVOS04_FLAGS_CC_SECURE_FALSE: c_uint = 0x00000000;
pub const NVOS04_FLAGS_CC_SECURE_TRUE: c_uint = 0x00000001;

pub const NVOS04_FLAGS_CHANNEL_SKIP_MAP_REFCOUNTING_FALSE: c_uint = 0x00000000;
pub const NVOS04_FLAGS_CHANNEL_SKIP_MAP_REFCOUNTING_TRUE: c_uint = 0x00000001;

pub const NVOS04_FLAGS_GROUP_CHANNEL_RUNQUEUE_DEFAULT: c_uint = 0x00000000;
pub const NVOS04_FLAGS_GROUP_CHANNEL_RUNQUEUE_ONE: c_uint = 0x00000001;

pub const NVOS04_FLAGS_PRIVILEGED_CHANNEL_FALSE: c_uint = 0x00000000;
pub const NVOS04_FLAGS_PRIVILEGED_CHANNEL_TRUE: c_uint = 0x00000001;

pub const NVOS04_FLAGS_DELAY_CHANNEL_SCHEDULING_FALSE: c_uint = 0x00000000;
pub const NVOS04_FLAGS_DELAY_CHANNEL_SCHEDULING_TRUE: c_uint = 0x00000001;

pub const NVOS04_FLAGS_CHANNEL_DENY_PHYSICAL_MODE_CE_FALSE: c_uint = 0x00000000;
pub const NVOS04_FLAGS_CHANNEL_DENY_PHYSICAL_MODE_CE_TRUE: c_uint = 0x00000001;

pub const NVOS04_FLAGS_CHANNEL_USERD_INDEX_FIXED_FALSE: c_uint = 0x00000000;
pub const NVOS04_FLAGS_CHANNEL_USERD_INDEX_FIXED_TRUE: c_uint = 0x00000001;

pub const NVOS04_FLAGS_CHANNEL_USERD_INDEX_PAGE_FIXED_FALSE: c_uint = 0x00000000;
pub const NVOS04_FLAGS_CHANNEL_USERD_INDEX_PAGE_FIXED_TRUE: c_uint = 0x00000001;

pub const NVOS04_FLAGS_CHANNEL_DENY_AUTH_LEVEL_PRIV_FALSE: c_uint = 0x00000000;
pub const NVOS04_FLAGS_CHANNEL_DENY_AUTH_LEVEL_PRIV_TRUE: c_uint = 0x00000001;

pub const NVOS04_FLAGS_CHANNEL_SKIP_SCRUBBER_FALSE: c_uint = 0x00000000;
pub const NVOS04_FLAGS_CHANNEL_SKIP_SCRUBBER_TRUE: c_uint = 0x00000001;

pub const NVOS04_FLAGS_CHANNEL_CLIENT_MAP_FIFO_FALSE: c_uint = 0x00000000;
pub const NVOS04_FLAGS_CHANNEL_CLIENT_MAP_FIFO_TRUE: c_uint = 0x00000001;

pub const NVOS04_FLAGS_SET_EVICT_LAST_CE_PREFETCH_CHANNEL_FALSE: c_uint = 0x00000000;
pub const NVOS04_FLAGS_SET_EVICT_LAST_CE_PREFETCH_CHANNEL_TRUE: c_uint = 0x00000001;

pub const NVOS04_FLAGS_CHANNEL_VGPU_PLUGIN_CONTEXT_FALSE: c_uint = 0x00000000;
pub const NVOS04_FLAGS_CHANNEL_VGPU_PLUGIN_CONTEXT_TRUE: c_uint = 0x00000001;

pub const NVOS04_FLAGS_CHANNEL_PBDMA_ACQUIRE_TIMEOUT_FALSE: c_uint = 0x00000000;
pub const NVOS04_FLAGS_CHANNEL_PBDMA_ACQUIRE_TIMEOUT_TRUE: c_uint = 0x00000001;

pub const NVOS04_FLAGS_GROUP_CHANNEL_THREAD_DEFAULT: c_uint = 0x00000000;
pub const NVOS04_FLAGS_GROUP_CHANNEL_THREAD_ONE: c_uint = 0x00000001;
pub const NVOS04_FLAGS_GROUP_CHANNEL_THREAD_TWO: c_uint = 0x00000002;

pub const NVOS04_FLAGS_MAP_CHANNEL_FALSE: c_uint = 0x00000000;
pub const NVOS04_FLAGS_MAP_CHANNEL_TRUE: c_uint = 0x00000001;

pub const NVOS04_FLAGS_SKIP_CTXBUFFER_ALLOC_FALSE: c_uint = 0x00000000;
pub const NVOS04_FLAGS_SKIP_CTXBUFFER_ALLOC_TRUE: c_uint = 0x00000001;
// !
// Initial state as passed in NV_CHANNEL_ALLOC_PARAMS by
// kernel CPU-RM clients.
//
// ! @brief Error notifier is explicitly not set.
//
// The corresponding hErrorContext or hEccErrorContext must be
// NV01_NULL_OBJECT.
//
// ! @brief Error notifier is a ContextDma
// ! @brief Error notifier is a NvNotification array in sysmem/vidmem

pub const NV_KERNELCHANNEL_ALLOC_INTERNALFLAGS_PRIVILEGE_USER: c_uint = 0x0;
pub const NV_KERNELCHANNEL_ALLOC_INTERNALFLAGS_PRIVILEGE_ADMIN: c_uint = 0x1;
pub const NV_KERNELCHANNEL_ALLOC_INTERNALFLAGS_PRIVILEGE_KERNEL: c_uint = 0x2;

pub const NV_KERNELCHANNEL_ALLOC_INTERNALFLAGS_GSP_OWNED_NO: c_uint = 0x0;
pub const NV_KERNELCHANNEL_ALLOC_INTERNALFLAGS_GSP_OWNED_YES: c_uint = 0x1;

pub const NV_KERNELCHANNEL_ALLOC_INTERNALFLAGS_UVM_OWNED_NO: c_uint = 0x0;
pub const NV_KERNELCHANNEL_ALLOC_INTERNALFLAGS_UVM_OWNED_YES: c_uint = 0x1;
pub const NV2080_CTRL_GPU_MAX_CONSTRUCTED_FALCONS: c_uint = 0x40;

