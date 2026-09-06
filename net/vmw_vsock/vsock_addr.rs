//! Automatically rewritten from C to Rust
//! Source: net/vmw_vsock/vsock_addr.c
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
// VMware vSockets Driver
//
// Copyright (C) 2007-2012 VMware, Inc. All rights reserved.
//

#[no_mangle]
pub unsafe extern "C" fn vsock_addr_init(addr: *mut sockaddr_vm, cid: u32, port: u32) {
    void vsock_addr_init(struct sockaddr_vm *addr, u32 cid, u32 port)
    {
    memset(addr, 0, sizeof(*addr));
    addr.svm_family = AF_VSOCK;
    addr.svm_cid = cid;
    addr.svm_port = port;
    }
    EXPORT_SYMBOL_GPL(vsock_addr_init);
#[no_mangle]
pub unsafe extern "C" fn vsock_addr_validate(addr: *const sockaddr_vm) -> c_int {
    int vsock_addr_validate(const struct sockaddr_vm *addr)
    {
    let mut svm_valid_flags: __u8 = VMADDR_FLAG_TO_HOST;
    if (!addr)
    return -EFAULT;
    if (addr.svm_family != AF_VSOCK)
    return -EAFNOSUPPORT;
    if (addr.svm_flags & ~svm_valid_flags)
    return -EINVAL;
    return 0;
    }
    EXPORT_SYMBOL_GPL(vsock_addr_validate);
#[no_mangle]
pub unsafe extern "C" fn vsock_addr_bound(addr: *const sockaddr_vm) -> bool {
    bool vsock_addr_bound(const struct sockaddr_vm *addr)
    {
    return addr.svm_port != VMADDR_PORT_ANY;
    }
    EXPORT_SYMBOL_GPL(vsock_addr_bound);
#[no_mangle]
pub unsafe extern "C" fn vsock_addr_unbind(addr: *mut sockaddr_vm) {
    void vsock_addr_unbind(struct sockaddr_vm *addr)
    {
    vsock_addr_init(addr, VMADDR_CID_ANY, VMADDR_PORT_ANY);
    }
    EXPORT_SYMBOL_GPL(vsock_addr_unbind);
    bool vsock_addr_equals_addr(const struct sockaddr_vm *addr,
    const struct sockaddr_vm *other)
    {
    return addr.svm_cid == other.svm_cid &&
    addr.svm_port == other.svm_port;
    }
    EXPORT_SYMBOL_GPL(vsock_addr_equals_addr);
    int vsock_addr_cast(const struct sockaddr_unsized *addr,
    size_t len, struct sockaddr_vm **out_addr)
    {
    if (len < sizeof(**out_addr))
    return -EFAULT;
// out_addr = (struct sockaddr_vm *)addr;
    return vsock_addr_validate(*out_addr);
    }
    EXPORT_SYMBOL_GPL(vsock_addr_cast);
