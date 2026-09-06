//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/nospec-branch.c
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

    let mut nobp: c_int = IS_ENABLED(CONFIG_KERNEL_NOBP);
#[no_mangle]
unsafe extern "C" fn nobp_setup_early(str: *mut c_char) -> int __init {
    static int __init nobp_setup_early(char *str)
    {
    bool enabled;
    int rc;
    rc = kstrtobool(str, &enabled);
    if (rc)
    return rc;
    if (enabled && test_facility(82)) {
//
// The user explicitly requested nobp=1, enable it and
// disable the expoline support.
//
    nobp = 1;
    if (IS_ENABLED(CONFIG_EXPOLINE))
    nospec_disable = 1;
    } else {
    nobp = 0;
    }
    return 0;
    }
    early_param("nobp", nobp_setup_early);
#[no_mangle]
unsafe extern "C" fn nospec_setup_early(str: *mut c_char) -> int __init {
    static int __init nospec_setup_early(char *str)
    {
    nobp = 0;
    return 0;
    }
    early_param("nospec", nospec_setup_early);
#[no_mangle]
unsafe extern "C" fn nospec_report() -> int __init {
    static int __init nospec_report(void)
    {
    if (test_facility(156))
    pr_info("Spectre V2 mitigation: etokens\n");
    if (nospec_uses_trampoline())
    pr_info("Spectre V2 mitigation: execute trampolines\n");
    if (nobp_enabled())
    pr_info("Spectre V2 mitigation: limited branch prediction\n");
    return 0;
    }
    arch_initcall(nospec_report);

    let mut nospec_disable: c_int = IS_ENABLED(CONFIG_EXPOLINE_OFF);
#[no_mangle]
unsafe extern "C" fn nospectre_v2_setup_early(str: *mut c_char) -> int __init {
    static int __init nospectre_v2_setup_early(char *str)
    {
    nospec_disable = 1;
    return 0;
    }
    early_param("nospectre_v2", nospectre_v2_setup_early);
#[no_mangle]
pub unsafe extern "C" fn nospec_auto_detect() -> void __init {
    void __init nospec_auto_detect(void)
    {
    if (test_facility(156) || cpu_mitigations_off()) {
//
// The machine supports etokens.
// Disable expolines and disable nobp.
//
    if (__is_defined(CC_USING_EXPOLINE))
    nospec_disable = 1;
    nobp = 0;
    } else if (__is_defined(CC_USING_EXPOLINE)) {
//
// The kernel has been compiled with expolines.
// Keep expolines enabled and disable nobp.
//
    nospec_disable = 0;
    nobp = 0;
    }
//
// If the kernel has not been compiled with expolines the
// nobp setting decides what is done, this depends on the
// CONFIG_KERNEL_NP option and the nobp/nospec parameters.
//
    }
#[no_mangle]
unsafe extern "C" fn spectre_v2_setup_early(str: *mut c_char) -> int __init {
    static int __init spectre_v2_setup_early(char *str)
    {
    if (str && !strncmp(str, "on", 2)) {
    nospec_disable = 0;
    nobp = 0;
    }
    if (str && !strncmp(str, "off", 3))
    nospec_disable = 1;
    if (str && !strncmp(str, "auto", 4))
    nospec_auto_detect();
    return 0;
    }
    early_param("spectre_v2", spectre_v2_setup_early);
#[no_mangle]
unsafe extern "C" fn __nospec_revert(start: *mut i32, end: *mut i32) -> void __init_or_module {
    static void __init_or_module __nospec_revert(s32 *start, s32 *end)
    {
    enum { BRCL_EXPOLINE, BRASL_EXPOLINE } type;
    static const u8 branch[] = { 0x47, 0x00, 0x07, 0x00 };
    u8 *instr, *thunk, *br;
    u8 insnbuf[6];
    s32 *epo;
// Second part of the instruction replace is always a nop
    memcpy(insnbuf + 2, branch, sizeof(branch));
    for (epo = start; epo < end; epo++) {
    instr = (u8 *) epo + *epo;
    if (instr[0] == 0xc0 && (instr[1] & 0x0f) == 0x04)
    type = BRCL_EXPOLINE;	/* brcl instruction */
#[no_mangle]
pub unsafe extern "C" fn if(0x05: instr[0] == 0xc0 && (instr[1] & 0x0f) ==) -> else {
    else if (instr[0] == 0xc0 && (instr[1] & 0x0f) == 0x05)
    type = BRASL_EXPOLINE;	/* brasl instruction */
    else
    continue;
    thunk = instr + (long)(*(int *)(instr + 2)) * 2;
    if (thunk[0] == 0xc6 && thunk[1] == 0x00)
// exrl %r0,<target-br>
    br = thunk + (long)(*(int *)(thunk + 2)) * 2;
    else
    continue;
    if (br[0] != 0x07 || (br[1] & 0xf0) != 0xf0)
    continue;
    switch (type) {
    case BRCL_EXPOLINE:
// brcl to thunk, replace with br + nop
    insnbuf[0] = br[0];
    insnbuf[1] = (instr[1] & 0xf0) | (br[1] & 0x0f);
    break;
    case BRASL_EXPOLINE:
// brasl to thunk, replace with basr + nop
    insnbuf[0] = 0x0d;
    insnbuf[1] = (instr[1] & 0xf0) | (br[1] & 0x0f);
    break;
    }
    s390_kernel_write(instr, insnbuf, 6);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn nospec_revert(start: *mut i32, end: *mut i32) -> void __init_or_module {
    void __init_or_module nospec_revert(s32 *start, s32 *end)
    {
    if (nospec_disable)
    __nospec_revert(start, end);
    }
    extern s32 __nospec_call_start[], __nospec_call_end[];
    extern s32 __nospec_return_start[], __nospec_return_end[];
#[no_mangle]
pub unsafe extern "C" fn nospec_init_branches() -> void __init {
    void __init nospec_init_branches(void)
    {
    nospec_revert(__nospec_call_start, __nospec_call_end);
    nospec_revert(__nospec_return_start, __nospec_return_end);
    }
