#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(unused_must_use)]
#![allow(clippy::derive_partial_eq_without_eq)]
use std::fmt::Write;
use ::protokit::*;
use ::protokit as root;
pub fn register_types(registry: &mut reflect::Registry) {
    registry.register(&FieldPath::default());
    registry.register(&FieldMask::default());
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct FieldPath {
    pub path: Vec<i32>,
    pub _unknown: (),
}
impl FieldPath {
    #[inline(always)]
    pub fn r#with_path(mut self, it: i32) -> Self {
        self.r#add_path(it);
        self
    }
    #[inline(always)]
    pub fn r#add_path(&mut self, it: i32) -> &mut Self {
        self.path.push(it);
        self
    }
}
impl textformat::Decodable for FieldPath {
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
impl textformat::Encodable for FieldPath {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.path != <Vec<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("path: ");
            textformat::Field::format(&self.path, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for FieldPath {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: &'b [u8],
    ) -> binformat::Result<&'b [u8]> {
        use binformat::format::*;
        match tag {
            8u32 => {
                buf = Decode::<Repeat::<VInt>>::decode(&mut self.path, buf)?;
            }
            10u32 => {
                buf = Decode::<Pack::<VInt>>::decode(&mut self.path, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for FieldPath {
    fn qualified_name(&self) -> &'static str {
        "api.FieldPath"
    }
    fn encode(&self, buf: &mut binformat::Buffer) -> binformat::Result<()> {
        use binformat::format::*;
        Decode::<Pack::<VInt>>::encode(&self.path, 10u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct FieldMask {
    pub paths: Vec<FieldPath>,
    pub _unknown: (),
}
impl FieldMask {
    #[inline(always)]
    pub fn r#with_paths(mut self, it: FieldPath) -> Self {
        self.r#add_paths(it);
        self
    }
    #[inline(always)]
    pub fn r#add_paths(&mut self, it: FieldPath) -> &mut Self {
        self.paths.push(it);
        self
    }
}
impl textformat::Decodable for FieldMask {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("paths") => {
                textformat::Field::merge(&mut self.paths, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for FieldMask {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.paths != <Vec<FieldPath> as Default>::default() {
            out.indent(pad);
            out.push_str("paths ");
            textformat::Field::format(&self.paths, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for FieldMask {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: &'b [u8],
    ) -> binformat::Result<&'b [u8]> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Decode::<Repeat::<Nest>>::decode(&mut self.paths, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for FieldMask {
    fn qualified_name(&self) -> &'static str {
        "api.FieldMask"
    }
    fn encode(&self, buf: &mut binformat::Buffer) -> binformat::Result<()> {
        use binformat::format::*;
        Decode::<Repeat::<Nest>>::encode(&self.paths, 10u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
