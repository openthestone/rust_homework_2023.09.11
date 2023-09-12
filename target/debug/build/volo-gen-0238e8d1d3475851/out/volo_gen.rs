pub mod volo_gen {
    #![allow(warnings, clippy::all)]

    pub mod mini_redis {
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct RedisServiceRedisCommandArgsSend {
            pub req: RedisRequest,
        }
        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for RedisServiceRedisCommandArgsSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "RedisServiceRedisCommandArgsSend",
                };

                protocol.write_struct_begin(&struct_ident)?;
                protocol.write_struct_field(1, &self.req, ::pilota::thrift::TType::Struct)?;
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut req = None;

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(err) = (|| {
                    loop {
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            protocol.field_stop_len();
                            break;
                        } else {
                            protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                req = Some(::pilota::thrift::Message::decode(protocol)?);
                            }
                            _ => {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }

                        protocol.read_field_end()?;
                        protocol.field_end_len();
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(err)),
                        format!("decode struct `RedisServiceRedisCommandArgsSend` field(#{}) failed", field_id),
                    ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end()?;

                let Some(req) = req else {
                    return Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        "field req is required".to_string(),
                    ));
                };

                let data = Self { req };
                Ok(data)
            }

            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let mut req = None;

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin().await?;
                if let Err(err) = async {
                    loop {
                        let field_ident = protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                req =
                                    Some(::pilota::thrift::Message::decode_async(protocol).await?);
                            }
                            _ => {
                                protocol.skip(field_ident.field_type).await?;
                            }
                        }

                        protocol.read_field_end().await?;
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                }
                .await
                {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(err)),
                        format!("decode struct `RedisServiceRedisCommandArgsSend` field(#{}) failed", field_id),
                    ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end().await?;

                let Some(req) = req else {
                    return Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        "field req is required".to_string(),
                    ));
                };

                let data = Self { req };
                Ok(data)
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "RedisServiceRedisCommandArgsSend",
                }) + protocol.struct_field_len(Some(1), &self.req)
                    + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        impl ::std::convert::From<RequestType> for i32 {
            fn from(e: RequestType) -> Self {
                e as _
            }
        }

        impl ::std::convert::TryFrom<i32> for RequestType {
            type Error = ::pilota::EnumConvertError<i32>;

            #[allow(non_upper_case_globals)]
            fn try_from(v: i32) -> ::std::result::Result<Self, ::pilota::EnumConvertError<i32>> {
                const Get: i32 = RequestType::Get as i32;
                const Set: i32 = RequestType::Set as i32;
                const Del: i32 = RequestType::Del as i32;
                const Ping: i32 = RequestType::Ping as i32;
                const Subscribe: i32 = RequestType::Subscribe as i32;
                const Publish: i32 = RequestType::Publish as i32;
                match v {
                    Get => ::std::result::Result::Ok(RequestType::Get),
                    Set => ::std::result::Result::Ok(RequestType::Set),
                    Del => ::std::result::Result::Ok(RequestType::Del),
                    Ping => ::std::result::Result::Ok(RequestType::Ping),
                    Subscribe => ::std::result::Result::Ok(RequestType::Subscribe),
                    Publish => ::std::result::Result::Ok(RequestType::Publish),

                    _ => ::std::result::Result::Err(::pilota::EnumConvertError::InvalidNum(
                        v,
                        "RequestType",
                    )),
                }
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, ::pilota::derivative::Derivative)]
        #[derivative(Default)]
        #[derive(Clone, PartialEq)]
        #[repr(i32)]
        #[derive(Copy)]
        pub enum RequestType {
            #[derivative(Default)]
            Get = 0,

            Set = 1,

            Del = 2,

            Ping = 3,

            Subscribe = 4,

            Publish = 5,
        }

        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for RequestType {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                protocol.write_i32(*self as i32)?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};
                let value = protocol.read_i32()?;
                Ok(::std::convert::TryFrom::try_from(value).map_err(|err| {
                    ::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        format!("invalid enum value for RequestType, value: {}", value),
                    )
                })?)
            }

            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let value = protocol.read_i32().await?;
                Ok(::std::convert::TryFrom::try_from(value).map_err(|err| {
                    ::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        format!("invalid enum value for RequestType, value: {}", value),
                    )
                })?)
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.i32_len(*self as i32)
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct RedisRequest {
            pub key: ::std::option::Option<::pilota::FastStr>,

            pub value: ::std::option::Option<::pilota::FastStr>,

            pub expire_time: ::std::option::Option<i32>,

            pub request_type: RequestType,
        }
        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for RedisRequest {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "RedisRequest",
                };

                protocol.write_struct_begin(&struct_ident)?;
                if let Some(value) = self.key.as_ref() {
                    protocol.write_faststr_field(1, (value).clone())?;
                }
                if let Some(value) = self.value.as_ref() {
                    protocol.write_faststr_field(2, (value).clone())?;
                }
                if let Some(value) = self.expire_time.as_ref() {
                    protocol.write_i32_field(3, *value)?;
                }
                protocol.write_i32_field(4, (*&self.request_type).into())?;
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut key = None;
                let mut value = None;
                let mut expire_time = None;
                let mut request_type = None;

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(err) = (|| {
                    loop {
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            protocol.field_stop_len();
                            break;
                        } else {
                            protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                key = Some(protocol.read_faststr()?);
                            }
                            Some(2)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                value = Some(protocol.read_faststr()?);
                            }
                            Some(3) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                                expire_time = Some(protocol.read_i32()?);
                            }
                            Some(4) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                                request_type = Some(::pilota::thrift::Message::decode(protocol)?);
                            }
                            _ => {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }

                        protocol.read_field_end()?;
                        protocol.field_end_len();
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(
                                err,
                            )),
                            format!("decode struct `RedisRequest` field(#{}) failed", field_id),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end()?;

                let Some(request_type) = request_type else {
                    return Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        "field request_type is required".to_string(),
                    ));
                };

                let data = Self {
                    key,
                    value,
                    expire_time,
                    request_type,
                };
                Ok(data)
            }

            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let mut key = None;
                let mut value = None;
                let mut expire_time = None;
                let mut request_type = None;

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin().await?;
                if let Err(err) = async {
                    loop {
                        let field_ident = protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                key = Some(protocol.read_faststr().await?);
                            }
                            Some(2)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                value = Some(protocol.read_faststr().await?);
                            }
                            Some(3) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                                expire_time = Some(protocol.read_i32().await?);
                            }
                            Some(4) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                                request_type =
                                    Some(::pilota::thrift::Message::decode_async(protocol).await?);
                            }
                            _ => {
                                protocol.skip(field_ident.field_type).await?;
                            }
                        }

                        protocol.read_field_end().await?;
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                }
                .await
                {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(
                                err,
                            )),
                            format!("decode struct `RedisRequest` field(#{}) failed", field_id),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end().await?;

                let Some(request_type) = request_type else {
                    return Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        "field request_type is required".to_string(),
                    ));
                };

                let data = Self {
                    key,
                    value,
                    expire_time,
                    request_type,
                };
                Ok(data)
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "RedisRequest",
                }) + self
                    .key
                    .as_ref()
                    .map_or(0, |value| protocol.faststr_field_len(Some(1), value))
                    + self
                        .value
                        .as_ref()
                        .map_or(0, |value| protocol.faststr_field_len(Some(2), value))
                    + self
                        .expire_time
                        .as_ref()
                        .map_or(0, |value| protocol.i32_field_len(Some(3), *value))
                    + protocol.i32_field_len(Some(4), (*&self.request_type).into())
                    + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct RedisServiceRedisCommandArgsRecv {
            pub req: RedisRequest,
        }
        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for RedisServiceRedisCommandArgsRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "RedisServiceRedisCommandArgsRecv",
                };

                protocol.write_struct_begin(&struct_ident)?;
                protocol.write_struct_field(1, &self.req, ::pilota::thrift::TType::Struct)?;
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut req = None;

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(err) = (|| {
                    loop {
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            protocol.field_stop_len();
                            break;
                        } else {
                            protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                req = Some(::pilota::thrift::Message::decode(protocol)?);
                            }
                            _ => {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }

                        protocol.read_field_end()?;
                        protocol.field_end_len();
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(err)),
                        format!("decode struct `RedisServiceRedisCommandArgsRecv` field(#{}) failed", field_id),
                    ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end()?;

                let Some(req) = req else {
                    return Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        "field req is required".to_string(),
                    ));
                };

                let data = Self { req };
                Ok(data)
            }

            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let mut req = None;

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin().await?;
                if let Err(err) = async {
                    loop {
                        let field_ident = protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                req =
                                    Some(::pilota::thrift::Message::decode_async(protocol).await?);
                            }
                            _ => {
                                protocol.skip(field_ident.field_type).await?;
                            }
                        }

                        protocol.read_field_end().await?;
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                }
                .await
                {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(err)),
                        format!("decode struct `RedisServiceRedisCommandArgsRecv` field(#{}) failed", field_id),
                    ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end().await?;

                let Some(req) = req else {
                    return Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        "field req is required".to_string(),
                    ));
                };

                let data = Self { req };
                Ok(data)
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "RedisServiceRedisCommandArgsRecv",
                }) + protocol.struct_field_len(Some(1), &self.req)
                    + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, ::pilota::derivative::Derivative)]
        #[derivative(Default)]
        #[derive(Clone, PartialEq)]

        pub enum RedisServiceRedisCommandResultSend {
            #[derivative(Default)]
            Ok(RedisResponse),
        }

        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for RedisServiceRedisCommandResultSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "RedisServiceRedisCommandResultSend",
                })?;
                match self {
                    RedisServiceRedisCommandResultSend::Ok(ref value) => {
                        protocol.write_struct_field(0, value, ::pilota::thrift::TType::Struct)?;
                    }
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};
                let mut ret = None;
                protocol.read_struct_begin()?;
                loop {
                    let field_ident = protocol.read_field_begin()?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        protocol.field_stop_len();
                        break;
                    } else {
                        protocol.field_begin_len(field_ident.field_type, field_ident.id);
                    }
                    match field_ident.id {
                        Some(0) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(protocol)?;
                                protocol.struct_len(&field_ident);
                                ret = Some(RedisServiceRedisCommandResultSend::Ok(field_ident));
                            } else {
                                return Err(::pilota::thrift::DecodeError::new(
                                    ::pilota::thrift::DecodeErrorKind::InvalidData,
                                    "received multiple fields for union from remote Message",
                                ));
                            }
                        }
                        _ => {
                            protocol.skip(field_ident.field_type)?;
                        }
                    }
                }
                protocol.read_field_end()?;
                protocol.read_struct_end()?;
                if let Some(ret) = ret {
                    Ok(ret)
                } else {
                    Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        "received empty union from remote Message",
                    ))
                }
            }

            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let mut ret = None;
                protocol.read_struct_begin().await?;
                loop {
                    let field_ident = protocol.read_field_begin().await?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        break;
                    } else {
                    }
                    match field_ident.id {
                        Some(0) => {
                            if ret.is_none() {
                                let field_ident =
                                    ::pilota::thrift::Message::decode_async(protocol).await?;

                                ret = Some(RedisServiceRedisCommandResultSend::Ok(field_ident));
                            } else {
                                return Err(::pilota::thrift::DecodeError::new(
                                    ::pilota::thrift::DecodeErrorKind::InvalidData,
                                    "received multiple fields for union from remote Message",
                                ));
                            }
                        }
                        _ => {
                            protocol.skip(field_ident.field_type).await?;
                        }
                    }
                }
                protocol.read_field_end().await?;
                protocol.read_struct_end().await?;
                if let Some(ret) = ret {
                    Ok(ret)
                } else {
                    Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        "received empty union from remote Message",
                    ))
                }
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "RedisServiceRedisCommandResultSend",
                }) + match self {
                    RedisServiceRedisCommandResultSend::Ok(ref value) => {
                        protocol.struct_field_len(Some(0), value)
                    }
                } + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct RedisResponse {
            pub response_type: ResponseType,

            pub value: ::std::option::Option<::pilota::FastStr>,
        }
        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for RedisResponse {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "RedisResponse",
                };

                protocol.write_struct_begin(&struct_ident)?;
                protocol.write_i32_field(1, (*&self.response_type).into())?;
                if let Some(value) = self.value.as_ref() {
                    protocol.write_faststr_field(2, (value).clone())?;
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut response_type = None;
                let mut value = None;

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(err) = (|| {
                    loop {
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            protocol.field_stop_len();
                            break;
                        } else {
                            protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                                response_type = Some(::pilota::thrift::Message::decode(protocol)?);
                            }
                            Some(2)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                value = Some(protocol.read_faststr()?);
                            }
                            _ => {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }

                        protocol.read_field_end()?;
                        protocol.field_end_len();
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(
                                err,
                            )),
                            format!("decode struct `RedisResponse` field(#{}) failed", field_id),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end()?;

                let Some(response_type) = response_type else {
                    return Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        "field response_type is required".to_string(),
                    ));
                };

                let data = Self {
                    response_type,
                    value,
                };
                Ok(data)
            }

            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let mut response_type = None;
                let mut value = None;

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin().await?;
                if let Err(err) = async {
                    loop {
                        let field_ident = protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                                response_type =
                                    Some(::pilota::thrift::Message::decode_async(protocol).await?);
                            }
                            Some(2)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                value = Some(protocol.read_faststr().await?);
                            }
                            _ => {
                                protocol.skip(field_ident.field_type).await?;
                            }
                        }

                        protocol.read_field_end().await?;
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                }
                .await
                {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(
                                err,
                            )),
                            format!("decode struct `RedisResponse` field(#{}) failed", field_id),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end().await?;

                let Some(response_type) = response_type else {
                    return Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        "field response_type is required".to_string(),
                    ));
                };

                let data = Self {
                    response_type,
                    value,
                };
                Ok(data)
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "RedisResponse",
                }) + protocol.i32_field_len(Some(1), (*&self.response_type).into())
                    + self
                        .value
                        .as_ref()
                        .map_or(0, |value| protocol.faststr_field_len(Some(2), value))
                    + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        impl ::std::convert::From<ResponseType> for i32 {
            fn from(e: ResponseType) -> Self {
                e as _
            }
        }

        impl ::std::convert::TryFrom<i32> for ResponseType {
            type Error = ::pilota::EnumConvertError<i32>;

            #[allow(non_upper_case_globals)]
            fn try_from(v: i32) -> ::std::result::Result<Self, ::pilota::EnumConvertError<i32>> {
                const Print: i32 = ResponseType::Print as i32;
                const Trap: i32 = ResponseType::Trap as i32;
                match v {
                    Print => ::std::result::Result::Ok(ResponseType::Print),
                    Trap => ::std::result::Result::Ok(ResponseType::Trap),

                    _ => ::std::result::Result::Err(::pilota::EnumConvertError::InvalidNum(
                        v,
                        "ResponseType",
                    )),
                }
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, ::pilota::derivative::Derivative)]
        #[derivative(Default)]
        #[derive(Clone, PartialEq)]
        #[repr(i32)]
        #[derive(Copy)]
        pub enum ResponseType {
            #[derivative(Default)]
            Print = 0,

            Trap = 1,
        }

        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for ResponseType {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                protocol.write_i32(*self as i32)?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};
                let value = protocol.read_i32()?;
                Ok(::std::convert::TryFrom::try_from(value).map_err(|err| {
                    ::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        format!("invalid enum value for ResponseType, value: {}", value),
                    )
                })?)
            }

            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let value = protocol.read_i32().await?;
                Ok(::std::convert::TryFrom::try_from(value).map_err(|err| {
                    ::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        format!("invalid enum value for ResponseType, value: {}", value),
                    )
                })?)
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.i32_len(*self as i32)
            }
        }
        #[::async_trait::async_trait]
        pub trait RedisService {
            async fn redis_command(
                &self,
                req: RedisRequest,
            ) -> ::core::result::Result<RedisResponse, ::volo_thrift::AnyhowError>;
        }
        pub struct RedisServiceServer<S> {
            inner: S, // handler
        }

        pub struct MkRedisServiceGenericClient;

        pub type RedisServiceClient = RedisServiceGenericClient<
            ::volo::service::BoxCloneService<
                ::volo_thrift::context::ClientContext,
                RedisServiceRequestSend,
                ::std::option::Option<RedisServiceResponseRecv>,
                ::volo_thrift::Error,
            >,
        >;

        impl<S> ::volo::client::MkClient<::volo_thrift::Client<S>> for MkRedisServiceGenericClient {
            type Target = RedisServiceGenericClient<S>;
            fn mk_client(&self, service: ::volo_thrift::Client<S>) -> Self::Target {
                RedisServiceGenericClient(service)
            }
        }

        #[derive(Clone)]
        pub struct RedisServiceGenericClient<S>(pub ::volo_thrift::Client<S>);

        pub struct RedisServiceOneShotClient<S>(pub ::volo_thrift::Client<S>);

        impl<
                S: ::volo::service::Service<
                        ::volo_thrift::context::ClientContext,
                        RedisServiceRequestSend,
                        Response = ::std::option::Option<RedisServiceResponseRecv>,
                        Error = ::volo_thrift::Error,
                    > + Send
                    + Sync
                    + 'static,
            > RedisServiceGenericClient<S>
        {
            pub fn with_callopt<
                Opt: ::volo::client::Apply<::volo_thrift::context::ClientContext>,
            >(
                self,
                opt: Opt,
            ) -> RedisServiceOneShotClient<::volo::client::WithOptService<S, Opt>> {
                RedisServiceOneShotClient(self.0.with_opt(opt))
            }

            pub async fn redis_command(
                &self,
                req: RedisRequest,
            ) -> ::std::result::Result<
                RedisResponse,
                ::volo_thrift::error::ResponseError<std::convert::Infallible>,
            > {
                let req =
                    RedisServiceRequestSend::RedisCommand(RedisServiceRedisCommandArgsSend { req });
                let mut cx = self.0.make_cx("RedisCommand", false);
                #[allow(unreachable_patterns)]
                let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
                    Some(RedisServiceResponseRecv::RedisCommand(
                        RedisServiceRedisCommandResultRecv::Ok(resp),
                    )) => Ok(resp),
                    None => unreachable!(),
                    _ => unreachable!(),
                };
                ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
                    let mut cache = cache.borrow_mut();
                    if cache.len() < cache.capacity() {
                        cache.push(cx);
                    }
                });
                resp
            }
        }

        impl<
                S: ::volo::client::OneShotService<
                        ::volo_thrift::context::ClientContext,
                        RedisServiceRequestSend,
                        Response = ::std::option::Option<RedisServiceResponseRecv>,
                        Error = ::volo_thrift::Error,
                    > + Send
                    + Sync
                    + 'static,
            > RedisServiceOneShotClient<S>
        {
            pub async fn redis_command(
                self,
                req: RedisRequest,
            ) -> ::std::result::Result<
                RedisResponse,
                ::volo_thrift::error::ResponseError<std::convert::Infallible>,
            > {
                let req =
                    RedisServiceRequestSend::RedisCommand(RedisServiceRedisCommandArgsSend { req });
                let mut cx = self.0.make_cx("RedisCommand", false);
                #[allow(unreachable_patterns)]
                let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
                    Some(RedisServiceResponseRecv::RedisCommand(
                        RedisServiceRedisCommandResultRecv::Ok(resp),
                    )) => Ok(resp),
                    None => unreachable!(),
                    _ => unreachable!(),
                };
                ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
                    let mut cache = cache.borrow_mut();
                    if cache.len() < cache.capacity() {
                        cache.push(cx);
                    }
                });
                resp
            }
        }

        pub struct RedisServiceClientBuilder {}

        impl RedisServiceClientBuilder {
            pub fn new(
                service_name: impl AsRef<str>,
            ) -> ::volo_thrift::client::ClientBuilder<
                ::volo::layer::Identity,
                ::volo::layer::Identity,
                MkRedisServiceGenericClient,
                RedisServiceRequestSend,
                RedisServiceResponseRecv,
                ::volo::net::dial::DefaultMakeTransport,
                ::volo_thrift::codec::default::DefaultMakeCodec<
                    ::volo_thrift::codec::default::ttheader::MakeTTHeaderCodec<
                        ::volo_thrift::codec::default::framed::MakeFramedCodec<
                            ::volo_thrift::codec::default::thrift::MakeThriftCodec,
                        >,
                    >,
                >,
                ::volo::loadbalance::LbConfig<
                    ::volo::loadbalance::random::WeightedRandomBalance<()>,
                    ::volo::discovery::DummyDiscover,
                >,
            > {
                ::volo_thrift::client::ClientBuilder::new(service_name, MkRedisServiceGenericClient)
            }
        }

        impl<S> RedisServiceServer<S>
        where
            S: RedisService + ::core::marker::Send + ::core::marker::Sync + 'static,
        {
            pub fn new(
                inner: S,
            ) -> ::volo_thrift::server::Server<
                Self,
                ::volo::layer::Identity,
                RedisServiceRequestRecv,
                ::volo_thrift::codec::default::DefaultMakeCodec<
                    ::volo_thrift::codec::default::ttheader::MakeTTHeaderCodec<
                        ::volo_thrift::codec::default::framed::MakeFramedCodec<
                            ::volo_thrift::codec::default::thrift::MakeThriftCodec,
                        >,
                    >,
                >,
                ::volo_thrift::tracing::DefaultProvider,
            > {
                ::volo_thrift::server::Server::new(Self { inner })
            }
        }

        impl<T>
            ::volo::service::Service<::volo_thrift::context::ServerContext, RedisServiceRequestRecv>
            for RedisServiceServer<T>
        where
            T: RedisService + Send + Sync + 'static,
        {
            type Response = RedisServiceResponseSend;
            type Error = ::anyhow::Error;

            type Future<'cx> = impl ::std::future::Future<Output = ::std::result::Result<Self::Response, Self::Error>>
                + 'cx;

            fn call<'cx, 's>(
                &'s self,
                _cx: &'cx mut ::volo_thrift::context::ServerContext,
                req: RedisServiceRequestRecv,
            ) -> Self::Future<'cx>
            where
                's: 'cx,
            {
                async move {
                    match req {
                        RedisServiceRequestRecv::RedisCommand(args) => {
                            Ok(RedisServiceResponseSend::RedisCommand(
                                match self.inner.redis_command(args.req).await {
                                    Ok(resp) => RedisServiceRedisCommandResultSend::Ok(resp),
                                    Err(err) => return Err(err),
                                },
                            ))
                        }
                    }
                }
            }
        }
        #[derive(Debug, Clone)]
        pub enum RedisServiceRequestRecv {
            RedisCommand(RedisServiceRedisCommandArgsRecv),
        }

        #[derive(Debug, Clone)]
        pub enum RedisServiceRequestSend {
            RedisCommand(RedisServiceRedisCommandArgsSend),
        }

        #[derive(Debug, Clone)]
        pub enum RedisServiceResponseRecv {
            RedisCommand(RedisServiceRedisCommandResultRecv),
        }

        #[derive(Debug, Clone)]
        pub enum RedisServiceResponseSend {
            RedisCommand(RedisServiceRedisCommandResultSend),
        }

        #[::async_trait::async_trait]
        impl ::volo_thrift::EntryMessage for RedisServiceRequestRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::core::result::Result<(), ::pilota::thrift::EncodeError> {
                match self {
                    Self::RedisCommand(value) => {
                        ::pilota::thrift::Message::encode(value, protocol).map_err(|err| err.into())
                    }
                }
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
                msg_ident: &::pilota::thrift::TMessageIdentifier,
            ) -> ::core::result::Result<Self, ::pilota::thrift::DecodeError> {
                Ok(match &*msg_ident.name {
                    "RedisCommand" => {
                        Self::RedisCommand(::pilota::thrift::Message::decode(protocol)?)
                    }
                    _ => {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::UnknownMethod,
                            format!("unknown method {}", msg_ident.name),
                        ));
                    }
                })
            }

            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
                msg_ident: &::pilota::thrift::TMessageIdentifier,
            ) -> ::core::result::Result<Self, ::pilota::thrift::DecodeError> {
                Ok(match &*msg_ident.name {
                    "RedisCommand" => {
                        Self::RedisCommand(::pilota::thrift::Message::decode_async(protocol).await?)
                    }
                    _ => {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::UnknownMethod,
                            format!("unknown method {}", msg_ident.name),
                        ));
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                match self {
                    Self::RedisCommand(value) => ::volo_thrift::Message::size(value, protocol),
                }
            }
        }

        #[::async_trait::async_trait]
        impl ::volo_thrift::EntryMessage for RedisServiceRequestSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::core::result::Result<(), ::pilota::thrift::EncodeError> {
                match self {
                    Self::RedisCommand(value) => {
                        ::pilota::thrift::Message::encode(value, protocol).map_err(|err| err.into())
                    }
                }
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
                msg_ident: &::pilota::thrift::TMessageIdentifier,
            ) -> ::core::result::Result<Self, ::pilota::thrift::DecodeError> {
                Ok(match &*msg_ident.name {
                    "RedisCommand" => {
                        Self::RedisCommand(::pilota::thrift::Message::decode(protocol)?)
                    }
                    _ => {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::UnknownMethod,
                            format!("unknown method {}", msg_ident.name),
                        ));
                    }
                })
            }

            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
                msg_ident: &::pilota::thrift::TMessageIdentifier,
            ) -> ::core::result::Result<Self, ::pilota::thrift::DecodeError> {
                Ok(match &*msg_ident.name {
                    "RedisCommand" => {
                        Self::RedisCommand(::pilota::thrift::Message::decode_async(protocol).await?)
                    }
                    _ => {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::UnknownMethod,
                            format!("unknown method {}", msg_ident.name),
                        ));
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                match self {
                    Self::RedisCommand(value) => ::volo_thrift::Message::size(value, protocol),
                }
            }
        }
        #[::async_trait::async_trait]
        impl ::volo_thrift::EntryMessage for RedisServiceResponseRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::core::result::Result<(), ::pilota::thrift::EncodeError> {
                match self {
                    Self::RedisCommand(value) => {
                        ::pilota::thrift::Message::encode(value, protocol).map_err(|err| err.into())
                    }
                }
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
                msg_ident: &::pilota::thrift::TMessageIdentifier,
            ) -> ::core::result::Result<Self, ::pilota::thrift::DecodeError> {
                Ok(match &*msg_ident.name {
                    "RedisCommand" => {
                        Self::RedisCommand(::pilota::thrift::Message::decode(protocol)?)
                    }
                    _ => {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::UnknownMethod,
                            format!("unknown method {}", msg_ident.name),
                        ));
                    }
                })
            }

            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
                msg_ident: &::pilota::thrift::TMessageIdentifier,
            ) -> ::core::result::Result<Self, ::pilota::thrift::DecodeError> {
                Ok(match &*msg_ident.name {
                    "RedisCommand" => {
                        Self::RedisCommand(::pilota::thrift::Message::decode_async(protocol).await?)
                    }
                    _ => {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::UnknownMethod,
                            format!("unknown method {}", msg_ident.name),
                        ));
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                match self {
                    Self::RedisCommand(value) => ::volo_thrift::Message::size(value, protocol),
                }
            }
        }

        #[::async_trait::async_trait]
        impl ::volo_thrift::EntryMessage for RedisServiceResponseSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::core::result::Result<(), ::pilota::thrift::EncodeError> {
                match self {
                    Self::RedisCommand(value) => {
                        ::pilota::thrift::Message::encode(value, protocol).map_err(|err| err.into())
                    }
                }
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
                msg_ident: &::pilota::thrift::TMessageIdentifier,
            ) -> ::core::result::Result<Self, ::pilota::thrift::DecodeError> {
                Ok(match &*msg_ident.name {
                    "RedisCommand" => {
                        Self::RedisCommand(::pilota::thrift::Message::decode(protocol)?)
                    }
                    _ => {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::UnknownMethod,
                            format!("unknown method {}", msg_ident.name),
                        ));
                    }
                })
            }

            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
                msg_ident: &::pilota::thrift::TMessageIdentifier,
            ) -> ::core::result::Result<Self, ::pilota::thrift::DecodeError> {
                Ok(match &*msg_ident.name {
                    "RedisCommand" => {
                        Self::RedisCommand(::pilota::thrift::Message::decode_async(protocol).await?)
                    }
                    _ => {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::UnknownMethod,
                            format!("unknown method {}", msg_ident.name),
                        ));
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                match self {
                    Self::RedisCommand(value) => ::volo_thrift::Message::size(value, protocol),
                }
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, ::pilota::derivative::Derivative)]
        #[derivative(Default)]
        #[derive(Clone, PartialEq)]

        pub enum RedisServiceRedisCommandResultRecv {
            #[derivative(Default)]
            Ok(RedisResponse),
        }

        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for RedisServiceRedisCommandResultRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "RedisServiceRedisCommandResultRecv",
                })?;
                match self {
                    RedisServiceRedisCommandResultRecv::Ok(ref value) => {
                        protocol.write_struct_field(0, value, ::pilota::thrift::TType::Struct)?;
                    }
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};
                let mut ret = None;
                protocol.read_struct_begin()?;
                loop {
                    let field_ident = protocol.read_field_begin()?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        protocol.field_stop_len();
                        break;
                    } else {
                        protocol.field_begin_len(field_ident.field_type, field_ident.id);
                    }
                    match field_ident.id {
                        Some(0) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(protocol)?;
                                protocol.struct_len(&field_ident);
                                ret = Some(RedisServiceRedisCommandResultRecv::Ok(field_ident));
                            } else {
                                return Err(::pilota::thrift::DecodeError::new(
                                    ::pilota::thrift::DecodeErrorKind::InvalidData,
                                    "received multiple fields for union from remote Message",
                                ));
                            }
                        }
                        _ => {
                            protocol.skip(field_ident.field_type)?;
                        }
                    }
                }
                protocol.read_field_end()?;
                protocol.read_struct_end()?;
                if let Some(ret) = ret {
                    Ok(ret)
                } else {
                    Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        "received empty union from remote Message",
                    ))
                }
            }

            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let mut ret = None;
                protocol.read_struct_begin().await?;
                loop {
                    let field_ident = protocol.read_field_begin().await?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        break;
                    } else {
                    }
                    match field_ident.id {
                        Some(0) => {
                            if ret.is_none() {
                                let field_ident =
                                    ::pilota::thrift::Message::decode_async(protocol).await?;

                                ret = Some(RedisServiceRedisCommandResultRecv::Ok(field_ident));
                            } else {
                                return Err(::pilota::thrift::DecodeError::new(
                                    ::pilota::thrift::DecodeErrorKind::InvalidData,
                                    "received multiple fields for union from remote Message",
                                ));
                            }
                        }
                        _ => {
                            protocol.skip(field_ident.field_type).await?;
                        }
                    }
                }
                protocol.read_field_end().await?;
                protocol.read_struct_end().await?;
                if let Some(ret) = ret {
                    Ok(ret)
                } else {
                    Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        "received empty union from remote Message",
                    ))
                }
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "RedisServiceRedisCommandResultRecv",
                }) + match self {
                    RedisServiceRedisCommandResultRecv::Ok(ref value) => {
                        protocol.struct_field_len(Some(0), value)
                    }
                } + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
    }
}
