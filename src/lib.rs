// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-autoconf: autoconf component model.
//!
//! Real implementation: autoconf_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AutoconfItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AutoconfError {
    EmptyAutoconfItemName,
    DuplicateAutoconfItem(String),
    EmptyEventSubject,
}

impl fmt::Display for AutoconfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyAutoconfItemName => write!(f, "autoconf_item name must not be empty"),
            Self::DuplicateAutoconfItem(id) => write!(f, "duplicate autoconf_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for AutoconfError {}

pub struct Autoconf {
    pub name: String,
    autoconf_items: BTreeMap<String, AutoconfItem>,
    events: BTreeMap<String, Event>,
}

impl Autoconf {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            autoconf_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_autoconf_item(&mut self, item: AutoconfItem) -> Result<(), AutoconfError> {
        if item.name.trim().is_empty() {
            return Err(AutoconfError::EmptyAutoconfItemName);
        }
        let id = item.id.clone();
        if self.autoconf_items.contains_key(&id) {
            return Err(AutoconfError::DuplicateAutoconfItem(id.clone()));
        }
        self.autoconf_items.insert(id, item);
        Ok(())
    }

    pub fn autoconf_item_count(&self) -> usize {
        self.autoconf_items.len()
    }

    pub fn get_autoconf_item(&self, id: &str) -> Option<&AutoconfItem> {
        self.autoconf_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), AutoconfError> {
        if event.subject.trim().is_empty() {
            return Err(AutoconfError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_autoconf_item(id: &str, name: &str) -> AutoconfItem {
        AutoconfItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Autoconf::new("Autoconf");
        assert_eq!(m.name, "Autoconf");
        assert_eq!(m.autoconf_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_autoconf_item() {
        let mut m = Autoconf::new("Autoconf");
        m.add_autoconf_item(sample_autoconf_item("a1", "Alpha")).unwrap();
        assert_eq!(m.autoconf_item_count(), 1);
        assert!(m.get_autoconf_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Autoconf::new("Autoconf");
        assert!(m.add_autoconf_item(sample_autoconf_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Autoconf::new("Autoconf");
        m.add_autoconf_item(sample_autoconf_item("x", "One")).unwrap();
        assert!(m.add_autoconf_item(sample_autoconf_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Autoconf::new("Autoconf");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
