// Generated from definition io.k8s.api.core.v1.NodeSpec

/// NodeSpec describes the attributes that a node is created with.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodeSpec {
    /// If specified, the source to get node configuration from The DynamicKubeletConfig feature gate must be enabled for the Kubelet to use this field
    pub config_source: Option<crate::api::core::v1::NodeConfigSource>,

    /// Deprecated. Not all kubelets will set this field. Remove field after 1.13. see: https://issues.k8s.io/61966
    pub external_id: Option<String>,

    /// PodCIDR represents the pod IP range assigned to the node.
    pub pod_cidr: Option<String>,

    /// podCIDRs represents the IP ranges assigned to the node for usage by Pods on that node. If this field is specified, the 0th entry must match the podCIDR field. It may contain at most 1 value for each of IPv4 and IPv6.
    pub pod_cidrs: Option<Vec<String>>,

    /// ID of the node assigned by the cloud provider in the format: \<ProviderName\>://\<ProviderSpecificNodeID\>
    pub provider_id: Option<String>,

    /// If specified, the node's taints.
    pub taints: Option<Vec<crate::api::core::v1::Taint>>,

    /// Unschedulable controls node schedulability of new pods. By default, node is schedulable. More info: https://kubernetes.io/docs/concepts/nodes/node/#manual-node-administration
    pub unschedulable: Option<bool>,
}

impl<'de> crate::serde::Deserialize<'de> for NodeSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_config_source,
            Key_external_id,
            Key_pod_cidr,
            Key_pod_cidrs,
            Key_provider_id,
            Key_taints,
            Key_unschedulable,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "configSource" => Field::Key_config_source,
                            "externalID" => Field::Key_external_id,
                            "podCIDR" => Field::Key_pod_cidr,
                            "podCIDRs" => Field::Key_pod_cidrs,
                            "providerID" => Field::Key_provider_id,
                            "taints" => Field::Key_taints,
                            "unschedulable" => Field::Key_unschedulable,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = NodeSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("NodeSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_config_source: Option<crate::api::core::v1::NodeConfigSource> = None;
                let mut value_external_id: Option<String> = None;
                let mut value_pod_cidr: Option<String> = None;
                let mut value_pod_cidrs: Option<Vec<String>> = None;
                let mut value_provider_id: Option<String> = None;
                let mut value_taints: Option<Vec<crate::api::core::v1::Taint>> = None;
                let mut value_unschedulable: Option<bool> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_config_source => value_config_source = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_external_id => value_external_id = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pod_cidr => value_pod_cidr = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pod_cidrs => value_pod_cidrs = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_provider_id => value_provider_id = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_taints => value_taints = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_unschedulable => value_unschedulable = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NodeSpec {
                    config_source: value_config_source,
                    external_id: value_external_id,
                    pod_cidr: value_pod_cidr,
                    pod_cidrs: value_pod_cidrs,
                    provider_id: value_provider_id,
                    taints: value_taints,
                    unschedulable: value_unschedulable,
                })
            }
        }

        deserializer.deserialize_struct(
            "NodeSpec",
            &[
                "configSource",
                "externalID",
                "podCIDR",
                "podCIDRs",
                "providerID",
                "taints",
                "unschedulable",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for NodeSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NodeSpec",
            self.config_source.as_ref().map_or(0, |_| 1) +
            self.external_id.as_ref().map_or(0, |_| 1) +
            self.pod_cidr.as_ref().map_or(0, |_| 1) +
            self.pod_cidrs.as_ref().map_or(0, |_| 1) +
            self.provider_id.as_ref().map_or(0, |_| 1) +
            self.taints.as_ref().map_or(0, |_| 1) +
            self.unschedulable.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.config_source {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "configSource", value)?;
        }
        if let Some(value) = &self.external_id {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "externalID", value)?;
        }
        if let Some(value) = &self.pod_cidr {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "podCIDR", value)?;
        }
        if let Some(value) = &self.pod_cidrs {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "podCIDRs", value)?;
        }
        if let Some(value) = &self.provider_id {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "providerID", value)?;
        }
        if let Some(value) = &self.taints {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "taints", value)?;
        }
        if let Some(value) = &self.unschedulable {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "unschedulable", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for NodeSpec {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.NodeSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("NodeSpec describes the attributes that a node is created with.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: IntoIterator::into_iter([
                    (
                        "configSource".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::NodeConfigSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("If specified, the source to get node configuration from The DynamicKubeletConfig feature gate must be enabled for the Kubelet to use this field".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "externalID".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Deprecated. Not all kubelets will set this field. Remove field after 1.13. see: https://issues.k8s.io/61966".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "podCIDR".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("PodCIDR represents the pod IP range assigned to the node.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "podCIDRs".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("podCIDRs represents the IP ranges assigned to the node for usage by Pods on that node. If this field is specified, the 0th entry must match the podCIDR field. It may contain at most 1 value for each of IPv4 and IPv6.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "providerID".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ID of the node assigned by the cloud provider in the format: <ProviderName>://<ProviderSpecificNodeID>".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "taints".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("If specified, the node's taints.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::Taint>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "unschedulable".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Unschedulable controls node schedulability of new pods. By default, node is schedulable. More info: https://kubernetes.io/docs/concepts/nodes/node/#manual-node-administration".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                ]).collect(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
