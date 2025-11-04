use crate::components::events::{EventChoice, EventEffect, EventTriggerType};
use crate::resources::{EventDefinition, EventDefinitions};
use serde::Deserialize;
use std::fs;

/// Internal representation of event data as loaded from RON files.
///
/// This struct matches the structure of event data files and is used
/// during deserialization. It's converted to [`EventDefinition`] after loading.
#[derive(Debug, Deserialize)]
struct EventDataCollection {
    events: Vec<EventDataDefinition>,
}

/// Single event definition as stored in RON format.
#[derive(Debug, Deserialize)]
struct EventDataDefinition {
    id: String,
    title: String,
    description: String,
    trigger: EventTriggerType,
    effects: Vec<EventEffect>,
    choices: Vec<EventChoice>,
}

/// Loader for event definitions from RON files.
///
/// Events are defined in `dominion_earth/assets/data/events.ron` using RON format.
/// The loader deserializes the file and converts it into [`EventDefinitions`] resource.
pub struct EventDataLoader;

impl EventDataLoader {
    /// Loads event definitions from a RON file.
    ///
    /// # Arguments
    /// * `path` - Path to the RON file containing event definitions
    ///
    /// # Returns
    /// * `Ok(EventDefinitions)` - Successfully loaded events
    /// * `Err` - File I/O error or RON parsing error
    ///
    /// # Example
    /// ```no_run
    /// use core_sim::EventDataLoader;
    ///
    /// match EventDataLoader::load_from_ron("dominion_earth/assets/data/events.ron") {
    ///     Ok(events) => println!("Loaded {} events", events.events.len()),
    ///     Err(e) => eprintln!("Failed to load events: {}", e),
    /// }
    /// ```
    pub fn load_from_ron(path: &str) -> Result<EventDefinitions, Box<dyn std::error::Error>> {
        let file_content = fs::read_to_string(path)?;
        let data: EventDataCollection = ron::from_str(&file_content)?;

        let events = data
            .events
            .into_iter()
            .map(|e| EventDefinition {
                id: e.id,
                title: e.title,
                description: e.description,
                trigger: e.trigger,
                effects: e.effects,
                choices: e.choices,
            })
            .collect();

        Ok(EventDefinitions { events })
    }
}
