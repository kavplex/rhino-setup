use std::iter::Cycle;
use std::time::Duration;

use relm4::adw::prelude::*;
use relm4::{adw, gtk, ComponentParts, ComponentSender, SimpleComponent};

pub(crate) struct WelcomeModel {
    title: &'static str,
    titles: Cycle<std::array::IntoIter<&'static str, 34>>,
}

#[derive(Debug)]
pub(crate) enum WelcomeInput {
    /// Indicates whether to change the title text.
    ChangeTitle,
}

#[derive(Debug)]
pub(crate) enum WelcomeOutput {
    /// Move to the next page.
    NextPage,
}

#[relm4::component(pub)]
impl SimpleComponent for WelcomeModel {
    type Init = ();
    type Input = WelcomeInput;
    type Output = WelcomeOutput;
    // AppWidgets is generated by the macro
    type Widgets = WelcomeWidgets;

    view! {
        #[root]
        adw::Bin {
            set_halign: gtk::Align::Fill,
            set_valign: gtk::Align::Fill,
            set_hexpand: true,

            adw::StatusPage {
                set_icon_name: Some("org.rollingrhino.rhino-setup.welcome"),
                #[watch]
                set_title: model.title,
                set_description: Some("Make your choices, this wizard will take care of everything."),

                gtk::Button::with_label("Let's Start") {
                    set_css_classes: &["pill", "suggested-action"],
                    set_halign: gtk::Align::Center,

                    connect_clicked[sender] => move|_| {
                        sender.output(WelcomeOutput::NextPage);
                    }
                }
            }
        }
    }

    // Initialize the UI.
    fn init(
        _counter: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = WelcomeModel {
            title: "Welcome!",
            titles: [
                "Welcome!",
                "Benvenuto",
                "Bienvenido",
                "Bienvenue",
                "Willkommen",
                "Bem-vindo",
                "Добро пожаловать",
                "欢迎",
                "ようこそ",
                "환영합니다",
                "أهلا بك",
                "ברוך הבא",
                "Καλώς ήρθατε",
                "Hoşgeldiniz",
                "Welkom",
                "Witamy",
                "Välkommen",
                "Tervetuloa",
                "Vítejte",
                "Üdvözöljük",
                "Bun venit",
                "Vitajte",
                "Tere tulemast",
                "Sveiki atvykę",
                "Dobrodošli",
                "خوش آمدید",
                "आपका स्वागत है",
                "স্বাগতম",
                "வரவேற்கிறோம்",
                "స్వాగతం",
                "स्वागत",
                "સુસ્વાગત છે",
                "ಸುಸ್ವಾಗತ",
                "സ്വാഗതം",
            ]
            .into_iter()
            .cycle(),
        };

        // Insert the macro code generation here
        let widgets = view_output!();

        std::thread::spawn(move || loop {
            sender.input(WelcomeInput::ChangeTitle);
            std::thread::sleep(Duration::new(1, 2));
        });

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            WelcomeInput::ChangeTitle => {
                self.title = self.titles.next().unwrap();
                log::trace!("Changing title to: {}", self.title);
            },
        }
    }
}
