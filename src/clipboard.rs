use arboard::Clipboard;

pub fn copy_text(text: &str) {
    let mut cb = Clipboard::new().unwrap();
    cb.set_text(text).unwrap()
}

#[allow(dead_code)] 
pub fn get_copied_text() -> String {
    let mut cb = Clipboard::new().unwrap();
    cb.get_text().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_works() {
        copy_text("holy");
        assert_eq!("holy", get_copied_text());
    }
}
