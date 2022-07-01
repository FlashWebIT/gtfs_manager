use std::rc::Rc;

use druid::im::{ordmap, vector, OrdMap, Vector};
use druid::keyboard_types::Key;
use druid::lens::{self, LensExt};
use druid::text::{EditableText, TextStorage};
use druid::widget::{
    Button, Checkbox, Container, Controller, CrossAxisAlignment, Either, Flex, FlexParams, Label,
    List, MainAxisAlignment, RadioGroup, Scroll, TextBox,
};
use druid::{
    AppDelegate, AppLauncher, Color, Data, Env, Event, EventCtx, FontDescriptor, FontFamily,
    FontWeight, Insets, Lens, LocalizedString, Point, Selector, UnitPoint, UpdateCtx, Widget,
    WidgetExt, WindowDesc,
};
use gtfs_structures::ContinuousPickupDropOff;

use crate::data::*;
use crate::map::MapWidget;

mod dropdown;
mod expander;
mod filtered_list;
use dropdown::*;
use expander::Expander;
use filtered_list::FilteredList;

// parameters
const SPACING_1: f64 = 20.;
const CORNER_RADIUS: f64 = 5.;
const HEADING_1: FontDescriptor = FontDescriptor::new(FontFamily::SYSTEM_UI)
    .with_weight(FontWeight::BOLD)
    .with_size(24.0);
const HEADING_2: FontDescriptor = FontDescriptor::new(FontFamily::SYSTEM_UI)
    .with_weight(FontWeight::BOLD)
    .with_size(20.0);

// command selectors
// (<item type>, <id>)
const ITEM_DELETE: Selector<(String, String)> = Selector::new("item.delete");
// (<item type>, <id>)
const ITEM_UPDATE: Selector<(String, String)> = Selector::new("item.update");
// (<item type>, <parent id>)
const ITEM_NEW_CHILD: Selector<(String, String)> = Selector::new("item.new.child");
const EDIT_DELETE: Selector<usize> = Selector::new("edit.delete");

pub struct Delegate;
impl AppDelegate<AppData> for Delegate {
    fn event(
        &mut self,
        ctx: &mut druid::DelegateCtx,
        window_id: druid::WindowId,
        event: Event,
        data: &mut AppData,
        env: &Env,
    ) -> Option<Event> {
        match &event {
            Event::KeyDown(key_event) => {
                // not firing for some reason
                println!("keydown");
                match key_event.key {
                    Key::ArrowUp => {
                        println!("arrowup");
                    }
                    Key::ArrowDown => {
                        println!("arrowdown");
                    }
                    _ => {}
                }
            }
            _ => {}
        };
        Some(event)
    }
    fn command(
        &mut self,
        ctx: &mut druid::DelegateCtx,
        target: druid::Target,
        cmd: &druid::Command,
        data: &mut AppData,
        env: &Env,
    ) -> druid::Handled {
        if let Some(item_delete) = cmd.get(ITEM_DELETE) {
            dbg!(item_delete);

            // data.edits.clear();
            for agency in data.agencies.iter_mut() {
                for route in agency.routes.iter_mut() {
                    if item_delete.0 == "route".to_string() && route.id() == item_delete.1 {
                        route.live = false;
                        data.actions.push_back(Action {
                            id: data.actions.len(),
                            edit_type: EditType::Delete,
                            item_type: "route".to_string(),
                            item_id: route.id(),
                            item_data: Some(Rc::new(route.clone())),
                        });
                    } else {
                        for trip in route.trips.iter_mut() {
                            if item_delete.0 == "trip".to_string() && trip.id() == item_delete.1 {
                                trip.live = false;
                                data.actions.push_back(Action {
                                    id: data.actions.len(),
                                    edit_type: EditType::Delete,
                                    item_type: "trip".to_string(),
                                    item_id: trip.id(),
                                    item_data: Some(Rc::new(trip.clone())),
                                });
                            } else {
                                for stop_time in trip.stops.iter_mut() {
                                    if item_delete.0 == "stop_time".to_string()
                                        && stop_time.id() == item_delete.1
                                    {
                                        stop_time.live = false;
                                        data.actions.push_back(Action {
                                            id: data.actions.len(),
                                            edit_type: EditType::Delete,
                                            item_type: "stop_time".to_string(),
                                            item_id: stop_time.id(),
                                            item_data: Some(Rc::new(stop_time.clone())),
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // druid::Handled::No
            druid::Handled::Yes
        } else if let Some(item_update) = cmd.get(ITEM_UPDATE) {
            dbg!(item_update);
            if item_update.0 == "trip".to_string() {
                for agency in data.agencies.iter() {
                    for route in agency.routes.iter() {
                        for trip in route.trips.iter() {
                            if trip.id() == item_update.1 {
                                dbg!(&trip.trip_headsign);
                                if data.actions.len() > 0
                                    && trip.id()
                                        == data
                                            .actions
                                            .last()
                                            .unwrap()
                                            .item_data
                                            .as_ref()
                                            .unwrap()
                                            .id()
                                    && data.actions.last().unwrap().edit_type == EditType::Update
                                {
                                    data.actions
                                        .get_mut(data.actions.len() - 1)
                                        .unwrap()
                                        .item_data = Some(Rc::new(trip.clone()));
                                } else {
                                    let edit = Action {
                                        id: data.actions.len(),
                                        edit_type: EditType::Update,
                                        item_type: "trip".to_string(),
                                        item_id: trip.id(),
                                        item_data: Some(Rc::new(trip.clone())),
                                    };
                                    data.actions.push_back(edit);
                                }

                                // edit = Action {
                                //     id: data.edits.len(),
                                //     edit_type: EditType::Update,
                                //     item_type: "trip".to_string(),
                                //     item_id: trip.id(),
                                //     item_data: Some(Rc::new(trip.clone())),
                                // };
                                // match data.edits.iter().position(|edit| {
                                //     edit.item_id == item_update.1
                                //         && edit.edit_type == EditType::Update
                                // }) {
                                //     Some(index) => {
                                //         data.edits.set(index, edit);
                                //     }
                                //     None => {
                                //         data.edits.push_back(edit);
                                //     }
                                // };
                            }
                            // for stop_time in trip.stops.iter() {
                            //     if !stop_time.live {
                            //         data.edits.push_back(Edit {
                            //             id: data.edits.len(),
                            //             edit_type: EditType::Delete,
                            //             item_type: "stop_time".to_string(),
                            //             item_id: stop_time.id(),
                            //             item_data: Some(Rc::new(stop_time.clone())),
                            //         });
                            //     }
                            // }
                        }
                    }
                }
            }
            // for stop_time in data.gtfs.stop_times {
            //     if stop_time.
            // }
            // for agency in data.gtfs.agencies {
            //     if agency.
            // }
            druid::Handled::Yes

            // delete edits
        } else if let Some(item) = cmd.get(ITEM_NEW_CHILD) {
            println!("new child");
            dbg!(item);
            let (item_type, parent_id) = item;

            // data.edits.clear();
            for agency in data.agencies.iter_mut() {
                if item_type == "agency" && &agency.id() == parent_id {
                    agency.new_child();
                    data.actions.push_back(Action {
                        id: data.actions.len(),
                        edit_type: EditType::Create,
                        // todo is the item type route? or should it be a trip?
                        item_type: "route".to_string(),
                        item_id: agency.id(),
                        item_data: Some(Rc::new(agency.clone())),
                    });
                } else {
                    for route in agency.routes.iter_mut() {
                        if item_type == "route" && &route.id() == parent_id {
                            route.new_child();
                            data.actions.push_back(Action {
                                id: data.actions.len(),
                                edit_type: EditType::Create,
                                // todo is the item type route? or should it be a trip?
                                item_type: "trip".to_string(),
                                item_id: route.id(),
                                item_data: Some(Rc::new(route.clone())),
                            });
                        } else {
                            // for trip in route.trips.iter_mut() {
                            //     if item_type == "trip" {
                            //         trip.live = false;
                            //         data.edits.push_back(Edit {
                            //             id: data.edits.len(),
                            //             edit_type: EditType::Delete,
                            //             item_type: "trip".to_string(),
                            //             item_id: trip.id(),
                            //             item_data: Some(Rc::new(trip.clone())),
                            //         });
                            //     } else {
                            //         for stop_time in trip.stops.iter_mut() {
                            //             if item_type == "stop_time" {
                            //                 stop_time.live = false;
                            //                 data.edits.push_back(Edit {
                            //                     id: data.edits.len(),
                            //                     edit_type: EditType::Delete,
                            //                     item_type: "stop_time".to_string(),
                            //                     item_id: stop_time.id(),
                            //                     item_data: Some(Rc::new(stop_time.clone())),
                            //                 });
                            //             }
                            //         }
                            //     }
                            // }
                        }
                    }
                }
            }
            // druid::Handled::No
            druid::Handled::Yes
        } else if let Some(edit_id) = cmd.get(EDIT_DELETE) {
            dbg!(edit_id);
            let edit = data.actions.get(*edit_id).unwrap();
            if edit.item_type == "stop_time".to_string() {
                for agency in data.agencies.iter_mut() {
                    for route in agency.routes.iter_mut() {
                        for trip in route.trips.iter_mut() {
                            for stop_time in trip.stops.iter_mut() {
                                if stop_time.id() == edit.item_id {
                                    stop_time.live = true;
                                }
                            }
                        }
                    }
                }
            }
            if edit.item_type == "trip".to_string() {
                for agency in data.agencies.iter_mut() {
                    for route in agency.routes.iter_mut() {
                        for trip in route.trips.iter_mut() {
                            if trip.id() == edit.item_id {
                                trip.live = true;
                            }
                        }
                    }
                }
            }
            data.actions.retain(|edit| edit.id != *edit_id);
            druid::Handled::Yes
        } else {
            druid::Handled::No
        }
    }
}

struct TextBoxOnChange;

impl<W: Widget<MyTrip>> Controller<MyTrip, W> for TextBoxOnChange {
    fn update(
        &mut self,
        child: &mut W,
        ctx: &mut UpdateCtx,
        old_data: &MyTrip,
        data: &MyTrip,
        env: &Env,
    ) {
        //  !bug
        if !old_data.trip_headsign.same(&data.trip_headsign) && old_data.id().same(&data.id()) {
            dbg!(&old_data.trip_headsign);
            dbg!(&data.trip_headsign);
            ctx.submit_command(ITEM_UPDATE.with((data.item_type(), data.id())));
        }
        child.update(ctx, old_data, data, env);
    }
}
fn delete_item_button<T: Data + ListItem>() -> impl Widget<T> {
    Button::new("delete").on_click(|ctx, data: &mut T, _| {
        ctx.submit_command(ITEM_DELETE.with((data.item_type(), data.id())));
    })
}
fn update_all_buttons<T: Data + ListItem>() -> impl Widget<T> {
    Flex::column()
        .with_child(Button::new("new child").on_click(|ctx, data: &mut T, _| {
            ctx.submit_command(ITEM_NEW_CHILD.with((data.item_type(), data.id())));
        }))
        .with_child(delete_item_button())
        .with_child(Button::new("select all").on_click(|_, data: &mut T, _| {
            data.update_all(true);
        }))
        .with_child(Button::new("deselect all").on_click(|_, data: &mut T, _| {
            data.update_all(false);
        }))
}
fn child_controls<T: Data + ListItem>() -> impl Widget<T> {
    Flex::row()
        .with_child(Button::new("new child").on_click(|ctx, data: &mut T, _| {
            ctx.submit_command(ITEM_NEW_CHILD.with((data.item_type(), data.id())));
        }))
        .with_child(Button::new("select all").on_click(|_, data: &mut T, _| {
            data.update_all(true);
        }))
        .with_child(Button::new("deselect all").on_click(|_, data: &mut T, _| {
            data.update_all(false);
        }))
}

// todo make a custom checkbox which has data (String, bool) so the label value can be taken from the data AND be clickable
pub fn stop_ui() -> impl Widget<MyStopTime> {
    Container::new(
        Flex::row()
            .with_child(
                TextBox::new()
                    .with_placeholder("stop name")
                    .lens(MyStopTime::name),
            )
            .with_child(Checkbox::new("").lens(MyStopTime::selected))
            .with_child(Label::new(|data: &MyStopTime, _env: &_| {
                format!("arrival/departure: {:?}", data.stop_time.arrival_time)
            }))
            .with_child(delete_item_button())
            // .with_child(Either::new(
            //     |data: &Trip, _env: &Env| data.selected,
            //     List::new(stop_ui).lens(Trip::stops),
            //     Label::new(""),
            // ))
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .padding((10., 10., 10., 10.)),
    )
    .rounded(CORNER_RADIUS)
    .background(Color::grey(0.16))
    .expand_width()
}

pub fn trip_ui() -> impl Widget<MyTrip> {
    // let label = Label::new(|data: &bool, env: &Env| "hi");
    let title = Flex::row()
        .with_child(Label::new(|data: &MyTrip, _env: &_| {
            format!("{}", data.name)
        }))
        .with_child(Checkbox::new("").lens(MyTrip::selected).on_click(
            |ctx: &mut EventCtx, data: &mut MyTrip, env: &Env| {
                if data.selected {
                    data.selected = false;
                    data.stops.iter_mut().for_each(|stop| stop.selected = false);
                } else {
                    data.selected = true;
                    data.stops.iter_mut().for_each(|stop| stop.selected = true);
                }
            },
        ));

    Container::new(
        Flex::column()
            .with_child(title)
            .with_spacer(SPACING_1)
            .with_child(
                Flex::row()
                    .with_child(Label::new("trip_headsign"))
                    .with_child(
                        TextBox::new()
                            .with_placeholder("trip_headsign")
                            .lens(MyTrip::trip_headsign)
                            .controller(TextBoxOnChange {}),
                    ),
            )
            .with_spacer(SPACING_1)
            .with_child(
                Flex::row()
                    .with_child(Expander::new("Stops").lens(MyTrip::expanded))
                    .with_child(update_all_buttons())
                    .main_axis_alignment(MainAxisAlignment::SpaceBetween)
                    .expand_width(),
            )
            .with_default_spacer()
            .with_child(Either::new(
                |data: &MyTrip, _env: &Env| data.expanded,
                FilteredList::new(
                    List::new(stop_ui).with_spacing(10.),
                    |item_data: &MyStopTime, filtered: &()| item_data.live,
                )
                .lens(druid::lens::Map::new(
                    |data: &MyTrip| (data.stops.clone(), ()),
                    |data: &mut MyTrip, inner: (Vector<MyStopTime>, ())| {
                        data.stops = inner.0;
                        // data.filter = inner.1;
                    },
                ))
                .disabled_if(|data, _| !data.selected),
                Flex::row(),
            ))
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .padding((10., 10., 10., 10.)),
    )
    .rounded(CORNER_RADIUS)
    // .background(Color::grey(0.1))
    .background(Color::rgb(54. / 255., 74. / 255., 63. / 255.))
    .expand_width()
}

fn option_string_checkbox() -> impl Widget<Option<String>> {
    // "poo".to_string().
    Flex::row()
        .with_child(Checkbox::new("").lens(druid::lens::Map::new(
            |data: &Option<String>| match data {
                Some(_) => true,
                None => false,
            },
            |data: &mut Option<String>, inner: bool| {
                *data = if inner {
                    Some("nuttin".to_string())
                } else {
                    None
                };
            },
        )))
        .with_child(
            TextBox::new()
                .with_placeholder("hi")
                .lens(druid::lens::Map::new(
                    |data: &Option<String>| match data {
                        Some(text) => text.clone(),
                        None => "nutin".to_string(),
                    },
                    |data: &mut Option<String>, inner: String| {
                        match data {
                            Some(old_inner) => {
                                old_inner.clear();
                                old_inner.push_str(&inner);
                            }
                            None => {}
                        };
                    },
                ))
                .disabled_if(|data: &Option<String>, _| data.is_none()),
        )
}
fn option_string() -> impl Widget<Option<String>> {
    TextBox::new()
        .with_placeholder("empty")
        .lens(druid::lens::Map::new(
            |data: &Option<String>| match data {
                Some(text) => text.clone(),
                None => "".to_string(),
            },
            |data: &mut Option<String>, inner: String| {
                if inner.is_empty() {
                    *data = None;
                } else {
                    *data = Some(inner);
                }
            },
        ))
}

fn field_row<T: Data>(name: &str, update: impl Widget<T> + 'static) -> impl Widget<T> {
    Flex::row()
        .with_child(Label::new(name).fix_width(150.))
        .with_child(update.fix_width(250.))
        .with_child(Label::new("status").fix_width(100.))
        .cross_axis_alignment(CrossAxisAlignment::Start)
}
pub fn route_ui() -> impl Widget<MyRoute> {
    let title = Flex::row()
        .with_child(
            Label::new(|data: &MyRoute, _env: &_| format!("{}", data.short_name))
                .with_font(HEADING_2),
        )
        .with_child(Checkbox::new("").lens(MyRoute::selected))
        .with_default_spacer()
        .with_child(Either::new(
            |data: &MyRoute, _env: &Env| data.live,
            Label::new(""),
            Label::new("deleted").with_text_color(Color::RED),
        ))
        .with_default_spacer()
        .with_child(Either::new(
            |data: &MyRoute, _env: &Env| data.new,
            Label::new("new item").with_text_color(Color::RED),
            Label::new(""),
        ))
        .with_default_spacer()
        .with_child(delete_item_button());

    let fields = Flex::column()
        .with_child(field_row(
            "id",
            Label::new(|data: &MyRoute, _: &_| format!("{:?}", data.id)),
        ))
        .with_child(field_row(
            "short_name",
            TextBox::new()
                .with_placeholder("route short_name")
                .lens(MyRoute::short_name),
        ))
        .with_child(field_row(
            "long_name",
            TextBox::new()
                .with_placeholder("route long_name")
                .lens(MyRoute::long_name),
        ))
        .with_child(field_row("desc", option_string().lens(MyRoute::desc)))
        // route_type
        .with_child(field_row("url", option_string().lens(MyRoute::url)))
        .with_child(field_row(
            "agency_id",
            Label::new(|data: &MyRoute, _: &_| format!("{:?}", data.agency_id)),
        ))
        .with_child(field_row(
            "order",
            Label::new(|data: &MyRoute, _: &_| format!("{:?}", data.order)),
        ))
        // .with_child(
        //     Flex::row()
        //         .with_child(Label::new("desc"))
        //         .with_child(RadioGroup::column(vec![Some(), None]).lens(MyRoute::desc)),
        // )
        .with_child(field_row(
            "continuous_pickup",
            RadioGroup::column(MyContinuousPickupDropOff::radio_vec()).lens(druid::lens::Map::new(
                |data: &MyRoute| data.continuous_pickup.clone(),
                |data: &mut MyRoute, inner: MyContinuousPickupDropOff| {
                    data.continuous_pickup = inner;
                },
            )),
        ))
        .cross_axis_alignment(CrossAxisAlignment::Start);

    Container::new(
        Flex::column()
            .with_child(title)
            .with_spacer(SPACING_1)
            .with_child(
                Flex::row()
                    .with_child(Label::new("trip_headsign"))
                    .with_child(
                        TextBox::new()
                            .with_placeholder("route short_name")
                            .lens(MyRoute::short_name), // .controller(TextBoxOnChange {}),
                    ),
            )
            .with_spacer(SPACING_1)
            .with_child(fields)
            .with_spacer(SPACING_1)
            .with_child(
                Flex::row()
                    .with_child(Expander::new("Trips").lens(MyRoute::expanded))
                    .with_child(Either::new(
                        |data: &MyRoute, _: &_| data.expanded,
                        child_controls(),
                        Flex::row(),
                    ))
                    .main_axis_alignment(MainAxisAlignment::SpaceBetween)
                    .expand_width(),
            )
            .with_default_spacer()
            .with_child(Either::new(
                |data: &MyRoute, _env: &Env| data.expanded,
                // removing filteredlist doesn't help with trips not updating
                // List::new(trip_ui)
                //     .with_spacing(10.)
                //     .lens(MyRoute::trips)
                //     .disabled_if(|data, _| !data.selected),
                // Flex::row(),
                FilteredList::new(
                    List::new(trip_ui).with_spacing(10.),
                    |item_data: &MyTrip, filtered: &()| item_data.live,
                )
                .lens(druid::lens::Map::new(
                    |data: &MyRoute| (data.trips.clone(), ()),
                    |data: &mut MyRoute, inner: (Vector<MyTrip>, ())| {
                        data.trips = inner.0;
                        // data.filter = inner.1;
                    },
                )),
                Flex::row(),
            ))
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .padding((10., 10., 10., 10.)),
    )
    .rounded(CORNER_RADIUS)
    .background(Color::grey(0.16))
    .expand_width()
}

pub fn agency_ui() -> impl Widget<MyAgency> {
    let title = Flex::row()
        .with_child(
            Label::new(|data: &MyAgency, _env: &_| format!("{}", data.agency.name))
                .with_font(HEADING_2),
        )
        .with_child(Checkbox::new("").lens(MyAgency::selected));

    Container::new(
        Flex::column()
            .with_child(title)
            .with_spacer(SPACING_1)
            .with_child(
                Flex::row()
                    .with_child(Expander::new("Routes").lens(MyAgency::expanded))
                    .with_child(update_all_buttons())
                    .main_axis_alignment(MainAxisAlignment::SpaceBetween)
                    .expand_width(),
            )
            .with_default_spacer()
            .with_child(Either::new(
                |data: &MyAgency, _env: &Env| data.expanded,
                FilteredList::new(
                    List::new(route_ui).with_spacing(10.),
                    |item_data: &MyRoute, filtered: &bool| {
                        *filtered || (!*filtered && item_data.live)
                    },
                )
                .lens(druid::lens::Map::new(
                    |data: &MyAgency| (data.routes.clone(), data.show_deleted),
                    |data: &mut MyAgency, inner: (Vector<MyRoute>, bool)| {
                        data.routes = inner.0;
                        data.show_deleted = inner.1;
                    },
                ))
                .disabled_if(|data, _| !data.selected),
                Flex::row(),
            ))
            .cross_axis_alignment(CrossAxisAlignment::Start),
    )
    .padding((10., 10., 10., 10.))
    // .background(Color::grey(0.1))
    .background(Color::rgb(54. / 255., 58. / 255., 74. / 255.))
    .rounded(CORNER_RADIUS)
    .fix_width(800.)
}

fn edit() -> impl Widget<Edit> {
    Container::new(Flex::row())
        .padding((10., 10., 10., 10.))
        // .background(Color::grey(0.1))
        .background(Color::rgb(54. / 255., 58. / 255., 74. / 255.))
        .rounded(CORNER_RADIUS)
}
fn action() -> impl Widget<Action> {
    Container::new(
        Flex::row()
            .with_child(Label::new(|data: &Action, _: &_| match data.edit_type {
                EditType::Delete => format!(
                    "Delete: {} {}. {}",
                    data.item_type,
                    data.item_id,
                    data.item_data.clone().unwrap().data_info()
                ),
                EditType::Create => format!(
                    "Create: {} {}. {}",
                    data.item_type,
                    data.item_id,
                    data.item_data.clone().unwrap().data_info()
                ),
                EditType::Update => format!(
                    "Update: {} {}. {}",
                    data.item_type,
                    data.item_id,
                    data.item_data.clone().unwrap().data_info()
                ),
            }))
            .with_child(
                Button::new("x").on_click(|ctx: &mut EventCtx, data: &mut Action, _| {
                    ctx.submit_command(EDIT_DELETE.with(data.id));
                }),
            ),
    )
    .padding((10., 10., 10., 10.))
    // .background(Color::grey(0.1))
    .background(Color::rgb(54. / 255., 58. / 255., 74. / 255.))
    .rounded(CORNER_RADIUS)
}

pub fn main_widget() -> impl Widget<AppData> {
    // todo what's the difference between Point::ZERO and Point::ORIGIN?
    println!("make main widget");
    let map_widget = (MapWidget::new(1., 1., Point::ZERO)).expand();
    Flex::row()
        .with_child(
            Flex::column()
                .with_child(Checkbox::new("show deleted").lens(AppData::show_deleted))
                .with_default_spacer()
                .with_child(Expander::new("Edits").lens(AppData::show_edits))
                .with_default_spacer()
                .with_child(
                    Either::new(
                        |data: &AppData, _env: &Env| data.show_edits,
                        List::new(edit)
                            .with_spacing(10.)
                            .lens(AppData::edits)
                            .fix_width(800.),
                        Flex::row().fix_width(800.),
                    )
                    .fix_width(800.),
                )
                .with_default_spacer()
                .with_child(Expander::new("Actions").lens(AppData::show_actions))
                .with_default_spacer()
                .with_child(
                    Either::new(
                        |data: &AppData, _env: &Env| data.show_actions,
                        List::new(action)
                            .with_spacing(10.)
                            .lens(AppData::actions)
                            .fix_width(800.),
                        Flex::row().fix_width(800.),
                    )
                    .fix_width(800.),
                )
                .with_default_spacer()
                .with_child(
                    Flex::row()
                        .with_child(Expander::new("Agencies").lens(AppData::expanded))
                        .with_child(update_all_buttons())
                        .main_axis_alignment(MainAxisAlignment::SpaceBetween)
                        .fix_width(800.),
                )
                .with_default_spacer()
                .with_flex_child(
                    Either::new(
                        |data: &AppData, _env: &Env| data.expanded,
                        Scroll::new(
                            Flex::column().with_child(
                                List::new(agency_ui)
                                    .with_spacing(10.)
                                    .lens(AppData::agencies),
                            ),
                        )
                        .fix_width(800.),
                        Flex::row().fix_width(800.),
                    )
                    // Scroll::new(
                    //     Flex::column().with_child(update_all_buttons()).with_child(
                    //         List::new(agency_ui)
                    //             .with_spacing(10.)
                    //             .lens(AppData::agencies),
                    //     ),
                    // )
                    .fix_width(800.),
                    1.,
                )
                .cross_axis_alignment(CrossAxisAlignment::Start),
        )
        .with_spacer(20.)
        .with_flex_child(map_widget, FlexParams::new(1.0, CrossAxisAlignment::Start))
        .main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .padding(20.)
}
