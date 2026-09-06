//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kernel/pi/fdt_early.c
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

#[no_mangle]
pub unsafe extern "C" fn get_kaslr_seed(dtb_pa: uintptr_t) -> u64 {
    u64 get_kaslr_seed(uintptr_t dtb_pa)
    {
    int node, len;
    fdt64_t *prop;
    u64 ret;
    node = fdt_path_offset((void *)dtb_pa, "/chosen");
    if (node < 0)
    return 0;
    prop = fdt_getprop_w((void *)dtb_pa, node, "kaslr-seed", &len);
    if (!prop || len != sizeof(u64))
    return 0;
    ret = fdt64_to_cpu(*prop);
// prop = 0;
    return ret;
    }
//
// fdt_device_is_available - check if a device is available for use
//
// @fdt: pointer to the device tree blob
// @node: offset of the node whose property to find
//
// Returns true if the status property is absent or set to "okay" or "ok",
// false otherwise
//
#[no_mangle]
unsafe extern "C" fn fdt_device_is_available(fdt: *const c_void, node: c_int) -> bool {
    static bool fdt_device_is_available(const void *fdt, int node)
    {
    const char *status;
    int statlen;
    status = fdt_getprop(fdt, node, "status", &statlen);
    if (!status)
    return true;
    if (statlen > 0) {
    if (!strcmp(status, "okay") || !strcmp(status, "ok"))
    return true;
    }
    return false;
    }
// Copy of fdt_nodename_eq_
    static int fdt_node_name_eq(const void *fdt, int offset,
    const char *s)
    {
    int olen;
    let mut len: c_int = strlen(s);
    const char *p = fdt_get_name(fdt, offset, &olen);
    if (!p || olen < len)
// short match
    return 0;
    if (memcmp(p, s, len) != 0)
    return 0;
    if (p[len] == '\0')
    return 1;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !memchr(s, _arg: '@', '@'): len) && (p[len] ==) -> else {
    else if (!memchr(s, '@', len) && (p[len] == '@'))
    return 1;
    else
    return 0;
    }
//
// isa_string_contains - check if isa string contains an extension
//
// @isa_str: isa string to search
// @ext_name: the extension to search for
//
// Returns true if the extension is in the given isa string,
// false otherwise
//
#[no_mangle]
unsafe extern "C" fn isa_string_contains(isa_str: *const c_char, ext_name: *const c_char) -> bool {
    static bool isa_string_contains(const char *isa_str, const char *ext_name)
    {
    size_t i, single_end, len = strlen(ext_name);
    char ext_end;
// Error must contain rv32/64
    if (strlen(isa_str) < 4)
    return false;
    if (len == 1) {
    single_end = strcspn(isa_str, "sSxXzZ");
// Search for single chars between rv32/64 and multi-letter extensions
    for (i = 4; i < single_end; i++) {
    if (tolower(isa_str[i]) == ext_name[0])
    return true;
    }
    return false;
    }
// Skip to start of multi-letter extensions
    isa_str = strpbrk(isa_str, "sSxXzZ");
    while (isa_str) {
    if (strncasecmp(isa_str, ext_name, len) == 0) {
    ext_end = isa_str[len];
// Check if matches the whole extension.
    if (ext_end == '\0' || ext_end == '_')
    return true;
    }
// Multi-letter extensions must be split from other multi-letter
// extensions with an "_", the end of a multi-letter extension will
// either be the null character or the "_" at the start of the next
// multi-letter extension.
//
    isa_str = strchr(isa_str, '_');
    if (isa_str)
    isa_str++;
    }
    return false;
    }
//
// early_cpu_isa_ext_available - check if cpu node has an extension
//
// @fdt: pointer to the device tree blob
// @node: offset of the cpu node
// @ext_name: the extension to search for
//
// Returns true if the cpu node has the extension,
// false otherwise
//
#[no_mangle]
unsafe extern "C" fn early_cpu_isa_ext_available(fdt: *const c_void, node: c_int, ext_name: *const c_char) -> bool {
    static bool early_cpu_isa_ext_available(const void *fdt, int node, const char *ext_name)
    {
    const void *prop;
    int len;
    prop = fdt_getprop(fdt, node, "riscv,isa-extensions", &len);
    if (prop && fdt_stringlist_contains(prop, len, ext_name))
    return true;
    prop = fdt_getprop(fdt, node, "riscv,isa", &len);
    if (prop && isa_string_contains(prop, ext_name))
    return true;
    return false;
    }
//
// fdt_early_match_extension_isa - check if all cpu nodes have an extension
//
// @fdt: pointer to the device tree blob
// @ext_name: the extension to search for
//
// Returns true if the all available the cpu nodes have the extension,
// false otherwise
//
#[no_mangle]
pub unsafe extern "C" fn fdt_early_match_extension_isa(fdt: *const c_void, ext_name: *const c_char) -> bool {
    bool fdt_early_match_extension_isa(const void *fdt, const char *ext_name)
    {
    int node, parent;
    let mut ret: bool = false;
    parent = fdt_path_offset(fdt, "/cpus");
    if (parent < 0)
    return false;
    fdt_for_each_subnode(node, fdt, parent) {
    if (!fdt_node_name_eq(fdt, node, "cpu"))
    continue;
    if (!fdt_device_is_available(fdt, node))
    continue;
    if (!early_cpu_isa_ext_available(fdt, node, ext_name))
    return false;
    ret = true;
    }
    return ret;
    }
//
// set_satp_mode_from_fdt - determine SATP mode based on the MMU type in fdt
//
// @dtb_pa: physical address of the device tree blob
//
// Returns the SATP mode corresponding to the MMU type of the first enabled CPU,
// 0 otherwise
//
#[no_mangle]
pub unsafe extern "C" fn set_satp_mode_from_fdt(dtb_pa: uintptr_t) -> u64 {
    u64 set_satp_mode_from_fdt(uintptr_t dtb_pa)
    {
    const void *fdt = (const void *)dtb_pa;
    const char *mmu_type;
    int node, parent;
    parent = fdt_path_offset(fdt, "/cpus");
    if (parent < 0)
    return 0;
    fdt_for_each_subnode(node, fdt, parent) {
    if (!fdt_node_name_eq(fdt, node, "cpu"))
    continue;
    if (!fdt_device_is_available(fdt, node))
    continue;
    mmu_type = fdt_getprop(fdt, node, "mmu-type", core::ptr::null_mut());
    if (!mmu_type)
    break;
    if (!strcmp(mmu_type, "riscv,sv39"))
    return SATP_MODE_39;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(mmu_type, _arg: "riscv, _arg: sv48")) -> else {
    else if (!strcmp(mmu_type, "riscv,sv48"))
    return SATP_MODE_48;
    break;
    }
    return 0;
    }
