use kdl::KdlNode;
use serde::{Deserialize, Serialize};

use crate::error::UsageErr;
use crate::spec::context::ParsingContext;
use crate::spec::helpers::NodeHelper;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SpecRequiredIf {
    pub required_if: Vec<String>,
}

impl SpecRequiredIf {
    pub(crate) fn parse(_ctx: &ParsingContext, node: &NodeHelper) -> Result<Self, UsageErr> {
        let mut config = Self::default();

        node.ensure_arg_len(1..)?;
        config.required_if = node
            .args()
            .map(|requirement| requirement.ensure_string())
            .collect::<Result<_, _>>()?;

        Ok(config)
    }
}

impl From<&SpecRequiredIf> for KdlNode {
    fn from(arg: &SpecRequiredIf) -> Self {
        let mut node = KdlNode::new("required_if");
        for dep in &arg.required_if { node.push(dep.to_string()); }

        node
    }
}

