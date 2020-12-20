use std::error::Error;

pub(crate) fn get_script_name() -> Result<Option<String>, Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        2 => {
            let file_name = args[1].clone();
            Ok(Some(file_name))
        }
        1 => Ok(None),
        _ => Err(format!("Usage: {} [script]", args[0]).into()),
    }
}
