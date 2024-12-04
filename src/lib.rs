use std::fs;

pub fn parse_input_file(input_file_path: std::string::String) -> Result<String, String> {
    let canonical_input_path = match fs::canonicalize(input_file_path) {
        Ok(path) => path,
        Err(err) => {
            return Err(format!("Cannot canonicalize input file: {}", err))
        }
    };

    if !canonical_input_path.exists() {
        return Err(format!("Input file '{}' does not exist.", canonical_input_path.display()))
    }

    let input_string = match fs::read_to_string(&canonical_input_path) {
        Ok(string) => string,
        Err(err) => {
            return Err(format!("Could not read input file: {:?} to string: {}", &canonical_input_path, err))
        }
    };
    Ok(input_string)
}
