use std::collections::VecDeque;
use std::time::{Duration, Instant};
use crossterm::event::{KeyCode, KeyModifiers};

const MAX_QUEUE_SIZE: usize = 32;
const EVENT_TIMEOUT: Duration = Duration::from_millis(100);

#[derive(Debug)]
pub struct EventQueue {
    events: VecDeque<KeyboardEvent>,
    last_processed: Option<Instant>,
    queue_size: usize,
}

#[derive(Debug, Clone)]
pub struct KeyboardEvent {
    pub key: KeyCode,
    pub modifiers: KeyModifiers,
    pub timestamp: Instant,
    pub processed: bool,
}

impl EventQueue {
    pub fn new() -> Self {
        Self {
            events: VecDeque::with_capacity(MAX_QUEUE_SIZE),
            last_processed: None,
            queue_size: MAX_QUEUE_SIZE,
        }
    }

    pub fn push(&mut self, event: KeyboardEvent) -> bool {
        if self.events.len() >= self.queue_size {
            self.events.pop_front(); // Remove oldest event if queue is full
        }
        self.events.push_back(event);
        true
    }

    pub fn pop(&mut self) -> Option<KeyboardEvent> {
        self.last_processed = Some(Instant::now());
        self.events.pop_front()
    }

    pub fn peek(&self) -> Option<&KeyboardEvent> {
        self.events.front()
    }

    pub fn clear(&mut self) {
        self.events.clear();
        self.last_processed = None;
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn cleanup_old_events(&mut self) {
        let now = Instant::now();
        while let Some(event) = self.events.front() {
            if now.duration_since(event.timestamp) > EVENT_TIMEOUT {
                self.events.pop_front();
            } else {
                break;
            }
        }
    }
}

impl KeyboardEvent {
    pub fn new(key: KeyCode, modifiers: KeyModifiers) -> Self {
        Self {
            key,
            modifiers,
            timestamp: Instant::now(),
            processed: false,
        }
    }

    pub fn is_expired(&self) -> bool {
        Instant::now().duration_since(self.timestamp) > EVENT_TIMEOUT
    }
} 