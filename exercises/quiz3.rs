// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

pub struct ReportCard {
    pub grade: f32,
    pub student_name: String,
    pub student_age: u8,
}

trait AlphabeticGrade {
    fn to_alphabetic(&self) -> String;
}

impl AlphabeticGrade for f32 {
    fn to_alphabetic(&self) -> String {
        match *self {
            grade if grade >= 4.5 => String::from("A+"),
            grade if grade >= 4.0 => String::from("A"),
            grade if grade >= 3.5 => String::from("A-"),
            grade if grade >= 3.0 => String::from("B+"),
            grade if grade >= 2.5 => String::from("B"),
            grade if grade >= 2.0 => String::from("B-"),
            grade if grade >= 1.5 => String::from("C+"),
            grade if grade >= 1.0 => String::from("C"),
            grade if grade >= 0.5 => String::from("C-"),
            _ => String::from("F"),
        }
    }
}

impl ReportCard {
    pub fn print(&self) -> String {
        let alphabetic_grade = self.grade.to_alphabetic();
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, alphabetic_grade)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of B-"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: 4.7,
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
