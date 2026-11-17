// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! The model catalog's order and choice.

use conversation_model::{Availability, ModelCatalog, ModelOption};

fn option(id: &str, availability: Availability) -> ModelOption {
    ModelOption {
        id: id.into(),
        name: id.to_uppercase(),
        remote: false,
        availability,
    }
}

#[test]
fn the_recommended_model_comes_first_then_what_can_answer() {
    let catalog = ModelCatalog::new(
        vec![
            option("small", Availability::Installed),
            option("large", Availability::NotInstalled),
            option("medium", Availability::Downloading),
            option("broken", Availability::Unavailable),
            option("tiny", Availability::Installed),
        ],
        Some("large".into()),
    );
    let ids: Vec<&str> = catalog.options().iter().map(|o| o.id.as_str()).collect();
    assert_eq!(ids, ["large", "small", "tiny", "medium", "broken"]);
    assert_eq!(catalog.recommended().unwrap().id, "large");
}

#[test]
fn a_choice_names_a_listed_model_that_can_answer_or_nothing() {
    let catalog = ModelCatalog::new(
        vec![
            option("small", Availability::Installed),
            option("large", Availability::NotInstalled),
        ],
        Some("large".into()),
    );
    // The recommended model is not installed, so the installed one answers.
    assert_eq!(catalog.choose(None).unwrap().id, "small");
    // A wanted model that is listed and installed wins.
    assert_eq!(catalog.choose(Some("small")).unwrap().id, "small");
    // A model the host did not list is never assumed.
    assert_eq!(catalog.choose(Some("ghost")).unwrap().id, "small");
    let empty = ModelCatalog::new(vec![option("large", Availability::NotInstalled)], None);
    assert!(empty.choose(Some("large")).is_none());
    assert!(ModelCatalog::default().choose(None).is_none());
}

#[test]
fn a_recommendation_the_host_did_not_list_is_dropped() {
    let catalog = ModelCatalog::new(
        vec![option("small", Availability::Installed)],
        Some("x".into()),
    );
    assert!(catalog.recommended().is_none());
}
