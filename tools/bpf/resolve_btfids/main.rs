//! Automatically rewritten from C to Rust
//! Source: tools/bpf/resolve_btfids/main.c
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


// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
//
// resolve_btfids scans ELF object for .BTF_ids section and resolves
// its symbols with BTF ID values.
//
// Each symbol points to 4 bytes data and is expected to have
// following name syntax:
//
// __BTF_ID__<type>__<symbol>[__<id>]
//
// type is:
//
// func    - lookup BTF_KIND_FUNC symbol with <symbol> name
// and store its ID into the data:
//
// __BTF_ID__func__vfs_close__1:
// .zero 4
//
// struct  - lookup BTF_KIND_STRUCT symbol with <symbol> name
// and store its ID into the data:
//
// __BTF_ID__struct__sk_buff__1:
// .zero 4
//
// union   - lookup BTF_KIND_UNION symbol with <symbol> name
// and store its ID into the data:
//
// __BTF_ID__union__thread_union__1:
// .zero 4
//
// typedef - lookup BTF_KIND_TYPEDEF symbol with <symbol> name
// and store its ID into the data:
//
// __BTF_ID__typedef__pid_t__1:
// .zero 4
//
// set     - store symbol size into first 4 bytes and sort following
// ID list
//
// __BTF_ID__set__list:
// .zero 4
// list:
// __BTF_ID__func__vfs_getattr__3:
// .zero 4
// __BTF_ID__func__vfs_fallocate__4:
// .zero 4
//
// set8    - store symbol size into first 4 bytes and sort following
// ID list
//
// __BTF_ID__set8__list:
// .zero 8
// list:
// __BTF_ID__func__vfs_getattr__3:
// .zero 4
// .word (1 << 0) | (1 << 2)
// __BTF_ID__func__vfs_fallocate__5:
// .zero 4
// .word (1 << 3) | (1 << 1) | (1 << 2)
//
// In addition to resolving BTF IDs, resolve_btfids performs kernel-specific
// BTF-to-BTF transformations for kfuncs found in BTF_SET8_KFUNCS sets. For
// each such kfunc it:
//
// - emits a "bpf_kfunc" decl tag, and "bpf_fastcall" when KF_FASTCALL is set;
// - wraps the return value and/or arguments that use arena pointers
// with the "address_space(1)" type attribute;
// - rewrites the prototype of KF_IMPLICIT_ARGS kfuncs.
//
// These kfunc annotations were historically produced by pahole.
//
// Macro flag: #define  _GNU_SOURCE

pub const ADDR_CNT: c_int = 100;

    enum btf_id_kind {
    BTF_ID_KIND_NONE,
    BTF_ID_KIND_SYM,
    BTF_ID_KIND_SET,
    BTF_ID_KIND_SET8
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_id {
    pub rb_node: rb_node,
    pub name: *mut c_char,
    union {
    pub id: c_int,
    pub cnt: c_int,
}

    enum btf_id_kind kind;
    int		 addr_cnt;
    Elf64_Addr	 addr[ADDR_CNT];
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct addr_sym {
    pub addr: Elf64_Addr,
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct object {
    pub path: *const c_char,
    pub btf_path: *const c_char,
    pub base_btf_path: *const c_char,
    pub btf: *mut btf,
    pub base_btf: *mut btf,
    pub distill_base: bool,
    struct {
    pub fd: c_int,
    pub elf: *mut Elf,
    pub symbols: *mut Elf_Data,
    pub idlist: *mut Elf_Data,
    pub symbols_shndx: c_int,
    pub idlist_shndx: c_int,
    pub strtabidx: usize,
    pub idlist_addr: c_ulong,
    pub encoding: c_int,
    pub efile: },
    pub sets: rb_root,
    pub structs: rb_root,
    pub unions: rb_root,
    pub typedefs: rb_root,
    pub funcs: rb_root,
    pub nr_funcs: c_int,
    pub nr_structs: c_int,
    pub nr_unions: c_int,
    pub nr_typedefs: c_int,
    pub addr_syms: *mut addr_sym,
    pub addr_syms_cnt: u32,
    pub addr_syms_cap: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfunc {
    pub rb_node: rb_node,
    pub name: *const c_char,
    pub btf_id: u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf2btf_context {
    pub btf: *mut btf,
    pub decl_tags: *mut u32,
    pub nr_decl_tags: u32,
    pub max_decl_tags: u32,
    pub kfuncs: rb_root,
}

    static int verbose;
    static int warnings;
#[no_mangle]
unsafe extern "C" fn eprintf(level: c_int, var: c_int, fmt: *const c_char, ...) -> c_int {
    static int eprintf(int level, int var, const char *fmt, ...)
    {
    va_list args;
    let mut ret: c_int = 0;
    if (var >= level) {
    va_start(args, fmt);
    ret = vfprintf(stderr, fmt, args);
    va_end(args);
    }
    return ret;
    }

    eprintf(1, verbose, pr_fmt(fmt), ##__VA_ARGS__)

    eprintf(n, verbose, pr_fmt(fmt), ##__VA_ARGS__)

    eprintf(0, verbose, pr_fmt(fmt), ##__VA_ARGS__)

    eprintf(0, verbose, pr_fmt(fmt), ##__VA_ARGS__)
//
// Grow *data so it can hold at least cnt elements of elem_sz bytes each.
// *cap is the capacity in elements and is updated on growth.
//
#[no_mangle]
unsafe extern "C" fn __ensure_mem(data: *mut c_void, cap: *mut u32, cnt: u32, elem_sz: usize) -> c_int {
    static int __ensure_mem(void **data, u32 *cap, u32 cnt, size_t elem_sz)
    {
    u32 new_cap, old_cap = *cap;
    void *arr;
    if (cnt <= old_cap)
    return 0;
    new_cap = max(old_cap + 256, old_cap * 2);
    if (new_cap < cnt)
    new_cap = cnt;
    arr = realloc(*data, elem_sz * new_cap);
    if (!arr)
    return -ENOMEM;
// data = arr;
// cap = new_cap;
    return 0;
    }

    __ensure_mem((void **)(arr_ptr), (cap_ptr), (cnt), sizeof(**(arr_ptr)))
#[no_mangle]
unsafe extern "C" fn is_btf_id(name: *const c_char) -> bool {
    static bool is_btf_id(const char *name)
    {
    return name && !strncmp(name, BTF_ID_PREFIX, sizeof(BTF_ID_PREFIX) - 1);
    }
    static struct btf_id *btf_id__find(struct rb_root *root, const char *name)
    {
    struct rb_node *p = root.rb_node;
    struct btf_id *id;
    int cmp;
    while (p) {
    id = rb_entry(p, struct btf_id, rb_node);
    cmp = strcmp(id.name, name);
    if (cmp < 0)
    p = p.rb_left;
#[no_mangle]
pub unsafe extern "C" fn if(0: cmp >) -> else {
    else if (cmp > 0)
    p = p.rb_right;
    else
    return id;
    }
    return core::ptr::null_mut();
    }
    static struct btf_id *__btf_id__add(struct rb_root *root,
    const char *name,
    enum btf_id_kind kind,
    bool unique)
    {
    struct rb_node **p = &root.rb_node;
    struct rb_node *parent = core::ptr::null_mut();
    struct btf_id *id;
    int cmp;
    while (*p != core::ptr::null_mut()) {
    parent = *p;
    id = rb_entry(parent, struct btf_id, rb_node);
    cmp = strcmp(id.name, name);
    if (cmp < 0)
    p = &(*p).rb_left;
#[no_mangle]
pub unsafe extern "C" fn if(0: cmp >) -> else {
    else if (cmp > 0)
    p = &(*p).rb_right;
    else
    return unique ? core::ptr::null_mut() : id;
    }
    id = zalloc(sizeof(*id));
    if (id) {
    pr_debug("adding symbol %s\n", name);
    id.name = strdup(name);
    if (!id.name) {
    free(id);
    return core::ptr::null_mut();
    }
    id.kind = kind;
    rb_link_node(&id.rb_node, parent, p);
    rb_insert_color(&id.rb_node, root);
    }
    return id;
    }
    static inline struct btf_id *btf_id__add(struct rb_root *root,
    const char *name,
    enum btf_id_kind kind)
    {
    return __btf_id__add(root, name, kind, false);
    }
    static inline struct btf_id *btf_id__add_unique(struct rb_root *root,
    const char *name,
    enum btf_id_kind kind)
    {
    return __btf_id__add(root, name, kind, true);
    }
#[no_mangle]
unsafe extern "C" fn get_id(prefix_end: *const c_char, buf: *mut c_char, buf_sz: usize) -> c_int {
    static int get_id(const char *prefix_end, char *buf, size_t buf_sz)
    {
//
// __BTF_ID__func__vfs_truncate__0
// prefix_end =  ^
// pos        =    ^
//
    let mut len: c_int = strlen(prefix_end);
    let mut pos: c_int = sizeof("__") - 1;
    char *p;
    if (pos >= len)
    return -1;
    if (len - pos >= buf_sz)
    return -1;
    strcpy(buf, prefix_end + pos);
//
// __BTF_ID__func__vfs_truncate__0
// buf =           ^
//
// cut the unique id part
//
    p = strrchr(buf, '_');
    p--;
    if (*p != '_')
    return -1;
// p = '\0';
    return 0;
    }
    static struct btf_id *add_set(struct object *obj, char *name, enum btf_id_kind kind)
    {
    let mut len: c_int = strlen(name);
    int prefixlen;
    char *id;
//
// __BTF_ID__set__name
// name =    ^
// id   =         ^
//
    switch (kind) {
    case BTF_ID_KIND_SET:
    prefixlen = sizeof(BTF_SET "__") - 1;
    break;
    case BTF_ID_KIND_SET8:
    prefixlen = sizeof(BTF_SET8 "__") - 1;
    break;
    default:
    pr_err("Unexpected kind %d passed to %s() for symbol %s\n", kind, __func__, name);
    return core::ptr::null_mut();
    }
    id = name + prefixlen;
    if (id >= name + len) {
    pr_err("FAILED to parse set name: %s\n", name);
    return core::ptr::null_mut();
    }
    return btf_id__add_unique(&obj.sets, id, kind);
    }
    static struct btf_id *add_symbol(struct rb_root *root, char *name, size_t size)
    {
    char id[KSYM_NAME_LEN];
    if (get_id(name + size, id, sizeof(id))) {
    pr_err("FAILED to parse symbol name: %s\n", name);
    return core::ptr::null_mut();
    }
    return btf_id__add(root, id, BTF_ID_KIND_SYM);
    }
#[no_mangle]
unsafe extern "C" fn btf_id__free_all(root: *mut rb_root) {
    static void btf_id__free_all(struct rb_root *root)
    {
    struct rb_node *next;
    struct btf_id *id;
    next = rb_first(root);
    while (next) {
    id = rb_entry(next, struct btf_id, rb_node);
    next = rb_next(&id.rb_node);
    rb_erase(&id.rb_node, root);
    free(id.name);
    free(id);
    }
    }
#[no_mangle]
unsafe extern "C" fn bswap_32_data(data: *mut c_void, nr_bytes: u32) {
    static void bswap_32_data(void *data, u32 nr_bytes)
    {
    u32 cnt, i;
    u32 *ptr;
    cnt = nr_bytes / sizeof(u32);
    ptr = data;
    for (i = 0; i < cnt; i++)
    ptr[i] = bswap_32(ptr[i]);
    }
#[no_mangle]
unsafe extern "C" fn elf_collect(obj: *mut object) -> c_int {
    static int elf_collect(struct object *obj)
    {
    Elf_Scn *scn = core::ptr::null_mut();
    size_t shdrstrndx;
    GElf_Ehdr ehdr;
    let mut idx: c_int = 0;
    Elf *elf;
    int fd;
    fd = open(obj.path, O_RDWR, 0666);
    if (fd == -1) {
    pr_err("FAILED cannot open %s: %s\n",
    obj.path, strerror(errno));
    return -1;
    }
    elf_version(EV_CURRENT);
    elf = elf_begin(fd, ELF_C_READ_MMAP_PRIVATE, core::ptr::null_mut());
    if (!elf) {
    close(fd);
    pr_err("FAILED cannot create ELF descriptor: %s\n",
    elf_errmsg(-1));
    return -1;
    }
    obj.efile.fd  = fd;
    obj.efile.elf = elf;
    elf_flagelf(elf, ELF_C_SET, ELF_F_LAYOUT);
    if (elf_getshdrstrndx(elf, &shdrstrndx) != 0) {
    pr_err("FAILED cannot get shdr str ndx\n");
    return -1;
    }
    if (gelf_getehdr(obj.efile.elf, &ehdr) == core::ptr::null_mut()) {
    pr_err("FAILED cannot get ELF header: %s\n",
    elf_errmsg(-1));
    return -1;
    }
    obj.efile.encoding = ehdr.e_ident[EI_DATA];
//
// Scan all the elf sections and look for save data
// from .BTF_ids section and symbols.
//
    while ((scn = elf_nextscn(elf, scn)) != core::ptr::null_mut()) {
    Elf_Data *data;
    GElf_Shdr sh;
    char *name;
    idx++;
    if (gelf_getshdr(scn, &sh) != &sh) {
    pr_err("FAILED get section(%d) header\n", idx);
    return -1;
    }
    name = elf_strptr(elf, shdrstrndx, sh.sh_name);
    if (!name) {
    pr_err("FAILED get section(%d) name\n", idx);
    return -1;
    }
    data = elf_getdata(scn, 0);
    if (!data) {
    pr_err("FAILED to get section(%d) data from %s\n",
    idx, name);
    return -1;
    }
    pr_debug2("section(%d) %s, size %ld, link %d, flags %lx, type=%d\n",
    idx, name, (unsigned long) data.d_size,
    (int) sh.sh_link, (unsigned long) sh.sh_flags,
    (int) sh.sh_type);
    if (sh.sh_type == SHT_SYMTAB) {
    obj.efile.symbols       = data;
    obj.efile.symbols_shndx = idx;
    obj.efile.strtabidx     = sh.sh_link;
    } else if (!strcmp(name, BTF_IDS_SECTION)) {
//
// If target endianness differs from host, we need to bswap32
// the .BTF_ids section data on load, because .BTF_ids has
// Elf_Type = ELF_T_BYTE, and so libelf returns data buffer in
// the target endianness. We repeat this on dump.
//
    if (obj.efile.encoding != ELFDATANATIVE) {
    pr_debug("bswap_32 .BTF_ids data from target to host endianness\n");
    bswap_32_data(data.d_buf, data.d_size);
    }
    obj.efile.idlist       = data;
    obj.efile.idlist_shndx = idx;
    obj.efile.idlist_addr  = sh.sh_addr;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn push_addr_sym(obj: *mut object, addr: Elf64_Addr, name: *const c_char) -> c_int {
    static int push_addr_sym(struct object *obj, Elf64_Addr addr, const char *name)
    {
    if (ensure_mem(&obj.addr_syms, &obj.addr_syms_cap, obj.addr_syms_cnt + 1))
    return -ENOMEM;
    obj.addr_syms[obj.addr_syms_cnt++] = (struct addr_sym){
    .addr = addr,
    .name = name,
    };
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cmp_addr_sym(a: *const c_void, b: *const c_void) -> c_int {
    static int cmp_addr_sym(const void *a, const void *b)
    {
    let mut aa: Elf64_Addr = ((const struct addr_sym *)a).addr;
    let mut ab: Elf64_Addr = ((const struct addr_sym *)b).addr;
    return (aa > ab) - (aa < ab);
    }
    static const char *find_name_by_addr(struct object *obj, Elf64_Addr addr)
    {
    let mut key: addr_sym = { .addr = addr };
    struct addr_sym *res;
    if (!obj.addr_syms_cnt)
    return core::ptr::null_mut();
    res = bsearch(&key, obj.addr_syms, obj.addr_syms_cnt,
    sizeof(*obj.addr_syms), cmp_addr_sym);
    return res ? res.name : core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn symbols_collect(obj: *mut object) -> c_int {
    static int symbols_collect(struct object *obj)
    {
    Elf_Scn *scn = core::ptr::null_mut();
    int n, i;
    GElf_Shdr sh;
    char *name;
    scn = elf_getscn(obj.efile.elf, obj.efile.symbols_shndx);
    if (!scn)
    return -1;
    if (gelf_getshdr(scn, &sh) != &sh)
    return -1;
    n = sh.sh_size / sh.sh_entsize;
//
// Scan symbols and look for the ones starting with
// __BTF_ID__* over .BTF_ids section.
//
    for (i = 0; i < n; i++) {
    char *prefix;
    struct btf_id *id;
    GElf_Sym sym;
    if (!gelf_getsym(obj.efile.symbols, i, &sym))
    return -1;
    if (sym.st_shndx != obj.efile.idlist_shndx)
    continue;
    name = elf_strptr(obj.efile.elf, obj.efile.strtabidx,
    sym.st_name);
    if (!is_btf_id(name))
    continue;
//
// __BTF_ID__TYPE__vfs_truncate__0
// prefix =  ^
//
    prefix = name + sizeof(BTF_ID_PREFIX) - 1;
// struct
    if (!strncmp(prefix, BTF_STRUCT, sizeof(BTF_STRUCT) - 1)) {
    obj.nr_structs++;
    id = add_symbol(&obj.structs, prefix, sizeof(BTF_STRUCT) - 1);
// union
    } else if (!strncmp(prefix, BTF_UNION, sizeof(BTF_UNION) - 1)) {
    obj.nr_unions++;
    id = add_symbol(&obj.unions, prefix, sizeof(BTF_UNION) - 1);
// typedef
    } else if (!strncmp(prefix, BTF_TYPEDEF, sizeof(BTF_TYPEDEF) - 1)) {
    obj.nr_typedefs++;
    id = add_symbol(&obj.typedefs, prefix, sizeof(BTF_TYPEDEF) - 1);
// func
    } else if (!strncmp(prefix, BTF_FUNC, sizeof(BTF_FUNC) - 1)) {
    obj.nr_funcs++;
    id = add_symbol(&obj.funcs, prefix, sizeof(BTF_FUNC) - 1);
// set8
    } else if (!strncmp(prefix, BTF_SET8, sizeof(BTF_SET8) - 1)) {
    id = add_set(obj, prefix, BTF_ID_KIND_SET8);
//
// SET8 objects store list's count, which is encoded
// in symbol's size, together with 'cnt' field hence
// that - 1.
//
    if (id)
    id.cnt = sym.st_size / sizeof(uint64_t) - 1;
// set
    } else if (!strncmp(prefix, BTF_SET, sizeof(BTF_SET) - 1)) {
    id = add_set(obj, prefix, BTF_ID_KIND_SET);
//
// SET objects store list's count, which is encoded
// in symbol's size, together with 'cnt' field hence
// that - 1.
//
    if (id)
    id.cnt = sym.st_size / sizeof(int) - 1;
    } else {
    pr_err("FAILED unsupported prefix %s\n", prefix);
    return -1;
    }
    if (!id)
    return -EINVAL;
    if (id.addr_cnt >= ADDR_CNT) {
    pr_err("FAILED symbol %s crossed the number of allowed lists\n",
    id.name);
    return -1;
    }
    id.addr[id.addr_cnt++] = sym.st_value;
    if (push_addr_sym(obj, sym.st_value, id.name))
    return -1;
    }
    if (obj.addr_syms_cnt)
    qsort(obj.addr_syms, obj.addr_syms_cnt,
    sizeof(*obj.addr_syms), cmp_addr_sym);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn load_btf(obj: *mut object) -> c_int {
    static int load_btf(struct object *obj)
    {
    struct btf *base_btf = core::ptr::null_mut(), *btf = core::ptr::null_mut();
    int err;
    if (obj.base_btf_path) {
    base_btf = btf__parse(obj.base_btf_path, core::ptr::null_mut());
    err = libbpf_get_error(base_btf);
    if (err) {
    pr_err("FAILED: load base BTF from %s: %s\n",
    obj.base_btf_path, strerror(-err));
    goto out_err;
    }
    }
    btf = btf__parse_split(obj.btf_path ?: obj.path, base_btf);
    err = libbpf_get_error(btf);
    if (err) {
    pr_err("FAILED: load BTF from %s: %s\n",
    obj.btf_path ?: obj.path, strerror(-err));
    goto out_err;
    }
    obj.base_btf = base_btf;
    obj.btf = btf;
    return 0;
    out_err:
    btf__free(base_btf);
    btf__free(btf);
    obj.base_btf = core::ptr::null_mut();
    obj.btf = core::ptr::null_mut();
    return err;
    }
#[no_mangle]
unsafe extern "C" fn symbols_resolve(obj: *mut object) -> c_int {
    static int symbols_resolve(struct object *obj)
    {
    let mut nr_typedefs: c_int = obj.nr_typedefs;
    let mut nr_structs: c_int = obj.nr_structs;
    let mut nr_unions: c_int = obj.nr_unions;
    let mut nr_funcs: c_int = obj.nr_funcs;
    struct btf *btf = obj.btf;
    int err, type_id;
    __u32 nr_types;
    err = -1;
    nr_types = btf__type_cnt(btf);
//
// Iterate all the BTF types and search for collected symbol IDs.
//
    for (type_id = 1; type_id < nr_types; type_id++) {
    const struct btf_type *type;
    struct rb_root *root;
    struct btf_id *id;
    const char *str;
    int *nr;
    type = btf__type_by_id(btf, type_id);
    if (!type) {
    pr_err("FAILED: malformed BTF, can't resolve type for ID %d\n",
    type_id);
    goto out;
    }
    if (btf_is_func(type) && nr_funcs) {
    nr   = &nr_funcs;
    root = &obj.funcs;
    } else if (btf_is_struct(type) && nr_structs) {
    nr   = &nr_structs;
    root = &obj.structs;
    } else if (btf_is_union(type) && nr_unions) {
    nr   = &nr_unions;
    root = &obj.unions;
    } else if (btf_is_typedef(type) && nr_typedefs) {
    nr   = &nr_typedefs;
    root = &obj.typedefs;
    } else
    continue;
    str = btf__name_by_offset(btf, type.name_off);
    if (!str) {
    pr_err("FAILED: malformed BTF, can't resolve name for ID %d\n",
    type_id);
    goto out;
    }
    id = btf_id__find(root, str);
    if (id) {
    if (id.id) {
    pr_info("WARN: multiple IDs found for '%s': %d, %d - using %d\n",
    str, id.id, type_id, id.id);
    warnings++;
    } else {
    id.id = type_id;
    (*nr)--;
    }
    }
    }
    err = 0;
    out:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn id_patch(obj: *mut object, id: *mut btf_id) -> c_int {
    static int id_patch(struct object *obj, struct btf_id *id)
    {
    Elf_Data *data = obj.efile.idlist;
    int *ptr = data.d_buf;
    int i;
// For set, set8, id->id may be 0
    if (!id.id && id.kind != BTF_ID_KIND_SET && id.kind != BTF_ID_KIND_SET8) {
    pr_err("WARN: resolve_btfids: unresolved symbol %s\n", id.name);
    warnings++;
    }
    for (i = 0; i < id.addr_cnt; i++) {
    let mut addr: c_ulong = id.addr[i];
    let mut idx: c_ulong = addr - obj.efile.idlist_addr;
    pr_debug("patching addr %5lu: ID %7d [%s]\n",
    idx, id.id, id.name);
    if (idx >= data.d_size) {
    pr_err("FAILED patching index %lu out of bounds %lu\n",
    idx, data.d_size);
    return -1;
    }
    idx = idx / sizeof(int);
    ptr[idx] = id.id;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __symbols_patch(obj: *mut object, root: *mut rb_root) -> c_int {
    static int __symbols_patch(struct object *obj, struct rb_root *root)
    {
    struct rb_node *next;
    struct btf_id *id;
    next = rb_first(root);
    while (next) {
    id = rb_entry(next, struct btf_id, rb_node);
    if (id_patch(obj, id))
    return -1;
    next = rb_next(next);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cmp_id(pa: *const c_void, pb: *const c_void) -> c_int {
    static int cmp_id(const void *pa, const void *pb)
    {
    const int *a = pa, *b = pb;
    return *a - *b;
    }
#[no_mangle]
unsafe extern "C" fn sets_patch(obj: *mut object) -> c_int {
    static int sets_patch(struct object *obj)
    {
    Elf_Data *data = obj.efile.idlist;
    struct rb_node *next;
    int cnt;
    next = rb_first(&obj.sets);
    while (next) {
    struct btf_id_set8 *set8 = core::ptr::null_mut();
    struct btf_id_set *set = core::ptr::null_mut();
    unsigned long addr, off;
    struct btf_id *id;
    id   = rb_entry(next, struct btf_id, rb_node);
    addr = id.addr[0];
    off = addr - obj.efile.idlist_addr;
// sets are unique
    if (id.addr_cnt != 1) {
    pr_err("FAILED malformed data for set '%s'\n",
    id.name);
    return -1;
    }
    switch (id.kind) {
    case BTF_ID_KIND_SET:
    set = data.d_buf + off;
    cnt = set.cnt;
    qsort(set.ids, set.cnt, sizeof(set.ids[0]), cmp_id);
    break;
    case BTF_ID_KIND_SET8:
    set8 = data.d_buf + off;
    cnt = set8.cnt;
//
// Make sure id is at the beginning of the pairs
// struct, otherwise the below qsort would not work.
//
    BUILD_BUG_ON((u32 *)set8.pairs != &set8.pairs[0].id);
    qsort(set8.pairs, set8.cnt, sizeof(set8.pairs[0]), cmp_id);
    break;
    default:
    pr_err("Unexpected btf_id_kind %d for set '%s'\n", id.kind, id.name);
    return -1;
    }
    pr_debug("sorting  addr %5lu: cnt %6d [%s]\n", off, cnt, id.name);
    next = rb_next(next);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn symbols_patch(obj: *mut object) -> c_int {
    static int symbols_patch(struct object *obj)
    {
    if (__symbols_patch(obj, &obj.structs)  ||
    __symbols_patch(obj, &obj.unions)   ||
    __symbols_patch(obj, &obj.typedefs) ||
    __symbols_patch(obj, &obj.funcs)    ||
    __symbols_patch(obj, &obj.sets))
    return -1;
    if (sets_patch(obj))
    return -1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dump_raw_data(out_path: *const c_char, data: *const c_void, size: u32) -> c_int {
    static int dump_raw_data(const char *out_path, const void *data, u32 size)
    {
    size_t written;
    FILE *file;
    file = fopen(out_path, "wb");
    if (!file) {
    pr_err("Couldn't open %s for writing\n", out_path);
    return -1;
    }
    written = fwrite(data, 1, size, file);
    if (written != size) {
    pr_err("Failed to write data to %s\n", out_path);
    fclose(file);
    unlink(out_path);
    return -1;
    }
    fclose(file);
    pr_debug("Dumped %lu bytes of data to %s\n", size, out_path);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dump_raw_btf_ids(obj: *mut object, out_path: *const c_char) -> c_int {
    static int dump_raw_btf_ids(struct object *obj, const char *out_path)
    {
    Elf_Data *data = obj.efile.idlist;
    int err;
    if (!data || !data.d_buf) {
    pr_debug("%s has no BTF_ids data to dump\n", obj.path);
    return 0;
    }
//
// If target endianness differs from host, we need to bswap32 the
// .BTF_ids section data before dumping so that the output is in
// target endianness.
//
    if (obj.efile.encoding != ELFDATANATIVE) {
    pr_debug("bswap_32 .BTF_ids data from host to target endianness\n");
    bswap_32_data(data.d_buf, data.d_size);
    }
    err = dump_raw_data(out_path, data.d_buf, data.d_size);
    if (err)
    return -1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dump_raw_btf(btf: *mut btf, out_path: *const c_char) -> c_int {
    static int dump_raw_btf(struct btf *btf, const char *out_path)
    {
    const void *raw_btf_data;
    u32 raw_btf_size;
    int err;
    raw_btf_data = btf__raw_data(btf, &raw_btf_size);
    if (!raw_btf_data) {
    pr_err("btf__raw_data() failed\n");
    return -1;
    }
    err = dump_raw_data(out_path, raw_btf_data, raw_btf_size);
    if (err)
    return -1;
    return 0;
    }
    static const struct btf_type *btf_type_skip_qualifiers(const struct btf *btf, s32 type_id)
    {
    const struct btf_type *t = btf__type_by_id(btf, type_id);
    while (btf_is_mod(t))
    t = btf__type_by_id(btf, t.type);
    return t;
    }
#[no_mangle]
unsafe extern "C" fn push_decl_tag_id(ctx: *mut btf2btf_context, decl_tag_id: u32) -> c_int {
    static int push_decl_tag_id(struct btf2btf_context *ctx, u32 decl_tag_id)
    {
    if (ensure_mem(&ctx.decl_tags, &ctx.max_decl_tags, ctx.nr_decl_tags + 1))
    return -ENOMEM;
    ctx.decl_tags[ctx.nr_decl_tags++] = decl_tag_id;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn push_kfunc(ctx: *mut btf2btf_context, kfunc: *mut kfunc) -> c_int {
    static int push_kfunc(struct btf2btf_context *ctx, struct kfunc *kfunc)
    {
    struct rb_node **p = &ctx.kfuncs.rb_node;
    struct rb_node *parent = core::ptr::null_mut();
    struct kfunc *k;
//
// Dedup by BTF ID: collecting the same kfunc twice is a no-op,
// UNLESS the kfunc flags are inconsistent, in which case we
// fail hard because it indicates a bug in a kfunc set declaration.
//
    while (*p) {
    parent = *p;
    k = rb_entry(parent, struct kfunc, rb_node);
    if (kfunc.btf_id < k.btf_id) {
    p = &(*p).rb_left;
    } else if (kfunc.btf_id > k.btf_id) {
    p = &(*p).rb_right;
    } else if (k.flags == kfunc.flags) {
    return 0;
    } else {
    pr_err("ERROR: resolve_btfids: kfunc %s has inconsistent flags across BTF ID sets: 0x%x != 0x%x\n",
    kfunc.name, k.flags, kfunc.flags);
    return -EINVAL;
    }
    }
    k = zalloc(sizeof(*k));
    if (!k)
    return -ENOMEM;
// k = *kfunc;
    rb_link_node(&k.rb_node, parent, p);
    rb_insert_color(&k.rb_node, &ctx.kfuncs);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn free_kfuncs(root: *mut rb_root) {
    static void free_kfuncs(struct rb_root *root)
    {
    struct rb_node *next;
    struct kfunc *kfunc;
    next = rb_first(root);
    while (next) {
    kfunc = rb_entry(next, struct kfunc, rb_node);
    next = rb_next(&kfunc.rb_node);
    rb_erase(&kfunc.rb_node, root);
    free(kfunc);
    }
    }
#[no_mangle]
unsafe extern "C" fn collect_decl_tags(ctx: *mut btf2btf_context) -> c_int {
    static int collect_decl_tags(struct btf2btf_context *ctx)
    {
    let mut type_cnt: u32 = btf__type_cnt(ctx.btf);
    struct btf *btf = ctx.btf;
    const struct btf_type *t;
    int err;
    for (u32 id = 1; id < type_cnt; id++) {
    t = btf__type_by_id(btf, id);
    if (!btf_is_decl_tag(t))
    continue;
    err = push_decl_tag_id(ctx, id);
    if (err)
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn param_name_has_suffix(name: *const c_char, suffix: *const c_char) -> bool {
    static bool param_name_has_suffix(const char *name, const char *suffix)
    {
    let mut name_len: usize = strlen(name);
    let mut suffix_len: usize = strlen(suffix);
    return name_len >= suffix_len && !strcmp(name + name_len - suffix_len, suffix);
    }
#[no_mangle]
unsafe extern "C" fn is_arena_param(btf: *const btf, param: *const btf_param) -> bool {
    static bool is_arena_param(const struct btf *btf, const struct btf_param *param)
    {
    const char *name = btf__name_by_offset(btf, param.name_off);
    return param_name_has_suffix(name, PARAM_SUFFIX_ARENA) ||
    param_name_has_suffix(name, PARAM_SUFFIX_ARENA_NULLABLE);
    }
#[no_mangle]
unsafe extern "C" fn collect_kfuncs(obj: *mut object, ctx: *mut btf2btf_context) -> c_int {
    static int collect_kfuncs(struct object *obj, struct btf2btf_context *ctx)
    {
    Elf_Data *idlist = obj.efile.idlist;
    struct btf *btf = ctx.btf;
    struct rb_node *next;
    if (!idlist || !idlist.d_buf)
    return 0;
    for (next = rb_first(&obj.sets); next; next = rb_next(next)) {
    struct btf_id_set8 *set8;
    struct btf_id *set_id;
    u64 set_addr;
    set_id = rb_entry(next, struct btf_id, rb_node);
    if (set_id.kind != BTF_ID_KIND_SET8 || set_id.addr_cnt != 1)
    continue;
    set_addr = set_id.addr[0];
    set8 = idlist.d_buf + (set_addr - obj.efile.idlist_addr);
    if (!(set8.flags & BTF_SET8_KFUNCS))
    continue;
    for (u32 i = 0; i < set_id.cnt; i++) {
    let mut off: usize = (char *)&set8.pairs[i] - (char *)set8;
    const char *name = find_name_by_addr(obj, set_addr + off);
    struct kfunc kfunc;
    s32 func_id;
    int err;
    if (!name) {
    pr_err("WARN: resolve_btfids: no BTF ID symbol for %s entry %u\n",
    set_id.name, i);
    warnings++;
    continue;
    }
    func_id = btf__find_by_name_kind_own(btf, name, BTF_KIND_FUNC);
    if (func_id < 0) {
    pr_err("WARN: resolve_btfids: no BTF func for kfunc %s in %s\n",
    name, set_id.name);
    warnings++;
    continue;
    }
    pr_debug("found kfunc %s in %s\n", name, set_id.name);
    kfunc.name = name;
    kfunc.btf_id = func_id;
    kfunc.flags = set8.pairs[i].flags;
    err = push_kfunc(ctx, &kfunc);
    if (err)
    return err;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn build_btf2btf_context(obj: *mut object, ctx: *mut btf2btf_context) -> c_int {
    static int build_btf2btf_context(struct object *obj, struct btf2btf_context *ctx)
    {
    int err;
    ctx.btf = obj.btf;
    err = collect_decl_tags(ctx);
    if (err) {
    pr_err("ERROR: resolve_btfids: failed to collect decl tags from BTF\n");
    return err;
    }
    err = collect_kfuncs(obj, ctx);
    if (err) {
    pr_err("ERROR: resolve_btfids: failed to collect kfuncs from BTF\n");
    return err;
    }
    return 0;
    }
// Implicit BPF kfunc arguments can only be of particular types
#[no_mangle]
unsafe extern "C" fn is_kf_implicit_arg(btf: *const btf, p: *const btf_param) -> bool {
    static bool is_kf_implicit_arg(const struct btf *btf, const struct btf_param *p)
    {
    static const char *const kf_implicit_arg_types[] = {
    "bpf_prog_aux",
    "btf_struct_meta",
    };
    const struct btf_type *t;
    const char *name;
    t = btf_type_skip_qualifiers(btf, p.type);
    if (!btf_is_ptr(t))
    return false;
    t = btf_type_skip_qualifiers(btf, t.type);
    if (!btf_is_struct(t))
    return false;
    name = btf__name_by_offset(btf, t.name_off);
    if (!name)
    return false;
    for (int i = 0; i < ARRAY_SIZE(kf_implicit_arg_types); i++)
    if (strcmp(name, kf_implicit_arg_types[i]) == 0)
    return true;
    return false;
    }
//
// For a kfunc with KF_IMPLICIT_ARGS we do the following:
// 1. Add a new function with _impl suffix in the name, with the prototype
// of the original kfunc.
// 2. Add all decl tags except "bpf_kfunc" for the _impl func.
// 3. Add a new function prototype with modified list of arguments:
// omitting implicit args.
// 4. Change the prototype of the original kfunc to the new one.
//
// This way we transform the BTF associated with the kfunc from
// __bpf_kfunc bpf_foo(int arg1, void *implicit_arg);
// into
// bpf_foo_impl(int arg1, void *implicit_arg);
// __bpf_kfunc bpf_foo(int arg1);
//
// If a kfunc with KF_IMPLICIT_ARGS already has an _impl counterpart
// in BTF, then it's a legacy case: an _impl function is declared in the
// source code. In this case, we can skip adding an _impl function, but we
// still have to add a func prototype that omits implicit args.
//
#[no_mangle]
unsafe extern "C" fn process_kfunc_with_implicit_args(ctx: *mut btf2btf_context, kfunc: *mut kfunc) -> c_int {
    static int process_kfunc_with_implicit_args(struct btf2btf_context *ctx, struct kfunc *kfunc)
    {
    s32 idx, new_proto_id, new_func_id, proto_id;
    const char *param_name, *tag_name;
    const struct btf_param *params;
    enum btf_func_linkage linkage;
    char tmp_name[KSYM_NAME_LEN];
    struct btf *btf = ctx.btf;
    int err, len, nr_params;
    struct btf_type *t;
    t = (struct btf_type *)btf__type_by_id(btf, kfunc.btf_id);
    if (!t || !btf_is_func(t)) {
    pr_err("ERROR: resolve_btfids: btf id %d is not a function\n", kfunc.btf_id);
    return -EINVAL;
    }
    linkage = btf_vlen(t);
    proto_id = t.type;
    t = (struct btf_type *)btf__type_by_id(btf, proto_id);
    if (!t || !btf_is_func_proto(t)) {
    pr_err("ERROR: resolve_btfids: btf id %d is not a function prototype\n", proto_id);
    return -EINVAL;
    }
    len = snprintf(tmp_name, sizeof(tmp_name), "%s%s", kfunc.name, KF_IMPL_SUFFIX);
    if (len < 0 || len >= sizeof(tmp_name)) {
    pr_err("ERROR: function name is too long: %s%s\n", kfunc.name, KF_IMPL_SUFFIX);
    return -E2BIG;
    }
    if (btf__find_by_name_kind_own(btf, tmp_name, BTF_KIND_FUNC) > 0) {
    pr_debug("resolve_btfids: function %s already exists in BTF\n", tmp_name);
    goto add_new_proto;
    }
// Add a new function with _impl suffix and original prototype
    new_func_id = btf__add_func(btf, tmp_name, linkage, proto_id);
    if (new_func_id < 0) {
    pr_err("ERROR: resolve_btfids: failed to add func %s to BTF\n", tmp_name);
    return new_func_id;
    }
// Copy all decl tags except "bpf_kfunc" from the original kfunc to the new one
    for (int i = 0; i < ctx.nr_decl_tags; i++) {
    t = (struct btf_type *)btf__type_by_id(btf, ctx.decl_tags[i]);
    if (t.type != kfunc.btf_id)
    continue;
    tag_name = btf__name_by_offset(btf, t.name_off);
    if (strcmp(tag_name, DECL_TAG_KFUNC) == 0)
    continue;
    idx = btf_decl_tag(t).component_idx;
    if (btf_kflag(t))
    err = btf__add_decl_attr(btf, tag_name, new_func_id, idx);
    else
    err = btf__add_decl_tag(btf, tag_name, new_func_id, idx);
    if (err < 0) {
    pr_err("ERROR: resolve_btfids: failed to add decl tag %s for %s\n",
    tag_name, tmp_name);
    return -EINVAL;
    }
    }
    add_new_proto:
    t = (struct btf_type *)btf__type_by_id(btf, proto_id);
    new_proto_id = btf__add_func_proto(btf, t.type);
    if (new_proto_id < 0) {
    pr_err("ERROR: resolve_btfids: failed to add func proto for %s\n", kfunc.name);
    return new_proto_id;
    }
// Add non-implicit args to the new prototype
    t = (struct btf_type *)btf__type_by_id(btf, proto_id);
    nr_params = btf_vlen(t);
    for (int i = 0; i < nr_params; i++) {
    params = btf_params(t);
    if (is_kf_implicit_arg(btf, &params[i]))
    break;
    param_name = btf__name_by_offset(btf, params[i].name_off);
    err = btf__add_func_param(btf, param_name, params[i].type);
    if (err < 0) {
    pr_err("ERROR: resolve_btfids: failed to add param %s for %s\n",
    param_name, kfunc.name);
    return err;
    }
    t = (struct btf_type *)btf__type_by_id(btf, proto_id);
    }
// Finally change the prototype of the original kfunc to the new one
    t = (struct btf_type *)btf__type_by_id(btf, kfunc.btf_id);
    t.type = new_proto_id;
    pr_debug("resolve_btfids: updated BTF for kfunc with implicit args %s\n", kfunc.name);
    return 0;
    }
    static bool is_arena_arg(const struct btf *btf, const struct kfunc *kfunc,
    const struct btf_param *param, u32 idx)
    {
    if (is_arena_param(btf, param))
    return true;
    switch (idx) {
    case 0:
    return kfunc.flags & KF_ARENA_ARG1;
    case 1:
    return kfunc.flags & KF_ARENA_ARG2;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn arena_tag_ptr(btf: *mut btf, ptr_id: u32, kfunc: *mut kfunc) -> i32 {
    static s32 arena_tag_ptr(struct btf *btf, u32 ptr_id, struct kfunc *kfunc)
    {
    const struct btf_type *ptr = btf__type_by_id(btf, ptr_id);
    s32 tag_id, new_ptr_id;
    if (!btf_is_ptr(ptr)) {
    pr_err("ERROR: resolve_btfids: kfunc %s: arena type is not a pointer\n",
    kfunc.name);
    return -EINVAL;
    }
    tag_id = btf__add_type_attr(btf, TYPE_ATTR_ARENA, ptr.type);
    if (tag_id < 0) {
    pr_err("ERROR: resolve_btfids: kfunc %s: failed to add a type attr to BTF: %d\n",
    kfunc.name, tag_id);
    return tag_id;
    }
    new_ptr_id = btf__add_ptr(btf, tag_id);
    if (new_ptr_id < 0) {
    pr_err("ERROR: resolve_btfids: kfunc %s: failed to add a pointer to BTF: %d\n",
    kfunc.name, new_ptr_id);
    }
    return new_ptr_id;
    }
//
// Add a FUNC_PROTO for @kfunc with each arena pointer tagged with an
// "address_space(1)" attribute. The original proto may be shared with
// other FUNCs, so it is never modified in place. Returns the original
// proto id when @kfunc has no arena return value or arguments.
//
#[no_mangle]
unsafe extern "C" fn add_arena_tagged_proto(btf: *mut btf, kfunc: *mut kfunc) -> i32 {
    static s32 add_arena_tagged_proto(struct btf *btf, struct kfunc *kfunc)
    {
    const struct btf_type *func = btf__type_by_id(btf, kfunc.btf_id);
    let mut proto_id: u32 = func.type;
    const struct btf_type *proto = btf__type_by_id(btf, proto_id);
    const struct btf_param *params = btf_params(proto);
    let mut nr_params: u32 = btf_vlen(proto);
    let mut ret_type_id: i32 = proto.type;
    const struct btf_type *t;
    struct btf_param *tag_params;
    s32 new_proto_id, id;
    const char *name;
    let mut has_arena_arg: bool = false;
    int err, i;
    for (i = 0; i < nr_params; i++) {
    if (is_arena_arg(btf, kfunc, &params[i], i)) {
    has_arena_arg = true;
    break;
    }
    }
    if (!(kfunc.flags & KF_ARENA_RET) && !has_arena_arg)
    return proto_id;
    if (kfunc.flags & KF_ARENA_RET) {
    ret_type_id = arena_tag_ptr(btf, ret_type_id, kfunc);
    if (ret_type_id < 0)
    return ret_type_id;
    }
    new_proto_id = btf__add_func_proto(btf, ret_type_id);
    if (new_proto_id < 0) {
    pr_err("ERROR: resolve_btfids: kfunc %s: failed to add a func proto to BTF: %d\n",
    kfunc.name, new_proto_id);
    return new_proto_id;
    }
    for (i = 0; i < nr_params; i++) {
// btf__add_func_param() below may move the proto, re-fetch
    proto = btf__type_by_id(btf, proto_id);
    name = btf__name_by_offset(btf, btf_params(proto)[i].name_off);
    err = btf__add_func_param(btf, name ?: "", btf_params(proto)[i].type);
    if (err < 0) {
    pr_err("ERROR: resolve_btfids: kfunc %s: failed to add a proto param to BTF: %d\n",
    kfunc.name, err);
    return err;
    }
    }
    for (i = 0; i < nr_params; i++) {
    t = btf__type_by_id(btf, new_proto_id);
    tag_params = btf_params(t);
    if (!is_arena_arg(btf, kfunc, &tag_params[i], i))
    continue;
    id = arena_tag_ptr(btf, tag_params[i].type, kfunc);
    if (id < 0)
    return id;
    t = btf__type_by_id(btf, new_proto_id);
    tag_params = btf_params(t);
    tag_params[i].type = id;
    }
    pr_debug("added arena-tagged proto for kfunc %s: %d\n", kfunc.name, new_proto_id);
    return new_proto_id;
    }
    static int process_kfunc_with_arena_attrs(struct btf2btf_context *ctx,
    struct kfunc *kfunc)
    {
    struct btf_type *t;
    s32 proto_id;
    proto_id = add_arena_tagged_proto(ctx.btf, kfunc);
    if (proto_id < 0)
    return proto_id;
    t = (struct btf_type *)btf__type_by_id(ctx.btf, kfunc.btf_id);
    t.type = proto_id;
    return 0;
    }
    static int add_decl_tag(struct btf2btf_context *ctx, const char *tag_name,
    u32 target_btf_id, int component_idx)
    {
    s32 new_id;
    new_id = btf__add_decl_tag(ctx.btf, tag_name, target_btf_id, component_idx);
    if (new_id < 0) {
    pr_err("ERROR: resolve_btfids: failed to add '%s' decl tag for BTF id %u: %d\n",
    tag_name, target_btf_id, new_id);
    return new_id;
    }
    return push_decl_tag_id(ctx, new_id);
    }
#[no_mangle]
unsafe extern "C" fn btf2btf(obj: *mut object) -> c_int {
    static int btf2btf(struct object *obj)
    {
    let mut ctx: btf2btf_context = {};
    struct rb_node *next;
    int err;
    err = build_btf2btf_context(obj, &ctx);
    if (err)
    goto out;
    for (next = rb_first(&ctx.kfuncs); next; next = rb_next(next)) {
    struct kfunc *kfunc = rb_entry(next, struct kfunc, rb_node);
    err = add_decl_tag(&ctx, DECL_TAG_KFUNC, kfunc.btf_id, -1);
    if (err)
    goto out;
    if (kfunc.flags & KF_FASTCALL) {
    err = add_decl_tag(&ctx, DECL_TAG_FASTCALL, kfunc.btf_id, -1);
    if (err)
    goto out;
    }
    if (kfunc.flags & KF_IMPLICIT_ARGS) {
    err = process_kfunc_with_implicit_args(&ctx, kfunc);
    if (err)
    goto out;
    }
    err = process_kfunc_with_arena_attrs(&ctx, kfunc);
    if (err)
    goto out;
    }
    err = 0;
    out:
    free(ctx.decl_tags);
    free_kfuncs(&ctx.kfuncs);
    return err;
    }
//
// Sort types by name in ascending order resulting in all
// anonymous types being placed before named types.
//
#[no_mangle]
unsafe extern "C" fn cmp_type_names(a: *const c_void, b: *const c_void, priv: *mut c_void) -> c_int {
    static int cmp_type_names(const void *a, const void *b, void *priv)
    {
    struct btf *btf = (struct btf *)priv;
    const struct btf_type *ta = btf__type_by_id(btf, *(__u32 *)a);
    const struct btf_type *tb = btf__type_by_id(btf, *(__u32 *)b);
    const char *na, *nb;
    int r;
    na = btf__str_by_offset(btf, ta.name_off);
    nb = btf__str_by_offset(btf, tb.name_off);
    r = strcmp(na, nb);
    if (r != 0)
    return r;
// preserve original relative order of anonymous or same-named types
    return *(__u32 *)a < *(__u32 *)b ? -1 : 1;
    }
#[no_mangle]
unsafe extern "C" fn sort_btf_by_name(btf: *mut btf) -> c_int {
    static int sort_btf_by_name(struct btf *btf)
    {
    __u32 *permute_ids = core::ptr::null_mut(), *id_map = core::ptr::null_mut();
    int nr_types, i, err = 0;
    let mut start_id: __u32 = 0, id;
    if (btf__base_btf(btf))
    start_id = btf__type_cnt(btf__base_btf(btf));
    nr_types = btf__type_cnt(btf) - start_id;
    permute_ids = calloc(nr_types, sizeof(*permute_ids));
    if (!permute_ids) {
    err = -ENOMEM;
    goto out;
    }
    id_map = calloc(nr_types, sizeof(*id_map));
    if (!id_map) {
    err = -ENOMEM;
    goto out;
    }
    for (i = 0, id = start_id; i < nr_types; i++, id++)
    permute_ids[i] = id;
    qsort_r(permute_ids, nr_types, sizeof(*permute_ids), cmp_type_names,
    btf);
    for (i = 0; i < nr_types; i++) {
    id = permute_ids[i] - start_id;
    id_map[id] = i + start_id;
    }
    err = btf__permute(btf, id_map, nr_types, core::ptr::null_mut());
    if (err)
    pr_err("FAILED: btf permute: %s\n", strerror(-err));
    out:
    free(permute_ids);
    free(id_map);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn finalize_btf(obj: *mut object) -> c_int {
    static int finalize_btf(struct object *obj)
    {
    struct btf *base_btf = obj.base_btf, *btf = obj.btf;
    int err;
    err = btf__dedup(obj.btf, core::ptr::null_mut());
    if (err) {
    pr_err("FAILED to dedup BTF: %s\n", strerror(errno));
    goto out_err;
    }
    if (obj.base_btf && obj.distill_base) {
    err = btf__distill_base(obj.btf, &base_btf, &btf);
    if (err) {
    pr_err("FAILED to distill base BTF: %s\n", strerror(errno));
    goto out_err;
    }
    btf__free(obj.base_btf);
    btf__free(obj.btf);
    obj.base_btf = base_btf;
    obj.btf = btf;
    }
    err = sort_btf_by_name(obj.btf);
    if (err) {
    pr_err("FAILED to sort BTF: %s\n", strerror(errno));
    goto out_err;
    }
    return 0;
    out_err:
    btf__free(base_btf);
    btf__free(btf);
    obj.base_btf = core::ptr::null_mut();
    obj.btf = core::ptr::null_mut();
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn make_out_path(buf: *mut c_char, buf_sz: u32, in_path: *const c_char, suffix: *const c_char) -> c_int {
    static inline int make_out_path(char *buf, u32 buf_sz, const char *in_path, const char *suffix)
    {
    let mut len: c_int = snprintf(buf, buf_sz, "%s%s", in_path, suffix);
    if (len < 0 || len >= buf_sz) {
    pr_err("Output path is too long: %s%s\n", in_path, suffix);
    return -E2BIG;
    }
    return 0;
    }
//
// Patch the .BTF_ids section of an ELF file with data from provided file.
// Equivalent to: objcopy --update-section .BTF_ids=<btfids> <elf>
//
// 1. Find .BTF_ids section in the ELF
// 2. Verify that blob file size matches section size
// 3. Update section data buffer with blob data
// 4. Write the ELF file
//
#[no_mangle]
unsafe extern "C" fn patch_btfids(btfids_path: *const c_char, elf_path: *const c_char) -> c_int {
    static int patch_btfids(const char *btfids_path, const char *elf_path)
    {
    Elf_Scn *scn = core::ptr::null_mut();
    FILE *btfids_file;
    size_t shdrstrndx;
    int fd, err = -1;
    Elf_Data *data;
    struct stat st;
    GElf_Shdr sh;
    char *name;
    Elf *elf;
    elf_version(EV_CURRENT);
    fd = open(elf_path, O_RDWR, 0666);
    if (fd < 0) {
    pr_err("FAILED to open %s: %s\n", elf_path, strerror(errno));
    return -1;
    }
    elf = elf_begin(fd, ELF_C_RDWR_MMAP, core::ptr::null_mut());
    if (!elf) {
    close(fd);
    pr_err("FAILED cannot create ELF descriptor: %s\n", elf_errmsg(-1));
    return -1;
    }
    elf_flagelf(elf, ELF_C_SET, ELF_F_LAYOUT);
    if (elf_getshdrstrndx(elf, &shdrstrndx) != 0) {
    pr_err("FAILED cannot get shdr str ndx\n");
    goto out;
    }
    while ((scn = elf_nextscn(elf, scn)) != core::ptr::null_mut()) {
    if (gelf_getshdr(scn, &sh) != &sh) {
    pr_err("FAILED to get section header\n");
    goto out;
    }
    name = elf_strptr(elf, shdrstrndx, sh.sh_name);
    if (!name)
    continue;
    if (strcmp(name, BTF_IDS_SECTION) == 0)
    break;
    }
    if (!scn) {
    pr_err("FAILED: section %s not found in %s\n", BTF_IDS_SECTION, elf_path);
    goto out;
    }
    data = elf_getdata(scn, core::ptr::null_mut());
    if (!data) {
    pr_err("FAILED to get %s section data from %s\n", BTF_IDS_SECTION, elf_path);
    goto out;
    }
    if (stat(btfids_path, &st) < 0) {
    pr_err("FAILED to stat %s: %s\n", btfids_path, strerror(errno));
    goto out;
    }
    if ((size_t)st.st_size != data.d_size) {
    pr_err("FAILED: size mismatch - %s section in %s is %zu bytes, %s is %zu bytes\n",
    BTF_IDS_SECTION, elf_path, data.d_size, btfids_path, (size_t)st.st_size);
    goto out;
    }
    btfids_file = fopen(btfids_path, "rb");
    if (!btfids_file) {
    pr_err("FAILED to open %s: %s\n", btfids_path, strerror(errno));
    goto out;
    }
    pr_debug("Copying data from %s to %s section of %s (%zu bytes)\n",
    btfids_path, BTF_IDS_SECTION, elf_path, data.d_size);
    if (fread(data.d_buf, data.d_size, 1, btfids_file) != 1) {
    pr_err("FAILED to read %s\n", btfids_path);
    fclose(btfids_file);
    goto out;
    }
    fclose(btfids_file);
    elf_flagdata(data, ELF_C_SET, ELF_F_DIRTY);
    if (elf_update(elf, ELF_C_WRITE) < 0) {
    pr_err("FAILED to update ELF file %s\n", elf_path);
    goto out;
    }
    err = 0;
    out:
    elf_end(elf);
    close(fd);
    return err;
    }
    static const char * const resolve_btfids_usage[] = {
    "resolve_btfids [<options>] <ELF object>",
    "resolve_btfids --patch_btfids <.BTF_ids file> <ELF object>",
    core::ptr::null_mut()
    };
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *const c_char) -> c_int {
    int main(int argc, const char **argv)
    {
    struct object obj = {
    .efile = {
    .idlist_shndx  = -1,
    .symbols_shndx = -1,
    },
    .structs  = RB_ROOT,
    .unions   = RB_ROOT,
    .typedefs = RB_ROOT,
    .funcs    = RB_ROOT,
    .sets     = RB_ROOT,
    };
    const char *btfids_path = core::ptr::null_mut();
    let mut fatal_warnings: bool = false;
    let mut resolve_btfids: bool = true;
    char out_path[PATH_MAX];
    struct option btfid_options[] = {
    OPT_INCR('v', "verbose", &verbose,
    "be more verbose (show errors, etc)"),
    OPT_STRING(0, "btf", &obj.btf_path, "file",
    "path to a file with input BTF data"),
    OPT_STRING('b', "btf_base", &obj.base_btf_path, "file",
    "path of file providing base BTF"),
    OPT_BOOLEAN(0, "fatal_warnings", &fatal_warnings,
    "turn warnings into errors"),
    OPT_BOOLEAN(0, "distill_base", &obj.distill_base,
    "distill --btf_base and emit .BTF.base section data"),
    OPT_STRING(0, "patch_btfids", &btfids_path, "file",
    "path to .BTF_ids section data blob to patch into ELF file"),
    OPT_END()
    };
    let mut err: c_int = -1;
    argc = parse_options(argc, argv, btfid_options, resolve_btfids_usage,
    PARSE_OPT_STOP_AT_NON_OPTION);
    if (argc != 1)
    usage_with_options(resolve_btfids_usage, btfid_options);
    obj.path = argv[0];
    if (btfids_path)
    return patch_btfids(btfids_path, obj.path);
    if (elf_collect(&obj))
    goto out;
//
// We did not find .BTF_ids section or symbols section,
// nothing to do..
//
    if (obj.efile.idlist_shndx == -1 ||
    obj.efile.symbols_shndx == -1) {
    pr_debug("Cannot find .BTF_ids or symbols sections, skip symbols resolution\n");
    resolve_btfids = false;
    }
    if (resolve_btfids)
    if (symbols_collect(&obj))
    goto out;
    if (load_btf(&obj))
    goto out;
    if (btf2btf(&obj))
    goto out;
    if (finalize_btf(&obj))
    goto out;
    if (!resolve_btfids)
    goto dump_btf;
    if (symbols_resolve(&obj))
    goto out;
    if (symbols_patch(&obj))
    goto out;
    err = make_out_path(out_path, sizeof(out_path), obj.path, BTF_IDS_SECTION);
    err = err ?: dump_raw_btf_ids(&obj, out_path);
    if (err)
    goto out;
    dump_btf:
    err = make_out_path(out_path, sizeof(out_path), obj.path, BTF_ELF_SEC);
    err = err ?: dump_raw_btf(obj.btf, out_path);
    if (err)
    goto out;
    if (obj.base_btf && obj.distill_base) {
    err = make_out_path(out_path, sizeof(out_path), obj.path, BTF_BASE_ELF_SEC);
    err = err ?: dump_raw_btf(obj.base_btf, out_path);
    if (err)
    goto out;
    }
    if (!(fatal_warnings && warnings))
    err = 0;
    out:
    btf__free(obj.base_btf);
    btf__free(obj.btf);
    btf_id__free_all(&obj.structs);
    btf_id__free_all(&obj.unions);
    btf_id__free_all(&obj.typedefs);
    btf_id__free_all(&obj.funcs);
    btf_id__free_all(&obj.sets);
    free(obj.addr_syms);
    if (obj.efile.elf) {
    elf_end(obj.efile.elf);
    close(obj.efile.fd);
    }
    return err;
    }
