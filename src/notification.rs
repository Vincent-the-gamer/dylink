use notify_rust::Notification;

pub fn use_notification<'a>(title: &'a str, body: &'a str) -> () {
    Notification::new()
        .summary(title)
        .body(body)
        .show()
        .expect("Notification create error!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        use_notification("Ciallo", "Ah! senpai~ ciallo~");
    }
}
