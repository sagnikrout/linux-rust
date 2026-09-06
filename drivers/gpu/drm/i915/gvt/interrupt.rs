//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gvt/interrupt.h
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
// Copyright(c) 2011-2016 Intel Corporation. All rights reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
// Authors:
// Kevin Tian <kevin.tian@intel.com>
// Zhi Wang <zhi.a.wang@intel.com>
//
// Contributors:
// Min he <min.he@intel.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_gvt_event_type {
    RCS_MI_USER_INTERRUPT = 0,
    RCS_DEBUG,
    RCS_MMIO_SYNC_FLUSH,
    RCS_CMD_STREAMER_ERR,
    RCS_PIPE_CONTROL,
    RCS_L3_PARITY_ERR,
    RCS_WATCHDOG_EXCEEDED,
    RCS_PAGE_DIRECTORY_FAULT,
    RCS_AS_CONTEXT_SWITCH,
    RCS_MONITOR_BUFF_HALF_FULL,

    VCS_MI_USER_INTERRUPT,
    VCS_MMIO_SYNC_FLUSH,
    VCS_CMD_STREAMER_ERR,
    VCS_MI_FLUSH_DW,
    VCS_WATCHDOG_EXCEEDED,
    VCS_PAGE_DIRECTORY_FAULT,
    VCS_AS_CONTEXT_SWITCH,

    VCS2_MI_USER_INTERRUPT,
    VCS2_MI_FLUSH_DW,
    VCS2_AS_CONTEXT_SWITCH,

    BCS_MI_USER_INTERRUPT,
    BCS_MMIO_SYNC_FLUSH,
    BCS_CMD_STREAMER_ERR,
    BCS_MI_FLUSH_DW,
    BCS_PAGE_DIRECTORY_FAULT,
    BCS_AS_CONTEXT_SWITCH,

    VECS_MI_USER_INTERRUPT,
    VECS_MI_FLUSH_DW,
    VECS_AS_CONTEXT_SWITCH,

    PIPE_A_FIFO_UNDERRUN,
    PIPE_B_FIFO_UNDERRUN,
    PIPE_A_CRC_ERR,
    PIPE_B_CRC_ERR,
    PIPE_A_CRC_DONE,
    PIPE_B_CRC_DONE,
    PIPE_A_ODD_FIELD,
    PIPE_B_ODD_FIELD,
    PIPE_A_EVEN_FIELD,
    PIPE_B_EVEN_FIELD,
    PIPE_A_LINE_COMPARE,
    PIPE_B_LINE_COMPARE,
    PIPE_C_LINE_COMPARE,
    PIPE_A_VBLANK,
    PIPE_B_VBLANK,
    PIPE_C_VBLANK,
    PIPE_A_VSYNC,
    PIPE_B_VSYNC,
    PIPE_C_VSYNC,
    PRIMARY_A_FLIP_DONE,
    PRIMARY_B_FLIP_DONE,
    PRIMARY_C_FLIP_DONE,
    SPRITE_A_FLIP_DONE,
    SPRITE_B_FLIP_DONE,
    SPRITE_C_FLIP_DONE,

    PCU_THERMAL,
    PCU_PCODE2DRIVER_MAILBOX,

    DPST_PHASE_IN,
    DPST_HISTOGRAM,
    GSE,
    DP_A_HOTPLUG,
    AUX_CHANNEL_A,
    PERF_COUNTER,
    POISON,
    GTT_FAULT,
    ERROR_INTERRUPT_COMBINED,

    FDI_RX_INTERRUPTS_TRANSCODER_A,
    AUDIO_CP_CHANGE_TRANSCODER_A,
    AUDIO_CP_REQUEST_TRANSCODER_A,
    FDI_RX_INTERRUPTS_TRANSCODER_B,
    AUDIO_CP_CHANGE_TRANSCODER_B,
    AUDIO_CP_REQUEST_TRANSCODER_B,
    FDI_RX_INTERRUPTS_TRANSCODER_C,
    AUDIO_CP_CHANGE_TRANSCODER_C,
    AUDIO_CP_REQUEST_TRANSCODER_C,
    ERR_AND_DBG,
    GMBUS,
    SDVO_B_HOTPLUG,
    CRT_HOTPLUG,
    DP_B_HOTPLUG,
    DP_C_HOTPLUG,
    DP_D_HOTPLUG,
    AUX_CHANNEL_B,
    AUX_CHANNEL_C,
    AUX_CHANNEL_D,
    AUDIO_POWER_STATE_CHANGE_B,
    AUDIO_POWER_STATE_CHANGE_C,
    AUDIO_POWER_STATE_CHANGE_D,

    INTEL_GVT_EVENT_RESERVED,
    INTEL_GVT_EVENT_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_gvt_irq_ops {
    pub irq): *mut *mut void (init_irq)(struct intel_gvt_irq,
    pub vgpu): *mut *mut void (check_pending_irq)(struct intel_vgpu,
}

// the list of physical interrupt control register groups
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_gvt_irq_type {
    INTEL_GVT_IRQ_INFO_GT,
    INTEL_GVT_IRQ_INFO_DPY,
    INTEL_GVT_IRQ_INFO_PCH,
    INTEL_GVT_IRQ_INFO_PM,

    INTEL_GVT_IRQ_INFO_MASTER,
    INTEL_GVT_IRQ_INFO_GT0,
    INTEL_GVT_IRQ_INFO_GT1,
    INTEL_GVT_IRQ_INFO_GT2,
    INTEL_GVT_IRQ_INFO_GT3,
    INTEL_GVT_IRQ_INFO_DE_PIPE_A,
    INTEL_GVT_IRQ_INFO_DE_PIPE_B,
    INTEL_GVT_IRQ_INFO_DE_PIPE_C,
    INTEL_GVT_IRQ_INFO_DE_PORT,
    INTEL_GVT_IRQ_INFO_DE_MISC,
    INTEL_GVT_IRQ_INFO_AUD,
    INTEL_GVT_IRQ_INFO_PCU,

    INTEL_GVT_IRQ_INFO_MAX,
}

pub const INTEL_GVT_IRQ_BITWIDTH: c_int = 32;
// per-event information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_gvt_event_info {
    pub /: *mut *mut int bit; / map to register bit,
    pub /: *mut *mut *mut intel_gvt_irq_info info; / register info,
    pub /: *mut *mut gvt_event_virt_handler_t v_handler; / for v_event,
}

// structure containing device specific IRQ state
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_gvt_irq {
    pub ops: *const intel_gvt_irq_ops,
    pub info: [*mut intel_gvt_irq_info; INTEL_GVT_IRQ_INFO_MAX],
    pub INTEL_GVT_IRQ_INFO_MAX): DECLARE_BITMAP(irq_info_bitmap,,
    pub events: [intel_gvt_event_info; INTEL_GVT_EVENT_MAX],
    pub irq_map: *mut intel_gvt_irq_map,
}

extern "C" {
    pub fn intel_gvt_init_irq(gvt: *mut intel_gvt) -> c_int;
}
extern "C" {
    pub fn gvt_ring_id_to_pipe_control_notify_event(ring_id: c_int) -> c_int;
}
extern "C" {
    pub fn gvt_ring_id_to_mi_flush_dw_event(ring_id: c_int) -> c_int;
}
extern "C" {
    pub fn gvt_ring_id_to_mi_user_interrupt_event(ring_id: c_int) -> c_int;
}
