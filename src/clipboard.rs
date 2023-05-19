use arboard::Clipboard;

pub fn copy_text(text: &str) -> Result<(), ()> {
    let mut cb = Clipboard::new().unwrap();
    cb.set_text(text).map_err(|_| ())
}

#[allow(dead_code)] 
pub fn get_copied_text() -> Result<String, ()> {
    let mut cb = Clipboard::new().unwrap();
    cb.get_text().map_err(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_works() {
        copy_text("holy").unwrap();
        assert_eq!("holy", get_copied_text().unwrap());
    }
}
