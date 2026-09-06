//! Automatically rewritten from C to Rust
//! Source: arch/s390/tools/relocs.c
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

// Macro flag: #define USE_BSD

pub const ELF_BITS: c_int = 64;

    static Elf_Ehdr		ehdr;
    static unsigned long	shnum;
    static unsigned int	shstrndx;
    static unsigned int	shsymtabndx;
    static unsigned int	shxsymtabndx;
    static int sym_index(Elf_Sym *sym);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct relocs {
    pub offset: *mut u32,
    pub count: c_ulong,
    pub size: c_ulong,
}

    static struct relocs relocs64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct section {
    pub shdr: Elf_Shdr,
    pub link: *mut section,
    pub symtab: *mut Elf_Sym,
    pub xsymtab: *mut Elf32_Word,
    pub reltab: *mut Elf_Rel,
    pub strtab: *mut c_char,
}

    static struct section *secs;
    static const char *sec_name(unsigned shndx)
    {
    const char *sec_strtab;
    const char *name = "<noname>";
    sec_strtab = secs[shstrndx].strtab;
    if (shndx < shnum)
    name = sec_strtab + secs[shndx].shdr.sh_name;
#[no_mangle]
pub unsafe extern "C" fn if(SHN_ABS: shndx ==) -> else {
    else if (shndx == SHN_ABS)
    name = "ABSOLUTE";
#[no_mangle]
pub unsafe extern "C" fn if(SHN_COMMON: shndx ==) -> else {
    else if (shndx == SHN_COMMON)
    name = "COMMON";
    return name;
    }
    static const char *sym_name(const char *sym_strtab, Elf_Sym *sym)
    {
    const char *name;
    if (sym.st_name)
    name = sym_strtab + sym.st_name;
    else
    name = sec_name(sym_index(sym));
    return name;
    }

#[no_mangle]
unsafe extern "C" fn elf16_to_cpu(val: u16) -> u16 {
    static uint16_t elf16_to_cpu(uint16_t val)
    {
    if (ehdr.e_ident[EI_DATA] == ELFDATA2LSB)
    return le16_to_cpu(val);
    else
    return be16_to_cpu(val);
    }
#[no_mangle]
unsafe extern "C" fn elf32_to_cpu(val: u32) -> u32 {
    static uint32_t elf32_to_cpu(uint32_t val)
    {
    if (ehdr.e_ident[EI_DATA] == ELFDATA2LSB)
    return le32_to_cpu(val);
    else
    return be32_to_cpu(val);
    }

#[no_mangle]
unsafe extern "C" fn elf64_to_cpu(val: u64) -> u64 {
    static uint64_t elf64_to_cpu(uint64_t val)
    {
    return be64_to_cpu(val);
    }

#[no_mangle]
unsafe extern "C" fn sym_index(sym: *mut Elf_Sym) -> c_int {
    static int sym_index(Elf_Sym *sym)
    {
    Elf_Sym *symtab = secs[shsymtabndx].symtab;
    Elf32_Word *xsymtab = secs[shxsymtabndx].xsymtab;
    unsigned long offset;
    int index;
    if (sym.st_shndx != SHN_XINDEX)
    return sym.st_shndx;
// calculate offset of sym from head of table.
    offset = (unsigned long)sym - (unsigned long)symtab;
    index = offset / sizeof(*sym);
    return elf32_to_cpu(xsymtab[index]);
    }
#[no_mangle]
unsafe extern "C" fn die(fmt: *mut c_char, ...) {
    static void die(char *fmt, ...)
    {
    va_list ap;
    va_start(ap, fmt);
    vfprintf(stderr, fmt, ap);
    va_end(ap);
    exit(1);
    }
#[no_mangle]
unsafe extern "C" fn read_ehdr(fp: *mut FILE) {
    static void read_ehdr(FILE *fp)
    {
    if (fread(&ehdr, sizeof(ehdr), 1, fp) != 1)
    die("Cannot read ELF header: %s\n", strerror(errno));
    if (memcmp(ehdr.e_ident, ELFMAG, SELFMAG) != 0)
    die("No ELF magic\n");
    if (ehdr.e_ident[EI_CLASS] != ELF_CLASS)
    die("Not a %d bit executable\n", ELF_BITS);
    if (ehdr.e_ident[EI_DATA] != ELF_ENDIAN)
    die("ELF endian mismatch\n");
    if (ehdr.e_ident[EI_VERSION] != EV_CURRENT)
    die("Unknown ELF version\n");
// Convert the fields to native endian
    ehdr.e_type	 = elf_half_to_cpu(ehdr.e_type);
    ehdr.e_machine	 = elf_half_to_cpu(ehdr.e_machine);
    ehdr.e_version	 = elf_word_to_cpu(ehdr.e_version);
    ehdr.e_entry	 = elf_addr_to_cpu(ehdr.e_entry);
    ehdr.e_phoff	 = elf_off_to_cpu(ehdr.e_phoff);
    ehdr.e_shoff	 = elf_off_to_cpu(ehdr.e_shoff);
    ehdr.e_flags	 = elf_word_to_cpu(ehdr.e_flags);
    ehdr.e_ehsize	 = elf_half_to_cpu(ehdr.e_ehsize);
    ehdr.e_phentsize = elf_half_to_cpu(ehdr.e_phentsize);
    ehdr.e_phnum	 = elf_half_to_cpu(ehdr.e_phnum);
    ehdr.e_shentsize = elf_half_to_cpu(ehdr.e_shentsize);
    ehdr.e_shnum	 = elf_half_to_cpu(ehdr.e_shnum);
    ehdr.e_shstrndx  = elf_half_to_cpu(ehdr.e_shstrndx);
    shnum = ehdr.e_shnum;
    shstrndx = ehdr.e_shstrndx;
    if ((ehdr.e_type != ET_EXEC) && (ehdr.e_type != ET_DYN))
    die("Unsupported ELF header type\n");
    if (ehdr.e_machine != ELF_MACHINE)
    die("Not for %s\n", ELF_MACHINE_NAME);
    if (ehdr.e_version != EV_CURRENT)
    die("Unknown ELF version\n");
    if (ehdr.e_ehsize != sizeof(Elf_Ehdr))
    die("Bad Elf header size\n");
    if (ehdr.e_phentsize != sizeof(Elf_Phdr))
    die("Bad program header entry\n");
    if (ehdr.e_shentsize != sizeof(Elf_Shdr))
    die("Bad section header entry\n");
    if (shnum == SHN_UNDEF || shstrndx == SHN_XINDEX) {
    Elf_Shdr shdr;
    if (fseek(fp, ehdr.e_shoff, SEEK_SET) < 0)
    die("Seek to %" FMT " failed: %s\n", ehdr.e_shoff, strerror(errno));
    if (fread(&shdr, sizeof(shdr), 1, fp) != 1)
    die("Cannot read initial ELF section header: %s\n", strerror(errno));
    if (shnum == SHN_UNDEF)
    shnum = elf_xword_to_cpu(shdr.sh_size);
    if (shstrndx == SHN_XINDEX)
    shstrndx = elf_word_to_cpu(shdr.sh_link);
    }
    if (shstrndx >= shnum)
    die("String table index out of bounds\n");
    }
#[no_mangle]
unsafe extern "C" fn read_shdrs(fp: *mut FILE) {
    static void read_shdrs(FILE *fp)
    {
    Elf_Shdr shdr;
    int i;
    secs = calloc(shnum, sizeof(struct section));
    if (!secs)
    die("Unable to allocate %ld section headers\n", shnum);
    if (fseek(fp, ehdr.e_shoff, SEEK_SET) < 0)
    die("Seek to %" FMT " failed: %s\n", ehdr.e_shoff, strerror(errno));
    for (i = 0; i < shnum; i++) {
    struct section *sec = &secs[i];
    if (fread(&shdr, sizeof(shdr), 1, fp) != 1) {
    die("Cannot read ELF section headers %d/%ld: %s\n",
    i, shnum, strerror(errno));
    }
    sec.shdr.sh_name      = elf_word_to_cpu(shdr.sh_name);
    sec.shdr.sh_type      = elf_word_to_cpu(shdr.sh_type);
    sec.shdr.sh_flags     = elf_xword_to_cpu(shdr.sh_flags);
    sec.shdr.sh_addr      = elf_addr_to_cpu(shdr.sh_addr);
    sec.shdr.sh_offset    = elf_off_to_cpu(shdr.sh_offset);
    sec.shdr.sh_size      = elf_xword_to_cpu(shdr.sh_size);
    sec.shdr.sh_link      = elf_word_to_cpu(shdr.sh_link);
    sec.shdr.sh_info      = elf_word_to_cpu(shdr.sh_info);
    sec.shdr.sh_addralign = elf_xword_to_cpu(shdr.sh_addralign);
    sec.shdr.sh_entsize   = elf_xword_to_cpu(shdr.sh_entsize);
    if (sec.shdr.sh_link < shnum)
    sec.link = &secs[sec.shdr.sh_link];
    }
    }
#[no_mangle]
unsafe extern "C" fn read_strtabs(fp: *mut FILE) {
    static void read_strtabs(FILE *fp)
    {
    int i;
    for (i = 0; i < shnum; i++) {
    struct section *sec = &secs[i];
    if (sec.shdr.sh_type != SHT_STRTAB)
    continue;
    sec.strtab = malloc(sec.shdr.sh_size);
    if (!sec.strtab)
    die("malloc of %" FMT " bytes for strtab failed\n", sec.shdr.sh_size);
    if (fseek(fp, sec.shdr.sh_offset, SEEK_SET) < 0)
    die("Seek to %" FMT " failed: %s\n", sec.shdr.sh_offset, strerror(errno));
    if (fread(sec.strtab, 1, sec.shdr.sh_size, fp) != sec.shdr.sh_size)
    die("Cannot read symbol table: %s\n", strerror(errno));
    }
    }
#[no_mangle]
unsafe extern "C" fn read_symtabs(fp: *mut FILE) {
    static void read_symtabs(FILE *fp)
    {
    int i, j;
    for (i = 0; i < shnum; i++) {
    struct section *sec = &secs[i];
    int num_syms;
    switch (sec.shdr.sh_type) {
    case SHT_SYMTAB_SHNDX:
    sec.xsymtab = malloc(sec.shdr.sh_size);
    if (!sec.xsymtab)
    die("malloc of %" FMT " bytes for xsymtab failed\n", sec.shdr.sh_size);
    if (fseek(fp, sec.shdr.sh_offset, SEEK_SET) < 0)
    die("Seek to %" FMT " failed: %s\n", sec.shdr.sh_offset, strerror(errno));
    if (fread(sec.xsymtab, 1, sec.shdr.sh_size, fp) != sec.shdr.sh_size)
    die("Cannot read extended symbol table: %s\n", strerror(errno));
    shxsymtabndx = i;
    continue;
    case SHT_SYMTAB:
    num_syms = sec.shdr.sh_size / sizeof(Elf_Sym);
    sec.symtab = malloc(sec.shdr.sh_size);
    if (!sec.symtab)
    die("malloc of %" FMT " bytes for symtab failed\n", sec.shdr.sh_size);
    if (fseek(fp, sec.shdr.sh_offset, SEEK_SET) < 0)
    die("Seek to %" FMT " failed: %s\n", sec.shdr.sh_offset, strerror(errno));
    if (fread(sec.symtab, 1, sec.shdr.sh_size, fp) != sec.shdr.sh_size)
    die("Cannot read symbol table: %s\n", strerror(errno));
    for (j = 0; j < num_syms; j++) {
    Elf_Sym *sym = &sec.symtab[j];
    sym.st_name  = elf_word_to_cpu(sym.st_name);
    sym.st_value = elf_addr_to_cpu(sym.st_value);
    sym.st_size  = elf_xword_to_cpu(sym.st_size);
    sym.st_shndx = elf_half_to_cpu(sym.st_shndx);
    }
    shsymtabndx = i;
    continue;
    default:
    continue;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn read_relocs(fp: *mut FILE) {
    static void read_relocs(FILE *fp)
    {
    int i, j;
    for (i = 0; i < shnum; i++) {
    struct section *sec = &secs[i];
    if (sec.shdr.sh_type != SHT_REL_TYPE)
    continue;
    sec.reltab = malloc(sec.shdr.sh_size);
    if (!sec.reltab)
    die("malloc of %" FMT " bytes for relocs failed\n", sec.shdr.sh_size);
    if (fseek(fp, sec.shdr.sh_offset, SEEK_SET) < 0)
    die("Seek to %" FMT " failed: %s\n", sec.shdr.sh_offset, strerror(errno));
    if (fread(sec.reltab, 1, sec.shdr.sh_size, fp) != sec.shdr.sh_size)
    die("Cannot read symbol table: %s\n", strerror(errno));
    for (j = 0; j < sec.shdr.sh_size / sizeof(Elf_Rel); j++) {
    Elf_Rel *rel = &sec.reltab[j];
    rel.r_offset = elf_addr_to_cpu(rel.r_offset);
    rel.r_info   = elf_xword_to_cpu(rel.r_info);

    rel.r_addend = elf_xword_to_cpu(rel.r_addend);

    }
    }
    }
#[no_mangle]
unsafe extern "C" fn add_reloc(r: *mut relocs, offset: u32) {
    static void add_reloc(struct relocs *r, uint32_t offset)
    {
    if (r.count == r.size) {
    let mut newsize: c_ulong = r.size + 50000;
    void *mem = realloc(r.offset, newsize * sizeof(r.offset[0]));
    if (!mem)
    die("realloc of %ld entries for relocs failed\n", newsize);
    r.offset = mem;
    r.size = newsize;
    }
    r.offset[r.count++] = offset;
    }
    static int do_reloc(struct section *sec, Elf_Rel *rel, ElfW(Sym) *sym,
    const char *symname)
    {
    let mut r_type: c_uint = ELF64_R_TYPE(rel.r_info);
    ElfW(Addr) offset = rel.r_offset;
    switch (r_type) {
    case R_390_NONE:
    case R_390_PC32:
    case R_390_PC64:
    case R_390_PC16DBL:
    case R_390_PC32DBL:
    case R_390_PLT32DBL:
    case R_390_GOTENT:
    case R_390_GOTPCDBL:
    case R_390_GOTOFF64:
    break;
    case R_390_32: {
    static const char kcfipfx[] = "__kcfi_typeid_";
    if (sym.st_shndx != SHN_ABS)
    die("Unsupported relocation type: %d\n", r_type);
//
// Symbols with __kcfi_typeid_ prefix have constant values,
// which do not change if bzImage is loaded at a different
// physical address than the address for which it has been
// compiled.
//
    if (!strncmp(kcfipfx, symname, sizeof(kcfipfx) - 1))
    break;
    die("Invalid absolute R_390_32 relocation: %s\n", symname);
    break;
    }
    case R_390_64:
    add_reloc(&relocs64, offset);
    break;
    default:
    die("Unsupported relocation type: %d\n", r_type);
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn walk_relocs() {
    static void walk_relocs(void)
    {
    int i;
// Walk through the relocations
    for (i = 0; i < shnum; i++) {
    char *sym_strtab;
    Elf_Sym *sh_symtab;
    struct section *sec_applies, *sec_symtab;
    int j;
    struct section *sec = &secs[i];
    if (sec.shdr.sh_type != SHT_REL_TYPE)
    continue;
    sec_symtab  = sec.link;
    sec_applies = &secs[sec.shdr.sh_info];
    if (!(sec_applies.shdr.sh_flags & SHF_ALLOC))
    continue;
    sh_symtab = sec_symtab.symtab;
    sym_strtab = sec_symtab.link.strtab;
    for (j = 0; j < sec.shdr.sh_size / sizeof(Elf_Rel); j++) {
    Elf_Rel *rel = &sec.reltab[j];
    Elf_Sym *sym = &sh_symtab[ELF_R_SYM(rel.r_info)];
    const char *symname = sym_name(sym_strtab, sym);
    do_reloc(sec, rel, sym, symname);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn cmp_relocs(va: *const c_void, vb: *const c_void) -> c_int {
    static int cmp_relocs(const void *va, const void *vb)
    {
    const uint32_t *a, *b;
    a = va; b = vb;
    return (*a == *b) ? 0 : (*a > *b) ? 1 : -1;
    }
#[no_mangle]
unsafe extern "C" fn sort_relocs(r: *mut relocs) {
    static void sort_relocs(struct relocs *r)
    {
    qsort(r.offset, r.count, sizeof(r.offset[0]), cmp_relocs);
    }
#[no_mangle]
unsafe extern "C" fn print_reloc(v: u32) -> c_int {
    static int print_reloc(uint32_t v)
    {
    return fprintf(stdout, "\t.long 0x%08"PRIx32"\n", v) > 0 ? 0 : -1;
    }
#[no_mangle]
unsafe extern "C" fn emit_relocs() {
    static void emit_relocs(void)
    {
    int i;
    walk_relocs();
    sort_relocs(&relocs64);
    printf(".section \".vmlinux.relocs_64\",\"a\"\n");
    for (i = 0; i < relocs64.count; i++)
    print_reloc(relocs64.offset[i]);
    }
#[no_mangle]
unsafe extern "C" fn process(fp: *mut FILE) {
    static void process(FILE *fp)
    {
    read_ehdr(fp);
    read_shdrs(fp);
    read_strtabs(fp);
    read_symtabs(fp);
    read_relocs(fp);
    emit_relocs();
    }
#[no_mangle]
unsafe extern "C" fn usage() {
    static void usage(void)
    {
    die("relocs vmlinux\n");
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    unsigned char e_ident[EI_NIDENT];
    const char *fname;
    FILE *fp;
    fname = core::ptr::null_mut();
    if (argc != 2)
    usage();
    fname = argv[1];
    fp = fopen(fname, "r");
    if (!fp)
    die("Cannot open %s: %s\n", fname, strerror(errno));
    if (fread(&e_ident, 1, EI_NIDENT, fp) != EI_NIDENT)
    die("Cannot read %s: %s", fname, strerror(errno));
    rewind(fp);
    process(fp);
    fclose(fp);
    return 0;
    }
