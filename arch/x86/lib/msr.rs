//! Automatically rewritten from C to Rust
//! Source: arch/x86/lib/msr.c
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

// Macro flag: #define CREATE_TRACE_POINTS

    struct msr __percpu *msrs_alloc(void)
    {
    struct msr __percpu *msrs = core::ptr::null_mut();
    msrs = alloc_percpu(struct msr);
    if (!msrs) {
    pr_warn("%s: error allocating msrs\n", __func__);
    return core::ptr::null_mut();
    }
    return msrs;
    }
    EXPORT_SYMBOL(msrs_alloc);
#[no_mangle]
pub unsafe extern "C" fn msrs_free(msrs: *mut msr __percpu) {
    void msrs_free(struct msr __percpu *msrs)
    {
    free_percpu(msrs);
    }
    EXPORT_SYMBOL(msrs_free);
//
// msr_read - Read an MSR with error handling
// @msr: MSR to read
// @m: value to read into
//
// It returns read data only on success, otherwise it doesn't change the output
// argument @m.
//
// Return: %0 for success, otherwise an error code
//
#[no_mangle]
unsafe extern "C" fn msr_read(msr: u32, m: *mut msr) -> c_int {
    static int msr_read(u32 msr, struct msr *m)
    {
    int err;
    u64 val;
    err = rdmsrq_safe(msr, &val);
    if (!err)
    m.q = val;
    return err;
    }
//
// msr_write - Write an MSR with error handling
//
// @msr: MSR to write
// @m: value to write
//
// Return: %0 for success, otherwise an error code
//
#[no_mangle]
unsafe extern "C" fn msr_write(msr: u32, m: *mut msr) -> c_int {
    static int msr_write(u32 msr, struct msr *m)
    {
    return wrmsrq_safe(msr, m.q);
    }
#[no_mangle]
pub unsafe extern "C" fn __flip_bit(msr: u32, bit: u8, set: bool) -> c_int {
    static inline int __flip_bit(u32 msr, u8 bit, bool set)
    {
    struct msr m, m1;
    let mut err: c_int = -EINVAL;
    if (bit > 63)
    return err;
    err = msr_read(msr, &m);
    if (err)
    return err;
    m1 = m;
    if (set)
    m1.q |=  BIT_64(bit);
    else
    m1.q &= ~BIT_64(bit);
    if (m1.q == m.q)
    return 0;
    err = msr_write(msr, &m1);
    if (err)
    return err;
    return 1;
    }
//
// msr_set_bit - Set @bit in a MSR @msr.
// @msr: MSR to write
// @bit: bit number to set
//
// Return:
// * < 0: An error was encountered.
// * = 0: Bit was already set.
// * > 0: Hardware accepted the MSR write.
//
#[no_mangle]
pub unsafe extern "C" fn msr_set_bit(msr: u32, bit: u8) -> c_int {
    int msr_set_bit(u32 msr, u8 bit)
    {
    return __flip_bit(msr, bit, true);
    }
    EXPORT_SYMBOL_FOR_KVM(msr_set_bit);
//
// msr_clear_bit - Clear @bit in a MSR @msr.
// @msr: MSR to write
// @bit: bit number to clear
//
// Return:
// * < 0: An error was encountered.
// * = 0: Bit was already cleared.
// * > 0: Hardware accepted the MSR write.
//
#[no_mangle]
pub unsafe extern "C" fn msr_clear_bit(msr: u32, bit: u8) -> c_int {
    int msr_clear_bit(u32 msr, u8 bit)
    {
    return __flip_bit(msr, bit, false);
    }
    EXPORT_SYMBOL_FOR_KVM(msr_clear_bit);

#[no_mangle]
pub unsafe extern "C" fn do_trace_write_msr(msr: u32, val: u64, failed: c_int) {
    void do_trace_write_msr(u32 msr, u64 val, int failed)
    {
    trace_write_msr(msr, val, failed);
    }
    EXPORT_SYMBOL(do_trace_write_msr);
    EXPORT_TRACEPOINT_SYMBOL(write_msr);
#[no_mangle]
pub unsafe extern "C" fn do_trace_read_msr(msr: u32, val: u64, failed: c_int) {
    void do_trace_read_msr(u32 msr, u64 val, int failed)
    {
    trace_read_msr(msr, val, failed);
    }
    EXPORT_SYMBOL(do_trace_read_msr);
    EXPORT_TRACEPOINT_SYMBOL(read_msr);
#[no_mangle]
pub unsafe extern "C" fn do_trace_rdpmc(msr: u32, val: u64, failed: c_int) {
    void do_trace_rdpmc(u32 msr, u64 val, int failed)
    {
    trace_rdpmc(msr, val, failed);
    }
    EXPORT_SYMBOL(do_trace_rdpmc);
    EXPORT_TRACEPOINT_SYMBOL(rdpmc);
