use gst::{
    glib,
    glib::once_cell::sync::Lazy,
    subclass::prelude::{
        ElementImpl, GstObjectImpl, ObjectImpl, ObjectSubclass, ObjectSubclassType,
    },
};
use gst_base::subclass::{base_transform::BaseTransformImpl, BaseTransformMode};

#[derive(Default)]
pub struct Filter {}

glib::wrapper! {
    pub struct FilterElement(ObjectSubclass<Filter>) @extends gst::Element, gst::Object;
}

pub fn register(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    gst::Element::register(Some(plugin), "myfilter", gst::Rank::None, Filter::type_())
}

#[glib::object_subclass]
impl ObjectSubclass for Filter {
    const NAME: &'static str = "Filter";
    type ParentType = gst_base::BaseTransform;
    type Type = FilterElement;
}

impl ObjectImpl for Filter {}

impl GstObjectImpl for Filter {}

impl ElementImpl for Filter {
    fn metadata() -> Option<&'static gst::subclass::ElementMetadata> {
        static ELEMENT_METADATA: Lazy<gst::subclass::ElementMetadata> = Lazy::new(|| {
            gst::subclass::ElementMetadata::new(
                "Filter",
                "Filter",
                "Filter all buffers",
                env!("CARGO_PKG_AUTHORS"),
            )
        });

        Some(&*ELEMENT_METADATA)
    }

    fn pad_templates() -> &'static [gst::PadTemplate] {
        static PAD_TEMPLATES: Lazy<Vec<gst::PadTemplate>> = Lazy::new(|| {
            let src_pad_template = gst::PadTemplate::new(
                "src",
                gst::PadDirection::Src,
                gst::PadPresence::Always,
                &gst::Caps::new_any(),
            )
            .unwrap();

            let sink_pad_template = gst::PadTemplate::new(
                "sink",
                gst::PadDirection::Sink,
                gst::PadPresence::Always,
                &gst::Caps::new_any(),
            )
            .unwrap();

            vec![src_pad_template, sink_pad_template]
        });

        PAD_TEMPLATES.as_ref()
    }
}

impl BaseTransformImpl for Filter {
    const MODE: BaseTransformMode = BaseTransformMode::AlwaysInPlace;
    const PASSTHROUGH_ON_SAME_CAPS: bool = false;
    const TRANSFORM_IP_ON_PASSTHROUGH: bool = false;

    fn transform_ip(&self, _buf: &mut gst::BufferRef) -> Result<gst::FlowSuccess, gst::FlowError> {
        Ok(gst_base::BASE_TRANSFORM_FLOW_DROPPED)
        // Ok(gst::FlowSuccess::Ok)
    }
}
