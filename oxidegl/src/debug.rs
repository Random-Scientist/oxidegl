use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use log::{Level, Record, RecordBuilder, logger};
use objc2::AllocAnyThread;
use objc2_foundation::NSString;
use std::{
    any,
    cell::Cell,
    collections::VecDeque,
    ffi::{CStr, CString, c_char, c_void},
    fmt::Arguments,
    mem::{self, MaybeUninit},
    pin::Pin,
    ptr::{self, NonNull},
    slice,
};
//TODO fallible implementations for these commands
use crate::{
    gl_enums::{DebugSeverity, DebugSource, DebugType},
    gl_types::{GLDEBUGPROC, GLchar, GLsizei},
};

use super::{
    Context,
    gl_object::{NamedObject, ObjectName},
};

thread_local! {
    // We store the debug logging infrastructure for the current context in a separate thread local to avoid passing it in by-reference every log call (which cannot be avoided/worked around with macros)
    static DEBUG_STATE: Cell<Option<DebugState>> = const { Cell::new(None) };
}

#[inline]
pub(crate) fn with_debug_state_mut<F, Ret>(func: F) -> Option<Ret>
where
    F: FnOnce(Pin<&mut DebugState>) -> Ret,
{
    if let Some(mut s) = DEBUG_STATE.take() {
        let p = Pin::new(&mut s);
        let ret = Some(func(p));
        DEBUG_STATE.replace(Some(s));
        ret
    } else {
        None
    }
}

#[inline]
pub(crate) fn with_debug_state<F, Ret>(func: F) -> Option<Ret>
where
    F: FnOnce(&DebugState) -> Ret,
{
    if let Some(s) = DEBUG_STATE.take() {
        let ret = Some(func(&s));
        DEBUG_STATE.replace(Some(s));
        ret
    } else {
        None
    }
}

impl Context {
    pub fn made_current(&mut self) {
        DEBUG_STATE.replace(self.gl_state.debug_state_holder.0.take());
    }
    pub fn made_not_current(&mut self) {
        self.gl_state.debug_state_holder.0 = DEBUG_STATE.take();
    }
}
impl From<Level> for DebugSeverity {
    fn from(value: Level) -> Self {
        match value {
            Level::Error => Self::DebugSeverityHigh,
            Level::Warn => Self::DebugSeverityMedium,
            Level::Info => Self::DebugSeverityLow,
            Level::Debug | Level::Trace => Self::DebugSeverityNotification,
        }
    }
}
#[derive(Debug)]
pub(crate) struct DebugMessageMeta {
    pub(crate) src: DebugSource,
    pub(crate) ty: DebugType,
    pub(crate) sev: DebugSeverity,
    pub(crate) id: u32,
}
#[derive(Debug)]
pub(crate) struct DebugLogMessage {
    text: Box<CStr>,
    meta: DebugMessageMeta,
}

#[derive(Debug)]
pub struct DebugState {
    messages: VecDeque<DebugLogMessage>,
    pub(crate) debug_groups: Vec<DebugGroup>,
    pub(crate) callback: GLDEBUGPROC,
    pub(crate) debug_labels: HashMap<any::TypeId, HashMap<u32, Box<CStr>>>,
    pub(crate) user_param_ptr: *mut c_void,
}
#[derive(Debug, Clone)]
pub(crate) struct DebugGroup {
    pub(crate) message: Box<CStr>,
    id: u32,
    src: DebugSource,
    filter: HashMap<u32, HashMap<u32, DisabledMessages>>,
}
#[derive(Debug, Clone)]
pub struct DisabledMessages {
    disabled_ids: HashSet<u32>,
    disabled_severities: HashSet<u32>,
}
impl DisabledMessages {
    fn is_enabled(&self, severity: DebugSeverity, id: u32) -> bool {
        !self.disabled_ids.contains(&id) && !self.disabled_severities.contains(&(severity as u32))
    }
    fn empty() -> Self {
        Self {
            disabled_ids: HashSet::new(),
            disabled_severities: HashSet::new(),
        }
    }
}
impl DebugState {
    #[inline]
    fn should_log(&self, meta: &DebugMessageMeta) -> bool {
        !self
            .debug_groups
            .last()
            .unwrap()
            .filter
            .get(&(meta.src as u32))
            .is_some_and(|m| {
                m.get(&(meta.ty as u32))
                    .is_some_and(|d| !d.is_enabled(meta.sev, meta.id))
            })
    }
    /// glDebugMessageControl interface
    pub(crate) unsafe fn message_control(
        &mut self,
        src: DebugSource,
        ty: DebugType,
        sev: DebugSeverity,
        count: GLsizei,
        ids: *const u32,
        enable: bool,
    ) {
        //FIXME: code duplication
        if count > 0 {
            let map_over_types = match self
                .debug_groups
                .last_mut()
                .unwrap()
                .filter
                .entry(src as u32)
            {
                std::collections::hash_map::Entry::Occupied(occupied_entry) => {
                    occupied_entry.into_mut()
                }
                std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(HashMap::new())
                }
            };
            let disable = match map_over_types.entry(ty as u32) {
                std::collections::hash_map::Entry::Occupied(occupied_entry) => {
                    occupied_entry.into_mut()
                }
                std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(DisabledMessages::empty())
                }
            };
            #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
            // Safety: caller ensures count and ids are valid to construct a slice of uint from
            for i in unsafe { slice::from_raw_parts(ids, count as usize) } {
                if enable {
                    disable.disabled_ids.remove(i);
                } else {
                    disable.disabled_ids.insert(*i);
                }
            }
        } else {
            let s = src as u32;
            let t = ty as u32;
            let sources = if src == DebugSource::DontCare {
                &DEBUG_SOURCES
            } else {
                slice::from_ref(&s)
            };
            let types = if ty == DebugType::DontCare {
                &DEBUG_TYPES
            } else {
                slice::from_ref(&t)
            };
            for src in sources {
                let map_over_types = match self.debug_groups.last_mut().unwrap().filter.entry(*src)
                {
                    std::collections::hash_map::Entry::Occupied(occupied_entry) => {
                        occupied_entry.into_mut()
                    }
                    std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                        vacant_entry.insert(HashMap::new())
                    }
                };
                for ty in types {
                    let disable = match map_over_types.entry(*ty) {
                        std::collections::hash_map::Entry::Occupied(occupied_entry) => {
                            occupied_entry.into_mut()
                        }
                        std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                            vacant_entry.insert(DisabledMessages::empty())
                        }
                    };
                    if enable {
                        disable.disabled_severities.remove(&(sev as u32));
                    } else {
                        disable.disabled_severities.insert(sev as u32);
                    }
                }
            }
        }
    }
    /// glPushDebugGroup impl
    pub(crate) unsafe fn push_debug_group(
        &mut self,
        source: DebugSource,
        id: u32,
        length: i32,
        message: *const c_char,
    ) {
        let msg: Box<CStr> = match length {
            // Safety: caller ensures that if length is negative, message points to a valid nul-terminated C-string
            ..0 => unsafe { CStr::from_ptr(message) }.into(),
            0 => c"".into(),
            1.. => {
                #[expect(clippy::cast_sign_loss, reason = "sign loss is avoided via match arm")]
                let mut bytes =
                // Safety: caller ensures that if length is positive, it represents the length of message
                    unsafe { slice::from_raw_parts(message.cast::<u8>(), length as usize) }
                        .to_vec();
                // message is not guaranteed to be nul-terminated afaict, rust's CStr(ing) (and the GL client) expects this
                if *bytes.last().unwrap() != 0 {
                    bytes.push(0);
                }
                CString::from_vec_with_nul(bytes)
                    .unwrap()
                    .into_boxed_c_str()
            }
        };
        let mut new_group = self.debug_groups.last().unwrap().clone();
        new_group.id = id;
        new_group.message = msg;
        new_group.src = source;
        self.debug_groups.push(new_group);

        // Cursed closure hack to work around an odd behavior in format_args
        let mut do_log = {
            #[inline]
            |args: Arguments| {
                let rec = RecordBuilder::new()
                    .args(args)
                    .level(Level::Debug)
                    .target("debug_groups")
                    .build();
                self.log_internal(
                    &rec,
                    DebugMessageMeta {
                        src: source,
                        ty: DebugType::DebugTypePushGroup,
                        sev: DebugSeverity::DebugSeverityNotification,
                        id,
                    },
                );
            }
        };
        do_log(format_args!("Pushing GL Debug group {id}"));
    }
    /// glPopDebugGroup impl
    pub(crate) fn pop_debug_group(&mut self) {
        if self.debug_groups.len() == 1 {
            // should be GL_STACK_UNDERFLOW (lmao)
            log::warn!("Tried to pop the root GL debug group");
            return;
        }
        let v = self.debug_groups.pop().unwrap();
        let mut do_log = {
            #[inline]
            |args: Arguments| {
                let rec = RecordBuilder::new()
                    .args(args)
                    .level(Level::Debug)
                    .target("debug_groups")
                    .build();
                self.log_internal(
                    &rec,
                    DebugMessageMeta {
                        src: v.src,
                        ty: DebugType::DebugTypePopGroup,
                        sev: DebugSeverity::DebugSeverityNotification,
                        id: v.id,
                    },
                );
            }
        };
        do_log(format_args!(
            "Popped GL debug group {}: {}",
            v.id,
            v.message.to_str().expect("message wasn't valid UTF-8!")
        ));
    }
    /// __NOT PART OF PUBLIC API__
    #[doc(hidden)]
    pub fn log_impl(id: u32, gl_src: DebugSource, gl_ty: DebugType, rec: &Record) {
        let meta = DebugMessageMeta {
            src: gl_src,
            ty: gl_ty,
            sev: rec.level().into(),
            id,
        };
        let Some(s) = DEBUG_STATE.take() else {
            log::warn!(
                "OxideGL tried to log the following message, but logging context wasn't set:"
            );
            logger().log(rec);
            return;
        };
        let mut state = Some(s);
        if rec.args().as_str().is_some() {
            // skip storeback if we can prove no Debug::fmt impls are invoked
            state.as_mut().unwrap().log_internal(rec, meta);
        } else {
            Self::log_internal_with_storeback(&mut state, rec, meta);
        }
        DEBUG_STATE.set(state);
    }
    fn log_internal(&mut self, rec: &Record, meta: DebugMessageMeta) {
        if !self.should_log(&meta) {
            return;
        }
        logger().log(rec);

        let mut bytes = rec.args().to_string().into_bytes();
        bytes.push(0);
        let msg = CString::from_vec_with_nul(bytes)
            .expect("failed to convert log message to C string!")
            .into_boxed_c_str();
        self.log_to_gl_client(msg, meta);
    }
    /// Alternative implementation of `log_internal` which writes the `DebugState` back into
    /// the thread local storage for potential access by the format args' lazy eval
    fn log_internal_with_storeback(
        this_opt: &mut Option<Self>,
        rec: &Record,
        meta: DebugMessageMeta,
    ) {
        let this = this_opt.as_mut().unwrap();
        if !this.should_log(&meta) {
            return;
        }
        logger().log(rec);

        // put the state back so the formatting code being invoked by to_string can access it
        let prev = DEBUG_STATE.replace(mem::take(this_opt));

        // The `to_string` call might try to use the debug state
        let mut bytes = rec.args().to_string().into_bytes();

        // get it back to use afterwards
        *this_opt = DEBUG_STATE.replace(prev);

        bytes.push(0);

        let this = this_opt.as_mut().unwrap();
        let msg = CString::from_vec_with_nul(bytes)
            .expect("failed to convert log message to C string!")
            .into_boxed_c_str();
        this.log_to_gl_client(msg, meta);
    }
    fn log_to_gl_client(&mut self, text: Box<CStr>, meta: DebugMessageMeta) {
        if let Some(func) = self.callback {
            #[expect(
                clippy::cast_possible_wrap,
                clippy::cast_possible_truncation,
                reason = "no >4mil char log messages"
            )]
            // Safety: string pointer points to a valid, nul-terminated C string,
            // str_len is the length of the string's allocation minus one for the nul byte
            unsafe {
                func(
                    meta.src as u32,
                    meta.ty as u32,
                    meta.id,
                    meta.sev as u32,
                    text.count_bytes() as i32,
                    text.as_ptr().cast(),
                    self.user_param_ptr,
                );
            };
        } else {
            self.messages.push_back(DebugLogMessage { text, meta });
        }
    }
    #[expect(
        clippy::undocumented_unsafe_blocks,
        clippy::too_many_arguments,
        reason = "blame the GL spec for this one"
    )]
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    pub(crate) unsafe fn get_log_message(
        &mut self,
        count: u32,
        buf_size: GLsizei,
        sources: *mut DebugSource,
        types: *mut DebugType,
        ids: *mut u32,
        severities: *mut DebugSeverity,
        lengths: *mut GLsizei,
        buf: *mut GLchar,
    ) -> u32 {
        #[inline]
        unsafe fn nullable_slice_mut<T: Sized>(
            // borrow ptr to infer a sane lifetime on the returned slice
            ptr: &*mut T,
            count: u32,
        ) -> Option<&mut [MaybeUninit<T>]> {
            if ptr.is_null() {
                None
            } else {
                // Safety: caller ensures slices are allocated (they are not required to be initialized, hence the MaybeUninit)
                Some(unsafe {
                    slice::from_raw_parts_mut(ptr.cast::<MaybeUninit<T>>(), count as usize)
                })
            }
        }
        let mut sources = unsafe { nullable_slice_mut(&sources, count) };
        let mut types = unsafe { nullable_slice_mut(&types, count) };
        let mut ids = unsafe { nullable_slice_mut(&ids, count) };
        let mut severities = unsafe { nullable_slice_mut(&severities, count) };
        let mut lengths = unsafe { nullable_slice_mut(&lengths, count) };
        let mut buf = unsafe { nullable_slice_mut(&buf, count) };
        let mut remaining = buf_size as usize;
        let mut count_written = 0;
        loop {
            let Some(msg) = self.messages.front() else {
                break;
            };
            let msg_len = msg.text.count_bytes() + 1;
            if remaining < msg_len || count_written == count {
                break;
            }
            //unwrap: checked to be some
            let msg = self.messages.pop_front().unwrap();
            if let Some(s) = sources.as_mut() {
                s[count_written as usize].write(msg.meta.src);
            }
            if let Some(s) = types.as_mut() {
                s[count_written as usize].write(msg.meta.ty);
            }
            if let Some(s) = ids.as_mut() {
                s[count_written as usize].write(msg.meta.id);
            }
            if let Some(s) = severities.as_mut() {
                s[count_written as usize].write(msg.meta.sev);
            }
            if let Some(s) = lengths.as_mut() {
                #[expect(
                    clippy::cast_possible_truncation,
                    clippy::cast_possible_wrap,
                    reason = "no >4gb log message"
                )]
                s[count_written as usize].write(msg_len as i32);
            }
            if let Some(r) = mem::take(&mut buf) {
                let (write, remaining) = r.split_at_mut(msg_len);
                buf = Some(remaining);
                write.copy_from_slice(
                    // Safety: cast from [i8] to [MaybeUninit<i8>] is valid as MaybeUninit<T> has a looser or equal validity invariant and same representation as T
                    unsafe { slice::from_raw_parts(msg.text.as_ptr().cast(), msg_len) },
                );
            }
            remaining -= msg_len;
            count_written += 1;
        }
        count_written
    }
    pub(crate) fn get_label<T: ?Sized + 'static>(&self, name: ObjectName<T>) -> Option<Box<CStr>> {
        self.debug_labels
            .get(&any::TypeId::of::<T>())
            .and_then(|v| v.get(&name.to_raw()))
            .cloned()
    }
    pub(crate) fn set_label<T: NamedObject>(
        &mut self,
        ctx: &mut Context,
        name: ObjectName<T>,
        label: Option<Box<CStr>>,
    ) {
        let map_for_type = match self.debug_labels.entry(any::TypeId::of::<T>()) {
            std::collections::hash_map::Entry::Occupied(occupied_entry) => {
                occupied_entry.into_mut()
            }
            std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(HashMap::new())
            }
        };

        T::set_debug_label(
            ctx,
            name,
            // Safety: null_terminated_c_string points to an initialized, nul-terminated C string (as upheld by the CStr type)
            label.as_deref().map(|c| unsafe {
                let alloc = NSString::alloc();
                NSString::initWithCString_encoding(
                    alloc,
                    NonNull::new_unchecked(c.as_ptr().cast_mut()),
                    NSString::defaultCStringEncoding(),
                )
                .expect("failed to create NSString")
            }),
        );
        if let Some(label) = label {
            map_for_type.insert(name.to_raw(), label);
        } else {
            let _ = map_for_type.remove(&name.to_raw());
        }
    }
}
impl Default for DebugState {
    fn default() -> Self {
        let debug_groups = vec![DebugGroup {
            message: c"default debug group".into(),
            id: 0,
            src: DebugSource::DebugSourceApi,
            filter: HashMap::new(),
        }];
        Self {
            messages: VecDeque::new(),
            debug_labels: HashMap::new(),
            debug_groups,
            callback: None,
            user_param_ptr: ptr::null_mut(),
        }
    }
}
// Safety: DebugType is repr(u32)
const DEBUG_TYPES: [u32; 9] = unsafe {
    mem::transmute([
        DebugType::DebugTypeDeprecatedBehavior,
        DebugType::DebugTypeError,
        DebugType::DebugTypeMarker,
        DebugType::DebugTypeOther,
        DebugType::DebugTypePerformance,
        DebugType::DebugTypePopGroup,
        DebugType::DebugTypePushGroup,
        DebugType::DebugTypePortability,
        DebugType::DebugTypeUndefinedBehavior,
    ])
};
// Safety: DebugSource is repr(u32)
const DEBUG_SOURCES: [u32; 6] = unsafe {
    mem::transmute([
        DebugSource::DebugSourceApi,
        DebugSource::DebugSourceApplication,
        DebugSource::DebugSourceOther,
        DebugSource::DebugSourceShaderCompiler,
        DebugSource::DebugSourceThirdParty,
        DebugSource::DebugSourceWindowSystem,
    ])
};

#[allow(unused)]
pub(crate) use macros::{gl_debug, gl_err, gl_info, gl_log, gl_trace, gl_warn};

pub mod macros {

    /// Root of the developer-facing `OxideGL` logging infrastructure.
    ///
    /// ## Arguments:
    /// `id`: `KHR_debug` ID of this message. Not recommended to set manually, as it may collide with automatically generated message IDs. Defaults to an automatically generated ID.
    ///
    /// `src`: `KHR_debug` debug source identifier for this message (defaults to `DEBUG_SOURCE_API`). Can be one of:
    /// * `Api` - Message generated by the API
    /// * `Application` - Message generated by the application
    /// * `ShaderCompiler` - Message generated by the shader compiler
    /// * `ThirdParty` - Message generated by a third party
    /// * `WindowSystem` - Message generated by the window system
    /// * `None` - No message source, implementation will pick an arbitrary one for you
    /// * `Other` - Message generated by something else
    ///
    /// `ty`: `KHR_debug` debug type identifier for this message (defaults to `DEBUG_TYPE_OTHER`). Can be one of:
    /// * `DeprecatedBehavior` - Message warns about deprecated usage patterns
    /// * `Error` - Message describes an error
    /// * `Marker` - Message denotes a debug marker
    /// * `Performance` - Message warns about usage patterns that decrease performance or are innefficient
    /// * `PushGroup`, `PopGroup` - Used internally by the `KHR_debug` implementation
    /// * `Portability` - Message warns about non-portable/unspecified behavior
    /// * `UndefinedBehavior` - Message warns the user that their application has caused Undefined Behavior
    /// * `None` - No message type, implementation will pick an arbitrary one for you
    /// * `Other` - Message describes something else
    ///
    /// `level`: Describes the severity of the message (no default, must be specified). Can be one of:
    /// * `Trace` - Extremely verbose information that is entirely irrelevant to the user unless they are debugging `OxideGL` itself (maps to `GL_DEBUG_SEVERITY_NOTIFICATION`)
    /// * `Debug` - Somewhat verbose information that might be helpful to the user to introspect what code paths their code is triggering within `OxideGL` (maps to `GL_DEBUG_SEVERITY_NOTIFICATION`)
    /// * `Info` - Information that is generally helpful to the user but does not indicate any failiure on the user or `OxideGL`'s part (maps to `GL_DEBUG_SEVERITY_LOW`)
    /// * `Warn` - Information that informs the user when `OxideGL` encounters a degenerate case or other recoverable error (maps to `GL_DEBUG_SEVERITY_MEDIUM`)
    /// * `Error` - Information that informs the user about an uncrecoverable error in `OxideGL` (maps to `GL_DEBUG_SEVERITY_HIGH`)
    ///
    /// `target`: Roughly identifies the region of `OxideGL`'s code that sent this message. This is not used in the generation of messages sent to the GL client,
    /// but it is forwarded to any Rust `Logger` that is associated with the current debug state. Defaults to the `std::module_path!()`.
    ///
    /// Message: the remaining arguments are passed verbatim to `std::format_args!()` to form the message.
    ///
    /// ## Examples
    /// ```
    /// use crate::context::debug::gl_log;
    ///
    /// gl_log!(src: Api, ty: UndefinedBehavior, level: Info, target: "test", "this is a message from {}", "the OxideGL logger!");
    ///
    /// gl_log!(src: Api, ty: UndefinedBehavior, level: Info, "this is a message from {}", "the OxideGL logger!");
    ///
    /// gl_log!(ty: UndefinedBehavior, level: Info, target: "test", "this is a message from {}", "the OxideGL logger!");
    ///
    /// gl_log!(ty: UndefinedBehavior, level: Info, "this is a message from {}", "the OxideGL logger!");
    ///
    /// gl_log!(src: Api, level: Info, target: "test", "this is a message from {}", "the OxideGL logger!");
    ///
    /// gl_log!(src: Api, level: Info, "this is a message from {}", "the OxideGL logger!");
    ///
    /// gl_log!(level: Info, target: "test", "this is a message from {}", "the OxideGL logger!");
    ///
    /// gl_log!(level: Info, "this is a message from {}", "the OxideGL logger!");
    /// ```
    macro_rules! gl_log {

        // gl_log!(id: 2, src: Api, ty: UndefinedBehavior, Level::Warn, target: "asdf", "this is an {} warning", "OxideGL")
        (id: $id:expr, src: $src:ident, ty: $ty:ident, level: $level:expr, target: $target:expr, $($rest:tt)+) => {
            let _: () = {
                const TARGET: &str = $target;
                const ID: u32 = $id;
                mod ns_src_lvl {
                    #![allow(unused_imports)]
                    use $crate::debug::__logging_private as log_impl;

                    use log_impl::Level::*;
                    pub(super) const LEVEL: log_impl::Level = $level;

                    use log_impl::DebugSource::DebugSourceApi as None;
                    use log_impl::DebugSource::DebugSourceApi as Api;
                    use log_impl::DebugSource::DebugSourceApplication as Application;
                    use log_impl::DebugSource::DebugSourceOther as Other;
                    use log_impl::DebugSource::DebugSourceShaderCompiler as ShaderCompiler;
                    use log_impl::DebugSource::DebugSourceThirdParty as ThirdParty;
                    use log_impl::DebugSource::DebugSourceWindowSystem as WindowSystem;
                    pub(super) const SOURCE: log_impl::DebugSource = $src;
                }
                mod ns_type {
                    #![allow(unused_imports)]
                    use $crate::debug::__logging_private as log_impl;
                    use log_impl::DebugType::DebugTypeOther as None;
                    use log_impl::DebugType::DebugTypeDeprecatedBehavior as DeprecatedBehavior;
                    use log_impl::DebugType::DebugTypeError as Error;
                    use log_impl::DebugType::DebugTypeMarker as Marker;
                    use log_impl::DebugType::DebugTypeOther as Other;
                    use log_impl::DebugType::DebugTypePerformance as Performance;
                    use log_impl::DebugType::DebugTypePopGroup as PopGroup;
                    use log_impl::DebugType::DebugTypePortability as Portability;
                    use log_impl::DebugType::DebugTypePushGroup as PushGroup;
                    use log_impl::DebugType::DebugTypeUndefinedBehavior as UndefinedBehavior;
                    pub(super) const TYPE: log_impl::DebugType = $ty;
                }
                // Pass the Arguments in due to an annoying bug/odd behavior in the way format_args! lifetimes work.
                // Also so the format argument expressions are evaluated in the outer scope instead
                // of within imp (which would break many argument expressions that are valid with log! and the like)
                #[inline]
                fn imp(args: $crate::debug::__logging_private::Arguments) {
                    let record = $crate::debug::__logging_private::Record::builder()
                        .args(args)
                        .target(TARGET)
                        .level(ns_src_lvl::LEVEL)
                        .module_path_static(Some(
                            $crate::debug::__logging_private::module_path!()
                        ))
                        .file_static(Some($crate::debug::__logging_private::file!()))
                        .line(Some($crate::debug::__logging_private::line!()))
                        .build();
                    $crate::debug::__logging_private::DebugState::log_impl(ID, ns_src_lvl::SOURCE, ns_type::TYPE, &record);
                }
                imp($crate::debug::__logging_private::format_args!($($rest)+));
            }
        };
        // gl_log!(src: Api, ty: UndefinedBehavior, Level::Warn, target: "asdf", "this is an {} warning", "OxideGL")
        (src: $src:ident, ty: $ty:ident, level: $level:expr, target: $target:expr, $($rest:tt)+) => {
            let _: () = {
                const TARGET: &str = $target;
                const ID: u32 = $crate::debug::__logging_private::make_message_id(
                    $crate::debug::__logging_private::file!(),
                    $crate::debug::__logging_private::line!()
                );
                mod ns_src_lvl {
                    #![allow(unused_imports)]
                    use $crate::debug::__logging_private as log_impl;

                    use log_impl::Level::*;
                    pub(super) const LEVEL: log_impl::Level = $level;

                    use log_impl::DebugSource::DebugSourceApi as None;
                    use log_impl::DebugSource::DebugSourceApi as Api;
                    use log_impl::DebugSource::DebugSourceApplication as Application;
                    use log_impl::DebugSource::DebugSourceOther as Other;
                    use log_impl::DebugSource::DebugSourceShaderCompiler as ShaderCompiler;
                    use log_impl::DebugSource::DebugSourceThirdParty as ThirdParty;
                    use log_impl::DebugSource::DebugSourceWindowSystem as WindowSystem;
                    pub(super) const SOURCE: log_impl::DebugSource = $src;
                }
                mod ns_type {
                    #![allow(unused_imports)]
                    use $crate::debug::__logging_private as log_impl;
                    use log_impl::DebugType::DebugTypeOther as None;
                    use log_impl::DebugType::DebugTypeDeprecatedBehavior as DeprecatedBehavior;
                    use log_impl::DebugType::DebugTypeError as Error;
                    use log_impl::DebugType::DebugTypeMarker as Marker;
                    use log_impl::DebugType::DebugTypeOther as Other;
                    use log_impl::DebugType::DebugTypePerformance as Performance;
                    use log_impl::DebugType::DebugTypePopGroup as PopGroup;
                    use log_impl::DebugType::DebugTypePortability as Portability;
                    use log_impl::DebugType::DebugTypePushGroup as PushGroup;
                    use log_impl::DebugType::DebugTypeUndefinedBehavior as UndefinedBehavior;
                    pub(super) const TYPE: log_impl::DebugType = $ty;
                }
                // Pass the Arguments in due to an annoying bug/odd behavior in the way format_args! lifetimes work.
                // Also so the format argument expressions are evaluated in the outer scope instead
                // of within imp (which would break many argument expressions that are valid with log! and the like)
                #[inline]
                fn imp(args: $crate::debug::__logging_private::Arguments) {
                    let record = $crate::debug::__logging_private::Record::builder()
                        .args(args)
                        .target(TARGET)
                        .level(ns_src_lvl::LEVEL)
                        .module_path_static(Some(
                            $crate::debug::__logging_private::module_path!()
                        ))
                        .file_static(Some($crate::debug::__logging_private::file!()))
                        .line(Some($crate::debug::__logging_private::line!()))
                        .build();
                    $crate::debug::__logging_private::DebugState::log_impl(ID, ns_src_lvl::SOURCE, ns_type::TYPE, &record);
                }
                imp($crate::debug::__logging_private::format_args!($($rest)+));
            };
        };
        // gl_log!(src: Api, ty: UndefinedBehavior, Level::Warn, "this is an {} warning", "OxideGL");
        (src: $src:ident, ty: $ty:ident, level: $level:expr, $($rest:tt)+) => {
            let _: () = {
                const TARGET: &str = $crate::debug::__logging_private::module_path!();
                const ID: u32 = $crate::debug::__logging_private::make_message_id(
                    $crate::debug::__logging_private::file!(),
                    $crate::debug::__logging_private::line!()
                );
                mod ns_src_lvl {
                    #![allow(unused_imports)]
                    use $crate::debug::__logging_private as log_impl;

                    use log_impl::Level::*;
                    pub(super) const LEVEL: log_impl::Level = $level;

                    use log_impl::DebugSource::DebugSourceApi as None;
                    use log_impl::DebugSource::DebugSourceApi as Api;
                    use log_impl::DebugSource::DebugSourceApplication as Application;
                    use log_impl::DebugSource::DebugSourceOther as Other;
                    use log_impl::DebugSource::DebugSourceShaderCompiler as ShaderCompiler;
                    use log_impl::DebugSource::DebugSourceThirdParty as ThirdParty;
                    use log_impl::DebugSource::DebugSourceWindowSystem as WindowSystem;
                    pub(super) const SOURCE: log_impl::DebugSource = $src;
                }
                mod ns_type {
                    #![allow(unused_imports)]
                    use $crate::debug::__logging_private as log_impl;
                    use log_impl::DebugType::DebugTypeOther as None;
                    use log_impl::DebugType::DebugTypeDeprecatedBehavior as DeprecatedBehavior;
                    use log_impl::DebugType::DebugTypeError as Error;
                    use log_impl::DebugType::DebugTypeMarker as Marker;
                    use log_impl::DebugType::DebugTypeOther as Other;
                    use log_impl::DebugType::DebugTypePerformance as Performance;
                    use log_impl::DebugType::DebugTypePopGroup as PopGroup;
                    use log_impl::DebugType::DebugTypePortability as Portability;
                    use log_impl::DebugType::DebugTypePushGroup as PushGroup;
                    use log_impl::DebugType::DebugTypeUndefinedBehavior as UndefinedBehavior;
                    pub(super) const TYPE: log_impl::DebugType = $ty;
                }
                // Pass the Arguments in due to an annoying bug/odd behavior in the way format_args! lifetimes work.
                // Also so the format argument expressions are evaluated in the outer scope instead
                // of within imp (which would break many argument expressions that are valid with log! and the like)
                #[inline]
                fn imp(args: $crate::debug::__logging_private::Arguments) {
                    let record = $crate::debug::__logging_private::Record::builder()
                        .args(args)
                        .target(TARGET)
                        .level(ns_src_lvl::LEVEL)
                        .module_path_static(Some(
                            $crate::debug::__logging_private::module_path!()
                        ))
                        .file_static(Some($crate::debug::__logging_private::file!()))
                        .line(Some($crate::debug::__logging_private::line!()))
                        .build();
                    $crate::debug::__logging_private::DebugState::log_impl(ID, ns_src_lvl::SOURCE, ns_type::TYPE, &record);
                }
                imp($crate::debug::__logging_private::format_args!($($rest)+));
            };
        };
        // gl_log!(ty: UndefinedBehavior, level: Info, target: "test", "this is a message from {}", "the OxideGL logger!");
        // gl_log!(ty: UndefinedBehavior, level: Info, "this is a message from {}", "the OxideGL logger!");
        (ty: $ty:ident, level: $level:expr, $($rest:tt)+) => {
            $crate::debug::__logging_private::gl_log!(src: None, ty: $ty, level: $level, $($rest)+);
        };
        // gl_log!(src: Api, level: Info, target: "test", "this is a message from {}", "the OxideGL logger!");
        // gl_log!(src: Api, level: Info, "this is a message from {}", "the OxideGL logger!");
        (src: $src:ident, level: $level:expr, $($rest:tt)+) => {
            $crate::debug::__logging_private::gl_log!(src: $src, ty: None, level: $level, $($rest)+);
        };
        // gl_log!(level: Info, target: "test", "this is a message from {}", "the OxideGL logger!");
        // gl_log!(level: Info, "this is a message from {}", "the OxideGL logger!");
        (level: $level:expr, $($rest:tt)+) => {
            $crate::debug::__logging_private::gl_log!(src: None, ty: None, level: $level, $($rest)+);
        };
    }
    pub(crate) use gl_log;
    // Thanks to yandros for pointers on refining the $_dollar trick
    macro_rules! gen_log_macros {(
        #![dollar = $_:tt]
        $(
            $( #[doc = $doc:expr] )+
            $lvl:ident => macro_rules! $name:ident
        ),+
        $(,)?
    ) => (
        $(
            $( #[doc = $doc] )+
            #[allow(unused)]
            macro_rules! $name {
                ( src: $_src:ident, ty: $_ty:ident, $_($_rest:tt)+ ) => (
                    $_ crate::debug::__logging_private::gl_log! { src: $_src, ty: $_ty, level: $lvl, $_($_rest)+ }
                );
                ( ty: $_ty:ident, $_($_rest:tt)+ ) => (
                    $_ crate::debug::__logging_private::gl_log! { ty: $_ty, level: $lvl, $_($_rest)+ }
                );
                ( src: $_src:ident, $_($_rest:tt)+ ) => (
                    $_ crate::debug::__logging_private::gl_log! { src: $_src, level: $lvl, $_($_rest)+ }
                );
                ( $_($_rest:tt)+ ) => (
                    $_ crate::debug::__logging_private::gl_log! {level: $lvl, $_($_rest)+ }
                );
            }

            pub(crate) use $name;
        )+
    )}
    gen_log_macros! {
        #![dollar = $]
        /// [`gl_log!`] but with its `level` argument preset to Trace. See the docs for [`gl_log!`] for more details
        Trace => macro_rules! gl_trace,
        /// [`gl_log!`] but with its `level` argument preset to Debug. See the docs for [`gl_log!`] for more details
        Debug => macro_rules! gl_debug,
        /// [`gl_log!`] but with its `level` argument preset to Info. See the docs for [`gl_log!`] for more details
        Info => macro_rules! gl_info,
        /// [`gl_log!`] but with its `level` argument preset to Warn. See the docs for [`gl_log!`] for more details
        Warn => macro_rules! gl_warn,
        /// [`gl_log!`] but with its `level` argument preset to Error. See the docs for [`gl_log!`] for more details
        Error => macro_rules! gl_err,
    }
}

#[doc(hidden)]
pub mod __logging_private {
    pub(crate) use crate::debug::macros::gl_log;
    pub use crate::gl_enums::DebugSource;
    pub use crate::gl_enums::DebugType;

    pub use super::DebugState;
    pub use core::file;
    pub use core::format_args;
    pub use core::line;
    pub use core::module_path;
    pub use log::Level;
    pub use log::Record;
    pub(crate) const fn make_message_id(file: &'static str, mut line: u32) -> u32 {
        let Some(fileid) = FNAME_LOOKUP.get(file) else {
            panic!("Failed to get file ID when trying to generate message id for log call");
        };
        assert!(
            line <= u16::MAX as u32 + 1,
            "Line number too large when trying to generate message id for log call at"
        );
        line -= 1;
        line | ((fileid as u32) << 16)
    }
    pub use core::fmt::Arguments;
    include!(concat!(env!("OUT_DIR"), "/generated.rs"));
    struct ConstStrToU16Map<const N: usize> {
        keys: [&'static str; N],
        vals: [u16; N],
    }
    impl<const N: usize> ConstStrToU16Map<N> {
        const fn get(&self, key: &'static str) -> Option<u16> {
            let mut low = 0;
            let mut high = self.keys.len() - 1;

            while low <= high {
                let mid = (low + high) / 2;
                if compare_strings(self.keys[mid], key) == 0 {
                    return Some(self.vals[mid]);
                } else if compare_strings(key, self.keys[mid]) == -1 {
                    high = mid - 1;
                } else {
                    low = mid + 1;
                }
            }

            Option::None
        }
    }
    const fn min_usize(a: usize, b: usize) -> usize {
        if a < b { a } else { b }
    }
    /// Returns: 1 if a > b, 0 if a = b, -1 if a < b
    const fn compare_strings(a: &str, b: &str) -> i32 {
        let (a, b) = (a.as_bytes(), b.as_bytes());
        let max_idx = min_usize(a.len(), b.len());
        let mut i = 0;
        while i < max_idx {
            if a[i] > b[i] {
                return 1;
            }
            if a[i] < b[i] {
                return -1;
            }
            i += 1;
        }
        if a.len() > b.len() {
            return 1;
        }
        if a.len() < b.len() {
            return -1;
        }
        0
    }
}
