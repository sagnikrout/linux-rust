//! Automatically rewritten from C to Rust
//! Source: drivers/nvme/host/fault_inject.c
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
// fault injection support for nvme.
//
// Copyright (c) 2018, Oracle and/or its affiliates
//

    static DECLARE_FAULT_ATTR(fail_default_attr);
// optional fault injection attributes boot time option:
// nvme_core.fail_request=<interval>,<probability>,<space>,<times>
//
    static char *fail_request;
    module_param(fail_request, charp, 0000);
    void nvme_fault_inject_init(struct nvme_fault_inject *fault_inj,
    const char *dev_name)
    {
    struct dentry *dir, *parent;
    struct fault_attr *attr = &fault_inj.attr;
// set default fault injection attribute
    if (fail_request)
    setup_fault_attr(&fail_default_attr, fail_request);
// create debugfs directory and attribute
    parent = debugfs_create_dir(dev_name, core::ptr::null_mut());
    if (IS_ERR(parent)) {
    pr_warn("%s: failed to create debugfs directory\n", dev_name);
    return;
    }
// attr = fail_default_attr;
    dir = fault_create_debugfs_attr("fault_inject", parent, attr);
    if (IS_ERR(dir)) {
    pr_warn("%s: failed to create debugfs attr\n", dev_name);
    debugfs_remove_recursive(parent);
    return;
    }
    fault_inj.parent = parent;
// create debugfs for opcode, status code, and dont_retry
    fault_inj.opcode = 0xffff;
    fault_inj.status = NVME_SC_INVALID_OPCODE;
    fault_inj.dont_retry = true;
    debugfs_create_x16("opcode", 0600, dir,	&fault_inj.opcode);
    debugfs_create_x16("status", 0600, dir,	&fault_inj.status);
    debugfs_create_bool("dont_retry", 0600, dir, &fault_inj.dont_retry);
    }
#[no_mangle]
pub unsafe extern "C" fn nvme_fault_inject_fini(fault_inject: *mut nvme_fault_inject) {
    void nvme_fault_inject_fini(struct nvme_fault_inject *fault_inject)
    {
// remove debugfs directories
    debugfs_remove_recursive(fault_inject.parent);
    }
#[no_mangle]
pub unsafe extern "C" fn nvme_should_fail(req: *mut request) {
    void nvme_should_fail(struct request *req)
    {
    struct gendisk *disk = req.q.disk;
    struct nvme_fault_inject *fault_inject = core::ptr::null_mut();
    struct nvme_command *cmd = nvme_req(req).cmd;
    u16 status;
    if (disk) {
    struct nvme_ns *ns = disk.private_data;
    if (ns)
    fault_inject = &ns.fault_inject;
    else
    WARN_ONCE(1, "No namespace found for request\n");
    } else {
    fault_inject = &nvme_req(req).ctrl.fault_inject;
    }
    if (!fault_inject)
    return;
    if (fault_inject.opcode <= 0xff &&
    fault_inject.opcode != cmd.common.opcode)
    return;
    if (should_fail(&fault_inject.attr, 1)) {
// inject status code and DNR bit
    status = fault_inject.status;
    if (fault_inject.dont_retry)
    status |= NVME_STATUS_DNR;
    nvme_req(req).status =	status;
    }
    }
    EXPORT_SYMBOL_GPL(nvme_should_fail);
