//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/intel_uncore.h
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
// Copyright © 2017 Intel Corporation
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
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_uncore_mmio_debug {
    pub /: *mut *mut *mut spinlock_t lock; / lock is also taken in irq contexts.,
    pub unclaimed_mmio_check: c_int,
    pub saved_mmio_check: c_int,
    pub suspend_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum forcewake_domain_id {
    FW_DOMAIN_ID_RENDER = 0,
    FW_DOMAIN_ID_GT,        /* also includes blitter engine */
    FW_DOMAIN_ID_MEDIA,
    FW_DOMAIN_ID_MEDIA_VDBOX0,
    FW_DOMAIN_ID_MEDIA_VDBOX1,
    FW_DOMAIN_ID_MEDIA_VDBOX2,
    FW_DOMAIN_ID_MEDIA_VDBOX3,
    FW_DOMAIN_ID_MEDIA_VDBOX4,
    FW_DOMAIN_ID_MEDIA_VDBOX5,
    FW_DOMAIN_ID_MEDIA_VDBOX6,
    FW_DOMAIN_ID_MEDIA_VDBOX7,
    FW_DOMAIN_ID_MEDIA_VEBOX0,
    FW_DOMAIN_ID_MEDIA_VEBOX1,
    FW_DOMAIN_ID_MEDIA_VEBOX2,
    FW_DOMAIN_ID_MEDIA_VEBOX3,
    FW_DOMAIN_ID_GSC,

    FW_DOMAIN_ID_COUNT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum forcewake_domains {
    FORCEWAKE_RENDER	= BIT(FW_DOMAIN_ID_RENDER),
    FORCEWAKE_GT		= BIT(FW_DOMAIN_ID_GT),
    FORCEWAKE_MEDIA		= BIT(FW_DOMAIN_ID_MEDIA),
    FORCEWAKE_MEDIA_VDBOX0	= BIT(FW_DOMAIN_ID_MEDIA_VDBOX0),
    FORCEWAKE_MEDIA_VDBOX1	= BIT(FW_DOMAIN_ID_MEDIA_VDBOX1),
    FORCEWAKE_MEDIA_VDBOX2	= BIT(FW_DOMAIN_ID_MEDIA_VDBOX2),
    FORCEWAKE_MEDIA_VDBOX3	= BIT(FW_DOMAIN_ID_MEDIA_VDBOX3),
    FORCEWAKE_MEDIA_VDBOX4	= BIT(FW_DOMAIN_ID_MEDIA_VDBOX4),
    FORCEWAKE_MEDIA_VDBOX5	= BIT(FW_DOMAIN_ID_MEDIA_VDBOX5),
    FORCEWAKE_MEDIA_VDBOX6	= BIT(FW_DOMAIN_ID_MEDIA_VDBOX6),
    FORCEWAKE_MEDIA_VDBOX7	= BIT(FW_DOMAIN_ID_MEDIA_VDBOX7),
    FORCEWAKE_MEDIA_VEBOX0	= BIT(FW_DOMAIN_ID_MEDIA_VEBOX0),
    FORCEWAKE_MEDIA_VEBOX1	= BIT(FW_DOMAIN_ID_MEDIA_VEBOX1),
    FORCEWAKE_MEDIA_VEBOX2	= BIT(FW_DOMAIN_ID_MEDIA_VEBOX2),
    FORCEWAKE_MEDIA_VEBOX3	= BIT(FW_DOMAIN_ID_MEDIA_VEBOX3),
    FORCEWAKE_GSC		= BIT(FW_DOMAIN_ID_GSC),

    FORCEWAKE_ALL = BIT(FW_DOMAIN_ID_COUNT) - 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_uncore_fw_get {
    pub domains): forcewake_domains,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_uncore_funcs {
    pub r): i915_reg_t,
    pub r): i915_reg_t,
    pub trace): i915_reg_t r, bool,
    pub trace): i915_reg_t r, bool,
    pub trace): i915_reg_t r, bool,
    pub trace): i915_reg_t r, bool,
    pub trace): i915_reg_t r, u8 val, bool,
    pub trace): i915_reg_t r, u16 val, bool,
    pub trace): i915_reg_t r, u32 val, bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_forcewake_range {
    pub start: u32,
    pub end: u32,
    pub domains: forcewake_domains,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_uncore {
    pub regs: *mut void __iomem,
    pub i915: *mut drm_i915_private,
    pub gt: *mut intel_gt,
    pub rpm: *mut intel_runtime_pm,
    pub /: *mut *mut *mut spinlock_t lock; / lock is also taken in irq contexts.,
//
// Do we need to apply an additional offset to reach the beginning
// of the basic non-engine GT registers (referred to as "GSI" on
// newer platforms, or "GT block" on older platforms)?  If so, we'll
// track that here and apply it transparently to registers in the
// appropriate range to maintain compatibility with our existing
// register definitions and GT code.
//
    pub gsi_offset: u32,
    pub flags: c_uint,

    pub fw_domains_table: *const intel_forcewake_range,
    pub fw_domains_table_entries: c_uint,
//
// Shadowed registers are special cases where we can safely write
// to the register *without* grabbing forcewake.
//
    pub shadowed_reg_table: *const i915_mmio_range,
    pub shadowed_reg_table_entries: c_uint,
    pub pmic_bus_access_nb: notifier_block,
    pub fw_get_funcs: *const intel_uncore_fw_get,
    pub funcs: intel_uncore_funcs,
    pub fifo_count: c_uint,
    pub fw_domains: forcewake_domains,
    pub fw_domains_active: forcewake_domains,
    pub fw_domains_timer: forcewake_domains,
    pub /: *mut *mut forcewake_domains fw_domains_saved; / user domains saved for S3,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_uncore_forcewake_domain {
    pub uncore: *mut intel_uncore,
    pub id: forcewake_domain_id,
    pub mask: forcewake_domains,
    pub wake_count: c_uint,
    pub active: bool,
    pub timer: hrtimer,
    pub reg_set: *mut u32 __iomem,
    pub reg_ack: *mut u32 __iomem,
    pub fw_domain: [*mut }; FW_DOMAIN_ID_COUNT],
    pub user_forcewake_count: c_uint,
    pub debug: *mut intel_uncore_mmio_debug,
}

// Iterate over initialised fw domains

extern "C" {
    pub fn intel_uncore_mmio_debug_init_early(i915: *mut drm_i915_private);
}
extern "C" {
    pub fn intel_uncore_setup_mmio(uncore: *mut intel_uncore, phys_addr: phys_addr_t) -> c_int;
}
extern "C" {
    pub fn intel_uncore_init_mmio(uncore: *mut intel_uncore) -> c_int;
}
extern "C" {
    pub fn intel_uncore_unclaimed_mmio(uncore: *mut intel_uncore) -> bool;
}
extern "C" {
    pub fn intel_uncore_arm_unclaimed_mmio_detection(uncore: *mut intel_uncore) -> bool;
}
extern "C" {
    pub fn intel_uncore_cleanup_mmio(uncore: *mut intel_uncore);
}
extern "C" {
    pub fn intel_uncore_fini_mmio(dev: *mut drm_device, data: *mut c_void);
}
extern "C" {
    pub fn intel_uncore_suspend(uncore: *mut intel_uncore);
}
extern "C" {
    pub fn intel_uncore_resume_early(uncore: *mut intel_uncore);
}
extern "C" {
    pub fn intel_uncore_runtime_resume(uncore: *mut intel_uncore);
}
extern "C" {
    pub fn assert_forcewakes_inactive(uncore: *mut intel_uncore);
}

//
// Like above but the caller must manage the uncore.lock itself.
// Must be used with intel_uncore_read_fw() and friends.
//
extern "C" {
    pub fn intel_uncore_forcewake_user_get(uncore: *mut intel_uncore);
}
extern "C" {
    pub fn intel_uncore_forcewake_user_put(uncore: *mut intel_uncore);
}

// register access functions

// Be very careful with read/write 64-bit values. On 32-bit machines, they
// will be implemented using 2 32-bit writes in an arbitrary order with
// an arbitrary delay between them. This can cause the hardware to
// act upon the intermediate value, possibly leading to corruption and
// machine death. For this reason we do not support intel_uncore_write64,
// or uncore->funcs.mmio_writeq.
//
// When reading a 64-bit value as two 32-bit values, the delay may cause
// the two reads to mismatch, e.g. a timestamp overflowing. Also note that
// occasionally a 64-bit register does not actually support a full readq
// and must be read using two 32-bit reads.
//
// You have been warned.
//

// These are untraced mmio-accessors that are only valid to be used inside
// critical sections, such as inside IRQ handlers, where forcewake is explicitly
// controlled.
//
// Think twice, and think again, before using these.
//
// As an example, these accessors can possibly be used between:
//
// spin_lock_irq(&uncore->lock);
// intel_uncore_forcewake_get__locked();
//
// and
//
// intel_uncore_forcewake_put__locked();
// spin_unlock_irq(&uncore->lock);
//
// Note: some registers may not need forcewake held, so
// intel_uncore_forcewake_{get,put} can be omitted, see
// intel_uncore_forcewake_for_reg().
//
// Certain architectures will die if the same cacheline is concurrently accessed
// by different clients (e.g. on Ivybridge). Access to registers should
// therefore generally be serialised, by either the dev_priv->uncore.lock or
// a more localised lock guarding all access to that bank of registers.
//

//
// The raw_reg_{read,write} macros are intended as a micro-optimization for
// interrupt handlers so that the pointer indirection on uncore->regs can
// be computed once (and presumably cached in a register) instead of generating
// extra load instructions for each MMIO access.
//
// Given that these macros are only intended for non-GSI interrupt registers
// (and the goal is to avoid extra instructions generated by the compiler),
// these macros do not account for uncore->gsi_offset.  Any caller that needs
// to use these macros on a GSI register is responsible for adding the
// appropriate GSI offset to the 'base' parameter.
//

