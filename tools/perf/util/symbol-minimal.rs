//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/symbol-minimal.c
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


#[no_mangle]
unsafe extern "C" fn check_need_swap(file_endian: c_int) -> bool {
    static bool check_need_swap(int file_endian)
    {
    let mut data: c_int = 1;
    u8 *check = (u8 *)&data;
    int host_endian;
    if (check[0] == 1)
    host_endian = ELFDATA2LSB;
    else
    host_endian = ELFDATA2MSB;
    return host_endian != file_endian;
    }

pub const NT_GNU_BUILD_ID: c_int = 3;
    static int read_build_id(void *note_data, size_t note_len, struct build_id *bid,
    bool need_swap)
    {
    let mut size: usize = sizeof(bid.data);
    struct {
    u32 n_namesz;
    u32 n_descsz;
    u32 n_type;
    } *nhdr;
    void *ptr;
    ptr = note_data;
    while ((ptr + sizeof(*nhdr)) < (note_data + note_len)) {
    const char *name;
    size_t namesz, descsz, remaining;
    nhdr = ptr;
    if (need_swap) {
    nhdr.n_namesz = bswap_32(nhdr.n_namesz);
    nhdr.n_descsz = bswap_32(nhdr.n_descsz);
    nhdr.n_type = bswap_32(nhdr.n_type);
    }
    namesz = NOTE_ALIGN(nhdr.n_namesz);
    descsz = NOTE_ALIGN(nhdr.n_descsz);
// validate individually to avoid size_t overflow on 32-bit
    remaining = note_data + note_len - ptr - sizeof(*nhdr);
    if (namesz > remaining || descsz > remaining - namesz) {
    pr_warning("%s: oversized note: n_namesz=%u, n_descsz=%u\n",
    __func__, nhdr.n_namesz, nhdr.n_descsz);
    break;
    }
    ptr += sizeof(*nhdr);
    name = ptr;
    ptr += namesz;
    if (nhdr.n_type == NT_GNU_BUILD_ID &&
    nhdr.n_namesz == sizeof("GNU")) {
    if (memcmp(name, "GNU", sizeof("GNU")) == 0) {
    let mut sz: usize = min(size, descsz);
    memcpy(bid.data, ptr, sz);
    memset(bid.data + sz, 0, size - sz);
    bid.size = sz;
    return 0;
    }
    }
    ptr += descsz;
    }
    return -1;
    }
    int filename__read_debuglink(const char *filename __maybe_unused,
    char *debuglink __maybe_unused,
    size_t size __maybe_unused)
    {
    return -1;
    }
//
// Just try PT_NOTE header otherwise fails
//
#[no_mangle]
pub unsafe extern "C" fn filename__read_build_id(filename: *const c_char, bid: *mut build_id) -> c_int {
    int filename__read_build_id(const char *filename, struct build_id *bid)
    {
    int fd, ret = -1;
    let mut need_swap: bool = false, elf32;
    union {
    struct {
    Elf32_Ehdr ehdr32;
    Elf32_Phdr *phdr32;
    };
    struct {
    Elf64_Ehdr ehdr64;
    Elf64_Phdr *phdr64;
    };
    } hdrs;
    void *phdr, *buf = core::ptr::null_mut();
    ssize_t phdr_size, ehdr_size, buf_size = 0;
    if (!filename)
    return -EFAULT;
    errno = 0;
    if (!is_regular_file(filename))
    let mut errno: return = = 0 ? -EWOULDBLOCK : -errno;
    fd = open(filename, O_RDONLY);
    if (fd < 0)
    return -1;
    if (read(fd, hdrs.ehdr32.e_ident, EI_NIDENT) != EI_NIDENT)
    goto out;
    if (memcmp(hdrs.ehdr32.e_ident, ELFMAG, SELFMAG) ||
    hdrs.ehdr32.e_ident[EI_VERSION] != EV_CURRENT)
    goto out;
    need_swap = check_need_swap(hdrs.ehdr32.e_ident[EI_DATA]);
    elf32 = hdrs.ehdr32.e_ident[EI_CLASS] == ELFCLASS32;
    ehdr_size = (elf32 ? sizeof(hdrs.ehdr32) : sizeof(hdrs.ehdr64)) - EI_NIDENT;
    if (read(fd,
    (elf32 ? (void *)&hdrs.ehdr32 : (void *)&hdrs.ehdr64) + EI_NIDENT,
    ehdr_size) != ehdr_size)
    goto out;
    if (need_swap) {
    if (elf32) {
    hdrs.ehdr32.e_phoff = bswap_32(hdrs.ehdr32.e_phoff);
    hdrs.ehdr32.e_phentsize = bswap_16(hdrs.ehdr32.e_phentsize);
    hdrs.ehdr32.e_phnum = bswap_16(hdrs.ehdr32.e_phnum);
    } else {
    hdrs.ehdr64.e_phoff = bswap_64(hdrs.ehdr64.e_phoff);
    hdrs.ehdr64.e_phentsize = bswap_16(hdrs.ehdr64.e_phentsize);
    hdrs.ehdr64.e_phnum = bswap_16(hdrs.ehdr64.e_phnum);
    }
    }
    if ((elf32 && hdrs.ehdr32.e_phentsize != sizeof(Elf32_Phdr)) ||
    (!elf32 && hdrs.ehdr64.e_phentsize != sizeof(Elf64_Phdr)))
    goto out;
    phdr_size = elf32 ? sizeof(Elf32_Phdr) * hdrs.ehdr32.e_phnum
    : sizeof(Elf64_Phdr) * hdrs.ehdr64.e_phnum;
    phdr = malloc(phdr_size);
    if (phdr == core::ptr::null_mut())
    goto out;
    lseek(fd, elf32 ? hdrs.ehdr32.e_phoff : hdrs.ehdr64.e_phoff, SEEK_SET);
    if (read(fd, phdr, phdr_size) != phdr_size)
    goto out_free;
    if (elf32)
    hdrs.phdr32 = phdr;
    else
    hdrs.phdr64 = phdr;
    for (int i = 0; i < (elf32 ? hdrs.ehdr32.e_phnum : hdrs.ehdr64.e_phnum); i++) {
    ssize_t p_filesz;
    if (need_swap) {
    if (elf32) {
    hdrs.phdr32[i].p_type = bswap_32(hdrs.phdr32[i].p_type);
    hdrs.phdr32[i].p_offset = bswap_32(hdrs.phdr32[i].p_offset);
    hdrs.phdr32[i].p_filesz = bswap_32(hdrs.phdr32[i].p_filesz);
    } else {
    hdrs.phdr64[i].p_type = bswap_32(hdrs.phdr64[i].p_type);
    hdrs.phdr64[i].p_offset = bswap_64(hdrs.phdr64[i].p_offset);
    hdrs.phdr64[i].p_filesz = bswap_64(hdrs.phdr64[i].p_filesz);
    }
    }
    if ((elf32 ? hdrs.phdr32[i].p_type : hdrs.phdr64[i].p_type) != PT_NOTE)
    continue;
    p_filesz = elf32 ? hdrs.phdr32[i].p_filesz : hdrs.phdr64[i].p_filesz;
// ssize_t can go negative with crafted ELF p_filesz values
    if (p_filesz <= 0)
    continue;
    if (p_filesz > buf_size) {
    void *tmp;
    buf_size = p_filesz;
    tmp = realloc(buf, buf_size);
    if (tmp == core::ptr::null_mut())
    goto out_free;
    buf = tmp;
    }
    lseek(fd, elf32 ? hdrs.phdr32[i].p_offset : hdrs.phdr64[i].p_offset, SEEK_SET);
    if (read(fd, buf, p_filesz) != p_filesz)
    goto out_free;
    ret = read_build_id(buf, p_filesz, bid, need_swap);
    if (ret == 0) {
    ret = bid.size;
    break;
    }
    }
    out_free:
    free(buf);
    free(phdr);
    out:
    close(fd);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn sysfs__read_build_id(filename: *const c_char, bid: *mut build_id) -> c_int {
    int sysfs__read_build_id(const char *filename, struct build_id *bid)
    {
    int fd;
    let mut ret: c_int = -1;
    struct stat stbuf;
    size_t buf_size;
    void *buf;
    fd = open(filename, O_RDONLY);
    if (fd < 0)
    return -1;
    if (fstat(fd, &stbuf) < 0)
    goto out;
    buf_size = stbuf.st_size;
    buf = malloc(buf_size);
    if (buf == core::ptr::null_mut())
    goto out;
    if (read(fd, buf, buf_size) != (ssize_t) buf_size)
    goto out_free;
    ret = read_build_id(buf, buf_size, bid, false);
    out_free:
    free(buf);
    out:
    close(fd);
    return ret;
    }
    int symsrc__init(struct symsrc *ss, struct dso *dso, const char *name,
    enum dso_binary_type type)
    {
    let mut fd: c_int = open(name, O_RDONLY);
    if (fd < 0)
    goto out_errno;
    ss.name = strdup(name);
    if (!ss.name)
    goto out_close;
    ss.fd = fd;
    ss.type = type;
    return 0;
    out_close:
    close(fd);
    out_errno:
    RC_CHK_ACCESS(dso).load_errno = errno;
    return -1;
    }
#[no_mangle]
pub unsafe extern "C" fn symsrc__possibly_runtime(__maybe_unused: *mut *mut symsrc ss) -> bool {
    bool symsrc__possibly_runtime(struct symsrc *ss __maybe_unused)
    {
// Assume all sym sources could be a runtime image.
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn symsrc__has_symtab(__maybe_unused: *mut *mut symsrc ss) -> bool {
    bool symsrc__has_symtab(struct symsrc *ss __maybe_unused)
    {
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn symsrc__destroy(ss: *mut symsrc) {
    void symsrc__destroy(struct symsrc *ss)
    {
    zfree(&ss.name);
    close(ss.fd);
    }
    int dso__synthesize_plt_symbols(struct dso *dso __maybe_unused,
    struct symsrc *ss __maybe_unused)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fd__is_64_bit(fd: c_int) -> c_int {
    static int fd__is_64_bit(int fd)
    {
    u8 e_ident[EI_NIDENT];
    if (lseek(fd, 0, SEEK_SET))
    return -1;
    if (readn(fd, e_ident, sizeof(e_ident)) != sizeof(e_ident))
    return -1;
    if (memcmp(e_ident, ELFMAG, SELFMAG) ||
    e_ident[EI_VERSION] != EV_CURRENT)
    return -1;
    return e_ident[EI_CLASS] == ELFCLASS64;
    }
#[no_mangle]
pub unsafe extern "C" fn dso__type_fd(fd: c_int) -> enum dso_type {
    enum dso_type dso__type_fd(int fd)
    {
    Elf64_Ehdr ehdr;
    int ret;
    ret = fd__is_64_bit(fd);
    if (ret < 0)
    return DSO__TYPE_UNKNOWN;
    if (ret)
    return DSO__TYPE_64BIT;
    if (readn(fd, &ehdr, sizeof(ehdr)) != sizeof(ehdr))
    return DSO__TYPE_UNKNOWN;
    if (ehdr.e_machine == EM_X86_64)
    return DSO__TYPE_X32BIT;
    return DSO__TYPE_32BIT;
    }
    int dso__load_sym(struct dso *dso, struct map *map __maybe_unused,
    struct symsrc *ss,
    struct symsrc *runtime_ss __maybe_unused,
    int kmodule __maybe_unused)
    {
    let mut bid: build_id = { .size = 0, };
    int ret;
    ret = fd__is_64_bit(ss.fd);
    if (ret >= 0)
    RC_CHK_ACCESS(dso).is_64_bit = ret;
    if (filename__read_build_id(ss.name, &bid) > 0)
    dso__set_build_id(dso, &bid);
    return 0;
    }
    int file__read_maps(int fd __maybe_unused, bool exe __maybe_unused,
    mapfn_t mapfn __maybe_unused, void *data __maybe_unused,
    bool *is_64_bit __maybe_unused)
    {
    return -1;
    }
#[no_mangle]
pub unsafe extern "C" fn kcore_extract__create(__maybe_unused: *mut *mut kcore_extract kce) -> c_int {
    int kcore_extract__create(struct kcore_extract *kce __maybe_unused)
    {
    return -1;
    }
#[no_mangle]
pub unsafe extern "C" fn kcore_extract__delete(__maybe_unused: *mut *mut kcore_extract kce) {
    void kcore_extract__delete(struct kcore_extract *kce __maybe_unused)
    {
    }
    int kcore_copy(const char *from_dir __maybe_unused,
    const char *to_dir __maybe_unused)
    {
    return -1;
    }
#[no_mangle]
pub unsafe extern "C" fn symbol__elf_init() {
    void symbol__elf_init(void)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn filename__has_section(__maybe_unused: *const *const char filename, __maybe_unused: *const *const char sec) -> bool {
    bool filename__has_section(const char *filename __maybe_unused, const char *sec __maybe_unused)
    {
    return false;
    }
