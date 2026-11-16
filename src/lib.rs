// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-ldap-client: ldap client component model.
//!
//! Real implementation: clients, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Client {
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
pub enum LdapClientError {
    EmptyClientName,
    DuplicateClient(String),
    EmptyEventSubject,
}

impl fmt::Display for LdapClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyClientName => write!(f, "client name must not be empty"),
            Self::DuplicateClient(id) => write!(f, "duplicate client {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for LdapClientError {}

pub struct LdapClient {
    pub name: String,
    clients: BTreeMap<String, Client>,
    events: BTreeMap<String, Event>,
}

impl LdapClient {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            clients: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_client(&mut self, item: Client) -> Result<(), LdapClientError> {
        if item.name.trim().is_empty() {
            return Err(LdapClientError::EmptyClientName);
        }
        let id = item.id.clone();
        if self.clients.contains_key(&id) {
            return Err(LdapClientError::DuplicateClient(id.clone()));
        }
        self.clients.insert(id, item);
        Ok(())
    }

    pub fn client_count(&self) -> usize {
        self.clients.len()
    }

    pub fn get_client(&self, id: &str) -> Option<&Client> {
        self.clients.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), LdapClientError> {
        if event.subject.trim().is_empty() {
            return Err(LdapClientError::EmptyEventSubject);
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

    fn sample_client(id: &str, name: &str) -> Client {
        Client {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = LdapClient::new("LdapClient");
        assert_eq!(m.name, "LdapClient");
        assert_eq!(m.client_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_client() {
        let mut m = LdapClient::new("LdapClient");
        m.add_client(sample_client("a1", "Alpha")).unwrap();
        assert_eq!(m.client_count(), 1);
        assert!(m.get_client("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = LdapClient::new("LdapClient");
        assert!(m.add_client(sample_client("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = LdapClient::new("LdapClient");
        m.add_client(sample_client("x", "One")).unwrap();
        assert!(m.add_client(sample_client("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = LdapClient::new("LdapClient");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
