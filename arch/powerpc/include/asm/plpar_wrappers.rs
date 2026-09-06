//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/plpar_wrappers.h
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


// SPDX-License-Identifier: GPL-2.0

extern "C" {
    pub fn plpar_hcall_norets(_arg: H_POLL_PENDING) -> return;
}
//
// We cannot call tracepoints inside RCU idle regions which
// means we must not trace H_CEDE.
//
extern "C" {
    pub fn plpar_hcall_norets_notrace(_arg: H_CEDE) -> return;
}
extern "C" {
    pub fn plpar_hcall_norets(_arg: H_REGISTER_VPA, _arg: flags, _arg: cpu, _arg: vpa) -> return;
}
extern "C" {
    pub fn vpa_call(_arg: H_VPA_DEREG_VPA, _arg: cpu, _arg: 0) -> return;
}
extern "C" {
    pub fn vpa_call(_arg: H_VPA_REG_VPA, _arg: cpu, _arg: vpa) -> return;
}
extern "C" {
    pub fn vpa_call(_arg: H_VPA_DEREG_SLB, _arg: cpu, _arg: 0) -> return;
}
extern "C" {
    pub fn vpa_call(_arg: H_VPA_REG_SLB, _arg: cpu, _arg: vpa) -> return;
}
extern "C" {
    pub fn vpa_call(_arg: H_VPA_DEREG_DTL, _arg: cpu, _arg: 0) -> return;
}
extern "C" {
    pub fn vpa_call(_arg: H_VPA_REG_DTL, _arg: cpu, _arg: vpa) -> return;
}
//
// Invokes H_HTM hcall with parameters passed from htm_hcall_wrapper.
// flags: Set to hardwareTarget.
// target: Specifies target using node index, nodal chip index and core index.
// operation : action to perform ie configure, start, stop, deconfigure, trace
// based on the HTM type.
// param1, param2, param3: parameters for each action.
//
extern "C" {
    pub fn vpa_init(cpu: c_int);
}
// slot = retbuf[0];
// old_pteh_ret = retbuf[0];
// old_ptel_ret = retbuf[1];
// plpar_pte_remove_raw can be called in real mode. It calls plpar_hcall_raw
// old_pteh_ret = retbuf[0];
// old_ptel_ret = retbuf[1];
// old_pteh_ret = retbuf[0];
// old_ptel_ret = retbuf[1];
// plpar_pte_read_raw can be called in real mode. It calls plpar_hcall_raw
// old_pteh_ret = retbuf[0];
// old_ptel_ret = retbuf[1];
//
// ptes must be 8*sizeof(unsigned long)
//
// plpar_pte_read_4_raw can be called in real mode.
// ptes must be 8*sizeof(unsigned long)
//
extern "C" {
    pub fn plpar_hcall_norets(_arg: H_PROTECT, _arg: flags, _arg: ptex, _arg: avpn) -> return;
}
extern "C" {
    pub fn plpar_hcall_norets(_arg: H_RESIZE_HPT_PREPARE, _arg: flags, _arg: shift) -> return;
}
extern "C" {
    pub fn plpar_hcall_norets(_arg: H_RESIZE_HPT_COMMIT, _arg: flags, _arg: shift) -> return;
}
// tce_ret = retbuf[0];
extern "C" {
    pub fn plpar_hcall_norets(_arg: H_PUT_TCE, _arg: liobn, _arg: ioba, _arg: tceval) -> return;
}
extern "C" {
    pub fn plpar_hcall_norets(_arg: H_PUT_TCE_INDIRECT, _arg: liobn, _arg: ioba, _arg: page, _arg: count) -> return;
}
extern "C" {
    pub fn plpar_hcall_norets(_arg: H_STUFF_TCE, _arg: liobn, _arg: ioba, _arg: tceval, _arg: count) -> return;
}
// Set various resource mode parameters
extern "C" {
    pub fn plpar_hcall_norets(_arg: H_SET_MODE, _arg: mflags, _arg: resource, _arg: value1, _arg: value2) -> return;
}
//
// Enable relocation on exceptions on this partition
//
// Note: this call has a partition wide scope and can take a while to complete.
// If it returns H_LONG_BUSY_* it should be retried periodically until it
// returns H_SUCCESS.
//
// mflags = 3: Exceptions at 0xC000000000004000
extern "C" {
    pub fn plpar_set_mode(_arg: 3, _arg: H_SET_MODE_RESOURCE_ADDR_TRANS_MODE, _arg: 0, _arg: 0) -> return;
}
//
// Disable relocation on exceptions on this partition
//
// Note: this call has a partition wide scope and can take a while to complete.
// If it returns H_LONG_BUSY_* it should be retried periodically until it
// returns H_SUCCESS.
//
extern "C" {
    pub fn plpar_set_mode(_arg: 0, _arg: H_SET_MODE_RESOURCE_ADDR_TRANS_MODE, _arg: 0, _arg: 0) -> return;
}
//
// Take exceptions in big endian mode on this partition
//
// Note: this call has a partition wide scope and can take a while to complete.
// If it returns H_LONG_BUSY_* it should be retried periodically until it
// returns H_SUCCESS.
//
// mflags = 0: big endian exceptions
extern "C" {
    pub fn plpar_set_mode(_arg: 0, _arg: H_SET_MODE_RESOURCE_LE, _arg: 0, _arg: 0) -> return;
}
//
// Take exceptions in little endian mode on this partition
//
// Note: this call has a partition wide scope and can take a while to complete.
// If it returns H_LONG_BUSY_* it should be retried periodically until it
// returns H_SUCCESS.
//
// mflags = 1: little endian exceptions
extern "C" {
    pub fn plpar_set_mode(_arg: 1, _arg: H_SET_MODE_RESOURCE_LE, _arg: 0, _arg: 0) -> return;
}
extern "C" {
    pub fn plpar_set_mode(_arg: 0, _arg: H_SET_MODE_RESOURCE_SET_CIABR, _arg: ciabr, _arg: 0) -> return;
}
extern "C" {
    pub fn plpar_set_mode(_arg: 0, _arg: H_SET_MODE_RESOURCE_SET_DAWR0, _arg: dawr0, _arg: dawrx0) -> return;
}
extern "C" {
    pub fn plpar_set_mode(_arg: 0, _arg: H_SET_MODE_RESOURCE_SET_DAWR1, _arg: dawr1, _arg: dawrx1) -> return;
}
extern "C" {
    pub fn plpar_hcall_norets(_arg: H_SIGNAL_SYS_RESET, _arg: cpu) -> return;
}
// guest_id = retbuf[0];
// failed_index = retbuf[0];
// trap = retbuf[0];
// failed_index = retbuf[0];
// capabilities = retbuf[0];
//
// Wrapper to H_RPT_INVALIDATE hcall that handles return values appropriately
//
// - Returns H_SUCCESS on success
// - For H_BUSY return value, we retry the hcall.
// - For any other hcall failures, attempt a full flush once before
// resorting to BUG().
//
// Note: This hcall is expected to fail only very rarely. The correct
// error recovery of killing the process/guest will be eventually
// needed.
//
// Flush request failed, try with a full flush once

