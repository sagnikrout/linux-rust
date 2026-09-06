//! Automatically rewritten from C to Rust
//! Source: rust/helpers/io.c
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

    __rust_helper void __iomem *rust_helper_ioremap(phys_addr_t offset, size_t size)
    {
    return ioremap(offset, size);
    }
    __rust_helper void __iomem *rust_helper_ioremap_np(phys_addr_t offset,
    size_t size)
    {
    return ioremap_np(offset, size);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_iounmap(addr: *mut void __iomem) -> __rust_helper void {
    __rust_helper void rust_helper_iounmap(void __iomem *addr)
    {
    iounmap(addr);
    }

    __rust_helper void rust_helper_memcpy_fromio(void *dst,
    const volatile void __iomem *src,
    size_t count)
    {
    memcpy_fromio(dst, src, count);
    }
    __rust_helper void rust_helper_memcpy_toio(volatile void __iomem *dst,
    const void *src, size_t count)
    {
    memcpy_toio(dst, src, count);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_readb(addr: *const void __iomem) -> __rust_helper u8 {
    __rust_helper u8 rust_helper_readb(const void __iomem *addr)
    {
    return readb(addr);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_readw(addr: *const void __iomem) -> __rust_helper u16 {
    __rust_helper u16 rust_helper_readw(const void __iomem *addr)
    {
    return readw(addr);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_readl(addr: *const void __iomem) -> __rust_helper u32 {
    __rust_helper u32 rust_helper_readl(const void __iomem *addr)
    {
    return readl(addr);
    }

#[no_mangle]
pub unsafe extern "C" fn rust_helper_readq(addr: *const void __iomem) -> __rust_helper u64 {
    __rust_helper u64 rust_helper_readq(const void __iomem *addr)
    {
    return readq(addr);
    }

#[no_mangle]
pub unsafe extern "C" fn rust_helper_writeb(value: u8, addr: *mut void __iomem) -> __rust_helper void {
    __rust_helper void rust_helper_writeb(u8 value, void __iomem *addr)
    {
    writeb(value, addr);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_writew(value: u16, addr: *mut void __iomem) -> __rust_helper void {
    __rust_helper void rust_helper_writew(u16 value, void __iomem *addr)
    {
    writew(value, addr);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_writel(value: u32, addr: *mut void __iomem) -> __rust_helper void {
    __rust_helper void rust_helper_writel(u32 value, void __iomem *addr)
    {
    writel(value, addr);
    }

#[no_mangle]
pub unsafe extern "C" fn rust_helper_writeq(value: u64, addr: *mut void __iomem) -> __rust_helper void {
    __rust_helper void rust_helper_writeq(u64 value, void __iomem *addr)
    {
    writeq(value, addr);
    }

#[no_mangle]
pub unsafe extern "C" fn rust_helper_readb_relaxed(addr: *const void __iomem) -> __rust_helper u8 {
    __rust_helper u8 rust_helper_readb_relaxed(const void __iomem *addr)
    {
    return readb_relaxed(addr);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_readw_relaxed(addr: *const void __iomem) -> __rust_helper u16 {
    __rust_helper u16 rust_helper_readw_relaxed(const void __iomem *addr)
    {
    return readw_relaxed(addr);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_readl_relaxed(addr: *const void __iomem) -> __rust_helper u32 {
    __rust_helper u32 rust_helper_readl_relaxed(const void __iomem *addr)
    {
    return readl_relaxed(addr);
    }

#[no_mangle]
pub unsafe extern "C" fn rust_helper_readq_relaxed(addr: *const void __iomem) -> __rust_helper u64 {
    __rust_helper u64 rust_helper_readq_relaxed(const void __iomem *addr)
    {
    return readq_relaxed(addr);
    }

#[no_mangle]
pub unsafe extern "C" fn rust_helper_writeb_relaxed(value: u8, addr: *mut void __iomem) -> __rust_helper void {
    __rust_helper void rust_helper_writeb_relaxed(u8 value, void __iomem *addr)
    {
    writeb_relaxed(value, addr);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_writew_relaxed(value: u16, addr: *mut void __iomem) -> __rust_helper void {
    __rust_helper void rust_helper_writew_relaxed(u16 value, void __iomem *addr)
    {
    writew_relaxed(value, addr);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_writel_relaxed(value: u32, addr: *mut void __iomem) -> __rust_helper void {
    __rust_helper void rust_helper_writel_relaxed(u32 value, void __iomem *addr)
    {
    writel_relaxed(value, addr);
    }

#[no_mangle]
pub unsafe extern "C" fn rust_helper_writeq_relaxed(value: u64, addr: *mut void __iomem) -> __rust_helper void {
    __rust_helper void rust_helper_writeq_relaxed(u64 value, void __iomem *addr)
    {
    writeq_relaxed(value, addr);
    }

#[no_mangle]
pub unsafe extern "C" fn rust_helper_resource_size(res: *mut resource) -> __rust_helper resource_size_t {
    __rust_helper resource_size_t rust_helper_resource_size(struct resource *res)
    {
    return resource_size(res);
    }
    __rust_helper struct resource *
    rust_helper_request_mem_region(resource_size_t start, resource_size_t n,
    const char *name)
    {
    return request_mem_region(start, n, name);
    }
    __rust_helper void rust_helper_release_mem_region(resource_size_t start,
    resource_size_t n)
    {
    release_mem_region(start, n);
    }
    __rust_helper struct resource *rust_helper_request_region(resource_size_t start,
    resource_size_t n,
    const char *name)
    {
    return request_region(start, n, name);
    }
    __rust_helper struct resource *
    rust_helper_request_muxed_region(resource_size_t start, resource_size_t n,
    const char *name)
    {
    return request_muxed_region(start, n, name);
    }
    __rust_helper void rust_helper_release_region(resource_size_t start,
    resource_size_t n)
    {
    release_region(start, n);
    }
