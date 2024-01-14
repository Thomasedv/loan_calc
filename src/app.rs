use crate::{calculation, formatting};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
use num_format::CustomFormat;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct LoanCalculator {
    loan_amount: f64,
    interest_rate: f64,
    loan_period_years: f64,
    term_price: f64,

    #[serde(skip)]
    formatter: CustomFormat,
}

impl Default for LoanCalculator {
    fn default() -> Self {
        Self {
            loan_amount: 2_000_000.0,
            interest_rate: 5.5,
            loan_period_years: 25.0,
            term_price: 65.0,
            formatter: formatting::get_formatter().unwrap(),
        }
    }
}

impl LoanCalculator {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        cc.egui_ctx.style_mut(|s| {
            s.spacing.slider_width = 300.0;
        });
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for LoanCalculator {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("Loan Calc");

            ui.add(
                egui::Slider::new(&mut self.loan_amount, 0.0..=5_000_000.0)
                    .text("value")
                    .custom_formatter(|n, _| {
                        formatting::present_int(n.round() as i64, &self.formatter).unwrap()
                    }),
            );

            ui.add(
                egui::Slider::new(&mut self.interest_rate, 0.0..=20.0)
                    .text("Interest (%)")
                    .min_decimals(1)
                    .max_decimals(3),
            );

            ui.add(
                egui::Slider::new(&mut self.loan_period_years, 0.0..=50.0)
                    .text("Loan duration (Years)")
                    .custom_formatter(|n, _| {
                        formatting::present_int(n.round() as i64, &self.formatter).unwrap()
                    }),
            );

            ui.add(
                egui::Slider::new(&mut self.term_price, 0.0..=100.0)
                    .text("Monthly fixed cost")
                    .custom_formatter(|n, _| {
                        formatting::present_int(n.round() as i64, &self.formatter).unwrap()
                    }),
            );

            ui.separator();

            let loan_terms = calculation::calculate_loan(
                self.loan_amount,
                self.interest_rate,
                self.loan_period_years,
                self.term_price,
            );

            ui.add(egui::Label::new("Total Cost"));
            ui.label(formatting::present_int(loan_terms.total_cost, &self.formatter).unwrap());
            ui.add(egui::Label::new("Monthly Cost"));
            ui.label(formatting::present_int(loan_terms.monthly_cost, &self.formatter).unwrap());
            ui.add(egui::Label::new("Total Interest Paid"));
            ui.label(formatting::present_int(loan_terms.interest_paid, &self.formatter).unwrap());

            ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/Thomasedv/loan_calc",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
