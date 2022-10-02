#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(unused_must_use)]
#![allow(clippy::derive_partial_eq_without_eq)]
use std::fmt::Write;
use ::protokit::*;
use ::protokit as root;
use root::types::empty::*;
use root::types::any::*;
use root::types::any::*;
pub fn register_types(registry: &mut reflect::Registry) {
    registry.register(&Ref::default());
    registry.register(&TypedRef::default());
    registry.register(&Object::default());
    registry.register(&List::default());
    registry.register(&PatchOp::default());
    registry.register(&Patch::default());
    registry.register(&Meta::default());
    registry.register(&PutRequest::default());
    registry.register(&PutResponse::default());
    registry.register(&PatchRequest::default());
    registry.register(&PatchResponse::default());
    registry.register(&GetRequest::default());
    registry.register(&GetResponse::default());
    registry.register(&WatchRequest::default());
    registry.register(&WatchResponse::default());
    registry.register(&WatchResponseUpdate::default());
    registry.register(&WatchResponseDelete::default());
    registry.register(&ListRequest::default());
    registry.register(&ListResponse::default());
    registry.register(&DeleteRequest::default());
    registry.register(&DeleteResponse::default());
    registry.register(&Registry::default());
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Ref {
    pub namespace: Option<String>,
    pub name: Option<String>,
    pub _unknown: (),
}
impl Ref {
    #[inline(always)]
    pub fn r#with_namespace(mut self, it: String) -> Self {
        self.r#set_namespace(it);
        self
    }
    #[inline(always)]
    pub fn r#set_namespace(&mut self, it: String) -> &mut Self {
        self.namespace = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_name(mut self, it: String) -> Self {
        self.r#set_name(it);
        self
    }
    #[inline(always)]
    pub fn r#set_name(&mut self, it: String) -> &mut Self {
        self.name = it.into();
        self
    }
}
impl textformat::Decodable for Ref {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("namespace") => {
                textformat::Field::merge(&mut self.namespace, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("name") => {
                textformat::Field::merge(&mut self.name, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for Ref {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.namespace != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("namespace: ");
            textformat::Field::format(&self.namespace, ctx, pad, out)?;
            out.push('\n');
        }
        if self.name != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("name: ");
            textformat::Field::format(&self.name, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for Ref {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: &'b [u8],
    ) -> binformat::Result<&'b [u8]> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Decode::<Bytes>::decode(&mut self.namespace, buf)?;
            }
            18u32 => {
                buf = Decode::<Bytes>::decode(&mut self.name, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for Ref {
    fn qualified_name(&self) -> &'static str {
        "api.Ref"
    }
    fn encode(&self, buf: &mut binformat::Buffer) -> binformat::Result<()> {
        use binformat::format::*;
        Decode::<Bytes>::encode(&self.namespace, 10u32, buf)?;
        Decode::<Bytes>::encode(&self.name, 18u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct TypedRef {
    pub namespace: Option<String>,
    pub name: Option<String>,
    pub r#type: Option<String>,
    pub _unknown: (),
}
impl TypedRef {
    #[inline(always)]
    pub fn r#with_namespace(mut self, it: String) -> Self {
        self.r#set_namespace(it);
        self
    }
    #[inline(always)]
    pub fn r#set_namespace(&mut self, it: String) -> &mut Self {
        self.namespace = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_name(mut self, it: String) -> Self {
        self.r#set_name(it);
        self
    }
    #[inline(always)]
    pub fn r#set_name(&mut self, it: String) -> &mut Self {
        self.name = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_type(mut self, it: String) -> Self {
        self.r#set_type(it);
        self
    }
    #[inline(always)]
    pub fn r#set_type(&mut self, it: String) -> &mut Self {
        self.r#type = it.into();
        self
    }
}
impl textformat::Decodable for TypedRef {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("namespace") => {
                textformat::Field::merge(&mut self.namespace, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("name") => {
                textformat::Field::merge(&mut self.name, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("type") => {
                textformat::Field::merge(&mut self.r#type, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for TypedRef {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.namespace != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("namespace: ");
            textformat::Field::format(&self.namespace, ctx, pad, out)?;
            out.push('\n');
        }
        if self.name != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("name: ");
            textformat::Field::format(&self.name, ctx, pad, out)?;
            out.push('\n');
        }
        if self.r#type != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("type: ");
            textformat::Field::format(&self.r#type, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for TypedRef {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: &'b [u8],
    ) -> binformat::Result<&'b [u8]> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Decode::<Bytes>::decode(&mut self.namespace, buf)?;
            }
            18u32 => {
                buf = Decode::<Bytes>::decode(&mut self.name, buf)?;
            }
            26u32 => {
                buf = Decode::<Bytes>::decode(&mut self.r#type, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for TypedRef {
    fn qualified_name(&self) -> &'static str {
        "api.TypedRef"
    }
    fn encode(&self, buf: &mut binformat::Buffer) -> binformat::Result<()> {
        use binformat::format::*;
        Decode::<Bytes>::encode(&self.namespace, 10u32, buf)?;
        Decode::<Bytes>::encode(&self.name, 18u32, buf)?;
        Decode::<Bytes>::encode(&self.r#type, 26u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Object {
    pub metadata: Option<Box<Meta>>,
    pub spec: Option<Box<Any>>,
    pub revision: u64,
    pub _unknown: (),
}
impl Object {
    #[inline(always)]
    pub fn r#with_metadata(mut self, it: Meta) -> Self {
        self.r#set_metadata(it);
        self
    }
    #[inline(always)]
    pub fn r#set_metadata(&mut self, it: Meta) -> &mut Self {
        self.metadata = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_spec(mut self, it: Any) -> Self {
        self.r#set_spec(it);
        self
    }
    #[inline(always)]
    pub fn r#set_spec(&mut self, it: Any) -> &mut Self {
        self.spec = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_revision(mut self, it: u64) -> Self {
        self.r#set_revision(it);
        self
    }
    #[inline(always)]
    pub fn r#set_revision(&mut self, it: u64) -> &mut Self {
        self.revision = it.into();
        self
    }
}
impl textformat::Decodable for Object {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("metadata") => {
                textformat::Field::merge(&mut self.metadata, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("spec") => {
                textformat::Field::merge(&mut self.spec, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("revision") => {
                textformat::Field::merge(&mut self.revision, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for Object {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.metadata != <Option<Box<Meta>> as Default>::default() {
            out.indent(pad);
            out.push_str("metadata ");
            textformat::Field::format(&self.metadata, ctx, pad, out)?;
            out.push('\n');
        }
        if self.spec != <Option<Box<Any>> as Default>::default() {
            out.indent(pad);
            out.push_str("spec ");
            textformat::Field::format(&self.spec, ctx, pad, out)?;
            out.push('\n');
        }
        if self.revision != <u64 as Default>::default() {
            out.indent(pad);
            out.push_str("revision: ");
            textformat::Field::format(&self.revision, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for Object {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: &'b [u8],
    ) -> binformat::Result<&'b [u8]> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Decode::<Nest>::decode(&mut self.metadata, buf)?;
            }
            18u32 => {
                buf = Decode::<Nest>::decode(&mut self.spec, buf)?;
            }
            24u32 => {
                buf = Decode::<VInt>::decode(&mut self.revision, buf)?;
            }
            26u32 => {
                buf = Decode::<VInt>::decode(&mut self.revision, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for Object {
    fn qualified_name(&self) -> &'static str {
        "api.Object"
    }
    fn encode(&self, buf: &mut binformat::Buffer) -> binformat::Result<()> {
        use binformat::format::*;
        Decode::<Nest>::encode(&self.metadata, 10u32, buf)?;
        Decode::<Nest>::encode(&self.spec, 18u32, buf)?;
        Decode::<VInt>::encode(&self.revision, 24u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct List {
    pub object: Vec<Object>,
    pub _unknown: (),
}
impl List {
    #[inline(always)]
    pub fn r#with_object(mut self, it: Object) -> Self {
        self.r#add_object(it);
        self
    }
    #[inline(always)]
    pub fn r#add_object(&mut self, it: Object) -> &mut Self {
        self.object.push(it);
        self
    }
}
impl textformat::Decodable for List {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("object") => {
                textformat::Field::merge(&mut self.object, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for List {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.object != <Vec<Object> as Default>::default() {
            out.indent(pad);
            out.push_str("object ");
            textformat::Field::format(&self.object, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for List {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: &'b [u8],
    ) -> binformat::Result<&'b [u8]> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Decode::<Repeat::<Nest>>::decode(&mut self.object, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for List {
    fn qualified_name(&self) -> &'static str {
        "api.List"
    }
    fn encode(&self, buf: &mut binformat::Buffer) -> binformat::Result<()> {
        use binformat::format::*;
        Decode::<Repeat::<Nest>>::encode(&self.object, 10u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct PatchOp {
    pub kind: PatchOpKind,
    pub path: String,
    pub _unknown: (),
}
impl PatchOp {
    #[inline(always)]
    pub fn r#with_kind(mut self, it: PatchOpKind) -> Self {
        self.r#set_kind(it);
        self
    }
    #[inline(always)]
    pub fn r#set_kind(&mut self, it: PatchOpKind) -> &mut Self {
        self.kind = it.into();
        self
    }
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
impl textformat::Decodable for PatchOp {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("kind") => {
                textformat::Field::merge(&mut self.kind, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("path") => {
                textformat::Field::merge(&mut self.path, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for PatchOp {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.kind != <PatchOpKind as Default>::default() {
            out.indent(pad);
            out.push_str("kind: ");
            textformat::Field::format(&self.kind, ctx, pad, out)?;
            out.push('\n');
        }
        if self.path != <String as Default>::default() {
            out.indent(pad);
            out.push_str("path: ");
            textformat::Field::format(&self.path, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for PatchOp {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: &'b [u8],
    ) -> binformat::Result<&'b [u8]> {
        use binformat::format::*;
        match tag {
            8u32 => {
                buf = Decode::<Enum>::decode(&mut self.kind, buf)?;
            }
            10u32 => {
                buf = Decode::<Enum>::decode(&mut self.kind, buf)?;
            }
            18u32 => {
                buf = Decode::<Bytes>::decode(&mut self.path, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for PatchOp {
    fn qualified_name(&self) -> &'static str {
        "api.PatchOp"
    }
    fn encode(&self, buf: &mut binformat::Buffer) -> binformat::Result<()> {
        use binformat::format::*;
        Decode::<Enum>::encode(&self.kind, 8u32, buf)?;
        Decode::<Bytes>::encode(&self.path, 18u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Patch {
    pub op: Vec<PatchOp>,
    pub spec: Option<Box<Any>>,
    pub _unknown: (),
}
impl Patch {
    #[inline(always)]
    pub fn r#with_op(mut self, it: PatchOp) -> Self {
        self.r#add_op(it);
        self
    }
    #[inline(always)]
    pub fn r#add_op(&mut self, it: PatchOp) -> &mut Self {
        self.op.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_spec(mut self, it: Any) -> Self {
        self.r#set_spec(it);
        self
    }
    #[inline(always)]
    pub fn r#set_spec(&mut self, it: Any) -> &mut Self {
        self.spec = Box::new(it).into();
        self
    }
}
impl textformat::Decodable for Patch {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("op") => {
                textformat::Field::merge(&mut self.op, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("spec") => {
                textformat::Field::merge(&mut self.spec, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for Patch {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.op != <Vec<PatchOp> as Default>::default() {
            out.indent(pad);
            out.push_str("op ");
            textformat::Field::format(&self.op, ctx, pad, out)?;
            out.push('\n');
        }
        if self.spec != <Option<Box<Any>> as Default>::default() {
            out.indent(pad);
            out.push_str("spec ");
            textformat::Field::format(&self.spec, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for Patch {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: &'b [u8],
    ) -> binformat::Result<&'b [u8]> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Decode::<Repeat::<Nest>>::decode(&mut self.op, buf)?;
            }
            18u32 => {
                buf = Decode::<Nest>::decode(&mut self.spec, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for Patch {
    fn qualified_name(&self) -> &'static str {
        "api.Patch"
    }
    fn encode(&self, buf: &mut binformat::Buffer) -> binformat::Result<()> {
        use binformat::format::*;
        Decode::<Repeat::<Nest>>::encode(&self.op, 10u32, buf)?;
        Decode::<Nest>::encode(&self.spec, 18u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Meta {
    pub namespace: Option<String>,
    pub name: String,
    pub _unknown: (),
}
impl Meta {
    #[inline(always)]
    pub fn r#with_namespace(mut self, it: String) -> Self {
        self.r#set_namespace(it);
        self
    }
    #[inline(always)]
    pub fn r#set_namespace(&mut self, it: String) -> &mut Self {
        self.namespace = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_name(mut self, it: String) -> Self {
        self.r#set_name(it);
        self
    }
    #[inline(always)]
    pub fn r#set_name(&mut self, it: String) -> &mut Self {
        self.name = it.into();
        self
    }
}
impl textformat::Decodable for Meta {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("namespace") => {
                textformat::Field::merge(&mut self.namespace, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("name") => {
                textformat::Field::merge(&mut self.name, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for Meta {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.namespace != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("namespace: ");
            textformat::Field::format(&self.namespace, ctx, pad, out)?;
            out.push('\n');
        }
        if self.name != <String as Default>::default() {
            out.indent(pad);
            out.push_str("name: ");
            textformat::Field::format(&self.name, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for Meta {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: &'b [u8],
    ) -> binformat::Result<&'b [u8]> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Decode::<Bytes>::decode(&mut self.namespace, buf)?;
            }
            18u32 => {
                buf = Decode::<Bytes>::decode(&mut self.name, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for Meta {
    fn qualified_name(&self) -> &'static str {
        "api.Meta"
    }
    fn encode(&self, buf: &mut binformat::Buffer) -> binformat::Result<()> {
        use binformat::format::*;
        Decode::<Bytes>::encode(&self.namespace, 10u32, buf)?;
        Decode::<Bytes>::encode(&self.name, 18u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct PutRequest {
    pub object: Vec<Object>,
    pub _unknown: (),
}
impl PutRequest {
    #[inline(always)]
    pub fn r#with_object(mut self, it: Object) -> Self {
        self.r#add_object(it);
        self
    }
    #[inline(always)]
    pub fn r#add_object(&mut self, it: Object) -> &mut Self {
        self.object.push(it);
        self
    }
}
impl textformat::Decodable for PutRequest {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("object") => {
                textformat::Field::merge(&mut self.object, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for PutRequest {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.object != <Vec<Object> as Default>::default() {
            out.indent(pad);
            out.push_str("object ");
            textformat::Field::format(&self.object, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for PutRequest {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: &'b [u8],
    ) -> binformat::Result<&'b [u8]> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Decode::<Repeat::<Nest>>::decode(&mut self.object, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for PutRequest {
    fn qualified_name(&self) -> &'static str {
        "api.PutRequest"
    }
    fn encode(&self, buf: &mut binformat::Buffer) -> binformat::Result<()> {
        use binformat::format::*;
        Decode::<Repeat::<Nest>>::encode(&self.object, 10u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct PutResponse {
    pub metadata: Option<Box<Meta>>,
    pub spec: Option<Box<Any>>,
    pub revision: u64,
    pub _unknown: (),
}
impl PutResponse {
    #[inline(always)]
    pub fn r#with_metadata(mut self, it: Meta) -> Self {
        self.r#set_metadata(it);
        self
    }
    #[inline(always)]
    pub fn r#set_metadata(&mut self, it: Meta) -> &mut Self {
        self.metadata = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_spec(mut self, it: Any) -> Self {
        self.r#set_spec(it);
        self
    }
    #[inline(always)]
    pub fn r#set_spec(&mut self, it: Any) -> &mut Self {
        self.spec = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_revision(mut self, it: u64) -> Self {
        self.r#set_revision(it);
        self
    }
    #[inline(always)]
    pub fn r#set_revision(&mut self, it: u64) -> &mut Self {
        self.revision = it.into();
        self
    }
}
impl textformat::Decodable for PutResponse {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("metadata") => {
                textformat::Field::merge(&mut self.metadata, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("spec") => {
                textformat::Field::merge(&mut self.spec, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("revision") => {
                textformat::Field::merge(&mut self.revision, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for PutResponse {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.metadata != <Option<Box<Meta>> as Default>::default() {
            out.indent(pad);
            out.push_str("metadata ");
            textformat::Field::format(&self.metadata, ctx, pad, out)?;
            out.push('\n');
        }
        if self.spec != <Option<Box<Any>> as Default>::default() {
            out.indent(pad);
            out.push_str("spec ");
            textformat::Field::format(&self.spec, ctx, pad, out)?;
            out.push('\n');
        }
        if self.revision != <u64 as Default>::default() {
            out.indent(pad);
            out.push_str("revision: ");
            textformat::Field::format(&self.revision, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for PutResponse {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: &'b [u8],
    ) -> binformat::Result<&'b [u8]> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Decode::<Nest>::decode(&mut self.metadata, buf)?;
            }
            18u32 => {
                buf = Decode::<Nest>::decode(&mut self.spec, buf)?;
            }
            24u32 => {
                buf = Decode::<VInt>::decode(&mut self.revision, buf)?;
            }
            26u32 => {
                buf = Decode::<VInt>::decode(&mut self.revision, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for PutResponse {
    fn qualified_name(&self) -> &'static str {
        "api.PutResponse"
    }
    fn encode(&self, buf: &mut binformat::Buffer) -> binformat::Result<()> {
        use binformat::format::*;
        Decode::<Nest>::encode(&self.metadata, 10u32, buf)?;
        Decode::<Nest>::encode(&self.spec, 18u32, buf)?;
        Decode::<VInt>::encode(&self.revision, 24u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct PatchRequest {
    pub metadata: Option<Box<Meta>>,
    pub patch: Vec<Patch>,
    pub _unknown: (),
}
impl PatchRequest {
    #[inline(always)]
    pub fn r#with_metadata(mut self, it: Meta) -> Self {
        self.r#set_metadata(it);
        self
    }
    #[inline(always)]
    pub fn r#set_metadata(&mut self, it: Meta) -> &mut Self {
        self.metadata = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_patch(mut self, it: Patch) -> Self {
        self.r#add_patch(it);
        self
    }
    #[inline(always)]
    pub fn r#add_patch(&mut self, it: Patch) -> &mut Self {
        self.patch.push(it);
        self
    }
}
impl textformat::Decodable for PatchRequest {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("metadata") => {
                textformat::Field::merge(&mut self.metadata, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("patch") => {
                textformat::Field::merge(&mut self.patch, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for PatchRequest {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.metadata != <Option<Box<Meta>> as Default>::default() {
            out.indent(pad);
            out.push_str("metadata ");
            textformat::Field::format(&self.metadata, ctx, pad, out)?;
            out.push('\n');
        }
        if self.patch != <Vec<Patch> as Default>::default() {
            out.indent(pad);
            out.push_str("patch ");
            textformat::Field::format(&self.patch, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for PatchRequest {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: &'b [u8],
    ) -> binformat::Result<&'b [u8]> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Decode::<Nest>::decode(&mut self.metadata, buf)?;
            }
            18u32 => {
                buf = Decode::<Repeat::<Nest>>::decode(&mut self.patch, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for PatchRequest {
    fn qualified_name(&self) -> &'static str {
        "api.PatchRequest"
    }
    fn encode(&self, buf: &mut binformat::Buffer) -> binformat::Result<()> {
        use binformat::format::*;
        Decode::<Nest>::encode(&self.metadata, 10u32, buf)?;
        Decode::<Repeat::<Nest>>::encode(&self.patch, 18u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct PatchResponse {
    pub metadata: Option<Box<Meta>>,
    pub spec: Option<Box<Any>>,
    pub revision: u64,
    pub _unknown: (),
}
impl PatchResponse {
    #[inline(always)]
    pub fn r#with_metadata(mut self, it: Meta) -> Self {
        self.r#set_metadata(it);
        self
    }
    #[inline(always)]
    pub fn r#set_metadata(&mut self, it: Meta) -> &mut Self {
        self.metadata = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_spec(mut self, it: Any) -> Self {
        self.r#set_spec(it);
        self
    }
    #[inline(always)]
    pub fn r#set_spec(&mut self, it: Any) -> &mut Self {
        self.spec = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_revision(mut self, it: u64) -> Self {
        self.r#set_revision(it);
        self
    }
    #[inline(always)]
    pub fn r#set_revision(&mut self, it: u64) -> &mut Self {
        self.revision = it.into();
        self
    }
}
impl textformat::Decodable for PatchResponse {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("metadata") => {
                textformat::Field::merge(&mut self.metadata, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("spec") => {
                textformat::Field::merge(&mut self.spec, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("revision") => {
                textformat::Field::merge(&mut self.revision, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for PatchResponse {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.metadata != <Option<Box<Meta>> as Default>::default() {
            out.indent(pad);
            out.push_str("metadata ");
            textformat::Field::format(&self.metadata, ctx, pad, out)?;
            out.push('\n');
        }
        if self.spec != <Option<Box<Any>> as Default>::default() {
            out.indent(pad);
            out.push_str("spec ");
            textformat::Field::format(&self.spec, ctx, pad, out)?;
            out.push('\n');
        }
        if self.revision != <u64 as Default>::default() {
            out.indent(pad);
            out.push_str("revision: ");
            textformat::Field::format(&self.revision, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for PatchResponse {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: &'b [u8],
    ) -> binformat::Result<&'b [u8]> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Decode::<Nest>::decode(&mut self.metadata, buf)?;
            }
            18u32 => {
                buf = Decode::<Nest>::decode(&mut self.spec, buf)?;
            }
            24u32 => {
                buf = Decode::<VInt>::decode(&mut self.revision, buf)?;
            }
            26u32 => {
                buf = Decode::<VInt>::decode(&mut self.revision, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for PatchResponse {
    fn qualified_name(&self) -> &'static str {
        "api.PatchResponse"
    }
    fn encode(&self, buf: &mut binformat::Buffer) -> binformat::Result<()> {
        use binformat::format::*;
        Decode::<Nest>::encode(&self.metadata, 10u32, buf)?;
        Decode::<Nest>::encode(&self.spec, 18u32, buf)?;
        Decode::<VInt>::encode(&self.revision, 24u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GetRequest {
    pub metadata: Option<Box<Meta>>,
    pub r#type: String,
    pub _unknown: (),
}
impl GetRequest {
    #[inline(always)]
    pub fn r#with_metadata(mut self, it: Meta) -> Self {
        self.r#set_metadata(it);
        self
    }
    #[inline(always)]
    pub fn r#set_metadata(&mut self, it: Meta) -> &mut Self {
        self.metadata = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_type(mut self, it: String) -> Self {
        self.r#set_type(it);
        self
    }
    #[inline(always)]
    pub fn r#set_type(&mut self, it: String) -> &mut Self {
        self.r#type = it.into();
        self
    }
}
impl textformat::Decodable for GetRequest {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("metadata") => {
                textformat::Field::merge(&mut self.metadata, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("type") => {
                textformat::Field::merge(&mut self.r#type, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for GetRequest {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.metadata != <Option<Box<Meta>> as Default>::default() {
            out.indent(pad);
            out.push_str("metadata ");
            textformat::Field::format(&self.metadata, ctx, pad, out)?;
            out.push('\n');
        }
        if self.r#type != <String as Default>::default() {
            out.indent(pad);
            out.push_str("type: ");
            textformat::Field::format(&self.r#type, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for GetRequest {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: &'b [u8],
    ) -> binformat::Result<&'b [u8]> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Decode::<Nest>::decode(&mut self.metadata, buf)?;
            }
            18u32 => {
                buf = Decode::<Bytes>::decode(&mut self.r#type, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for GetRequest {
    fn qualified_name(&self) -> &'static str {
        "api.GetRequest"
    }
    fn encode(&self, buf: &mut binformat::Buffer) -> binformat::Result<()> {
        use binformat::format::*;
        Decode::<Nest>::encode(&self.metadata, 10u32, buf)?;
        Decode::<Bytes>::encode(&self.r#type, 18u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GetResponse {
    pub metadata: Option<Box<Meta>>,
    pub spec: Option<Box<Any>>,
    pub revision: u64,
    pub _unknown: (),
}
impl GetResponse {
    #[inline(always)]
    pub fn r#with_metadata(mut self, it: Meta) -> Self {
        self.r#set_metadata(it);
        self
    }
    #[inline(always)]
    pub fn r#set_metadata(&mut self, it: Meta) -> &mut Self {
        self.metadata = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_spec(mut self, it: Any) -> Self {
        self.r#set_spec(it);
        self
    }
    #[inline(always)]
    pub fn r#set_spec(&mut self, it: Any) -> &mut Self {
        self.spec = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_revision(mut self, it: u64) -> Self {
        self.r#set_revision(it);
        self
    }
    #[inline(always)]
    pub fn r#set_revision(&mut self, it: u64) -> &mut Self {
        self.revision = it.into();
        self
    }
}
impl textformat::Decodable for GetResponse {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("metadata") => {
                textformat::Field::merge(&mut self.metadata, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("spec") => {
                textformat::Field::merge(&mut self.spec, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("revision") => {
                textformat::Field::merge(&mut self.revision, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for GetResponse {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.metadata != <Option<Box<Meta>> as Default>::default() {
            out.indent(pad);
            out.push_str("metadata ");
            textformat::Field::format(&self.metadata, ctx, pad, out)?;
            out.push('\n');
        }
        if self.spec != <Option<Box<Any>> as Default>::default() {
            out.indent(pad);
            out.push_str("spec ");
            textformat::Field::format(&self.spec, ctx, pad, out)?;
            out.push('\n');
        }
        if self.revision != <u64 as Default>::default() {
            out.indent(pad);
            out.push_str("revision: ");
            textformat::Field::format(&self.revision, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for GetResponse {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: &'b [u8],
    ) -> binformat::Result<&'b [u8]> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Decode::<Nest>::decode(&mut self.metadata, buf)?;
            }
            18u32 => {
                buf = Decode::<Nest>::decode(&mut self.spec, buf)?;
            }
            24u32 => {
                buf = Decode::<VInt>::decode(&mut self.revision, buf)?;
            }
            26u32 => {
                buf = Decode::<VInt>::decode(&mut self.revision, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for GetResponse {
    fn qualified_name(&self) -> &'static str {
        "api.GetResponse"
    }
    fn encode(&self, buf: &mut binformat::Buffer) -> binformat::Result<()> {
        use binformat::format::*;
        Decode::<Nest>::encode(&self.metadata, 10u32, buf)?;
        Decode::<Nest>::encode(&self.spec, 18u32, buf)?;
        Decode::<VInt>::encode(&self.revision, 24u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct WatchRequest {
    pub namespace: Option<String>,
    pub name: Option<String>,
    pub from_revision: Option<u64>,
    pub r#type: String,
    pub _unknown: (),
}
impl WatchRequest {
    #[inline(always)]
    pub fn r#with_namespace(mut self, it: String) -> Self {
        self.r#set_namespace(it);
        self
    }
    #[inline(always)]
    pub fn r#set_namespace(&mut self, it: String) -> &mut Self {
        self.namespace = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_name(mut self, it: String) -> Self {
        self.r#set_name(it);
        self
    }
    #[inline(always)]
    pub fn r#set_name(&mut self, it: String) -> &mut Self {
        self.name = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_from_revision(mut self, it: u64) -> Self {
        self.r#set_from_revision(it);
        self
    }
    #[inline(always)]
    pub fn r#set_from_revision(&mut self, it: u64) -> &mut Self {
        self.from_revision = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_type(mut self, it: String) -> Self {
        self.r#set_type(it);
        self
    }
    #[inline(always)]
    pub fn r#set_type(&mut self, it: String) -> &mut Self {
        self.r#type = it.into();
        self
    }
}
impl textformat::Decodable for WatchRequest {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("namespace") => {
                textformat::Field::merge(&mut self.namespace, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("name") => {
                textformat::Field::merge(&mut self.name, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("from_revision") => {
                textformat::Field::merge(&mut self.from_revision, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("type") => {
                textformat::Field::merge(&mut self.r#type, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for WatchRequest {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.namespace != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("namespace: ");
            textformat::Field::format(&self.namespace, ctx, pad, out)?;
            out.push('\n');
        }
        if self.name != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("name: ");
            textformat::Field::format(&self.name, ctx, pad, out)?;
            out.push('\n');
        }
        if self.from_revision != <Option<u64> as Default>::default() {
            out.indent(pad);
            out.push_str("from_revision: ");
            textformat::Field::format(&self.from_revision, ctx, pad, out)?;
            out.push('\n');
        }
        if self.r#type != <String as Default>::default() {
            out.indent(pad);
            out.push_str("type: ");
            textformat::Field::format(&self.r#type, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for WatchRequest {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: &'b [u8],
    ) -> binformat::Result<&'b [u8]> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Decode::<Bytes>::decode(&mut self.namespace, buf)?;
            }
            18u32 => {
                buf = Decode::<Bytes>::decode(&mut self.name, buf)?;
            }
            24u32 => {
                buf = Decode::<VInt>::decode(&mut self.from_revision, buf)?;
            }
            26u32 => {
                buf = Decode::<VInt>::decode(&mut self.from_revision, buf)?;
            }
            34u32 => {
                buf = Decode::<Bytes>::decode(&mut self.r#type, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for WatchRequest {
    fn qualified_name(&self) -> &'static str {
        "api.WatchRequest"
    }
    fn encode(&self, buf: &mut binformat::Buffer) -> binformat::Result<()> {
        use binformat::format::*;
        Decode::<Bytes>::encode(&self.namespace, 10u32, buf)?;
        Decode::<Bytes>::encode(&self.name, 18u32, buf)?;
        Decode::<VInt>::encode(&self.from_revision, 24u32, buf)?;
        Decode::<Bytes>::encode(&self.r#type, 34u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct WatchResponse {
    pub change: WatchResponseOneOfChange,
    pub _unknown: (),
}
impl WatchResponse {
    #[inline(always)]
    pub fn r#with_change_update(mut self, it: WatchResponseUpdate) -> Self {
        self.change = WatchResponseOneOfChange::Update(it);
        self
    }
    #[inline(always)]
    pub fn r#set_change_update(&mut self, it: WatchResponseUpdate) -> &mut Self {
        self.change = WatchResponseOneOfChange::Update(it);
        self
    }
    #[inline(always)]
    pub fn r#with_change_delete(mut self, it: WatchResponseDelete) -> Self {
        self.change = WatchResponseOneOfChange::Delete(it);
        self
    }
    #[inline(always)]
    pub fn r#set_change_delete(&mut self, it: WatchResponseDelete) -> &mut Self {
        self.change = WatchResponseOneOfChange::Delete(it);
        self
    }
}
impl textformat::Decodable for WatchResponse {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("update") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self.change = WatchResponseOneOfChange::Update(target);
            }
            textformat::ast::FieldName::Normal("delete") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self.change = WatchResponseOneOfChange::Delete(target);
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for WatchResponse {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        match &self.change {
            WatchResponseOneOfChange::Update(value) => {
                out.indent(pad);
                out.push_str("update ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            WatchResponseOneOfChange::Delete(value) => {
                out.indent(pad);
                out.push_str("delete ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            WatchResponseOneOfChange::Unknown(..) => {}
        }
        Ok(())
    }
}
impl binformat::Decodable for WatchResponse {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: &'b [u8],
    ) -> binformat::Result<&'b [u8]> {
        use binformat::format::*;
        match tag {
            10u32 => {
                let mut tmp = Default::default();
                buf = Decode::<Nest>::decode(&mut tmp, buf)?;
                self.change = WatchResponseOneOfChange::Update(tmp);
            }
            18u32 => {
                let mut tmp = Default::default();
                buf = Decode::<Nest>::decode(&mut tmp, buf)?;
                self.change = WatchResponseOneOfChange::Delete(tmp);
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for WatchResponse {
    fn qualified_name(&self) -> &'static str {
        "api.WatchResponse"
    }
    fn encode(&self, buf: &mut binformat::Buffer) -> binformat::Result<()> {
        use binformat::format::*;
        match &self.change {
            WatchResponseOneOfChange::Update(value) => {
                Decode::<Nest>::encode(value, 10u32, buf)?;
            }
            WatchResponseOneOfChange::Delete(value) => {
                Decode::<Nest>::encode(value, 18u32, buf)?;
            }
            WatchResponseOneOfChange::Unknown(..) => {}
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C, u32)]
#[derive(Debug, Clone, PartialEq)]
pub enum WatchResponseOneOfChange {
    Update(WatchResponseUpdate),
    Delete(WatchResponseDelete),
    Unknown(::core::marker::PhantomData<()>),
}
impl Default for WatchResponseOneOfChange {
    fn default() -> Self {
        WatchResponseOneOfChange::Unknown(::core::marker::PhantomData)
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct WatchResponseUpdate {
    pub metadata: Option<Box<Meta>>,
    pub spec: Option<Box<Any>>,
    pub revision: u64,
    pub _unknown: (),
}
impl WatchResponseUpdate {
    #[inline(always)]
    pub fn r#with_metadata(mut self, it: Meta) -> Self {
        self.r#set_metadata(it);
        self
    }
    #[inline(always)]
    pub fn r#set_metadata(&mut self, it: Meta) -> &mut Self {
        self.metadata = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_spec(mut self, it: Any) -> Self {
        self.r#set_spec(it);
        self
    }
    #[inline(always)]
    pub fn r#set_spec(&mut self, it: Any) -> &mut Self {
        self.spec = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_revision(mut self, it: u64) -> Self {
        self.r#set_revision(it);
        self
    }
    #[inline(always)]
    pub fn r#set_revision(&mut self, it: u64) -> &mut Self {
        self.revision = it.into();
        self
    }
}
impl textformat::Decodable for WatchResponseUpdate {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("metadata") => {
                textformat::Field::merge(&mut self.metadata, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("spec") => {
                textformat::Field::merge(&mut self.spec, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("revision") => {
                textformat::Field::merge(&mut self.revision, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for WatchResponseUpdate {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.metadata != <Option<Box<Meta>> as Default>::default() {
            out.indent(pad);
            out.push_str("metadata ");
            textformat::Field::format(&self.metadata, ctx, pad, out)?;
            out.push('\n');
        }
        if self.spec != <Option<Box<Any>> as Default>::default() {
            out.indent(pad);
            out.push_str("spec ");
            textformat::Field::format(&self.spec, ctx, pad, out)?;
            out.push('\n');
        }
        if self.revision != <u64 as Default>::default() {
            out.indent(pad);
            out.push_str("revision: ");
            textformat::Field::format(&self.revision, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for WatchResponseUpdate {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: &'b [u8],
    ) -> binformat::Result<&'b [u8]> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Decode::<Nest>::decode(&mut self.metadata, buf)?;
            }
            18u32 => {
                buf = Decode::<Nest>::decode(&mut self.spec, buf)?;
            }
            24u32 => {
                buf = Decode::<VInt>::decode(&mut self.revision, buf)?;
            }
            26u32 => {
                buf = Decode::<VInt>::decode(&mut self.revision, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for WatchResponseUpdate {
    fn qualified_name(&self) -> &'static str {
        "api.WatchResponse.Update"
    }
    fn encode(&self, buf: &mut binformat::Buffer) -> binformat::Result<()> {
        use binformat::format::*;
        Decode::<Nest>::encode(&self.metadata, 10u32, buf)?;
        Decode::<Nest>::encode(&self.spec, 18u32, buf)?;
        Decode::<VInt>::encode(&self.revision, 24u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct WatchResponseDelete {
    pub metadata: Option<Box<Meta>>,
    pub spec: Option<Box<Any>>,
    pub revision: u64,
    pub _unknown: (),
}
impl WatchResponseDelete {
    #[inline(always)]
    pub fn r#with_metadata(mut self, it: Meta) -> Self {
        self.r#set_metadata(it);
        self
    }
    #[inline(always)]
    pub fn r#set_metadata(&mut self, it: Meta) -> &mut Self {
        self.metadata = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_spec(mut self, it: Any) -> Self {
        self.r#set_spec(it);
        self
    }
    #[inline(always)]
    pub fn r#set_spec(&mut self, it: Any) -> &mut Self {
        self.spec = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_revision(mut self, it: u64) -> Self {
        self.r#set_revision(it);
        self
    }
    #[inline(always)]
    pub fn r#set_revision(&mut self, it: u64) -> &mut Self {
        self.revision = it.into();
        self
    }
}
impl textformat::Decodable for WatchResponseDelete {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("metadata") => {
                textformat::Field::merge(&mut self.metadata, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("spec") => {
                textformat::Field::merge(&mut self.spec, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("revision") => {
                textformat::Field::merge(&mut self.revision, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for WatchResponseDelete {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.metadata != <Option<Box<Meta>> as Default>::default() {
            out.indent(pad);
            out.push_str("metadata ");
            textformat::Field::format(&self.metadata, ctx, pad, out)?;
            out.push('\n');
        }
        if self.spec != <Option<Box<Any>> as Default>::default() {
            out.indent(pad);
            out.push_str("spec ");
            textformat::Field::format(&self.spec, ctx, pad, out)?;
            out.push('\n');
        }
        if self.revision != <u64 as Default>::default() {
            out.indent(pad);
            out.push_str("revision: ");
            textformat::Field::format(&self.revision, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for WatchResponseDelete {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: &'b [u8],
    ) -> binformat::Result<&'b [u8]> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Decode::<Nest>::decode(&mut self.metadata, buf)?;
            }
            18u32 => {
                buf = Decode::<Nest>::decode(&mut self.spec, buf)?;
            }
            24u32 => {
                buf = Decode::<VInt>::decode(&mut self.revision, buf)?;
            }
            26u32 => {
                buf = Decode::<VInt>::decode(&mut self.revision, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for WatchResponseDelete {
    fn qualified_name(&self) -> &'static str {
        "api.WatchResponse.Delete"
    }
    fn encode(&self, buf: &mut binformat::Buffer) -> binformat::Result<()> {
        use binformat::format::*;
        Decode::<Nest>::encode(&self.metadata, 10u32, buf)?;
        Decode::<Nest>::encode(&self.spec, 18u32, buf)?;
        Decode::<VInt>::encode(&self.revision, 24u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ListRequest {
    pub namespace: Option<String>,
    pub r#type: Option<String>,
    pub _unknown: (),
}
impl ListRequest {
    #[inline(always)]
    pub fn r#with_namespace(mut self, it: String) -> Self {
        self.r#set_namespace(it);
        self
    }
    #[inline(always)]
    pub fn r#set_namespace(&mut self, it: String) -> &mut Self {
        self.namespace = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_type(mut self, it: String) -> Self {
        self.r#set_type(it);
        self
    }
    #[inline(always)]
    pub fn r#set_type(&mut self, it: String) -> &mut Self {
        self.r#type = it.into();
        self
    }
}
impl textformat::Decodable for ListRequest {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("namespace") => {
                textformat::Field::merge(&mut self.namespace, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("type") => {
                textformat::Field::merge(&mut self.r#type, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for ListRequest {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.namespace != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("namespace: ");
            textformat::Field::format(&self.namespace, ctx, pad, out)?;
            out.push('\n');
        }
        if self.r#type != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("type: ");
            textformat::Field::format(&self.r#type, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for ListRequest {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: &'b [u8],
    ) -> binformat::Result<&'b [u8]> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Decode::<Bytes>::decode(&mut self.namespace, buf)?;
            }
            18u32 => {
                buf = Decode::<Bytes>::decode(&mut self.r#type, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for ListRequest {
    fn qualified_name(&self) -> &'static str {
        "api.ListRequest"
    }
    fn encode(&self, buf: &mut binformat::Buffer) -> binformat::Result<()> {
        use binformat::format::*;
        Decode::<Bytes>::encode(&self.namespace, 10u32, buf)?;
        Decode::<Bytes>::encode(&self.r#type, 18u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ListResponse {
    pub object: Vec<Object>,
    pub _unknown: (),
}
impl ListResponse {
    #[inline(always)]
    pub fn r#with_object(mut self, it: Object) -> Self {
        self.r#add_object(it);
        self
    }
    #[inline(always)]
    pub fn r#add_object(&mut self, it: Object) -> &mut Self {
        self.object.push(it);
        self
    }
}
impl textformat::Decodable for ListResponse {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("object") => {
                textformat::Field::merge(&mut self.object, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for ListResponse {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.object != <Vec<Object> as Default>::default() {
            out.indent(pad);
            out.push_str("object ");
            textformat::Field::format(&self.object, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for ListResponse {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: &'b [u8],
    ) -> binformat::Result<&'b [u8]> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Decode::<Repeat::<Nest>>::decode(&mut self.object, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for ListResponse {
    fn qualified_name(&self) -> &'static str {
        "api.ListResponse"
    }
    fn encode(&self, buf: &mut binformat::Buffer) -> binformat::Result<()> {
        use binformat::format::*;
        Decode::<Repeat::<Nest>>::encode(&self.object, 10u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct DeleteRequest {
    pub metadata: Option<Box<Meta>>,
    pub r#type: String,
    pub _unknown: (),
}
impl DeleteRequest {
    #[inline(always)]
    pub fn r#with_metadata(mut self, it: Meta) -> Self {
        self.r#set_metadata(it);
        self
    }
    #[inline(always)]
    pub fn r#set_metadata(&mut self, it: Meta) -> &mut Self {
        self.metadata = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_type(mut self, it: String) -> Self {
        self.r#set_type(it);
        self
    }
    #[inline(always)]
    pub fn r#set_type(&mut self, it: String) -> &mut Self {
        self.r#type = it.into();
        self
    }
}
impl textformat::Decodable for DeleteRequest {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("metadata") => {
                textformat::Field::merge(&mut self.metadata, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("type") => {
                textformat::Field::merge(&mut self.r#type, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for DeleteRequest {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.metadata != <Option<Box<Meta>> as Default>::default() {
            out.indent(pad);
            out.push_str("metadata ");
            textformat::Field::format(&self.metadata, ctx, pad, out)?;
            out.push('\n');
        }
        if self.r#type != <String as Default>::default() {
            out.indent(pad);
            out.push_str("type: ");
            textformat::Field::format(&self.r#type, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for DeleteRequest {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: &'b [u8],
    ) -> binformat::Result<&'b [u8]> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Decode::<Nest>::decode(&mut self.metadata, buf)?;
            }
            18u32 => {
                buf = Decode::<Bytes>::decode(&mut self.r#type, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for DeleteRequest {
    fn qualified_name(&self) -> &'static str {
        "api.DeleteRequest"
    }
    fn encode(&self, buf: &mut binformat::Buffer) -> binformat::Result<()> {
        use binformat::format::*;
        Decode::<Nest>::encode(&self.metadata, 10u32, buf)?;
        Decode::<Bytes>::encode(&self.r#type, 18u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct DeleteResponse {
    pub metadata: Option<Box<Meta>>,
    pub spec: Option<Box<Any>>,
    pub revision: u64,
    pub _unknown: (),
}
impl DeleteResponse {
    #[inline(always)]
    pub fn r#with_metadata(mut self, it: Meta) -> Self {
        self.r#set_metadata(it);
        self
    }
    #[inline(always)]
    pub fn r#set_metadata(&mut self, it: Meta) -> &mut Self {
        self.metadata = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_spec(mut self, it: Any) -> Self {
        self.r#set_spec(it);
        self
    }
    #[inline(always)]
    pub fn r#set_spec(&mut self, it: Any) -> &mut Self {
        self.spec = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_revision(mut self, it: u64) -> Self {
        self.r#set_revision(it);
        self
    }
    #[inline(always)]
    pub fn r#set_revision(&mut self, it: u64) -> &mut Self {
        self.revision = it.into();
        self
    }
}
impl textformat::Decodable for DeleteResponse {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("metadata") => {
                textformat::Field::merge(&mut self.metadata, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("spec") => {
                textformat::Field::merge(&mut self.spec, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("revision") => {
                textformat::Field::merge(&mut self.revision, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for DeleteResponse {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.metadata != <Option<Box<Meta>> as Default>::default() {
            out.indent(pad);
            out.push_str("metadata ");
            textformat::Field::format(&self.metadata, ctx, pad, out)?;
            out.push('\n');
        }
        if self.spec != <Option<Box<Any>> as Default>::default() {
            out.indent(pad);
            out.push_str("spec ");
            textformat::Field::format(&self.spec, ctx, pad, out)?;
            out.push('\n');
        }
        if self.revision != <u64 as Default>::default() {
            out.indent(pad);
            out.push_str("revision: ");
            textformat::Field::format(&self.revision, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for DeleteResponse {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: &'b [u8],
    ) -> binformat::Result<&'b [u8]> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Decode::<Nest>::decode(&mut self.metadata, buf)?;
            }
            18u32 => {
                buf = Decode::<Nest>::decode(&mut self.spec, buf)?;
            }
            24u32 => {
                buf = Decode::<VInt>::decode(&mut self.revision, buf)?;
            }
            26u32 => {
                buf = Decode::<VInt>::decode(&mut self.revision, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for DeleteResponse {
    fn qualified_name(&self) -> &'static str {
        "api.DeleteResponse"
    }
    fn encode(&self, buf: &mut binformat::Buffer) -> binformat::Result<()> {
        use binformat::format::*;
        Decode::<Nest>::encode(&self.metadata, 10u32, buf)?;
        Decode::<Nest>::encode(&self.spec, 18u32, buf)?;
        Decode::<VInt>::encode(&self.revision, 24u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Registry {
    pub r#type: Vec<String>,
    pub _unknown: (),
}
impl Registry {
    #[inline(always)]
    pub fn r#with_type(mut self, it: String) -> Self {
        self.r#add_type(it);
        self
    }
    #[inline(always)]
    pub fn r#add_type(&mut self, it: String) -> &mut Self {
        self.r#type.push(it);
        self
    }
}
impl textformat::Decodable for Registry {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("type") => {
                textformat::Field::merge(&mut self.r#type, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for Registry {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.r#type != <Vec<String> as Default>::default() {
            out.indent(pad);
            out.push_str("type: ");
            textformat::Field::format(&self.r#type, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for Registry {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: &'b [u8],
    ) -> binformat::Result<&'b [u8]> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Decode::<Repeat::<Bytes>>::decode(&mut self.r#type, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for Registry {
    fn qualified_name(&self) -> &'static str {
        "api.Registry"
    }
    fn encode(&self, buf: &mut binformat::Buffer) -> binformat::Result<()> {
        use binformat::format::*;
        Decode::<Repeat::<Bytes>>::encode(&self.r#type, 10u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum PatchOpKind {
    MERGE,
    REPLACE,
    REMOVE,
    Unknown(u32),
}
impl Default for PatchOpKind {
    fn default() -> PatchOpKind {
        Self::from(0)
    }
}
impl binformat::format::ProtoEnum for PatchOpKind {}
impl From<u32> for PatchOpKind {
    fn from(v: u32) -> PatchOpKind {
        match v {
            0u32 => PatchOpKind::MERGE,
            1u32 => PatchOpKind::REPLACE,
            2u32 => PatchOpKind::REMOVE,
            other => PatchOpKind::Unknown(other),
        }
    }
}
impl Into<u32> for PatchOpKind {
    fn into(self) -> u32 {
        match self {
            PatchOpKind::MERGE => 0u32,
            PatchOpKind::REPLACE => 1u32,
            PatchOpKind::REMOVE => 2u32,
            PatchOpKind::Unknown(other) => other,
        }
    }
}
impl textformat::Field for PatchOpKind {
    fn format(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut String,
    ) -> ::std::fmt::Result {
        let str = match self {
            PatchOpKind::MERGE => "MERGE",
            PatchOpKind::REPLACE => "REPLACE",
            PatchOpKind::REMOVE => "REMOVE",
            PatchOpKind::Unknown(n) => {
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
            textformat::ast::Literal::Identifier("MERGE") => *self = PatchOpKind::MERGE,
            textformat::ast::Literal::Identifier("REPLACE") => {
                *self = PatchOpKind::REPLACE;
            }
            textformat::ast::Literal::Identifier("REMOVE") => *self = PatchOpKind::REMOVE,
            textformat::ast::Literal::Int(i) => *self = Self::from(*i as u32),
            other => textformat::bail!("Invalid enum value: {other:?}"),
        }
        Ok(())
    }
}
mod Api_server {
    use super::root;
    use protokit::grpc::*;
    #[protokit::grpc::async_trait]
    pub trait Api: Send + Sync + 'static {
        async fn put(
            &self,
            req: tonic::Request<super::PutRequest>,
        ) -> Result<tonic::Response<super::PutResponse>, tonic::Status>;
        async fn get(
            &self,
            req: tonic::Request<super::GetRequest>,
        ) -> Result<tonic::Response<super::GetResponse>, tonic::Status>;
        async fn list(
            &self,
            req: tonic::Request<super::ListRequest>,
        ) -> Result<tonic::Response<super::ListResponse>, tonic::Status>;
        async fn patch(
            &self,
            req: tonic::Request<super::PatchRequest>,
        ) -> Result<tonic::Response<super::PatchResponse>, tonic::Status>;
        async fn delete(
            &self,
            req: tonic::Request<super::DeleteRequest>,
        ) -> Result<tonic::Response<super::DeleteResponse>, tonic::Status>;
        type WatchStream: Stream<Item = Result<super::WatchResponse, Status>>
            + Send
            + 'static;
        async fn watch(
            &self,
            req: tonic::Request<super::WatchRequest>,
        ) -> Result<tonic::Response<Self::WatchStream>, tonic::Status>;
        async fn inspect(
            &self,
            req: tonic::Request<super::Empty>,
        ) -> Result<tonic::Response<super::Registry>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ApiServer<S: Api>(pub Arc<S>);
    impl<S: Api> Clone for ApiServer<S> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<S: Api> From<S> for ApiServer<S> {
        fn from(v: S) -> Self {
            Self(::std::sync::Arc::new(v))
        }
    }
    impl<S: Api> From<::std::sync::Arc<S>> for ApiServer<S> {
        fn from(v: ::std::sync::Arc<S>) -> Self {
            Self(v)
        }
    }
    struct PutSvc<S: Api>(Arc<S>);
    impl<S: Api> tonic::server::UnaryService<super::PutRequest> for PutSvc<S> {
        type Response = super::PutResponse;
        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
        fn call(&mut self, request: tonic::Request<super::PutRequest>) -> Self::Future {
            let inner = self.0.clone();
            Box::pin(async move { inner.put(request).await })
        }
    }
    struct GetSvc<S: Api>(Arc<S>);
    impl<S: Api> tonic::server::UnaryService<super::GetRequest> for GetSvc<S> {
        type Response = super::GetResponse;
        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
        fn call(&mut self, request: tonic::Request<super::GetRequest>) -> Self::Future {
            let inner = self.0.clone();
            Box::pin(async move { inner.get(request).await })
        }
    }
    struct ListSvc<S: Api>(Arc<S>);
    impl<S: Api> tonic::server::UnaryService<super::ListRequest> for ListSvc<S> {
        type Response = super::ListResponse;
        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
        fn call(&mut self, request: tonic::Request<super::ListRequest>) -> Self::Future {
            let inner = self.0.clone();
            Box::pin(async move { inner.list(request).await })
        }
    }
    struct PatchSvc<S: Api>(Arc<S>);
    impl<S: Api> tonic::server::UnaryService<super::PatchRequest> for PatchSvc<S> {
        type Response = super::PatchResponse;
        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
        fn call(
            &mut self,
            request: tonic::Request<super::PatchRequest>,
        ) -> Self::Future {
            let inner = self.0.clone();
            Box::pin(async move { inner.patch(request).await })
        }
    }
    struct DeleteSvc<S: Api>(Arc<S>);
    impl<S: Api> tonic::server::UnaryService<super::DeleteRequest> for DeleteSvc<S> {
        type Response = super::DeleteResponse;
        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
        fn call(
            &mut self,
            request: tonic::Request<super::DeleteRequest>,
        ) -> Self::Future {
            let inner = self.0.clone();
            Box::pin(async move { inner.delete(request).await })
        }
    }
    struct WatchSvc<S: Api>(Arc<S>);
    impl<S: Api> tonic::server::ServerStreamingService<super::WatchRequest>
    for WatchSvc<S> {
        type Response = super::WatchResponse;
        type ResponseStream = S::WatchStream;
        type Future = BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
        fn call(
            &mut self,
            request: tonic::Request<super::WatchRequest>,
        ) -> Self::Future {
            let inner = self.0.clone();
            Box::pin(async move { inner.watch(request).await })
        }
    }
    struct InspectSvc<S: Api>(Arc<S>);
    impl<S: Api> tonic::server::UnaryService<super::Empty> for InspectSvc<S> {
        type Response = super::Registry;
        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
        fn call(&mut self, request: tonic::Request<super::Empty>) -> Self::Future {
            let inner = self.0.clone();
            Box::pin(async move { inner.inspect(request).await })
        }
    }
    impl<S, B> Service<http::Request<B>> for ApiServer<S>
    where
        S: Api,
        B: Body + Send + 'static,
        B::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>> + Send
            + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.0.clone();
            match req.uri().path() {
                "/api.Api/Put" => {
                    let inner = self.0.clone();
                    let fut = async move {
                        let method = PutSvc(inner);
                        let codec = ::protokit::grpc::TonicCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.Api/Get" => {
                    let inner = self.0.clone();
                    let fut = async move {
                        let method = GetSvc(inner);
                        let codec = ::protokit::grpc::TonicCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.Api/List" => {
                    let inner = self.0.clone();
                    let fut = async move {
                        let method = ListSvc(inner);
                        let codec = ::protokit::grpc::TonicCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.Api/Patch" => {
                    let inner = self.0.clone();
                    let fut = async move {
                        let method = PatchSvc(inner);
                        let codec = ::protokit::grpc::TonicCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.Api/Delete" => {
                    let inner = self.0.clone();
                    let fut = async move {
                        let method = DeleteSvc(inner);
                        let codec = ::protokit::grpc::TonicCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.Api/Watch" => {
                    let inner = self.0.clone();
                    let fut = async move {
                        let method = WatchSvc(inner);
                        let codec = ::protokit::grpc::TonicCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.Api/Inspect" => {
                    let inner = self.0.clone();
                    let fut = async move {
                        let method = InspectSvc(inner);
                        let codec = ::protokit::grpc::TonicCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<S: Api> tonic::transport::NamedService for ApiServer<S> {
        const NAME: &'static str = "api.Api";
    }
}
pub use Api_server::*;
mod Api_client {
    use super::root;
    use protokit::grpc::*;
    #[derive(Debug, Clone)]
    pub struct ApiClient<C> {
        inner: tonic::client::Grpc<C>,
    }
    impl ApiClient<tonic::transport::Channel> {
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<S> ApiClient<S>
    where
        S: tonic::client::GrpcService<tonic::body::BoxBody>,
        S::Error: Into<StdError>,
        S::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <S::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: S) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: S,
            interceptor: F,
        ) -> ApiClient<InterceptedService<S, F>>
        where
            F: tonic::service::Interceptor,
            S::ResponseBody: Default,
            S: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <S as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <S as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            ApiClient::new(InterceptedService::new(inner, interceptor))
        }
        pub async fn put(
            &mut self,
            request: impl tonic::IntoRequest<super::PutRequest>,
        ) -> Result<tonic::Response<super::PutResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    Status::new(
                        Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = ::protokit::grpc::TonicCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.Api/Put");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRequest>,
        ) -> Result<tonic::Response<super::GetResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    Status::new(
                        Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = ::protokit::grpc::TonicCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.Api/Get");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRequest>,
        ) -> Result<tonic::Response<super::ListResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    Status::new(
                        Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = ::protokit::grpc::TonicCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.Api/List");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn patch(
            &mut self,
            request: impl tonic::IntoRequest<super::PatchRequest>,
        ) -> Result<tonic::Response<super::PatchResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    Status::new(
                        Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = ::protokit::grpc::TonicCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.Api/Patch");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRequest>,
        ) -> Result<tonic::Response<super::DeleteResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    Status::new(
                        Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = ::protokit::grpc::TonicCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.Api/Delete");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn watch(
            &mut self,
            request: impl tonic::IntoRequest<super::WatchRequest>,
        ) -> Result<
            tonic::Response<tonic::Streaming<super::WatchResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    Status::new(
                        Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = ::protokit::grpc::TonicCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.Api/Watch");
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn inspect(
            &mut self,
            request: impl tonic::IntoRequest<super::Empty>,
        ) -> Result<tonic::Response<super::Registry>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    Status::new(
                        Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = ::protokit::grpc::TonicCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.Api/Inspect");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
pub use Api_client::*;
