use gtk::{glib::{self, subclass::{prelude::*, InitializingObject}}, subclass::{prelude::*, widget::{CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetImpl}, window::WindowImpl}, Button, CompositeTemplate, Entry, TemplateChild, Window};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/chardoncs/justshell/url-dialog.ui")]
pub struct UrlDialog {
    pub url: Option<String>,

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
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for UrlDialog {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for UrlDialog {}

impl WindowImpl for UrlDialog {}
