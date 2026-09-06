//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/boot/ps3.c
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
// PS3 bootwrapper support.
//
// Copyright (C) 2007 Sony Computer Entertainment Inc.
// Copyright 2007 Sony Corp.
//

    extern int lv1_panic(u64 in_1);
    extern int lv1_get_logical_partition_id(u64 *out_1);
    extern int lv1_get_logical_ppe_id(u64 *out_1);
    extern int lv1_get_repository_node_value(u64 in_1, u64 in_2, u64 in_3,
    u64 in_4, u64 in_5, u64 *out_1, u64 *out_2);
    BSS_STACK(4096);
// A buffer that may be edited by tools operating on a zImage binary so as to
// edit the command line passed to vmlinux (by setting /chosen/bootargs).
// The buffer is put in its own section so that tools may locate it easier.
//
    static char cmdline[BOOT_COMMAND_LINE_SIZE]
    __attribute__((__section__("__builtin_cmdline")));
#[no_mangle]
unsafe extern "C" fn prep_cmdline(chosen: *mut c_void) {
    static void prep_cmdline(void *chosen)
    {
    if (cmdline[0] == '\0')
    getprop(chosen, "bootargs", cmdline, BOOT_COMMAND_LINE_SIZE-1);
    else
    setprop_str(chosen, "bootargs", cmdline);
    printf("cmdline: '%s'\n", cmdline);
    }
#[no_mangle]
unsafe extern "C" fn ps3_console_write(buf: *const c_char, len: c_int) {
    static void ps3_console_write(const char *buf, int len)
    {
    }
#[no_mangle]
unsafe extern "C" fn ps3_exit() {
    static void ps3_exit(void)
    {
    printf("ps3_exit\n");
// lv1_panic will shutdown the lpar.
    lv1_panic(0); /* zero = do not reboot */
    while (1);
    }
#[no_mangle]
unsafe extern "C" fn ps3_repository_read_rm_size(rm_size: *mut u64) -> c_int {
    static int ps3_repository_read_rm_size(u64 *rm_size)
    {
    int result;
    u64 lpar_id;
    u64 ppe_id;
    u64 v2;
    result = lv1_get_logical_partition_id(&lpar_id);
    if (result)
    return -1;
    result = lv1_get_logical_ppe_id(&ppe_id);
    if (result)
    return -1;
//
// n1: 0000000062690000 : ....bi..
// n2: 7075000000000000 : pu......
// n3: 0000000000000001 : ........
// n4: 726d5f73697a6500 : rm_size.
//
    result = lv1_get_repository_node_value(lpar_id, 0x0000000062690000ULL,
    0x7075000000000000ULL, ppe_id, 0x726d5f73697a6500ULL, rm_size,
    &v2);
    printf("%s:%d: ppe_id  %lu \n", __func__, __LINE__,
    (unsigned long)ppe_id);
    printf("%s:%d: lpar_id %lu \n", __func__, __LINE__,
    (unsigned long)lpar_id);
    printf("%s:%d: rm_size %llxh \n", __func__, __LINE__, *rm_size);
    return result ? -1 : 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ps3_copy_vectors() {
    void ps3_copy_vectors(void)
    {
    extern char __system_reset_kernel[];
    memcpy((void *)0x100, __system_reset_kernel, 512);
    flush_cache((void *)0x100, 512);
    }
#[no_mangle]
pub unsafe extern "C" fn platform_init() {
    void platform_init(void)
    {
    const u32 heapsize = 0x1000000 - (u32)_end; /* 16MiB */
    void *chosen;
    unsigned long ft_addr;
    u64 rm_size;
    console_ops.write = ps3_console_write;
    platform_ops.exit = ps3_exit;
    printf("\n-- PS3 bootwrapper --\n");
    simple_alloc_init(_end, heapsize, 32, 64);
    fdt_init(_dtb_start);
    chosen = finddevice("/chosen");
    ps3_repository_read_rm_size(&rm_size);
    dt_fixup_memory(0, rm_size);
    if (&_initrd_end > &_initrd_start) {
    setprop_val(chosen, "linux,initrd-start", (u32)(_initrd_start));
    setprop_val(chosen, "linux,initrd-end", (u32)(_initrd_end));
    }
    prep_cmdline(chosen);
    ft_addr = dt_ops.finalize();
    ps3_copy_vectors();
    printf(" flat tree at 0x%lx\n\r", ft_addr);
    ((kernel_entry_t)0)(ft_addr, 0, core::ptr::null_mut());
    ps3_exit();
    }
