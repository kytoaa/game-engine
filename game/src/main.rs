use engine_lib;

fn main() {
    let mut app = engine_lib::App::begin_build().build().unwrap();
    app.add_layer(Box::new(ECSLayer));

    let result = app.run();

    if let Err(e) = result {
        engine_lib::error!("ERROR: {}", e);
    }
}

struct ECSLayer;

impl engine_lib::core::layers::Layer for ECSLayer {
    fn init(&mut self, app: &mut engine_lib::App) {}
    fn update(&mut self) {}
    fn close(&mut self) {}
}

#[cfg(test)]
mod tests {
    use std::any::Any;
    #[test]
    fn type_identifier() {
        let a = (25, true);
        let b = (240, true, 1.0);

        assert_ne!(a.type_id(), b.type_id());
    }
}
