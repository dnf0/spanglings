use crate::core::conjugator::{conjugate_verb, VerbTable};
use colored::Colorize;

pub fn run_conjugate(verb: &str, tense_arg: Option<&str>, json: bool) -> anyhow::Result<()> {
    let table = match conjugate_verb(verb) {
        Some(t) => t,
        None => {
            println!(
                "{} Could not conjugate '{}'. Please ensure it is a valid Spanish infinitive (e.g. 'hablar', 'ser', 'tener').",
                "✗".red().bold(),
                verb.bold()
            );
            return Ok(());
        }
    };

    if json {
        let json_str = serde_json::to_string_pretty(&table)?;
        println!("{}", json_str);
        return Ok(());
    }

    print_conjugation_summary(&table, tense_arg);
    Ok(())
}

fn print_conjugation_summary(table: &VerbTable, tense_filter: Option<&str>) {
    let type_str = if table.is_irregular {
        "Irregular".bright_magenta().bold()
    } else {
        "Regular".green().bold()
    };

    println!(
        "\n{} {} ({}) — {}\n",
        "📖".bold(),
        table.infinitive.to_uppercase().bright_yellow().bold(),
        type_str,
        table.english.italic()
    );
    println!(
        "  {} Gerundio: {}  |  Participio: {}",
        "•".cyan(),
        table.gerund.bold(),
        table.participle.bold()
    );
    println!();

    if let Some(t_filter) = tense_filter {
        let filter = t_filter.to_lowercase();
        if filter.contains("subj") {
            println!("{}", "── SUBJUNTIVO (SUBJUNCTIVE) ──".cyan().bold());
            println!(
                "  {:<15} {:<18} {:<18} {:<18}",
                "Persona", "Presente", "Imperfecto (-ra)", "Imperfecto (-se)"
            );
            println!("  {}", "─".repeat(70).dimmed());
            let pres = &table.present_subjunctive;
            let ra = &table.imperfect_subjunctive_ra;
            let se = &table.imperfect_subjunctive_se;
            println!(
                "  {:<15} {:<18} {:<18} {:<18}",
                "yo",
                pres.yo.bold(),
                ra.yo.bold(),
                se.yo.bold()
            );
            println!(
                "  {:<15} {:<18} {:<18} {:<18}",
                "tú",
                pres.tu.bold(),
                ra.tu.bold(),
                se.tu.bold()
            );
            println!(
                "  {:<15} {:<18} {:<18} {:<18}",
                "vos",
                pres.vos.bold(),
                ra.vos.bold(),
                se.vos.bold()
            );
            println!(
                "  {:<15} {:<18} {:<18} {:<18}",
                "él / ella / Ud.",
                pres.el_ella_usted.bold(),
                ra.el_ella_usted.bold(),
                se.el_ella_usted.bold()
            );
            println!(
                "  {:<15} {:<18} {:<18} {:<18}",
                "nosotros",
                pres.nosotros.bold(),
                ra.nosotros.bold(),
                se.nosotros.bold()
            );
            println!(
                "  {:<15} {:<18} {:<18} {:<18}",
                "vosotros",
                pres.vosotros.bold(),
                ra.vosotros.bold(),
                se.vosotros.bold()
            );
            println!(
                "  {:<15} {:<18} {:<18} {:<18}",
                "ellos / Uds.",
                pres.ellos_ellas_ustedes.bold(),
                ra.ellos_ellas_ustedes.bold(),
                se.ellos_ellas_ustedes.bold()
            );
            return;
        } else if filter.contains("imp") && !filter.contains("imperf") {
            println!("{}", "── IMPERATIVO (COMMANDS) ──".cyan().bold());
            println!(
                "  {:<15} {:<22} {:<22}",
                "Persona", "Afirmativo (+)", "Negativo (-)"
            );
            println!("  {}", "─".repeat(60).dimmed());
            let aff = &table.imperative_affirmative;
            let neg = &table.imperative_negative;
            println!(
                "  {:<15} {:<22} {:<22}",
                "tú",
                aff.tu.green().bold(),
                neg.tu.red().bold()
            );
            println!(
                "  {:<15} {:<22} {:<22}",
                "vos",
                aff.vos.green().bold(),
                neg.vos.red().bold()
            );
            println!(
                "  {:<15} {:<22} {:<22}",
                "usted",
                aff.usted.green().bold(),
                neg.usted.red().bold()
            );
            println!(
                "  {:<15} {:<22} {:<22}",
                "nosotros",
                aff.nosotros.green().bold(),
                neg.nosotros.red().bold()
            );
            println!(
                "  {:<15} {:<22} {:<22}",
                "vosotros",
                aff.vosotros.green().bold(),
                neg.vosotros.red().bold()
            );
            println!(
                "  {:<15} {:<22} {:<22}",
                "ustedes",
                aff.ustedes.green().bold(),
                neg.ustedes.red().bold()
            );
            return;
        }
    }

    // Default overview: Indicative & Subjunctive Present
    println!("{}", "── INDICATIVO (INDICATIVE) ──".cyan().bold());
    println!(
        "  {:<12} {:<14} {:<14} {:<14} {:<14} {:<14}",
        "Persona", "Presente", "Pretérito", "Imperfecto", "Futuro", "Condicional"
    );
    println!("  {}", "─".repeat(78).dimmed());

    let p = &table.present;
    let pret = &table.preterite;
    let imp = &table.imperfect;
    let fut = &table.future;
    let cond = &table.conditional;

    println!(
        "  {:<12} {:<14} {:<14} {:<14} {:<14} {:<14}",
        "yo",
        p.yo.bold(),
        pret.yo.bold(),
        imp.yo.bold(),
        fut.yo.bold(),
        cond.yo.bold()
    );
    println!(
        "  {:<12} {:<14} {:<14} {:<14} {:<14} {:<14}",
        "tú",
        p.tu.bold(),
        pret.tu.bold(),
        imp.tu.bold(),
        fut.tu.bold(),
        cond.tu.bold()
    );
    println!(
        "  {:<12} {:<14} {:<14} {:<14} {:<14} {:<14}",
        "vos",
        p.vos.bold(),
        pret.vos.bold(),
        imp.vos.bold(),
        fut.vos.bold(),
        cond.vos.bold()
    );
    println!(
        "  {:<12} {:<14} {:<14} {:<14} {:<14} {:<14}",
        "él/ella/Ud.",
        p.el_ella_usted.bold(),
        pret.el_ella_usted.bold(),
        imp.el_ella_usted.bold(),
        fut.el_ella_usted.bold(),
        cond.el_ella_usted.bold()
    );
    println!(
        "  {:<12} {:<14} {:<14} {:<14} {:<14} {:<14}",
        "nosotros",
        p.nosotros.bold(),
        pret.nosotros.bold(),
        imp.nosotros.bold(),
        fut.nosotros.bold(),
        cond.nosotros.bold()
    );
    println!(
        "  {:<12} {:<14} {:<14} {:<14} {:<14} {:<14}",
        "vosotros",
        p.vosotros.bold(),
        pret.vosotros.bold(),
        imp.vosotros.bold(),
        fut.vosotros.bold(),
        cond.vosotros.bold()
    );
    println!(
        "  {:<12} {:<14} {:<14} {:<14} {:<14} {:<14}",
        "ellos/Uds.",
        p.ellos_ellas_ustedes.bold(),
        pret.ellos_ellas_ustedes.bold(),
        imp.ellos_ellas_ustedes.bold(),
        fut.ellos_ellas_ustedes.bold(),
        cond.ellos_ellas_ustedes.bold()
    );

    println!("\n{}", "── SUBJUNTIVO (SUBJUNCTIVE) ──".cyan().bold());
    println!(
        "  {:<12} {:<18} {:<18} {:<18}",
        "Persona", "Presente", "Imperfecto (-ra)", "Imperfecto (-se)"
    );
    println!("  {}", "─".repeat(68).dimmed());
    let pres_s = &table.present_subjunctive;
    let ra_s = &table.imperfect_subjunctive_ra;
    let se_s = &table.imperfect_subjunctive_se;

    println!(
        "  {:<12} {:<18} {:<18} {:<18}",
        "yo",
        pres_s.yo.bold(),
        ra_s.yo.bold(),
        se_s.yo.bold()
    );
    println!(
        "  {:<12} {:<18} {:<18} {:<18}",
        "tú",
        pres_s.tu.bold(),
        ra_s.tu.bold(),
        se_s.tu.bold()
    );
    println!(
        "  {:<12} {:<18} {:<18} {:<18}",
        "vos",
        pres_s.vos.bold(),
        ra_s.vos.bold(),
        se_s.vos.bold()
    );
    println!(
        "  {:<12} {:<18} {:<18} {:<18}",
        "él/ella/Ud.",
        pres_s.el_ella_usted.bold(),
        ra_s.el_ella_usted.bold(),
        se_s.el_ella_usted.bold()
    );
    println!(
        "  {:<12} {:<18} {:<18} {:<18}",
        "nosotros",
        pres_s.nosotros.bold(),
        ra_s.nosotros.bold(),
        se_s.nosotros.bold()
    );
    println!(
        "  {:<12} {:<18} {:<18} {:<18}",
        "vosotros",
        pres_s.vosotros.bold(),
        ra_s.vosotros.bold(),
        se_s.vosotros.bold()
    );
    println!(
        "  {:<12} {:<18} {:<18} {:<18}",
        "ellos/Uds.",
        pres_s.ellos_ellas_ustedes.bold(),
        ra_s.ellos_ellas_ustedes.bold(),
        se_s.ellos_ellas_ustedes.bold()
    );
}
