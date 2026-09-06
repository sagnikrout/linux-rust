//! Automatically rewritten from C to Rust
//! Source: drivers/char/tpm/eventlog/tpm2.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2016 IBM Corporation
//
// Authors:
// Nayna Jain <nayna@linux.vnet.ibm.com>
//
// Access to TPM 2.0 event log as written by Firmware.
// It assumes that writer of event log has followed TCG Specification
// for Family "2.0" and written the event data in little endian.
// With that, it doesn't need any endian conversion for structure
// content.
//

//
// calc_tpm2_event_size() - calculate the event size, where event
// is an entry in the TPM 2.0 event log. The event is of type Crypto
// Agile Log Entry Format as defined in TCG EFI Protocol Specification
// Family "2.0".
// @event: event whose size is to be calculated.
// @event_header: the first event in the event log.
//
// Returns size of the event. If it is an invalid event, returns 0.
//
    static size_t calc_tpm2_event_size(struct tcg_pcr_event2_head *event,
    struct tcg_pcr_event *event_header)
    {
    return __calc_tpm2_event_size(event, event_header, false);
    }
    static void *tpm2_bios_measurements_start(struct seq_file *m, loff_t *pos)
    {
    struct tpm_chip *chip = m.private;
    struct tpm_bios_log *log = &chip.log;
    void *addr = log.bios_event_log;
    void *limit = log.bios_event_log_end;
    struct tcg_pcr_event *event_header;
    struct tcg_pcr_event2_head *event;
    size_t size;
    int i;
    event_header = addr;
    size = struct_size(event_header, event, event_header.event_size);
    if (*pos == 0) {
    if (addr + size < limit) {
    if ((event_header.event_type == 0) &&
    (event_header.event_size == 0))
    return core::ptr::null_mut();
    return SEQ_START_TOKEN;
    }
    }
    if (*pos > 0) {
    addr += size;
    event = addr;
    size = calc_tpm2_event_size(event, event_header);
    if ((addr + size >=  limit) || (size == 0))
    return core::ptr::null_mut();
    }
    for (i = 0; i < (*pos - 1); i++) {
    event = addr;
    size = calc_tpm2_event_size(event, event_header);
    if ((addr + size >= limit) || (size == 0))
    return core::ptr::null_mut();
    addr += size;
    }
    return addr;
    }
    static void *tpm2_bios_measurements_next(struct seq_file *m, void *v,
    loff_t *pos)
    {
    struct tcg_pcr_event *event_header;
    struct tcg_pcr_event2_head *event;
    struct tpm_chip *chip = m.private;
    struct tpm_bios_log *log = &chip.log;
    void *limit = log.bios_event_log_end;
    size_t event_size;
    void *marker;
    (*pos)++;
    event_header = log.bios_event_log;
    if (v == SEQ_START_TOKEN) {
    event_size = struct_size(event_header, event,
    event_header.event_size);
    marker = event_header;
    } else {
    event = v;
    event_size = calc_tpm2_event_size(event, event_header);
    if (event_size == 0)
    return core::ptr::null_mut();
    marker = event;
    }
    marker = marker + event_size;
    if (marker >= limit)
    return core::ptr::null_mut();
    v = marker;
    event = v;
    event_size = calc_tpm2_event_size(event, event_header);
    if (((v + event_size) >= limit) || (event_size == 0))
    return core::ptr::null_mut();
    return v;
    }
#[no_mangle]
unsafe extern "C" fn tpm2_bios_measurements_stop(m: *mut seq_file, v: *mut c_void) {
    static void tpm2_bios_measurements_stop(struct seq_file *m, void *v)
    {
    }
#[no_mangle]
unsafe extern "C" fn tpm2_binary_bios_measurements_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int tpm2_binary_bios_measurements_show(struct seq_file *m, void *v)
    {
    struct tpm_chip *chip = m.private;
    struct tpm_bios_log *log = &chip.log;
    struct tcg_pcr_event *event_header = log.bios_event_log;
    struct tcg_pcr_event2_head *event = v;
    void *temp_ptr;
    size_t size;
    if (v == SEQ_START_TOKEN) {
    size = struct_size(event_header, event,
    event_header.event_size);
    temp_ptr = event_header;
    if (size > 0)
    seq_write(m, temp_ptr, size);
    } else {
    size = calc_tpm2_event_size(event, event_header);
    temp_ptr = event;
    if (size > 0)
    seq_write(m, temp_ptr, size);
    }
    return 0;
    }
    const struct seq_operations tpm2_binary_b_measurements_seqops = {
    .start = tpm2_bios_measurements_start,
    .next = tpm2_bios_measurements_next,
    .stop = tpm2_bios_measurements_stop,
    .show = tpm2_binary_bios_measurements_show,
    };
