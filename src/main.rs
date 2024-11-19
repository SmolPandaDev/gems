use std::collections::{HashMap, HashSet};


fn main() {
    let gems: Vec<String> = vec!["Diamond".into(), "Silver".into(), "Ruby".into(), "Emerald".into()];
    println!("Hello, world!");

    print_gem_colours(&gems);
    print_gem_colours2(&gems);
    
    let gems_hs: HashSet<&String> = gems.iter().collect();
    dbg!(gems_hs);

    let items = vec!["Gold".into(), "Rock".into(), "Sand".into(), "Gold".into(), "Plastic".into(), "Diamond".into()];
    dbg!(sieve_minerals_for_gold(&items));

    dbg!(sieve_for_gems(&items));

    let christmas_stockings = pairs_of_socks(vec!["Coal".into(), "Socks".into(), "Candy".into()]);
    dbg!(christmas_stockings);


    let mut backpack: HashMap<String, Vec<String>> = HashMap::new();

    backpack.entry("Treasure".into())
        .or_insert_with(Vec::new)
        .push("Emerald".into());

    backpack.entry("Junk".into())
        .or_insert_with(Vec::new)
        .push("Socks".into());

    backpack.entry("Treasure".into())
        .or_insert_with(Vec::new)
        .push("Ruby".into());

    dbg!(backpack);


    // We can do the above backpack in a more declarative way with fold
    let stuff = vec!["Pokeball".into(), "Berries".into(), "Masterball".into(), "Pokedex".into(), "Old rod".into()];
    let organised_backpack: HashMap<String, Vec<String>> = stuff.into_iter()
        .fold(HashMap::new(), |mut acc, item| {
            if item == "Pokeball" || item == "Masterball" {
                acc.entry("Balls".into()).or_insert_with(Vec::new).push(item);
            } else {
                acc.entry("Equipment".into())
                .or_insert_with(Vec::new).push(item);
            }
            acc
        });

    dbg!(organised_backpack);
        

}


fn print_gem_colours(gems: &Vec<String>) {
    let item_colors_result: Result<Vec<String>, String> = gems
        .into_iter()
        .map(|item| {
            match item.as_str() {
                "Diamond" => Ok("Clear".into()),
                "Ruby" => Ok("Red".into()),
                "Emerald" => Ok("Green".into()),
                "Platinum" => Ok("Blue".into()),
                "Onyx" => Ok("Black".into()),
                _ => Err("Unknown gem".into())
            }
        })
        .collect();

    dbg!(item_colors_result);
}

fn print_gem_colours2(gems: &Vec<String>) {
    let item_colors_result: Vec<Result<String, String>> = gems
        .into_iter()
        .map(|item| {
            match item.as_str() {
                "Diamond" => Ok("Clear".into()),
                "Ruby" => Ok("Red".into()),
                "Emerald" => Ok("Green".into()),
                "Platinum" => Ok("Blue".into()),
                "Onyx" => Ok("Black".into()),
                _ => Err("Unknown gem".into())
            }
        })
        .collect();

    dbg!(item_colors_result);
}

fn sieve_minerals_for_gold(minerals: &Vec<String>) -> Vec<&String> {
    minerals.into_iter().filter(|&mineral| mineral == "Gold").collect()
}

fn sieve_for_gems(items: &Vec<String>) -> Vec<String> {
    items.into_iter().filter_map(|item| {
        if item == "Rock" || item == "Coal" {
            return None
        }

        let colour: String = match item.as_str() {
            "Diamond" => "Clear",
            "Ruby" => "Red",
            "Gold" => "Gold",
            "Emerald" => "Green",
            "Platinum" => "Blue",
            "Onyx" => "Black",
            _ => "Unknown"
        }.into();

        Some(colour)

    }).collect()
}

fn pairs_of_socks(items: Vec<String>) -> Vec<String> {
    items
    .into_iter()
    .flat_map(|item| {
        if item == "Socks" {
            return vec!["Sock".into(), "Sock".into()];
        }
        vec![item]
    })
    .collect()
}