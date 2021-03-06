use item::Item;
use types::Gold;

/// A single slot of the inventory
#[derive(Debug)]
pub struct InventorySlot {
    item: Item,
    amount: usize,
}

/// An inventory holding items
#[derive(Debug)]
pub struct Inventory {
    contents: Vec<InventorySlot>,
    gold: Gold,
    max_size: usize,
}

impl Inventory {
    /// Creates a new instance of `Inventory`
    pub fn new(max_size: usize) -> Inventory {
        Inventory {
            contents: Vec::new(),
            max_size: max_size,
            gold: 0,
        }
    }

    /// Adds an item to the inventory. If the inventory is full, the item won't be added to the
    /// inventory and a `Err(Item)` is returned.
    pub fn add_item(&mut self, new_item: Item) -> Result<(), Item> {
        for slot in &mut self.contents {
            if slot.item == new_item {
                if slot.item.stack_size > slot.amount {
                    slot.amount += 1;
                    return Ok(());
                }
            }
        }

        if self.contents.len() < self.max_size {
            self.contents.push(InventorySlot {
                item: new_item,
                amount: 1,
            });
        } else {
            return Err(new_item);
        }

        Ok(())
    }

    /// Returns `true` it the inventory is full
    pub fn is_full(&self) -> bool {
        self.contents.len() == self.max_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use item_generator;
    use item::ItemType;

    #[test]
    fn inventory_full() {
        let mut inv_1 = Inventory::new(30);

        for _ in 0..40 {
            let random_item =
                item_generator::ItemGenerator::new().item_type(ItemType::ArmorHead).gen();
            if let Err(_) = inv_1.add_item(random_item) {
                assert!(inv_1.is_full());
            }
        }
    }

    #[test]
    fn stackability() {
        let mut inv = Inventory::new(30);

        let random_item_1 =
            item_generator::ItemGenerator::new().item_type(ItemType::ConsumablePotion).gen();
        for _ in 0..random_item_1.stack_size {
            let _ = inv.add_item(random_item_1.clone());
        }

        assert_eq!(inv.contents[0].amount, random_item_1.stack_size);

        for _ in 0..(random_item_1.stack_size / 4) {
            let _ = inv.add_item(random_item_1.clone());
        }

        assert_eq!(inv.contents[1].amount, random_item_1.stack_size / 4);
    }
}
