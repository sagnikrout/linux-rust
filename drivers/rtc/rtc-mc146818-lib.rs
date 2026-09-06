//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-mc146818-lib.c
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


// SPDX-License-Identifier: GPL-2.0-only

//
// Execute a function while the UIP (Update-in-progress) bit of the RTC is
// unset. The timeout is configurable by the caller in ms.
//
// Warning: callback may be executed more then once.
//
    bool mc146818_avoid_UIP(void (*callback)(unsigned char seconds, void *param),
    int timeout,
    void *param)
    {
    int i;
    unsigned long flags;
    unsigned char seconds;
    for (i = 0; UIP_RECHECK_LOOPS_MS(i) < timeout; i++) {
    spin_lock_irqsave(&rtc_lock, flags);
//
// Check whether there is an update in progress during which the
// readout is unspecified. The maximum update time is ~2ms. Poll
// for completion.
//
// Store the second value before checking UIP so a long lasting
// NMI which happens to hit after the UIP check cannot make
// an update cycle invisible.
//
    seconds = CMOS_READ(RTC_SECONDS);
    if (CMOS_READ(RTC_FREQ_SELECT) & RTC_UIP) {
    spin_unlock_irqrestore(&rtc_lock, flags);
    udelay(UIP_RECHECK_DELAY);
    continue;
    }
// Revalidate the above readout
    if (seconds != CMOS_READ(RTC_SECONDS)) {
    spin_unlock_irqrestore(&rtc_lock, flags);
    continue;
    }
    if (callback)
    callback(seconds, param);
//
// Check for the UIP bit again. If it is set now then
// the above values may contain garbage.
//
    if (CMOS_READ(RTC_FREQ_SELECT) & RTC_UIP) {
    spin_unlock_irqrestore(&rtc_lock, flags);
    udelay(UIP_RECHECK_DELAY);
    continue;
    }
//
// A NMI might have interrupted the above sequence so check
// whether the seconds value has changed which indicates that
// the NMI took longer than the UIP bit was set. Unlikely, but
// possible and there is also virt...
//
    if (seconds != CMOS_READ(RTC_SECONDS)) {
    spin_unlock_irqrestore(&rtc_lock, flags);
    continue;
    }
    spin_unlock_irqrestore(&rtc_lock, flags);
    if (UIP_RECHECK_LOOPS_MS(i) >= 100)
    pr_warn("Reading current time from RTC took around %li ms\n",
    UIP_RECHECK_LOOPS_MS(i));
    return true;
    }
    return false;
    }
    EXPORT_SYMBOL_GPL(mc146818_avoid_UIP);
//
// If the UIP (Update-in-progress) bit of the RTC is set for more then
// 10ms, the RTC is apparently broken or not present.
//
#[no_mangle]
pub unsafe extern "C" fn mc146818_does_rtc_work() -> bool {
    bool mc146818_does_rtc_work(void)
    {
    return mc146818_avoid_UIP(core::ptr::null_mut(), 1000, core::ptr::null_mut());
    }
    EXPORT_SYMBOL_GPL(mc146818_does_rtc_work);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mc146818_get_time_callback_param {
    pub time: *mut rtc_time,
    pub ctrl: c_uchar,

    pub century: c_uchar,

    pub real_year: c_uint,

}

#[no_mangle]
unsafe extern "C" fn mc146818_get_time_callback(seconds: c_uchar, param_in: *mut c_void) {
    static void mc146818_get_time_callback(unsigned char seconds, void *param_in)
    {
    struct mc146818_get_time_callback_param *p = param_in;
//
// Only the values that we read from the RTC are set. We leave
// tm_wday, tm_yday and tm_isdst untouched. Even though the
// RTC has RTC_DAY_OF_WEEK, we ignore it, as it is only updated
// by the RTC when initially set to a non-zero value.
//
    p.time.tm_sec = seconds;
    p.time.tm_min = CMOS_READ(RTC_MINUTES);
    p.time.tm_hour = CMOS_READ(RTC_HOURS);
    p.time.tm_mday = CMOS_READ(RTC_DAY_OF_MONTH);
    p.time.tm_mon = CMOS_READ(RTC_MONTH);
    p.time.tm_year = CMOS_READ(RTC_YEAR);

    p.real_year = CMOS_READ(RTC_DEC_YEAR);

    if (acpi_gbl_FADT.header.revision >= FADT2_REVISION_ID &&
    acpi_gbl_FADT.century) {
    p.century = CMOS_READ(acpi_gbl_FADT.century);
    } else {
    p.century = 0;
    }

    p.ctrl = CMOS_READ(RTC_CONTROL);
    }
//
// mc146818_get_time - Get the current time from the RTC
// @time: pointer to struct rtc_time to store the current time
// @timeout: timeout value in ms
//
// This function reads the current time from the RTC and stores it in the
// provided struct rtc_time. The timeout parameter specifies the maximum
// time to wait for the RTC to become ready.
//
// Return: 0 on success, -ETIMEDOUT if the RTC did not become ready within
// the specified timeout, or another error code if an error occurred.
//
#[no_mangle]
pub unsafe extern "C" fn mc146818_get_time(time: *mut rtc_time, timeout: c_int) -> c_int {
    int mc146818_get_time(struct rtc_time *time, int timeout)
    {
    struct mc146818_get_time_callback_param p = {
    .time = time
    };
    if (!mc146818_avoid_UIP(mc146818_get_time_callback, timeout, &p)) {
    memset(time, 0, sizeof(*time));
    return -ETIMEDOUT;
    }
    if (!(p.ctrl & RTC_DM_BINARY) || RTC_ALWAYS_BCD)
    {
    time.tm_sec = bcd2bin(time.tm_sec);
    time.tm_min = bcd2bin(time.tm_min);
    time.tm_hour = bcd2bin(time.tm_hour);
    time.tm_mday = bcd2bin(time.tm_mday);
    time.tm_mon = bcd2bin(time.tm_mon);
    time.tm_year = bcd2bin(time.tm_year);

    p.century = bcd2bin(p.century);

    }

    time.tm_year += p.real_year - 72;

    if (p.century > 19)
    time.tm_year += (p.century - 19) * 100;

//
// Account for differences between how the RTC uses the values
// and how they are defined in a struct rtc_time;
//
    if (time.tm_year <= 69)
    time.tm_year += 100;
    time.tm_mon--;
    return 0;
    }
    EXPORT_SYMBOL_GPL(mc146818_get_time);
// AMD systems don't allow access to AltCentury with DV1
#[no_mangle]
unsafe extern "C" fn apply_amd_register_a_behavior() -> bool {
    static bool apply_amd_register_a_behavior(void)
    {

    if (boot_cpu_data.x86_vendor == X86_VENDOR_AMD ||
    boot_cpu_data.x86_vendor == X86_VENDOR_HYGON)
    return true;

    return false;
    }
// Set the current date and time in the real time clock.
#[no_mangle]
pub unsafe extern "C" fn mc146818_set_time(time: *mut rtc_time) -> c_int {
    int mc146818_set_time(struct rtc_time *time)
    {
    unsigned long flags;
    unsigned char mon, day, hrs, min, sec;
    unsigned char save_control, save_freq_select;
    unsigned int yrs;

    unsigned int real_yrs;

    let mut century: c_uchar = 0;
    yrs = time.tm_year;
    mon = time.tm_mon + 1;   /* tm_mon starts at zero */
    day = time.tm_mday;
    hrs = time.tm_hour;
    min = time.tm_min;
    sec = time.tm_sec;
    if (yrs > 255)	/* They are unsigned */
    return -EINVAL;

    real_yrs = yrs;
    yrs = 72;
//
// We want to keep the year set to 73 until March
// for non-leap years, so that Feb, 29th is handled
// correctly.
//
    if (!is_leap_year(real_yrs + 1900) && mon < 3) {
    real_yrs--;
    yrs = 73;
    }

    if (acpi_gbl_FADT.header.revision >= FADT2_REVISION_ID &&
    acpi_gbl_FADT.century) {
    century = (yrs + 1900) / 100;
    yrs %= 100;
    }

// These limits and adjustments are independent of
// whether the chip is in binary mode or not.
//
    if (yrs > 169)
    return -EINVAL;
    if (yrs >= 100)
    yrs -= 100;
    spin_lock_irqsave(&rtc_lock, flags);
    save_control = CMOS_READ(RTC_CONTROL);
    spin_unlock_irqrestore(&rtc_lock, flags);
    if (!(save_control & RTC_DM_BINARY) || RTC_ALWAYS_BCD) {
    sec = bin2bcd(sec);
    min = bin2bcd(min);
    hrs = bin2bcd(hrs);
    day = bin2bcd(day);
    mon = bin2bcd(mon);
    yrs = bin2bcd(yrs);
    century = bin2bcd(century);
    }
    spin_lock_irqsave(&rtc_lock, flags);
    save_control = CMOS_READ(RTC_CONTROL);
    CMOS_WRITE((save_control|RTC_SET), RTC_CONTROL);
    save_freq_select = CMOS_READ(RTC_FREQ_SELECT);
    if (apply_amd_register_a_behavior())
    CMOS_WRITE((save_freq_select & ~RTC_AMD_BANK_SELECT), RTC_FREQ_SELECT);
    else
    CMOS_WRITE((save_freq_select|RTC_DIV_RESET2), RTC_FREQ_SELECT);

    CMOS_WRITE(real_yrs, RTC_DEC_YEAR);

    CMOS_WRITE(yrs, RTC_YEAR);
    CMOS_WRITE(mon, RTC_MONTH);
    CMOS_WRITE(day, RTC_DAY_OF_MONTH);
    CMOS_WRITE(hrs, RTC_HOURS);
    CMOS_WRITE(min, RTC_MINUTES);
    CMOS_WRITE(sec, RTC_SECONDS);

    if (acpi_gbl_FADT.header.revision >= FADT2_REVISION_ID &&
    acpi_gbl_FADT.century)
    CMOS_WRITE(century, acpi_gbl_FADT.century);

    CMOS_WRITE(save_control, RTC_CONTROL);
    CMOS_WRITE(save_freq_select, RTC_FREQ_SELECT);
    spin_unlock_irqrestore(&rtc_lock, flags);
    return 0;
    }
    EXPORT_SYMBOL_GPL(mc146818_set_time);
