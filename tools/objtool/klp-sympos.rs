//! Automatically rewritten from C to Rust
//! Source: tools/objtool/klp-sympos.c
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
// Compute "sympos", the position used by livepatch to disambiguate
// duplicate symbol names in the patched object.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmlinux_sym {
    pub hash: hlist_node,
    pub name: *const c_char,
    pub addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmlinux_symid {
    pub hash: hlist_node,
    pub id: u64,
    pub addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmlinux_o_symid {
    pub hash: hlist_node,
    pub id: u64,
    pub sym_idx: c_uint,
}

    static DEFINE_HASHTABLE(vmlinux_o_symids, 16);
//
// The original linked kernel, found next to the orig vmlinux.o.  Read with raw
// libelf rather than elf_open_read(): only the symbol table and the resolved
// .klp.symid table are needed, not the (huge) instruction/reloc machinery.
//
// Both tables are built once by read_orig_vmlinux().  The Elf handle stays
// open because the hashed names point into its mmapped string table.
//
    static struct {
    Elf *elf;
    DECLARE_HASHTABLE(syms, 16);	/* name . address */
    DECLARE_HASHTABLE(symids, 16);	/* .klp.symid id . address */
    } vmlinux;
//
// Would the symbol be visible to the runtime's kallsyms-based symbol lookup?
//
#[no_mangle]
unsafe extern "C" fn vmlinux_sym_in_kallsyms(elf: *mut Elf, sym: *mut GElf_Sym) -> bool {
    static bool vmlinux_sym_in_kallsyms(Elf *elf, GElf_Sym *sym)
    {
    let mut type: c_uint = GELF_ST_TYPE(sym.st_info);
    GElf_Shdr shdr;
    Elf_Scn *scn;
    if (sym.st_shndx == SHN_UNDEF || sym.st_shndx >= SHN_LORESERVE)
    return false;
    if (type == STT_SECTION || type == STT_FILE)
    return false;
    scn = elf_getscn(elf, sym.st_shndx);
    if (!scn || !gelf_getshdr(scn, &shdr))
    return false;
    return shdr.sh_flags & SHF_ALLOC;
    }
#[no_mangle]
unsafe extern "C" fn read_orig_vmlinux(filename: *const c_char) -> c_int {
    static int read_orig_vmlinux(const char *filename)
    {
    size_t shstrndx, nr_syms = 0, nr_symids = 0, strtab_idx = 0;
    Elf_Data *symtab_data = core::ptr::null_mut(), *symid_data = core::ptr::null_mut();
    struct klp_symid *symids;
    Elf_Scn *scn = core::ptr::null_mut();
    GElf_Ehdr ehdr;
    int fd;
    fd = open(filename, O_RDONLY);
    if (fd == -1) {
    ERROR_GLIBC("can't open '%s'", filename);
    return -1;
    }
    if (elf_version(EV_CURRENT) == EV_NONE) {
    ERROR_ELF("elf_version");
    return -1;
    }
    vmlinux.elf = elf_begin(fd, ELF_C_READ_MMAP, core::ptr::null_mut());
    if (!vmlinux.elf) {
    ERROR_ELF("elf_begin");
    return -1;
    }
    if (!gelf_getehdr(vmlinux.elf, &ehdr)) {
    ERROR_ELF("gelf_getehdr");
    return -1;
    }
    if (elf_getshdrstrndx(vmlinux.elf, &shstrndx)) {
    ERROR_ELF("elf_getshdrstrndx");
    return -1;
    }
    while ((scn = elf_nextscn(vmlinux.elf, scn))) {
    const char *name;
    GElf_Shdr shdr;
    if (!gelf_getshdr(scn, &shdr)) {
    ERROR_ELF("gelf_getshdr");
    return -1;
    }
    if (shdr.sh_type == SHT_SYMTAB) {
    symtab_data = elf_getdata(scn, core::ptr::null_mut());
    if (!symtab_data) {
    ERROR_ELF("elf_getdata");
    return -1;
    }
    nr_syms = shdr.sh_size / shdr.sh_entsize;
    strtab_idx = shdr.sh_link;
    continue;
    }
    name = elf_strptr(vmlinux.elf, shstrndx, shdr.sh_name);
    if (name && !strcmp(name, KLP_SYMID_SEC)) {
    if (shdr.sh_size % sizeof(struct klp_symid)) {
    ERROR("%s: %s: struct klp_symid size mismatch",
    filename, KLP_SYMID_SEC);
    return -1;
    }
    symid_data = elf_getdata(scn, core::ptr::null_mut());
    if (!symid_data) {
    ERROR_ELF("elf_getdata");
    return -1;
    }
    nr_symids = shdr.sh_size / sizeof(struct klp_symid);
    }
    }
    if (!symtab_data) {
    ERROR("%s: missing symbol table", filename);
    return -1;
    }
    if (!symid_data) {
    ERROR("%s: missing %s section, kernel not built with CONFIG_KLP_BUILD?",
    filename, KLP_SYMID_SEC);
    return -1;
    }
    for (size_t i = 0; i < nr_syms; i++) {
    struct vmlinux_sym *vsym;
    const char *name;
    GElf_Sym s;
    if (!gelf_getsym(symtab_data, i, &s)) {
    ERROR_ELF("gelf_getsym");
    return -1;
    }
    if (!vmlinux_sym_in_kallsyms(vmlinux.elf, &s))
    continue;
    name = elf_strptr(vmlinux.elf, strtab_idx, s.st_name);
    if (!name)
    continue;
    vsym = calloc(1, sizeof(*vsym));
    if (!vsym) {
    ERROR_GLIBC("calloc");
    return -1;
    }
    vsym.name = name;
    vsym.addr = s.st_value;
    hash_add(vmlinux.syms, &vsym.hash, str_hash(name));
    }
    symids = symid_data.d_buf;
    for (size_t i = 0; i < nr_symids; i++) {
    struct vmlinux_symid *vsymid;
    vsymid = calloc(1, sizeof(*vsymid));
    if (!vsymid) {
    ERROR_GLIBC("calloc");
    return -1;
    }
    vsymid.id = __bswap_if_needed(&ehdr, symids[i].id);
    vsymid.addr = __bswap_if_needed(&ehdr, symids[i].addr);
    hash_add(vmlinux.symids, &vsymid.hash, vsymid.id);
    }
// the fd and Elf handle stay open, the hashed names live in the mmap
    return 0;
    }
//
// Read the orig vmlinux.o's .klp.symid table, an array of entries whose 'addr'
// fields have relocs to the symbols they describe.
//
#[no_mangle]
unsafe extern "C" fn read_vmlinux_o_symids(vmlinux_o: *mut elf) -> c_int {
    static int read_vmlinux_o_symids(struct elf *vmlinux_o)
    {
    struct section *sec;
    for_each_sec(vmlinux_o, sec) {
    unsigned long nr;
    if (strcmp(sec.name, KLP_SYMID_SEC))
    continue;
    if (sec_size(sec) % sizeof(struct klp_symid)) {
    ERROR("%s: %s: struct klp_symid size mismatch",
    vmlinux_o.name, KLP_SYMID_SEC);
    return -1;
    }
    nr = sec_size(sec) / sizeof(struct klp_symid);
    for (unsigned long i = 0; i < nr; i++) {
    let mut offset: c_ulong = i * sizeof(struct klp_symid);
    struct vmlinux_o_symid *entry;
    struct klp_symid *symid;
    struct reloc *reloc;
    entry = calloc(1, sizeof(*entry));
    if (!entry) {
    ERROR_GLIBC("calloc");
    return -1;
    }
    symid = sec.data.d_buf + offset;
    entry.id = bswap_if_needed(vmlinux_o, symid.id);
    reloc = find_reloc_by_dest(vmlinux_o, sec,
    offset + offsetof(struct klp_symid, addr));
    if (!reloc) {
    ERROR("%s: missing reloc for %s entry",
    vmlinux_o.name, KLP_SYMID_SEC);
    return -1;
    }
    entry.sym_idx = reloc.sym.idx;
    hash_add(vmlinux_o_symids, &entry.hash, entry.sym_idx);
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn klp_sympos_init(orig: *mut elf) -> c_int {
    int klp_sympos_init(struct elf *orig)
    {
    char *filename;
    int ret;
    if (!str_ends_with(objname, "vmlinux.o"))
    return 0;
    if (read_vmlinux_o_symids(orig))
    return -1;
    filename = strndup(objname, strlen(objname) - 2);
    if (!filename) {
    ERROR_GLIBC("strndup");
    return -1;
    }
    ret = read_orig_vmlinux(filename);
    free(filename);
    return ret;
    }
// Find the symbol's id in the orig vmlinux.o's .klp.symid table
#[no_mangle]
unsafe extern "C" fn find_vmlinux_o_symid(sym: *mut symbol, id: *mut u64) -> c_int {
    static int find_vmlinux_o_symid(struct symbol *sym, u64 *id)
    {
    struct vmlinux_o_symid *entry;
    hash_for_each_possible(vmlinux_o_symids, entry, hash, sym.idx) {
    if (entry.sym_idx == sym.idx) {
// id = entry->id;
    return 0;
    }
    }
    ERROR("no %s entry for symbol %s in orig vmlinux.o", KLP_SYMID_SEC,
    sym.name);
    return -1;
    }
// Find the symbol's final address in the orig vmlinux's .klp.symid table
#[no_mangle]
unsafe extern "C" fn find_vmlinux_symid_addr(id: u64, addr: *mut u64) -> c_int {
    static int find_vmlinux_symid_addr(u64 id, u64 *addr)
    {
    struct vmlinux_symid *symid;
    hash_for_each_possible(vmlinux.symids, symid, hash, id) {
    if (symid.id == id) {
// addr = symid->addr;
    return 0;
    }
    }
    return -1;
    }
//
// Find the sympos of a vmlinux-local symbol by ranking its final address
// among the duplicately named symbols in the linked orig vmlinux, replicating
// the order in which kallsyms_on_each_match_symbol() counts them.
//
#[no_mangle]
unsafe extern "C" fn find_vmlinux_sympos(sym: *mut symbol) -> c_ulong {
    static unsigned long find_vmlinux_sympos(struct symbol *sym)
    {
    let mut nr_matches: c_ulong = 0, sympos = 1;
    let mut key: u32 = str_hash(sym.name);
    struct vmlinux_sym *vsym;
    let mut found: bool = false;
    u64 id, addr;
    hash_for_each_possible(vmlinux.syms, vsym, hash, key)
    if (!strcmp(vsym.name, sym.name))
    nr_matches++;
    if (!nr_matches) {
    ERROR("can't find symbol %s in orig vmlinux", sym.name);
    return ULONG_MAX;
    }
//
// Unique symbols don't need disambiguating.  They also have no
// .klp.symid entry, which is only emitted for names duplicated in
// vmlinux.o, so the lookups below would fail.
//
    if (nr_matches == 1)
    return 0;
    if (find_vmlinux_o_symid(sym, &id))
    return ULONG_MAX;
    if (find_vmlinux_symid_addr(id, &addr)) {
    ERROR("no %s entry for symbol %s in orig vmlinux", KLP_SYMID_SEC,
    sym.name);
    return ULONG_MAX;
    }
    hash_for_each_possible(vmlinux.syms, vsym, hash, key) {
    if (strcmp(vsym.name, sym.name))
    continue;
    if (vsym.addr < addr)
    sympos++;
#[no_mangle]
pub unsafe extern "C" fn if(addr: vsym->addr ==) -> else {
    else if (vsym.addr == addr)
    found = true;
    }
    if (!found) {
    ERROR("%s address mismatch for symbol %s, stale orig vmlinux?",
    KLP_SYMID_SEC, sym.name);
    return ULONG_MAX;
    }
    return sympos;
    }
#[no_mangle]
unsafe extern "C" fn is_init_sym(sym: *mut symbol) -> bool {
    static bool is_init_sym(struct symbol *sym)
    {
    return strstarts(sym.sec.name, ".init");
    }
//
// "sympos" is used by livepatch to disambiguate duplicate symbol names.
//
#[no_mangle]
pub unsafe extern "C" fn klp_find_sympos(elf: *mut elf, sym: *mut symbol) -> c_ulong {
    unsigned long klp_find_sympos(struct elf *elf, struct symbol *sym)
    {
    let mut sympos: c_ulong = 0, nr_matches = 0;
    let mut has_dup: bool = false;
    struct symbol *s;
    if (is_init_sym(sym)) {
    ERROR("%s: can't patch or reference init code/data", sym.name);
    return ULONG_MAX;
    }
    if (sym.bind != STB_LOCAL)
    return 0;
//
// vmlinux: the final link reorders symbols relative to vmlinux.o,
// so the position needs to be derived from the linked orig vmlinux via
// the .klp.symid table.
//
    if (vmlinux.elf)
    return find_vmlinux_sympos(sym);
//
// modules: the final .ko preserves symbol table order, so a
// symtab-order count here matches the runtime count done by
// module_kallsyms_on_each_symbol().
//
    for_each_sym(elf, s) {
    if (!strcmp(s.name, sym.name)) {
    nr_matches++;
    if (s == sym)
    sympos = nr_matches;
    else
    has_dup = true;
    }
    }
    if (!sympos) {
    ERROR("can't find sympos for %s", sym.name);
    return ULONG_MAX;
    }
    return has_dup ? sympos : 0;
    }
