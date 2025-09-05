use yew::prelude::*;

/// Props for GenderInput.
/// - `selected`: currently selected gender as String ("Male", "Female")
/// - `on_gender_change`: callback triggered when gender selection changes
#[derive(Properties, PartialEq)]
pub struct GenderInputProps {
    pub selected: String,
    pub on_gender_change: Callback<String>,
}

#[function_component(GenderInput)]
pub fn gender_input(props: &GenderInputProps) -> Html {
    let options = vec!["Male", "Female"];
    let on_select_change = {
        let cb = props.on_gender_change.clone();
        Callback::from(move |e: Event| {
            let select = e.target_dyn_into::<web_sys::HtmlSelectElement>().unwrap();
            cb.emit(select.value());
        })
    };

    html! {
        <select value={props.selected.clone()} onchange={on_select_change}>
            { for options.iter().map(|option| html! {
                <option value={option.to_string()} selected={*option == props.selected}>{option}</option>
            })}
        </select>
    }
}
