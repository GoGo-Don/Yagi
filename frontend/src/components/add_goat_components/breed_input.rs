use shared::Breed;
use web_sys::HtmlInputElement;
use yew::prelude::*;

/// Props for BreedInput:
/// - `selected`: current breed value as string (e.g. "Beetal" or "Other")
/// - `other_value`: current 'other' breed string value
/// - `on_breed_change`: callback when selected breed changes
/// - `on_other_change`: callback when 'other' text changes
#[derive(Properties, PartialEq)]
pub struct BreedInputProps {
    pub selected: String,
    pub other_value: String,
    pub on_breed_change: Callback<String>,
    pub on_other_change: Callback<String>,
}

#[function_component(BreedInput)]
pub fn breed_input(props: &BreedInputProps) -> Html {
    let is_other = props.selected == "Other";
    let breed_options = vec![
        "Beetal",
        "Jamunapari",
        "Barbari",
        "Sirohi",
        "Osmanabadi",
        "BlackBengal",
        "Kutchi",
        "Kaghani",
        "Chegu",
        "Jakhrana",
        "Other",
    ];

    let on_select_change = {
        let cb = props.on_breed_change.clone();
        Callback::from(move |e: Event| {
            let select = e.target_dyn_into::<web_sys::HtmlSelectElement>().unwrap();
            cb.emit(select.value());
        })
    };

    let on_other_input = {
        let cb = props.on_other_change.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                cb.emit(input.value());
            }
        })
    };

    html! {
        <>
            <select value={props.selected.clone()} onchange={on_select_change}>
                { for breed_options.iter().map(|option| html! {
                    <option value={option.to_string()} selected={*option == props.selected}>{option}</option>
                })}
            </select>
            if is_other {
                html! {
                    <input
                        type="text"
                        placeholder="Enter custom breed"
                        value={props.other_value.clone()}
                        oninput={on_other_input}
                    />
                }
            }
        </>
    }
}
