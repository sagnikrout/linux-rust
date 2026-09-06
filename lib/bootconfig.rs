//! Automatically rewritten from C to Rust
//! Source: lib/bootconfig.c
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
//
// Extra Boot Config
// Masami Hiramatsu <mhiramat@kernel.org>
//
// NOTE: This is only for tools/bootconfig, because tools/bootconfig will
// run the parser sanity test.
// This does NOT mean lib/bootconfig.c is available in the user space.
// However, if you change this file, please make sure the tools/bootconfig
// has no issue on building and running.
//

// embedded_bootconfig_data is defined in bootconfig-data.S
    extern __visible const char embedded_bootconfig_data[];
    extern __visible const char embedded_bootconfig_data_end[];
#[no_mangle]
pub unsafe extern "C" fn xbc_get_embedded_bootconfig(size: *mut usize) -> *const char  __init {
    const char * __init xbc_get_embedded_bootconfig(size_t *size)
    {
// size = embedded_bootconfig_data_end - embedded_bootconfig_data;
    return (*size) ? embedded_bootconfig_data : core::ptr::null_mut();
    }

// embedded_kernel_cmdline is defined in embedded-cmdline.S
    extern __visible const char embedded_kernel_cmdline[];
    extern __visible const char embedded_kernel_cmdline_end[];
// Set once the embedded cmdline has actually been prepended.
    static bool xbc_cmdline_applied __initdata;
//
// str_prepend() - Prepend @src in front of the string in @dst, in place
// @dst: NUL-terminated destination buffer, currently @dst_len bytes long
// @dst_len: length of the current @dst string (excluding its NUL)
// @src: bytes to prepend (not NUL-terminated)
// @src_len: number of bytes from @src to prepend
//
// The caller must guarantee @dst has room for src_len + dst_len + 1 bytes.
// Moving dst_len + 1 bytes carries @dst's NUL terminator too, so an empty
// @dst needs no special case.
//
    static void __init str_prepend(char *dst, size_t dst_len,
    const char *src, size_t src_len)
    {
    memmove(dst + src_len, dst, dst_len + 1);
    memcpy(dst, src, src_len);
    }
//
// xbc_prepend_embedded_cmdline() - Prepend embedded bootconfig cmdline
// @dst: cmdline buffer to prepend into (must already contain a NUL byte)
// @size: total capacity of @dst in bytes
//
// Prepend the build-time-rendered "kernel" subtree of the embedded
// bootconfig to @dst. The rendered string already ends with a single
// space (the xbc_snprint_cmdline() invariant), which serves as the
// separator between the embedded keys and any existing content of @dst.
// On overflow, log an error and leave @dst untouched rather than
// silently truncating: booting without the embedded values is better
// than refusing to boot, and the error message tells the user why
// their embedded keys are missing.
//
// Intended to be called from setup_arch() before parse_early_param() so
// that early_param() handlers see the embedded values.
//
#[no_mangle]
pub unsafe extern "C" fn xbc_prepend_embedded_cmdline(dst: *mut c_char, size: usize) -> void __init {
    void __init xbc_prepend_embedded_cmdline(char *dst, size_t size)
    {
    let mut embed_len: usize = embedded_kernel_cmdline_end - embedded_kernel_cmdline;
    size_t dst_len;
    if (!size || embed_len <= 1)	/* trailing NUL only */
    return;
    embed_len--;			/* exclude trailing NUL byte */
    dst_len = strnlen(dst, size);
    if (embed_len + dst_len + 1 > size) {
    pr_err("embedded bootconfig cmdline (%zu bytes) does not fit in COMMAND_LINE_SIZE with %zu bytes already used; ignoring embedded values\n",
    embed_len, dst_len);
    return;
    }
    str_prepend(dst, dst_len, embedded_kernel_cmdline, embed_len);
    xbc_cmdline_applied = true;
    }
//
// xbc_embedded_cmdline_applied() - Did the embedded cmdline get prepended?
//
// Return true if xbc_prepend_embedded_cmdline() actually prepended the
// embedded "kernel" subtree. setup_boot_config() uses this to avoid
// rendering the same keys a second time.
//
#[no_mangle]
pub unsafe extern "C" fn xbc_embedded_cmdline_applied() -> bool __init {
    bool __init xbc_embedded_cmdline_applied(void)
    {
    return xbc_cmdline_applied;
    }

// parse_args() callback: flag when the "bootconfig" parameter is present.
    static int __init bootconfig_optin(char *param, char *val,
    const char *unused, void *arg)
    {
    if (!strcmp(param, "bootconfig"))
// (bool *)arg = true;
    return 0;
    }
//
// bootconfig_cmdline_requested() - Was "bootconfig" passed on the cmdline?
// @boot_cmdline: kernel command line to inspect (not modified)
// @end_offset: if non-NULL, set to the offset of the init arguments that
// follow a "--" separator, or 0 when there is none
//
// Parse a private copy of @boot_cmdline (parse_args() is destructive) and
// report whether "bootconfig" is present before the "--" separator.
// setup_arch() uses this to gate prepending the build-time embedded cmdline;
// setup_boot_config() uses it for the runtime opt-in and to locate the init
// arguments via @end_offset. Sharing one parser keeps the early and late
// paths agreeing on what counts as opt-in. CONFIG_BOOT_CONFIG_FORCE is not
// folded in here; callers apply it where they need it.
//
#[no_mangle]
pub unsafe extern "C" fn bootconfig_cmdline_requested(boot_cmdline: *const c_char, end_offset: *mut c_int) -> bool __init {
    bool __init bootconfig_cmdline_requested(const char *boot_cmdline, int *end_offset)
    {
    static char tmp_cmdline[COMMAND_LINE_SIZE] __initdata;
    let mut found: bool = false;
    char *err;
    if (end_offset)
// end_offset = 0;
    strscpy(tmp_cmdline, boot_cmdline, COMMAND_LINE_SIZE);
    err = parse_args("bootconfig", tmp_cmdline, core::ptr::null_mut(), 0, 0, 0,
    &found, bootconfig_optin);
    if (IS_ERR(err))
    return false;
// parse_args() stops at "--" and returns the address of the rest.
    if (end_offset && err)
// end_offset = err - tmp_cmdline;
    return found;
    }

//
// Extra Boot Config (XBC) is given as tree-structured ascii text of
// key-value pairs on memory.
// xbc_parse() parses the text to build a simple tree. Each tree node is
// simply a key word or a value. A key node may have a next key node or/and
// a child node (both key and value). A value node may have a next value
// node (for array).
//
    static struct xbc_node *xbc_nodes __initdata;
    static int xbc_node_num __initdata;
    static char *xbc_data __initdata;
    static size_t xbc_data_size __initdata;
    static struct xbc_node *last_parent __initdata;
    static const char *xbc_err_msg __initdata;
    static int xbc_err_pos __initdata;
    static int open_brace[XBC_DEPTH_MAX] __initdata;
    static int brace_index __initdata;

#[no_mangle]
pub unsafe extern "C" fn xbc_alloc_mem(size: usize) -> *mut void  __init {
    static inline void * __init xbc_alloc_mem(size_t size)
    {
    return memblock_alloc(size, SMP_CACHE_BYTES);
    }
#[no_mangle]
pub unsafe extern "C" fn xbc_free_mem(addr: *mut c_void, size: usize, early: bool) -> void __init {
    static inline void __init xbc_free_mem(void *addr, size_t size, bool early)
    {
    if (early)
    memblock_free(addr, size);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: addr) -> else {
    else if (addr)
    memblock_free(addr, size);
    }

    static inline void *xbc_alloc_mem(size_t size)
    {
    return calloc(1, size);
    }
#[no_mangle]
pub unsafe extern "C" fn xbc_free_mem(addr: *mut c_void, size: usize, early: bool) {
    static inline void xbc_free_mem(void *addr, size_t size, bool early)
    {
    free(addr);
    }

//
// xbc_get_info() - Get the information of loaded boot config
// @node_size: A pointer to store the number of nodes.
// @data_size: A pointer to store the size of bootconfig data.
//
// Get the number of used nodes in @node_size if it is not NULL,
// and the size of bootconfig data in @data_size if it is not NULL.
// Return 0 if the boot config is initialized, or return -ENODEV.
//
#[no_mangle]
pub unsafe extern "C" fn xbc_get_info(node_size: *mut c_int, data_size: *mut usize) -> int __init {
    int __init xbc_get_info(int *node_size, size_t *data_size)
    {
    if (!xbc_data)
    return -ENODEV;
    if (node_size)
// node_size = xbc_node_num;
    if (data_size)
// data_size = xbc_data_size;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xbc_parse_error(msg: *const c_char, p: *const c_char) -> int __init {
    static int __init xbc_parse_error(const char *msg, const char *p)
    {
    xbc_err_msg = msg;
    xbc_err_pos = (int)(p - xbc_data);
    return -EINVAL;
    }
//
// xbc_root_node() - Get the root node of extended boot config
//
// Return the address of root node of extended boot config. If the
// extended boot config is not initialized, return NULL.
//
#[no_mangle]
pub unsafe extern "C" fn xbc_root_node() -> *mut xbc_node  __init {
    struct xbc_node * __init xbc_root_node(void)
    {
    if (unlikely(!xbc_data))
    return core::ptr::null_mut();
    return xbc_nodes;
    }
//
// xbc_node_index() - Get the index of XBC node
// @node: A target node of getting index.
//
// Return the index number of @node in XBC node list.
//
#[no_mangle]
pub unsafe extern "C" fn xbc_node_index(node: *mut xbc_node) -> uint16_t __init {
    uint16_t __init xbc_node_index(struct xbc_node *node)
    {
    return (uint16_t)(node - &xbc_nodes[0]);
    }
//
// xbc_node_get_parent() - Get the parent XBC node
// @node: An XBC node.
//
// Return the parent node of @node. If the node is top node of the tree,
// return NULL.
//
#[no_mangle]
pub unsafe extern "C" fn xbc_node_get_parent(node: *mut xbc_node) -> *mut xbc_node  __init {
    struct xbc_node * __init xbc_node_get_parent(struct xbc_node *node)
    {
    return node.parent == XBC_NODE_MAX ? core::ptr::null_mut() : &xbc_nodes[node.parent];
    }
//
// xbc_node_get_child() - Get the child XBC node
// @node: An XBC node.
//
// Return the first child node of @node. If the node has no child, return
// NULL.
//
#[no_mangle]
pub unsafe extern "C" fn xbc_node_get_child(node: *mut xbc_node) -> *mut xbc_node  __init {
    struct xbc_node * __init xbc_node_get_child(struct xbc_node *node)
    {
    return node.child ? &xbc_nodes[node.child] : core::ptr::null_mut();
    }
//
// xbc_node_get_next() - Get the next sibling XBC node
// @node: An XBC node.
//
// Return the NEXT sibling node of @node. If the node has no next sibling,
// return NULL. Note that even if this returns NULL, it doesn't mean @node
// has no siblings. (You also has to check whether the parent's child node
// is @node or not.)
//
#[no_mangle]
pub unsafe extern "C" fn xbc_node_get_next(node: *mut xbc_node) -> *mut xbc_node  __init {
    struct xbc_node * __init xbc_node_get_next(struct xbc_node *node)
    {
    return node.next ? &xbc_nodes[node.next] : core::ptr::null_mut();
    }
//
// xbc_node_get_data() - Get the data of XBC node
// @node: An XBC node.
//
// Return the data (which is always a null terminated string) of @node.
// If the node has invalid data, warn and return NULL.
//
#[no_mangle]
pub unsafe extern "C" fn xbc_node_get_data(node: *mut xbc_node) -> *const char  __init {
    const char * __init xbc_node_get_data(struct xbc_node *node)
    {
    let mut offset: usize = node.data & ~XBC_VALUE;
    if (WARN_ON(offset >= xbc_data_size))
    return core::ptr::null_mut();
    return xbc_data + offset;
    }
    static bool __init
    xbc_node_match_prefix(struct xbc_node *node, const char **prefix)
    {
    const char *p = xbc_node_get_data(node);
    let mut len: usize = strlen(p);
    if (strncmp(*prefix, p, len))
    return false;
    p = *prefix + len;
    if (*p == '.')
    p++;
#[no_mangle]
pub unsafe extern "C" fn if('\0': *mut *mut p !=) -> else {
    else if (*p != '\0')
    return false;
// prefix = p;
    return true;
    }
//
// xbc_node_find_subkey() - Find a subkey node which matches given key
// @parent: An XBC node.
// @key: A key string.
//
// Search a key node under @parent which matches @key. The @key can contain
// several words jointed with '.'. If @parent is NULL, this searches the
// node from whole tree. Return NULL if no node is matched.
//
    struct xbc_node * __init
    xbc_node_find_subkey(struct xbc_node *parent, const char *key)
    {
    struct xbc_node *node;
    if (parent)
    node = xbc_node_get_subkey(parent);
    else
    node = xbc_root_node();
    while (node && xbc_node_is_key(node)) {
    if (!xbc_node_match_prefix(node, &key))
    node = xbc_node_get_next(node);
#[no_mangle]
pub unsafe extern "C" fn if('\0': *mut *mut key !=) -> else {
    else if (*key != '\0')
    node = xbc_node_get_subkey(node);
    else
    break;
    }
    return node;
    }
//
// xbc_node_find_value() - Find a value node which matches given key
// @parent: An XBC node.
// @key: A key string.
// @vnode: A container pointer of found XBC node.
//
// Search a value node under @parent whose (parent) key node matches @key,
// store it in *@vnode, and returns the value string.
// The @key can contain several words jointed with '.'. If @parent is NULL,
// this searches the node from whole tree. Return the value string if a
// matched key found, return NULL if no node is matched.
// Note that this returns 0-length string and stores NULL in *@vnode if the
// key has no value. And also it will return the value of the first entry if
// the value is an array.
//
    const char * __init
    xbc_node_find_value(struct xbc_node *parent, const char *key,
    struct xbc_node **vnode)
    {
    struct xbc_node *node = xbc_node_find_subkey(parent, key);
    if (!node || !xbc_node_is_key(node))
    return core::ptr::null_mut();
    node = xbc_node_get_child(node);
    if (node && !xbc_node_is_value(node))
    return core::ptr::null_mut();
    if (vnode)
// vnode = node;
    return node ? xbc_node_get_data(node) : "";
    }
//
// xbc_node_compose_key_after() - Compose partial key string of the XBC node
// @root: Root XBC node
// @node: Target XBC node.
// @buf: A buffer to store the key.
// @size: The size of the @buf.
//
// Compose the partial key of the @node into @buf, which is starting right
// after @root (@root is not included.) If @root is NULL, this returns full
// key words of @node.
// Returns the total length of the key stored in @buf. Returns -EINVAL
// if @node is NULL or @root is not the ancestor of @node or @root is @node,
// or returns -ERANGE if the key depth is deeper than max depth.
// This is expected to be used with xbc_find_node() to list up all (child)
// keys under given key.
//
    int __init xbc_node_compose_key_after(struct xbc_node *root,
    struct xbc_node *node,
    char *buf, size_t size)
    {
    uint16_t keys[XBC_DEPTH_MAX];
    let mut depth: c_int = 0, ret = 0, total = 0;
    if (!node || node == root)
    return -EINVAL;
    if (xbc_node_is_value(node))
    node = xbc_node_get_parent(node);
    while (node && node != root) {
    keys[depth++] = xbc_node_index(node);
    if (depth == XBC_DEPTH_MAX)
    return -ERANGE;
    node = xbc_node_get_parent(node);
    }
    if (!node && root)
    return -EINVAL;
    while (--depth >= 0) {
    node = xbc_nodes + keys[depth];
    ret = snprintf(buf, size, "%s%s", xbc_node_get_data(node),
    depth ? "." : "");
    if (ret < 0)
    return ret;
    if (ret >= size) {
    size = 0;
    } else {
    size -= ret;
    buf += ret;
    }
    total += ret;
    }
    return total;
    }
//
// xbc_node_find_next_leaf() - Find the next leaf node under given node
// @root: An XBC root node
// @node: An XBC node which starts from.
//
// Search the next leaf node (which means the terminal key node) of @node
// under @root node (including @root node itself).
// Return the next node or NULL if next leaf node is not found.
//
    struct xbc_node * __init xbc_node_find_next_leaf(struct xbc_node *root,
    struct xbc_node *node)
    {
    struct xbc_node *next;
    if (unlikely(!xbc_data))
    return core::ptr::null_mut();
    if (!node) {	/* First try */
    node = root;
    if (!node)
    node = xbc_nodes;
    } else {
// Leaf node may have a subkey
    next = xbc_node_get_subkey(node);
    if (next) {
    node = next;
    goto found;
    }
    if (node == root)	/* @root was a leaf, no child node. */
    return core::ptr::null_mut();
    while (!node.next) {
    node = xbc_node_get_parent(node);
    if (node == root)
    return core::ptr::null_mut();
// User passed a node which is not under parent
    if (WARN_ON(!node))
    return core::ptr::null_mut();
    }
    node = xbc_node_get_next(node);
    }
    found:
    while (node && !xbc_node_is_leaf(node))
    node = xbc_node_get_child(node);
    return node;
    }
//
// xbc_node_find_next_key_value() - Find the next key-value pair nodes
// @root: An XBC root node
// @leaf: A container pointer of XBC node which starts from.
//
// Search the next leaf node (which means the terminal key node) of *@leaf
// under @root node. Returns the value and update *@leaf if next leaf node
// is found, or NULL if no next leaf node is found.
// Note that this returns 0-length string if the key has no value, or
// the value of the first entry if the value is an array.
//
    const char * __init xbc_node_find_next_key_value(struct xbc_node *root,
    struct xbc_node **leaf)
    {
// tip must be passed
    if (WARN_ON(!leaf))
    return core::ptr::null_mut();
// leaf = xbc_node_find_next_leaf(root, *leaf);
    if (!*leaf)
    return core::ptr::null_mut();
    if ((*leaf).child)
    return xbc_node_get_data(xbc_node_get_child(*leaf));
    else
    return "";	/* No value key */
    }
    static char xbc_namebuf[XBC_KEYLEN_MAX] __initdata;

//
// xbc_snprint_cmdline() - Render bootconfig keys under @root as a cmdline string
// @buf: Destination buffer (may be NULL when @size is 0 to query the length)
// @size: Size of @buf in bytes
// @root: Subtree root whose key=value pairs should be rendered
//
// Walk all key/value pairs under @root and emit them as a space-separated
// cmdline string into @buf. Values containing whitespace are quoted with
// double quotes. Returns the number of bytes that would be written if @buf
// were large enough (matching snprintf semantics), or a negative errno on
// failure.
//
#[no_mangle]
pub unsafe extern "C" fn xbc_snprint_cmdline(buf: *mut c_char, size: usize, root: *mut xbc_node) -> int __init {
    int __init xbc_snprint_cmdline(char *buf, size_t size, struct xbc_node *root)
    {
    struct xbc_node *knode, *vnode;
    const char *val, *q;
    let mut len: usize = 0;
    int ret;
//
// Track the running written length rather than advancing @buf, so we
// never form "buf + size" or "buf += ret" while @buf is NULL (the
// size-probe call passes buf=NULL, size=0). NULL pointer arithmetic
// is undefined behavior and trips host UBSan / FORTIFY_SOURCE when
// this renderer runs at kernel build time. snprintf(NULL, 0, ...)
// itself is well defined and returns the would-be length.
//
    xbc_node_for_each_key_value(root, knode, val) {
//
// An empty or value-only @root (e.g. "kernel {}" or
// "kernel = x", possibly alongside "kernel.foo = bar")
// yields @root itself here. Skip it: composing a key for it
// would fail with -EINVAL, yet any real descendant keys must
// still be rendered. An entirely empty subtree then renders
// nothing and returns 0 rather than an error.
//
    if (knode == root)
    continue;
    ret = xbc_node_compose_key_after(root, knode,
    xbc_namebuf, XBC_KEYLEN_MAX);
    if (ret < 0)
    return ret;
    vnode = xbc_node_get_child(knode);
    if (!vnode) {
    ret = snprintf(buf ? buf + len : core::ptr::null_mut(), rest(len, size),
    "%s ", xbc_namebuf);
    if (ret < 0)
    return ret;
    len += ret;
    continue;
    }
    xbc_array_for_each_value(vnode, val) {
//
// For prettier and more readable /proc/cmdline, only
// quote the value when necessary, i.e. when it contains
// whitespace.
//
    q = strpbrk(val, " \t\r\n") ? "\"" : "";
    ret = snprintf(buf ? buf + len : core::ptr::null_mut(), rest(len, size),
    "%s=%s%s%s ", xbc_namebuf, q, val, q);
    if (ret < 0)
    return ret;
    len += ret;
    }
    }
    return len;
    }

// XBC parse and tree build
#[no_mangle]
unsafe extern "C" fn xbc_init_node(node: *mut xbc_node, data: *mut c_char, flag: u16) -> int __init {
    static int __init xbc_init_node(struct xbc_node *node, char *data, uint16_t flag)
    {
    let mut offset: c_long = data - xbc_data;
    if (WARN_ON(offset < 0 || offset >= XBC_DATA_MAX))
    return -EINVAL;
    node.data = (uint16_t)offset | flag;
    node.child = 0;
    node.next = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xbc_add_node(data: *mut c_char, flag: u16) -> *mut xbc_node  __init {
    static struct xbc_node * __init xbc_add_node(char *data, uint16_t flag)
    {
    struct xbc_node *node;
    if (xbc_node_num == XBC_NODE_MAX)
    return core::ptr::null_mut();
    node = &xbc_nodes[xbc_node_num];
    if (xbc_init_node(node, data, flag) < 0)
    return core::ptr::null_mut();
    xbc_node_num++;
    return node;
    }
    static inline __init struct xbc_node *xbc_last_sibling(struct xbc_node *node)
    {
    while (node.next)
    node = xbc_node_get_next(node);
    return node;
    }
    static inline __init struct xbc_node *xbc_last_child(struct xbc_node *node)
    {
    while (node.child)
    node = xbc_node_get_child(node);
    return node;
    }
#[no_mangle]
unsafe extern "C" fn __xbc_add_sibling(data: *mut c_char, flag: u16, head: bool) -> *mut xbc_node  __init {
    static struct xbc_node * __init __xbc_add_sibling(char *data, uint16_t flag, bool head)
    {
    struct xbc_node *sib, *node = xbc_add_node(data, flag);
    if (node) {
    if (!last_parent) {
// Ignore @head in this case
    node.parent = XBC_NODE_MAX;
    sib = xbc_last_sibling(xbc_nodes);
    sib.next = xbc_node_index(node);
    } else {
    node.parent = xbc_node_index(last_parent);
    if (!last_parent.child || head) {
    node.next = last_parent.child;
    last_parent.child = xbc_node_index(node);
    } else {
    sib = xbc_node_get_child(last_parent);
    sib = xbc_last_sibling(sib);
    sib.next = xbc_node_index(node);
    }
    }
    } else {
    xbc_parse_error("Too many nodes", data);
    }
    return node;
    }
#[no_mangle]
pub unsafe extern "C" fn xbc_add_sibling(data: *mut c_char, flag: u16) -> *mut xbc_node  __init {
    static inline struct xbc_node * __init xbc_add_sibling(char *data, uint16_t flag)
    {
    return __xbc_add_sibling(data, flag, false);
    }
#[no_mangle]
pub unsafe extern "C" fn xbc_add_head_sibling(data: *mut c_char, flag: u16) -> *mut xbc_node  __init {
    static inline struct xbc_node * __init xbc_add_head_sibling(char *data, uint16_t flag)
    {
    return __xbc_add_sibling(data, flag, true);
    }
    static inline __init struct xbc_node *xbc_add_child(char *data, uint16_t flag)
    {
    struct xbc_node *node = xbc_add_sibling(data, flag);
    if (node)
    last_parent = node;
    return node;
    }
#[no_mangle]
pub unsafe extern "C" fn xbc_valid_keyword(key: *mut c_char) -> __init bool {
    static inline __init bool xbc_valid_keyword(char *key)
    {
    if (key[0] == '\0')
    return false;
    while (isalnum(*key) || *key == '-' || *key == '_')
    key++;
    return *key == '\0';
    }
    static char *skip_comment(char *p)
    {
    char *ret;
    ret = strchr(p, '\n');
    if (!ret)
    ret = p + strlen(p);
    else
    ret++;
    return ret;
    }
    static char *skip_spaces_until_newline(char *p)
    {
    while (isspace(*p) && *p != '\n')
    p++;
    return p;
    }
#[no_mangle]
unsafe extern "C" fn __xbc_open_brace(p: *mut c_char) -> int __init {
    static int __init __xbc_open_brace(char *p)
    {
// Push the last key as open brace
    if (brace_index >= XBC_DEPTH_MAX)
    return xbc_parse_error("Exceed max depth of braces", p);
    open_brace[brace_index++] = xbc_node_index(last_parent);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __xbc_close_brace(p: *mut c_char) -> int __init {
    static int __init __xbc_close_brace(char *p)
    {
    brace_index--;
    if (!last_parent || brace_index < 0 ||
    (open_brace[brace_index] != xbc_node_index(last_parent)))
    return xbc_parse_error("Unexpected closing brace", p);
    if (brace_index == 0)
    last_parent = core::ptr::null_mut();
    else
    last_parent = &xbc_nodes[open_brace[brace_index - 1]];
    return 0;
    }
//
// Return delimiter or error, no node added. As same as lib/cmdline.c,
// you can use " around spaces, but can't escape " for value.
// *@__v must point real value string. (not including spaces before value.)
//
#[no_mangle]
unsafe extern "C" fn __xbc_parse_value(__v: *mut c_char, __n: *mut c_char) -> int __init {
    static int __init __xbc_parse_value(char **__v, char **__n)
    {
    char *p, *v = *__v;
    int c, quotes = 0;
    if (*v == '"' || *v == '\'') {
    quotes = *v;
    v++;
    }
    p = v - 1;
    while ((c = *++p)) {
    if (!isprint(c) && !isspace(c))
    return xbc_parse_error("Non printable value", p);
    if (quotes) {
    if (c != quotes)
    continue;
    quotes = 0;
// p++ = '\0';
    p = skip_spaces_until_newline(p);
    c = *p;
    if (c && !strchr(",;\n#}", c))
    return xbc_parse_error("No value delimiter", p);
    if (*p)
    p++;
    break;
    }
    if (strchr(",;\n#}", c)) {
// p++ = '\0';
    v = strim(v);
    break;
    }
    }
    if (quotes)
    return xbc_parse_error("No closing quotes", p);
    if (c == '#') {
    p = skip_comment(p);
    c = '\n';	/* A comment must be treated as a newline */
    }
// __n = p;
// __v = v;
    return c;
    }
#[no_mangle]
unsafe extern "C" fn xbc_parse_array(__v: *mut c_char) -> int __init {
    static int __init xbc_parse_array(char **__v)
    {
    struct xbc_node *node;
    char *next;
    let mut c: c_int = 0;
    if (last_parent.child)
    last_parent = xbc_node_get_child(last_parent);
    do {
// Search the next array value beyond comments and empty lines
    next = skip_spaces(*__v);
    while (*next == '#') {
    next = skip_comment(next);
    next = skip_spaces(next);
    }
// __v = next;
    c = __xbc_parse_value(__v, &next);
    if (c < 0)
    return c;
    node = xbc_add_child(*__v, XBC_VALUE);
    if (!node)
    return -ENOMEM;
// __v = next;
    } while (c == ',');
    node.child = 0;
    return c;
    }
    static inline __init
    struct xbc_node *find_match_node(struct xbc_node *node, char *k)
    {
    while (node) {
    if (!strcmp(xbc_node_get_data(node), k))
    break;
    node = xbc_node_get_next(node);
    }
    return node;
    }
#[no_mangle]
unsafe extern "C" fn __xbc_add_key(k: *mut c_char) -> int __init {
    static int __init __xbc_add_key(char *k)
    {
    struct xbc_node *node, *child;
    if (!xbc_valid_keyword(k))
    return xbc_parse_error("Invalid keyword", k);
    if (unlikely(xbc_node_num == 0))
    goto add_node;
    if (!last_parent) {	/* the first level */
    node = find_match_node(xbc_nodes, k);
    } else {
    child = xbc_node_get_child(last_parent);
// Since the value node is the first child, skip it.
    if (child && xbc_node_is_value(child))
    child = xbc_node_get_next(child);
    node = find_match_node(child, k);
    }
    if (node) {
    last_parent = node;
    } else {
    add_node:
    node = xbc_add_child(k, XBC_KEY);
    if (!node)
    return -ENOMEM;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __xbc_parse_keys(k: *mut c_char) -> int __init {
    static int __init __xbc_parse_keys(char *k)
    {
    char *p;
    int ret;
    k = strim(k);
    while ((p = strchr(k, '.'))) {
// p++ = '\0';
    ret = __xbc_add_key(k);
    if (ret)
    return ret;
    k = p;
    }
    return __xbc_add_key(k);
    }
#[no_mangle]
unsafe extern "C" fn xbc_parse_kv(k: *mut c_char, v: *mut c_char, op: c_int) -> int __init {
    static int __init xbc_parse_kv(char **k, char *v, int op)
    {
    struct xbc_node *prev_parent = last_parent;
    struct xbc_node *child;
    char *next;
    int c, ret;
    ret = __xbc_parse_keys(*k);
    if (ret)
    return ret;
    v = skip_spaces_until_newline(v);
// If there is a comment, this has an empty value.
    if (*v == '#') {
    next = skip_comment(v);
// v = '\0';
    c = '\n';
    } else {
    c = __xbc_parse_value(&v, &next);
    if (c < 0)
    return c;
    }
    child = xbc_node_get_child(last_parent);
    if (child && xbc_node_is_value(child)) {
    if (op == '=')
    return xbc_parse_error("Value is redefined", v);
    if (op == ':') {
    let mut nidx: c_ushort = child.next;
    if (xbc_init_node(child, v, XBC_VALUE) < 0)
    return xbc_parse_error("Failed to override value", v);
    child.next = nidx;	/* keep subkeys */
    goto array;
    }
// op must be '+'
    last_parent = xbc_last_child(child);
    }
// The value node should always be the first child
    if (!xbc_add_head_sibling(v, XBC_VALUE))
    return -ENOMEM;
    array:
    if (c == ',') {	/* Array */
    c = xbc_parse_array(&next);
    if (c < 0)
    return c;
    }
    last_parent = prev_parent;
    if (c == '}') {
    ret = __xbc_close_brace(next - 1);
    if (ret < 0)
    return ret;
    }
// k = next;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xbc_parse_key(k: *mut c_char, n: *mut c_char) -> int __init {
    static int __init xbc_parse_key(char **k, char *n)
    {
    struct xbc_node *prev_parent = last_parent;
    int ret;
// k = strim(*k);
    if (**k != '\0') {
    ret = __xbc_parse_keys(*k);
    if (ret)
    return ret;
    last_parent = prev_parent;
    }
// k = n;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xbc_open_brace(k: *mut c_char, n: *mut c_char) -> int __init {
    static int __init xbc_open_brace(char **k, char *n)
    {
    int ret;
    ret = __xbc_parse_keys(*k);
    if (ret)
    return ret;
// k = n;
    return __xbc_open_brace(n - 1);
    }
#[no_mangle]
unsafe extern "C" fn xbc_close_brace(k: *mut c_char, n: *mut c_char) -> int __init {
    static int __init xbc_close_brace(char **k, char *n)
    {
    int ret;
    ret = xbc_parse_key(k, n);
    if (ret)
    return ret;
// k is updated in xbc_parse_key()
    return __xbc_close_brace(n - 1);
    }
#[no_mangle]
unsafe extern "C" fn xbc_verify_tree() -> int __init {
    static int __init xbc_verify_tree(void)
    {
    int i, depth;
    size_t len, wlen;
    struct xbc_node *n, *m;
// Brace closing
    if (brace_index) {
    n = &xbc_nodes[open_brace[brace_index - 1]];
    return xbc_parse_error("Brace is not closed",
    xbc_node_get_data(n));
    }
// Empty tree
    if (xbc_node_num == 0) {
    xbc_parse_error("Empty config", xbc_data);
    return -ENOENT;
    }
    for (i = 0; i < xbc_node_num; i++) {
    if (xbc_nodes[i].next >= xbc_node_num) {
    return xbc_parse_error("No closing brace",
    xbc_node_get_data(xbc_nodes + i));
    }
    if (xbc_nodes[i].child >= xbc_node_num) {
    return xbc_parse_error("Broken child node",
    xbc_node_get_data(xbc_nodes + i));
    }
    }
// Key tree limitation check
    n = &xbc_nodes[0];
    depth = 1;
    len = 0;
    while (n) {
    wlen = strlen(xbc_node_get_data(n)) + 1;
    len += wlen;
    if (len > XBC_KEYLEN_MAX)
    return xbc_parse_error("Too long key length",
    xbc_node_get_data(n));
    m = xbc_node_get_child(n);
    if (m && xbc_node_is_key(m)) {
    n = m;
    depth++;
    if (depth > XBC_DEPTH_MAX)
    return xbc_parse_error("Too many key words",
    xbc_node_get_data(n));
    continue;
    }
    len -= wlen;
    m = xbc_node_get_next(n);
    while (!m) {
    n = xbc_node_get_parent(n);
    if (!n)
    break;
    len -= strlen(xbc_node_get_data(n)) + 1;
    depth--;
    m = xbc_node_get_next(n);
    }
    n = m;
    }
    return 0;
    }
// Need to setup xbc_data and xbc_nodes before call this.
#[no_mangle]
unsafe extern "C" fn xbc_parse_tree() -> int __init {
    static int __init xbc_parse_tree(void)
    {
    char *p, *q;
    let mut ret: c_int = 0, c;
    last_parent = core::ptr::null_mut();
    p = xbc_data;
    do {
    q = strpbrk(p, "{}=+;:\n#");
    if (!q) {
    p = skip_spaces(p);
    if (*p != '\0')
    ret = xbc_parse_error("No delimiter", p);
    break;
    }
    c = *q;
// q++ = '\0';
    switch (c) {
    case ':':
    case '+':
    if (*q++ != '=') {
    ret = xbc_parse_error(c == '+' ?
    "Wrong '+' operator" :
    "Wrong ':' operator",
    q - 2);
    break;
    }
    fallthrough;
    case '=':
    ret = xbc_parse_kv(&p, q, c);
    break;
    case '{':
    ret = xbc_open_brace(&p, q);
    break;
    case '#':
    q = skip_comment(q);
    fallthrough;
    case ';':
    case '\n':
    ret = xbc_parse_key(&p, q);
    break;
    case '}':
    ret = xbc_close_brace(&p, q);
    break;
    }
    } while (!ret);
    return ret;
    }
//
// _xbc_exit() - Clean up all parsed bootconfig
// @early: Set true if this is called before budy system is initialized.
//
// This clears all data structures of parsed bootconfig on memory.
// If you need to reuse xbc_init() with new boot config, you can
// use this.
//
#[no_mangle]
pub unsafe extern "C" fn _xbc_exit(early: bool) -> void __init {
    void __init _xbc_exit(bool early)
    {
    xbc_free_mem(xbc_data, xbc_data_size, early);
    xbc_data = core::ptr::null_mut();
    xbc_data_size = 0;
    xbc_node_num = 0;
    xbc_free_mem(xbc_nodes, sizeof(struct xbc_node) * XBC_NODE_MAX, early);
    xbc_nodes = core::ptr::null_mut();
    brace_index = 0;
    }
//
// xbc_init() - Parse given XBC file and build XBC internal tree
// @data: The boot config text original data
// @size: The size of @data
// @emsg: A pointer of const char * to store the error message
// @epos: A pointer of int to store the error position
//
// This parses the boot config text in @data. @size must be smaller
// than XBC_DATA_MAX.
// Return the number of stored nodes (>0) if succeeded, or -errno
// if there is any error.
// In error cases, @emsg will be updated with an error message and
// @epos will be updated with the error position which is the byte offset
// of @buf. If the error is not a parser error, @epos will be -1.
//
#[no_mangle]
pub unsafe extern "C" fn xbc_init(data: *const c_char, size: usize, emsg: *const c_char, epos: *mut c_int) -> int __init {
    int __init xbc_init(const char *data, size_t size, const char **emsg, int *epos)
    {
    int ret;
    if (epos)
// epos = -1;
    if (xbc_data) {
    if (emsg)
// emsg = "Bootconfig is already initialized";
    return -EBUSY;
    }
    if (size > XBC_DATA_MAX || size == 0) {
    if (emsg)
// emsg = size ? "Config data is too big" :
    "Config data is empty";
    return -ERANGE;
    }
    xbc_data = xbc_alloc_mem(size + 1);
    if (!xbc_data) {
    if (emsg)
// emsg = "Failed to allocate bootconfig data";
    return -ENOMEM;
    }
    memcpy(xbc_data, data, size);
    xbc_data[size] = '\0';
    xbc_data_size = size + 1;
    xbc_nodes = xbc_alloc_mem(sizeof(struct xbc_node) * XBC_NODE_MAX);
    if (!xbc_nodes) {
    if (emsg)
// emsg = "Failed to allocate bootconfig nodes";
    _xbc_exit(true);
    return -ENOMEM;
    }
    ret = xbc_parse_tree();
    if (!ret)
    ret = xbc_verify_tree();
    if (ret < 0) {
    if (epos)
// epos = xbc_err_pos;
    if (emsg)
// emsg = xbc_err_msg;
    _xbc_exit(true);
    } else {
    ret = xbc_node_num;
    }
    return ret;
    }
