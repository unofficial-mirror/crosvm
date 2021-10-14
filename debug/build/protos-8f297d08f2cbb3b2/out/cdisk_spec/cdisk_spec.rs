// This file is generated by rust-protobuf 2.25.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `cdisk_spec.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_1;

#[derive(PartialEq,Clone,Default)]
pub struct ComponentDisk {
    // message fields
    pub file_path: ::std::string::String,
    pub offset: u64,
    pub read_write_capability: ReadWriteCapability,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ComponentDisk {
    fn default() -> &'a ComponentDisk {
        <ComponentDisk as ::protobuf::Message>::default_instance()
    }
}

impl ComponentDisk {
    pub fn new() -> ComponentDisk {
        ::std::default::Default::default()
    }

    // string file_path = 1;


    pub fn get_file_path(&self) -> &str {
        &self.file_path
    }
    pub fn clear_file_path(&mut self) {
        self.file_path.clear();
    }

    // Param is passed by value, moved
    pub fn set_file_path(&mut self, v: ::std::string::String) {
        self.file_path = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_file_path(&mut self) -> &mut ::std::string::String {
        &mut self.file_path
    }

    // Take field
    pub fn take_file_path(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.file_path, ::std::string::String::new())
    }

    // uint64 offset = 2;


    pub fn get_offset(&self) -> u64 {
        self.offset
    }
    pub fn clear_offset(&mut self) {
        self.offset = 0;
    }

    // Param is passed by value, moved
    pub fn set_offset(&mut self, v: u64) {
        self.offset = v;
    }

    // .ReadWriteCapability read_write_capability = 3;


    pub fn get_read_write_capability(&self) -> ReadWriteCapability {
        self.read_write_capability
    }
    pub fn clear_read_write_capability(&mut self) {
        self.read_write_capability = ReadWriteCapability::READ_ONLY;
    }

    // Param is passed by value, moved
    pub fn set_read_write_capability(&mut self, v: ReadWriteCapability) {
        self.read_write_capability = v;
    }
}

impl ::protobuf::Message for ComponentDisk {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.file_path)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.offset = tmp;
                },
                3 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.read_write_capability, 3, &mut self.unknown_fields)?
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.file_path.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.file_path);
        }
        if self.offset != 0 {
            my_size += ::protobuf::rt::value_size(2, self.offset, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.read_write_capability != ReadWriteCapability::READ_ONLY {
            my_size += ::protobuf::rt::enum_size(3, self.read_write_capability);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.file_path.is_empty() {
            os.write_string(1, &self.file_path)?;
        }
        if self.offset != 0 {
            os.write_uint64(2, self.offset)?;
        }
        if self.read_write_capability != ReadWriteCapability::READ_ONLY {
            os.write_enum(3, ::protobuf::ProtobufEnum::value(&self.read_write_capability))?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> ComponentDisk {
        ComponentDisk::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "file_path",
                |m: &ComponentDisk| { &m.file_path },
                |m: &mut ComponentDisk| { &mut m.file_path },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "offset",
                |m: &ComponentDisk| { &m.offset },
                |m: &mut ComponentDisk| { &mut m.offset },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ReadWriteCapability>>(
                "read_write_capability",
                |m: &ComponentDisk| { &m.read_write_capability },
                |m: &mut ComponentDisk| { &mut m.read_write_capability },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<ComponentDisk>(
                "ComponentDisk",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static ComponentDisk {
        static instance: ::protobuf::rt::LazyV2<ComponentDisk> = ::protobuf::rt::LazyV2::INIT;
        instance.get(ComponentDisk::new)
    }
}

impl ::protobuf::Clear for ComponentDisk {
    fn clear(&mut self) {
        self.file_path.clear();
        self.offset = 0;
        self.read_write_capability = ReadWriteCapability::READ_ONLY;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ComponentDisk {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ComponentDisk {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CompositeDisk {
    // message fields
    pub version: u64,
    pub component_disks: ::protobuf::RepeatedField<ComponentDisk>,
    pub length: u64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CompositeDisk {
    fn default() -> &'a CompositeDisk {
        <CompositeDisk as ::protobuf::Message>::default_instance()
    }
}

impl CompositeDisk {
    pub fn new() -> CompositeDisk {
        ::std::default::Default::default()
    }

    // uint64 version = 1;


    pub fn get_version(&self) -> u64 {
        self.version
    }
    pub fn clear_version(&mut self) {
        self.version = 0;
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u64) {
        self.version = v;
    }

    // repeated .ComponentDisk component_disks = 2;


    pub fn get_component_disks(&self) -> &[ComponentDisk] {
        &self.component_disks
    }
    pub fn clear_component_disks(&mut self) {
        self.component_disks.clear();
    }

    // Param is passed by value, moved
    pub fn set_component_disks(&mut self, v: ::protobuf::RepeatedField<ComponentDisk>) {
        self.component_disks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_component_disks(&mut self) -> &mut ::protobuf::RepeatedField<ComponentDisk> {
        &mut self.component_disks
    }

    // Take field
    pub fn take_component_disks(&mut self) -> ::protobuf::RepeatedField<ComponentDisk> {
        ::std::mem::replace(&mut self.component_disks, ::protobuf::RepeatedField::new())
    }

    // uint64 length = 3;


    pub fn get_length(&self) -> u64 {
        self.length
    }
    pub fn clear_length(&mut self) {
        self.length = 0;
    }

    // Param is passed by value, moved
    pub fn set_length(&mut self, v: u64) {
        self.length = v;
    }
}

impl ::protobuf::Message for CompositeDisk {
    fn is_initialized(&self) -> bool {
        for v in &self.component_disks {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.version = tmp;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.component_disks)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.length = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.version != 0 {
            my_size += ::protobuf::rt::value_size(1, self.version, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.component_disks {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.length != 0 {
            my_size += ::protobuf::rt::value_size(3, self.length, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.version != 0 {
            os.write_uint64(1, self.version)?;
        }
        for v in &self.component_disks {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.length != 0 {
            os.write_uint64(3, self.length)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> CompositeDisk {
        CompositeDisk::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "version",
                |m: &CompositeDisk| { &m.version },
                |m: &mut CompositeDisk| { &mut m.version },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ComponentDisk>>(
                "component_disks",
                |m: &CompositeDisk| { &m.component_disks },
                |m: &mut CompositeDisk| { &mut m.component_disks },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "length",
                |m: &CompositeDisk| { &m.length },
                |m: &mut CompositeDisk| { &mut m.length },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<CompositeDisk>(
                "CompositeDisk",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static CompositeDisk {
        static instance: ::protobuf::rt::LazyV2<CompositeDisk> = ::protobuf::rt::LazyV2::INIT;
        instance.get(CompositeDisk::new)
    }
}

impl ::protobuf::Clear for CompositeDisk {
    fn clear(&mut self) {
        self.version = 0;
        self.component_disks.clear();
        self.length = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CompositeDisk {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CompositeDisk {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ReadWriteCapability {
    READ_ONLY = 0,
    READ_WRITE = 1,
}

impl ::protobuf::ProtobufEnum for ReadWriteCapability {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ReadWriteCapability> {
        match value {
            0 => ::std::option::Option::Some(ReadWriteCapability::READ_ONLY),
            1 => ::std::option::Option::Some(ReadWriteCapability::READ_WRITE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ReadWriteCapability] = &[
            ReadWriteCapability::READ_ONLY,
            ReadWriteCapability::READ_WRITE,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<ReadWriteCapability>("ReadWriteCapability", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for ReadWriteCapability {
}

impl ::std::default::Default for ReadWriteCapability {
    fn default() -> Self {
        ReadWriteCapability::READ_ONLY
    }
}

impl ::protobuf::reflect::ProtobufValue for ReadWriteCapability {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10cdisk_spec.proto\"\x8e\x01\n\rComponentDisk\x12\x1b\n\tfile_path\
    \x18\x01\x20\x01(\tR\x08filePath\x12\x16\n\x06offset\x18\x02\x20\x01(\
    \x04R\x06offset\x12H\n\x15read_write_capability\x18\x03\x20\x01(\x0e2\
    \x14.ReadWriteCapabilityR\x13readWriteCapability\"z\n\rCompositeDisk\x12\
    \x18\n\x07version\x18\x01\x20\x01(\x04R\x07version\x127\n\x0fcomponent_d\
    isks\x18\x02\x20\x03(\x0b2\x0e.ComponentDiskR\x0ecomponentDisks\x12\x16\
    \n\x06length\x18\x03\x20\x01(\x04R\x06length*4\n\x13ReadWriteCapability\
    \x12\r\n\tREAD_ONLY\x10\0\x12\x0e\n\nREAD_WRITE\x10\x01b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}