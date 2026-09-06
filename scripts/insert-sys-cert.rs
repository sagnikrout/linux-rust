//! Automatically rewritten from C to Rust
//! Source: scripts/insert-sys-cert.c
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


// Write the contents of the <certfile> into kernel symbol system_extra_cert
//
// Copyright (C) IBM Corporation, 2015
//
// Author: Mehmet Kayaalp <mkayaalp@linux.vnet.ibm.com>
//
// This software may be used and distributed according to the terms
// of the GNU General Public License, incorporated herein by reference.
//
// Usage: insert-sys-cert [-s <System.map> -b <vmlinux> -c <certfile>
//
// Macro flag: #define _GNU_SOURCE

#[no_mangle]
unsafe extern "C" fn endianness() -> c_uchar {
    static unsigned char endianness(void)
    {
    let mut two_byte: u16 = 0x00FF;
    let mut low_address: u8 = *((uint8_t *)&two_byte);
    if (low_address == 0)
    return ELFDATA2MSB;
    else
    return ELFDATA2LSB;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sym {
    pub name: *mut c_char,
    pub address: c_ulong,
    pub offset: c_ulong,
    pub content: *mut c_void,
    pub size: c_int,
}

#[no_mangle]
unsafe extern "C" fn get_offset_from_address(hdr: *mut Elf_Ehdr, addr: c_ulong) -> c_ulong {
    static unsigned long get_offset_from_address(Elf_Ehdr *hdr, unsigned long addr)
    {
    Elf_Shdr *x;
    unsigned int i, num_sections;
    x = (void *)hdr + hdr.e_shoff;
    if (hdr.e_shnum == SHN_UNDEF)
    num_sections = x[0].sh_size;
    else
    num_sections = hdr.e_shnum;
    for (i = 1; i < num_sections; i++) {
    let mut start: c_ulong = x[i].sh_addr;
    let mut end: c_ulong = start + x[i].sh_size;
    let mut offset: c_ulong = x[i].sh_offset;
    if (addr >= start && addr <= end)
    return addr - start + offset;
    }
    return 0;
    }
pub const LINE_SIZE: c_int = 100;
    static void get_symbol_from_map(Elf_Ehdr *hdr, FILE *f, char *name,
    struct sym *s)
    {
    char l[LINE_SIZE];
    char *w, *p, *n;
    s.size = 0;
    s.address = 0;
    s.offset = 0;
    if (fseek(f, 0, SEEK_SET) != 0) {
    perror("File seek failed");
    exit(EXIT_FAILURE);
    }
    while (fgets(l, LINE_SIZE, f)) {
    p = strchr(l, '\n');
    if (!p) {
    err("Missing line ending.\n");
    return;
    }
    n = strstr(l, name);
    if (n)
    break;
    }
    if (!n) {
    err("Unable to find symbol: %s\n", name);
    return;
    }
    w = strchr(l, ' ');
    if (!w)
    return;
// w = '\0';
    s.address = strtoul(l, core::ptr::null_mut(), 16);
    if (s.address == 0)
    return;
    s.offset = get_offset_from_address(hdr, s.address);
    s.name = name;
    s.content = (void *)hdr + s.offset;
    }
    static Elf_Sym *find_elf_symbol(Elf_Ehdr *hdr, Elf_Shdr *symtab, char *name)
    {
    Elf_Sym *sym, *symtab_start;
    char *strtab, *symname;
    unsigned int link;
    Elf_Shdr *x;
    int i, n;
    x = (void *)hdr + hdr.e_shoff;
    link = symtab.sh_link;
    symtab_start = (void *)hdr + symtab.sh_offset;
    n = symtab.sh_size / symtab.sh_entsize;
    strtab = (void *)hdr + x[link].sh_offset;
    for (i = 0; i < n; i++) {
    sym = &symtab_start[i];
    symname = strtab + sym.st_name;
    if (strcmp(symname, name) == 0)
    return sym;
    }
    err("Unable to find symbol: %s\n", name);
    return core::ptr::null_mut();
    }
    static void get_symbol_from_table(Elf_Ehdr *hdr, Elf_Shdr *symtab,
    char *name, struct sym *s)
    {
    Elf_Shdr *sec;
    int secndx;
    Elf_Sym *elf_sym;
    Elf_Shdr *x;
    x = (void *)hdr + hdr.e_shoff;
    s.size = 0;
    s.address = 0;
    s.offset = 0;
    elf_sym = find_elf_symbol(hdr, symtab, name);
    if (!elf_sym)
    return;
    secndx = elf_sym.st_shndx;
    if (!secndx)
    return;
    sec = &x[secndx];
    s.size = elf_sym.st_size;
    s.address = elf_sym.st_value;
    s.offset = s.address - sec.sh_addr
    + sec.sh_offset;
    s.name = name;
    s.content = (void *)hdr + s.offset;
    }
    static Elf_Shdr *get_symbol_table(Elf_Ehdr *hdr)
    {
    Elf_Shdr *x;
    unsigned int i, num_sections;
    x = (void *)hdr + hdr.e_shoff;
    if (hdr.e_shnum == SHN_UNDEF)
    num_sections = x[0].sh_size;
    else
    num_sections = hdr.e_shnum;
    for (i = 1; i < num_sections; i++)
    if (x[i].sh_type == SHT_SYMTAB)
    return &x[i];
    return core::ptr::null_mut();
    }
    static void *map_file(char *file_name, int *size)
    {
    struct stat st;
    void *map;
    int fd;
    fd = open(file_name, O_RDWR);
    if (fd < 0) {
    perror(file_name);
    return core::ptr::null_mut();
    }
    if (fstat(fd, &st)) {
    perror("Could not determine file size");
    close(fd);
    return core::ptr::null_mut();
    }
// size = st.st_size;
    map = mmap(core::ptr::null_mut(), *size, PROT_READ|PROT_WRITE, MAP_SHARED, fd, 0);
    if (map == MAP_FAILED) {
    perror("Mapping to memory failed");
    close(fd);
    return core::ptr::null_mut();
    }
    close(fd);
    return map;
    }
    static char *read_file(char *file_name, int *size)
    {
    struct stat st;
    char *buf;
    int fd;
    fd = open(file_name, O_RDONLY);
    if (fd < 0) {
    perror(file_name);
    return core::ptr::null_mut();
    }
    if (fstat(fd, &st)) {
    perror("Could not determine file size");
    close(fd);
    return core::ptr::null_mut();
    }
// size = st.st_size;
    buf = malloc(*size);
    if (!buf) {
    perror("Allocating memory failed");
    close(fd);
    return core::ptr::null_mut();
    }
    if (read(fd, buf, *size) != *size) {
    perror("File read failed");
    close(fd);
    return core::ptr::null_mut();
    }
    close(fd);
    return buf;
    }
#[no_mangle]
unsafe extern "C" fn print_sym(hdr: *mut Elf_Ehdr, s: *mut sym) {
    static void print_sym(Elf_Ehdr *hdr, struct sym *s)
    {
    info("sym:    %s\n", s.name);
    info("addr:   0x%lx\n", s.address);
    info("size:   %d\n", s.size);
    info("offset: 0x%lx\n", (unsigned long)s.offset);
    }
#[no_mangle]
unsafe extern "C" fn print_usage(e: *mut c_char) {
    static void print_usage(char *e)
    {
    printf("Usage %s [-s <System.map>] -b <vmlinux> -c <certfile>\n", e);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    char *system_map_file = core::ptr::null_mut();
    char *vmlinux_file = core::ptr::null_mut();
    char *cert_file = core::ptr::null_mut();
    int vmlinux_size;
    int cert_size;
    Elf_Ehdr *hdr;
    char *cert;
    FILE *system_map;
    unsigned long *lsize;
    int *used;
    int opt;
    Elf_Shdr *symtab = core::ptr::null_mut();
    struct sym cert_sym, lsize_sym, used_sym;
    while ((opt = getopt(argc, argv, "b:c:s:")) != -1) {
    switch (opt) {
    case 's':
    system_map_file = optarg;
    break;
    case 'b':
    vmlinux_file = optarg;
    break;
    case 'c':
    cert_file = optarg;
    break;
    default:
    break;
    }
    }
    if (!vmlinux_file || !cert_file) {
    print_usage(argv[0]);
    exit(EXIT_FAILURE);
    }
    cert = read_file(cert_file, &cert_size);
    if (!cert)
    exit(EXIT_FAILURE);
    hdr = map_file(vmlinux_file, &vmlinux_size);
    if (!hdr)
    exit(EXIT_FAILURE);
    if (vmlinux_size < sizeof(*hdr)) {
    err("Invalid ELF file.\n");
    exit(EXIT_FAILURE);
    }
    if ((hdr.e_ident[EI_MAG0] != ELFMAG0) ||
    (hdr.e_ident[EI_MAG1] != ELFMAG1) ||
    (hdr.e_ident[EI_MAG2] != ELFMAG2) ||
    (hdr.e_ident[EI_MAG3] != ELFMAG3)) {
    err("Invalid ELF magic.\n");
    exit(EXIT_FAILURE);
    }
    if (hdr.e_ident[EI_CLASS] != CURRENT_ELFCLASS) {
    err("ELF class mismatch.\n");
    exit(EXIT_FAILURE);
    }
    if (hdr.e_ident[EI_DATA] != endianness()) {
    err("ELF endian mismatch.\n");
    exit(EXIT_FAILURE);
    }
    if (hdr.e_shoff > vmlinux_size) {
    err("Could not find section header.\n");
    exit(EXIT_FAILURE);
    }
    symtab = get_symbol_table(hdr);
    if (!symtab) {
    warn("Could not find the symbol table.\n");
    if (!system_map_file) {
    err("Please provide a System.map file.\n");
    print_usage(argv[0]);
    exit(EXIT_FAILURE);
    }
    system_map = fopen(system_map_file, "r");
    if (!system_map) {
    perror(system_map_file);
    exit(EXIT_FAILURE);
    }
    get_symbol_from_map(hdr, system_map, CERT_SYM, &cert_sym);
    get_symbol_from_map(hdr, system_map, USED_SYM, &used_sym);
    get_symbol_from_map(hdr, system_map, LSIZE_SYM, &lsize_sym);
    cert_sym.size = used_sym.address - cert_sym.address;
    } else {
    info("Symbol table found.\n");
    if (system_map_file)
    warn("System.map is ignored.\n");
    get_symbol_from_table(hdr, symtab, CERT_SYM, &cert_sym);
    get_symbol_from_table(hdr, symtab, USED_SYM, &used_sym);
    get_symbol_from_table(hdr, symtab, LSIZE_SYM, &lsize_sym);
    }
    if (!cert_sym.offset || !lsize_sym.offset || !used_sym.offset)
    exit(EXIT_FAILURE);
    print_sym(hdr, &cert_sym);
    print_sym(hdr, &used_sym);
    print_sym(hdr, &lsize_sym);
    lsize = (unsigned long *)lsize_sym.content;
    used = (int *)used_sym.content;
    if (cert_sym.size < cert_size) {
    err("Certificate is larger than the reserved area!\n");
    exit(EXIT_FAILURE);
    }
// If the existing cert is the same, don't overwrite
    if (cert_size == *used &&
    strncmp(cert_sym.content, cert, cert_size) == 0) {
    warn("Certificate was already inserted.\n");
    exit(EXIT_SUCCESS);
    }
    if (*used > 0)
    warn("Replacing previously inserted certificate.\n");
    memcpy(cert_sym.content, cert, cert_size);
    if (cert_size < cert_sym.size)
    memset(cert_sym.content + cert_size,
    0, cert_sym.size - cert_size);
// lsize = *lsize + cert_size - *used;
// used = cert_size;
    info("Inserted the contents of %s into %lx.\n", cert_file,
    cert_sym.address);
    info("Used %d bytes out of %d bytes reserved.\n", *used,
    cert_sym.size);
    exit(EXIT_SUCCESS);
    }
