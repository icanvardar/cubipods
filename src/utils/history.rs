use std::error::Error;

use crate::instruction::InstructionType;

use super::{bytes::from_u8_32, errors::HistoryError};

#[derive(Debug)]
pub struct History {
    registry: Vec<Registry>,
}

#[derive(Debug)]
pub struct Registry {
    pub description: String,
    pub component: Component,
}

#[derive(Debug)]
pub struct StackInfo {
    pub instruction: InstructionType,
    pub item_1: Option<[u8; 32]>,
    pub item_1_index: Option<u16>,
    pub item_2: Option<[u8; 32]>,
    pub item_2_index: Option<u16>,
}

#[derive(Debug)]
pub struct MemoryInfo {
    pub location: usize,
    pub value: [u8; 32],
}

#[derive(Debug)]
pub struct StorageInfo {
    pub slot: [u8; 32],
    pub value: [u8; 32],
}

#[derive(Debug)]
pub enum Component {
    Stack(StackInfo),
    Memory(MemoryInfo),
    Storage(StorageInfo),
}

impl Registry {
    pub fn new(description: String, component: Component) -> Result<Self, HistoryError> {
        if description.is_empty() {
            return Err(HistoryError::EmptyDescription);
        }

        Ok(Self {
            description,
            component,
        })
    }
}

impl History {
    pub fn new() -> Self {
        Self {
            registry: Vec::new(),
        }
    }

    pub fn save_on_event(&mut self, component: Component) -> Result<(), Box<dyn Error>> {
        match &component {
            Component::Stack(info) => {
                let format_item_info = |item: [u8; 32], index: u16| -> String {
                    let item: String = from_u8_32(item);

                    format!("the item {item} with the index of {index}")
                };

                let mut description = format!(
                    "[STACK]: The opcode {:?} was called by using {}",
                    info.instruction,
                    format_item_info(info.item_1.unwrap(), info.item_1_index.unwrap())
                );

                if !info.item_2.is_none() {
                    description = format!(
                        "{} and {}",
                        description,
                        format_item_info(info.item_2.unwrap(), info.item_2_index.unwrap())
                    );
                }

                self.registry
                    .push(Registry::new(format!("{}.", description), component)?);
            }
            Component::Memory(info) => {
                let description = format!(
                    "[MEMORY]: The value {} was pushed to the location of {}.",
                    from_u8_32::<String>(info.value),
                    info.location.to_string()
                );

                self.registry.push(Registry::new(description, component)?);
            }
            Component::Storage(info) => {
                let description = format!(
                    "[STORAGE]: The value {} was pushed to the storage slot of {}.",
                    from_u8_32::<String>(info.value),
                    from_u8_32::<String>(info.slot),
                );

                self.registry.push(Registry::new(description, component)?);
            }
        }

        Ok(())
    }

    pub fn summarize(self) {
        println!(
            "{}",
            self.registry
                .iter()
                .map(|r| r.description.clone() + "\n")
                .collect::<String>()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_on_event() {
        let mut history = History::new();
    }
}
