/// Convert utility class to CSS property
pub fn convert_utility_to_css(utility: &str) -> syn::Result<String> {
    // Handle color utilities with theme variables
    if utility.starts_with("bg-") {
        let color = &utility[3..];
        if color.starts_with("primary")
            || color.starts_with("success")
            || color.starts_with("warning")
            || color.starts_with("error")
        {
            return Ok(format!(
                "background-color: var(--{}-color);",
                color.replace('-', "-")
            ));
        }
        return Ok(format!(
            "background-color: {};",
            convert_color_value(color)?
        ));
    }

    if utility.starts_with("text-") {
        let value = &utility[5..];
        if value.starts_with("primary")
            || value.starts_with("success")
            || value.starts_with("warning")
            || value.starts_with("error")
        {
            return Ok(format!("color: var(--{}-color);", value.replace('-', "-")));
        }
        // Handle text sizes
        match value {
            "xs" => return Ok("font-size: var(--font-size-xs);".to_string()),
            "sm" => return Ok("font-size: var(--font-size-sm);".to_string()),
            "base" => return Ok("font-size: var(--font-size-base);".to_string()),
            "lg" => return Ok("font-size: var(--font-size-lg);".to_string()),
            "xl" => return Ok("font-size: var(--font-size-xl);".to_string()),
            "2xl" => return Ok("font-size: var(--font-size-2xl);".to_string()),
            _ => return Ok(format!("color: {};", convert_color_value(value)?)),
        }
    }

    // Handle spacing utilities
    if utility.starts_with("p-") || utility.starts_with("m-") {
        let property = if utility.starts_with("p-") {
            "padding"
        } else {
            "margin"
        };
        let value = &utility[2..];
        let spacing_value = convert_spacing_value(value)?;
        return Ok(format!("{}: {};", property, spacing_value));
    }

    // Handle border utilities
    if utility.starts_with("border") {
        if utility == "border" {
            return Ok("border: 1px solid var(--border-color);".to_string());
        }
        if utility.starts_with("border-") {
            let value = &utility[7..];
            return Ok(format!("border-color: {};", convert_color_value(value)?));
        }
    }

    // Handle rounded utilities
    if utility.starts_with("rounded") {
        if utility == "rounded" {
            return Ok("border-radius: var(--border-radius);".to_string());
        }
        if utility.starts_with("rounded-") {
            let value = &utility[8..];
            let radius_value = convert_radius_value(value)?;
            return Ok(format!("border-radius: {};", radius_value));
        }
    }

    // Handle display utilities
    match utility {
        "block" => return Ok("display: block;".to_string()),
        "inline-block" => return Ok("display: inline-block;".to_string()),
        "inline" => return Ok("display: inline;".to_string()),
        "flex" => return Ok("display: flex;".to_string()),
        "inline-flex" => return Ok("display: inline-flex;".to_string()),
        "grid" => return Ok("display: grid;".to_string()),
        "inline-grid" => return Ok("display: inline-grid;".to_string()),
        "hidden" => return Ok("display: none;".to_string()),
        _ => {}
    }

    // Handle position utilities
    match utility {
        "static" => return Ok("position: static;".to_string()),
        "fixed" => return Ok("position: fixed;".to_string()),
        "absolute" => return Ok("position: absolute;".to_string()),
        "relative" => return Ok("position: relative;".to_string()),
        "sticky" => return Ok("position: sticky;".to_string()),
        _ => {}
    }

    // Handle flexbox utilities
    if utility.starts_with("flex-") {
        let value = &utility[5..];
        match value {
            "row" => return Ok("flex-direction: row;".to_string()),
            "row-reverse" => return Ok("flex-direction: row-reverse;".to_string()),
            "col" => return Ok("flex-direction: column;".to_string()),
            "col-reverse" => return Ok("flex-direction: column-reverse;".to_string()),
            "wrap" => return Ok("flex-wrap: wrap;".to_string()),
            "wrap-reverse" => return Ok("flex-wrap: wrap-reverse;".to_string()),
            "nowrap" => return Ok("flex-wrap: nowrap;".to_string()),
            "1" => return Ok("flex: 1 1 0%;".to_string()),
            "auto" => return Ok("flex: 1 1 auto;".to_string()),
            "initial" => return Ok("flex: 0 1 auto;".to_string()),
            "none" => return Ok("flex: none;".to_string()),
            _ => {}
        }
    }

    // Handle justify-content utilities
    if utility.starts_with("justify-") {
        let value = &utility[8..];
        match value {
            "start" => return Ok("justify-content: flex-start;".to_string()),
            "end" => return Ok("justify-content: flex-end;".to_string()),
            "center" => return Ok("justify-content: center;".to_string()),
            "between" => return Ok("justify-content: space-between;".to_string()),
            "around" => return Ok("justify-content: space-around;".to_string()),
            "evenly" => return Ok("justify-content: space-evenly;".to_string()),
            _ => {}
        }
    }

    // Handle align-items utilities
    if utility.starts_with("items-") {
        let value = &utility[6..];
        match value {
            "start" => return Ok("align-items: flex-start;".to_string()),
            "end" => return Ok("align-items: flex-end;".to_string()),
            "center" => return Ok("align-items: center;".to_string()),
            "baseline" => return Ok("align-items: baseline;".to_string()),
            "stretch" => return Ok("align-items: stretch;".to_string()),
            _ => {}
        }
    }

    // Handle width utilities
    if utility.starts_with("w-") {
        let value = &utility[2..];
        let width_value = convert_size_value(value)
            .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), e))?;
        return Ok(format!("width: {};", width_value));
    }

    // Handle height utilities
    if utility.starts_with("h-") {
        let value = &utility[2..];
        let height_value = convert_size_value(value)
            .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), e))?;
        return Ok(format!("height: {};", height_value));
    }

    // Handle overflow utilities
    match utility {
        "overflow-auto" => return Ok("overflow: auto;".to_string()),
        "overflow-hidden" => return Ok("overflow: hidden;".to_string()),
        "overflow-visible" => return Ok("overflow: visible;".to_string()),
        "overflow-scroll" => return Ok("overflow: scroll;".to_string()),
        "overflow-x-auto" => return Ok("overflow-x: auto;".to_string()),
        "overflow-x-hidden" => return Ok("overflow-x: hidden;".to_string()),
        "overflow-y-auto" => return Ok("overflow-y: auto;".to_string()),
        "overflow-y-hidden" => return Ok("overflow-y: hidden;".to_string()),
        _ => {}
    }

    // Handle font utilities
    if utility.starts_with("text-") {
        let value = &utility[5..];
        match value {
            "xs" => return Ok("font-size: 0.75rem; line-height: 1rem;".to_string()),
            "sm" => return Ok("font-size: 0.875rem; line-height: 1.25rem;".to_string()),
            "base" => return Ok("font-size: 1rem; line-height: 1.5rem;".to_string()),
            "lg" => return Ok("font-size: 1.125rem; line-height: 1.75rem;".to_string()),
            "xl" => return Ok("font-size: 1.25rem; line-height: 1.75rem;".to_string()),
            "2xl" => return Ok("font-size: 1.5rem; line-height: 2rem;".to_string()),
            "3xl" => return Ok("font-size: 1.875rem; line-height: 2.25rem;".to_string()),
            "4xl" => return Ok("font-size: 2.25rem; line-height: 2.5rem;".to_string()),
            "5xl" => return Ok("font-size: 3rem; line-height: 1;".to_string()),
            "6xl" => return Ok("font-size: 3.75rem; line-height: 1;".to_string()),
            "7xl" => return Ok("font-size: 4.5rem; line-height: 1;".to_string()),
            "8xl" => return Ok("font-size: 6rem; line-height: 1;".to_string()),
            "9xl" => return Ok("font-size: 8rem; line-height: 1;".to_string()),
            "left" => return Ok("text-align: left;".to_string()),
            "center" => return Ok("text-align: center;".to_string()),
            "right" => return Ok("text-align: right;".to_string()),
            "justify" => return Ok("text-align: justify;".to_string()),
            _ => {
                // Handle text colors
                if let Ok(color_value) = convert_color_value(value) {
                    return Ok(format!("color: {};", color_value));
                }
            }
        }
    }

    // Handle font weight utilities
    if utility.starts_with("font-") {
        let value = &utility[5..];
        match value {
            "thin" => return Ok("font-weight: 100;".to_string()),
            "extralight" => return Ok("font-weight: 200;".to_string()),
            "light" => return Ok("font-weight: 300;".to_string()),
            "normal" => return Ok("font-weight: 400;".to_string()),
            "medium" => return Ok("font-weight: 500;".to_string()),
            "semibold" => return Ok("font-weight: 600;".to_string()),
            "bold" => return Ok("font-weight: 700;".to_string()),
            "extrabold" => return Ok("font-weight: 800;".to_string()),
            "black" => return Ok("font-weight: 900;".to_string()),
            _ => {}
        }
    }

    // Handle opacity utilities
    if utility.starts_with("opacity-") {
        let value = &utility[8..];
        let opacity_value = match value {
            "0" => "0",
            "5" => "0.05",
            "10" => "0.1",
            "20" => "0.2",
            "25" => "0.25",
            "30" => "0.3",
            "40" => "0.4",
            "50" => "0.5",
            "60" => "0.6",
            "70" => "0.7",
            "75" => "0.75",
            "80" => "0.8",
            "90" => "0.9",
            "95" => "0.95",
            "100" => "1",
            _ => value,
        };
        return Ok(format!("opacity: {};", opacity_value));
    }

    // Handle z-index utilities
    if utility.starts_with("z-") {
        let value = &utility[2..];
        let z_value = match value {
            "0" => "0",
            "10" => "10",
            "20" => "20",
            "30" => "30",
            "40" => "40",
            "50" => "50",
            "auto" => "auto",
            _ => value,
        };
        return Ok(format!("z-index: {};", z_value));
    }

    // Handle shadow utilities
    match utility {
        "shadow-sm" => return Ok("box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05);".to_string()),
        "shadow" => {
            return Ok(
                "box-shadow: 0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1);"
                    .to_string(),
            )
        }
        "shadow-md" => {
            return Ok(
                "box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1);"
                    .to_string(),
            )
        }
        "shadow-lg" => {
            return Ok(
                "box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1);"
                    .to_string(),
            )
        }
        "shadow-xl" => {
            return Ok(
                "box-shadow: 0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1);"
                    .to_string(),
            )
        }
        "shadow-2xl" => return Ok("box-shadow: 0 25px 50px -12px rgb(0 0 0 / 0.25);".to_string()),
        "shadow-inner" => return Ok("box-shadow: inset 0 2px 4px 0 rgb(0 0 0 / 0.05);".to_string()),
        "shadow-none" => return Ok("box-shadow: none;".to_string()),
        _ => {}
    }

    // Handle cursor utilities
    if utility.starts_with("cursor-") {
        let value = &utility[7..];
        match value {
            "auto" => return Ok("cursor: auto;".to_string()),
            "default" => return Ok("cursor: default;".to_string()),
            "pointer" => return Ok("cursor: pointer;".to_string()),
            "wait" => return Ok("cursor: wait;".to_string()),
            "text" => return Ok("cursor: text;".to_string()),
            "move" => return Ok("cursor: move;".to_string()),
            "help" => return Ok("cursor: help;".to_string()),
            "not-allowed" => return Ok("cursor: not-allowed;".to_string()),
            _ => {}
        }
    }

    // Handle transition utilities
    match utility {
        "transition" => return Ok("transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms;".to_string()),
        "transition-none" => return Ok("transition-property: none;".to_string()),
        "transition-all" => return Ok("transition-property: all; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms;".to_string()),
        "transition-colors" => return Ok("transition-property: color, background-color, border-color, text-decoration-color, fill, stroke; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms;".to_string()),
        "transition-opacity" => return Ok("transition-property: opacity; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms;".to_string()),
        "transition-shadow" => return Ok("transition-property: box-shadow; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms;".to_string()),
        "transition-transform" => return Ok("transition-property: transform; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms;".to_string()),
        _ => {}
    }

    // Fallback: treat as custom CSS property
    Ok(format!("{}: {};", utility.replace('-', "-"), "inherit"))
}

/// Convert color value to CSS
fn convert_color_value(color: &str) -> syn::Result<String> {
    match color {
        "white" => Ok("#ffffff".to_string()),
        "black" => Ok("#000000".to_string()),
        "transparent" => Ok("transparent".to_string()),
        "current" => Ok("currentColor".to_string()),
        _ => {
            // Handle numbered colors like "gray-500", "blue-600"
            if color.contains('-') {
                let parts: Vec<&str> = color.split('-').collect();
                if parts.len() == 2 {
                    return Ok(format!("var(--{}-{})", parts[0], parts[1]));
                }
            }
            Ok(format!("var(--color-{})", color))
        }
    }
}

/// Convert spacing value to CSS
fn convert_spacing_value(value: &str) -> syn::Result<String> {
    match value {
        "0" => Ok("0".to_string()),
        "1" => Ok("var(--spacing-1)".to_string()),
        "2" => Ok("var(--spacing-2)".to_string()),
        "3" => Ok("var(--spacing-3)".to_string()),
        "4" => Ok("var(--spacing-4)".to_string()),
        "5" => Ok("var(--spacing-5)".to_string()),
        "6" => Ok("var(--spacing-6)".to_string()),
        "8" => Ok("var(--spacing-8)".to_string()),
        "10" => Ok("var(--spacing-10)".to_string()),
        "12" => Ok("var(--spacing-12)".to_string()),
        "16" => Ok("var(--spacing-16)".to_string()),
        "20" => Ok("var(--spacing-20)".to_string()),
        "24" => Ok("var(--spacing-24)".to_string()),
        "32" => Ok("var(--spacing-32)".to_string()),
        "auto" => Ok("auto".to_string()),
        _ => Ok(format!("{}px", value)),
    }
}

/// Convert radius value to CSS
fn convert_radius_value(value: &str) -> syn::Result<String> {
    match value {
        "none" => Ok("0".to_string()),
        "sm" => Ok("0.125rem".to_string()),
        "md" => Ok("0.375rem".to_string()),
        "lg" => Ok("0.5rem".to_string()),
        "xl" => Ok("0.75rem".to_string()),
        "2xl" => Ok("1rem".to_string()),
        "3xl" => Ok("1.5rem".to_string()),
        "full" => Ok("9999px".to_string()),
        _ => {
            if value.ends_with("px") || value.ends_with("rem") || value.ends_with("em") {
                Ok(value.to_string())
            } else {
                Ok(format!("{}px", value))
            }
        }
    }
}

/// Convert size value to CSS
fn convert_size_value(value: &str) -> Result<String, String> {
    match value {
        "0" => Ok("0px".to_string()),
        "px" => Ok("1px".to_string()),
        "0.5" => Ok("0.125rem".to_string()),
        "1" => Ok("0.25rem".to_string()),
        "1.5" => Ok("0.375rem".to_string()),
        "2" => Ok("0.5rem".to_string()),
        "2.5" => Ok("0.625rem".to_string()),
        "3" => Ok("0.75rem".to_string()),
        "3.5" => Ok("0.875rem".to_string()),
        "4" => Ok("1rem".to_string()),
        "5" => Ok("1.25rem".to_string()),
        "6" => Ok("1.5rem".to_string()),
        "7" => Ok("1.75rem".to_string()),
        "8" => Ok("2rem".to_string()),
        "9" => Ok("2.25rem".to_string()),
        "10" => Ok("2.5rem".to_string()),
        "11" => Ok("2.75rem".to_string()),
        "12" => Ok("3rem".to_string()),
        "14" => Ok("3.5rem".to_string()),
        "16" => Ok("4rem".to_string()),
        "20" => Ok("5rem".to_string()),
        "24" => Ok("6rem".to_string()),
        "28" => Ok("7rem".to_string()),
        "32" => Ok("8rem".to_string()),
        "36" => Ok("9rem".to_string()),
        "40" => Ok("10rem".to_string()),
        "44" => Ok("11rem".to_string()),
        "48" => Ok("12rem".to_string()),
        "52" => Ok("13rem".to_string()),
        "56" => Ok("14rem".to_string()),
        "60" => Ok("15rem".to_string()),
        "64" => Ok("16rem".to_string()),
        "72" => Ok("18rem".to_string()),
        "80" => Ok("20rem".to_string()),
        "96" => Ok("24rem".to_string()),
        "auto" => Ok("auto".to_string()),
        "full" => Ok("100%".to_string()),
        "screen" => Ok("100vh".to_string()),
        "min" => Ok("min-content".to_string()),
        "max" => Ok("max-content".to_string()),
        "fit" => Ok("fit-content".to_string()),
        _ => {
            // Handle fractions like 1/2, 1/3, 2/3, etc.
            if value.contains('/') {
                let parts: Vec<&str> = value.split('/').collect();
                if parts.len() == 2 {
                    if let (Ok(numerator), Ok(denominator)) =
                        (parts[0].parse::<f32>(), parts[1].parse::<f32>())
                    {
                        let percentage = (numerator / denominator) * 100.0;
                        return Ok(format!("{}%", percentage));
                    }
                }
            }
            // Handle custom values with units
            if value.ends_with("px")
                || value.ends_with("rem")
                || value.ends_with("em")
                || value.ends_with("%")
                || value.ends_with("vh")
                || value.ends_with("vw")
            {
                Ok(value.to_string())
            } else {
                // Try to parse as number and add rem
                if let Ok(_) = value.parse::<f32>() {
                    Ok(format!("{}rem", value))
                } else {
                    Ok(value.to_string())
                }
            }
        }
    }
}
