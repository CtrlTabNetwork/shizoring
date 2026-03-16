use std::{collections::{HashMap, HashSet}, env};

use dialoguer::{MultiSelect, Select};
use er_save_lib::{Item, ItemType, SaveApi, StorageItemType, StorageType};

const BLACKLIST: [u32; 1] = [110000]; // 110000 = Unarmed (For some reason it's an item)

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let file_name = match env::args().nth(1) { 
		Some(name) => name,
		None => "ER0000.sl2".to_string()
	};

	let extension = file_name.split(".").last().unwrap();

	let mut save_api = SaveApi::from_path(file_name.clone())?;

	let max_index = save_api.active_characters()
		.into_iter().filter(|b| *b).count();

	let names = (0..max_index)
		.map(|i| save_api.character_name(i))
		.collect::<Vec<_>>();

	let index = Select::new()
	    .with_prompt("Select character")
		.items(&names)
		.interact()?;

	let result = MultiSelect::new()
		.with_prompt("Select items to deduplicate")
		.item("Armors")
		.item("Weapons (down to 2 copies)")
		.item("Talismans (except Sacrificial Twig)")
		.defaults(&[true, true, true])
		.interact()?;

	let dedup_armors = result.contains(&0);
	let dedup_weapons = result.contains(&1);
	let dedup_talismans = result.contains(&2);

	let inventory = save_api.get_inventory(index, StorageType::Held, StorageItemType::Regular)?.clone();

	let mut seen = HashMap::<u32, HashSet<Item>>::new();
	
	for item in inventory.iter() {
		if BLACKLIST.contains(&item.item_id) {
			continue;
		}

		if !match item.item_type {
			ItemType::Armor => dedup_armors,
			ItemType::Weapon => dedup_weapons,
			ItemType::Accessory => dedup_talismans,
			_ => continue
		} {
			continue;
		}

		let copies = match item.item_type {
			ItemType::Armor => 1,
			ItemType::Weapon => 2,
			ItemType::Accessory => 1,
			_ => continue
		};

		seen.entry(item.item_id)
			.and_modify(|e| {
				e.insert(item.clone());
			})
			.or_insert([item.clone()].into());

		if seen[&item.item_id].len() > copies {
			let category = match item.item_type {
				ItemType::Armor => "Armor",
				ItemType::Weapon => "Weapon",
				ItemType::Accessory => "Talisman",
				_ => "Other"
			};
			
			print!("Duplicate item: {} ({}) (ID: {})", item.item_name, category, item.item_id);
			if save_api.is_equipped_item_id(index, &item) {

				let not_equipped: Vec<_> = seen[&item.item_id].iter().cloned()
					.filter(|i| !save_api.is_equipped(index, i)).collect();

				for i in not_equipped.iter() {
					save_api.remove_item(index, StorageType::Held, StorageItemType::Regular, &i);
					seen.get_mut(&item.item_id).unwrap().remove(&i);
				}
				if not_equipped.is_empty() {
					println!(" [skipped, equipped]");
				} else {
					println!();
				}
				
				continue;
			}

			if item.item_id == 6070 { // Sacrificial Twig
				println!(" [skipped]");
				continue;
			}

			println!();
			save_api.remove_item(index, StorageType::Held, StorageItemType::Regular, &item);
		}
	}

	// save_api.sanitize_inventories(index);
	save_api.write_to_path("./deduped.".to_string() + &extension)?;

	SaveApi::from_path("./deduped.".to_string() + &extension)?;
			
	Ok(())
}
