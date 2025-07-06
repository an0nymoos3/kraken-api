/// Kraken-SDK error types
pub enum KrakErr {
    ApiConnection(String),
    ApiBadRequest(String),
    ApiBadAuth(String),
    ApiUnknownError(String),
    DbConnection(String),
    DbBadRequest(String),
}
