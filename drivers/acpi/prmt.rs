//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/prmt.c
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
// Author: Erik Kaneda <erik.kaneda@intel.com>
// Copyright 2020 Intel Corporation
//
// prmt.c
//
// Each PRM service is an executable that is run in a restricted environment
// that is invoked by writing to the PlatformRtMechanism OperationRegion from
// AML bytecode.
//
// init_prmt initializes the Platform Runtime Mechanism (PRM) services by
// processing data in the PRMT as well as registering an ACPI OperationRegion
// handler for the PlatformRtMechanism subtype.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prm_mmio_addr_range {
    pub phys_addr: u64,
    pub virt_addr: u64,
    pub length: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prm_mmio_info {
    pub mmio_count: u64,
    pub addr_ranges: [prm_mmio_addr_range; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prm_buffer {
    pub prm_status: u8,
    pub efi_status: u64,
    pub prm_cmd: u8,
    pub handler_guid: guid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prm_context_buffer {
    pub signature: [c_char; ACPI_NAMESEG_SIZE],
    pub revision: u16,
    pub reserved: u16,
    pub identifier: guid_t,
    pub static_data_buffer: u64,
    pub mmio_ranges: *mut prm_mmio_info,
}

    static LIST_HEAD(prm_module_list);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prm_handler_info {
    pub guid: efi_guid_t,
    pub ): *mut *mut efi_status_t (__efiapi handler_addr)(u64, void,
    pub static_data_buffer_addr: u64,
    pub acpi_param_buffer_addr: u64,
    pub handler_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prm_module_info {
    pub guid: guid_t,
    pub major_rev: u16,
    pub minor_rev: u16,
    pub handler_count: u16,
    pub mmio_info: *mut prm_mmio_info,
    pub updatable: bool,
    pub module_list: list_head,
    pub __counted_by(handler_count): prm_handler_info handlers[],
}

#[no_mangle]
unsafe extern "C" fn efi_pa_va_lookup(guid: *mut efi_guid_t, pa: u64) -> u64 {
    static u64 efi_pa_va_lookup(efi_guid_t *guid, u64 pa)
    {
    efi_memory_desc_t *md;
    let mut pa_offset: u64 = pa & ~PAGE_MASK;
    let mut page: u64 = pa & PAGE_MASK;
    for_each_efi_memory_desc(md) {
    if ((md.attribute & EFI_MEMORY_RUNTIME) &&
    (md.phys_addr < pa && pa < md.phys_addr + PAGE_SIZE * md.num_pages)) {
    return pa_offset + md.virt_addr + page - md.phys_addr;
    }
    }
    return 0;
    }

    static int __init
    acpi_parse_prmt(union acpi_subtable_headers *header, const unsigned long end)
    {
    struct acpi_prmt_module_info *module_info;
    struct acpi_prmt_handler_info *handler_info;
    struct prm_handler_info *th;
    struct prm_module_info *tm;
    u64 *mmio_count;
    let mut cur_handler: u64 = 0;
    let mut module_info_size: u32 = 0;
    let mut mmio_range_size: u64 = 0;
    void *temp_mmio;
    module_info = (struct acpi_prmt_module_info *) header;
    module_info_size = struct_size(tm, handlers, module_info.handler_info_count);
    tm = kmalloc(module_info_size, GFP_KERNEL);
    if (!tm)
    goto parse_prmt_out1;
    guid_copy(&tm.guid, (guid_t *) module_info.module_guid);
    tm.major_rev = module_info.major_rev;
    tm.minor_rev = module_info.minor_rev;
    tm.handler_count = module_info.handler_info_count;
    tm.updatable = true;
    if (module_info.mmio_list_pointer) {
//
// Each module is associated with a list of addr
// ranges that it can use during the service
//
    mmio_count = (u64 *) memremap(module_info.mmio_list_pointer, 8, MEMREMAP_WB);
    if (!mmio_count)
    goto parse_prmt_out2;
    mmio_range_size = struct_size(tm.mmio_info, addr_ranges, *mmio_count);
    tm.mmio_info = kmalloc(mmio_range_size, GFP_KERNEL);
    if (!tm.mmio_info)
    goto parse_prmt_out3;
    temp_mmio = memremap(module_info.mmio_list_pointer, mmio_range_size, MEMREMAP_WB);
    if (!temp_mmio)
    goto parse_prmt_out4;
    memmove(tm.mmio_info, temp_mmio, mmio_range_size);
    } else {
    tm.mmio_info = kmalloc_obj(*tm.mmio_info);
    if (!tm.mmio_info)
    goto parse_prmt_out2;
    tm.mmio_info.mmio_count = 0;
    }
    INIT_LIST_HEAD(&tm.module_list);
    list_add(&tm.module_list, &prm_module_list);
    handler_info = get_first_handler(module_info);
    do {
    th = &tm.handlers[cur_handler];
    guid_copy(&th.guid, (guid_t *)handler_info.handler_guid);
//
// Print an error message if handler_address is NULL, the parse of VA also
// can be skipped.
//
    if (unlikely(!handler_info.handler_address)) {
    pr_info("Skipping handler with core::ptr::null_mut() address for GUID: %pUL",
    (guid_t *)handler_info.handler_guid);
    continue;
    }
    th.handler_addr =
    (void *)efi_pa_va_lookup(&th.guid, handler_info.handler_address);
//
// Print a warning message and skip the parse of VA if handler_addr is zero
// which is not expected to ever happen.
//
    if (unlikely(!th.handler_addr)) {
    pr_warn("Failed to find VA of handler for GUID: %pUL, PA: 0x%llx",
    &th.guid, handler_info.handler_address);
    continue;
    }
    th.static_data_buffer_addr =
    efi_pa_va_lookup(&th.guid, handler_info.static_data_buffer_address);
//
// According to the PRM specification, static_data_buffer_address can be zero,
// so avoid printing a warning message in that case.  Otherwise, if the
// return value of efi_pa_va_lookup() is zero, print the message.
//
    if (unlikely(!th.static_data_buffer_addr && handler_info.static_data_buffer_address))
    pr_warn("Failed to find VA of static data buffer for GUID: %pUL, PA: 0x%llx",
    &th.guid, handler_info.static_data_buffer_address);
    th.acpi_param_buffer_addr =
    efi_pa_va_lookup(&th.guid, handler_info.acpi_param_buffer_address);
//
// According to the PRM specification, acpi_param_buffer_address can be zero,
// so avoid printing a warning message in that case.  Otherwise, if the
// return value of efi_pa_va_lookup() is zero, print the message.
//
    if (unlikely(!th.acpi_param_buffer_addr && handler_info.acpi_param_buffer_address))
    pr_warn("Failed to find VA of acpi param buffer for GUID: %pUL, PA: 0x%llx",
    &th.guid, handler_info.acpi_param_buffer_address);
    } while (++cur_handler < tm.handler_count && (handler_info = get_next_handler(handler_info)));
    return 0;
    parse_prmt_out4:
    kfree(tm.mmio_info);
    parse_prmt_out3:
    memunmap(mmio_count);
    parse_prmt_out2:
    kfree(tm);
    parse_prmt_out1:
    return -ENOMEM;
    }
pub const GET_MODULE: c_int = 0;
pub const GET_HANDLER: c_int = 1;
    static void *find_guid_info(const guid_t *guid, u8 mode)
    {
    struct prm_handler_info *cur_handler;
    struct prm_module_info *cur_module;
    let mut i: c_int = 0;
    list_for_each_entry(cur_module, &prm_module_list, module_list) {
    for (i = 0; i < cur_module.handler_count; ++i) {
    cur_handler = &cur_module.handlers[i];
    if (guid_equal(guid, &cur_handler.guid)) {
    if (mode == GET_MODULE)
    return (void *)cur_module;
    else
    return (void *)cur_handler;
    }
    }
    }
    return core::ptr::null_mut();
    }
    static struct prm_module_info *find_prm_module(const guid_t *guid)
    {
    return (struct prm_module_info *)find_guid_info(guid, GET_MODULE);
    }
    static struct prm_handler_info *find_prm_handler(const guid_t *guid)
    {
    return (struct prm_handler_info *) find_guid_info(guid, GET_HANDLER);
    }
#[no_mangle]
pub unsafe extern "C" fn acpi_prm_handler_available(guid: *const guid_t) -> bool {
    bool acpi_prm_handler_available(const guid_t *guid)
    {
    return find_prm_handler(guid) && find_prm_module(guid);
    }
    EXPORT_SYMBOL_GPL(acpi_prm_handler_available);
// In-coming PRM commands
pub const PRM_CMD_RUN_SERVICE: c_int = 0;
pub const PRM_CMD_START_TRANSACTION: c_int = 1;
pub const PRM_CMD_END_TRANSACTION: c_int = 2;
// statuses that can be passed back to ASL
pub const PRM_HANDLER_SUCCESS: c_int = 0;
pub const PRM_HANDLER_ERROR: c_int = 1;
pub const INVALID_PRM_COMMAND: c_int = 2;
pub const PRM_HANDLER_GUID_NOT_FOUND: c_int = 3;
pub const UPDATE_LOCK_ALREADY_HELD: c_int = 4;
pub const UPDATE_UNLOCK_WITHOUT_LOCK: c_int = 5;
#[no_mangle]
pub unsafe extern "C" fn acpi_call_prm_handler(handler_guid: guid_t, param_buffer: *mut c_void) -> c_int {
    int acpi_call_prm_handler(guid_t handler_guid, void *param_buffer)
    {
    struct prm_handler_info *handler = find_prm_handler(&handler_guid);
    struct prm_module_info *module = find_prm_module(&handler_guid);
    struct prm_context_buffer context;
    efi_status_t status;
    if (!module || !handler)
    return -ENODEV;
    memset(&context, 0, sizeof(context));
    ACPI_COPY_NAMESEG(context.signature, "PRMC");
    context.identifier         = handler.guid;
    context.static_data_buffer = handler.static_data_buffer_addr;
    context.mmio_ranges        = module.mmio_info;
    status = efi_call_acpi_prm_handler(handler.handler_addr,
    (u64)param_buffer,
    &context);
    return efi_status_to_err(status);
    }
    EXPORT_SYMBOL_GPL(acpi_call_prm_handler);
//
// This is the PlatformRtMechanism opregion space handler.
// @function: indicates the read/write. In fact as the PlatformRtMechanism
// message is driven by command, only write is meaningful.
//
// @addr   : not used
// @bits   : not used.
// @value  : it is an in/out parameter. It points to the PRM message buffer.
// @handler_context: not used
//
    static acpi_status acpi_platformrt_space_handler(u32 function,
    acpi_physical_address addr,
    u32 bits, acpi_integer *value,
    void *handler_context,
    void *region_context)
    {
    struct prm_buffer *buffer = ACPI_CAST_PTR(struct prm_buffer, value);
    struct prm_handler_info *handler;
    struct prm_module_info *module;
    efi_status_t status;
    struct prm_context_buffer context;
    if (!efi_enabled(EFI_RUNTIME_SERVICES)) {
    pr_err_ratelimited("PRM: EFI runtime services no longer available\n");
    return AE_NO_HANDLER;
    }
//
// The returned acpi_status will always be AE_OK. Error values will be
// saved in the first byte of the PRM message buffer to be used by ASL.
//
    switch (buffer.prm_cmd) {
    case PRM_CMD_RUN_SERVICE:
    handler = find_prm_handler(&buffer.handler_guid);
    module = find_prm_module(&buffer.handler_guid);
    if (!handler || !module)
    goto invalid_guid;
    if (!handler.handler_addr) {
    buffer.prm_status = PRM_HANDLER_ERROR;
    return AE_OK;
    }
    ACPI_COPY_NAMESEG(context.signature, "PRMC");
    context.revision = 0x0;
    context.reserved = 0x0;
    context.identifier = handler.guid;
    context.static_data_buffer = handler.static_data_buffer_addr;
    context.mmio_ranges = module.mmio_info;
    status = efi_call_acpi_prm_handler(handler.handler_addr,
    handler.acpi_param_buffer_addr,
    &context);
    if (status == EFI_SUCCESS) {
    buffer.prm_status = PRM_HANDLER_SUCCESS;
    } else {
    buffer.prm_status = PRM_HANDLER_ERROR;
    buffer.efi_status = status;
    }
    break;
    case PRM_CMD_START_TRANSACTION:
    module = find_prm_module(&buffer.handler_guid);
    if (!module)
    goto invalid_guid;
    if (module.updatable)
    module.updatable = false;
    else
    buffer.prm_status = UPDATE_LOCK_ALREADY_HELD;
    break;
    case PRM_CMD_END_TRANSACTION:
    module = find_prm_module(&buffer.handler_guid);
    if (!module)
    goto invalid_guid;
    if (module.updatable)
    buffer.prm_status = UPDATE_UNLOCK_WITHOUT_LOCK;
    else
    module.updatable = true;
    break;
    default:
    buffer.prm_status = INVALID_PRM_COMMAND;
    break;
    }
    return AE_OK;
    invalid_guid:
    buffer.prm_status = PRM_HANDLER_GUID_NOT_FOUND;
    return AE_OK;
    }
#[no_mangle]
pub unsafe extern "C" fn init_prmt() -> void __init {
    void __init init_prmt(void)
    {
    struct acpi_table_header *tbl;
    acpi_status status;
    int mc;
    status = acpi_get_table(ACPI_SIG_PRMT, 0, &tbl);
    if (ACPI_FAILURE(status))
    return;
    mc = acpi_table_parse_entries(ACPI_SIG_PRMT, sizeof(struct acpi_table_prmt) +
    sizeof (struct acpi_table_prmt_header),
    0, acpi_parse_prmt, 0);
    acpi_put_table(tbl);
//
// Return immediately if PRMT table is not present or no PRM module found.
//
    if (mc <= 0)
    return;
    pr_info("PRM: found %u modules\n", mc);
    if (!efi_enabled(EFI_RUNTIME_SERVICES)) {
    pr_err("PRM: EFI runtime services unavailable\n");
    return;
    }
    status = acpi_install_address_space_handler(ACPI_ROOT_OBJECT,
    ACPI_ADR_SPACE_PLATFORM_RT,
    &acpi_platformrt_space_handler,
    core::ptr::null_mut(), core::ptr::null_mut());
    if (ACPI_FAILURE(status))
    pr_alert("PRM: OperationRegion handler could not be installed\n");
    }
