use std::fs;
use std::iter;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

fn part1(foods: &Vec<(BTreeSet<String>, BTreeSet<String>)>) -> usize {
	let mut known_allergens: BTreeMap<String, String> = BTreeMap::new();
	let mut known_ingredients: BTreeMap<String, String> = BTreeMap::new();
	let mut possible_ingredients: BTreeMap<BTreeSet<String>, BTreeSet<String>> = BTreeMap::new();
	let mut all_ingredients = BTreeSet::new();
	let mut all_allergens = BTreeSet::new();
	for (allergens, ingredients) in foods {
		let ingredients = if possible_ingredients.contains_key(&allergens) {
			let set: BTreeSet<String> = ingredients.intersection(possible_ingredients.get(&allergens).unwrap()).cloned().collect();
			set
		} else {
			ingredients.clone()
		};
		possible_ingredients.insert(allergens.clone(), ingredients.clone());
		for ingredient in ingredients {
			all_ingredients.insert(ingredient);
		}
		for allergen in allergens {
			all_allergens.insert(allergen.clone());
		}
	}
	for allergen in all_allergens {
		let allergens = iter::once(allergen.clone()).collect();
		if !possible_ingredients.contains_key(&allergens) {
			possible_ingredients.insert(allergens, all_ingredients.clone());
		}
	}
	let mut changed = true;
	while changed {
		// println!("{:?}", possible_ingredients);
		changed = false;
		'outer: for (allergens, ingredients) in possible_ingredients.clone() {
			if ingredients.len() == allergens.len() {
				//possible_ingredients.remove(&allergens);
				let allergen = allergens.iter().next().unwrap();
				for ingredient in &ingredients {
					known_ingredients.insert(allergen.clone(), ingredient.clone());
					if allergens.len() == 1 {
						known_allergens.insert(ingredient.clone(), allergen.clone());
						for (other_allergens, _) in possible_ingredients.clone() {
							if !allergens.is_subset(&other_allergens) {
								changed |= possible_ingredients.get_mut(&other_allergens).unwrap().remove(ingredient);
							}
						}
					}
				}
				if changed {
					break;
				}
			}
			for (other_allergens, other_ingredients) in possible_ingredients.clone() {
				if allergens.is_subset(&other_allergens) && allergens != other_allergens {
					let shared_ingredients = ingredients.intersection(&other_ingredients);
					if shared_ingredients.clone().count() != ingredients.len() {
						possible_ingredients.insert(allergens, shared_ingredients.cloned().collect());
						changed = true;
						break 'outer;
					}
				}
			}
		}
	}
	// println!("{:?}", known_ingredients);
	// println!("{:?}", known_allergens);
	let mut non_allergens = BTreeSet::new();
	for ingredient in &all_ingredients {
		let mut found = false;
		for (allergens, ingredients) in &possible_ingredients {
			if allergens.len() == 1 && ingredients.contains(ingredient) {
				found = true;
				break;
			}
		}
		if !found && !known_allergens.contains_key(ingredient) {
			non_allergens.insert(ingredient);
		}
	}
	let mut count = 0;
	for (_, ingredients) in foods {
		for ingredient in ingredients {
			if non_allergens.contains(ingredient) {
				// println!("{:?}", ingredient);
				count += 1;
			}
		}
	}
	for (allergens, ingredients) in possible_ingredients {
		if allergens.len() == 1 {
			print!("{},", ingredients.iter().clone().next().unwrap());
		}
	}
	println!("");
	count
}

fn main() {
    let contents = fs::read_to_string("input").expect("Couldn't read file");

    let mut foods = Vec::new();

    for line in contents.split("\n") {
    	let mut split_parens = line.split(" (contains ");
    	let ingredients_str = split_parens.next().unwrap();
    	let allergens_str = split_parens.next().unwrap();
    	let allergens_str = &allergens_str[..allergens_str.len() - 1];
    	let ingredients: BTreeSet<String> = ingredients_str.split(" ").map(|s| s.to_string()).collect();
    	let allergens: BTreeSet<String> = allergens_str.split(", ").map(|s| s.to_string()).collect();
    	foods.push((allergens, ingredients));
    }

    println!("{}", part1(&foods));
}
