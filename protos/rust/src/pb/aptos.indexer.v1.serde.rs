// Copyright (c) Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

// @generated
impl serde::Serialize for ActiveStream {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id.is_some() {
            len += 1;
        }
        if self.current_version.is_some() {
            len += 1;
        }
        if self.end_version.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aptos.indexer.v1.ActiveStream", len)?;
        if let Some(v) = self.id.as_ref() {
            struct_ser.serialize_field("id", v)?;
        }
        if let Some(v) = self.current_version.as_ref() {
            struct_ser.serialize_field("currentVersion", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.end_version.as_ref() {
            struct_ser.serialize_field("endVersion", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ActiveStream {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "current_version",
            "currentVersion",
            "end_version",
            "endVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            CurrentVersion,
            EndVersion,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "currentVersion" | "current_version" => Ok(GeneratedField::CurrentVersion),
                            "endVersion" | "end_version" => Ok(GeneratedField::EndVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ActiveStream;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aptos.indexer.v1.ActiveStream")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ActiveStream, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut current_version__ = None;
                let mut end_version__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = map.next_value()?;
                        }
                        GeneratedField::CurrentVersion => {
                            if current_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currentVersion"));
                            }
                            current_version__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::EndVersion => {
                            if end_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endVersion"));
                            }
                            end_version__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(ActiveStream {
                    id: id__,
                    current_version: current_version__,
                    end_version: end_version__,
                })
            }
        }
        deserializer.deserialize_struct("aptos.indexer.v1.ActiveStream", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FullNodeInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("aptos.indexer.v1.FullNodeInfo", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FullNodeInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FullNodeInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aptos.indexer.v1.FullNodeInfo")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FullNodeInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(FullNodeInfo {
                })
            }
        }
        deserializer.deserialize_struct("aptos.indexer.v1.FullNodeInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetTransactionsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.starting_version.is_some() {
            len += 1;
        }
        if self.transactions_count.is_some() {
            len += 1;
        }
        if self.batch_size.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aptos.indexer.v1.GetTransactionsRequest", len)?;
        if let Some(v) = self.starting_version.as_ref() {
            struct_ser.serialize_field("startingVersion", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.transactions_count.as_ref() {
            struct_ser.serialize_field("transactionsCount", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.batch_size.as_ref() {
            struct_ser.serialize_field("batchSize", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetTransactionsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "starting_version",
            "startingVersion",
            "transactions_count",
            "transactionsCount",
            "batch_size",
            "batchSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StartingVersion,
            TransactionsCount,
            BatchSize,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "startingVersion" | "starting_version" => Ok(GeneratedField::StartingVersion),
                            "transactionsCount" | "transactions_count" => Ok(GeneratedField::TransactionsCount),
                            "batchSize" | "batch_size" => Ok(GeneratedField::BatchSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetTransactionsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aptos.indexer.v1.GetTransactionsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetTransactionsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut starting_version__ = None;
                let mut transactions_count__ = None;
                let mut batch_size__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::StartingVersion => {
                            if starting_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startingVersion"));
                            }
                            starting_version__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::TransactionsCount => {
                            if transactions_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transactionsCount"));
                            }
                            transactions_count__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::BatchSize => {
                            if batch_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchSize"));
                            }
                            batch_size__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(GetTransactionsRequest {
                    starting_version: starting_version__,
                    transactions_count: transactions_count__,
                    batch_size: batch_size__,
                })
            }
        }
        deserializer.deserialize_struct("aptos.indexer.v1.GetTransactionsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HeartbeatRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.service_info.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aptos.indexer.v1.HeartbeatRequest", len)?;
        if let Some(v) = self.service_info.as_ref() {
            struct_ser.serialize_field("serviceInfo", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HeartbeatRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "service_info",
            "serviceInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ServiceInfo,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "serviceInfo" | "service_info" => Ok(GeneratedField::ServiceInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HeartbeatRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aptos.indexer.v1.HeartbeatRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HeartbeatRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut service_info__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ServiceInfo => {
                            if service_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceInfo"));
                            }
                            service_info__ = map.next_value()?;
                        }
                    }
                }
                Ok(HeartbeatRequest {
                    service_info: service_info__,
                })
            }
        }
        deserializer.deserialize_struct("aptos.indexer.v1.HeartbeatRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HeartbeatResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.known_latest_version.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aptos.indexer.v1.HeartbeatResponse", len)?;
        if let Some(v) = self.known_latest_version.as_ref() {
            struct_ser.serialize_field("knownLatestVersion", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HeartbeatResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "known_latest_version",
            "knownLatestVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KnownLatestVersion,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "knownLatestVersion" | "known_latest_version" => Ok(GeneratedField::KnownLatestVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HeartbeatResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aptos.indexer.v1.HeartbeatResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HeartbeatResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut known_latest_version__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::KnownLatestVersion => {
                            if known_latest_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("knownLatestVersion"));
                            }
                            known_latest_version__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(HeartbeatResponse {
                    known_latest_version: known_latest_version__,
                })
            }
        }
        deserializer.deserialize_struct("aptos.indexer.v1.HeartbeatResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HistoricalDataServiceInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.timestamp.is_some() {
            len += 1;
        }
        if self.known_latest_version.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aptos.indexer.v1.HistoricalDataServiceInfo", len)?;
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", v)?;
        }
        if let Some(v) = self.known_latest_version.as_ref() {
            struct_ser.serialize_field("knownLatestVersion", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HistoricalDataServiceInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "timestamp",
            "known_latest_version",
            "knownLatestVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Timestamp,
            KnownLatestVersion,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "knownLatestVersion" | "known_latest_version" => Ok(GeneratedField::KnownLatestVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HistoricalDataServiceInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aptos.indexer.v1.HistoricalDataServiceInfo")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HistoricalDataServiceInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut timestamp__ = None;
                let mut known_latest_version__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = map.next_value()?;
                        }
                        GeneratedField::KnownLatestVersion => {
                            if known_latest_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("knownLatestVersion"));
                            }
                            known_latest_version__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(HistoricalDataServiceInfo {
                    timestamp: timestamp__,
                    known_latest_version: known_latest_version__,
                })
            }
        }
        deserializer.deserialize_struct("aptos.indexer.v1.HistoricalDataServiceInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LiveDataServiceInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.timestamp.is_some() {
            len += 1;
        }
        if self.known_latest_version.is_some() {
            len += 1;
        }
        if self.stream_info.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aptos.indexer.v1.LiveDataServiceInfo", len)?;
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", v)?;
        }
        if let Some(v) = self.known_latest_version.as_ref() {
            struct_ser.serialize_field("knownLatestVersion", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.stream_info.as_ref() {
            struct_ser.serialize_field("streamInfo", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LiveDataServiceInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "timestamp",
            "known_latest_version",
            "knownLatestVersion",
            "stream_info",
            "streamInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Timestamp,
            KnownLatestVersion,
            StreamInfo,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "knownLatestVersion" | "known_latest_version" => Ok(GeneratedField::KnownLatestVersion),
                            "streamInfo" | "stream_info" => Ok(GeneratedField::StreamInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LiveDataServiceInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aptos.indexer.v1.LiveDataServiceInfo")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<LiveDataServiceInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut timestamp__ = None;
                let mut known_latest_version__ = None;
                let mut stream_info__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = map.next_value()?;
                        }
                        GeneratedField::KnownLatestVersion => {
                            if known_latest_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("knownLatestVersion"));
                            }
                            known_latest_version__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::StreamInfo => {
                            if stream_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("streamInfo"));
                            }
                            stream_info__ = map.next_value()?;
                        }
                    }
                }
                Ok(LiveDataServiceInfo {
                    timestamp: timestamp__,
                    known_latest_version: known_latest_version__,
                    stream_info: stream_info__,
                })
            }
        }
        deserializer.deserialize_struct("aptos.indexer.v1.LiveDataServiceInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PingHistoricalDataServiceRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.known_latest_version.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aptos.indexer.v1.PingHistoricalDataServiceRequest", len)?;
        if let Some(v) = self.known_latest_version.as_ref() {
            struct_ser.serialize_field("knownLatestVersion", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PingHistoricalDataServiceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "known_latest_version",
            "knownLatestVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KnownLatestVersion,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "knownLatestVersion" | "known_latest_version" => Ok(GeneratedField::KnownLatestVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PingHistoricalDataServiceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aptos.indexer.v1.PingHistoricalDataServiceRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PingHistoricalDataServiceRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut known_latest_version__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::KnownLatestVersion => {
                            if known_latest_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("knownLatestVersion"));
                            }
                            known_latest_version__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(PingHistoricalDataServiceRequest {
                    known_latest_version: known_latest_version__,
                })
            }
        }
        deserializer.deserialize_struct("aptos.indexer.v1.PingHistoricalDataServiceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PingHistoricalDataServiceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.service_info.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aptos.indexer.v1.PingHistoricalDataServiceResponse", len)?;
        if let Some(v) = self.service_info.as_ref() {
            struct_ser.serialize_field("serviceInfo", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PingHistoricalDataServiceResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "service_info",
            "serviceInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ServiceInfo,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "serviceInfo" | "service_info" => Ok(GeneratedField::ServiceInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PingHistoricalDataServiceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aptos.indexer.v1.PingHistoricalDataServiceResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PingHistoricalDataServiceResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut service_info__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ServiceInfo => {
                            if service_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceInfo"));
                            }
                            service_info__ = map.next_value()?;
                        }
                    }
                }
                Ok(PingHistoricalDataServiceResponse {
                    service_info: service_info__,
                })
            }
        }
        deserializer.deserialize_struct("aptos.indexer.v1.PingHistoricalDataServiceResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PingLiveDataServiceRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.known_latest_version.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aptos.indexer.v1.PingLiveDataServiceRequest", len)?;
        if let Some(v) = self.known_latest_version.as_ref() {
            struct_ser.serialize_field("knownLatestVersion", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PingLiveDataServiceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "known_latest_version",
            "knownLatestVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KnownLatestVersion,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "knownLatestVersion" | "known_latest_version" => Ok(GeneratedField::KnownLatestVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PingLiveDataServiceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aptos.indexer.v1.PingLiveDataServiceRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PingLiveDataServiceRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut known_latest_version__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::KnownLatestVersion => {
                            if known_latest_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("knownLatestVersion"));
                            }
                            known_latest_version__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(PingLiveDataServiceRequest {
                    known_latest_version: known_latest_version__,
                })
            }
        }
        deserializer.deserialize_struct("aptos.indexer.v1.PingLiveDataServiceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PingLiveDataServiceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.service_info.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aptos.indexer.v1.PingLiveDataServiceResponse", len)?;
        if let Some(v) = self.service_info.as_ref() {
            struct_ser.serialize_field("serviceInfo", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PingLiveDataServiceResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "service_info",
            "serviceInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ServiceInfo,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "serviceInfo" | "service_info" => Ok(GeneratedField::ServiceInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PingLiveDataServiceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aptos.indexer.v1.PingLiveDataServiceResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PingLiveDataServiceResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut service_info__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ServiceInfo => {
                            if service_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceInfo"));
                            }
                            service_info__ = map.next_value()?;
                        }
                    }
                }
                Ok(PingLiveDataServiceResponse {
                    service_info: service_info__,
                })
            }
        }
        deserializer.deserialize_struct("aptos.indexer.v1.PingLiveDataServiceResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ServiceInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.address.is_some() {
            len += 1;
        }
        if self.service_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aptos.indexer.v1.ServiceInfo", len)?;
        if let Some(v) = self.address.as_ref() {
            struct_ser.serialize_field("address", v)?;
        }
        if let Some(v) = self.service_type.as_ref() {
            match v {
                service_info::ServiceType::LiveDataServiceInfo(v) => {
                    struct_ser.serialize_field("liveDataServiceInfo", v)?;
                }
                service_info::ServiceType::HistoricalDataServiceInfo(v) => {
                    struct_ser.serialize_field("historicalDataServiceInfo", v)?;
                }
                service_info::ServiceType::FullNodeInfo(v) => {
                    struct_ser.serialize_field("fullNodeInfo", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ServiceInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
            "live_data_service_info",
            "liveDataServiceInfo",
            "historical_data_service_info",
            "historicalDataServiceInfo",
            "full_node_info",
            "fullNodeInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            LiveDataServiceInfo,
            HistoricalDataServiceInfo,
            FullNodeInfo,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "address" => Ok(GeneratedField::Address),
                            "liveDataServiceInfo" | "live_data_service_info" => Ok(GeneratedField::LiveDataServiceInfo),
                            "historicalDataServiceInfo" | "historical_data_service_info" => Ok(GeneratedField::HistoricalDataServiceInfo),
                            "fullNodeInfo" | "full_node_info" => Ok(GeneratedField::FullNodeInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ServiceInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aptos.indexer.v1.ServiceInfo")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ServiceInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut service_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = map.next_value()?;
                        }
                        GeneratedField::LiveDataServiceInfo => {
                            if service_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("liveDataServiceInfo"));
                            }
                            service_type__ = map.next_value::<::std::option::Option<_>>()?.map(service_info::ServiceType::LiveDataServiceInfo)
;
                        }
                        GeneratedField::HistoricalDataServiceInfo => {
                            if service_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("historicalDataServiceInfo"));
                            }
                            service_type__ = map.next_value::<::std::option::Option<_>>()?.map(service_info::ServiceType::HistoricalDataServiceInfo)
;
                        }
                        GeneratedField::FullNodeInfo => {
                            if service_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fullNodeInfo"));
                            }
                            service_type__ = map.next_value::<::std::option::Option<_>>()?.map(service_info::ServiceType::FullNodeInfo)
;
                        }
                    }
                }
                Ok(ServiceInfo {
                    address: address__,
                    service_type: service_type__,
                })
            }
        }
        deserializer.deserialize_struct("aptos.indexer.v1.ServiceInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.active_stream.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aptos.indexer.v1.StreamInfo", len)?;
        if !self.active_stream.is_empty() {
            struct_ser.serialize_field("activeStream", &self.active_stream)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "active_stream",
            "activeStream",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ActiveStream,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "activeStream" | "active_stream" => Ok(GeneratedField::ActiveStream),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aptos.indexer.v1.StreamInfo")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StreamInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut active_stream__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ActiveStream => {
                            if active_stream__.is_some() {
                                return Err(serde::de::Error::duplicate_field("activeStream"));
                            }
                            active_stream__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(StreamInfo {
                    active_stream: active_stream__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aptos.indexer.v1.StreamInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TransactionsInStorage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.transactions.is_empty() {
            len += 1;
        }
        if self.starting_version.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aptos.indexer.v1.TransactionsInStorage", len)?;
        if !self.transactions.is_empty() {
            struct_ser.serialize_field("transactions", &self.transactions)?;
        }
        if let Some(v) = self.starting_version.as_ref() {
            struct_ser.serialize_field("startingVersion", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TransactionsInStorage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "transactions",
            "starting_version",
            "startingVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Transactions,
            StartingVersion,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "transactions" => Ok(GeneratedField::Transactions),
                            "startingVersion" | "starting_version" => Ok(GeneratedField::StartingVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TransactionsInStorage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aptos.indexer.v1.TransactionsInStorage")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TransactionsInStorage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut transactions__ = None;
                let mut starting_version__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Transactions => {
                            if transactions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transactions"));
                            }
                            transactions__ = Some(map.next_value()?);
                        }
                        GeneratedField::StartingVersion => {
                            if starting_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startingVersion"));
                            }
                            starting_version__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(TransactionsInStorage {
                    transactions: transactions__.unwrap_or_default(),
                    starting_version: starting_version__,
                })
            }
        }
        deserializer.deserialize_struct("aptos.indexer.v1.TransactionsInStorage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TransactionsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.transactions.is_empty() {
            len += 1;
        }
        if self.chain_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aptos.indexer.v1.TransactionsResponse", len)?;
        if !self.transactions.is_empty() {
            struct_ser.serialize_field("transactions", &self.transactions)?;
        }
        if let Some(v) = self.chain_id.as_ref() {
            struct_ser.serialize_field("chainId", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TransactionsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "transactions",
            "chain_id",
            "chainId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Transactions,
            ChainId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "transactions" => Ok(GeneratedField::Transactions),
                            "chainId" | "chain_id" => Ok(GeneratedField::ChainId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TransactionsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aptos.indexer.v1.TransactionsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TransactionsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut transactions__ = None;
                let mut chain_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Transactions => {
                            if transactions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transactions"));
                            }
                            transactions__ = Some(map.next_value()?);
                        }
                        GeneratedField::ChainId => {
                            if chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainId"));
                            }
                            chain_id__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(TransactionsResponse {
                    transactions: transactions__.unwrap_or_default(),
                    chain_id: chain_id__,
                })
            }
        }
        deserializer.deserialize_struct("aptos.indexer.v1.TransactionsResponse", FIELDS, GeneratedVisitor)
    }
}
