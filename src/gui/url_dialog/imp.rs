use std::sync::OnceLock;

use gtk::{prelude::*, glib::{self, subclass::{prelude::*, InitializingObject, Signal}}, prelude::StaticType, subclass::{prelude::*, widget::{CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetImpl}, window::WindowImpl}, Button, CompositeTemplate, Entry, TemplateChild, Window};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/chardoncs/justshell/url-dialog.ui")]
pub struct UrlDialog {
    #[template_child]
    pub entry: TemplateChild<Entry>,

    #[template_child]
    pub ok_button: TemplateChild<Button>,
    #[template_child]
    pub cancel_button: TemplateChild<Button>,
}

#[glib::object_subclass]
impl ObjectSubclass for UrlDialog {
    const NAME: &'static str = "UrlDialog";
    type Type = super::UrlDialog;
    type ParentType = Window;

    fn class_init(c: &mut Self::Class) {
        c.bind_template();
        c.set_css_name("url-dialog");
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for UrlDialog {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();

        obj.setup_callbacks();

        self.ok_button.set_sensitive(false);
    }

    fn signals() -> &'static [glib::subclass::Signal] {
        static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();

        SIGNALS.get_or_init(|| {
            vec![
                Signal::builder("proceed")
                    .param_types([String::static_type()])
                    .build()
            ]
        })
    }
}

impl WidgetImpl for UrlDialog {}

impl WindowImpl for UrlDialog {}
