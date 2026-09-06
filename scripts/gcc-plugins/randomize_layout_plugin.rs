//! Automatically rewritten from C to Rust
//! Source: scripts/gcc-plugins/randomize_layout_plugin.c
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


//
// Copyright 2014-2016 by Open Source Security, Inc., Brad Spengler <spender@grsecurity.net>
// and PaX Team <pageexec@freemail.hu>
// Licensed under the GPL v2
//
// Note: the choice of the license means that the compilation process is
// NOT 'eligible' as defined by gcc's library exception to the GPL v3,
// but for the kernel it doesn't matter since it doesn't link against
// any of the gcc libraries
//
// Usage:
// $ # for 4.5/4.6/C based 4.7
// $ gcc -I`gcc -print-file-name=plugin`/include -I`gcc -print-file-name=plugin`/include/c-family -fPIC -shared -O2 -o randomize_layout_plugin.so randomize_layout_plugin.c
// $ # for C++ based 4.7/4.8+
// $ g++ -I`g++ -print-file-name=plugin`/include -I`g++ -print-file-name=plugin`/include/c-family -fPIC -shared -O2 -o randomize_layout_plugin.so randomize_layout_plugin.c
// $ gcc -fplugin=./randomize_layout_plugin.so test.c -O2
//

    (TYPE_NAME(TYPE_MAIN_VARIANT(node)) != NULL_TREE ? ((const unsigned char *)IDENTIFIER_POINTER(TYPE_NAME(TYPE_MAIN_VARIANT(node)))) : (const unsigned char *)"anonymous")

    __visible int plugin_is_GPL_compatible;
    static int performance_mode;
    static struct plugin_info randomize_layout_plugin_info = {
    .version	= PLUGIN_VERSION,
    .help		= "disable\t\t\tdo not activate plugin\n"
    "performance-mode\tenable cacheline-aware layout randomization\n"
    };
// from old Linux dcache.h
    static inline unsigned long
    partial_name_hash(unsigned long c, unsigned long prevhash)
    {
    return (prevhash + (c << 4) + (c >> 4)) * 11;
    }
    static inline unsigned int
    name_hash(const unsigned char *name)
    {
    let mut hash: c_ulong = 0;
    let mut len: c_uint = strlen((const char *)name);
    while (len--)
    hash = partial_name_hash(*name++, hash);
    return (unsigned int)hash;
    }
#[no_mangle]
unsafe extern "C" fn handle_randomize_layout_attr(node: *mut tree, name: tree, args: tree, flags: c_int, no_add_attrs: *mut bool) -> tree {
    static tree handle_randomize_layout_attr(tree *node, tree name, tree args, int flags, bool *no_add_attrs)
    {
    tree type;
// no_add_attrs = true;
    if (TREE_CODE(*node) == FUNCTION_DECL) {
    error("%qE attribute does not apply to functions (%qF)", name, *node);
    return NULL_TREE;
    }
    if (TREE_CODE(*node) == PARM_DECL) {
    error("%qE attribute does not apply to function parameters (%qD)", name, *node);
    return NULL_TREE;
    }
    if (TREE_CODE(*node) == VAR_DECL) {
    error("%qE attribute does not apply to variables (%qD)", name, *node);
    return NULL_TREE;
    }
    if (TYPE_P(*node)) {
    type = *node;
    } else if (TREE_CODE(*node) == FIELD_DECL) {
// no_add_attrs = false;
    return NULL_TREE;
    } else {
    gcc_assert(TREE_CODE(*node) == TYPE_DECL);
    type = TREE_TYPE(*node);
    }
    if (TREE_CODE(type) != RECORD_TYPE) {
    error("%qE attribute used on %qT applies to struct types only", name, type);
    return NULL_TREE;
    }
    if (lookup_attribute(IDENTIFIER_POINTER(name), TYPE_ATTRIBUTES(type))) {
    error("%qE attribute is already applied to the type %qT", name, type);
    return NULL_TREE;
    }
// no_add_attrs = false;
    return NULL_TREE;
    }
// set on complete types that we don't need to inspect further at all
#[no_mangle]
unsafe extern "C" fn handle_randomize_considered_attr(node: *mut tree, name: tree, args: tree, flags: c_int, no_add_attrs: *mut bool) -> tree {
    static tree handle_randomize_considered_attr(tree *node, tree name, tree args, int flags, bool *no_add_attrs)
    {
// no_add_attrs = false;
    return NULL_TREE;
    }
//
// set on types that we've performed a shuffle on, to prevent re-shuffling
// this does not preclude us from inspecting its fields for potential shuffles
//
#[no_mangle]
unsafe extern "C" fn handle_randomize_performed_attr(node: *mut tree, name: tree, args: tree, flags: c_int, no_add_attrs: *mut bool) -> tree {
    static tree handle_randomize_performed_attr(tree *node, tree name, tree args, int flags, bool *no_add_attrs)
    {
// no_add_attrs = false;
    return NULL_TREE;
    }
//
// 64bit variant of Bob Jenkins' public domain PRNG
// 256 bits of internal state
//
    typedef unsigned long long u64;
    typedef struct ranctx { u64 a; u64 b; u64 c; u64 d; } ranctx;

#[no_mangle]
unsafe extern "C" fn ranval(x: *mut ranctx) -> u64 {
    let mut e: u64 = x.a - rot(x.b, 7);
    x.a = x.b ^ rot(x.c, 13);
    x.b = x.c + rot(x.d, 37);
    x.c = x.d + e;
    x.d = e + x.a;
    return x.d;
    }
#[no_mangle]
unsafe extern "C" fn raninit(x: *mut ranctx, seed: *mut u64) {
    int i;
    x.a = seed[0];
    x.b = seed[1];
    x.c = seed[2];
    x.d = seed[3];
    for (i=0; i < 30; ++i)
    (void)ranval(x);
    }
    static u64 shuffle_seed[4];
#[repr(C)]
#[derive(Copy, Clone)]
pub struct partition_group {
    pub tree_start: tree,
    pub start: c_ulong,
    pub length: c_ulong,
}

#[no_mangle]
unsafe extern "C" fn partition_struct(fields: *mut tree, length: c_ulong, size_groups: *mut partition_group, num_groups: *mut c_ulong) {
    static void partition_struct(tree *fields, unsigned long length, struct partition_group *size_groups, unsigned long *num_groups)
    {
    unsigned long i;
    let mut accum_size: c_ulong = 0;
    let mut accum_length: c_ulong = 0;
    let mut group_idx: c_ulong = 0;
    gcc_assert(length < INT_MAX);
    memset(size_groups, 0, sizeof(struct partition_group) * length);
    for (i = 0; i < length; i++) {
    if (size_groups[group_idx].tree_start == NULL_TREE) {
    size_groups[group_idx].tree_start = fields[i];
    size_groups[group_idx].start = i;
    accum_length = 0;
    accum_size = 0;
    }
    accum_size += (unsigned long)int_size_in_bytes(TREE_TYPE(fields[i]));
    accum_length++;
    if (accum_size >= 64) {
    size_groups[group_idx].length = accum_length;
    accum_length = 0;
    group_idx++;
    }
    }
    if (size_groups[group_idx].tree_start != NULL_TREE &&
    !size_groups[group_idx].length) {
    size_groups[group_idx].length = accum_length;
    group_idx++;
    }
// num_groups = group_idx;
    }
#[no_mangle]
unsafe extern "C" fn performance_shuffle(newtree: *mut tree, length: c_ulong, prng_state: *mut ranctx) {
    static void performance_shuffle(tree *newtree, unsigned long length, ranctx *prng_state)
    {
    unsigned long i, x, index;
    struct partition_group size_group[length];
    let mut num_groups: c_ulong = 0;
    unsigned long randnum;
    partition_struct(newtree, length, (struct partition_group *)&size_group, &num_groups);
// FIXME: this group shuffle is currently a no-op.
    for (i = num_groups - 1; i > 0; i--) {
    struct partition_group tmp;
    randnum = ranval(prng_state) % (i + 1);
    tmp = size_group[i];
    size_group[i] = size_group[randnum];
    size_group[randnum] = tmp;
    }
    for (x = 0; x < num_groups; x++) {
    for (index = size_group[x].length - 1; index > 0; index--) {
    tree tmp;
    i = size_group[x].start + index;
    if (DECL_BIT_FIELD_TYPE(newtree[i]))
    continue;
    randnum = ranval(prng_state) % (index + 1);
    randnum += size_group[x].start;
// we could handle this case differently if desired
    if (DECL_BIT_FIELD_TYPE(newtree[randnum]))
    continue;
    tmp = newtree[i];
    newtree[i] = newtree[randnum];
    newtree[randnum] = tmp;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn full_shuffle(newtree: *mut tree, length: c_ulong, prng_state: *mut ranctx) {
    static void full_shuffle(tree *newtree, unsigned long length, ranctx *prng_state)
    {
    unsigned long i, randnum;
    for (i = length - 1; i > 0; i--) {
    tree tmp;
    randnum = ranval(prng_state) % (i + 1);
    tmp = newtree[i];
    newtree[i] = newtree[randnum];
    newtree[randnum] = tmp;
    }
    }
// modern in-place Fisher-Yates shuffle
#[no_mangle]
unsafe extern "C" fn shuffle(type: const_tree, newtree: *mut tree, length: c_ulong) {
    static void shuffle(const_tree type, tree *newtree, unsigned long length)
    {
    unsigned long i;
    u64 seed[4];
    ranctx prng_state;
    const unsigned char *structname;
    if (length == 0)
    return;
    gcc_assert(TREE_CODE(type) == RECORD_TYPE);
    structname = ORIG_TYPE_NAME(type);

    fprintf(stderr, "Shuffling struct %s %p\n", (const char *)structname, type);

    debug_tree((tree)type);

    for (i = 0; i < 4; i++) {
    seed[i] = shuffle_seed[i];
    seed[i] ^= name_hash(structname);
    }
    raninit(&prng_state, (u64 *)&seed);
    if (performance_mode)
    performance_shuffle(newtree, length, &prng_state);
    else
    full_shuffle(newtree, length, &prng_state);
    }
#[no_mangle]
unsafe extern "C" fn is_flexible_array(field: const_tree) -> bool {
    static bool is_flexible_array(const_tree field)
    {
    const_tree fieldtype;
    const_tree typesize;
    fieldtype = TREE_TYPE(field);
    typesize = TYPE_SIZE(fieldtype);
    if (TREE_CODE(fieldtype) != ARRAY_TYPE)
    return false;
// size of type is represented in bits
    if (typesize == NULL_TREE && TYPE_DOMAIN(fieldtype) != NULL_TREE &&
    TYPE_MAX_VALUE(TYPE_DOMAIN(fieldtype)) == NULL_TREE)
    return true;
    return false;
    }
#[no_mangle]
unsafe extern "C" fn relayout_struct(type: tree) -> c_int {
    static int relayout_struct(tree type)
    {
    let mut num_fields: c_ulong = (unsigned long)list_length(TYPE_FIELDS(type));
    let mut shuffle_length: c_ulong = num_fields;
    tree field;
    tree newtree[num_fields];
    unsigned long i;
    tree list;
    tree variant;
    tree main_variant;
    expanded_location xloc;
    let mut has_flexarray: bool = false;
    if (TYPE_FIELDS(type) == NULL_TREE)
    return 0;
    if (num_fields < 2)
    return 0;
    gcc_assert(TREE_CODE(type) == RECORD_TYPE);
    gcc_assert(num_fields < INT_MAX);
    if (lookup_attribute("randomize_performed", TYPE_ATTRIBUTES(type)) ||
    lookup_attribute("no_randomize_layout", TYPE_ATTRIBUTES(TYPE_MAIN_VARIANT(type))))
    return 0;
// Workaround for 3rd-party VirtualBox source that we can't modify ourselves
    if (!strcmp((const char *)ORIG_TYPE_NAME(type), "INTNETTRUNKFACTORY") ||
    !strcmp((const char *)ORIG_TYPE_NAME(type), "RAWPCIFACTORY"))
    return 0;
// throw out any structs in uapi
    xloc = expand_location(DECL_SOURCE_LOCATION(TYPE_FIELDS(type)));
    if (strstr(xloc.file, "/uapi/"))
    error(G_("attempted to randomize userland API struct %s"), ORIG_TYPE_NAME(type));
    for (field = TYPE_FIELDS(type), i = 0; field; field = TREE_CHAIN(field), i++) {
    gcc_assert(TREE_CODE(field) == FIELD_DECL);
    newtree[i] = field;
    }
//
// enforce that we don't randomize the layout of the last
// element of a struct if it's a proper flexible array
//
    if (is_flexible_array(newtree[num_fields - 1])) {
    has_flexarray = true;
    shuffle_length--;
    }
    shuffle(type, (tree *)newtree, shuffle_length);
    for (i = 0; i < num_fields - 1; i++)
    TREE_CHAIN(newtree[i]) = newtree[i+1];
    TREE_CHAIN(newtree[num_fields - 1]) = NULL_TREE;
    add_type_attr(type, "randomize_performed", NULL_TREE);
    add_type_attr(type, "designated_init", NULL_TREE);
    if (has_flexarray)
    add_type_attr(type, "has_flexarray", NULL_TREE);
    main_variant = TYPE_MAIN_VARIANT(type);
    for (variant = main_variant; variant; variant = TYPE_NEXT_VARIANT(variant))
    TYPE_FIELDS(variant) = newtree[0];
//
// force a re-layout of the main variant
// the TYPE_SIZE for all variants will be recomputed
// by finalize_type_size()
//
    TYPE_SIZE(main_variant) = NULL_TREE;
    layout_type(main_variant);
    gcc_assert(TYPE_SIZE(main_variant) != NULL_TREE);
    return 1;
    }
// from constify plugin
#[no_mangle]
unsafe extern "C" fn get_field_type(field: const_tree) -> const_tree {
    static const_tree get_field_type(const_tree field)
    {
    return strip_array_types(TREE_TYPE(field));
    }
// from constify plugin
#[no_mangle]
unsafe extern "C" fn is_fptr(fieldtype: const_tree) -> bool {
    static bool is_fptr(const_tree fieldtype)
    {
    if (TREE_CODE(fieldtype) != POINTER_TYPE)
    return false;
    return TREE_CODE(TREE_TYPE(fieldtype)) == FUNCTION_TYPE;
    }
// derived from constify plugin
#[no_mangle]
unsafe extern "C" fn is_pure_ops_struct(node: const_tree) -> c_int {
    static int is_pure_ops_struct(const_tree node)
    {
    const_tree field;
    gcc_assert(TREE_CODE(node) == RECORD_TYPE || TREE_CODE(node) == UNION_TYPE);
    for (field = TYPE_FIELDS(node); field; field = TREE_CHAIN(field)) {
    let mut fieldtype: const_tree = get_field_type(field);
    let mut code: enum tree_code = TREE_CODE(fieldtype);
    if (node == fieldtype)
    continue;
    if (code == RECORD_TYPE || code == UNION_TYPE) {
    if (!is_pure_ops_struct(fieldtype))
    return 0;
    continue;
    }
    if (!is_fptr(fieldtype))
    return 0;
    }
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn randomize_type(type: tree) {
    static void randomize_type(tree type)
    {
    tree variant;
    gcc_assert(TREE_CODE(type) == RECORD_TYPE);
    if (lookup_attribute("randomize_considered", TYPE_ATTRIBUTES(type)))
    return;
    if (lookup_attribute("randomize_layout", TYPE_ATTRIBUTES(TYPE_MAIN_VARIANT(type))) || is_pure_ops_struct(type))
    relayout_struct(type);
    add_type_attr(type, "randomize_considered", NULL_TREE);

    fprintf(stderr, "Marking randomize_considered on struct %s\n", ORIG_TYPE_NAME(type));

    debug_tree(type);

    }
#[no_mangle]
unsafe extern "C" fn update_decl_size(decl: tree) {
    static void update_decl_size(tree decl)
    {
    tree lastval, lastidx, field, init, type, flexsize;
    unsigned HOST_WIDE_INT len;
    type = TREE_TYPE(decl);
    if (!lookup_attribute("has_flexarray", TYPE_ATTRIBUTES(type)))
    return;
    init = DECL_INITIAL(decl);
    if (init == NULL_TREE || init == error_mark_node)
    return;
    if (TREE_CODE(init) != CONSTRUCTOR)
    return;
    len = CONSTRUCTOR_NELTS(init);
    if (!len)
    return;
    lastval = CONSTRUCTOR_ELT(init, CONSTRUCTOR_NELTS(init) - 1).value;
    lastidx = CONSTRUCTOR_ELT(init, CONSTRUCTOR_NELTS(init) - 1).index;
    for (field = TYPE_FIELDS(TREE_TYPE(decl)); TREE_CHAIN(field); field = TREE_CHAIN(field))
    ;
    if (lastidx != field)
    return;
    if (TREE_CODE(lastval) != STRING_CST) {
    error("Only string constants are supported as initializers "
    "for randomized structures with flexible arrays");
    return;
    }
    flexsize = bitsize_int(TREE_STRING_LENGTH(lastval) *
    tree_to_uhwi(TYPE_SIZE(TREE_TYPE(TREE_TYPE(lastval)))));
    DECL_SIZE(decl) = size_binop(PLUS_EXPR, TYPE_SIZE(type), flexsize);
    return;
    }
#[no_mangle]
unsafe extern "C" fn randomize_layout_finish_decl(event_data: *mut c_void, data: *mut c_void) {
    static void randomize_layout_finish_decl(void *event_data, void *data)
    {
    let mut decl: tree = (tree)event_data;
    tree type;
    if (decl == NULL_TREE || decl == error_mark_node)
    return;
    type = TREE_TYPE(decl);
    if (TREE_CODE(decl) != VAR_DECL)
    return;
    if (TREE_CODE(type) != RECORD_TYPE && TREE_CODE(type) != UNION_TYPE)
    return;
    if (!lookup_attribute("randomize_performed", TYPE_ATTRIBUTES(type)))
    return;
    DECL_SIZE(decl) = 0;
    DECL_SIZE_UNIT(decl) = 0;
    SET_DECL_ALIGN(decl, 0);
    SET_DECL_MODE (decl, VOIDmode);
    SET_DECL_RTL(decl, 0);
    update_decl_size(decl);
    layout_decl(decl, 0);
    }
#[no_mangle]
unsafe extern "C" fn finish_type(event_data: *mut c_void, data: *mut c_void) {
    static void finish_type(void *event_data, void *data)
    {
    let mut type: tree = (tree)event_data;
    if (type == NULL_TREE || type == error_mark_node)
    return;
    if (TREE_CODE(type) != RECORD_TYPE)
    return;
    if (TYPE_FIELDS(type) == NULL_TREE)
    return;
    if (lookup_attribute("randomize_considered", TYPE_ATTRIBUTES(type)))
    return;

    fprintf(stderr, "Calling randomize_type on %s\n", ORIG_TYPE_NAME(type));

    debug_tree(type);

    randomize_type(type);
    return;
    }
    let mut randomize_layout_attr: static struct attribute_spec = { };
    let mut no_randomize_layout_attr: static struct attribute_spec = { };
    let mut randomize_considered_attr: static struct attribute_spec = { };
    let mut randomize_performed_attr: static struct attribute_spec = { };
#[no_mangle]
unsafe extern "C" fn register_attributes(event_data: *mut c_void, data: *mut c_void) {
    static void register_attributes(void *event_data, void *data)
    {
    randomize_layout_attr.name		= "randomize_layout";
    randomize_layout_attr.type_required	= true;
    randomize_layout_attr.handler		= handle_randomize_layout_attr;
    randomize_layout_attr.affects_type_identity = true;
    no_randomize_layout_attr.name		= "no_randomize_layout";
    no_randomize_layout_attr.type_required	= true;
    no_randomize_layout_attr.handler	= handle_randomize_layout_attr;
    no_randomize_layout_attr.affects_type_identity = true;
    randomize_considered_attr.name		= "randomize_considered";
    randomize_considered_attr.type_required	= true;
    randomize_considered_attr.handler	= handle_randomize_considered_attr;
    randomize_performed_attr.name		= "randomize_performed";
    randomize_performed_attr.type_required	= true;
    randomize_performed_attr.handler	= handle_randomize_performed_attr;
    register_attribute(&randomize_layout_attr);
    register_attribute(&no_randomize_layout_attr);
    register_attribute(&randomize_considered_attr);
    register_attribute(&randomize_performed_attr);
    }
#[no_mangle]
unsafe extern "C" fn check_bad_casts_in_constructor(var: tree, init: tree) {
    static void check_bad_casts_in_constructor(tree var, tree init)
    {
    unsigned HOST_WIDE_INT idx;
    tree field, val;
    tree field_type, val_type;
    FOR_EACH_CONSTRUCTOR_ELT(CONSTRUCTOR_ELTS(init), idx, field, val) {
    if (TREE_CODE(val) == CONSTRUCTOR) {
    check_bad_casts_in_constructor(var, val);
    continue;
    }
// pipacs' plugin creates franken-arrays that differ from those produced by
    normal code which all have valid 'field' trees. work around this */
    if (field == NULL_TREE)
    continue;
    field_type = TREE_TYPE(field);
    val_type = TREE_TYPE(val);
    if (TREE_CODE(field_type) != POINTER_TYPE || TREE_CODE(val_type) != POINTER_TYPE)
    continue;
    if (field_type == val_type)
    continue;
    field_type = TYPE_MAIN_VARIANT(strip_array_types(TYPE_MAIN_VARIANT(TREE_TYPE(field_type))));
    val_type = TYPE_MAIN_VARIANT(strip_array_types(TYPE_MAIN_VARIANT(TREE_TYPE(val_type))));
    if (field_type == void_type_node)
    continue;
    if (field_type == val_type)
    continue;
    if (TREE_CODE(val_type) != RECORD_TYPE)
    continue;
    if (!lookup_attribute("randomize_performed", TYPE_ATTRIBUTES(val_type)))
    continue;
    MISMATCH(DECL_SOURCE_LOCATION(var), "constructor\n", TYPE_MAIN_VARIANT(field_type), TYPE_MAIN_VARIANT(val_type));
    }
    }
// derived from the constify plugin
#[no_mangle]
unsafe extern "C" fn check_global_variables(event_data: *mut c_void, data: *mut c_void) {
    static void check_global_variables(void *event_data, void *data)
    {
    struct varpool_node *node;
    tree init;
    FOR_EACH_VARIABLE(node) {
    let mut var: tree = NODE_DECL(node);
    init = DECL_INITIAL(var);
    if (init == NULL_TREE)
    continue;
    if (TREE_CODE(init) != CONSTRUCTOR)
    continue;
    check_bad_casts_in_constructor(var, init);
    }
    }
#[no_mangle]
unsafe extern "C" fn dominated_by_is_err(rhs: const_tree, bb: basic_block) -> bool {
    static bool dominated_by_is_err(const_tree rhs, basic_block bb)
    {
    basic_block dom;
    gimple dom_stmt;
    gimple call_stmt;
    const_tree dom_lhs;
    const_tree poss_is_err_cond;
    const_tree poss_is_err_func;
    const_tree is_err_arg;
    dom = get_immediate_dominator(CDI_DOMINATORS, bb);
    if (!dom)
    return false;
    dom_stmt = last_stmt(dom);
    if (!dom_stmt)
    return false;
    if (gimple_code(dom_stmt) != GIMPLE_COND)
    return false;
    if (gimple_cond_code(dom_stmt) != NE_EXPR)
    return false;
    if (!integer_zerop(gimple_cond_rhs(dom_stmt)))
    return false;
    poss_is_err_cond = gimple_cond_lhs(dom_stmt);
    if (TREE_CODE(poss_is_err_cond) != SSA_NAME)
    return false;
    call_stmt = SSA_NAME_DEF_STMT(poss_is_err_cond);
    if (gimple_code(call_stmt) != GIMPLE_CALL)
    return false;
    dom_lhs = gimple_get_lhs(call_stmt);
    poss_is_err_func = gimple_call_fndecl(call_stmt);
    if (!poss_is_err_func)
    return false;
    if (dom_lhs != poss_is_err_cond)
    return false;
    if (strcmp(DECL_NAME_POINTER(poss_is_err_func), "IS_ERR"))
    return false;
    is_err_arg = gimple_call_arg(call_stmt, 0);
    if (!is_err_arg)
    return false;
    if (is_err_arg != rhs)
    return false;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn handle_local_var_initializers() {
    static void handle_local_var_initializers(void)
    {
    tree var;
    unsigned int i;
    FOR_EACH_LOCAL_DECL(cfun, i, var) {
    let mut init: tree = DECL_INITIAL(var);
    if (!init)
    continue;
    if (TREE_CODE(init) != CONSTRUCTOR)
    continue;
    check_bad_casts_in_constructor(var, init);
    }
    }
//
// iterate over all statements to find "bad" casts:
// those where the address of the start of a structure is cast
// to a pointer of a structure of a different type, or a
// structure pointer type is cast to a different structure pointer type
//
#[no_mangle]
unsafe extern "C" fn find_bad_casts_execute() -> c_uint {
    static unsigned int find_bad_casts_execute(void)
    {
    basic_block bb;
    handle_local_var_initializers();
    FOR_EACH_BB_FN(bb, cfun) {
    gimple_stmt_iterator gsi;
    for (gsi = gsi_start_bb(bb); !gsi_end_p(gsi); gsi_next(&gsi)) {
    gimple stmt;
    const_tree lhs;
    const_tree lhs_type;
    const_tree rhs1;
    const_tree rhs_type;
    const_tree ptr_lhs_type;
    const_tree ptr_rhs_type;
    const_tree op0;
    const_tree op0_type;
    enum tree_code rhs_code;
    stmt = gsi_stmt(gsi);

    debug_gimple_stmt(stmt);
    debug_tree(gimple_get_lhs(stmt));

    if (gimple_code(stmt) != GIMPLE_ASSIGN)
    continue;

    debug_tree(gimple_assign_rhs1(stmt));

    rhs_code = gimple_assign_rhs_code(stmt);
    if (rhs_code != ADDR_EXPR && rhs_code != SSA_NAME)
    continue;
    lhs = gimple_get_lhs(stmt);
    lhs_type = TREE_TYPE(lhs);
    rhs1 = gimple_assign_rhs1(stmt);
    rhs_type = TREE_TYPE(rhs1);
    if (TREE_CODE(rhs_type) != POINTER_TYPE ||
    TREE_CODE(lhs_type) != POINTER_TYPE)
    continue;
    ptr_lhs_type = TYPE_MAIN_VARIANT(strip_array_types(TYPE_MAIN_VARIANT(TREE_TYPE(lhs_type))));
    ptr_rhs_type = TYPE_MAIN_VARIANT(strip_array_types(TYPE_MAIN_VARIANT(TREE_TYPE(rhs_type))));
    if (ptr_rhs_type == void_type_node)
    continue;
    if (ptr_lhs_type == void_type_node)
    continue;
    if (dominated_by_is_err(rhs1, bb))
    continue;
    if (TREE_CODE(ptr_rhs_type) != RECORD_TYPE) {

    if (lookup_attribute("randomize_performed", TYPE_ATTRIBUTES(ptr_lhs_type)))

    MISMATCH(gimple_location(stmt), "rhs", ptr_lhs_type, ptr_rhs_type);
    continue;
    }
    if (rhs_code == SSA_NAME && ptr_lhs_type == ptr_rhs_type)
    continue;
    if (rhs_code == ADDR_EXPR) {
    op0 = TREE_OPERAND(rhs1, 0);
    if (op0 == NULL_TREE)
    continue;
    if (TREE_CODE(op0) != VAR_DECL)
    continue;
    op0_type = TYPE_MAIN_VARIANT(strip_array_types(TYPE_MAIN_VARIANT(TREE_TYPE(op0))));
    if (op0_type == ptr_lhs_type)
    continue;

    if (lookup_attribute("randomize_performed", TYPE_ATTRIBUTES(op0_type)))

    MISMATCH(gimple_location(stmt), "op0", ptr_lhs_type, op0_type);
    } else {
    let mut ssa_name_var: const_tree = SSA_NAME_VAR(rhs1);
// skip bogus type casts introduced by container_of
    if (ssa_name_var != NULL_TREE && DECL_NAME(ssa_name_var) &&
    !strcmp((const char *)DECL_NAME_POINTER(ssa_name_var), "__mptr"))
    continue;

    if (lookup_attribute("randomize_performed", TYPE_ATTRIBUTES(ptr_rhs_type)))

    MISMATCH(gimple_location(stmt), "ssa", ptr_lhs_type, ptr_rhs_type);
    }
    }
    }
    return 0;
    }

// Macro flag: #define NO_GATE

#[no_mangle]
pub unsafe extern "C" fn plugin_init(plugin_info: *mut plugin_name_args, version: *mut plugin_gcc_version) -> __visible int {
    __visible int plugin_init(struct plugin_name_args *plugin_info, struct plugin_gcc_version *version)
    {
    int i;
    let mut plugin_name: *const char  const = plugin_info.base_name;
    let mut argc: c_int = plugin_info.argc;
    let mut argv: *const plugin_argument  const = plugin_info.argv;
    let mut enable: bool = true;
    let mut obtained_seed: c_int = 0;
    struct register_pass_info find_bad_casts_pass_info;
    find_bad_casts_pass_info.pass			= make_find_bad_casts_pass();
    find_bad_casts_pass_info.reference_pass_name	= "ssa";
    find_bad_casts_pass_info.ref_pass_instance_number	= 1;
    find_bad_casts_pass_info.pos_op			= PASS_POS_INSERT_AFTER;
    if (!plugin_default_version_check(version, &gcc_version)) {
    error(G_("incompatible gcc/plugin versions"));
    return 1;
    }
    if (strncmp(lang_hooks.name, "GNU C", 5) && !strncmp(lang_hooks.name, "GNU C+", 6)) {
    inform(UNKNOWN_LOCATION, G_("%s supports C only, not %s"), plugin_name, lang_hooks.name);
    enable = false;
    }
    for (i = 0; i < argc; ++i) {
    if (!strcmp(argv[i].key, "disable")) {
    enable = false;
    continue;
    }
    if (!strcmp(argv[i].key, "performance-mode")) {
    performance_mode = 1;
    continue;
    }
    error(G_("unknown option '-fplugin-arg-%s-%s'"), plugin_name, argv[i].key);
    }
    if (strlen(randstruct_seed) != 64) {
    error(G_("invalid seed value supplied for %s plugin"), plugin_name);
    return 1;
    }
    obtained_seed = sscanf(randstruct_seed, "%016llx%016llx%016llx%016llx",
    &shuffle_seed[0], &shuffle_seed[1], &shuffle_seed[2], &shuffle_seed[3]);
    if (obtained_seed != 4) {
    error(G_("Invalid seed supplied for %s plugin"), plugin_name);
    return 1;
    }
    register_callback(plugin_name, PLUGIN_INFO, core::ptr::null_mut(), &randomize_layout_plugin_info);
    if (enable) {
    register_callback(plugin_name, PLUGIN_ALL_IPA_PASSES_START, check_global_variables, core::ptr::null_mut());
    register_callback(plugin_name, PLUGIN_PASS_MANAGER_SETUP, core::ptr::null_mut(), &find_bad_casts_pass_info);
    register_callback(plugin_name, PLUGIN_FINISH_TYPE, finish_type, core::ptr::null_mut());
    register_callback(plugin_name, PLUGIN_FINISH_DECL, randomize_layout_finish_decl, core::ptr::null_mut());
    }
    register_callback(plugin_name, PLUGIN_ATTRIBUTES, register_attributes, core::ptr::null_mut());
    return 0;
    }
