pub struct CastleSchema {
    pub schema: String,
}

impl CastleSchema {
    pub fn new() -> Self {
        let schema = "
            //This is the real schema
            
        ".to_string();

        Self {
            schema,
        }
    }
}