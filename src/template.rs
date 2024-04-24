
#[cfg(test)]
mod tests {
    use arboard::Clipboard;

    #[test]
    fn test() {
        let mut clipboard = Clipboard::new().unwrap();
        clipboard.set_text("hello world").unwrap();
    }
}