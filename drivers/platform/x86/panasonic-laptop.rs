//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/panasonic-laptop.c
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
// Panasonic HotKey and LCD brightness control driver
// (C) 2004 Hiroshi Miura <miura@da-cha.org>
// (C) 2004 NTT DATA Intellilink Co. http://www.intellilink.co.jp
// (C) YOKOTA Hiroshi <yokota (at) netlab. is. tsukuba. ac. jp>
// (C) 2004 David Bronaugh <dbronaugh>
// (C) 2006-2008 Harald Welte <laforge@gnumonks.org>
//
// derived from toshiba_acpi.c, Copyright (C) 2002-2004 John Belmonte
//
// ---------------------------------------------------------------------------
//
// ChangeLog:
// Aug.18, 2020	Kenneth Chan <kenneth.t.chan@gmail.com>
// -v0.98	add platform devices for firmware brightness registers
// add support for battery charging threshold (eco mode)
// resolve hotkey double trigger
// add write support to mute
// fix sticky_key init bug
// fix naming of platform files for consistency with other
// modules
// split MODULE_AUTHOR() by one author per macro call
// replace ACPI prints with pr_*() macros
// -v0.97	add support for cdpower hardware switch
// -v0.96	merge Lucina's enhancement
// Jan.13, 2009 Martin Lucina <mato@kotelna.sk>
// - add support for optical driver power in
// Y and W series
//
// Sep.23, 2008	Harald Welte <laforge@gnumonks.org>
// -v0.95	rename driver from drivers/acpi/pcc_acpi.c to
// drivers/misc/panasonic-laptop.c
//
// Jul.04, 2008	Harald Welte <laforge@gnumonks.org>
// -v0.94	replace /proc interface with device attributes
// support {set,get}keycode on th input device
//
// Jun.27, 2008	Harald Welte <laforge@gnumonks.org>
// -v0.92	merge with 2.6.26-rc6 input API changes
// remove broken <= 2.6.15 kernel support
// resolve all compiler warnings
// various coding style fixes (checkpatch.pl)
// add support for backlight api
// major code restructuring
//
// Dac.28, 2007	Harald Welte <laforge@gnumonks.org>
// -v0.91	merge with 2.6.24-rc6 ACPI changes
//
// Nov.04, 2006	Hiroshi Miura <miura@da-cha.org>
// -v0.9	remove warning about section reference.
// remove acpi_os_free
// add /proc/acpi/pcc/brightness interface for HAL access
// merge dbronaugh's enhancement
// Aug.17, 2004 David Bronaugh (dbronaugh)
// - Added screen brightness setting interface
// Thanks to FreeBSD crew (acpi_panasonic.c)
// for the ideas I needed to accomplish it
//
// May.29, 2006	Hiroshi Miura <miura@da-cha.org>
// -v0.8.4 follow to change keyinput structure
// thanks Fabian Yamaguchi <fabs@cs.tu-berlin.de>,
// Jacob Bower <jacob.bower@ic.ac.uk> and
// Hiroshi Yokota for providing solutions.
//
// Oct.02, 2004	Hiroshi Miura <miura@da-cha.org>
// -v0.8.2	merge code of YOKOTA Hiroshi
// <yokota@netlab.is.tsukuba.ac.jp>.
// Add sticky key mode interface.
// Refactoring acpi_pcc_generate_keyinput().
//
// Sep.15, 2004	Hiroshi Miura <miura@da-cha.org>
// -v0.8	Generate key input event on input subsystem.
// This is based on yet another driver written by
// Ryuta Nakanishi.
//
// Sep.10, 2004	Hiroshi Miura <miura@da-cha.org>
// -v0.7	Change proc interface functions using seq_file
// facility as same as other ACPI drivers.
//
// Aug.28, 2004	Hiroshi Miura <miura@da-cha.org>
// -v0.6.4 Fix a silly error with status checking
//
// Aug.25, 2004	Hiroshi Miura <miura@da-cha.org>
// -v0.6.3 replace read_acpi_int by standard function
// acpi_evaluate_integer
// some clean up and make smart copyright notice.
// fix return value of pcc_acpi_get_key()
// fix checking return value of acpi_bus_register_driver()
//
// Aug.22, 2004    David Bronaugh <dbronaugh@linuxboxen.org>
// -v0.6.2 Add check on ACPI data (num_sifr)
// Coding style cleanups, better error messages/handling
// Fixed an off-by-one error in memory allocation
//
// Aug.21, 2004    David Bronaugh <dbronaugh@linuxboxen.org>
// -v0.6.1 Fix a silly error with status checking
//
// Aug.20, 2004    David Bronaugh <dbronaugh@linuxboxen.org>
// - v0.6  Correct brightness controls to reflect reality
// based on information gleaned by Hiroshi Miura
// and discussions with Hiroshi Miura
//
// Aug.10, 2004	Hiroshi Miura <miura@da-cha.org>
// - v0.5  support LCD brightness control
// based on the disclosed information by MEI.
//
// Jul.25, 2004	Hiroshi Miura <miura@da-cha.org>
// - v0.4  first post version
// add function to retrive SIFR
//
// Jul.24, 2004	Hiroshi Miura <miura@da-cha.org>
// - v0.3  get proper status of hotkey
//
// Jul.22, 2004	Hiroshi Miura <miura@da-cha.org>
// - v0.2  add HotKey handler
//
// Jul.17, 2004	Hiroshi Miura <miura@da-cha.org>
// - v0.1  start from toshiba_acpi driver written by John Belmonte
//

    MODULE_AUTHOR("Hiroshi Miura <miura@da-cha.org>");
    MODULE_AUTHOR("David Bronaugh <dbronaugh@linuxboxen.org>");
    MODULE_AUTHOR("Harald Welte <laforge@gnumonks.org>");
    MODULE_AUTHOR("Martin Lucina <mato@kotelna.sk>");
    MODULE_AUTHOR("Kenneth Chan <kenneth.t.chan@gmail.com>");
    MODULE_DESCRIPTION("ACPI HotKey driver for Panasonic Let's Note laptops");
    MODULE_LICENSE("GPL");

// Define ACPI PATHs
// Lets note hotkeys

pub const HKEY_NOTIFY: c_uint = 0x80;
pub const ECO_MODE_OFF: c_uint = 0x00;
pub const ECO_MODE_ON: c_uint = 0x80;

// LCD_TYPEs: 0 = Normal, 1 = Semi-transparent
    ECO_MODEs: 0x03 = off, 0x83 = on
//
    enum SINF_BITS { SINF_NUM_BATTERIES = 0,
    SINF_LCD_TYPE,
    SINF_AC_MAX_BRIGHT,
    SINF_AC_MIN_BRIGHT,
    SINF_AC_CUR_BRIGHT,
    SINF_DC_MAX_BRIGHT,
    SINF_DC_MIN_BRIGHT,
    SINF_DC_CUR_BRIGHT,
    SINF_MUTE,
    SINF_RESERVED,
    SINF_ECO_MODE = 0x0A,
    SINF_CUR_BRIGHT = 0x0D,
    SINF_STICKY_KEY = 0x80,
    };
// R1 handles SINF_AC_CUR_BRIGHT as SINF_CUR_BRIGHT, doesn't know AC state
    static int acpi_pcc_hotkey_probe(struct platform_device *pdev);
    static void acpi_pcc_hotkey_remove(struct platform_device *pdev);
    static void acpi_pcc_hotkey_notify(acpi_handle handle, u32 event, void *data);
    static const struct acpi_device_id pcc_device_ids[] = {
    { "MAT0012", 0},
    { "MAT0013", 0},
    { "MAT0018", 0},
    { "MAT0019", 0},
    { "", 0},
    };
    MODULE_DEVICE_TABLE(acpi, pcc_device_ids);

    static int acpi_pcc_hotkey_resume(struct device *dev);

    static SIMPLE_DEV_PM_OPS(acpi_pcc_hotkey_pm, core::ptr::null_mut(), acpi_pcc_hotkey_resume);
    static struct platform_driver acpi_pcc_driver = {
    .probe = acpi_pcc_hotkey_probe,
    .remove = acpi_pcc_hotkey_remove,
    .driver = {
    .name = ACPI_PCC_DRIVER_NAME,
    .acpi_match_table = pcc_device_ids,
    .pm = &acpi_pcc_hotkey_pm,
    },
    };
    static const struct key_entry panasonic_keymap[] = {
    { KE_KEY, 0, { KEY_RESERVED } },
    { KE_KEY, 1, { KEY_BRIGHTNESSDOWN } },
    { KE_KEY, 2, { KEY_BRIGHTNESSUP } },
    { KE_KEY, 3, { KEY_DISPLAYTOGGLE } },
    { KE_KEY, 4, { KEY_MUTE } },
    { KE_KEY, 5, { KEY_VOLUMEDOWN } },
    { KE_KEY, 6, { KEY_VOLUMEUP } },
    { KE_KEY, 7, { KEY_SLEEP } },
    { KE_KEY, 8, { KEY_PROG1 } }, /* Change CPU boost */
    { KE_KEY, 9, { KEY_BATTERY } },
    { KE_KEY, 10, { KEY_SUSPEND } },
    { KE_KEY, 21, { KEY_MACRO1 } },
    { KE_KEY, 22, { KEY_MACRO2 } },
    { KE_KEY, 24, { KEY_MACRO3 } },
    { KE_KEY, 25, { KEY_MACRO4 } },
    { KE_KEY, 34, { KEY_MACRO5 } },
    { KE_KEY, 35, { KEY_MACRO6 } },
    { KE_KEY, 36, { KEY_MACRO7 } },
    { KE_KEY, 37, { KEY_MACRO8 } },
    { KE_KEY, 41, { KEY_MACRO9 } },
    { KE_KEY, 42, { KEY_MACRO10 } },
    { KE_KEY, 43, { KEY_MACRO11 } },
    { KE_END, 0 }
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcc_acpi {
    pub handle: acpi_handle,
    pub num_sifr: c_ulong,
    pub sticky_key: c_int,
    pub eco_mode: c_int,
    pub mute: c_int,
    pub ac_brightness: c_int,
    pub dc_brightness: c_int,
    pub current_brightness: c_int,
    pub device: *mut acpi_device,
    pub input_dev: *mut input_dev,
    pub backlight: *mut backlight_device,
    pub platform: *mut platform_device,
    pub __counted_by(num_sifr): u32 sinf[],
}

//
// On some Panasonic models the volume up / down / mute keys send duplicate
// keypress events over the PS/2 kbd interface, filter these out.
//
    static bool panasonic_i8042_filter(unsigned char data, unsigned char str,
    struct serio *port, void *context)
    {
    static bool extended;
    if (str & I8042_STR_AUXDATA)
    return false;
    if (data == 0xe0) {
    extended = true;
    return true;
    } else if (extended) {
    extended = false;
    switch (data & 0x7f) {
    case 0x20: /* e0 20 / e0 a0, Volume Mute press / release */
    case 0x2e: /* e0 2e / e0 ae, Volume Down press / release */
    case 0x30: /* e0 30 / e0 b0, Volume Up press / release */
    return true;
    default:
//
// Report the previously filtered e0 before continuing
// with the next non-filtered byte.
//
    serio_interrupt(port, 0xe0, 0);
    return false;
    }
    }
    return false;
    }
// method access functions
#[no_mangle]
unsafe extern "C" fn acpi_pcc_write_sset(pcc: *mut pcc_acpi, func: c_int, val: c_int) -> c_int {
    static int acpi_pcc_write_sset(struct pcc_acpi *pcc, int func, int val)
    {
    union acpi_object in_objs[] = {
    { .integer.type  = ACPI_TYPE_INTEGER,
    .integer.value = func, },
    { .integer.type  = ACPI_TYPE_INTEGER,
    .integer.value = val, },
    };
    struct acpi_object_list params = {
    .count   = ARRAY_SIZE(in_objs),
    .pointer = in_objs,
    };
    let mut status: acpi_status = AE_OK;
    status = acpi_evaluate_object(pcc.handle, METHOD_HKEY_SSET,
    &params, core::ptr::null_mut());
    return (status == AE_OK) ? 0 : -EIO;
    }
#[no_mangle]
pub unsafe extern "C" fn acpi_pcc_get_sqty(device: *mut acpi_device) -> c_int {
    static inline int acpi_pcc_get_sqty(struct acpi_device *device)
    {
    unsigned long long s;
    acpi_status status;
    status = acpi_evaluate_integer(device.handle, METHOD_HKEY_SQTY,
    core::ptr::null_mut(), &s);
    if (ACPI_SUCCESS(status))
    return s;
    else {
    pr_err("evaluation error HKEY.SQTY\n");
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn acpi_pcc_retrieve_biosdata(pcc: *mut pcc_acpi) -> c_int {
    static int acpi_pcc_retrieve_biosdata(struct pcc_acpi *pcc)
    {
    acpi_status status;
    let mut buffer: acpi_buffer = {ACPI_ALLOCATE_BUFFER, core::ptr::null_mut()};
    union acpi_object *hkey = core::ptr::null_mut();
    int i;
    status = acpi_evaluate_object(pcc.handle, METHOD_HKEY_SINF, core::ptr::null_mut(),
    &buffer);
    if (ACPI_FAILURE(status)) {
    pr_err("evaluation error HKEY.SINF\n");
    return 0;
    }
    hkey = buffer.pointer;
    if (!hkey || (hkey.type != ACPI_TYPE_PACKAGE)) {
    pr_err("Invalid HKEY.SINF\n");
    status = AE_ERROR;
    goto end;
    }
    if (pcc.num_sifr < hkey.package.count) {
    pr_err("SQTY reports bad SINF length SQTY: %lu SINF-pkg-count: %u\n",
    pcc.num_sifr, hkey.package.count);
    status = AE_ERROR;
    goto end;
    }
    for (i = 0; i < hkey.package.count; i++) {
    union acpi_object *element = &(hkey.package.elements[i]);
    if (likely(element.type == ACPI_TYPE_INTEGER)) {
    pcc.sinf[i] = element.integer.value;
    } else
    pr_err("Invalid HKEY.SINF data\n");
    }
//
// pcc->sinf[] has pcc->num_sifr elements (valid indices
// 0..num_sifr-1). On DSDTs where SINF's package count equals
// num_sifr exactly -- the off-by-one case probe()'s num_sifr++
// already allocates a spare element for -- there is no room left
// for this trailing sentinel; nothing reads it back, so just skip
// the write rather than running one element past the flex array.
//
    if (hkey.package.count < pcc.num_sifr)
    pcc.sinf[hkey.package.count] = -1;
    end:
    kfree(buffer.pointer);
    let mut status: return = = AE_OK;
    }
// backlight API interface functions
// This driver currently treats AC and DC brightness identical,
// since we don't need to invent an interface to the core ACPI
// logic to receive events in case a power supply is plugged in
// or removed
#[no_mangle]
unsafe extern "C" fn bl_get(bd: *mut backlight_device) -> c_int {
    static int bl_get(struct backlight_device *bd)
    {
    struct pcc_acpi *pcc = bl_get_data(bd);
    if (!acpi_pcc_retrieve_biosdata(pcc))
    return -EIO;
    return pcc.sinf[SINF_AC_CUR_BRIGHT];
    }
#[no_mangle]
unsafe extern "C" fn bl_set_status(bd: *mut backlight_device) -> c_int {
    static int bl_set_status(struct backlight_device *bd)
    {
    struct pcc_acpi *pcc = bl_get_data(bd);
    let mut bright: c_int = bd.props.brightness;
    int rc;
    if (!acpi_pcc_retrieve_biosdata(pcc))
    return -EIO;
    if (bright < pcc.sinf[SINF_AC_MIN_BRIGHT])
    bright = pcc.sinf[SINF_AC_MIN_BRIGHT];
    if (bright < pcc.sinf[SINF_DC_MIN_BRIGHT])
    bright = pcc.sinf[SINF_DC_MIN_BRIGHT];
    if (bright < pcc.sinf[SINF_AC_MIN_BRIGHT] ||
    bright > pcc.sinf[SINF_AC_MAX_BRIGHT])
    return -EINVAL;
    rc = acpi_pcc_write_sset(pcc, SINF_AC_CUR_BRIGHT, bright);
    if (rc < 0)
    return rc;
    return acpi_pcc_write_sset(pcc, SINF_DC_CUR_BRIGHT, bright);
    }
    static const struct backlight_ops pcc_backlight_ops = {
    .get_brightness	= bl_get,
    .update_status	= bl_set_status,
    };
// returns ACPI_SUCCESS if methods to control optical drive are present
#[no_mangle]
unsafe extern "C" fn check_optd_present() -> acpi_status {
    static acpi_status check_optd_present(void)
    {
    let mut status: acpi_status = AE_OK;
    acpi_handle handle;
    status = acpi_get_handle(core::ptr::null_mut(), "\\_SB.STAT", &handle);
    if (ACPI_FAILURE(status))
    goto out;
    status = acpi_get_handle(core::ptr::null_mut(), "\\_SB.FBAY", &handle);
    if (ACPI_FAILURE(status))
    goto out;
    status = acpi_get_handle(core::ptr::null_mut(), "\\_SB.CDDI", &handle);
    if (ACPI_FAILURE(status))
    goto out;
    out:
    return status;
    }
// get optical driver power state
#[no_mangle]
unsafe extern "C" fn get_optd_power_state() -> c_int {
    static int get_optd_power_state(void)
    {
    acpi_status status;
    unsigned long long state;
    int result;
    status = acpi_evaluate_integer(core::ptr::null_mut(), "\\_SB.STAT", core::ptr::null_mut(), &state);
    if (ACPI_FAILURE(status)) {
    pr_err("evaluation error _SB.STAT\n");
    result = -EIO;
    goto out;
    }
    switch (state) {
    case 0: /* power off */
    result = 0;
    break;
    case 0x0f: /* power on */
    result = 1;
    break;
    default:
    result = -EIO;
    break;
    }
    out:
    return result;
    }
// set optical drive power state
#[no_mangle]
unsafe extern "C" fn set_optd_power_state(new_state: c_int) -> c_int {
    static int set_optd_power_state(int new_state)
    {
    int result;
    acpi_status status;
    result = get_optd_power_state();
    if (result < 0)
    goto out;
    if (new_state == result)
    goto out;
    switch (new_state) {
    case 0: /* power off */
// Call CDDR instead, since they both call the same method
// while CDDI takes 1 arg and we are not quite sure what it is.
//
    status = acpi_evaluate_object(core::ptr::null_mut(), "\\_SB.CDDR", core::ptr::null_mut(), core::ptr::null_mut());
    if (ACPI_FAILURE(status)) {
    pr_err("evaluation error _SB.CDDR\n");
    result = -EIO;
    }
    break;
    case 1: /* power on */
    status = acpi_evaluate_object(core::ptr::null_mut(), "\\_SB.FBAY", core::ptr::null_mut(), core::ptr::null_mut());
    if (ACPI_FAILURE(status)) {
    pr_err("evaluation error _SB.FBAY\n");
    result = -EIO;
    }
    break;
    default:
    result = -EINVAL;
    break;
    }
    out:
    return result;
    }
// sysfs user interface functions
    static ssize_t numbatt_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct acpi_device *acpi = to_acpi_device(dev);
    struct pcc_acpi *pcc = acpi_driver_data(acpi);
    if (!acpi_pcc_retrieve_biosdata(pcc))
    return -EIO;
    return sysfs_emit(buf, "%u\n", pcc.sinf[SINF_NUM_BATTERIES]);
    }
    static ssize_t lcdtype_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct acpi_device *acpi = to_acpi_device(dev);
    struct pcc_acpi *pcc = acpi_driver_data(acpi);
    if (!acpi_pcc_retrieve_biosdata(pcc))
    return -EIO;
    return sysfs_emit(buf, "%u\n", pcc.sinf[SINF_LCD_TYPE]);
    }
    static ssize_t mute_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct acpi_device *acpi = to_acpi_device(dev);
    struct pcc_acpi *pcc = acpi_driver_data(acpi);
    if (!acpi_pcc_retrieve_biosdata(pcc))
    return -EIO;
    return sysfs_emit(buf, "%u\n", pcc.sinf[SINF_MUTE]);
    }
    static ssize_t mute_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct acpi_device *acpi = to_acpi_device(dev);
    struct pcc_acpi *pcc = acpi_driver_data(acpi);
    int err, val;
    err = kstrtoint(buf, 0, &val);
    if (err)
    return err;
    if (val == 0 || val == 1) {
    acpi_pcc_write_sset(pcc, SINF_MUTE, val);
    pcc.mute = val;
    }
    return count;
    }
    static ssize_t sticky_key_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct acpi_device *acpi = to_acpi_device(dev);
    struct pcc_acpi *pcc = acpi_driver_data(acpi);
    if (!acpi_pcc_retrieve_biosdata(pcc))
    return -EIO;
    return sysfs_emit(buf, "%u\n", pcc.sticky_key);
    }
    static ssize_t sticky_key_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct acpi_device *acpi = to_acpi_device(dev);
    struct pcc_acpi *pcc = acpi_driver_data(acpi);
    int err, val;
    err = kstrtoint(buf, 0, &val);
    if (err)
    return err;
    if (val == 0 || val == 1) {
    acpi_pcc_write_sset(pcc, SINF_STICKY_KEY, val);
    pcc.sticky_key = val;
    }
    return count;
    }
    static ssize_t eco_mode_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct acpi_device *acpi = to_acpi_device(dev);
    struct pcc_acpi *pcc = acpi_driver_data(acpi);
    int result;
    if (!acpi_pcc_retrieve_biosdata(pcc))
    return -EIO;
    switch (pcc.sinf[SINF_ECO_MODE]) {
    case (ECO_MODE_OFF + 3):
    result = 0;
    break;
    case (ECO_MODE_ON + 3):
    result = 1;
    break;
    default:
    return -EIO;
    }
    return sysfs_emit(buf, "%u\n", result);
    }
    static ssize_t eco_mode_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct acpi_device *acpi = to_acpi_device(dev);
    struct pcc_acpi *pcc = acpi_driver_data(acpi);
    int err, state;
    union acpi_object param[2];
    struct acpi_object_list input;
    acpi_status status;
    param[0].type = ACPI_TYPE_INTEGER;
    param[0].integer.value = 0x15;
    param[1].type = ACPI_TYPE_INTEGER;
    input.count = 2;
    input.pointer = param;
    err = kstrtoint(buf, 0, &state);
    if (err)
    return err;
    switch (state) {
    case 0:
    param[1].integer.value = ECO_MODE_OFF;
    pcc.sinf[SINF_ECO_MODE] = 0;
    pcc.eco_mode = 0;
    break;
    case 1:
    param[1].integer.value = ECO_MODE_ON;
    pcc.sinf[SINF_ECO_MODE] = 1;
    pcc.eco_mode = 1;
    break;
    default:
// nothing to do
    return count;
    }
    status = acpi_evaluate_object(core::ptr::null_mut(), METHOD_ECWR,
    &input, core::ptr::null_mut());
    if (ACPI_FAILURE(status)) {
    pr_err("%s evaluation failed\n", METHOD_ECWR);
    return -EINVAL;
    }
    return count;
    }
    static ssize_t ac_brightness_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct acpi_device *acpi = to_acpi_device(dev);
    struct pcc_acpi *pcc = acpi_driver_data(acpi);
    if (!acpi_pcc_retrieve_biosdata(pcc))
    return -EIO;
    return sysfs_emit(buf, "%u\n", pcc.sinf[SINF_AC_CUR_BRIGHT]);
    }
    static ssize_t ac_brightness_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct acpi_device *acpi = to_acpi_device(dev);
    struct pcc_acpi *pcc = acpi_driver_data(acpi);
    int err, val;
    err = kstrtoint(buf, 0, &val);
    if (err)
    return err;
    if (val >= 0 && val <= 255) {
    acpi_pcc_write_sset(pcc, SINF_AC_CUR_BRIGHT, val);
    pcc.ac_brightness = val;
    }
    return count;
    }
    static ssize_t dc_brightness_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct acpi_device *acpi = to_acpi_device(dev);
    struct pcc_acpi *pcc = acpi_driver_data(acpi);
    if (!acpi_pcc_retrieve_biosdata(pcc))
    return -EIO;
    return sysfs_emit(buf, "%u\n", pcc.sinf[SINF_DC_CUR_BRIGHT]);
    }
    static ssize_t dc_brightness_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct acpi_device *acpi = to_acpi_device(dev);
    struct pcc_acpi *pcc = acpi_driver_data(acpi);
    int err, val;
    err = kstrtoint(buf, 0, &val);
    if (err)
    return err;
    if (val >= 0 && val <= 255) {
    acpi_pcc_write_sset(pcc, SINF_DC_CUR_BRIGHT, val);
    pcc.dc_brightness = val;
    }
    return count;
    }
    static ssize_t current_brightness_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct acpi_device *acpi = to_acpi_device(dev);
    struct pcc_acpi *pcc = acpi_driver_data(acpi);
    if (!acpi_pcc_retrieve_biosdata(pcc))
    return -EIO;
    return sysfs_emit(buf, "%u\n", pcc.sinf[SINF_CUR_BRIGHT]);
    }
    static ssize_t current_brightness_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct acpi_device *acpi = to_acpi_device(dev);
    struct pcc_acpi *pcc = acpi_driver_data(acpi);
    int err, val;
    err = kstrtoint(buf, 0, &val);
    if (err)
    return err;
    if (val >= 0 && val <= 255) {
    err = acpi_pcc_write_sset(pcc, SINF_CUR_BRIGHT, val);
    pcc.current_brightness = val;
    }
    return count;
    }
    static ssize_t cdpower_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    let mut state: c_int = get_optd_power_state();
    if (state < 0)
    return state;
    return sysfs_emit(buf, "%d\n", state);
    }
    static ssize_t cdpower_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    int err, val;
    err = kstrtoint(buf, 10, &val);
    if (err)
    return err;
    set_optd_power_state(val);
    return count;
    }
    static DEVICE_ATTR_RO(numbatt);
    static DEVICE_ATTR_RO(lcdtype);
    static DEVICE_ATTR_RW(mute);
    static DEVICE_ATTR_RW(sticky_key);
    static DEVICE_ATTR_RW(eco_mode);
    static DEVICE_ATTR_RW(ac_brightness);
    static DEVICE_ATTR_RW(dc_brightness);
    static DEVICE_ATTR_RW(current_brightness);
    static DEVICE_ATTR_RW(cdpower);
#[no_mangle]
unsafe extern "C" fn pcc_sysfs_is_visible(kobj: *mut kobject, attr: *mut attribute, idx: c_int) -> umode_t {
    static umode_t pcc_sysfs_is_visible(struct kobject *kobj, struct attribute *attr, int idx)
    {
    struct device *dev = kobj_to_dev(kobj);
    struct acpi_device *acpi = to_acpi_device(dev);
    struct pcc_acpi *pcc = acpi_driver_data(acpi);
    if (attr == &dev_attr_mute.attr)
    return (pcc.num_sifr > SINF_MUTE) ? attr.mode : 0;
    if (attr == &dev_attr_eco_mode.attr)
    return (pcc.num_sifr > SINF_ECO_MODE) ? attr.mode : 0;
    if (attr == &dev_attr_current_brightness.attr)
    return (pcc.num_sifr > SINF_CUR_BRIGHT) ? attr.mode : 0;
    return attr.mode;
    }
    static struct attribute *pcc_sysfs_entries[] = {
    &dev_attr_numbatt.attr,
    &dev_attr_lcdtype.attr,
    &dev_attr_mute.attr,
    &dev_attr_sticky_key.attr,
    &dev_attr_eco_mode.attr,
    &dev_attr_ac_brightness.attr,
    &dev_attr_dc_brightness.attr,
    &dev_attr_current_brightness.attr,
    &dev_attr_cdpower.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group pcc_attr_group = {
    .name		= core::ptr::null_mut(),		/* put in device directory */
    .attrs		= pcc_sysfs_entries,
    .is_visible	= pcc_sysfs_is_visible,
    };
// hotkey input device driver
    static int sleep_keydown_seen;
#[no_mangle]
unsafe extern "C" fn acpi_pcc_generate_keyinput(pcc: *mut pcc_acpi) {
    static void acpi_pcc_generate_keyinput(struct pcc_acpi *pcc)
    {
    struct input_dev *hotk_input_dev = pcc.input_dev;
    int rc;
    unsigned long long result;
    unsigned int key;
    unsigned int updown;
    rc = acpi_evaluate_integer(pcc.handle, METHOD_HKEY_QUERY,
    core::ptr::null_mut(), &result);
    if (ACPI_FAILURE(rc)) {
    pr_err("error getting hotkey status\n");
    return;
    }
    key = result & GENMASK(6, 0);
    updown = result & BIT(7); /* 0x80 == key down; 0x00 = key up */
// hack: some firmware sends no key down for sleep / hibernate
    if (key == 7 || key == 10) {
    if (updown)
    sleep_keydown_seen = 1;
    if (!sleep_keydown_seen)
    sparse_keymap_report_event(hotk_input_dev,
    key, 0x80, false);
    }
//
// Don't report brightness key-presses if they are also reported
// by the ACPI video bus.
//
    if ((key == 1 || key == 2) && acpi_video_handles_brightness_key_presses())
    return;
    if (!sparse_keymap_report_event(hotk_input_dev, key, updown, false))
    pr_err("Unknown hotkey event: 0x%04llx\n", result);
    }
#[no_mangle]
unsafe extern "C" fn acpi_pcc_hotkey_notify(handle: acpi_handle, event: u32, data: *mut c_void) {
    static void acpi_pcc_hotkey_notify(acpi_handle handle, u32 event, void *data)
    {
    struct pcc_acpi *pcc = data;
    switch (event) {
    case HKEY_NOTIFY:
    acpi_pcc_generate_keyinput(pcc);
    break;
    default:
// nothing to do
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn pcc_optd_notify(handle: acpi_handle, event: u32, data: *mut c_void) {
    static void pcc_optd_notify(acpi_handle handle, u32 event, void *data)
    {
    if (event != ACPI_NOTIFY_EJECT_REQUEST)
    return;
    set_optd_power_state(0);
    }
#[no_mangle]
unsafe extern "C" fn pcc_register_optd_notifier(pcc: *mut pcc_acpi, node: *mut c_char) {
    static void pcc_register_optd_notifier(struct pcc_acpi *pcc, char *node)
    {
    acpi_status status;
    acpi_handle handle;
    status = acpi_get_handle(core::ptr::null_mut(), node, &handle);
    if (ACPI_SUCCESS(status)) {
    status = acpi_install_notify_handler(handle,
    ACPI_SYSTEM_NOTIFY,
    pcc_optd_notify, pcc);
    if (ACPI_FAILURE(status))
    pr_err("Failed to register notify on %s\n", node);
    }
    }
#[no_mangle]
unsafe extern "C" fn pcc_unregister_optd_notifier(pcc: *mut pcc_acpi, node: *mut c_char) {
    static void pcc_unregister_optd_notifier(struct pcc_acpi *pcc, char *node)
    {
    let mut status: acpi_status = AE_OK;
    acpi_handle handle;
    status = acpi_get_handle(core::ptr::null_mut(), node, &handle);
    if (ACPI_SUCCESS(status)) {
    status = acpi_remove_notify_handler(handle,
    ACPI_SYSTEM_NOTIFY,
    pcc_optd_notify);
    if (ACPI_FAILURE(status))
    pr_err("Error removing optd notify handler %s\n",
    node);
    }
    }
#[no_mangle]
unsafe extern "C" fn acpi_pcc_init_input(pcc: *mut pcc_acpi) -> c_int {
    static int acpi_pcc_init_input(struct pcc_acpi *pcc)
    {
    struct input_dev *input_dev;
    int error;
    input_dev = input_allocate_device();
    if (!input_dev)
    return -ENOMEM;
    input_dev.name = ACPI_PCC_DRIVER_NAME;
    input_dev.phys = ACPI_PCC_INPUT_PHYS;
    input_dev.id.bustype = BUS_HOST;
    input_dev.id.vendor = 0x0001;
    input_dev.id.product = 0x0001;
    input_dev.id.version = 0x0100;
    error = sparse_keymap_setup(input_dev, panasonic_keymap, core::ptr::null_mut());
    if (error) {
    pr_err("Unable to setup input device keymap\n");
    goto err_free_dev;
    }
    error = input_register_device(input_dev);
    if (error) {
    pr_err("Unable to register input device\n");
    goto err_free_dev;
    }
    pcc.input_dev = input_dev;
    return 0;
    err_free_dev:
    input_free_device(input_dev);
    return error;
    }
// kernel module interface

#[no_mangle]
unsafe extern "C" fn acpi_pcc_hotkey_resume(dev: *mut device) -> c_int {
    static int acpi_pcc_hotkey_resume(struct device *dev)
    {
    struct pcc_acpi *pcc = acpi_driver_data(ACPI_COMPANION(dev));
    if (pcc.num_sifr > SINF_MUTE)
    acpi_pcc_write_sset(pcc, SINF_MUTE, pcc.mute);
    if (pcc.num_sifr > SINF_ECO_MODE)
    acpi_pcc_write_sset(pcc, SINF_ECO_MODE, pcc.eco_mode);
    acpi_pcc_write_sset(pcc, SINF_STICKY_KEY, pcc.sticky_key);
    acpi_pcc_write_sset(pcc, SINF_AC_CUR_BRIGHT, pcc.ac_brightness);
    acpi_pcc_write_sset(pcc, SINF_DC_CUR_BRIGHT, pcc.dc_brightness);
    if (pcc.num_sifr > SINF_CUR_BRIGHT)
    acpi_pcc_write_sset(pcc, SINF_CUR_BRIGHT, pcc.current_brightness);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn acpi_pcc_hotkey_probe(pdev: *mut platform_device) -> c_int {
    static int acpi_pcc_hotkey_probe(struct platform_device *pdev)
    {
    struct backlight_properties props;
    struct acpi_device *device;
    struct pcc_acpi *pcc;
    int num_sifr, result;
    device = ACPI_COMPANION(&pdev.dev);
    if (!device)
    return -ENODEV;
    num_sifr = acpi_pcc_get_sqty(device);
//
// pcc->sinf is expected to at least have the AC+DC brightness entries.
// Accesses to higher SINF entries are checked against num_sifr.
//
    if (num_sifr <= SINF_DC_CUR_BRIGHT || num_sifr > 255) {
    pr_err("num_sifr %d out of range %d - 255\n", num_sifr, SINF_DC_CUR_BRIGHT + 1);
    return -ENODEV;
    }
//
// Some DSDT-s have an off-by-one bug where the SINF package count is
// one higher than the SQTY reported value, allocate 1 entry extra.
//
    num_sifr++;
    pcc = kzalloc_flex(*pcc, sinf, num_sifr);
    if (!pcc) {
    pr_err("Couldn't allocate mem for pcc");
    return -ENOMEM;
    }
    pcc.num_sifr = num_sifr;
    pcc.device = device;
    pcc.handle = device.handle;
    device.driver_data = pcc;
    result = acpi_pcc_init_input(pcc);
    if (result) {
    pr_err("Error installing keyinput handler\n");
    goto out_hotkey;
    }
    if (!acpi_pcc_retrieve_biosdata(pcc)) {
    result = -EIO;
    pr_err("Couldn't retrieve BIOS data\n");
    goto out_input;
    }
    if (acpi_video_get_backlight_type() == acpi_backlight_vendor) {
// initialize backlight
    memset(&props, 0, sizeof(struct backlight_properties));
    props.type = BACKLIGHT_PLATFORM;
    props.max_brightness = pcc.sinf[SINF_AC_MAX_BRIGHT];
    pcc.backlight = backlight_device_register("panasonic", core::ptr::null_mut(), pcc,
    &pcc_backlight_ops, &props);
    if (IS_ERR(pcc.backlight)) {
    result = PTR_ERR(pcc.backlight);
    goto out_input;
    }
// read the initial brightness setting from the hardware
    pcc.backlight.props.brightness = pcc.sinf[SINF_AC_CUR_BRIGHT];
    }
// Reset initial sticky key mode since the hardware register state is not consistent
    acpi_pcc_write_sset(pcc, SINF_STICKY_KEY, 0);
    pcc.sticky_key = 0;
    pcc.ac_brightness = pcc.sinf[SINF_AC_CUR_BRIGHT];
    pcc.dc_brightness = pcc.sinf[SINF_DC_CUR_BRIGHT];
    if (pcc.num_sifr > SINF_MUTE)
    pcc.mute = pcc.sinf[SINF_MUTE];
    if (pcc.num_sifr > SINF_ECO_MODE)
    pcc.eco_mode = pcc.sinf[SINF_ECO_MODE];
    if (pcc.num_sifr > SINF_CUR_BRIGHT)
    pcc.current_brightness = pcc.sinf[SINF_CUR_BRIGHT];
// add sysfs attributes
    result = sysfs_create_group(&device.dev.kobj, &pcc_attr_group);
    if (result)
    goto out_backlight;
    result = acpi_dev_install_notify_handler(device, ACPI_DEVICE_NOTIFY,
    acpi_pcc_hotkey_notify, pcc);
    if (result)
    goto out_sysfs;
// optical drive initialization
    if (ACPI_SUCCESS(check_optd_present())) {
    pcc.platform = platform_device_register_simple("panasonic",
    PLATFORM_DEVID_NONE, core::ptr::null_mut(), 0);
    if (IS_ERR(pcc.platform)) {
    result = PTR_ERR(pcc.platform);
    goto out_notify_handler;
    }
    result = device_create_file(&pcc.platform.dev,
    &dev_attr_cdpower);
    if (result)
    goto out_platform;
    pcc_register_optd_notifier(pcc, "\\_SB.PCI0.EHCI.ERHB.OPTD");
    } else {
    pcc.platform = core::ptr::null_mut();
    }
    i8042_install_filter(panasonic_i8042_filter, core::ptr::null_mut());
    return 0;
    out_platform:
    platform_device_unregister(pcc.platform);
    out_notify_handler:
    acpi_dev_remove_notify_handler(device, ACPI_DEVICE_NOTIFY,
    acpi_pcc_hotkey_notify);
    out_sysfs:
    sysfs_remove_group(&device.dev.kobj, &pcc_attr_group);
    out_backlight:
    backlight_device_unregister(pcc.backlight);
    out_input:
    input_unregister_device(pcc.input_dev);
    out_hotkey:
    device.driver_data = core::ptr::null_mut();
    kfree(pcc);
    return result;
    }
#[no_mangle]
unsafe extern "C" fn acpi_pcc_hotkey_remove(pdev: *mut platform_device) {
    static void acpi_pcc_hotkey_remove(struct platform_device *pdev)
    {
    struct acpi_device *device = ACPI_COMPANION(&pdev.dev);
    struct pcc_acpi *pcc = acpi_driver_data(device);
    i8042_remove_filter(panasonic_i8042_filter);
    if (pcc.platform) {
    pcc_unregister_optd_notifier(pcc, "\\_SB.PCI0.EHCI.ERHB.OPTD");
    device_remove_file(&pcc.platform.dev, &dev_attr_cdpower);
    platform_device_unregister(pcc.platform);
    }
    acpi_dev_remove_notify_handler(device, ACPI_DEVICE_NOTIFY,
    acpi_pcc_hotkey_notify);
    sysfs_remove_group(&device.dev.kobj, &pcc_attr_group);
    backlight_device_unregister(pcc.backlight);
    input_unregister_device(pcc.input_dev);
    device.driver_data = core::ptr::null_mut();
    kfree(pcc);
    }
    module_platform_driver(acpi_pcc_driver);
