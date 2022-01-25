struct ForInsert<'a> {
    lines: Vec<&'a str>,
    topic: &'a str
}

enum DbInsertionError {
    TopicDoesNotExist
}

trait DbInserter {
    fn insert(&self, for_insert_vec: Vec<ForInsert>) -> Result<(), DbInsertionError>;
}