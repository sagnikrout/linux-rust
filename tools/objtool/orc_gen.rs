//! Automatically rewritten from C to Rust
//! Source: tools/objtool/orc_gen.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2017 Josh Poimboeuf <jpoimboe@redhat.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orc_list_entry {
    pub list: list_head,
    pub orc: orc_entry,
    pub insn_sec: *mut section,
    pub insn_off: c_ulong,
}

    static int orc_list_add(struct list_head *orc_list, struct orc_entry *orc,
    struct section *sec, unsigned long offset)
    {
    struct orc_list_entry *entry = malloc(sizeof(*entry));
    if (!entry) {
    WARN("malloc failed");
    return -1;
    }
    entry.orc	= *orc;
    entry.insn_sec = sec;
    entry.insn_off = offset;
    list_add_tail(&entry.list, orc_list);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn alt_group_len(alt_group: *mut alt_group) -> c_ulong {
    static unsigned long alt_group_len(struct alt_group *alt_group)
    {
    return alt_group.last_insn.offset +
    alt_group.last_insn.len -
    alt_group.first_insn.offset;
    }
#[no_mangle]
pub unsafe extern "C" fn orc_create(file: *mut objtool_file) -> c_int {
    int orc_create(struct objtool_file *file)
    {
    struct section *sec, *orc_sec;
    let mut nr: c_uint = 0, idx = 0;
    struct orc_list_entry *entry;
    struct list_head orc_list;
    let mut null: orc_entry = { .type = ORC_TYPE_UNDEFINED };
// Build a deduplicated list of ORC entries:
    INIT_LIST_HEAD(&orc_list);
    for_each_sec(file.elf, sec) {
    struct orc_entry orc, prev_orc = {0};
    struct instruction *insn;
    let mut empty: bool = true;
    if (!sec.text)
    continue;
    sec_for_each_insn(file, sec, insn) {
    struct alt_group *alt_group = insn.alt_group;
    int i;
    if (!alt_group) {
    if (init_orc_entry(&orc, insn.cfi, insn))
    return -1;
    if (!memcmp(&prev_orc, &orc, sizeof(orc)))
    continue;
    if (orc_list_add(&orc_list, &orc, sec,
    insn.offset))
    return -1;
    nr++;
    prev_orc = orc;
    empty = false;
    continue;
    }
//
// Alternatives can have different stack layout
// possibilities (but they shouldn't conflict).
// Instead of traversing the instructions, use the
// alt_group's flattened byte-offset-addressed CFI
// array.
//
    for (i = 0; i < alt_group_len(alt_group); i++) {
    struct cfi_state *cfi = alt_group.cfi[i];
    if (!cfi)
    continue;
// errors are reported on the original insn
    if (init_orc_entry(&orc, cfi, insn))
    return -1;
    if (!memcmp(&prev_orc, &orc, sizeof(orc)))
    continue;
    if (orc_list_add(&orc_list, &orc, insn.sec,
    insn.offset + i))
    return -1;
    nr++;
    prev_orc = orc;
    empty = false;
    }
// Skip to the end of the alt_group
    insn = alt_group.last_insn;
    }
// Add a section terminator
    if (!empty) {
    orc_list_add(&orc_list, &null, sec, sec.sh.sh_size);
    nr++;
    }
    }
    if (!nr)
    return 0;
// Create .orc_unwind, .orc_unwind_ip and .rela.orc_unwind_ip sections:
    sec = find_section_by_name(file.elf, ".orc_unwind");
    if (sec) {
    WARN("file already has .orc_unwind section, skipping");
    return -1;
    }
    orc_sec = elf_create_section(file.elf, ".orc_unwind",
    nr * sizeof(struct orc_entry),
    sizeof(struct orc_entry),
    SHT_PROGBITS,
    1,
    SHF_ALLOC);
    if (!orc_sec)
    return -1;
    sec = elf_create_section_pair(file.elf, ".orc_unwind_ip", sizeof(int), nr, nr);
    if (!sec)
    return -1;
// Write ORC entries to sections:
    list_for_each_entry(entry, &orc_list, list) {
    if (write_orc_entry(file.elf, orc_sec, sec, idx++,
    entry.insn_sec, entry.insn_off,
    &entry.orc))
    return -1;
    }
    return 0;
    }
