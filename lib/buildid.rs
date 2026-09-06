//! Automatically rewritten from C to Rust
//! Source: lib/buildid.c
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

pub const BUILD_ID: c_int = 3;
pub const MAX_PHDR_CNT: c_int = 256;
    void freader_init_from_file(struct freader *r, void *buf, u32 buf_sz,
    struct file *file, bool may_fault)
    {
    memset(r, 0, sizeof(*r));
    r.buf = buf;
    r.buf_sz = buf_sz;
    r.file = file;
    r.may_fault = may_fault;
    }
#[no_mangle]
pub unsafe extern "C" fn freader_init_from_mem(r: *mut freader, data: *const c_char, data_sz: u64) {
    void freader_init_from_mem(struct freader *r, const char *data, u64 data_sz)
    {
    memset(r, 0, sizeof(*r));
    r.data = data;
    r.data_sz = data_sz;
    }
#[no_mangle]
unsafe extern "C" fn freader_put_folio(r: *mut freader) {
    static void freader_put_folio(struct freader *r)
    {
    if (!r.folio)
    return;
    kunmap_local(r.addr);
    folio_put(r.folio);
    r.folio = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn freader_get_folio(r: *mut freader, file_off: loff_t) -> c_int {
    static int freader_get_folio(struct freader *r, loff_t file_off)
    {
// check if we can just reuse current folio
    if (r.folio && file_off >= r.folio_off &&
    file_off < r.folio_off + folio_size(r.folio))
    return 0;
    freader_put_folio(r);
// only use page cache lookup - fail if not already cached
    r.folio = filemap_get_folio(r.file.f_mapping, file_off >> PAGE_SHIFT);
    if (IS_ERR(r.folio) || !folio_test_uptodate(r.folio)) {
    if (!IS_ERR(r.folio))
    folio_put(r.folio);
    r.folio = core::ptr::null_mut();
    return -EFAULT;
    }
    r.folio_off = folio_pos(r.folio);
    r.addr = kmap_local_folio(r.folio, 0);
    return 0;
    }
    const void *freader_fetch(struct freader *r, loff_t file_off, size_t sz)
    {
    size_t folio_sz;
// provided internal temporary buffer should be sized correctly
    if (WARN_ON(r.buf && sz > r.buf_sz)) {
    r.err = -E2BIG;
    return core::ptr::null_mut();
    }
    if (unlikely(file_off + sz < file_off)) {
    r.err = -EOVERFLOW;
    return core::ptr::null_mut();
    }
// working with memory buffer is much more straightforward
    if (!r.buf) {
    if (file_off + sz > r.data_sz) {
    r.err = -ERANGE;
    return core::ptr::null_mut();
    }
    return r.data + file_off;
    }
// reject secretmem folios created with memfd_secret()
    if (secretmem_mapping(r.file.f_mapping)) {
    r.err = -EFAULT;
    return core::ptr::null_mut();
    }
// use __kernel_read() for sleepable context
    if (r.may_fault) {
    ssize_t ret;
    ret = __kernel_read(r.file, r.buf, sz, &file_off);
    if (ret != sz) {
    r.err = (ret < 0) ? ret : -EIO;
    return core::ptr::null_mut();
    }
    return r.buf;
    }
// fetch or reuse folio for given file offset
    r.err = freader_get_folio(r, file_off);
    if (r.err)
    return core::ptr::null_mut();
// if requested data is crossing folio boundaries, we have to copy
// everything into our local buffer to keep a simple linear memory
// access interface
//
    folio_sz = folio_size(r.folio);
    if (file_off + sz > r.folio_off + folio_sz) {
    let mut part_sz: u64 = r.folio_off + folio_sz - file_off, off;
    memcpy(r.buf, r.addr + file_off - r.folio_off, part_sz);
    off = part_sz;
    while (off < sz) {
// fetch next folio
    r.err = freader_get_folio(r, r.folio_off + folio_sz);
    if (r.err)
    return core::ptr::null_mut();
    folio_sz = folio_size(r.folio);
    part_sz = min_t(u64, sz - off, folio_sz);
    memcpy(r.buf + off, r.addr, part_sz);
    off += part_sz;
    }
    return r.buf;
    }
// if data fits in a single folio, just return direct pointer
    return r.addr + (file_off - r.folio_off);
    }
#[no_mangle]
pub unsafe extern "C" fn freader_cleanup(r: *mut freader) {
    void freader_cleanup(struct freader *r)
    {
    if (!r.buf)
    return; /* non-file-backed mode */
    freader_put_folio(r);
    }
//
// Parse build id from the note segment. This logic can be shared between
// 32-bit and 64-bit system, because Elf32_Nhdr and Elf64_Nhdr are
// identical.
//
    static int parse_build_id(struct freader *r, unsigned char *build_id, __u32 *size,
    loff_t note_off, Elf32_Word note_size)
    {
    const char note_name[] = "GNU";
    let mut note_name_sz: usize = sizeof(note_name);
    u32 build_id_off, new_off, note_end, name_sz, desc_sz;
    const Elf32_Nhdr *nhdr;
    const char *data;
    if (check_add_overflow(note_off, note_size, &note_end))
    return -EINVAL;
    while (note_end - note_off > sizeof(Elf32_Nhdr) + note_name_sz) {
    nhdr = freader_fetch(r, note_off, sizeof(Elf32_Nhdr) + note_name_sz);
    if (!nhdr)
    return r.err;
    name_sz = READ_ONCE(nhdr.n_namesz);
    desc_sz = READ_ONCE(nhdr.n_descsz);
    new_off = note_off + sizeof(Elf32_Nhdr);
    if (check_add_overflow(new_off, ALIGN(name_sz, 4), &new_off) ||
    check_add_overflow(new_off, ALIGN(desc_sz, 4), &new_off) ||
    new_off > note_end)
    break;
    if (nhdr.n_type == BUILD_ID &&
    name_sz == note_name_sz &&
    memcmp(nhdr + 1, note_name, note_name_sz) == 0 &&
    desc_sz > 0 && desc_sz <= BUILD_ID_SIZE_MAX) {
    build_id_off = note_off + sizeof(Elf32_Nhdr) + ALIGN(note_name_sz, 4);
// freader_fetch() will invalidate nhdr pointer
    data = freader_fetch(r, build_id_off, desc_sz);
    if (!data)
    return r.err;
    memcpy(build_id, data, desc_sz);
    memset(build_id + desc_sz, 0, BUILD_ID_SIZE_MAX - desc_sz);
    if (size)
// size = desc_sz;
    return 0;
    }
    note_off = new_off;
    }
    return -EINVAL;
    }
// Parse build ID from 32-bit ELF
#[no_mangle]
unsafe extern "C" fn get_build_id_32(r: *mut freader, build_id: *mut c_uchar, size: *mut __u32) -> c_int {
    static int get_build_id_32(struct freader *r, unsigned char *build_id, __u32 *size)
    {
    const Elf32_Ehdr *ehdr;
    const Elf32_Phdr *phdr;
    __u32 phnum, phoff, i;
    ehdr = freader_fetch(r, 0, sizeof(Elf32_Ehdr));
    if (!ehdr)
    return r.err;
// subsequent freader_fetch() calls invalidate pointers, so remember locally
    phnum = READ_ONCE(ehdr.e_phnum);
    phoff = READ_ONCE(ehdr.e_phoff);
// set upper bound on amount of segments (phdrs) we iterate
    if (phnum > MAX_PHDR_CNT)
    phnum = MAX_PHDR_CNT;
// check that phoff is not large enough to cause an overflow
    if (phoff + phnum * sizeof(Elf32_Phdr) < phoff)
    return -EINVAL;
    for (i = 0; i < phnum; ++i) {
    phdr = freader_fetch(r, phoff + i * sizeof(Elf32_Phdr), sizeof(Elf32_Phdr));
    if (!phdr)
    return r.err;
    if (phdr.p_type == PT_NOTE &&
    !parse_build_id(r, build_id, size, READ_ONCE(phdr.p_offset),
    READ_ONCE(phdr.p_filesz)))
    return 0;
    }
    return -EINVAL;
    }
// Parse build ID from 64-bit ELF
#[no_mangle]
unsafe extern "C" fn get_build_id_64(r: *mut freader, build_id: *mut c_uchar, size: *mut __u32) -> c_int {
    static int get_build_id_64(struct freader *r, unsigned char *build_id, __u32 *size)
    {
    const Elf64_Ehdr *ehdr;
    const Elf64_Phdr *phdr;
    __u32 phnum, i;
    __u64 phoff;
    ehdr = freader_fetch(r, 0, sizeof(Elf64_Ehdr));
    if (!ehdr)
    return r.err;
// subsequent freader_fetch() calls invalidate pointers, so remember locally
    phnum = READ_ONCE(ehdr.e_phnum);
    phoff = READ_ONCE(ehdr.e_phoff);
// set upper bound on amount of segments (phdrs) we iterate
    if (phnum > MAX_PHDR_CNT)
    phnum = MAX_PHDR_CNT;
// check that phoff is not large enough to cause an overflow
    if (phoff + phnum * sizeof(Elf64_Phdr) < phoff)
    return -EINVAL;
    for (i = 0; i < phnum; ++i) {
    phdr = freader_fetch(r, phoff + i * sizeof(Elf64_Phdr), sizeof(Elf64_Phdr));
    if (!phdr)
    return r.err;
    if (phdr.p_type == PT_NOTE &&
    !parse_build_id(r, build_id, size, READ_ONCE(phdr.p_offset),
    READ_ONCE(phdr.p_filesz)))
    return 0;
    }
    return -EINVAL;
    }
// enough for Elf64_Ehdr, Elf64_Phdr, and all the smaller requests
pub const MAX_FREADER_BUF_SZ: c_int = 64;
    static int __build_id_parse(struct file *file, unsigned char *build_id,
    __u32 *size, bool may_fault)
    {
    const Elf32_Ehdr *ehdr;
    struct freader r;
    char buf[MAX_FREADER_BUF_SZ];
    int ret;
    freader_init_from_file(&r, buf, sizeof(buf), file, may_fault);
// fetch first 18 bytes of ELF header for checks
    ehdr = freader_fetch(&r, 0, offsetofend(Elf32_Ehdr, e_type));
    if (!ehdr) {
    ret = r.err;
    goto out;
    }
    ret = -EINVAL;
// compare magic x7f "ELF"
    if (memcmp(ehdr.e_ident, ELFMAG, SELFMAG) != 0)
    goto out;
// only support executable file and shared object file
    if (ehdr.e_type != ET_EXEC && ehdr.e_type != ET_DYN)
    goto out;
    if (ehdr.e_ident[EI_CLASS] == ELFCLASS32)
    ret = get_build_id_32(&r, build_id, size);
#[no_mangle]
pub unsafe extern "C" fn if(ELFCLASS64: ehdr->e_ident[EI_CLASS] ==) -> else {
    else if (ehdr.e_ident[EI_CLASS] == ELFCLASS64)
    ret = get_build_id_64(&r, build_id, size);
    out:
    freader_cleanup(&r);
    return ret;
    }
//
// build_id_parse_nofault() - Parse build ID of ELF file mapped to vma
// @vma:      vma object
// @build_id: buffer to store build id, at least BUILD_ID_SIZE long
// @size:     returns actual build id size in case of success
//
// Assumes no page fault can be taken, so if relevant portions of ELF file are
// not already paged in, fetching of build ID fails.
//
// Return: 0 on success; negative error, otherwise
//
#[no_mangle]
pub unsafe extern "C" fn build_id_parse_nofault(vma: *mut vm_area_struct, build_id: *mut c_uchar, size: *mut __u32) -> c_int {
    int build_id_parse_nofault(struct vm_area_struct *vma, unsigned char *build_id, __u32 *size)
    {
    if (!vma.vm_file)
    return -EINVAL;
    return __build_id_parse(vma.vm_file, build_id, size, false /* !may_fault */);
    }
//
// build_id_parse() - Parse build ID of ELF file mapped to VMA
// @vma:      vma object
// @build_id: buffer to store build id, at least BUILD_ID_SIZE long
// @size:     returns actual build id size in case of success
//
// Assumes faultable context and can cause page faults to bring in file data
// into page cache.
//
// Return: 0 on success; negative error, otherwise
//
#[no_mangle]
pub unsafe extern "C" fn build_id_parse(vma: *mut vm_area_struct, build_id: *mut c_uchar, size: *mut __u32) -> c_int {
    int build_id_parse(struct vm_area_struct *vma, unsigned char *build_id, __u32 *size)
    {
    if (!vma.vm_file)
    return -EINVAL;
    return __build_id_parse(vma.vm_file, build_id, size, true /* may_fault */);
    }
//
// build_id_parse_file() - Parse build ID of ELF file
// @file:      file object
// @build_id: buffer to store build id, at least BUILD_ID_SIZE long
// @size:     returns actual build id size in case of success
//
// Assumes faultable context and can cause page faults to bring in file data
// into page cache.
//
// Return: 0 on success; negative error, otherwise
//
#[no_mangle]
pub unsafe extern "C" fn build_id_parse_file(file: *mut file, build_id: *mut c_uchar, size: *mut __u32) -> c_int {
    int build_id_parse_file(struct file *file, unsigned char *build_id, __u32 *size)
    {
    return __build_id_parse(file, build_id, size, true /* may_fault */);
    }
//
// build_id_parse_buf - Get build ID from a buffer
// @buf:      ELF note section(s) to parse
// @buf_size: Size of @buf in bytes
// @build_id: Build ID parsed from @buf, at least BUILD_ID_SIZE_MAX long
//
// Return: 0 on success, -EINVAL otherwise
//
#[no_mangle]
pub unsafe extern "C" fn build_id_parse_buf(buf: *const c_void, build_id: *mut c_uchar, buf_size: u32) -> c_int {
    int build_id_parse_buf(const void *buf, unsigned char *build_id, u32 buf_size)
    {
    struct freader r;
    int err;
    freader_init_from_mem(&r, buf, buf_size);
    err = parse_build_id(&r, build_id, core::ptr::null_mut(), 0, buf_size);
    freader_cleanup(&r);
    return err;
    }

    unsigned char vmlinux_build_id[BUILD_ID_SIZE_MAX] __ro_after_init;
//
// init_vmlinux_build_id - Compute and stash the running kernel's build ID
//
#[no_mangle]
pub unsafe extern "C" fn init_vmlinux_build_id() -> void __init {
    void __init init_vmlinux_build_id(void)
    {
    extern const void __start_notes;
    extern const void __stop_notes;
    let mut size: c_uint = &__stop_notes - &__start_notes;
    build_id_parse_buf(&__start_notes, vmlinux_build_id, size);
    }
