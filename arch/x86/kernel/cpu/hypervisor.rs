//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/cpu/hypervisor.c
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


//
// Common hypervisor code
//
// Copyright (C) 2008, VMware, Inc.
// Author : Alok N Kataria <akataria@vmware.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY OR FITNESS FOR A PARTICULAR PURPOSE, GOOD TITLE or
// NON INFRINGEMENT.  See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA.
//

    static const __initconst struct hypervisor_x86 * const hypervisors[] =
    {

    &x86_hyper_xen_pv,

    &x86_hyper_xen_hvm,

    &x86_hyper_vmware,
    &x86_hyper_ms_hyperv,

    &x86_hyper_kvm,

    &x86_hyper_jailhouse,

    &x86_hyper_acrn,

    &x86_hyper_bhyve,

    };
    enum x86_hypervisor_type x86_hyper_type;
    EXPORT_SYMBOL(x86_hyper_type);
    bool __initdata nopv;
#[no_mangle]
unsafe extern "C" fn parse_nopv(arg: *mut c_char) -> __init int {
    static __init int parse_nopv(char *arg)
    {
    nopv = true;
    return 0;
    }
    early_param("nopv", parse_nopv);
    static inline const struct hypervisor_x86 * __init
    detect_hypervisor_vendor(void)
    {
    const struct hypervisor_x86 *h = core::ptr::null_mut(), * const *p;
    uint32_t pri, max_pri = 0;
    for (p = hypervisors; p < hypervisors + ARRAY_SIZE(hypervisors); p++) {
    if (unlikely(nopv) && !(*p).ignore_nopv)
    continue;
    pri = (*p).detect();
    if (pri > max_pri) {
    max_pri = pri;
    h = *p;
    }
    }
    if (h)
    pr_info("Hypervisor detected: %s\n", h.name);
    return h;
    }
#[no_mangle]
unsafe extern "C" fn copy_array(src: *const c_void, target: *mut c_void, size: c_uint) -> void __init {
    static void __init copy_array(const void *src, void *target, unsigned int size)
    {
    unsigned int i, n = size / sizeof(void *);
    const void * const *from = (const void * const *)src;
    const void **to = (const void **)target;
    for (i = 0; i < n; i++)
    if (from[i])
    to[i] = from[i];
    }
#[no_mangle]
pub unsafe extern "C" fn init_hypervisor_platform() -> void __init {
    void __init init_hypervisor_platform(void)
    {
    const struct hypervisor_x86 *h;
    h = detect_hypervisor_vendor();
    if (!h)
    return;
    copy_array(&h.init, &x86_init.hyper, sizeof(h.init));
    copy_array(&h.runtime, &x86_platform.hyper, sizeof(h.runtime));
    x86_hyper_type = h.type;
    x86_init.hyper.init_platform();
    }
