//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/x86/s2idle.c
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
// Architecture-specific ACPI-based support for suspend-to-idle.
//
// Author: Rafael J. Wysocki <rafael.j.wysocki@intel.com>
// Author: Srinivas Pandruvada <srinivas.pandruvada@linux.intel.com>
// Author: Shyam Sundar S K <Shyam-sundar.S-k@amd.com>
//
// On platforms supporting the Low Power S0 Idle interface there is an ACPI
// device object with the PNP0D80 compatible device ID (System Power Management
// Controller) and a specific _DSM method under it.  That method, if present,
// can be used to indicate to the platform that the OS is transitioning into a
// low-power state in which certain types of activity are not desirable or that
// it is leaving such a state, which allows the platform to adjust its operation
// mode accordingly.
//

    static bool sleep_no_lps0 __read_mostly;
    module_param(sleep_no_lps0, bool, 0644);
    MODULE_PARM_DESC(sleep_no_lps0, "Do not use the special LPS0 device interface");
    static bool check_lps0_constraints __read_mostly;
    module_param(check_lps0_constraints, bool, 0644);
    MODULE_PARM_DESC(check_lps0_constraints, "Check LPS0 device constraints");
    static const struct acpi_device_id lps0_device_ids[] = {
    {"PNP0D80", },
    {"", },
    };
// Microsoft platform agnostic UUID

pub const ACPI_LPS0_GET_DEVICE_CONSTRAINTS: c_int = 1;
pub const ACPI_LPS0_SCREEN_OFF: c_int = 3;
pub const ACPI_LPS0_SCREEN_ON: c_int = 4;
pub const ACPI_LPS0_ENTRY: c_int = 5;
pub const ACPI_LPS0_EXIT: c_int = 6;
pub const ACPI_LPS0_MS_ENTRY: c_int = 7;
pub const ACPI_LPS0_MS_EXIT: c_int = 8;
pub const ACPI_MS_TURN_ON_DISPLAY: c_int = 9;
// AMD

pub const ACPI_LPS0_ENTRY_AMD: c_int = 2;
pub const ACPI_LPS0_EXIT_AMD: c_int = 3;
pub const ACPI_LPS0_SCREEN_OFF_AMD: c_int = 4;
pub const ACPI_LPS0_SCREEN_ON_AMD: c_int = 5;
    static acpi_handle lps0_device_handle;
    static guid_t lps0_dsm_guid;
    static int lps0_dsm_func_mask;
    static guid_t lps0_dsm_guid_microsoft;
    static int lps0_dsm_func_mask_microsoft;
    static int lps0_dsm_state;
// Device constraint entry structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpi_device_info {
    pub name: *mut c_char,
    pub enabled: c_int,
    pub package: *mut union acpi_object,
}

// Constraint package structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpi_device_constraint {
    pub uid: c_int,
    pub min_dstate: c_int,
    pub function_states: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpi_constraints {
    pub handle: acpi_handle,
    pub min_dstate: c_int,
}

// AMD Constraint package structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpi_device_constraint_amd {
    pub name: *mut c_char,
    pub enabled: c_int,
    pub function_states: c_int,
    pub min_dstate: c_int,
}

    static LIST_HEAD(lps0_s2idle_devops_head);
    static struct lpi_constraints *lpi_constraints_table;
    static int lpi_constraints_table_size;
    static int rev_id;

    for (int i = 0;								\
    entry = &lpi_constraints_table[i], i < lpi_constraints_table_size;	\
    i++)
#[no_mangle]
unsafe extern "C" fn lpi_device_get_constraints_amd() {
    static void lpi_device_get_constraints_amd(void)
    {
    union acpi_object *out_obj;
    int i, j, k;
    out_obj = acpi_evaluate_dsm_typed(lps0_device_handle, &lps0_dsm_guid,
    rev_id, ACPI_LPS0_GET_DEVICE_CONSTRAINTS,
    core::ptr::null_mut(), ACPI_TYPE_PACKAGE);
    acpi_handle_debug(lps0_device_handle, "_DSM function 1 eval %s\n",
    out_obj ? "successful" : "failed");
    if (!out_obj)
    return;
    for (i = 0; i < out_obj.package.count; i++) {
    union acpi_object *package = &out_obj.package.elements[i];
    if (package.type == ACPI_TYPE_PACKAGE) {
    if (lpi_constraints_table) {
    acpi_handle_err(lps0_device_handle,
    "Duplicate constraints list\n");
    goto free_acpi_buffer;
    }
    lpi_constraints_table = kzalloc_objs(*lpi_constraints_table,
    package.package.count);
    if (!lpi_constraints_table)
    goto free_acpi_buffer;
    acpi_handle_debug(lps0_device_handle,
    "LPI: constraints list begin:\n");
    for (j = 0; j < package.package.count; j++) {
    union acpi_object *info_obj = &package.package.elements[j];
    let mut dev_info: lpi_device_constraint_amd = {};
    struct lpi_constraints *list;
    acpi_status status;
    list = &lpi_constraints_table[lpi_constraints_table_size];
    for (k = 0; k < info_obj.package.count; k++) {
    union acpi_object *obj = &info_obj.package.elements[k];
    switch (k) {
    case 0:
    dev_info.enabled = obj.integer.value;
    break;
    case 1:
    dev_info.name = obj.string.pointer;
    break;
    case 2:
    dev_info.function_states = obj.integer.value;
    break;
    case 3:
    dev_info.min_dstate = obj.integer.value;
    break;
    }
    }
    acpi_handle_debug(lps0_device_handle,
    "Name:%s, Enabled: %d, States: %d, MinDstate: %d\n",
    dev_info.name,
    dev_info.enabled,
    dev_info.function_states,
    dev_info.min_dstate);
    if (!dev_info.enabled || !dev_info.name ||
    !dev_info.min_dstate)
    continue;
    status = acpi_get_handle(core::ptr::null_mut(), dev_info.name, &list.handle);
    if (ACPI_FAILURE(status))
    continue;
    list.min_dstate = dev_info.min_dstate;
    lpi_constraints_table_size++;
    }
    }
    }
    acpi_handle_debug(lps0_device_handle, "LPI: constraints list end\n");
    free_acpi_buffer:
    ACPI_FREE(out_obj);
    }
#[no_mangle]
unsafe extern "C" fn lpi_device_get_constraints() {
    static void lpi_device_get_constraints(void)
    {
    union acpi_object *out_obj;
    int i;
    out_obj = acpi_evaluate_dsm_typed(lps0_device_handle, &lps0_dsm_guid,
    1, ACPI_LPS0_GET_DEVICE_CONSTRAINTS,
    core::ptr::null_mut(), ACPI_TYPE_PACKAGE);
    acpi_handle_debug(lps0_device_handle, "_DSM function 1 eval %s\n",
    out_obj ? "successful" : "failed");
    if (!out_obj)
    return;
    lpi_constraints_table = kzalloc_objs(*lpi_constraints_table,
    out_obj.package.count);
    if (!lpi_constraints_table)
    goto free_acpi_buffer;
    acpi_handle_debug(lps0_device_handle, "LPI: constraints list begin:\n");
    for (i = 0; i < out_obj.package.count; i++) {
    struct lpi_constraints *constraint;
    acpi_status status;
    union acpi_object *package = &out_obj.package.elements[i];
    let mut info: lpi_device_info = { };
    let mut package_count: c_int = 0, j;
    if (!package)
    continue;
    for (j = 0; j < package.package.count; j++) {
    union acpi_object *element =
    &(package.package.elements[j]);
    switch (element.type) {
    case ACPI_TYPE_INTEGER:
    info.enabled = element.integer.value;
    break;
    case ACPI_TYPE_STRING:
    info.name = element.string.pointer;
    break;
    case ACPI_TYPE_PACKAGE:
    package_count = element.package.count;
    info.package = element.package.elements;
    break;
    }
    }
    if (!info.enabled || !info.package || !info.name)
    continue;
    constraint = &lpi_constraints_table[lpi_constraints_table_size];
    status = acpi_get_handle(core::ptr::null_mut(), info.name, &constraint.handle);
    if (ACPI_FAILURE(status))
    continue;
    acpi_handle_debug(lps0_device_handle,
    "index:%d Name:%s\n", i, info.name);
    constraint.min_dstate = -1;
    for (j = 0; j < package_count; j++) {
    union acpi_object *info_obj = &info.package[j];
    union acpi_object *cnstr_pkg;
    union acpi_object *obj;
    struct lpi_device_constraint dev_info;
    switch (info_obj.type) {
    case ACPI_TYPE_INTEGER:
// version
    break;
    case ACPI_TYPE_PACKAGE:
    if (info_obj.package.count < 2)
    break;
    cnstr_pkg = info_obj.package.elements;
    obj = &cnstr_pkg[0];
    dev_info.uid = obj.integer.value;
    obj = &cnstr_pkg[1];
    dev_info.min_dstate = obj.integer.value;
    acpi_handle_debug(lps0_device_handle,
    "uid:%d min_dstate:%s\n",
    dev_info.uid,
    acpi_power_state_string(dev_info.min_dstate));
    constraint.min_dstate = dev_info.min_dstate;
    break;
    }
    }
    if (constraint.min_dstate < 0) {
    acpi_handle_debug(lps0_device_handle,
    "Incomplete constraint defined\n");
    continue;
    }
    lpi_constraints_table_size++;
    }
    acpi_handle_debug(lps0_device_handle, "LPI: constraints list end\n");
    free_acpi_buffer:
    ACPI_FREE(out_obj);
    }
#[no_mangle]
unsafe extern "C" fn lpi_check_constraints() {
    static void lpi_check_constraints(void)
    {
    struct lpi_constraints *entry;
    if (IS_ERR_OR_NULL(lpi_constraints_table))
    return;
    for_each_lpi_constraint(entry) {
    struct acpi_device *adev = acpi_fetch_acpi_dev(entry.handle);
    if (!adev)
    continue;
    acpi_handle_debug(entry.handle,
    "LPI: required min power state:%s current power state:%s\n",
    acpi_power_state_string(entry.min_dstate),
    acpi_power_state_string(adev.power.state));
    if (!adev.flags.power_manageable) {
    acpi_handle_info(entry.handle, "LPI: Device not power manageable\n");
    entry.handle = core::ptr::null_mut();
    continue;
    }
    if (adev.power.state < entry.min_dstate)
    acpi_handle_info(entry.handle,
    "LPI: Constraint not met; min power state:%s current power state:%s\n",
    acpi_power_state_string(entry.min_dstate),
    acpi_power_state_string(adev.power.state));
    }
    }
#[no_mangle]
unsafe extern "C" fn acpi_s2idle_vendor_amd() -> bool {
    static bool acpi_s2idle_vendor_amd(void)
    {
    return boot_cpu_data.x86_vendor == X86_VENDOR_AMD;
    }
    static const char *acpi_sleep_dsm_state_to_str(unsigned int state)
    {
    if (lps0_dsm_func_mask_microsoft || !acpi_s2idle_vendor_amd()) {
    switch (state) {
    case ACPI_LPS0_SCREEN_OFF:
    return "screen off";
    case ACPI_LPS0_SCREEN_ON:
    return "screen on";
    case ACPI_LPS0_ENTRY:
    return "lps0 entry";
    case ACPI_LPS0_EXIT:
    return "lps0 exit";
    case ACPI_LPS0_MS_ENTRY:
    return "lps0 ms entry";
    case ACPI_LPS0_MS_EXIT:
    return "lps0 ms exit";
    case ACPI_MS_TURN_ON_DISPLAY:
    return "lps0 ms turn on display";
    }
    } else {
    switch (state) {
    case ACPI_LPS0_SCREEN_ON_AMD:
    return "screen on";
    case ACPI_LPS0_SCREEN_OFF_AMD:
    return "screen off";
    case ACPI_LPS0_ENTRY_AMD:
    return "lps0 entry";
    case ACPI_LPS0_EXIT_AMD:
    return "lps0 exit";
    }
    }
    return "unknown";
    }
#[no_mangle]
unsafe extern "C" fn acpi_sleep_run_lps0_dsm(func: c_uint, func_mask: c_uint, dsm_guid: guid_t) {
    static void acpi_sleep_run_lps0_dsm(unsigned int func, unsigned int func_mask, guid_t dsm_guid)
    {
    union acpi_object *out_obj;
    if (!(func_mask & (1 << func)))
    return;
    out_obj = acpi_evaluate_dsm(lps0_device_handle, &dsm_guid,
    rev_id, func, core::ptr::null_mut());
    ACPI_FREE(out_obj);
    lps0_dsm_state = func;
    if (pm_debug_messages_on) {
    acpi_handle_info(lps0_device_handle,
    "%s transitioned to state %s\n",
    out_obj ? "Successfully" : "Failed to",
    acpi_sleep_dsm_state_to_str(lps0_dsm_state));
    }
    }
#[no_mangle]
unsafe extern "C" fn validate_dsm(handle: acpi_handle, uuid: *const c_char, rev: c_int, dsm_guid: *mut guid_t) -> c_int {
    static int validate_dsm(acpi_handle handle, const char *uuid, int rev, guid_t *dsm_guid)
    {
    union acpi_object *obj;
    let mut ret: c_int = -EINVAL;
    guid_parse(uuid, dsm_guid);
// Check if the _DSM is present and as expected.
    obj = acpi_evaluate_dsm_typed(handle, dsm_guid, rev, 0, core::ptr::null_mut(), ACPI_TYPE_BUFFER);
    if (!obj || obj.buffer.length == 0 || obj.buffer.length > sizeof(u32)) {
    acpi_handle_debug(handle,
    "_DSM UUID %s rev %d function 0 evaluation failed\n", uuid, rev);
    goto out;
    }
    ret = *(int *)obj.buffer.pointer;
    acpi_handle_debug(handle, "_DSM UUID %s rev %d function mask: 0x%x\n", uuid, rev, ret);
    out:
    ACPI_FREE(obj);
    return ret;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_lps0_hid_device_data {
    pub check_off_by_one: bool,
}

    static const struct amd_lps0_hid_device_data amd_picasso = {
    .check_off_by_one = true,
    };
    static const struct amd_lps0_hid_device_data amd_cezanne = {
    .check_off_by_one = false,
    };
    static const struct acpi_device_id amd_hid_ids[] = {
    {"AMD0004",	(kernel_ulong_t)&amd_picasso,	},
    {"AMD0005",	(kernel_ulong_t)&amd_picasso,	},
    {"AMDI0005",	(kernel_ulong_t)&amd_picasso,	},
    {"AMDI0006",	(kernel_ulong_t)&amd_cezanne,	},
    {}
    };
    static int lps0_device_attach(struct acpi_device *adev,
    const struct acpi_device_id *not_used)
    {
    if (lps0_device_handle)
    return 0;
    lps0_dsm_func_mask_microsoft = validate_dsm(adev.handle,
    ACPI_LPS0_DSM_UUID_MICROSOFT, 0,
    &lps0_dsm_guid_microsoft);
    if (acpi_s2idle_vendor_amd()) {
    static const struct acpi_device_id *dev_id;
    const struct amd_lps0_hid_device_data *data;
    for (dev_id = &amd_hid_ids[0]; dev_id.id[0]; dev_id++)
    if (acpi_dev_hid_uid_match(adev, dev_id.id, core::ptr::null_mut()))
    break;
    if (dev_id.id[0])
    data = (const struct amd_lps0_hid_device_data *) dev_id.driver_data;
    else
    data = &amd_cezanne;
    lps0_dsm_func_mask = validate_dsm(adev.handle,
    ACPI_LPS0_DSM_UUID_AMD, rev_id, &lps0_dsm_guid);
    if (lps0_dsm_func_mask > 0x3 && data.check_off_by_one) {
    lps0_dsm_func_mask = (lps0_dsm_func_mask << 1) | 0x1;
    acpi_handle_debug(adev.handle, "_DSM UUID %s: Adjusted function mask: 0x%x\n",
    ACPI_LPS0_DSM_UUID_AMD, lps0_dsm_func_mask);
    }
    } else {
    rev_id = 1;
    lps0_dsm_func_mask = validate_dsm(adev.handle,
    ACPI_LPS0_DSM_UUID, rev_id, &lps0_dsm_guid);
    if (lps0_dsm_func_mask > 0 && lps0_dsm_func_mask_microsoft > 0) {
    unsigned int func_mask;
//
// Log a message if the _DSM function sets for two
// different UUIDs overlap.
//
    func_mask = lps0_dsm_func_mask & lps0_dsm_func_mask_microsoft;
    if (func_mask)
    acpi_handle_info(adev.handle,
    "Duplicate LPS0 _DSM functions (mask: 0x%x)\n",
    func_mask);
    }
    }
    if (lps0_dsm_func_mask < 0 && lps0_dsm_func_mask_microsoft < 0)
    return 0; //function evaluation failed
    lps0_device_handle = adev.handle;
//
// Use suspend-to-idle by default if ACPI_FADT_LOW_POWER_S0 is set in
// the FADT and the default suspend mode was not set from the command
// line.
//
    if ((acpi_gbl_FADT.flags & ACPI_FADT_LOW_POWER_S0) &&
    mem_sleep_default > PM_SUSPEND_MEM && !acpi_sleep_default_s3) {
    mem_sleep_current = PM_SUSPEND_TO_IDLE;
    pr_info("Low-power S0 idle used by default for system suspend\n");
    }
//
// Some LPS0 systems, like ASUS Zenbook UX430UNR/i7-8550U, require the
// EC GPE to be enabled while suspended for certain wakeup devices to
// work, so mark it as wakeup-capable.
//
    acpi_ec_mark_gpe_for_wake();
    return 0;
    }
    static struct acpi_scan_handler lps0_handler = {
    .ids = lps0_device_ids,
    .attach = lps0_device_attach,
    };
#[no_mangle]
unsafe extern "C" fn acpi_s2idle_begin_lps0() -> c_int {
    static int acpi_s2idle_begin_lps0(void)
    {
    if (lps0_device_handle && !sleep_no_lps0 && check_lps0_constraints &&
    !lpi_constraints_table) {
    if (acpi_s2idle_vendor_amd())
    lpi_device_get_constraints_amd();
    else
    lpi_device_get_constraints();
//
// Try to retrieve the constraints only once because failures
// to do so usually are sticky.
//
    if (!lpi_constraints_table)
    lpi_constraints_table = ERR_PTR(-ENODATA);
    }
    return acpi_s2idle_begin();
    }
#[no_mangle]
unsafe extern "C" fn acpi_s2idle_prepare_late_lps0() -> c_int {
    static int acpi_s2idle_prepare_late_lps0(void)
    {
    struct acpi_s2idle_dev_ops *handler;
    if (!lps0_device_handle || sleep_no_lps0)
    return 0;
    if (check_lps0_constraints)
    lpi_check_constraints();
// Screen off
    if (lps0_dsm_func_mask > 0)
    acpi_sleep_run_lps0_dsm(acpi_s2idle_vendor_amd() ?
    ACPI_LPS0_SCREEN_OFF_AMD :
    ACPI_LPS0_SCREEN_OFF,
    lps0_dsm_func_mask, lps0_dsm_guid);
    if (lps0_dsm_func_mask_microsoft > 0)
    acpi_sleep_run_lps0_dsm(ACPI_LPS0_SCREEN_OFF,
    lps0_dsm_func_mask_microsoft, lps0_dsm_guid_microsoft);
// LPS0 entry
    if (lps0_dsm_func_mask > 0 && acpi_s2idle_vendor_amd())
    acpi_sleep_run_lps0_dsm(ACPI_LPS0_ENTRY_AMD,
    lps0_dsm_func_mask, lps0_dsm_guid);
    if (lps0_dsm_func_mask_microsoft > 0) {
// Modern Standby entry
    acpi_sleep_run_lps0_dsm(ACPI_LPS0_MS_ENTRY,
    lps0_dsm_func_mask_microsoft, lps0_dsm_guid_microsoft);
    acpi_sleep_run_lps0_dsm(ACPI_LPS0_ENTRY,
    lps0_dsm_func_mask_microsoft, lps0_dsm_guid_microsoft);
    }
    if (lps0_dsm_func_mask > 0 && !acpi_s2idle_vendor_amd())
    acpi_sleep_run_lps0_dsm(ACPI_LPS0_ENTRY,
    lps0_dsm_func_mask, lps0_dsm_guid);
    list_for_each_entry(handler, &lps0_s2idle_devops_head, list_node) {
    if (handler.prepare)
    handler.prepare();
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn acpi_s2idle_check_lps0() {
    static void acpi_s2idle_check_lps0(void)
    {
    struct acpi_s2idle_dev_ops *handler;
    if (!lps0_device_handle || sleep_no_lps0)
    return;
    list_for_each_entry(handler, &lps0_s2idle_devops_head, list_node) {
    if (handler.check)
    handler.check();
    }
    }
#[no_mangle]
unsafe extern "C" fn acpi_s2idle_restore_early_lps0() {
    static void acpi_s2idle_restore_early_lps0(void)
    {
    struct acpi_s2idle_dev_ops *handler;
    if (!lps0_device_handle || sleep_no_lps0)
    return;
    list_for_each_entry(handler, &lps0_s2idle_devops_head, list_node)
    if (handler.restore)
    handler.restore();
// LPS0 exit
    if (lps0_dsm_func_mask > 0)
    acpi_sleep_run_lps0_dsm(acpi_s2idle_vendor_amd() ?
    ACPI_LPS0_EXIT_AMD :
    ACPI_LPS0_EXIT,
    lps0_dsm_func_mask, lps0_dsm_guid);
    if (lps0_dsm_func_mask_microsoft > 0) {
    acpi_sleep_run_lps0_dsm(ACPI_LPS0_EXIT,
    lps0_dsm_func_mask_microsoft, lps0_dsm_guid_microsoft);
// Intent to turn on display
    acpi_sleep_run_lps0_dsm(ACPI_MS_TURN_ON_DISPLAY,
    lps0_dsm_func_mask_microsoft, lps0_dsm_guid_microsoft);
// Modern Standby exit
    acpi_sleep_run_lps0_dsm(ACPI_LPS0_MS_EXIT,
    lps0_dsm_func_mask_microsoft, lps0_dsm_guid_microsoft);
    }
// Screen on
    if (lps0_dsm_func_mask_microsoft > 0)
    acpi_sleep_run_lps0_dsm(ACPI_LPS0_SCREEN_ON,
    lps0_dsm_func_mask_microsoft, lps0_dsm_guid_microsoft);
    if (lps0_dsm_func_mask > 0)
    acpi_sleep_run_lps0_dsm(acpi_s2idle_vendor_amd() ?
    ACPI_LPS0_SCREEN_ON_AMD :
    ACPI_LPS0_SCREEN_ON,
    lps0_dsm_func_mask, lps0_dsm_guid);
    }
    static const struct platform_s2idle_ops acpi_s2idle_ops_lps0 = {
    .begin = acpi_s2idle_begin_lps0,
    .prepare = acpi_s2idle_prepare,
    .prepare_late = acpi_s2idle_prepare_late_lps0,
    .check = acpi_s2idle_check_lps0,
    .wake = acpi_s2idle_wake,
    .restore_early = acpi_s2idle_restore_early_lps0,
    .restore = acpi_s2idle_restore,
    .end = acpi_s2idle_end,
    };
#[no_mangle]
pub unsafe extern "C" fn acpi_s2idle_setup() -> void __init {
    void __init acpi_s2idle_setup(void)
    {
    acpi_scan_add_handler(&lps0_handler);
    s2idle_set_ops(&acpi_s2idle_ops_lps0);
    }
#[no_mangle]
pub unsafe extern "C" fn acpi_register_lps0_dev(arg: *mut acpi_s2idle_dev_ops) -> c_int {
    int acpi_register_lps0_dev(struct acpi_s2idle_dev_ops *arg)
    {
    unsigned int sleep_flags;
    if (!lps0_device_handle || sleep_no_lps0)
    return -ENODEV;
    sleep_flags = lock_system_sleep();
    list_add(&arg.list_node, &lps0_s2idle_devops_head);
    unlock_system_sleep(sleep_flags);
    return 0;
    }
    EXPORT_SYMBOL_GPL(acpi_register_lps0_dev);
#[no_mangle]
pub unsafe extern "C" fn acpi_unregister_lps0_dev(arg: *mut acpi_s2idle_dev_ops) {
    void acpi_unregister_lps0_dev(struct acpi_s2idle_dev_ops *arg)
    {
    unsigned int sleep_flags;
    if (!lps0_device_handle || sleep_no_lps0)
    return;
    sleep_flags = lock_system_sleep();
    list_del(&arg.list_node);
    unlock_system_sleep(sleep_flags);
    }
    EXPORT_SYMBOL_GPL(acpi_unregister_lps0_dev);
