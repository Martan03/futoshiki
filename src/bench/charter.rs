use std::{collections::HashMap, ops::Range};

use plotters::{
    chart::{ChartBuilder, SeriesLabelPosition},
    prelude::{BitMapBackend, IntoDrawingArea, IntoLogRange, PathElement},
    series::LineSeries,
    style::{Color, IntoFont, Palette, Palette99, BLACK, WHITE},
};

#[derive(Debug, Clone)]
pub struct Charter {
    title: String,
    lines: HashMap<String, Vec<(i32, f64)>>,
    x_label: String,
    y_label: String,
}

impl Charter {
    pub fn empty(title: &str) -> Self {
        Self {
            title: title.to_string(),
            lines: HashMap::new(),
            x_label: String::from("Size"),
            y_label: String::from("Secs."),
        }
    }

    pub fn y_label(mut self, label: &str) -> Self {
        self.y_label = label.to_string();
        self
    }

    pub fn push(&mut self, title: String, pos: i32, value: f64) {
        if let Some(line) = self.lines.get_mut(&title) {
            line.push((pos, value));
        } else {
            self.lines.insert(title, vec![(pos, value)]);
        }
    }

    pub fn plot(
        &self,
        filename: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let root =
            BitMapBackend::new(filename, (600, 470)).into_drawing_area();
        root.fill(&WHITE)?;

        let (x_range, y_range) = self.get_range();
        let mut chart = ChartBuilder::on(&root)
            .caption(&self.title, ("sans-serif", 50).into_font())
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(55)
            .build_cartesian_2d(x_range, y_range.log_scale())?;

        chart
            .configure_mesh()
            .x_desc(&self.x_label)
            .y_desc(&self.y_label)
            .draw()?;

        for (i, (title, line)) in self.lines.iter().enumerate() {
            let color = Palette99::pick(i);
            chart
                .draw_series(LineSeries::new(line.iter().copied(), &color))?
                .label(title)
                .legend(move |(x, y)| {
                    PathElement::new([(x, y), (x + 20, y)], &color)
                });
        }

        chart
            .configure_series_labels()
            .background_style(WHITE.mix(0.8))
            .border_style(BLACK)
            .position(SeriesLabelPosition::UpperLeft)
            .draw()?;

        Ok(())
    }

    pub fn plot_lin(
        &self,
        filename: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let root =
            BitMapBackend::new(filename, (600, 470)).into_drawing_area();
        root.fill(&WHITE)?;

        let (x_range, y_range) = self.get_range();
        let mut chart = ChartBuilder::on(&root)
            .caption(&self.title, ("sans-serif", 50).into_font())
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(55)
            .build_cartesian_2d(x_range, y_range)?;

        chart
            .configure_mesh()
            .x_desc(&self.x_label)
            .y_desc(&self.y_label)
            .draw()?;

        for (i, (title, line)) in self.lines.iter().enumerate() {
            let color = Palette99::pick(i);
            chart
                .draw_series(LineSeries::new(line.iter().copied(), &color))?
                .label(title)
                .legend(move |(x, y)| {
                    PathElement::new([(x, y), (x + 20, y)], &color)
                });
        }

        chart
            .configure_series_labels()
            .background_style(WHITE.mix(0.8))
            .border_style(BLACK)
            .position(SeriesLabelPosition::UpperLeft)
            .draw()?;

        Ok(())
    }

    fn get_range(&self) -> (Range<i32>, Range<f64>) {
        let x_min = self
            .lines
            .values()
            .flat_map(|line| line.iter().map(|&(x, _)| x))
            .min()
            .unwrap_or(0);
        let x_max = self
            .lines
            .values()
            .flat_map(|line| line.iter().map(|&(x, _)| x))
            .max()
            .unwrap_or(10);
        let y_min = self
            .lines
            .values()
            .flat_map(|line| line.iter().map(|&(_, y)| y))
            .min_by(|a, b| {
                a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or(0.0);
        let y_max = self
            .lines
            .values()
            .flat_map(|line| line.iter().map(|&(_, y)| y))
            .max_by(|a, b| {
                a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or(100.0);

        let padding = (y_max - y_min) * 0.001;

        (x_min..x_max, (y_min - padding).max(0.)..y_max)
    }
}
