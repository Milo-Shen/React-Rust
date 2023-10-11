// todo: ignore type WorkTag

pub const FUNCTION_COMPONENT: i32 = 0;
pub const CLASS_COMPONENT: i32 = 1;
pub const INDETERMINATE_COMPONENT: i32 = 2; // Before we know whether it is function or class
pub const HOST_ROOT: i32 = 3; // Root of a host tree. Could be nested inside another node.
pub const HOST_PORTAL: i32 = 4; // A subtree. Could be an entry point to a different renderer.
pub const HOST_COMPONENT: i32 = 5;
pub const HOST_TEXT: i32 = 6;
pub const FRAGMENT: i32 = 7;
pub const MODE: i32 = 8;
pub const CONTEXT_CONSUMER: i32 = 9;
pub const CONTEXT_PROVIDER: i32 = 10;
pub const FORWARD_REF: i32 = 11;
pub const PROFILER: i32 = 12;
pub const SUSPENSE_COMPONENT: i32 = 13;
pub const MEMO_COMPONENT: i32 = 14;
pub const SIMPLE_MEMO_COMPONENT: i32 = 15;
pub const LAZY_COMPONENT: i32 = 16;
pub const INCOMPLETE_CLASS_COMPONENT: i32 = 17;
pub const DEHYDRATED_FRAGMENT: i32 = 18;
pub const SUSPENSE_LIST_COMPONENT: i32 = 19;
pub const SCOPE_COMPONENT: i32 = 21;
pub const OFFSCREEN_COMPONENT: i32 = 22;
pub const LEGACY_HIDDEN_COMPONENT: i32 = 23;
pub const CACHE_COMPONENT: i32 = 24;
pub const TRACING_MARKER_COMPONENT: i32 = 25;
