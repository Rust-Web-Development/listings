use std::collections::HashMap;
use handle_errors::Error;

/// Pagination struct which is getting extract
/// from query params
#[derive(Debug)]
pub struct Pagination {
    pub start: usize,
    pub end: usize,
}

/// Extract query parameters from the `/questions` route
/// # Example query
/// GET requests to this route can have a paginatino attached so we just
/// return the questions we need
/// `/questions?start=1&end=10`
pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    // Could be improved in the future
    if params.contains_key("start") && params.contains_key("end") {
        return Ok(Pagination {
            // Takes the "start" parameter in the query
            // and tries to convert it to a number
            start: params
                .get("start")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
            // Takes the "end" parameter in the query
            // and tries to convert it to a number
            end: params
                .get("end")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
        });
    }

    Err(Error::MissingParameters)
}