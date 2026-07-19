use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{resource_location::ResourceLocation, types::Double};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(deny_unknown_fields)]
pub enum TintSource {
    #[serde(rename = "minecraft:constant")]
    Constant { value: i32 },
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum SelectItemModelWhen {
    Gui,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SelectItemModelCase {
    model: ItemModel,
    when: SelectItemModelWhen,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum SelectItemModelProperty {
    #[serde(rename = "minecraft:display_context")]
    DisplayContext,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum BooleanProperty {
    #[serde(rename = "minecraft:bundle/has_selected_item")]
    BundleHasSelectedItem,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(deny_unknown_fields)]
pub enum ItemModel {
    #[serde(rename = "minecraft:model")]
    Model {
        model: ResourceLocation,
        tints: Option<Vec<TintSource>>,
        transformation: Option<HashMap<ResourceLocation, Vec<Double>>>,
    },
    #[serde(rename = "minecraft:select")]
    Select {
        cases: Vec<SelectItemModelCase>,
        fallback: Box<Self>,
        property: SelectItemModelProperty,
    },
    #[serde(rename = "minecraft:condition")]
    Condition {
        on_false: Box<Self>,
        on_true: Box<Self>,
        property: BooleanProperty,
    },
    #[serde(rename = "minecraft:composite")]
    Composite { models: Vec<Self> },
    #[serde(rename = "minecraft:bundle/selected_item")]
    BundleSelectedItem,
    #[serde(rename = "minecraft:special")]
    Special,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ItemModelDefinition {
    pub model: ItemModel,
}
