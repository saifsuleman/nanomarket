use crate::event::Event;

#[derive(Debug, Clone)]
pub struct EventLog {
    events: Vec<Event>,
}

impl EventLog {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn from_events(events: Vec<Event>) -> Self {
        Self { events }
    }

    pub fn append(&mut self, event: Event) -> usize {
        let index = self.events.len();
        self.events.push(event);
        index
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&Event> {
        self.events.get(index)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Event> {
        self.events.iter()
    }

    pub fn events(&self) -> &[Event] {
        &self.events
    }

    pub fn slice(&self, start: usize, end: usize) -> &[Event] {
        let end = end.min(self.events.len());
        let start = start.min(end);
        &self.events[start..end]
    }

    pub fn truncate(&mut self, len: usize) {
        self.events.truncate(len);
    }

    pub fn fork_at(&self, at: usize) -> Self {
        let at = at.min(self.events.len());
        Self {
            events: self.events[..at].to_vec(),
        }
    }

    pub fn replace(&mut self, index: usize, new_event: Event) -> Option<Event> {
        if index < self.events.len() {
            let old = std::mem::replace(&mut self.events[index], new_event);
            Some(old)
        } else {
            None
        }
    }

    pub fn splice_insert(&mut self, index: usize, event: Event) -> usize {
        let index = index.min(self.events.len());
        self.events.insert(index, event);
        self.events.len()
    }

    pub fn splice_remove(&mut self, index: usize) -> Option<Event> {
        if index < self.events.len() {
            Some(self.events.remove(index))
        } else {
            None
        }
    }
}

impl Default for EventLog {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for EventLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "EventLog ({} events):", self.events.len())?;
        for (i, event) in self.events.iter().enumerate() {
            writeln!(f, "  [{:>4}] {}", i, event)?;
        }
        Ok(())
    }
}
