use std::collections::HashSet;

fn main() -> anyhow::Result<()>{
    let symbols = get_set("./data/symbols")?;
    let quotes  = get_set("./data/quotes")?;
    let bases   = get_set("./data/bases")?;

    let currencies: Vec<String> = quotes.union(&bases).cloned().collect();

    let mut pair_a = String::new();
    let mut pair_b = String::new();
    let mut pair_c = String::new();

    for a in &currencies {
        for b in &currencies {
            for c in &currencies {
                pair_a.clear();
                pair_b.clear();
                pair_c.clear();

                pair_a.push_str(b);
                pair_a.push_str(a);

                pair_b.push_str(c);
                pair_b.push_str(b);

                pair_c.push_str(a);
                pair_c.push_str(c);

                if symbols.contains(&pair_a) && symbols.contains(&pair_b) && symbols.contains(&pair_c) {
                    println!("[\"{a}\", \"{b}\", \"{c}\"]");
                }
            }
        }
    }

    Ok(())
}

fn get_set(filename: &str) -> anyhow::Result<HashSet<String>> {
    let content = std::fs::read_to_string(filename)?;
    let lines: Vec<_> = content.split('\n').map(|line| line.trim().to_string()).filter(|line| line.len() > 0).collect();
    let mut out = HashSet::new();
    for line in lines {
        out.insert(line);
    }
    Ok(out)
}
