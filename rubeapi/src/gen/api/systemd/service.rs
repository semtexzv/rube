#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(unused_must_use)]
#![allow(clippy::derive_partial_eq_without_eq)]
use std::fmt::Write;

use ::protokit as root;
use ::protokit::*;
pub fn register_types(registry: &mut reflect::Registry) {
    registry.register(&Unit::default());
    registry.register(&Service::default());
    registry.register(&ServiceExec::default());
    registry.register(&ServiceTime::default());
    registry.register(&ServiceStatus::default());
    registry.register(&Socket::default());
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Unit {
    pub description: String,
    pub doc_url: Vec<String>,
    pub wants: Vec<String>,
    pub requires: Vec<String>,
    pub requisite: Vec<String>,
    pub binds_to: Vec<String>,
    pub part_of: Vec<String>,
    pub enabled: bool,
    pub details: UnitOneOfDetails,
    pub _unknown: (),
}
impl Unit {
    #[inline(always)]
    pub fn r#with_description(mut self, it: String) -> Self {
        self.r#set_description(it);
        self
    }
    #[inline(always)]
    pub fn r#set_description(&mut self, it: String) -> &mut Self {
        self.description = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_doc_url(mut self, it: String) -> Self {
        self.r#add_doc_url(it);
        self
    }
    #[inline(always)]
    pub fn r#add_doc_url(&mut self, it: String) -> &mut Self {
        self.doc_url.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_wants(mut self, it: String) -> Self {
        self.r#add_wants(it);
        self
    }
    #[inline(always)]
    pub fn r#add_wants(&mut self, it: String) -> &mut Self {
        self.wants.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_requires(mut self, it: String) -> Self {
        self.r#add_requires(it);
        self
    }
    #[inline(always)]
    pub fn r#add_requires(&mut self, it: String) -> &mut Self {
        self.requires.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_requisite(mut self, it: String) -> Self {
        self.r#add_requisite(it);
        self
    }
    #[inline(always)]
    pub fn r#add_requisite(&mut self, it: String) -> &mut Self {
        self.requisite.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_binds_to(mut self, it: String) -> Self {
        self.r#add_binds_to(it);
        self
    }
    #[inline(always)]
    pub fn r#add_binds_to(&mut self, it: String) -> &mut Self {
        self.binds_to.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_part_of(mut self, it: String) -> Self {
        self.r#add_part_of(it);
        self
    }
    #[inline(always)]
    pub fn r#add_part_of(&mut self, it: String) -> &mut Self {
        self.part_of.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_enabled(mut self, it: bool) -> Self {
        self.r#set_enabled(it);
        self
    }
    #[inline(always)]
    pub fn r#set_enabled(&mut self, it: bool) -> &mut Self {
        self.enabled = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_details_service(mut self, it: Service) -> Self {
        self.details = UnitOneOfDetails::Service(it);
        self
    }
    #[inline(always)]
    pub fn r#set_details_service(&mut self, it: Service) -> &mut Self {
        self.details = UnitOneOfDetails::Service(it);
        self
    }
    #[inline(always)]
    pub fn r#with_details_socket(mut self, it: Socket) -> Self {
        self.details = UnitOneOfDetails::Socket(it);
        self
    }
    #[inline(always)]
    pub fn r#set_details_socket(&mut self, it: Socket) -> &mut Self {
        self.details = UnitOneOfDetails::Socket(it);
        self
    }
}
impl textformat::Decodable for Unit {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("description") => {
                textformat::Field::merge(&mut self.description, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("doc_url") => {
                textformat::Field::merge(&mut self.doc_url, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("wants") => {
                textformat::Field::merge(&mut self.wants, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("requires") => {
                textformat::Field::merge(&mut self.requires, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("requisite") => {
                textformat::Field::merge(&mut self.requisite, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("binds_to") => {
                textformat::Field::merge(&mut self.binds_to, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("part_of") => {
                textformat::Field::merge(&mut self.part_of, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("enabled") => {
                textformat::Field::merge(&mut self.enabled, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("service") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self.details = UnitOneOfDetails::Service(target);
            }
            textformat::ast::FieldName::Normal("socket") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self.details = UnitOneOfDetails::Socket(target);
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for Unit {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.description != <String as Default>::default() {
            out.indent(pad);
            out.push_str("description: ");
            textformat::Field::format(&self.description, ctx, pad, out)?;
            out.push('\n');
        }
        if self.doc_url != <Vec<String> as Default>::default() {
            out.indent(pad);
            out.push_str("doc_url: ");
            textformat::Field::format(&self.doc_url, ctx, pad, out)?;
            out.push('\n');
        }
        if self.wants != <Vec<String> as Default>::default() {
            out.indent(pad);
            out.push_str("wants: ");
            textformat::Field::format(&self.wants, ctx, pad, out)?;
            out.push('\n');
        }
        if self.requires != <Vec<String> as Default>::default() {
            out.indent(pad);
            out.push_str("requires: ");
            textformat::Field::format(&self.requires, ctx, pad, out)?;
            out.push('\n');
        }
        if self.requisite != <Vec<String> as Default>::default() {
            out.indent(pad);
            out.push_str("requisite: ");
            textformat::Field::format(&self.requisite, ctx, pad, out)?;
            out.push('\n');
        }
        if self.binds_to != <Vec<String> as Default>::default() {
            out.indent(pad);
            out.push_str("binds_to: ");
            textformat::Field::format(&self.binds_to, ctx, pad, out)?;
            out.push('\n');
        }
        if self.part_of != <Vec<String> as Default>::default() {
            out.indent(pad);
            out.push_str("part_of: ");
            textformat::Field::format(&self.part_of, ctx, pad, out)?;
            out.push('\n');
        }
        if self.enabled != <bool as Default>::default() {
            out.indent(pad);
            out.push_str("enabled: ");
            textformat::Field::format(&self.enabled, ctx, pad, out)?;
            out.push('\n');
        }
        match &self.details {
            UnitOneOfDetails::Service(value) => {
                out.indent(pad);
                out.push_str("service ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            UnitOneOfDetails::Socket(value) => {
                out.indent(pad);
                out.push_str("socket ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            UnitOneOfDetails::Unknown(..) => {}
        }
        Ok(())
    }
}
impl binformat::Decodable for Unit {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: &'b [u8],
    ) -> binformat::Result<&'b [u8]> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Decode::<Bytes>::decode(&mut self.description, buf)?;
            }
            18u32 => {
                buf = Decode::<Repeat<Bytes>>::decode(&mut self.doc_url, buf)?;
            }
            26u32 => {
                buf = Decode::<Repeat<Bytes>>::decode(&mut self.wants, buf)?;
            }
            34u32 => {
                buf = Decode::<Repeat<Bytes>>::decode(&mut self.requires, buf)?;
            }
            42u32 => {
                buf = Decode::<Repeat<Bytes>>::decode(&mut self.requisite, buf)?;
            }
            50u32 => {
                buf = Decode::<Repeat<Bytes>>::decode(&mut self.binds_to, buf)?;
            }
            58u32 => {
                buf = Decode::<Repeat<Bytes>>::decode(&mut self.part_of, buf)?;
            }
            64u32 => {
                buf = Decode::<Fix>::decode(&mut self.enabled, buf)?;
            }
            66u32 => {
                buf = Decode::<Fix>::decode(&mut self.enabled, buf)?;
            }
            162u32 => {
                let mut tmp = Default::default();
                buf = Decode::<Nest>::decode(&mut tmp, buf)?;
                self.details = UnitOneOfDetails::Service(tmp);
            }
            170u32 => {
                let mut tmp = Default::default();
                buf = Decode::<Nest>::decode(&mut tmp, buf)?;
                self.details = UnitOneOfDetails::Socket(tmp);
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for Unit {
    fn qualified_name(&self) -> &'static str {
        "api.systemd.Unit"
    }
    fn encode(&self, buf: &mut Vec<u8>) -> binformat::Result<()> {
        use binformat::format::*;
        Decode::<Bytes>::encode(&self.description, 10u32, buf)?;
        Decode::<Repeat<Bytes>>::encode(&self.doc_url, 18u32, buf)?;
        Decode::<Repeat<Bytes>>::encode(&self.wants, 26u32, buf)?;
        Decode::<Repeat<Bytes>>::encode(&self.requires, 34u32, buf)?;
        Decode::<Repeat<Bytes>>::encode(&self.requisite, 42u32, buf)?;
        Decode::<Repeat<Bytes>>::encode(&self.binds_to, 50u32, buf)?;
        Decode::<Repeat<Bytes>>::encode(&self.part_of, 58u32, buf)?;
        Decode::<Fix>::encode(&self.enabled, 64u32, buf)?;
        match &self.details {
            UnitOneOfDetails::Service(value) => {
                Decode::<Nest>::encode(value, 162u32, buf)?;
            }
            UnitOneOfDetails::Socket(value) => {
                Decode::<Nest>::encode(value, 170u32, buf)?;
            }
            UnitOneOfDetails::Unknown(..) => {}
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C, u32)]
#[derive(Debug, Clone, PartialEq)]
pub enum UnitOneOfDetails {
    Service(Service),
    Socket(Socket),
    Unknown(::core::marker::PhantomData<()>),
}
impl Default for UnitOneOfDetails {
    fn default() -> Self {
        UnitOneOfDetails::Unknown(::core::marker::PhantomData)
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Service {
    pub title: String,
    pub r#type: ServiceType,
    pub exit_type: ServiceExitType,
    pub remain_after_exit: bool,
    pub guess_pid: bool,
    pub pid_file: String,
    pub bus_name: String,
    pub exec: Option<Box<ServiceExec>>,
    pub time: Option<Box<ServiceTime>>,
    pub restart: ServiceRestart,
    pub status: Option<Box<ServiceStatus>>,
    pub non_blocking: bool,
    pub notify_access: ServiceNotifyAccess,
    pub sockets: Vec<String>,
    pub oom_policy: ServiceOOMPolicy,
    pub _unknown: (),
}
impl Service {
    #[inline(always)]
    pub fn r#with_title(mut self, it: String) -> Self {
        self.r#set_title(it);
        self
    }
    #[inline(always)]
    pub fn r#set_title(&mut self, it: String) -> &mut Self {
        self.title = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_type(mut self, it: ServiceType) -> Self {
        self.r#set_type(it);
        self
    }
    #[inline(always)]
    pub fn r#set_type(&mut self, it: ServiceType) -> &mut Self {
        self.r#type = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_exit_type(mut self, it: ServiceExitType) -> Self {
        self.r#set_exit_type(it);
        self
    }
    #[inline(always)]
    pub fn r#set_exit_type(&mut self, it: ServiceExitType) -> &mut Self {
        self.exit_type = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_remain_after_exit(mut self, it: bool) -> Self {
        self.r#set_remain_after_exit(it);
        self
    }
    #[inline(always)]
    pub fn r#set_remain_after_exit(&mut self, it: bool) -> &mut Self {
        self.remain_after_exit = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_guess_pid(mut self, it: bool) -> Self {
        self.r#set_guess_pid(it);
        self
    }
    #[inline(always)]
    pub fn r#set_guess_pid(&mut self, it: bool) -> &mut Self {
        self.guess_pid = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_pid_file(mut self, it: String) -> Self {
        self.r#set_pid_file(it);
        self
    }
    #[inline(always)]
    pub fn r#set_pid_file(&mut self, it: String) -> &mut Self {
        self.pid_file = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_bus_name(mut self, it: String) -> Self {
        self.r#set_bus_name(it);
        self
    }
    #[inline(always)]
    pub fn r#set_bus_name(&mut self, it: String) -> &mut Self {
        self.bus_name = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_exec(mut self, it: ServiceExec) -> Self {
        self.r#set_exec(it);
        self
    }
    #[inline(always)]
    pub fn r#set_exec(&mut self, it: ServiceExec) -> &mut Self {
        self.exec = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_time(mut self, it: ServiceTime) -> Self {
        self.r#set_time(it);
        self
    }
    #[inline(always)]
    pub fn r#set_time(&mut self, it: ServiceTime) -> &mut Self {
        self.time = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_restart(mut self, it: ServiceRestart) -> Self {
        self.r#set_restart(it);
        self
    }
    #[inline(always)]
    pub fn r#set_restart(&mut self, it: ServiceRestart) -> &mut Self {
        self.restart = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_status(mut self, it: ServiceStatus) -> Self {
        self.r#set_status(it);
        self
    }
    #[inline(always)]
    pub fn r#set_status(&mut self, it: ServiceStatus) -> &mut Self {
        self.status = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_non_blocking(mut self, it: bool) -> Self {
        self.r#set_non_blocking(it);
        self
    }
    #[inline(always)]
    pub fn r#set_non_blocking(&mut self, it: bool) -> &mut Self {
        self.non_blocking = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_notify_access(mut self, it: ServiceNotifyAccess) -> Self {
        self.r#set_notify_access(it);
        self
    }
    #[inline(always)]
    pub fn r#set_notify_access(&mut self, it: ServiceNotifyAccess) -> &mut Self {
        self.notify_access = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_sockets(mut self, it: String) -> Self {
        self.r#add_sockets(it);
        self
    }
    #[inline(always)]
    pub fn r#add_sockets(&mut self, it: String) -> &mut Self {
        self.sockets.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_oom_policy(mut self, it: ServiceOOMPolicy) -> Self {
        self.r#set_oom_policy(it);
        self
    }
    #[inline(always)]
    pub fn r#set_oom_policy(&mut self, it: ServiceOOMPolicy) -> &mut Self {
        self.oom_policy = it.into();
        self
    }
}
impl textformat::Decodable for Service {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("title") => {
                textformat::Field::merge(&mut self.title, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("type") => {
                textformat::Field::merge(&mut self.r#type, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("exit_type") => {
                textformat::Field::merge(&mut self.exit_type, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("remain_after_exit") => {
                textformat::Field::merge(&mut self.remain_after_exit, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("guess_pid") => {
                textformat::Field::merge(&mut self.guess_pid, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("pid_file") => {
                textformat::Field::merge(&mut self.pid_file, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("bus_name") => {
                textformat::Field::merge(&mut self.bus_name, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("exec") => {
                textformat::Field::merge(&mut self.exec, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("time") => {
                textformat::Field::merge(&mut self.time, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("restart") => {
                textformat::Field::merge(&mut self.restart, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("status") => {
                textformat::Field::merge(&mut self.status, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("non_blocking") => {
                textformat::Field::merge(&mut self.non_blocking, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("notify_access") => {
                textformat::Field::merge(&mut self.notify_access, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("sockets") => {
                textformat::Field::merge(&mut self.sockets, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("oom_policy") => {
                textformat::Field::merge(&mut self.oom_policy, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for Service {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.title != <String as Default>::default() {
            out.indent(pad);
            out.push_str("title: ");
            textformat::Field::format(&self.title, ctx, pad, out)?;
            out.push('\n');
        }
        if self.r#type != <ServiceType as Default>::default() {
            out.indent(pad);
            out.push_str("type: ");
            textformat::Field::format(&self.r#type, ctx, pad, out)?;
            out.push('\n');
        }
        if self.exit_type != <ServiceExitType as Default>::default() {
            out.indent(pad);
            out.push_str("exit_type: ");
            textformat::Field::format(&self.exit_type, ctx, pad, out)?;
            out.push('\n');
        }
        if self.remain_after_exit != <bool as Default>::default() {
            out.indent(pad);
            out.push_str("remain_after_exit: ");
            textformat::Field::format(&self.remain_after_exit, ctx, pad, out)?;
            out.push('\n');
        }
        if self.guess_pid != <bool as Default>::default() {
            out.indent(pad);
            out.push_str("guess_pid: ");
            textformat::Field::format(&self.guess_pid, ctx, pad, out)?;
            out.push('\n');
        }
        if self.pid_file != <String as Default>::default() {
            out.indent(pad);
            out.push_str("pid_file: ");
            textformat::Field::format(&self.pid_file, ctx, pad, out)?;
            out.push('\n');
        }
        if self.bus_name != <String as Default>::default() {
            out.indent(pad);
            out.push_str("bus_name: ");
            textformat::Field::format(&self.bus_name, ctx, pad, out)?;
            out.push('\n');
        }
        if self.exec != <Option<Box<ServiceExec>> as Default>::default() {
            out.indent(pad);
            out.push_str("exec ");
            textformat::Field::format(&self.exec, ctx, pad, out)?;
            out.push('\n');
        }
        if self.time != <Option<Box<ServiceTime>> as Default>::default() {
            out.indent(pad);
            out.push_str("time ");
            textformat::Field::format(&self.time, ctx, pad, out)?;
            out.push('\n');
        }
        if self.restart != <ServiceRestart as Default>::default() {
            out.indent(pad);
            out.push_str("restart: ");
            textformat::Field::format(&self.restart, ctx, pad, out)?;
            out.push('\n');
        }
        if self.status != <Option<Box<ServiceStatus>> as Default>::default() {
            out.indent(pad);
            out.push_str("status ");
            textformat::Field::format(&self.status, ctx, pad, out)?;
            out.push('\n');
        }
        if self.non_blocking != <bool as Default>::default() {
            out.indent(pad);
            out.push_str("non_blocking: ");
            textformat::Field::format(&self.non_blocking, ctx, pad, out)?;
            out.push('\n');
        }
        if self.notify_access != <ServiceNotifyAccess as Default>::default() {
            out.indent(pad);
            out.push_str("notify_access: ");
            textformat::Field::format(&self.notify_access, ctx, pad, out)?;
            out.push('\n');
        }
        if self.sockets != <Vec<String> as Default>::default() {
            out.indent(pad);
            out.push_str("sockets: ");
            textformat::Field::format(&self.sockets, ctx, pad, out)?;
            out.push('\n');
        }
        if self.oom_policy != <ServiceOOMPolicy as Default>::default() {
            out.indent(pad);
            out.push_str("oom_policy: ");
            textformat::Field::format(&self.oom_policy, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for Service {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: &'b [u8],
    ) -> binformat::Result<&'b [u8]> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Decode::<Bytes>::decode(&mut self.title, buf)?;
            }
            16u32 => {
                buf = Decode::<Enum>::decode(&mut self.r#type, buf)?;
            }
            18u32 => {
                buf = Decode::<Enum>::decode(&mut self.r#type, buf)?;
            }
            24u32 => {
                buf = Decode::<Enum>::decode(&mut self.exit_type, buf)?;
            }
            26u32 => {
                buf = Decode::<Enum>::decode(&mut self.exit_type, buf)?;
            }
            32u32 => {
                buf = Decode::<Fix>::decode(&mut self.remain_after_exit, buf)?;
            }
            34u32 => {
                buf = Decode::<Fix>::decode(&mut self.remain_after_exit, buf)?;
            }
            40u32 => {
                buf = Decode::<Fix>::decode(&mut self.guess_pid, buf)?;
            }
            42u32 => {
                buf = Decode::<Fix>::decode(&mut self.guess_pid, buf)?;
            }
            50u32 => {
                buf = Decode::<Bytes>::decode(&mut self.pid_file, buf)?;
            }
            58u32 => {
                buf = Decode::<Bytes>::decode(&mut self.bus_name, buf)?;
            }
            66u32 => {
                buf = Decode::<Nest>::decode(&mut self.exec, buf)?;
            }
            74u32 => {
                buf = Decode::<Nest>::decode(&mut self.time, buf)?;
            }
            80u32 => {
                buf = Decode::<Enum>::decode(&mut self.restart, buf)?;
            }
            82u32 => {
                buf = Decode::<Enum>::decode(&mut self.restart, buf)?;
            }
            90u32 => {
                buf = Decode::<Nest>::decode(&mut self.status, buf)?;
            }
            96u32 => {
                buf = Decode::<Fix>::decode(&mut self.non_blocking, buf)?;
            }
            98u32 => {
                buf = Decode::<Fix>::decode(&mut self.non_blocking, buf)?;
            }
            104u32 => {
                buf = Decode::<Enum>::decode(&mut self.notify_access, buf)?;
            }
            106u32 => {
                buf = Decode::<Enum>::decode(&mut self.notify_access, buf)?;
            }
            114u32 => {
                buf = Decode::<Repeat<Bytes>>::decode(&mut self.sockets, buf)?;
            }
            120u32 => {
                buf = Decode::<Enum>::decode(&mut self.oom_policy, buf)?;
            }
            122u32 => {
                buf = Decode::<Enum>::decode(&mut self.oom_policy, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for Service {
    fn qualified_name(&self) -> &'static str {
        "api.systemd.Service"
    }
    fn encode(&self, buf: &mut Vec<u8>) -> binformat::Result<()> {
        use binformat::format::*;
        Decode::<Bytes>::encode(&self.title, 10u32, buf)?;
        Decode::<Enum>::encode(&self.r#type, 16u32, buf)?;
        Decode::<Enum>::encode(&self.exit_type, 24u32, buf)?;
        Decode::<Fix>::encode(&self.remain_after_exit, 32u32, buf)?;
        Decode::<Fix>::encode(&self.guess_pid, 40u32, buf)?;
        Decode::<Bytes>::encode(&self.pid_file, 50u32, buf)?;
        Decode::<Bytes>::encode(&self.bus_name, 58u32, buf)?;
        Decode::<Nest>::encode(&self.exec, 66u32, buf)?;
        Decode::<Nest>::encode(&self.time, 74u32, buf)?;
        Decode::<Enum>::encode(&self.restart, 80u32, buf)?;
        Decode::<Nest>::encode(&self.status, 90u32, buf)?;
        Decode::<Fix>::encode(&self.non_blocking, 96u32, buf)?;
        Decode::<Enum>::encode(&self.notify_access, 104u32, buf)?;
        Decode::<Repeat<Bytes>>::encode(&self.sockets, 114u32, buf)?;
        Decode::<Enum>::encode(&self.oom_policy, 120u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ServiceExec {
    pub start: String,
    pub pre_start: Vec<String>,
    pub post_start: Vec<String>,
    pub condition: Vec<String>,
    pub reload: Vec<String>,
    pub stop: Vec<String>,
    pub post_stop: Vec<String>,
    pub _unknown: (),
}
impl ServiceExec {
    #[inline(always)]
    pub fn r#with_start(mut self, it: String) -> Self {
        self.r#set_start(it);
        self
    }
    #[inline(always)]
    pub fn r#set_start(&mut self, it: String) -> &mut Self {
        self.start = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_pre_start(mut self, it: String) -> Self {
        self.r#add_pre_start(it);
        self
    }
    #[inline(always)]
    pub fn r#add_pre_start(&mut self, it: String) -> &mut Self {
        self.pre_start.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_post_start(mut self, it: String) -> Self {
        self.r#add_post_start(it);
        self
    }
    #[inline(always)]
    pub fn r#add_post_start(&mut self, it: String) -> &mut Self {
        self.post_start.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_condition(mut self, it: String) -> Self {
        self.r#add_condition(it);
        self
    }
    #[inline(always)]
    pub fn r#add_condition(&mut self, it: String) -> &mut Self {
        self.condition.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_reload(mut self, it: String) -> Self {
        self.r#add_reload(it);
        self
    }
    #[inline(always)]
    pub fn r#add_reload(&mut self, it: String) -> &mut Self {
        self.reload.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_stop(mut self, it: String) -> Self {
        self.r#add_stop(it);
        self
    }
    #[inline(always)]
    pub fn r#add_stop(&mut self, it: String) -> &mut Self {
        self.stop.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_post_stop(mut self, it: String) -> Self {
        self.r#add_post_stop(it);
        self
    }
    #[inline(always)]
    pub fn r#add_post_stop(&mut self, it: String) -> &mut Self {
        self.post_stop.push(it);
        self
    }
}
impl textformat::Decodable for ServiceExec {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("start") => {
                textformat::Field::merge(&mut self.start, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("pre_start") => {
                textformat::Field::merge(&mut self.pre_start, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("post_start") => {
                textformat::Field::merge(&mut self.post_start, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("condition") => {
                textformat::Field::merge(&mut self.condition, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("reload") => {
                textformat::Field::merge(&mut self.reload, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("stop") => {
                textformat::Field::merge(&mut self.stop, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("post_stop") => {
                textformat::Field::merge(&mut self.post_stop, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for ServiceExec {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.start != <String as Default>::default() {
            out.indent(pad);
            out.push_str("start: ");
            textformat::Field::format(&self.start, ctx, pad, out)?;
            out.push('\n');
        }
        if self.pre_start != <Vec<String> as Default>::default() {
            out.indent(pad);
            out.push_str("pre_start: ");
            textformat::Field::format(&self.pre_start, ctx, pad, out)?;
            out.push('\n');
        }
        if self.post_start != <Vec<String> as Default>::default() {
            out.indent(pad);
            out.push_str("post_start: ");
            textformat::Field::format(&self.post_start, ctx, pad, out)?;
            out.push('\n');
        }
        if self.condition != <Vec<String> as Default>::default() {
            out.indent(pad);
            out.push_str("condition: ");
            textformat::Field::format(&self.condition, ctx, pad, out)?;
            out.push('\n');
        }
        if self.reload != <Vec<String> as Default>::default() {
            out.indent(pad);
            out.push_str("reload: ");
            textformat::Field::format(&self.reload, ctx, pad, out)?;
            out.push('\n');
        }
        if self.stop != <Vec<String> as Default>::default() {
            out.indent(pad);
            out.push_str("stop: ");
            textformat::Field::format(&self.stop, ctx, pad, out)?;
            out.push('\n');
        }
        if self.post_stop != <Vec<String> as Default>::default() {
            out.indent(pad);
            out.push_str("post_stop: ");
            textformat::Field::format(&self.post_stop, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for ServiceExec {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: &'b [u8],
    ) -> binformat::Result<&'b [u8]> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Decode::<Bytes>::decode(&mut self.start, buf)?;
            }
            18u32 => {
                buf = Decode::<Repeat<Bytes>>::decode(&mut self.pre_start, buf)?;
            }
            26u32 => {
                buf = Decode::<Repeat<Bytes>>::decode(&mut self.post_start, buf)?;
            }
            34u32 => {
                buf = Decode::<Repeat<Bytes>>::decode(&mut self.condition, buf)?;
            }
            42u32 => {
                buf = Decode::<Repeat<Bytes>>::decode(&mut self.reload, buf)?;
            }
            50u32 => {
                buf = Decode::<Repeat<Bytes>>::decode(&mut self.stop, buf)?;
            }
            58u32 => {
                buf = Decode::<Repeat<Bytes>>::decode(&mut self.post_stop, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for ServiceExec {
    fn qualified_name(&self) -> &'static str {
        "api.systemd.Service.Exec"
    }
    fn encode(&self, buf: &mut Vec<u8>) -> binformat::Result<()> {
        use binformat::format::*;
        Decode::<Bytes>::encode(&self.start, 10u32, buf)?;
        Decode::<Repeat<Bytes>>::encode(&self.pre_start, 18u32, buf)?;
        Decode::<Repeat<Bytes>>::encode(&self.post_start, 26u32, buf)?;
        Decode::<Repeat<Bytes>>::encode(&self.condition, 34u32, buf)?;
        Decode::<Repeat<Bytes>>::encode(&self.reload, 42u32, buf)?;
        Decode::<Repeat<Bytes>>::encode(&self.stop, 50u32, buf)?;
        Decode::<Repeat<Bytes>>::encode(&self.post_stop, 58u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ServiceTime {
    pub restart: i32,
    pub start_timeout: i32,
    pub stop_timeout: i32,
    pub abort_timeout: i32,
    pub timeout: i32,
    pub limit: i32,
    pub watchdog: i32,
    pub _unknown: (),
}
impl ServiceTime {
    #[inline(always)]
    pub fn r#with_restart(mut self, it: i32) -> Self {
        self.r#set_restart(it);
        self
    }
    #[inline(always)]
    pub fn r#set_restart(&mut self, it: i32) -> &mut Self {
        self.restart = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_start_timeout(mut self, it: i32) -> Self {
        self.r#set_start_timeout(it);
        self
    }
    #[inline(always)]
    pub fn r#set_start_timeout(&mut self, it: i32) -> &mut Self {
        self.start_timeout = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_stop_timeout(mut self, it: i32) -> Self {
        self.r#set_stop_timeout(it);
        self
    }
    #[inline(always)]
    pub fn r#set_stop_timeout(&mut self, it: i32) -> &mut Self {
        self.stop_timeout = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_abort_timeout(mut self, it: i32) -> Self {
        self.r#set_abort_timeout(it);
        self
    }
    #[inline(always)]
    pub fn r#set_abort_timeout(&mut self, it: i32) -> &mut Self {
        self.abort_timeout = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_timeout(mut self, it: i32) -> Self {
        self.r#set_timeout(it);
        self
    }
    #[inline(always)]
    pub fn r#set_timeout(&mut self, it: i32) -> &mut Self {
        self.timeout = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_limit(mut self, it: i32) -> Self {
        self.r#set_limit(it);
        self
    }
    #[inline(always)]
    pub fn r#set_limit(&mut self, it: i32) -> &mut Self {
        self.limit = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_watchdog(mut self, it: i32) -> Self {
        self.r#set_watchdog(it);
        self
    }
    #[inline(always)]
    pub fn r#set_watchdog(&mut self, it: i32) -> &mut Self {
        self.watchdog = it.into();
        self
    }
}
impl textformat::Decodable for ServiceTime {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("restart") => {
                textformat::Field::merge(&mut self.restart, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("start_timeout") => {
                textformat::Field::merge(&mut self.start_timeout, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("stop_timeout") => {
                textformat::Field::merge(&mut self.stop_timeout, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("abort_timeout") => {
                textformat::Field::merge(&mut self.abort_timeout, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("timeout") => {
                textformat::Field::merge(&mut self.timeout, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("limit") => {
                textformat::Field::merge(&mut self.limit, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("watchdog") => {
                textformat::Field::merge(&mut self.watchdog, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for ServiceTime {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.restart != <i32 as Default>::default() {
            out.indent(pad);
            out.push_str("restart: ");
            textformat::Field::format(&self.restart, ctx, pad, out)?;
            out.push('\n');
        }
        if self.start_timeout != <i32 as Default>::default() {
            out.indent(pad);
            out.push_str("start_timeout: ");
            textformat::Field::format(&self.start_timeout, ctx, pad, out)?;
            out.push('\n');
        }
        if self.stop_timeout != <i32 as Default>::default() {
            out.indent(pad);
            out.push_str("stop_timeout: ");
            textformat::Field::format(&self.stop_timeout, ctx, pad, out)?;
            out.push('\n');
        }
        if self.abort_timeout != <i32 as Default>::default() {
            out.indent(pad);
            out.push_str("abort_timeout: ");
            textformat::Field::format(&self.abort_timeout, ctx, pad, out)?;
            out.push('\n');
        }
        if self.timeout != <i32 as Default>::default() {
            out.indent(pad);
            out.push_str("timeout: ");
            textformat::Field::format(&self.timeout, ctx, pad, out)?;
            out.push('\n');
        }
        if self.limit != <i32 as Default>::default() {
            out.indent(pad);
            out.push_str("limit: ");
            textformat::Field::format(&self.limit, ctx, pad, out)?;
            out.push('\n');
        }
        if self.watchdog != <i32 as Default>::default() {
            out.indent(pad);
            out.push_str("watchdog: ");
            textformat::Field::format(&self.watchdog, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for ServiceTime {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: &'b [u8],
    ) -> binformat::Result<&'b [u8]> {
        use binformat::format::*;
        match tag {
            8u32 => {
                buf = Decode::<VInt>::decode(&mut self.restart, buf)?;
            }
            10u32 => {
                buf = Decode::<VInt>::decode(&mut self.restart, buf)?;
            }
            16u32 => {
                buf = Decode::<VInt>::decode(&mut self.start_timeout, buf)?;
            }
            18u32 => {
                buf = Decode::<VInt>::decode(&mut self.start_timeout, buf)?;
            }
            24u32 => {
                buf = Decode::<VInt>::decode(&mut self.stop_timeout, buf)?;
            }
            26u32 => {
                buf = Decode::<VInt>::decode(&mut self.stop_timeout, buf)?;
            }
            32u32 => {
                buf = Decode::<VInt>::decode(&mut self.abort_timeout, buf)?;
            }
            34u32 => {
                buf = Decode::<VInt>::decode(&mut self.abort_timeout, buf)?;
            }
            40u32 => {
                buf = Decode::<VInt>::decode(&mut self.timeout, buf)?;
            }
            42u32 => {
                buf = Decode::<VInt>::decode(&mut self.timeout, buf)?;
            }
            48u32 => {
                buf = Decode::<VInt>::decode(&mut self.limit, buf)?;
            }
            50u32 => {
                buf = Decode::<VInt>::decode(&mut self.limit, buf)?;
            }
            56u32 => {
                buf = Decode::<VInt>::decode(&mut self.watchdog, buf)?;
            }
            58u32 => {
                buf = Decode::<VInt>::decode(&mut self.watchdog, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for ServiceTime {
    fn qualified_name(&self) -> &'static str {
        "api.systemd.Service.Time"
    }
    fn encode(&self, buf: &mut Vec<u8>) -> binformat::Result<()> {
        use binformat::format::*;
        Decode::<VInt>::encode(&self.restart, 8u32, buf)?;
        Decode::<VInt>::encode(&self.start_timeout, 16u32, buf)?;
        Decode::<VInt>::encode(&self.stop_timeout, 24u32, buf)?;
        Decode::<VInt>::encode(&self.abort_timeout, 32u32, buf)?;
        Decode::<VInt>::encode(&self.timeout, 40u32, buf)?;
        Decode::<VInt>::encode(&self.limit, 48u32, buf)?;
        Decode::<VInt>::encode(&self.watchdog, 56u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ServiceStatus {
    pub success: Vec<i32>,
    pub prevent_restart: Vec<i32>,
    pub force_restart: Vec<i32>,
    pub _unknown: (),
}
impl ServiceStatus {
    #[inline(always)]
    pub fn r#with_success(mut self, it: i32) -> Self {
        self.r#add_success(it);
        self
    }
    #[inline(always)]
    pub fn r#add_success(&mut self, it: i32) -> &mut Self {
        self.success.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_prevent_restart(mut self, it: i32) -> Self {
        self.r#add_prevent_restart(it);
        self
    }
    #[inline(always)]
    pub fn r#add_prevent_restart(&mut self, it: i32) -> &mut Self {
        self.prevent_restart.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_force_restart(mut self, it: i32) -> Self {
        self.r#add_force_restart(it);
        self
    }
    #[inline(always)]
    pub fn r#add_force_restart(&mut self, it: i32) -> &mut Self {
        self.force_restart.push(it);
        self
    }
}
impl textformat::Decodable for ServiceStatus {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("success") => {
                textformat::Field::merge(&mut self.success, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("prevent_restart") => {
                textformat::Field::merge(&mut self.prevent_restart, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("force_restart") => {
                textformat::Field::merge(&mut self.force_restart, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for ServiceStatus {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.success != <Vec<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("success: ");
            textformat::Field::format(&self.success, ctx, pad, out)?;
            out.push('\n');
        }
        if self.prevent_restart != <Vec<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("prevent_restart: ");
            textformat::Field::format(&self.prevent_restart, ctx, pad, out)?;
            out.push('\n');
        }
        if self.force_restart != <Vec<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("force_restart: ");
            textformat::Field::format(&self.force_restart, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for ServiceStatus {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: &'b [u8],
    ) -> binformat::Result<&'b [u8]> {
        use binformat::format::*;
        match tag {
            8u32 => {
                buf = Decode::<Repeat<VInt>>::decode(&mut self.success, buf)?;
            }
            10u32 => {
                buf = Decode::<Pack<VInt>>::decode(&mut self.success, buf)?;
            }
            16u32 => {
                buf = Decode::<Repeat<VInt>>::decode(&mut self.prevent_restart, buf)?;
            }
            18u32 => {
                buf = Decode::<Pack<VInt>>::decode(&mut self.prevent_restart, buf)?;
            }
            24u32 => {
                buf = Decode::<Repeat<VInt>>::decode(&mut self.force_restart, buf)?;
            }
            26u32 => {
                buf = Decode::<Pack<VInt>>::decode(&mut self.force_restart, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for ServiceStatus {
    fn qualified_name(&self) -> &'static str {
        "api.systemd.Service.Status"
    }
    fn encode(&self, buf: &mut Vec<u8>) -> binformat::Result<()> {
        use binformat::format::*;
        Decode::<Repeat<VInt>>::encode(&self.success, 8u32, buf)?;
        Decode::<Repeat<VInt>>::encode(&self.prevent_restart, 16u32, buf)?;
        Decode::<Repeat<VInt>>::encode(&self.force_restart, 24u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Socket {
    pub path: String,
    pub _unknown: (),
}
impl Socket {
    #[inline(always)]
    pub fn r#with_path(mut self, it: String) -> Self {
        self.r#set_path(it);
        self
    }
    #[inline(always)]
    pub fn r#set_path(&mut self, it: String) -> &mut Self {
        self.path = it.into();
        self
    }
}
impl textformat::Decodable for Socket {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("path") => {
                textformat::Field::merge(&mut self.path, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for Socket {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.path != <String as Default>::default() {
            out.indent(pad);
            out.push_str("path: ");
            textformat::Field::format(&self.path, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for Socket {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: &'b [u8],
    ) -> binformat::Result<&'b [u8]> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Decode::<Bytes>::decode(&mut self.path, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for Socket {
    fn qualified_name(&self) -> &'static str {
        "api.systemd.Socket"
    }
    fn encode(&self, buf: &mut Vec<u8>) -> binformat::Result<()> {
        use binformat::format::*;
        Decode::<Bytes>::encode(&self.path, 10u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum ServiceType {
    SIMPLE,
    EXEC,
    FORKING,
    ONESHOT,
    DBUS,
    NOTIFY,
    IDLE,
    Unknown(u32),
}
impl Default for ServiceType {
    fn default() -> ServiceType {
        Self::from(0)
    }
}
impl binformat::format::ProtoEnum for ServiceType {}
impl From<u32> for ServiceType {
    fn from(v: u32) -> ServiceType {
        match v {
            0u32 => ServiceType::SIMPLE,
            1u32 => ServiceType::EXEC,
            2u32 => ServiceType::FORKING,
            3u32 => ServiceType::ONESHOT,
            4u32 => ServiceType::DBUS,
            5u32 => ServiceType::NOTIFY,
            6u32 => ServiceType::IDLE,
            other => ServiceType::Unknown(other),
        }
    }
}
impl Into<u32> for ServiceType {
    fn into(self) -> u32 {
        match self {
            ServiceType::SIMPLE => 0u32,
            ServiceType::EXEC => 1u32,
            ServiceType::FORKING => 2u32,
            ServiceType::ONESHOT => 3u32,
            ServiceType::DBUS => 4u32,
            ServiceType::NOTIFY => 5u32,
            ServiceType::IDLE => 6u32,
            ServiceType::Unknown(other) => other,
        }
    }
}
impl textformat::Field for ServiceType {
    fn format(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut String,
    ) -> ::std::fmt::Result {
        let str = match self {
            ServiceType::SIMPLE => "SIMPLE",
            ServiceType::EXEC => "EXEC",
            ServiceType::FORKING => "FORKING",
            ServiceType::ONESHOT => "ONESHOT",
            ServiceType::DBUS => "DBUS",
            ServiceType::NOTIFY => "NOTIFY",
            ServiceType::IDLE => "IDLE",
            ServiceType::Unknown(n) => {
                write!(out, "{n}")?;
                return Ok(());
            }
        };
        out.push_str(str);
        Ok(())
    }
    fn merge_scalar(
        &mut self,
        _ctx: &textformat::Context,
        v: &textformat::ast::Literal,
    ) -> textformat::Result<()> {
        match v {
            textformat::ast::Literal::Identifier("SIMPLE") => *self = ServiceType::SIMPLE,
            textformat::ast::Literal::Identifier("EXEC") => *self = ServiceType::EXEC,
            textformat::ast::Literal::Identifier("FORKING") => {
                *self = ServiceType::FORKING;
            }
            textformat::ast::Literal::Identifier("ONESHOT") => {
                *self = ServiceType::ONESHOT;
            }
            textformat::ast::Literal::Identifier("DBUS") => *self = ServiceType::DBUS,
            textformat::ast::Literal::Identifier("NOTIFY") => *self = ServiceType::NOTIFY,
            textformat::ast::Literal::Identifier("IDLE") => *self = ServiceType::IDLE,
            textformat::ast::Literal::Int(i) => *self = Self::from(*i as u32),
            other => textformat::bail!("Invalid enum value: {other:?}"),
        }
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum ServiceExitType {
    MAIN,
    CGROUP,
    Unknown(u32),
}
impl Default for ServiceExitType {
    fn default() -> ServiceExitType {
        Self::from(0)
    }
}
impl binformat::format::ProtoEnum for ServiceExitType {}
impl From<u32> for ServiceExitType {
    fn from(v: u32) -> ServiceExitType {
        match v {
            0u32 => ServiceExitType::MAIN,
            1u32 => ServiceExitType::CGROUP,
            other => ServiceExitType::Unknown(other),
        }
    }
}
impl Into<u32> for ServiceExitType {
    fn into(self) -> u32 {
        match self {
            ServiceExitType::MAIN => 0u32,
            ServiceExitType::CGROUP => 1u32,
            ServiceExitType::Unknown(other) => other,
        }
    }
}
impl textformat::Field for ServiceExitType {
    fn format(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut String,
    ) -> ::std::fmt::Result {
        let str = match self {
            ServiceExitType::MAIN => "MAIN",
            ServiceExitType::CGROUP => "CGROUP",
            ServiceExitType::Unknown(n) => {
                write!(out, "{n}")?;
                return Ok(());
            }
        };
        out.push_str(str);
        Ok(())
    }
    fn merge_scalar(
        &mut self,
        _ctx: &textformat::Context,
        v: &textformat::ast::Literal,
    ) -> textformat::Result<()> {
        match v {
            textformat::ast::Literal::Identifier("MAIN") => *self = ServiceExitType::MAIN,
            textformat::ast::Literal::Identifier("CGROUP") => {
                *self = ServiceExitType::CGROUP;
            }
            textformat::ast::Literal::Int(i) => *self = Self::from(*i as u32),
            other => textformat::bail!("Invalid enum value: {other:?}"),
        }
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum ServiceRestart {
    NO,
    ON_SUCCESS,
    ON_FAILURE,
    ON_ABNORMAL,
    ON_ABORT,
    ALWAYS,
    Unknown(u32),
}
impl Default for ServiceRestart {
    fn default() -> ServiceRestart {
        Self::from(0)
    }
}
impl binformat::format::ProtoEnum for ServiceRestart {}
impl From<u32> for ServiceRestart {
    fn from(v: u32) -> ServiceRestart {
        match v {
            0u32 => ServiceRestart::NO,
            1u32 => ServiceRestart::ON_SUCCESS,
            2u32 => ServiceRestart::ON_FAILURE,
            3u32 => ServiceRestart::ON_ABNORMAL,
            4u32 => ServiceRestart::ON_ABORT,
            5u32 => ServiceRestart::ALWAYS,
            other => ServiceRestart::Unknown(other),
        }
    }
}
impl Into<u32> for ServiceRestart {
    fn into(self) -> u32 {
        match self {
            ServiceRestart::NO => 0u32,
            ServiceRestart::ON_SUCCESS => 1u32,
            ServiceRestart::ON_FAILURE => 2u32,
            ServiceRestart::ON_ABNORMAL => 3u32,
            ServiceRestart::ON_ABORT => 4u32,
            ServiceRestart::ALWAYS => 5u32,
            ServiceRestart::Unknown(other) => other,
        }
    }
}
impl textformat::Field for ServiceRestart {
    fn format(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut String,
    ) -> ::std::fmt::Result {
        let str = match self {
            ServiceRestart::NO => "NO",
            ServiceRestart::ON_SUCCESS => "ON_SUCCESS",
            ServiceRestart::ON_FAILURE => "ON_FAILURE",
            ServiceRestart::ON_ABNORMAL => "ON_ABNORMAL",
            ServiceRestart::ON_ABORT => "ON_ABORT",
            ServiceRestart::ALWAYS => "ALWAYS",
            ServiceRestart::Unknown(n) => {
                write!(out, "{n}")?;
                return Ok(());
            }
        };
        out.push_str(str);
        Ok(())
    }
    fn merge_scalar(
        &mut self,
        _ctx: &textformat::Context,
        v: &textformat::ast::Literal,
    ) -> textformat::Result<()> {
        match v {
            textformat::ast::Literal::Identifier("NO") => *self = ServiceRestart::NO,
            textformat::ast::Literal::Identifier("ON_SUCCESS") => {
                *self = ServiceRestart::ON_SUCCESS;
            }
            textformat::ast::Literal::Identifier("ON_FAILURE") => {
                *self = ServiceRestart::ON_FAILURE;
            }
            textformat::ast::Literal::Identifier("ON_ABNORMAL") => {
                *self = ServiceRestart::ON_ABNORMAL;
            }
            textformat::ast::Literal::Identifier("ON_ABORT") => {
                *self = ServiceRestart::ON_ABORT;
            }
            textformat::ast::Literal::Identifier("ALWAYS") => {
                *self = ServiceRestart::ALWAYS;
            }
            textformat::ast::Literal::Int(i) => *self = Self::from(*i as u32),
            other => textformat::bail!("Invalid enum value: {other:?}"),
        }
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum ServiceNotifyAccess {
    ACCESS_NO,
    ACCESS_MAIN,
    ACCESS_EXEC,
    ACCESS_ALL,
    Unknown(u32),
}
impl Default for ServiceNotifyAccess {
    fn default() -> ServiceNotifyAccess {
        Self::from(0)
    }
}
impl binformat::format::ProtoEnum for ServiceNotifyAccess {}
impl From<u32> for ServiceNotifyAccess {
    fn from(v: u32) -> ServiceNotifyAccess {
        match v {
            0u32 => ServiceNotifyAccess::ACCESS_NO,
            1u32 => ServiceNotifyAccess::ACCESS_MAIN,
            2u32 => ServiceNotifyAccess::ACCESS_EXEC,
            3u32 => ServiceNotifyAccess::ACCESS_ALL,
            other => ServiceNotifyAccess::Unknown(other),
        }
    }
}
impl Into<u32> for ServiceNotifyAccess {
    fn into(self) -> u32 {
        match self {
            ServiceNotifyAccess::ACCESS_NO => 0u32,
            ServiceNotifyAccess::ACCESS_MAIN => 1u32,
            ServiceNotifyAccess::ACCESS_EXEC => 2u32,
            ServiceNotifyAccess::ACCESS_ALL => 3u32,
            ServiceNotifyAccess::Unknown(other) => other,
        }
    }
}
impl textformat::Field for ServiceNotifyAccess {
    fn format(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut String,
    ) -> ::std::fmt::Result {
        let str = match self {
            ServiceNotifyAccess::ACCESS_NO => "ACCESS_NO",
            ServiceNotifyAccess::ACCESS_MAIN => "ACCESS_MAIN",
            ServiceNotifyAccess::ACCESS_EXEC => "ACCESS_EXEC",
            ServiceNotifyAccess::ACCESS_ALL => "ACCESS_ALL",
            ServiceNotifyAccess::Unknown(n) => {
                write!(out, "{n}")?;
                return Ok(());
            }
        };
        out.push_str(str);
        Ok(())
    }
    fn merge_scalar(
        &mut self,
        _ctx: &textformat::Context,
        v: &textformat::ast::Literal,
    ) -> textformat::Result<()> {
        match v {
            textformat::ast::Literal::Identifier("ACCESS_NO") => {
                *self = ServiceNotifyAccess::ACCESS_NO;
            }
            textformat::ast::Literal::Identifier("ACCESS_MAIN") => {
                *self = ServiceNotifyAccess::ACCESS_MAIN;
            }
            textformat::ast::Literal::Identifier("ACCESS_EXEC") => {
                *self = ServiceNotifyAccess::ACCESS_EXEC;
            }
            textformat::ast::Literal::Identifier("ACCESS_ALL") => {
                *self = ServiceNotifyAccess::ACCESS_ALL;
            }
            textformat::ast::Literal::Int(i) => *self = Self::from(*i as u32),
            other => textformat::bail!("Invalid enum value: {other:?}"),
        }
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum ServiceOOMPolicy {
    CONTINUE,
    STOP,
    KILL,
    Unknown(u32),
}
impl Default for ServiceOOMPolicy {
    fn default() -> ServiceOOMPolicy {
        Self::from(0)
    }
}
impl binformat::format::ProtoEnum for ServiceOOMPolicy {}
impl From<u32> for ServiceOOMPolicy {
    fn from(v: u32) -> ServiceOOMPolicy {
        match v {
            0u32 => ServiceOOMPolicy::CONTINUE,
            1u32 => ServiceOOMPolicy::STOP,
            2u32 => ServiceOOMPolicy::KILL,
            other => ServiceOOMPolicy::Unknown(other),
        }
    }
}
impl Into<u32> for ServiceOOMPolicy {
    fn into(self) -> u32 {
        match self {
            ServiceOOMPolicy::CONTINUE => 0u32,
            ServiceOOMPolicy::STOP => 1u32,
            ServiceOOMPolicy::KILL => 2u32,
            ServiceOOMPolicy::Unknown(other) => other,
        }
    }
}
impl textformat::Field for ServiceOOMPolicy {
    fn format(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut String,
    ) -> ::std::fmt::Result {
        let str = match self {
            ServiceOOMPolicy::CONTINUE => "CONTINUE",
            ServiceOOMPolicy::STOP => "STOP",
            ServiceOOMPolicy::KILL => "KILL",
            ServiceOOMPolicy::Unknown(n) => {
                write!(out, "{n}")?;
                return Ok(());
            }
        };
        out.push_str(str);
        Ok(())
    }
    fn merge_scalar(
        &mut self,
        _ctx: &textformat::Context,
        v: &textformat::ast::Literal,
    ) -> textformat::Result<()> {
        match v {
            textformat::ast::Literal::Identifier("CONTINUE") => {
                *self = ServiceOOMPolicy::CONTINUE;
            }
            textformat::ast::Literal::Identifier("STOP") => {
                *self = ServiceOOMPolicy::STOP;
            }
            textformat::ast::Literal::Identifier("KILL") => {
                *self = ServiceOOMPolicy::KILL;
            }
            textformat::ast::Literal::Int(i) => *self = Self::from(*i as u32),
            other => textformat::bail!("Invalid enum value: {other:?}"),
        }
        Ok(())
    }
}
