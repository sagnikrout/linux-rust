//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/unwind-libunwind.c
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

pub const DW_EH_PE_FORMAT_MASK: c_uint = 0x0f	/* format of the encoded value */;
pub const DW_EH_PE_APPL_MASK: c_uint = 0x70	/* how the value is to be applied */;
// Pointer-encoding formats:
pub const DW_EH_PE_omit: c_uint = 0xff;
pub const DW_EH_PE_ptr: c_uint = 0x00	/* pointer-sized unsigned value */;
pub const DW_EH_PE_udata4: c_uint = 0x03	/* unsigned 32-bit value */;
pub const DW_EH_PE_udata8: c_uint = 0x04	/* unsigned 64-bit value */;
pub const DW_EH_PE_sdata4: c_uint = 0x0b	/* signed 32-bit value */;
pub const DW_EH_PE_sdata8: c_uint = 0x0c	/* signed 64-bit value */;
// Pointer-encoding application:
pub const DW_EH_PE_absptr: c_uint = 0x00	/* absolute value */;
pub const DW_EH_PE_pcrel: c_uint = 0x10	/* rel. to addr. of encoded value */;
//
// The following are not documented by LSB v1.3, yet they are used by
// GCC, presumably they aren't documented by LSB since they aren't
// used on Linux:
//
pub const DW_EH_PE_funcrel: c_uint = 0x40	/* start-of-procedure-relative */;
pub const DW_EH_PE_aligned: c_uint = 0x50	/* aligned pointer */;
// Flags intentionally not handled, since they're not needed:
// #define DW_EH_PE_indirect      0x80
// #define DW_EH_PE_uleb128       0x01
// #define DW_EH_PE_udata2        0x02
// #define DW_EH_PE_sleb128       0x09
// #define DW_EH_PE_sdata2        0x0a
// #define DW_EH_PE_textrel       0x20
// #define DW_EH_PE_datarel       0x30
//

    type *__p = (type *) ptr;	\
    type  __v;			\
    if ((__p + 1) > (type *) end)	\
    return -EINVAL;		\
    __v = *__p++;			\
    ptr = (typeof(ptr)) __p;	\
    __v;				\
    })
    static int __dw_read_encoded_value(u8 **p, u8 *end, u64 *val,
    u8 encoding)
    {
    u8 *cur = *p;
// val = 0;
    switch (encoding) {
    case DW_EH_PE_omit:
// val = 0;
    goto out;
    case DW_EH_PE_ptr:
// val = dw_read(cur, unsigned long, end);
    goto out;
    default:
    break;
    }
    switch (encoding & DW_EH_PE_APPL_MASK) {
    case DW_EH_PE_absptr:
    break;
    case DW_EH_PE_pcrel:
// val = (unsigned long) cur;
    break;
    default:
    return -EINVAL;
    }
    if ((encoding & 0x07) == 0x00)
    encoding |= DW_EH_PE_udata4;
    switch (encoding & DW_EH_PE_FORMAT_MASK) {
    case DW_EH_PE_sdata4:
// val += dw_read(cur, s32, end);
    break;
    case DW_EH_PE_udata4:
// val += dw_read(cur, u32, end);
    break;
    case DW_EH_PE_sdata8:
// val += dw_read(cur, s64, end);
    break;
    case DW_EH_PE_udata8:
// val += dw_read(cur, u64, end);
    break;
    default:
    return -EINVAL;
    }
    out:
// p = cur;
    return 0;
    }

    u64 __v;						\
    if (__dw_read_encoded_value(&ptr, end, &__v, enc)) {	\
    return -EINVAL;                                 \
    }                                                       \
    __v;                                                    \
    })
#[no_mangle]
unsafe extern "C" fn elf_base_address(fd: c_int) -> u64 {
    static u64 elf_base_address(int fd)
    {
    Elf *elf = elf_begin(fd, PERF_ELF_C_READ_MMAP, core::ptr::null_mut());
    GElf_Phdr phdr;
    let mut retval: u64 = 0;
    size_t i, phdrnum = 0;
    if (elf == core::ptr::null_mut())
    return 0;
    (void)elf_getphdrnum(elf, &phdrnum);
// PT_LOAD segments are sorted by p_vaddr, so the first has the minimum p_vaddr.
    for (i = 0; i < phdrnum; i++) {
    if (gelf_getphdr(elf, i, &phdr) && phdr.p_type == PT_LOAD) {
    retval = phdr.p_vaddr & -getpagesize();
    break;
    }
    }
    elf_end(elf);
    return retval;
    }
    static int unwind_spec_ehframe(struct dso *dso, struct machine *machine,
    u64 offset, u64 *table_data_offset, u64 *fde_count)
    {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eh_frame_hdr {
    pub version: c_uchar,
    pub eh_frame_ptr_enc: c_uchar,
    pub fde_count_enc: c_uchar,
    pub table_enc: c_uchar,
//
// The rest of the header is variable-length and consists of the
// following members:
//
// encoded_t eh_frame_ptr;
// encoded_t fde_count;
//
// A single encoded pointer should not be more than 8 bytes.
    pub enc: [u64; 2],
//
// struct {
// encoded_t start_ip;
// encoded_t fde_addr;
// } binary_search_table[fde_count];
//
    pub data: [c_char; ],
    pub hdr: } __packed,
    pub &hdr.enc: *mut *mut *mut u8 enc = (u8 ),
    pub &hdr.data: *mut *mut *mut u8 end = (u8 ),
    pub r: isize,
    pub sizeof(hdr)): *mut *mut r = dso__data_read_offset(dso, machine, offset, (u8 ) &hdr,,
    if (r != sizeof(hdr))
    pub -EINVAL: return,
// We dont need eh_frame_ptr, just skip it.
    pub hdr.eh_frame_ptr_enc): dw_read_encoded_value(enc, end,,
// fde_count  = dw_read_encoded_value(enc, end, hdr.fde_count_enc);
// table_data_offset = enc - (u8 *) &hdr;
    pub 0: return,
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct read_unwind_spec_eh_frame_maps_cb_args {
    pub dso: *mut dso,
    pub base_addr: u64,
}

#[no_mangle]
unsafe extern "C" fn read_unwind_spec_eh_frame_maps_cb(map: *mut map, data: *mut c_void) -> c_int {
    static int read_unwind_spec_eh_frame_maps_cb(struct map *map, void *data)
    {
    struct read_unwind_spec_eh_frame_maps_cb_args *args = data;
    if (map__dso(map) == args.dso && map__start(map) - map__pgoff(map) < args.base_addr)
    args.base_addr = map__start(map) - map__pgoff(map);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn elf_section_address_and_offset(fd: c_int, name: *const c_char, address: *mut u64, offset: *mut u64) -> c_int {
    static int elf_section_address_and_offset(int fd, const char *name, u64 *address, u64 *offset)
    {
    Elf *elf;
    GElf_Ehdr ehdr;
    GElf_Shdr shdr;
    let mut ret: c_int = -1;
    elf = elf_begin(fd, PERF_ELF_C_READ_MMAP, core::ptr::null_mut());
    if (elf == core::ptr::null_mut())
    return -1;
    if (gelf_getehdr(elf, &ehdr) == core::ptr::null_mut())
    goto out_err;
    if (!elf_section_by_name(elf, &ehdr, &shdr, name, core::ptr::null_mut()))
    goto out_err;
// address = shdr.sh_addr;
// offset = shdr.sh_offset;
    ret = 0;
    out_err:
    elf_end(elf);
    return ret;
    }
    static int read_unwind_spec_eh_frame(struct dso *dso, struct unwind_info *ui,
    u64 *table_data, u64 *segbase,
    u64 *fde_count)
    {
    struct read_unwind_spec_eh_frame_maps_cb_args args = {
    .dso = dso,
    .base_addr = UINT64_MAX,
    };
    int ret, fd;
    if (dso__data(dso).eh_frame_hdr_offset == 0) {
    if (!dso__data_get_fd(dso, ui.machine, &fd))
    return -EINVAL;
// Check the .eh_frame section for unwinding info
    ret = elf_section_address_and_offset(fd, ".eh_frame_hdr",
    &dso__data(dso).eh_frame_hdr_addr,
    &dso__data(dso).eh_frame_hdr_offset);
    dso__data(dso).elf_base_addr = elf_base_address(fd);
    dso__data_put_fd(dso);
    if (ret || dso__data(dso).eh_frame_hdr_offset == 0)
    return -EINVAL;
    }
    maps__for_each_map(thread__maps(ui.thread), read_unwind_spec_eh_frame_maps_cb, &args);
    args.base_addr -= dso__data(dso).elf_base_addr;
// Address of .eh_frame_hdr
// segbase = args.base_addr + dso__data(dso)->eh_frame_hdr_addr;
    ret = unwind_spec_ehframe(dso, ui.machine, dso__data(dso).eh_frame_hdr_offset,
    table_data, fde_count);
    if (ret)
    return ret;
// binary_search_table offset plus .eh_frame_hdr address
// table_data += *segbase;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn elf_section_offset(fd: c_int, name: *const c_char) -> u64 {
    static u64 elf_section_offset(int fd, const char *name)
    {
    u64 address, offset = 0;
    if (elf_section_address_and_offset(fd, name, &address, &offset))
    return 0;
    return offset;
    }
    static int read_unwind_spec_debug_frame(struct dso *dso,
    struct machine *machine, u64 *offset)
    {
    int fd;
    let mut ofs: u64 = dso__data(dso).debug_frame_offset;
// debug_frame can reside in:
// - dso
// - debug pointed by symsrc_filename
// - gnu_debuglink, which doesn't necessary
// has to be pointed by symsrc_filename
//
    if (ofs == 0) {
    if (dso__data_get_fd(dso, machine, &fd)) {
    ofs = elf_section_offset(fd, ".debug_frame");
    dso__data_put_fd(dso);
    }
    if (ofs <= 0) {
    fd = open(dso__symsrc_filename(dso), O_RDONLY);
    if (fd >= 0) {
    ofs = elf_section_offset(fd, ".debug_frame");
    close(fd);
    }
    }
    if (ofs <= 0) {
    char *debuglink = malloc(PATH_MAX);
    let mut ret: c_int = 0;
    if (debuglink == core::ptr::null_mut()) {
    pr_err("unwind: Can't read unwind spec debug frame.\n");
    return -ENOMEM;
    }
    ret = dso__read_binary_type_filename(
    dso, DSO_BINARY_TYPE__DEBUGLINK,
    machine.root_dir, debuglink, PATH_MAX);
    if (!ret) {
    fd = open(debuglink, O_RDONLY);
    if (fd >= 0) {
    ofs = elf_section_offset(fd,
    ".debug_frame");
    close(fd);
    }
    }
    if (ofs > 0) {
    if (dso__symsrc_filename(dso) != core::ptr::null_mut()) {
    pr_warning(
    "%s: overwrite symsrc(%s,%s)\n",
    __func__,
    dso__symsrc_filename(dso),
    debuglink);
    dso__free_symsrc_filename(dso);
    }
    dso__set_symsrc_filename(dso, debuglink);
    } else {
    free(debuglink);
    }
    }
    dso__data(dso).debug_frame_offset = ofs;
    }
// offset = ofs;
    if (*offset)
    return 0;
    return -EINVAL;
    }
    static struct map *find_map(uint64_t ip, struct unwind_info *ui)
    {
    struct addr_location al;
    struct map *ret;
    addr_location__init(&al);
    thread__find_map(ui.thread, PERF_RECORD_MISC_USER, ip, &al);
    ret = map__get(al.map);
    addr_location__exit(&al);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn elf_is_exec(fd: c_int, name: *const c_char) -> c_int {
    static int elf_is_exec(int fd, const char *name)
    {
    Elf *elf;
    GElf_Ehdr ehdr;
    let mut retval: c_int = 0;
    elf = elf_begin(fd, PERF_ELF_C_READ_MMAP, core::ptr::null_mut());
    if (elf == core::ptr::null_mut())
    return 0;
    if (gelf_getehdr(elf, &ehdr) == core::ptr::null_mut())
    goto out;
    retval = (ehdr.e_type == ET_EXEC);
    out:
    elf_end(elf);
    pr_debug3("unwind: elf_is_exec(%s): %d\n", name, retval);
    return retval;
    }
#[no_mangle]
pub unsafe extern "C" fn __libunwind__find_proc_info(as: *mut c_void, ip: u64, pi: *mut c_void, need_unwind_info: c_int, arg: *mut c_void) -> c_int {
    int __libunwind__find_proc_info(void *as, uint64_t ip, void *pi, int need_unwind_info, void *arg)
    {
    struct unwind_info *ui = arg;
    struct map *map;
    struct dso *dso;
    u64 table_data, segbase, fde_count;
    let mut ret: c_int = -EINVAL;
    map = find_map(ip, ui);
    if (!map)
    return -EINVAL;
    dso = map__dso(map);
    if (!dso) {
    map__put(map);
    return -EINVAL;
    }
    pr_debug3("unwind: find_proc_info dso %s\n", dso__name(dso));
// Check the .eh_frame section for unwinding info
    if (!read_unwind_spec_eh_frame(dso, ui, &table_data, &segbase, &fde_count)) {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct table_entry {
    pub start_ip_offset: u32,
    pub fde_offset: u32,
}

    struct libarch_unwind__dyn_info di = {
    .start_ip = map__start(map),
    .end_ip   = map__end(map),
    .segbase    = segbase,
    .table_data = table_data,
    .table_len  = fde_count * sizeof(struct table_entry) / ui.unw_word_t_size,
    };
    ret = libunwind_arch__dwarf_search_unwind_table(ui.e_machine, as, ip, &di, pi,
    need_unwind_info, arg);
    }
// Check the .debug_frame section for unwinding info
    if (ret < 0 && !read_unwind_spec_debug_frame(dso, ui.machine, &segbase)) {
    int fd;
    let mut start: u64 = map__start(map);
    let mut base: u64 = start;
    const char *symfile;
    let mut di: libarch_unwind__dyn_info = {};
    if (dso__data_get_fd(dso, ui.machine, &fd)) {
    if (elf_is_exec(fd, dso__name(dso)))
    base = 0;
    dso__data_put_fd(dso);
    }
    symfile = dso__symsrc_filename(dso) ?: dso__name(dso);
    if (libunwind_arch__dwarf_find_debug_frame(ui.e_machine, /*found=*/0, &di, ip,
    base, symfile, start, map__end(map))) {
    ret = libunwind_arch__dwarf_search_unwind_table(ui.e_machine, as, ip, &di, pi,
    need_unwind_info, arg);
    }
    }
    map__put(map);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn access_dso_mem(ui: *mut unwind_info, addr: u64, data_word: *mut c_void) -> c_int {
    static int access_dso_mem(struct unwind_info *ui, uint64_t addr, void *data_word)
    {
    struct map *map;
    struct dso *dso;
    ssize_t size;
    map = find_map(addr, ui);
    if (!map) {
    pr_debug("unwind: no map for %lx\n", (unsigned long)addr);
    return -1;
    }
    dso = map__dso(map);
    if (!dso) {
    map__put(map);
    return -1;
    }
    size = dso__data_read_addr(dso, map, ui.machine,
    addr,
    (u8 *) data_word,
    ui.unw_word_t_size);
    map__put(map);
    return !((size_t)size == ui.unw_word_t_size);
    }
    int __libunwind__access_mem(void *as __maybe_unused, uint64_t addr, void *valp_word,
    int __write, void *arg)
    {
    struct unwind_info *ui = arg;
    struct stack_dump *stack = &ui.sample.user_stack;
    u64 start, end;
    int offset;
    int ret;
// Don't support write, probably not needed.
    if (__write || !stack || !ui.sample.user_regs || !ui.sample.user_regs.regs) {
    let mut zero: u64 = 0;
    memcpy(valp_word, &zero, ui.unw_word_t_size);
    return 0;
    }
    ret = perf_reg_value(&start, perf_sample__user_regs(ui.sample),
    perf_arch_reg_sp(ui.e_machine));
    if (ret)
    return ret;
    end = start + stack.size;
// Check overflow.
    if (addr + ui.unw_word_t_size < addr)
    return -EINVAL;
    if (addr < start || addr + ui.unw_word_t_size >= end) {
    ret = access_dso_mem(ui, addr, valp_word);
    if (ret) {
    pr_debug3("unwind: access_mem %p not inside range"
    " 0x%" PRIx64 "-0x%" PRIx64 "\n",
    (void *) (uintptr_t) addr, start, end);
    memset(valp_word, 0, ui.unw_word_t_size);
    return ret;
    }
    return 0;
    }
    offset = addr - start;
    memcpy(valp_word, &stack.data[offset], ui.unw_word_t_size);
    pr_debug3("unwind: access_mem addr %p val %lx, offset %d\n",
    (void *) (uintptr_t) addr, *((unsigned long *)valp_word), offset);
    return 0;
    }
    int __libunwind__access_reg(void *as __maybe_unused, int regnum, void *valp_word, int __write,
    void *arg)
    {
    struct unwind_info *ui = arg;
    int id, ret;
    u64 val;
// Don't support write, I suspect we don't need it.
    if (__write) {
    pr_err("unwind: access_reg w %d\n", regnum);
    return 0;
    }
    if (!ui.sample.user_regs || !ui.sample.user_regs.regs) {
    memset(valp_word, 0, ui.unw_word_t_size);
    return 0;
    }
    id = get_perf_regnum_for_unw_regnum(ui.e_machine, regnum);
    if (id < 0)
    return -EINVAL;
    ret = perf_reg_value(&val, perf_sample__user_regs(ui.sample), id);
    if (ret) {
    if (!ui.best_effort)
    pr_err("unwind: can't read reg %d\n", regnum);
    return ret;
    }
    if (ui.unw_word_t_size == 8)
// (uint64_t *)valp_word = val;
    else
// (uint32_t *)valp_word = (uint32_t)val;
    pr_debug3("unwind: reg %d, val %lx\n", regnum, val);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn unwind__prepare_access(maps: *mut maps, e_machine: u16) -> c_int {
    int unwind__prepare_access(struct maps *maps, uint16_t e_machine)
    {
    void *addr_space;
    if (!dwarf_callchain_users)
    return 0;
    if (maps__addr_space(maps)) {
    pr_debug3("unwind: thread map already set\n");
    return 0;
    }
    if (e_machine == EM_NONE)
    return 0;
    maps__set_e_machine(maps, e_machine);
    addr_space = libunwind_arch__create_addr_space(e_machine);
    maps__set_addr_space(maps, addr_space);
    if (!addr_space) {
    pr_err("unwind: Can't create unwind address space.\n");
    return -ENOMEM;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn unwind__flush_access(maps: *mut maps) {
    void unwind__flush_access(struct maps *maps)
    {
    libunwind_arch__flush_access(maps);
    }
#[no_mangle]
pub unsafe extern "C" fn unwind__finish_access(maps: *mut maps) {
    void unwind__finish_access(struct maps *maps)
    {
    libunwind_arch__finish_access(maps);
    }
#[no_mangle]
unsafe extern "C" fn entry(ip: u64, thread: *mut thread, cb: unwind_entry_cb_t, arg: *mut c_void) -> c_int {
    static int entry(uint64_t ip, struct thread *thread, unwind_entry_cb_t cb, void *arg)
    {
    struct unwind_entry e;
    struct addr_location al;
    int ret;
    addr_location__init(&al);
    e.ms.sym = thread__find_symbol(thread, PERF_RECORD_MISC_USER, ip, &al);
    e.ip     = ip;
    e.ms.map = al.map;
    e.ms.thread = thread__get(al.thread);
    pr_debug("unwind: %s:ip = 0x%" PRIx64 " (0x%" PRIx64 ")\n",
    al.sym ? al.sym.name : "''",
    ip,
    al.map ? map__map_ip(al.map, ip) : (u64) 0);
    ret = cb(&e, arg);
    addr_location__exit(&al);
    return ret;
    }
    int libunwind__get_entries(unwind_entry_cb_t cb, void *arg,
    struct thread *thread,
    struct perf_sample *sample, int max_stack,
    bool best_effort)
    {
    struct unwind_info *ui;
    uint64_t first_ip;
    int ret, i = 0, entries = 0;
    uint16_t e_machine;
    if (!sample.user_regs || !sample.user_regs.regs)
    return 0;
    if (max_stack <= 0)
    return 0;
    if (!thread) {
    pr_warning_once("WARNING: thread is core::ptr::null_mut()");
    return 0;
    }
    e_machine = thread__e_machine(thread, /*machine=*/core::ptr::null_mut(), /*e_flags=*/core::ptr::null_mut());
    ret = perf_reg_value(&first_ip, perf_sample__user_regs(sample),
    perf_arch_reg_ip(e_machine));
    if (ret)
    return 0;
    if (max_stack == 1) {
// Special case for a single entry.
    ret = entry(first_ip, thread, cb, arg);
    return ret ? (ret == -ENOMEM ? -ENOMEM : 0) : 1;
    }
    ui = libunwind_arch_unwind_info__new(thread, sample, max_stack, best_effort, e_machine, first_ip);
    if (!ui)
    return -ENOMEM;
    do {
    ret = libunwind_arch__unwind_step(ui);
    if (ret < 0)
    goto out;
    } while (ret);
//
// Display what we got based on the order setup.
//
    for (i = 0; i < ui.cur_ip; i++) {
    let mut j: c_int = callchain_param.order == ORDER_CALLEE ? i : ui.cur_ip - i - 1;
    if (ui.ips[j]) {
    ret = entry(ui.ips[j], thread, cb, arg);
    if (ret)
    break;
    entries++;
    }
    }
    out:
    libunwind_arch_unwind_info__delete(ui);
//
// Unwinder return contract:
// > 0 : unwinding succeeded (stops fallback).
// 0 : unwinding failed without yielding frames. Ignore non-fatal errors
// (e.g. stepping failure) to allow fallback unwinder or kernel callchains.
// < 0 : fatal error (e.g. -ENOMEM). Aborts unwinding entirely.
//
    if (ret == -ENOMEM)
    return -ENOMEM;
    return (entries > 0 || ret == 0) ? entries : 0;
    }
