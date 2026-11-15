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
