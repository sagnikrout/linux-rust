//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_atpx_handler.c
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
// Copyright (c) 2010 Red Hat Inc.
// Author : Dave Airlie <airlied@redhat.com>
//
// ATPX support for both Intel/ATI
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_px_quirk {
    pub chip_vendor: u32,
    pub chip_device: u32,
    pub subsys_vendor: u32,
    pub subsys_device: u32,
    pub px_quirk_flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_atpx_functions {
    pub px_params: bool,
    pub power_cntl: bool,
    pub disp_mux_cntl: bool,
    pub i2c_mux_cntl: bool,
    pub switch_start: bool,
    pub switch_end: bool,
    pub disp_connectors_mapping: bool,
    pub disp_detection_ports: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_atpx {
    pub handle: acpi_handle,
    pub functions: amdgpu_atpx_functions,
    pub is_hybrid: bool,
    pub dgpu_req_power_for_displays: bool,
}

    static struct amdgpu_atpx_priv {
    bool atpx_detected;
    bool bridge_pm_usable;
    unsigned int quirks;
// handle for device - and atpx
    acpi_handle dhandle;
    acpi_handle other_handle;
    struct amdgpu_atpx atpx;
    } amdgpu_atpx_priv;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atpx_verify_interface {
    pub /: *mut *mut u16 size; / structure size in bytes (includes size field),
    pub /: *mut *mut u16 version; / version,
    pub /: *mut *mut u32 function_bits; / supported functions bit vector,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atpx_px_params {
    pub /: *mut *mut u16 size; / structure size in bytes (includes size field),
    pub /: *mut *mut u32 valid_flags; / which flags are valid,
    pub /: *mut *mut u32 flags; / flags,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atpx_power_control {
    pub size: u16,
    pub dgpu_state: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atpx_mux {
    pub size: u16,
    pub mux: u16,
    pub __packed: },
#[no_mangle]
pub unsafe extern "C" fn amdgpu_has_atpx() -> bool {
    bool amdgpu_has_atpx(void)
    {
    pub amdgpu_atpx_priv.atpx_detected: return,
    }
#[no_mangle]
pub unsafe extern "C" fn amdgpu_has_atpx_dgpu_power_cntl() -> bool {
    bool amdgpu_has_atpx_dgpu_power_cntl(void)
    {
    pub amdgpu_atpx_priv.atpx.functions.power_cntl: return,
    }
#[no_mangle]
pub unsafe extern "C" fn amdgpu_is_atpx_hybrid() -> bool {
    bool amdgpu_is_atpx_hybrid(void)
    {
    pub amdgpu_atpx_priv.atpx.is_hybrid: return,
    }
    static bool amdgpu_atpx_buffer_validate(const union acpi_object *obj,
    size_t min_size)
    {
    return obj && obj.type == ACPI_TYPE_BUFFER &&
    obj.buffer.length >= sizeof(u16) &&
    obj.buffer.length >= *(u16 *)obj.buffer.pointer &&
// (u16 *)obj->buffer.pointer >= min_size;
    }
//
// amdgpu_atpx_call - call an ATPX method
//
// @handle: acpi handle
// @function: the ATPX function to execute
// @params: ATPX function params
//
// Executes the requested ATPX function (all asics).
// Returns a pointer to the acpi output buffer.
//
    static union acpi_object *amdgpu_atpx_call(acpi_handle handle, int function,
    struct acpi_buffer *params)
    {
    pub status: acpi_status,
    pub atpx_arg_elements: [union acpi_object; 2],
    pub atpx_arg: acpi_object_list,
    pub }: acpi_buffer buffer = { ACPI_ALLOCATE_BUFFER, NULL,
    pub 2: atpx_arg.count =,
    pub &atpx_arg_elements[0]: atpx_arg.pointer =,
    pub ACPI_TYPE_INTEGER: atpx_arg_elements[0].type =,
    pub function: atpx_arg_elements[0].integer.value =,
    if (params) {
    pub ACPI_TYPE_BUFFER: atpx_arg_elements[1].type =,
    pub params->length: atpx_arg_elements[1].buffer.length =,
    pub params->pointer: atpx_arg_elements[1].buffer.pointer =,
    } else {
// We need a second fake parameter
    pub ACPI_TYPE_INTEGER: atpx_arg_elements[1].type =,
    pub 0: atpx_arg_elements[1].integer.value =,
    }
    pub &buffer): status = acpi_evaluate_object(handle, NULL, &atpx_arg,,
// Fail only if calling the method fails and ATPX is supported
    if (ACPI_FAILURE(status) && status != AE_NOT_FOUND) {
    pr_err("failed to evaluate ATPX got %s\n",
    pub NULL: return,
    }
    pub buffer.pointer: return,
    }
//
// amdgpu_atpx_parse_functions - parse supported functions
//
// @f: supported functions struct
// @mask: supported functions mask from ATPX
//
// Use the supported functions mask from ATPX function
// ATPX_FUNCTION_VERIFY_INTERFACE to determine what functions
// are supported (all asics).
//
#[no_mangle]
unsafe extern "C" fn amdgpu_atpx_parse_functions(f: *mut amdgpu_atpx_functions, mask: u32) {
    static void amdgpu_atpx_parse_functions(struct amdgpu_atpx_functions *f, u32 mask)
    {
    pub ATPX_GET_PX_PARAMETERS_SUPPORTED: f->px_params = mask &,
    pub ATPX_POWER_CONTROL_SUPPORTED: f->power_cntl = mask &,
    pub ATPX_DISPLAY_MUX_CONTROL_SUPPORTED: f->disp_mux_cntl = mask &,
    pub ATPX_I2C_MUX_CONTROL_SUPPORTED: f->i2c_mux_cntl = mask &,
    pub ATPX_GRAPHICS_DEVICE_SWITCH_START_NOTIFICATION_SUPPORTED: f->switch_start = mask &,
    pub ATPX_GRAPHICS_DEVICE_SWITCH_END_NOTIFICATION_SUPPORTED: f->switch_end = mask &,
    pub ATPX_GET_DISPLAY_CONNECTORS_MAPPING_SUPPORTED: f->disp_connectors_mapping = mask &,
    pub ATPX_GET_DISPLAY_DETECTION_PORTS_SUPPORTED: f->disp_detection_ports = mask &,
    }
//
// amdgpu_atpx_validate - validate ATPX functions
//
// @atpx: amdgpu atpx struct
//
// Validate that required functions are enabled (all asics).
// returns 0 on success, error on failure.
//
#[no_mangle]
unsafe extern "C" fn amdgpu_atpx_validate(atpx: *mut amdgpu_atpx) -> c_int {
    static int amdgpu_atpx_validate(struct amdgpu_atpx *atpx)
    {
    pub 0: u32 valid_bits =,
    if (atpx.functions.px_params) {
    pub info: *mut union acpi_object,
    pub output: atpx_px_params,
    pub size: usize,
    pub NULL): info = amdgpu_atpx_call(atpx->handle, ATPX_FUNCTION_GET_PX_PARAMETERS,,
    if (!info)
    pub -EIO: return,
    if (!amdgpu_atpx_buffer_validate(info, sizeof(output))) {
    pub response\n"): pr_err("Invalid ATPX GET_PX_PARAMETERS,
    pub -EINVAL: return,
    }
    pub sizeof(output)): memset(&output, 0,,
    pub )info->buffer.pointer): *mut *mut size = min(sizeof(output), (size_t)(u16,
    pub size): memcpy(&output, info->buffer.pointer,,
    pub output.valid_flags: valid_bits = output.flags &,
    }
// if separate mux flag is set, mux controls are required
    if (valid_bits & ATPX_SEPARATE_MUX_FOR_I2C) {
    pub true: atpx->functions.i2c_mux_cntl =,
    pub true: atpx->functions.disp_mux_cntl =,
    }
// if any outputs are muxed, mux controls are required
    if (valid_bits & (ATPX_CRT1_RGB_SIGNAL_MUXED |
    ATPX_TV_SIGNAL_MUXED |
    ATPX_DFP_SIGNAL_MUXED))
    pub true: atpx->functions.disp_mux_cntl =,
// some bioses set these bits rather than flagging power_cntl as supported
    if (valid_bits & (ATPX_DYNAMIC_PX_SUPPORTED |
    ATPX_DYNAMIC_DGPU_POWER_OFF_SUPPORTED))
    pub true: atpx->functions.power_cntl =,
    pub false: atpx->is_hybrid =,
    if (valid_bits & ATPX_MS_HYBRID_GFX_SUPPORTED) {
    if (amdgpu_atpx_priv.quirks & AMDGPU_PX_QUIRK_FORCE_ATPX) {
    pub ATPX\n"): pr_warn("ATPX Hybrid Graphics, forcing to,
    pub true: atpx->functions.power_cntl =,
    pub false: atpx->is_hybrid =,
    } else {
    pub Graphics\n"): pr_notice("ATPX Hybrid,
//
// Disable legacy PM methods only when pcie port PM is usable,
// otherwise the device might fail to power off or power on.
//
    pub !amdgpu_atpx_priv.bridge_pm_usable: atpx->functions.power_cntl =,
    pub true: atpx->is_hybrid =,
    }
    }
    pub false: atpx->dgpu_req_power_for_displays =,
    if (valid_bits & ATPX_DGPU_REQ_POWER_FOR_DISPLAYS)
    pub true: atpx->dgpu_req_power_for_displays =,
    pub 0: return,
    }
//
// amdgpu_atpx_verify_interface - verify ATPX
//
// @atpx: amdgpu atpx struct
//
// Execute the ATPX_FUNCTION_VERIFY_INTERFACE ATPX function
// to initialize ATPX and determine what features are supported
// (all asics).
// returns 0 on success, error on failure.
//
#[no_mangle]
unsafe extern "C" fn amdgpu_atpx_verify_interface(atpx: *mut amdgpu_atpx) -> c_int {
    static int amdgpu_atpx_verify_interface(struct amdgpu_atpx *atpx)
    {
    pub info: *mut union acpi_object,
    pub output: atpx_verify_interface,
    pub size: usize,
    pub 0: int err =,
    pub NULL): info = amdgpu_atpx_call(atpx->handle, ATPX_FUNCTION_VERIFY_INTERFACE,,
    if (!info)
    pub -EIO: return,
    if (!amdgpu_atpx_buffer_validate(info, sizeof(output))) {
    pub response\n"): pr_err("Invalid ATPX VERIFY_INTERFACE,
    pub -EINVAL: err =,
    pub out: goto,
    }
    pub sizeof(output)): memset(&output, 0,,
    pub )info->buffer.pointer): *mut *mut size = min(sizeof(output), (size_t)(u16,
    pub size): memcpy(&output, info->buffer.pointer,,
// TODO: check version?
    pr_notice("ATPX version %u, functions 0x%08x\n",
    pub output.function_bits): output.version,,
    pub output.function_bits): amdgpu_atpx_parse_functions(&atpx->functions,,
    out:
    pub err: return,
    }
//
// amdgpu_atpx_set_discrete_state - power up/down discrete GPU
//
// @atpx: atpx info struct
// @state: discrete GPU state (0 = power down, 1 = power up)
//
// Execute the ATPX_FUNCTION_POWER_CONTROL ATPX function to
// power down/up the discrete GPU (all asics).
// Returns 0 on success, error on failure.
//
#[no_mangle]
unsafe extern "C" fn amdgpu_atpx_set_discrete_state(atpx: *mut amdgpu_atpx, state: u8) -> c_int {
    static int amdgpu_atpx_set_discrete_state(struct amdgpu_atpx *atpx, u8 state)
    {
    pub params: acpi_buffer,
    pub info: *mut union acpi_object,
    pub input: atpx_power_control,
    if (atpx.functions.power_cntl) {
    pub 3: input.size =,
    pub state: input.dgpu_state =,
    pub input.size: params.length =,
    pub &input: params.pointer =,
    info = amdgpu_atpx_call(atpx.handle,
    ATPX_FUNCTION_POWER_CONTROL,
    if (!info)
    pub -EIO: return,
// 200ms delay is required after off
    if (state == 0)
    }
    pub 0: return,
    }
//
// amdgpu_atpx_switch_disp_mux - switch display mux
//
// @atpx: atpx info struct
// @mux_id: mux state (0 = integrated GPU, 1 = discrete GPU)
//
// Execute the ATPX_FUNCTION_DISPLAY_MUX_CONTROL ATPX function to
// switch the display mux between the discrete GPU and integrated GPU
// (all asics).
// Returns 0 on success, error on failure.
//
#[no_mangle]
unsafe extern "C" fn amdgpu_atpx_switch_disp_mux(atpx: *mut amdgpu_atpx, mux_id: u16) -> c_int {
    static int amdgpu_atpx_switch_disp_mux(struct amdgpu_atpx *atpx, u16 mux_id)
    {
    pub params: acpi_buffer,
    pub info: *mut union acpi_object,
    pub input: atpx_mux,
    if (atpx.functions.disp_mux_cntl) {
    pub 4: input.size =,
    pub mux_id: input.mux =,
    pub input.size: params.length =,
    pub &input: params.pointer =,
    info = amdgpu_atpx_call(atpx.handle,
    ATPX_FUNCTION_DISPLAY_MUX_CONTROL,
    if (!info)
    pub -EIO: return,
    }
    pub 0: return,
    }
//
// amdgpu_atpx_switch_i2c_mux - switch i2c/hpd mux
//
// @atpx: atpx info struct
// @mux_id: mux state (0 = integrated GPU, 1 = discrete GPU)
//
// Execute the ATPX_FUNCTION_I2C_MUX_CONTROL ATPX function to
// switch the i2c/hpd mux between the discrete GPU and integrated GPU
// (all asics).
// Returns 0 on success, error on failure.
//
#[no_mangle]
unsafe extern "C" fn amdgpu_atpx_switch_i2c_mux(atpx: *mut amdgpu_atpx, mux_id: u16) -> c_int {
    static int amdgpu_atpx_switch_i2c_mux(struct amdgpu_atpx *atpx, u16 mux_id)
    {
    pub params: acpi_buffer,
    pub info: *mut union acpi_object,
    pub input: atpx_mux,
    if (atpx.functions.i2c_mux_cntl) {
    pub 4: input.size =,
    pub mux_id: input.mux =,
    pub input.size: params.length =,
    pub &input: params.pointer =,
    info = amdgpu_atpx_call(atpx.handle,
    ATPX_FUNCTION_I2C_MUX_CONTROL,
    if (!info)
    pub -EIO: return,
    }
    pub 0: return,
    }
//
// amdgpu_atpx_switch_start - notify the sbios of a GPU switch
//
// @atpx: atpx info struct
// @mux_id: mux state (0 = integrated GPU, 1 = discrete GPU)
//
// Execute the ATPX_FUNCTION_GRAPHICS_DEVICE_SWITCH_START_NOTIFICATION ATPX
// function to notify the sbios that a switch between the discrete GPU and
// integrated GPU has begun (all asics).
// Returns 0 on success, error on failure.
//
#[no_mangle]
unsafe extern "C" fn amdgpu_atpx_switch_start(atpx: *mut amdgpu_atpx, mux_id: u16) -> c_int {
    static int amdgpu_atpx_switch_start(struct amdgpu_atpx *atpx, u16 mux_id)
    {
    pub params: acpi_buffer,
    pub info: *mut union acpi_object,
    pub input: atpx_mux,
    if (atpx.functions.switch_start) {
    pub 4: input.size =,
    pub mux_id: input.mux =,
    pub input.size: params.length =,
    pub &input: params.pointer =,
    info = amdgpu_atpx_call(atpx.handle,
    ATPX_FUNCTION_GRAPHICS_DEVICE_SWITCH_START_NOTIFICATION,
    if (!info)
    pub -EIO: return,
    }
    pub 0: return,
    }
//
// amdgpu_atpx_switch_end - notify the sbios of a GPU switch
//
// @atpx: atpx info struct
// @mux_id: mux state (0 = integrated GPU, 1 = discrete GPU)
//
// Execute the ATPX_FUNCTION_GRAPHICS_DEVICE_SWITCH_END_NOTIFICATION ATPX
// function to notify the sbios that a switch between the discrete GPU and
// integrated GPU has ended (all asics).
// Returns 0 on success, error on failure.
//
#[no_mangle]
unsafe extern "C" fn amdgpu_atpx_switch_end(atpx: *mut amdgpu_atpx, mux_id: u16) -> c_int {
    static int amdgpu_atpx_switch_end(struct amdgpu_atpx *atpx, u16 mux_id)
    {
    pub params: acpi_buffer,
    pub info: *mut union acpi_object,
    pub input: atpx_mux,
    if (atpx.functions.switch_end) {
    pub 4: input.size =,
    pub mux_id: input.mux =,
    pub input.size: params.length =,
    pub &input: params.pointer =,
    info = amdgpu_atpx_call(atpx.handle,
    ATPX_FUNCTION_GRAPHICS_DEVICE_SWITCH_END_NOTIFICATION,
    if (!info)
    pub -EIO: return,
    }
    pub 0: return,
    }
//
// amdgpu_atpx_switchto - switch to the requested GPU
//
// @id: GPU to switch to
//
// Execute the necessary ATPX functions to switch between the discrete GPU and
// integrated GPU (all asics).
// Returns 0 on success, error on failure.
//
#[no_mangle]
unsafe extern "C" fn amdgpu_atpx_switchto(id: enum vga_switcheroo_client_id) -> c_int {
    static int amdgpu_atpx_switchto(enum vga_switcheroo_client_id id)
    {
    pub gpu_id: u16,
    if (id == VGA_SWITCHEROO_IGD)
    pub ATPX_INTEGRATED_GPU: gpu_id =,
    else
    pub ATPX_DISCRETE_GPU: gpu_id =,
    pub gpu_id): amdgpu_atpx_switch_start(&amdgpu_atpx_priv.atpx,,
    pub gpu_id): amdgpu_atpx_switch_disp_mux(&amdgpu_atpx_priv.atpx,,
    pub gpu_id): amdgpu_atpx_switch_i2c_mux(&amdgpu_atpx_priv.atpx,,
    pub gpu_id): amdgpu_atpx_switch_end(&amdgpu_atpx_priv.atpx,,
    pub 0: return,
    }
//
// amdgpu_atpx_power_state - power down/up the requested GPU
//
// @id: GPU to power down/up
// @state: requested power state (0 = off, 1 = on)
//
// Execute the necessary ATPX function to power down/up the discrete GPU
// (all asics).
// Returns 0 on success, error on failure.
//
    static int amdgpu_atpx_power_state(enum vga_switcheroo_client_id id,
    enum vga_switcheroo_state state)
    {
// on w500 ACPI can't change intel gpu state
    if (id == VGA_SWITCHEROO_IGD)
    pub 0: return,
    pub state): amdgpu_atpx_set_discrete_state(&amdgpu_atpx_priv.atpx,,
    pub 0: return,
    }
//
// amdgpu_atpx_pci_probe_handle - look up the ATPX handle
//
// @pdev: pci device
//
// Look up the ATPX handles (all asics).
// Returns true if the handles are found, false if not.
//
#[no_mangle]
unsafe extern "C" fn amdgpu_atpx_pci_probe_handle(pdev: *mut pci_dev) -> bool {
    static bool amdgpu_atpx_pci_probe_handle(struct pci_dev *pdev)
    {
    pub atpx_handle: acpi_handle dhandle,,
    pub status: acpi_status,
    pub ACPI_HANDLE(&pdev->dev): dhandle =,
    if (!dhandle)
    pub false: return,
    pub &atpx_handle): status = acpi_get_handle(dhandle, "ATPX",,
    if (ACPI_FAILURE(status)) {
    pub dhandle: amdgpu_atpx_priv.other_handle =,
    pub false: return,
    }
    pub dhandle: amdgpu_atpx_priv.dhandle =,
    pub atpx_handle: amdgpu_atpx_priv.atpx.handle =,
    pub true: return,
    }
//
// amdgpu_atpx_init - verify the ATPX interface
//
// Verify the ATPX interface (all asics).
// Returns 0 on success, error on failure.
//
#[no_mangle]
unsafe extern "C" fn amdgpu_atpx_init() -> c_int {
    static int amdgpu_atpx_init(void)
    {
    pub r: c_int,
// set up the ATPX handle
    pub amdgpu_atpx_verify_interface(&amdgpu_atpx_priv.atpx): r =,
    if (r)
    pub r: return,
// validate the atpx setup
    pub amdgpu_atpx_validate(&amdgpu_atpx_priv.atpx): r =,
    if (r)
    pub r: return,
    pub 0: return,
    }
//
// amdgpu_atpx_get_client_id - get the client id
//
// @pdev: pci device
//
// look up whether we are the integrated or discrete GPU (all asics).
// Returns the client id.
//
#[no_mangle]
unsafe extern "C" fn amdgpu_atpx_get_client_id(pdev: *mut pci_dev) -> enum vga_switcheroo_client_id {
    static enum vga_switcheroo_client_id amdgpu_atpx_get_client_id(struct pci_dev *pdev)
    {
    if (amdgpu_atpx_priv.dhandle == ACPI_HANDLE(&pdev.dev))
    pub VGA_SWITCHEROO_IGD: return,
    else
    pub VGA_SWITCHEROO_DIS: return,
    }
    static const struct vga_switcheroo_handler amdgpu_atpx_handler = {
    .switchto = amdgpu_atpx_switchto,
    .power_state = amdgpu_atpx_power_state,
    .get_client_id = amdgpu_atpx_get_client_id,
}

    static const struct amdgpu_px_quirk amdgpu_px_quirk_list[] = {
// HG _PR3 doesn't seem to work on this A+A weston board
    { 0x1002, 0x6900, 0x1002, 0x0124, AMDGPU_PX_QUIRK_FORCE_ATPX },
    { 0x1002, 0x6900, 0x1028, 0x0812, AMDGPU_PX_QUIRK_FORCE_ATPX },
    { 0x1002, 0x6900, 0x1028, 0x0813, AMDGPU_PX_QUIRK_FORCE_ATPX },
    { 0x1002, 0x699f, 0x1028, 0x0814, AMDGPU_PX_QUIRK_FORCE_ATPX },
    { 0x1002, 0x6900, 0x1025, 0x125A, AMDGPU_PX_QUIRK_FORCE_ATPX },
    { 0x1002, 0x6900, 0x17AA, 0x3806, AMDGPU_PX_QUIRK_FORCE_ATPX },
    { 0, 0, 0, 0, 0 },
    };
#[no_mangle]
unsafe extern "C" fn amdgpu_atpx_get_quirks(pdev: *mut pci_dev) {
    static void amdgpu_atpx_get_quirks(struct pci_dev *pdev)
    {
    const struct amdgpu_px_quirk *p = amdgpu_px_quirk_list;
// Apply PX quirks
    while (p && p.chip_device != 0) {
    if (pdev.vendor == p.chip_vendor &&
    pdev.device == p.chip_device &&
    pdev.subsystem_vendor == p.subsys_vendor &&
    pdev.subsystem_device == p.subsys_device) {
    amdgpu_atpx_priv.quirks |= p.px_quirk_flags;
    break;
    }
    ++p;
    }
    }
//
// amdgpu_atpx_detect - detect whether we have PX
//
// Check if we have a PX system (all asics).
// Returns true if we have a PX system, false if not.
//
#[no_mangle]
unsafe extern "C" fn amdgpu_atpx_detect() -> bool {
    static bool amdgpu_atpx_detect(void)
    {
    char acpi_method_name[255] = { 0 };
    let mut buffer: acpi_buffer = {sizeof(acpi_method_name), acpi_method_name};
    struct pci_dev *pdev = core::ptr::null_mut();
    let mut has_atpx: bool = false;
    let mut vga_count: c_int = 0;
    let mut d3_supported: bool = false;
    struct pci_dev *parent_pdev;
    while ((pdev = pci_get_class(PCI_CLASS_DISPLAY_VGA << 8, pdev)) != core::ptr::null_mut()) {
    vga_count++;
    has_atpx |= amdgpu_atpx_pci_probe_handle(pdev);
    parent_pdev = pci_upstream_bridge(pdev);
    d3_supported |= parent_pdev && parent_pdev.bridge_d3;
    amdgpu_atpx_get_quirks(pdev);
    }
    while ((pdev = pci_get_class(PCI_CLASS_DISPLAY_OTHER << 8, pdev)) != core::ptr::null_mut()) {
    vga_count++;
    has_atpx |= amdgpu_atpx_pci_probe_handle(pdev);
    parent_pdev = pci_upstream_bridge(pdev);
    d3_supported |= parent_pdev && parent_pdev.bridge_d3;
    amdgpu_atpx_get_quirks(pdev);
    }
    if (has_atpx && vga_count == 2) {
    acpi_get_name(amdgpu_atpx_priv.atpx.handle, ACPI_FULL_PATHNAME, &buffer);
    pr_info("vga_switcheroo: detected switching method %s handle\n",
    acpi_method_name);
    amdgpu_atpx_priv.atpx_detected = true;
    amdgpu_atpx_priv.bridge_pm_usable = d3_supported;
    amdgpu_atpx_init();
    return true;
    }
    return false;
    }
//
// amdgpu_register_atpx_handler - register with vga_switcheroo
//
// Register the PX callbacks with vga_switcheroo (all asics).
//
#[no_mangle]
pub unsafe extern "C" fn amdgpu_register_atpx_handler() {
    void amdgpu_register_atpx_handler(void)
    {
    bool r;
    let mut handler_flags: enum vga_switcheroo_handler_flags_t = 0;
// detect if we have any ATPX + 2 VGA in the system
    r = amdgpu_atpx_detect();
    if (!r)
    return;
    vga_switcheroo_register_handler(&amdgpu_atpx_handler, handler_flags);
    }
//
// amdgpu_unregister_atpx_handler - unregister with vga_switcheroo
//
// Unregister the PX callbacks with vga_switcheroo (all asics).
//
#[no_mangle]
pub unsafe extern "C" fn amdgpu_unregister_atpx_handler() {
    void amdgpu_unregister_atpx_handler(void)
    {
    vga_switcheroo_unregister_handler();
    }
