use crate::models::{Platform, Problem, TestCase};
use scraper::{Html, Selector};

fn parse_contest_and_problem(problem_id: &str) -> Option<(String, String)> {
    // Handles formats: "1971D", "1971d", "1971-D", "1971_D"
    let cleaned = problem_id.replace(['-', '_'], "");
    let digits: String = cleaned.chars().take_while(|c| c.is_ascii_digit()).collect();
    let letters: String = cleaned.chars().skip_while(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() || letters.is_empty() {
        return None;
    }
    Some((digits, letters.to_ascii_uppercase()))
}

fn build_url(contest_id: &str, problem_index: &str) -> String {
    format!(
        "https://codeforces.com/contest/{}/problem/{}",
        contest_id, problem_index
    )
}

fn extract_text_by_selector(document: &Html, selector_str: &str) -> Option<String> {
    let selector = Selector::parse(selector_str).ok()?;
    let element = document.select(&selector).next()?;
    Some(element.text().collect::<Vec<_>>().join("").trim().to_string())
}

fn parse_time_limit_ms(text: &str) -> u32 {
    let tokens: Vec<&str> = text.split_whitespace().collect();
    for (i, token) in tokens.iter().enumerate() {
        if token.eq_ignore_ascii_case("second") || token.eq_ignore_ascii_case("seconds") {
            if let Some(prev) = tokens.get(i.wrapping_sub(1)) {
                if let Ok(val) = prev.parse::<f64>() {
                    return (val * 1000.0) as u32;
                }
            }
        }
    }
    1000 // default 1 second
}

fn parse_memory_limit_mb(text: &str) -> u32 {
    let tokens: Vec<&str> = text.split_whitespace().collect();
    for (i, token) in tokens.iter().enumerate() {
        if token.eq_ignore_ascii_case("megabyte")
            || token.eq_ignore_ascii_case("megabytes")
            || token.eq_ignore_ascii_case("mb")
        {
            if let Some(prev) = tokens.get(i.wrapping_sub(1)) {
                if let Ok(val) = prev.parse::<u32>() {
                    return val;
                }
            }
        }
    }
    256 // default 256 MB
}

fn clean_statement(html: &str) -> String {
    let document = Html::parse_fragment(html);
    let text = document.root_element().text().collect::<Vec<_>>().join("");
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn parse_samples(document: &Html) -> Vec<TestCase> {
    let mut samples = Vec::new();
    let sample_selector = match Selector::parse(".sample-test") {
        Ok(s) => s,
        Err(_) => return samples,
    };
    let input_selector = match Selector::parse(".input pre") {
        Ok(s) => s,
        Err(_) => return samples,
    };
    let output_selector = match Selector::parse(".output pre") {
        Ok(s) => s,
        Err(_) => return samples,
    };

    for sample in document.select(&sample_selector) {
        let input = sample
            .select(&input_selector)
            .next()
            .map(|el| el.text().collect::<Vec<_>>().join(""))
            .unwrap_or_default();
        let output = sample
            .select(&output_selector)
            .next()
            .map(|el| el.text().collect::<Vec<_>>().join(""))
            .unwrap_or_default();
        samples.push(TestCase { input, output });
    }
    samples
}

fn parse_tags(document: &Html) -> Vec<String> {
    let mut tags = Vec::new();
    let Ok(tag_selector) = Selector::parse(".tag-box") else {
        return tags;
    };
    for tag_el in document.select(&tag_selector) {
        let text = tag_el.text().collect::<Vec<_>>().join("").trim().to_string();
        if !text.is_empty() {
            tags.push(text);
        }
    }
    tags
}

pub async fn fetch_problem(problem_id: &str) -> Result<Problem, Box<dyn std::error::Error>> {
    let (contest_id, problem_index) = parse_contest_and_problem(problem_id)
        .ok_or_else(|| format!("invalid problem id format: {}", problem_id))?;
    let url = build_url(&contest_id, &problem_index);

    let client = reqwest::Client::builder()
        .user_agent("Deck/0.1.0")
        .build()?;
    let html = client.get(&url).send().await?.text().await?;
    let document = Html::parse_document(&html);

    // Title
    let title = extract_text_by_selector(&document, ".title")
        .unwrap_or_default()
        .replace(&format!("{}.", problem_index), "")
        .trim()
        .to_string();
    let title = if title.is_empty() {
        problem_id.to_string()
    } else {
        title
    };

    // Time limit
    let time_limit_text = extract_text_by_selector(&document, ".time-limit").unwrap_or_default();
    let time_limit = parse_time_limit_ms(&time_limit_text);

    // Memory limit
    let memory_limit_text =
        extract_text_by_selector(&document, ".memory-limit").unwrap_or_default();
    let memory_limit = parse_memory_limit_mb(&memory_limit_text);

    // Statement
    let statement = extract_text_by_selector(&document, ".problem-statement")
        .map(|s| clean_statement(&s))
        .unwrap_or_default();

    // Samples
    let samples = parse_samples(&document);

    // Tags
    let tags = parse_tags(&document);

    Ok(Problem {
        id: problem_id.to_string(),
        platform: Platform::Codeforces,
        title,
        rating: None,
        tags,
        time_limit,
        memory_limit,
        statement,
        samples,
        solved: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_contest_and_problem() {
        assert_eq!(parse_contest_and_problem("1971D"), Some(("1971".into(), "D".into())));
        assert_eq!(parse_contest_and_problem("1971d"), Some(("1971".into(), "D".into())));
        assert_eq!(
            parse_contest_and_problem("1971-D"),
            Some(("1971".into(), "D".into()))
        );
        assert_eq!(
            parse_contest_and_problem("1971_D"),
            Some(("1971".into(), "D".into()))
        );
        assert_eq!(parse_contest_and_problem("D"), None);
    }

    #[test]
    fn test_parse_time_limit_ms() {
        assert_eq!(parse_time_limit_ms("1 second"), 1000);
        assert_eq!(parse_time_limit_ms("2 seconds"), 2000);
        assert_eq!(parse_time_limit_ms("1.5 seconds"), 1500);
        assert_eq!(parse_time_limit_ms(""), 1000);
    }

    #[test]
    fn test_parse_memory_limit_mb() {
        assert_eq!(parse_memory_limit_mb("256 megabytes"), 256);
        assert_eq!(parse_memory_limit_mb("512 MB"), 512);
        assert_eq!(parse_memory_limit_mb(""), 256);
    }
}
