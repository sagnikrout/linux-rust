//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/vmclock-abi.h
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


// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-2-Clause)
//
// This structure provides a vDSO-style clock to VM guests, exposing the
// relationship (or lack thereof) between the CPU clock (TSC, timebase, arch
// counter, etc.) and real time. It is designed to address the problem of
// live migration, which other clock enlightenments do not.
//
// When a guest is live migrated, this affects the clock in two ways.
//
// First, even between identical hosts the actual frequency of the underlying
// counter will change within the tolerances of its specification (typically
// ±50PPM, or 4 seconds a day). This frequency also varies over time on the
// same host, but can be tracked by NTP as it generally varies slowly. With
// live migration there is a step change in the frequency, with no warning.
//
// Second, there may be a step change in the value of the counter itself, as
// its accuracy is limited by the precision of the NTP synchronization on the
// source and destination hosts.
//
// So any calibration (NTP, PTP, etc.) which the guest has done on the source
// host before migration is invalid, and needs to be redone on the new host.
//
// In its most basic mode, this structure provides only an indication to the
// guest that live migration has occurred. This allows the guest to know that
// its clock is invalid and take remedial action. For applications that need
// reliable accurate timestamps (e.g. distributed databases), the structure
// can be mapped all the way to userspace. This allows the application to see
// directly for itself that the clock is disrupted and take appropriate
// action, even when using a vDSO-style method to get the time instead of a
// system call.
//
// In its more advanced mode. this structure can also be used to expose the
// precise relationship of the CPU counter to real time, as calibrated by the
// host. This means that userspace applications can have accurate time
// immediately after live migration, rather than having to pause operations
// and wait for NTP to recover. This mode does, of course, rely on the
// counter being reliable and consistent across CPUs.
//
// Note that this must be true UTC, never with smeared leap seconds. If a
// guest wishes to construct a smeared clock, it can do so. Presenting a
// smeared clock through this interface would be problematic because it
// actually messes with the apparent counter *period*. A linear smearing
// of 1 ms per second would effectively tweak the counter period by 1000PPM
// at the start/end of the smearing period, while a sinusoidal smear would
// basically be impossible to represent.
//
// This structure is offered with the intent that it be adopted into the
// nascent virtio-rtc standard, as a virtio-rtc that does not address the live
// migration problem seems a little less than fit for purpose. For that
// reason, certain fields use precisely the same numeric definitions as in
// the virtio-rtc proposal. The structure can also be exposed through an ACPI
// device with the CID "VMCLOCK", modelled on the "VMGENID" device except for
// the fact that it uses a real _CRS to convey the address of the structure
// (which should be a full page, to allow for mapping directly to userspace).
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmclock_abi {
// CONSTANT FIELDS
    pub magic: __le32,
pub const VMCLOCK_MAGIC: c_uint = 0x4b4c4356 /* "VCLK" */;
    pub /: *mut *mut __le32 size; / Size of region containing this structure,
    pub /: *mut *mut __le16 version; / 1,
    pub /: *mut *mut __u8 counter_id; / Matches VIRTIO_RTC_COUNTER_xxx except INVALID,
pub const VMCLOCK_COUNTER_ARM_VCNT: c_int = 0;
pub const VMCLOCK_COUNTER_X86_TSC: c_int = 1;
pub const VMCLOCK_COUNTER_INVALID: c_uint = 0xff;
    pub /: *mut *mut __u8 time_type; / Matches VIRTIO_RTC_TYPE_xxx,

// NON-CONSTANT FIELDS PROTECTED BY SEQCOUNT LOCK
    pub /: *mut *mut __le32 seq_count; / Low bit means an update is in progress,
//
// This field changes to another non-repeating value when the CPU
// counter is disrupted, for example on live migration. This lets
// the guest know that it should discard any calibration it has
// performed of the counter against external sources (NTP/PTP/etc.).
//
    pub disruption_marker: __le64,
    pub flags: __le64,
// Indicates that the tai_offset_sec field is valid

//
// Optionally used to notify guests of pending maintenance events.
// A guest which provides latency-sensitive services may wish to
// remove itself from service if an event is coming up. Two flags
// indicate the approximate imminence of the event.
//

//
// If the MONOTONIC flag is set then (other than leap seconds) it is
// guaranteed that the time calculated according this structure at
// any given moment shall never appear to be later than the time
// calculated via the structure at any *later* moment.
//
// In particular, a timestamp based on a counter reading taken
// immediately after setting the low bit of seq_count (and the
// associated memory barrier), using the previously-valid time and
// period fields, shall never be later than a timestamp based on
// a counter reading taken immediately before *clearing* the low
// bit again after the update, using the about-to-be-valid fields.
//

//
// If the VM_GEN_COUNTER_PRESENT flag is set, the hypervisor will
// bump the vm_generation_counter field every time the guest is
// loaded from some save state (restored from a snapshot).
//

//
// If the NOTIFICATION_PRESENT flag is set, the hypervisor will send
// a notification every time it updates seq_count to a new even number.
//
    pub pad: [__u8; 2],
    pub clock_status: __u8,
pub const VMCLOCK_STATUS_UNKNOWN: c_int = 0;
pub const VMCLOCK_STATUS_INITIALIZING: c_int = 1;
pub const VMCLOCK_STATUS_SYNCHRONIZED: c_int = 2;
pub const VMCLOCK_STATUS_FREERUNNING: c_int = 3;
pub const VMCLOCK_STATUS_UNRELIABLE: c_int = 4;
//
// The time exposed through this device is never smeared. This field
// corresponds to the 'subtype' field in virtio-rtc, which indicates
// the smearing method. However in this case it provides a *hint* to
// the guest operating system, such that *if* the guest OS wants to
// provide its users with an alternative clock which does not follow
// UTC, it may do so in a fashion consistent with the other systems
// in the nearby environment.
//
    pub /: *mut *mut __u8 leap_second_smearing_hint; / Matches VIRTIO_RTC_SUBTYPE_xxx,
pub const VMCLOCK_SMEARING_STRICT: c_int = 0;
pub const VMCLOCK_SMEARING_NOON_LINEAR: c_int = 1;
pub const VMCLOCK_SMEARING_UTC_SLS: c_int = 2;
    pub /: *mut *mut __le16 tai_offset_sec; / Actually two's complement signed,
    pub leap_indicator: __u8,
//
// This field is based on the VIRTIO_RTC_LEAP_xxx values as defined
// in the current draft of virtio-rtc, but since smearing cannot be
// used with the shared memory device, some values are not used.
//
// The _POST_POS and _POST_NEG values allow the guest to perform
// its own smearing during the day or so after a leap second when
// such smearing may need to continue being applied for a leap
// second which is now theoretically "historical".
//
pub const VMCLOCK_LEAP_NONE: c_uint = 0x00	/* No known nearby leap second */;
pub const VMCLOCK_LEAP_PRE_POS: c_uint = 0x01	/* Positive leap second at EOM */;
pub const VMCLOCK_LEAP_PRE_NEG: c_uint = 0x02	/* Negative leap second at EOM */;
pub const VMCLOCK_LEAP_POS: c_uint = 0x03	/* Set during 23:59:60 second */;
pub const VMCLOCK_LEAP_POST_POS: c_uint = 0x04;
pub const VMCLOCK_LEAP_POST_NEG: c_uint = 0x05;
// Bit shift for counter_period_frac_sec and its error rate
    pub counter_period_shift: __u8,
//
// Paired values of counter and UTC at a given point in time.
//
    pub counter_value: __le64,
//
// Counter period, and error margin of same. The unit of these
// fields is 1/2^(64 + counter_period_shift) of a second.
//
    pub counter_period_frac_sec: __le64,
    pub counter_period_esterror_rate_frac_sec: __le64,
    pub counter_period_maxerror_rate_frac_sec: __le64,
//
// Time according to time_type field above.
//
    pub /: *mut *mut __le64 time_sec; / Seconds since time_type epoch,
    pub /: *mut *mut __le64 time_frac_sec; / Units of 1/2^64 of a second,
    pub time_esterror_nanosec: __le64,
    pub time_maxerror_nanosec: __le64,
//
// This field changes to another non-repeating value when the guest
// has been loaded from a snapshot. In addition to handling a
// disruption in time (which will also be signalled through the
// disruption_marker field), a guest may wish to discard UUIDs,
// reset network connections, reseed entropy, etc.
//
    pub vm_generation_counter: __le64,
}
