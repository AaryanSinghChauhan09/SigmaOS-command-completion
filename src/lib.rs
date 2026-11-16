// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-command-completion: command completion component model.
//!
//! Real implementation: completions, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Completion {
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
pub enum CommandCompletionError {
    EmptyCompletionName,
    DuplicateCompletion(String),
    EmptyEventSubject,
}

impl fmt::Display for CommandCompletionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyCompletionName => write!(f, "completion name must not be empty"),
            Self::DuplicateCompletion(id) => write!(f, "duplicate completion {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for CommandCompletionError {}

pub struct CommandCompletion {
    pub name: String,
    completions: BTreeMap<String, Completion>,
    events: BTreeMap<String, Event>,
}

impl CommandCompletion {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            completions: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_completion(&mut self, item: Completion) -> Result<(), CommandCompletionError> {
        if item.name.trim().is_empty() {
            return Err(CommandCompletionError::EmptyCompletionName);
        }
        let id = item.id.clone();
        if self.completions.contains_key(&id) {
            return Err(CommandCompletionError::DuplicateCompletion(id.clone()));
        }
        self.completions.insert(id, item);
        Ok(())
    }

    pub fn completion_count(&self) -> usize {
        self.completions.len()
    }

    pub fn get_completion(&self, id: &str) -> Option<&Completion> {
        self.completions.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), CommandCompletionError> {
        if event.subject.trim().is_empty() {
            return Err(CommandCompletionError::EmptyEventSubject);
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

    fn sample_completion(id: &str, name: &str) -> Completion {
        Completion {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = CommandCompletion::new("CommandCompletion");
        assert_eq!(m.name, "CommandCompletion");
        assert_eq!(m.completion_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_completion() {
        let mut m = CommandCompletion::new("CommandCompletion");
        m.add_completion(sample_completion("a1", "Alpha")).unwrap();
        assert_eq!(m.completion_count(), 1);
        assert!(m.get_completion("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = CommandCompletion::new("CommandCompletion");
        assert!(m.add_completion(sample_completion("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = CommandCompletion::new("CommandCompletion");
        m.add_completion(sample_completion("x", "One")).unwrap();
        assert!(m.add_completion(sample_completion("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = CommandCompletion::new("CommandCompletion");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
