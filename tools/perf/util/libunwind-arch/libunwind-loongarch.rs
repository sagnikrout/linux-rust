//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/libunwind-arch/libunwind-loongarch.c
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

#[no_mangle]
pub unsafe extern "C" fn __get_perf_regnum_for_unw_regnum_loongarch(__maybe_unused: int unw_regnum) -> c_int {
    int __get_perf_regnum_for_unw_regnum_loongarch(int unw_regnum __maybe_unused)
    {

    return -EINVAL;

    switch (unw_regnum) {
    case UNW_LOONGARCH64_R1 ... UNW_LOONGARCH64_R31:
    return unw_regnum - UNW_LOONGARCH64_R1 + PERF_REG_LOONGARCH_R1;
    case UNW_LOONGARCH64_PC:
    return PERF_REG_LOONGARCH_PC;
    default:
    pr_err("unwind: invalid reg id %d\n", unw_regnum);
    return -EINVAL;
    }

    }
#[no_mangle]
pub unsafe extern "C" fn __libunwind_arch__flush_access_loongarch(__maybe_unused: *mut *mut maps maps) {
    void __libunwind_arch__flush_access_loongarch(struct maps *maps __maybe_unused)
    {

    unw_flush_cache(maps__addr_space(maps), 0, 0);

    }
#[no_mangle]
pub unsafe extern "C" fn __libunwind_arch__finish_access_loongarch(__maybe_unused: *mut *mut maps maps) {
    void __libunwind_arch__finish_access_loongarch(struct maps *maps __maybe_unused)
    {

    unw_destroy_addr_space(maps__addr_space(maps));

    }

    static int find_proc_info(unw_addr_space_t as, unw_word_t ip, unw_proc_info_t *pi,
    int need_unwind_info, void *arg)
    {
    return __libunwind__find_proc_info(as, ip, pi, need_unwind_info, arg);
    }
    static void put_unwind_info(unw_addr_space_t __maybe_unused as,
    unw_proc_info_t *pi __maybe_unused,
    void *arg __maybe_unused)
    {
    pr_debug("unwind: put_unwind_info called\n");
    }
    static int get_dyn_info_list_addr(unw_addr_space_t __maybe_unused as,
    unw_word_t __maybe_unused *dil_addr,
    void __maybe_unused *arg)
    {
    return -UNW_ENOINFO;
    }
    static int access_mem(unw_addr_space_t as, unw_word_t addr, unw_word_t *valp,
    int __write, void *arg)
    {
    return __libunwind__access_mem(as, addr, valp, __write, arg);
    }
    static int access_reg(unw_addr_space_t as, unw_regnum_t regnum, unw_word_t *valp,
    int __write, void *arg)
    {
    return __libunwind__access_reg(as, regnum, valp, __write, arg);
    }
    static int access_fpreg(unw_addr_space_t __maybe_unused as,
    unw_regnum_t __maybe_unused num,
    unw_fpreg_t __maybe_unused *val,
    int __maybe_unused __write,
    void __maybe_unused *arg)
    {
    pr_err("unwind: access_fpreg unsupported\n");
    return -UNW_EINVAL;
    }
    static int resume(unw_addr_space_t __maybe_unused as,
    unw_cursor_t __maybe_unused *cu,
    void __maybe_unused *arg)
    {
    pr_err("unwind: resume unsupported\n");
    return -UNW_EINVAL;
    }
    static int get_proc_name(unw_addr_space_t __maybe_unused as,
    unw_word_t __maybe_unused addr,
    char __maybe_unused *bufp, size_t __maybe_unused buf_len,
    unw_word_t __maybe_unused *offp, void __maybe_unused *arg)
    {
    pr_err("unwind: get_proc_name unsupported\n");
    return -UNW_EINVAL;
    }

    void *__libunwind_arch__create_addr_space_loongarch(void)
    {

    static unw_accessors_t accessors = {
    .find_proc_info		= find_proc_info,
    .put_unwind_info	= put_unwind_info,
    .get_dyn_info_list_addr	= get_dyn_info_list_addr,
    .access_mem		= access_mem,
    .access_reg		= access_reg,
    .access_fpreg		= access_fpreg,
    .resume			= resume,
    .get_proc_name		= get_proc_name,
    };
    unw_addr_space_t addr_space;
    addr_space = unw_create_addr_space(&accessors, /*byte_order=*/0);
    unw_set_caching_policy(addr_space, UNW_CACHE_GLOBAL);
    return addr_space;

    return core::ptr::null_mut();

    }

    extern int UNW_OBJ(dwarf_search_unwind_table) (unw_addr_space_t as,
    unw_word_t ip,
    unw_dyn_info_t *di,
    unw_proc_info_t *pi,
    int need_unwind_info, void *arg);

    int __libunwind_arch__dwarf_search_unwind_table_loongarch(void *as __maybe_unused,
    uint64_t ip __maybe_unused,
    struct libarch_unwind__dyn_info *_di __maybe_unused,
    void *pi __maybe_unused,
    int need_unwind_info __maybe_unused,
    void *arg __maybe_unused)
    {

    unw_dyn_info_t di = {
    .format     = UNW_INFO_FORMAT_REMOTE_TABLE,
    .start_ip   = _di.start_ip,
    .end_ip     = _di.end_ip,
    .u = {
    .rti = {
    .segbase    = _di.segbase,
    .table_data = _di.table_data,
    .table_len  = _di.table_len,
    },
    },
    };
    let mut ret: c_int = dwarf_search_unwind_table(as, ip, &di, pi, need_unwind_info, arg);
    _di.start_ip = di.start_ip;
    _di.end_ip = di.end_ip;
    _di.segbase = di.u.rti.segbase;
    _di.table_data = di.u.rti.table_data;
    _di.table_len = di.u.rti.table_len;
    return ret;

    return -EINVAL;

    }

    extern int UNW_OBJ(dwarf_find_debug_frame) (int found, unw_dyn_info_t *di_debug,
    unw_word_t ip,
    unw_word_t segbase,
    const char *obj_name, unw_word_t start,
    unw_word_t end);

    int __libunwind_arch__dwarf_find_debug_frame_loongarch(int found __maybe_unused,
    struct libarch_unwind__dyn_info *_di __maybe_unused,
    uint64_t ip __maybe_unused,
    uint64_t segbase __maybe_unused,
    const char *obj_name __maybe_unused,
    uint64_t start __maybe_unused,
    uint64_t end __maybe_unused)
    {

    unw_dyn_info_t di = {
    .format     = UNW_INFO_FORMAT_REMOTE_TABLE,
    .start_ip   = _di.start_ip,
    .end_ip     = _di.end_ip,
    .u = {
    .rti = {
    .segbase    = _di.segbase,
    .table_data = _di.table_data,
    .table_len  = _di.table_len,
    },
    },
    };
    let mut ret: c_int = dwarf_find_debug_frame(found, &di, ip, segbase, obj_name, start, end);
    _di.start_ip = di.start_ip;
    _di.end_ip = di.end_ip;
    _di.segbase = di.u.ti.segbase;
    _di.table_data = di.u.ti.table_data;
    _di.table_len = di.u.ti.table_len;
    return ret;

    return -EINVAL;

    }
    struct unwind_info *__libunwind_arch_unwind_info__new_loongarch(struct thread *thread __maybe_unused,
    struct perf_sample *sample  __maybe_unused,
    int max_stack __maybe_unused,
    bool best_effort  __maybe_unused,
    uint64_t first_ip  __maybe_unused)
    {

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_unwind_info {
    pub ui: unwind_info,
    pub _cursor: unw_cursor_t,
    pub _ips: [u64; ],
}

    struct maps *maps = thread__maps(thread);
    void *addr_space = maps__addr_space(maps);
    struct arch_unwind_info *ui;
    int ret;
    if (addr_space == core::ptr::null_mut())
    return core::ptr::null_mut();
    ui = zalloc(sizeof(*ui) + sizeof(ui._ips[0]) * max_stack);
    if (!ui)
    return core::ptr::null_mut();
    ui.ui.machine = maps__machine(maps);
    ui.ui.thread = thread;
    ui.ui.sample = sample;
    ui.ui.cursor = &ui._cursor;
    ui.ui.ips = &ui._ips[0];
    ui.ui.ips[0] = first_ip;
    ui.ui.cur_ip = 1;
    ui.ui.max_ips = max_stack;
    ui.ui.unw_word_t_size = sizeof(unw_word_t);
    ui.ui.e_machine = EM_LOONGARCH;
    ui.ui.best_effort = best_effort;
    ret = unw_init_remote(&ui._cursor, addr_space, &ui.ui);
    if (ret) {
    if (!best_effort)
    pr_err("libunwind: %s\n", unw_strerror(ret));
    free(ui);
    return core::ptr::null_mut();
    }
    return &ui.ui;

    return core::ptr::null_mut();

    }
#[no_mangle]
pub unsafe extern "C" fn __libunwind_arch__unwind_step_loongarch(__maybe_unused: *mut *mut unwind_info ui) -> c_int {
    int __libunwind_arch__unwind_step_loongarch(struct unwind_info *ui __maybe_unused)
    {

    int ret;
    if (ui.cur_ip >= ui.max_ips)
    return 0;
    ret = unw_step(ui.cursor);
    if (ret > 0) {
    uint64_t ip;
    unw_get_reg(ui.cursor, UNW_REG_IP, &ip);
    if (unw_is_signal_frame(ui.cursor) <= 0) {
//
// Decrement the IP for any non-activation frames. This
// is required to properly find the srcline for caller
// frames.  See also the documentation for
// dwfl_frame_pc(), which this code tries to replicate.
//
    --ip;
    }
    ui.ips[ui.cur_ip++] = ip;
    }
    return ret;

    return -EINVAL;

    }
