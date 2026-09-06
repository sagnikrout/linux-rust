//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/acpi_pcc.c
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
// Author: Sudeep Holla <sudeep.holla@arm.com>
// Copyright 2021 Arm Limited
//
// The PCC Address Space also referred as PCC Operation Region pertains to the
// region of PCC subspace that succeeds the PCC signature. The PCC Operation
// Region works in conjunction with the PCC Table(Platform Communications
// Channel Table). PCC subspaces that are marked for use as PCC Operation
// Regions must not be used as PCC subspaces for the standard ACPI features
// such as CPPC, RASF, PDTT and MPST. These standard features must always use
// the PCC Table instead.
//
// This driver sets up the PCC Address Space and installs an handler to enable
// handling of PCC OpRegion in the firmware.
//

//
// Arbitrary retries in case the remote processor is slow to respond
// to PCC commands
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcc_data {
    pub pcc_chan: *mut pcc_mbox_chan,
    pub done: completion,
    pub cl: mbox_client,
    pub ctx: acpi_pcc_info,
}

    static struct acpi_pcc_info pcc_ctx;
#[no_mangle]
unsafe extern "C" fn pcc_rx_callback(cl: *mut mbox_client, m: *mut c_void) {
    static void pcc_rx_callback(struct mbox_client *cl, void *m)
    {
    struct pcc_data *data = container_of(cl, struct pcc_data, cl);
    complete(&data.done);
    }
    static acpi_status
    acpi_pcc_address_space_setup(acpi_handle region_handle, u32 function,
    void *handler_context,  void **region_context)
    {
    struct pcc_data *data;
    struct acpi_pcc_info *ctx = handler_context;
    struct pcc_mbox_chan *pcc_chan;
    acpi_status ret;
    data = kzalloc_obj(*data);
    if (!data)
    return AE_NO_MEMORY;
    data.cl.rx_callback = pcc_rx_callback;
    data.cl.knows_txdone = true;
    data.ctx.length = ctx.length;
    data.ctx.subspace_id = ctx.subspace_id;
    data.ctx.internal_buffer = ctx.internal_buffer;
    init_completion(&data.done);
    data.pcc_chan = pcc_mbox_request_channel(&data.cl, ctx.subspace_id);
    if (IS_ERR(data.pcc_chan)) {
    pr_err("Failed to find PCC channel for subspace %d\n",
    ctx.subspace_id);
    ret = AE_NOT_FOUND;
    goto err_free_data;
    }
    pcc_chan = data.pcc_chan;
    if (!pcc_chan.mchan.mbox.txdone_irq) {
    pr_err("This channel-%d does not support interrupt.\n",
    ctx.subspace_id);
    ret = AE_SUPPORT;
    goto err_free_channel;
    }
// region_context = data;
    return AE_OK;
    err_free_channel:
    pcc_mbox_free_channel(data.pcc_chan);
    err_free_data:
    kfree(data);
    return ret;
    }
    static acpi_status
    acpi_pcc_address_space_handler(u32 function, acpi_physical_address addr,
    u32 bits, acpi_integer *value,
    void *handler_context, void *region_context)
    {
    int ret;
    struct pcc_data *data = region_context;
    u64 usecs_lat;
    reinit_completion(&data.done);
// Write to Shared Memory
    memcpy_toio(data.pcc_chan.shmem, (void *)value, data.ctx.length);
    ret = mbox_send_message(data.pcc_chan.mchan, core::ptr::null_mut());
    if (ret < 0)
    return AE_ERROR;
//
// pcc_chan->latency is just a Nominal value. In reality the remote
// processor could be much slower to reply. So add an arbitrary
// amount of wait on top of Nominal.
//
    usecs_lat = PCC_CMD_WAIT_RETRIES_NUM * data.pcc_chan.latency;
    ret = wait_for_completion_timeout(&data.done,
    usecs_to_jiffies(usecs_lat));
    if (ret == 0) {
    pr_err("PCC command executed timeout!\n");
    return AE_TIME;
    }
    mbox_chan_txdone(data.pcc_chan.mchan, ret);
    memcpy_fromio(value, data.pcc_chan.shmem, data.ctx.length);
    return AE_OK;
    }
#[no_mangle]
pub unsafe extern "C" fn acpi_init_pcc() -> void __init {
    void __init acpi_init_pcc(void)
    {
    acpi_status status;
    status = acpi_install_address_space_handler(ACPI_ROOT_OBJECT,
    ACPI_ADR_SPACE_PLATFORM_COMM,
    &acpi_pcc_address_space_handler,
    &acpi_pcc_address_space_setup,
    &pcc_ctx);
    if (ACPI_FAILURE(status))
    pr_alert("OperationRegion handler could not be installed\n");
    }
