use engine_lib;

fn main() {
    let mut app = engine_lib::App::begin_build()
        .with_input_system()
        .add_init_hook(|_| {
            engine_lib::debug!("initializing");
            Ok(())
        })
        .add_init_hook(|app| {
            app.event_system.add_listener(Box::new(EventListener));
            Ok(())
        })
        .build()
        .unwrap();
    app.add_layer(Box::new(ECSLayer));

    let result = app.run();

    if let Err(e) = result {
        engine_lib::error!("ERROR: {}", e);
    }
}

struct ECSLayer;

impl engine_lib::core::layers::Layer for ECSLayer {
    fn init(&mut self, _app: &mut engine_lib::App) {}
    fn update(&mut self) {}
    fn close(&mut self) {}
}

struct EventListener;
impl engine_lib::core::events::EventListener<engine_lib::core::events::event::MouseMotion>
    for EventListener
{
    fn invoke_event(
        &mut self,
        event: &engine_lib::core::events::event::MouseMotion,
    ) -> engine_lib::core::events::EventEvaluateState {
        engine_lib::trace!("mouse motion event: {:?}", event);
        engine_lib::core::events::EventEvaluateState::Handled
    }
}
