// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! The models a person may pick from, in the order to offer them.

/// Whether a model can answer now.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Availability {
    /// On this machine (or remote and enabled): it can answer.
    Installed,
    /// Its file is being fetched.
    Downloading,
    /// A local model whose file is not here yet.
    NotInstalled,
    /// It crashed too often; the engine stopped trying.
    Unavailable,
    /// The host could not say.
    Unknown,
}

impl Availability {
    /// Whether a prompt sent to it can be answered now.
    pub fn can_answer(self) -> bool {
        self == Availability::Installed
    }
}

/// One model the caller may use.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModelOption {
    /// The host's id, used in every call.
    pub id: String,
    /// The name shown to a person.
    pub name: String,
    /// Runs at a remote provider rather than on this machine.
    pub remote: bool,
    /// Whether it can answer now.
    pub availability: Availability,
}

/// The models to offer: chat models only, the recommended one first, then
/// those that can answer, then the rest. Nothing is assumed to exist: a
/// selection names a model the host listed, or nothing.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ModelCatalog {
    options: Vec<ModelOption>,
    recommended: Option<String>,
}

impl ModelCatalog {
    /// A catalog of what the host listed, with the host's recommendation if
    /// it gave one.
    pub fn new(mut options: Vec<ModelOption>, recommended: Option<String>) -> Self {
        let rank = |option: &ModelOption| {
            let first = recommended.as_deref() != Some(option.id.as_str());
            (first, option.availability, option.remote)
        };
        options.sort_by(|a, b| rank(a).cmp(&rank(b)).then_with(|| a.name.cmp(&b.name)));
        let recommended = recommended.filter(|id| options.iter().any(|o| &o.id == id));
        Self {
            options,
            recommended,
        }
    }

    /// The options, in the order to offer them.
    pub fn options(&self) -> &[ModelOption] {
        &self.options
    }

    /// The recommended model, if the host named one it also listed.
    pub fn recommended(&self) -> Option<&ModelOption> {
        let id = self.recommended.as_deref()?;
        self.get(id)
    }

    /// A listed model by id.
    pub fn get(&self, id: &str) -> Option<&ModelOption> {
        self.options.iter().find(|o| o.id == id)
    }

    /// Record a model's availability as the host now reports it. `false`
    /// if the model is not listed or nothing changed.
    pub fn set_availability(&mut self, id: &str, availability: Availability) -> bool {
        match self.options.iter_mut().find(|o| o.id == id) {
            Some(option) if option.availability != availability => {
                option.availability = availability;
                true
            }
            _ => false,
        }
    }

    /// The models that are on their way: downloading now.
    pub fn downloading(&self) -> impl Iterator<Item = &ModelOption> {
        self.options
            .iter()
            .filter(|o| o.availability == Availability::Downloading)
    }

    /// The model to use: `wanted` if it is listed and can answer, else the
    /// recommended one if it can, else the first that can, else none.
    pub fn choose(&self, wanted: Option<&str>) -> Option<&ModelOption> {
        let usable = |option: &&ModelOption| option.availability.can_answer();
        wanted
            .and_then(|id| self.get(id))
            .filter(usable)
            .or_else(|| self.recommended().filter(usable))
            .or_else(|| self.options.iter().find(usable))
    }
}

/// A model's name as a person reads it: the host's name without a trailing
/// quantisation tag, so "Qwen3.5 2B (Q4_K_M)" reads "Qwen3.5 2B". The id is
/// untouched; only what is shown changes.
pub fn display_name(name: &str) -> String {
    let name = name.trim();
    let quantisation = |tag: &str| {
        let tag = tag.trim().to_ascii_uppercase();
        let tag = tag.strip_prefix('I').unwrap_or(&tag);
        let numbered = |prefix: &str| {
            tag.strip_prefix(prefix)
                .is_some_and(|rest| rest.starts_with(|c: char| c.is_ascii_digit()))
        };
        numbered("Q") || numbered("F") || numbered("BF") || numbered("FP") || numbered("INT")
    };
    if let Some(open) = name.rfind(" (")
        && name.ends_with(')')
        && quantisation(&name[open + 2..name.len() - 1])
    {
        return name[..open].trim_end().to_owned();
    }
    if let Some(space) = name.rfind(' ')
        && quantisation(&name[space + 1..])
    {
        return name[..space].trim_end().to_owned();
    }
    name.to_owned()
}

#[cfg(test)]
mod tests {
    use super::display_name;

    #[test]
    fn a_quantisation_tag_is_not_part_of_the_name() {
        assert_eq!(display_name("Qwen3.5 2B (Q4_K_M)"), "Qwen3.5 2B");
        assert_eq!(display_name("Llama 3.2 3B (IQ4_XS)"), "Llama 3.2 3B");
        assert_eq!(display_name("Gemma 3 4B BF16"), "Gemma 3 4B");
        assert_eq!(display_name("Phi 4 (f16)"), "Phi 4");
        // Other words in brackets, and names without a tag, stay.
        assert_eq!(display_name("Medium (recommended)"), "Medium (recommended)");
        assert_eq!(display_name("Qwen3.5 2B"), "Qwen3.5 2B");
        assert_eq!(display_name("Mistral Small 3"), "Mistral Small 3");
    }
}
