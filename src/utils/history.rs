use std::error::Error;

use crate::instruction::InstructionType;

use super::{bytes::from_u8_32, errors::HistoryError};

#[derive(Debug, Default)]
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

                if info.item_2.is_some() {
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
                    info.location
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

    pub fn size(&self) -> usize {
        self.registry.len()
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

impl Component {
    pub fn build_stack(
        instruction: InstructionType,
        item_1: [u8; 32],
        item_1_index: u16,
        item_2: [u8; 32],
        item_2_index: u16,
    ) -> Self {
        Component::Stack(StackInfo {
            instruction,
            item_1: Some(item_1),
            item_1_index: Some(item_1_index),
            item_2: Some(item_2),
            item_2_index: Some(item_2_index),
        })
    }

    pub fn build_stack_with_one_item(
        instruction: InstructionType,
        item_1: [u8; 32],
        item_1_index: u16,
    ) -> Self {
        Component::Stack(StackInfo {
            instruction,
            item_1: Some(item_1),
            item_1_index: Some(item_1_index),
            item_2: None,
            item_2_index: None,
        })
    }

    pub fn build_memory(location: usize, value: [u8; 32]) -> Self {
        Component::Memory(MemoryInfo { location, value })
    }

    pub fn build_storage(slot: [u8; 32], value: [u8; 32]) -> Self {
        Component::Storage(StorageInfo { slot, value })
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::bytes::to_u8_32;

    use super::*;

    #[test]
    fn test_save_on_event() -> Result<(), Box<dyn Error>> {
        let mut history = History::new();

        history.save_on_event(Component::Stack(StackInfo {
            instruction: InstructionType::PUSH(1),
            item_1: Some(to_u8_32(&"01".to_string())),
            item_1_index: Some(2),
            item_2: None,
            item_2_index: None,
        }))?;
        history.save_on_event(Component::Stack(StackInfo {
            instruction: InstructionType::PUSH(3),
            item_1: Some(to_u8_32(&"010203".to_string())),
            item_1_index: Some(1),
            item_2: None,
            item_2_index: None,
        }))?;
        history.save_on_event(Component::Stack(StackInfo {
            instruction: InstructionType::MSTORE,
            item_1: Some(to_u8_32(&"01".to_string())),
            item_1_index: Some(2),
            item_2: Some(to_u8_32(&"010203".to_string())),
            item_2_index: Some(1),
        }))?;
        history.save_on_event(Component::Memory(MemoryInfo {
            location: 1,
            value: to_u8_32(&"010203".to_string()),
        }))?;
        history.save_on_event(Component::Storage(StorageInfo {
            slot: to_u8_32(&"01".to_string()),
            value: to_u8_32(&"010203".to_string()),
        }))?;

        assert_eq!(history.registry.len(), 5);

        Ok(())
    }

    #[test]
    fn it_creates_registry_with_empty_description_returns_history_error(
    ) -> Result<(), Box<dyn Error>> {
        let result = Registry::new(
            "".to_string(),
            Component::Stack(StackInfo {
                instruction: InstructionType::STOP,
                item_1: None,
                item_1_index: None,
                item_2: None,
                item_2_index: None,
            }),
        );
        assert!(matches!(result, Err(HistoryError::EmptyDescription)));

        Ok(())
    }

    #[test]
    fn test_build_stack() {
        let stack_component =
            Component::build_stack(InstructionType::ADD, to_u8_32(1), 1, to_u8_32(1), 2);

        if let Component::Stack(stack_info) = stack_component {
            assert_eq!(stack_info.item_1.is_some(), true);
            assert_eq!(stack_info.item_1_index.is_some(), true);
            assert_eq!(stack_info.item_2.is_some(), true);
            assert_eq!(stack_info.item_2_index.is_some(), true);
        }
    }

    #[test]
    fn test_build_stack_with_one_item() {
        let stack_component =
            Component::build_stack_with_one_item(InstructionType::ADD, to_u8_32(1), 1);

        if let Component::Stack(stack_info) = stack_component {
            assert_eq!(stack_info.item_1.is_some(), true);
            assert_eq!(stack_info.item_1_index.is_some(), true);
            assert_eq!(stack_info.item_2.is_some(), false);
            assert_eq!(stack_info.item_2_index.is_some(), false);
        }
    }

    #[test]
    fn test_build_memory() {
        let memory_component = Component::build_memory(1, to_u8_32(1));

        if let Component::Memory(memory_info) = memory_component {
            assert_eq!(memory_info.location, 1);
            assert_eq!(memory_info.value, to_u8_32(1));
        }
    }

    #[test]
    fn test_build_storage() {
        let storage_component = Component::build_storage(to_u8_32(1), to_u8_32(1));

        if let Component::Storage(storage_info) = storage_component {
            assert_eq!(storage_info.slot, to_u8_32(1));
            assert_eq!(storage_info.value, to_u8_32(1));
        }
    }
}
