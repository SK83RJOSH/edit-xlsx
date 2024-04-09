use serde::{Deserialize, Serialize};
use crate::api::format::border::{FormatBorder, FormatBorderElement};
use crate::FormatBorderType;
use crate::xml::common::FromFormat;
use crate::xml::style::color::Color;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct Borders {
    #[serde(rename = "@count", default)]
    count: u32,
    border: Vec<Border>,
}

impl Borders {
    pub(crate) fn add_border(&mut self, border: &Border) -> u32 {
        for i in 0..self.border.len() {
            if self.border[i] == *border {
                return i as u32;
            }
        }
        self.count += 1;
        self.border.push(border.clone());
        self.border.len() as u32 - 1
    }

    pub(crate) fn get_border(&self, id: u32) -> Option<&Border> {
        self.border.get(id as usize)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub(crate) struct Border {
    pub(crate) left: BorderElement,
    pub(crate) right: BorderElement,
    pub(crate) top: BorderElement,
    pub(crate) bottom: BorderElement,
    pub(crate) diagonal: BorderElement,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub(crate) struct BorderElement {
    #[serde(rename = "@style", skip_serializing_if = "Option::is_none")]
    pub(crate) style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) color: Option<Color>,
}

impl Default for BorderElement {
    fn default() -> Self {
        BorderElement {
            style: None,
            color: Some(Color::default()),
        }
    }
}

impl Default for Border {
    fn default() -> Self {
        Border {
            left: BorderElement::default(),
            right: BorderElement::default(),
            top: BorderElement::default(),
            bottom: BorderElement::default(),
            diagonal: BorderElement::default(),
        }
    }
}

impl Default for Borders {
    fn default() -> Self {
        Borders {
            count: 0,
            border: vec![],
        }
    }
}
