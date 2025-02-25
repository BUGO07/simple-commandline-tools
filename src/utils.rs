pub fn write_to_json<T: serde::Serialize>(file_name: &str, data: T) -> Result<(), std::io::Error> {
    std::fs::write(file_name, serde_json::to_string_pretty(&data).unwrap())
}

pub fn read_from_json(file_name: &str) -> Result<Vec<crate::Todo>, std::io::Error> {
    match std::fs::read(file_name) {
        Ok(data) => {
            let data = String::from_utf8(data).unwrap();
            serde_json::from_str::<Vec<crate::Todo>>(data.as_str())
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
        }
        Err(e) => Err(e),
    }
}

pub fn exists(file_name: &str) -> bool {
    std::fs::exists(file_name).unwrap_or(false)
}

pub fn qes(a: f64, b: f64, c: f64) -> (f64, f64, f64) {
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        (discriminant, f64::NAN, f64::NAN)
    } else {
        let sqrt_d = discriminant.sqrt();
        let x1 = (-b + sqrt_d) / (2.0 * a);
        let x2 = (-b - sqrt_d) / (2.0 * a);
        (discriminant, x1, x2)
    }
}

pub fn pos_string(s: f64) -> String {
    if s >= 0.0 {
        "+".to_string()
    } else {
        "-".to_string()
    }
}

pub fn get_todos() -> String {
    dirs::data_dir()
        .unwrap()
        .join("todos.json")
        .to_str()
        .unwrap()
        .to_string()
}
