//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/cpu/mce/threshold.c
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
//
// Common corrected MCE threshold handler code:
//

    static u32 mce_apei_thr_limit;
#[no_mangle]
pub unsafe extern "C" fn mce_save_apei_thr_limit(thr_limit: u32) {
    void mce_save_apei_thr_limit(u32 thr_limit)
    {
    mce_apei_thr_limit = thr_limit;
    pr_info("HEST corrected error threshold limit: %u\n", thr_limit);
    }
#[no_mangle]
pub unsafe extern "C" fn mce_get_apei_thr_limit() -> u32 {
    u32 mce_get_apei_thr_limit(void)
    {
    return mce_apei_thr_limit;
    }
#[no_mangle]
unsafe extern "C" fn default_threshold_interrupt() {
    static void default_threshold_interrupt(void)
    {
    pr_err("Unexpected threshold interrupt at vector %x\n",
    THRESHOLD_APIC_VECTOR);
    }
    void (*mce_threshold_vector)(void) = default_threshold_interrupt;
    DEFINE_IDTENTRY_SYSVEC(sysvec_threshold)
    {
    trace_threshold_apic_entry(THRESHOLD_APIC_VECTOR);
    inc_irq_stat(THRESHOLD_APIC);
    mce_threshold_vector();
    trace_threshold_apic_exit(THRESHOLD_APIC_VECTOR);
    apic_eoi();
    }
    DEFINE_PER_CPU(struct mca_storm_desc, storm_desc);
#[no_mangle]
pub unsafe extern "C" fn mce_inherit_storm(bank: c_uint) {
    void mce_inherit_storm(unsigned int bank)
    {
    struct mca_storm_desc *storm = this_cpu_ptr(&storm_desc);
//
// Previous CPU owning this bank had put it into storm mode,
// but the precise history of that storm is unknown. Assume
// the worst (all recent polls of the bank found a valid error
// logged). This will avoid the new owner prematurely declaring
// the storm has ended.
//
    storm.banks[bank].history = ~0ull;
    storm.banks[bank].timestamp = jiffies;
    }
#[no_mangle]
pub unsafe extern "C" fn mce_get_storm_mode() -> bool {
    bool mce_get_storm_mode(void)
    {
    return __this_cpu_read(storm_desc.poll_mode);
    }
#[no_mangle]
pub unsafe extern "C" fn mce_set_storm_mode(storm: bool) {
    void mce_set_storm_mode(bool storm)
    {
    __this_cpu_write(storm_desc.poll_mode, storm);
    }
#[no_mangle]
unsafe extern "C" fn mce_handle_storm(bank: c_uint, on: bool) {
    static void mce_handle_storm(unsigned int bank, bool on)
    {
    switch (boot_cpu_data.x86_vendor) {
    case X86_VENDOR_INTEL:
    mce_intel_handle_storm(bank, on);
    break;
    case X86_VENDOR_AMD:
    mce_amd_handle_storm(bank, on);
    break;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn cmci_storm_begin(bank: c_uint) {
    void cmci_storm_begin(unsigned int bank)
    {
    struct mca_storm_desc *storm = this_cpu_ptr(&storm_desc);
    __set_bit(bank, this_cpu_ptr(mce_poll_banks));
    storm.banks[bank].in_storm_mode = true;
//
// If this is the first bank on this CPU to enter storm mode
// start polling.
//
    if (++storm.stormy_bank_count == 1)
    mce_timer_kick(true);
    }
#[no_mangle]
pub unsafe extern "C" fn cmci_storm_end(bank: c_uint) {
    void cmci_storm_end(unsigned int bank)
    {
    struct mca_storm_desc *storm = this_cpu_ptr(&storm_desc);
    if (!mce_flags.amd_threshold)
    __clear_bit(bank, this_cpu_ptr(mce_poll_banks));
    storm.banks[bank].history = 0;
    storm.banks[bank].in_storm_mode = false;
// If no banks left in storm mode, stop polling.
    if (!--storm.stormy_bank_count)
    mce_timer_kick(false);
    }
#[no_mangle]
pub unsafe extern "C" fn mce_track_storm(mce: *mut mce) {
    void mce_track_storm(struct mce *mce)
    {
    struct mca_storm_desc *storm = this_cpu_ptr(&storm_desc);
    let mut now: c_ulong = jiffies, delta;
    let mut shift: c_uint = 1;
    let mut history: u64 = 0;
// No tracking needed for banks that do not support CMCI
    if (storm.banks[mce.bank].poll_only)
    return;
//
// When a bank is in storm mode it is polled once per second and
// the history mask will record about the last minute of poll results.
// If it is not in storm mode, then the bank is only checked when
// there is a CMCI interrupt. Check how long it has been since
// this bank was last checked, and adjust the amount of "shift"
// to apply to history.
//
    if (!storm.banks[mce.bank].in_storm_mode) {
    delta = now - storm.banks[mce.bank].timestamp;
    shift = (delta + HZ) / HZ;
    }
// If it has been a long time since the last poll, clear history.
    if (shift < NUM_HISTORY_BITS)
    history = storm.banks[mce.bank].history << shift;
    storm.banks[mce.bank].timestamp = now;
// History keeps track of corrected errors. VAL=1 && UC=0
    if ((mce.status & MCI_STATUS_VAL) && mce_is_correctable(mce))
    history |= 1;
    storm.banks[mce.bank].history = history;
    if (storm.banks[mce.bank].in_storm_mode) {
    if (history & GENMASK_ULL(STORM_END_POLL_THRESHOLD, 0))
    return;
    printk_deferred(KERN_NOTICE "CPU%d BANK%d CMCI storm subsided\n", smp_processor_id(), mce.bank);
    mce_handle_storm(mce.bank, false);
    cmci_storm_end(mce.bank);
    } else {
    if (hweight64(history) < STORM_BEGIN_THRESHOLD)
    return;
    printk_deferred(KERN_NOTICE "CPU%d BANK%d CMCI storm detected\n", smp_processor_id(), mce.bank);
    mce_handle_storm(mce.bank, true);
    cmci_storm_begin(mce.bank);
    }
    }
