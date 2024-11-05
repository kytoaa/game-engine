/// Represents an engine layer, layers are distinct 'layers' of the engine, allowing for easy
/// injection of custom systems
pub trait Layer {
    /// called either when the app is initialized, or when the layer is added
    fn init(&mut self, app: &mut crate::App);
    /// the layer's update loop
    fn update(&mut self);
    /// called when the layer is being removed, either manually or on shutdown
    fn close(&mut self);
}
