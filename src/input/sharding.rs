use std::collections::HashMap;
use regex::{Error as RegexError, Regex};

pub trait ShardSelector {
    fn select(&self, file_name: &str) -> Option<String>;
}

struct PrefixBasedShardSelector {
    shards: Vec<(Regex, String)>
}

impl PrefixBasedShardSelector {
    fn new(regex_and_shard: HashMap<String, String>) -> Result<PrefixBasedShardSelector, RegexError> {
        let mut shards: Vec<(Regex, String)> = vec!();
        for (prefix, shard) in regex_and_shard {
            let regex = Regex::new(prefix.as_str())?;
            shards.push((regex, shard));
        }

        Ok(PrefixBasedShardSelector { shards })
    }
}

impl ShardSelector for PrefixBasedShardSelector {
    fn select(&self, file_name: &str) -> Option<String> {
        self.shards.iter()
            .find(|(regex, _)| regex.is_match(file_name))
            .map(|(_, shard)| shard.clone())
    }
}