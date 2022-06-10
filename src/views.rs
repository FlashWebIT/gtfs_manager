use druid::im::{ordmap, vector, OrdMap, Vector};
use druid::lens::{self, LensExt};
use druid::widget::{
    Button, Checkbox, CrossAxisAlignment, Either, Flex, FlexParams, Label, List, MainAxisAlignment,
    Scroll,
};
use druid::{
    AppLauncher, Color, Data, Env, Insets, Lens, LocalizedString, Point, UnitPoint, Widget,
    WidgetExt, WindowDesc,
};

use crate::data::*;
use crate::map::MapWidget;

pub fn stop_ui() -> impl Widget<MyStopTime> {
    Flex::row()
        .with_child(Checkbox::new("").lens(MyStopTime::selected))
        .with_child(Label::new(|data: &MyStopTime, _env: &_| {
            format!("{}", data.name)
        }))
        // .with_child(Either::new(
        //     |data: &Trip, _env: &Env| data.selected,
        //     List::new(stop_ui).lens(Trip::stops),
        //     Label::new(""),
        // ))
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .padding((10., 0., 0., 0.))
}

pub fn trip_ui() -> impl Widget<MyTrip> {
    let title = Flex::row()
        .with_child(Checkbox::new("").lens(MyTrip::selected))
        .with_child(Label::new(|data: &MyTrip, _env: &_| {
            format!("{}", data.name)
        }));

    Flex::column()
        .with_child(title)
        .with_child(Either::new(
            |data: &MyTrip, _env: &Env| data.selected,
            List::new(stop_ui).lens(MyTrip::stops),
            Flex::row(),
        ))
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .padding((10., 0., 0., 0.))
}

pub fn route_ui() -> impl Widget<MyRoute> {
    let title = Flex::row()
        .with_child(Checkbox::new("").lens(MyRoute::selected))
        .with_child(Label::new(|data: &MyRoute, _env: &_| {
            format!("{}", data.route.short_name)
        }));

    Flex::column()
        .with_child(title)
        .with_child(Either::new(
            |data: &MyRoute, _env: &Env| data.selected,
            List::new(trip_ui).lens(MyRoute::trips),
            Flex::row(),
        ))
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .padding((10., 0., 0., 0.))
}

pub fn agency_ui() -> impl Widget<MyAgency> {
    let title = Flex::row()
        .with_child(Checkbox::new("").lens(MyAgency::selected))
        .with_child(Label::new(|data: &MyAgency, _env: &_| {
            format!("{}", data.agency.name)
        }));

    Flex::column()
        .with_child(title)
        .with_child(Either::new(
            |data: &MyAgency, _env: &Env| data.selected,
            List::new(route_ui).lens(MyAgency::routes),
            Flex::row(),
        ))
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .padding((10., 0., 0., 0.))
}

pub fn main_widget() -> impl Widget<AppData> {
    // todo what's the difference between Point::ZERO and Point::ORIGIN?
    let map_widget = (MapWidget::new(1., 1., Point::ZERO)).expand();
    Flex::row()
        .with_flex_child(
            Scroll::new(List::new(agency_ui).lens(AppData::agencies)),
            FlexParams::new(1.0, CrossAxisAlignment::Start),
        )
        .with_flex_child(map_widget, FlexParams::new(1.0, CrossAxisAlignment::Start))
        .main_axis_alignment(MainAxisAlignment::SpaceBetween)
}
