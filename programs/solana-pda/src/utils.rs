const ADDRESS_STORAGE: usize = 32;
const DISCRIMINATOR_STORAGE: usize = 8;
const STRING_LENGTH_STORAGE: usize = 4;
const BUMP_SEED_STORAGE: usize = 1;

fn calculate_string_storage(string_field: &String) -> usize {
    STRING_LENGTH_STORAGE + string_field.len()
}

pub fn calculate_account_storage(params: AccountStorageParams) -> usize {
    DISCRIMINATOR_STORAGE
    + ADDRESS_STORAGE
    + BUMP_SEED_STORAGE
    + calculate_string_storage(&params.message)
    + calculate_string_storage(&params.id.to_string())
}

// not sure whether it is a good practice
pub struct AccountStorageParams<'a> {
    pub id: &'a u32,
    pub message: &'a String,
}
