use crate::activities::Activity;
use crate::reports::ReportGenerator;
use crate::user::User;
use std::fmt;

/// FitnessTracker struct that manages user activities
/// Demonstrates Dependency Inversion Principle - depends on abstractions (Activity trait)
pub struct FitnessTracker {
    user: User,
    activities: Vec<Box<dyn Activity>>,
}

impl FitnessTracker {
    /// Creates a new FitnessTracker for the given user
    pub fn new(user: User) -> Self {
        FitnessTracker {
            user,
            activities: Vec::new(),
        }
    }

    /// Adds an activity to the tracker
    pub fn add_activity(&mut self, activity: Box<dyn Activity>) {
        self.activities.push(activity);
    }

    /// Displays user statistics
    pub fn display_user_stats(&self) {
        println!("{}", self.user);
        println!("Total activities: {}", self.activities.len());
    }
}

impl ReportGenerator for FitnessTracker {
    fn generate_report(&self) -> String {
        let mut report = format!("Fitness Report for {}\n", self.user.name());
        report.push_str(&format!("Total Activities: {}\n", self.activities.len()));

        let total_calories: f32 = self.activities.iter().map(|a| a.calories_burned()).sum();

        report.push_str(&format!("Total Calories Burned: {:.1}\n", total_calories));

        report.push_str("\nActivity Details:\n");
        for activity in &self.activities {
            report.push_str(&format!("- {}\n", activity.summary()));
        }

        report
    }
}

impl fmt::Display for FitnessTracker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FitnessTracker for {} with {} activities",
            self.user.name(),
            self.activities.len()
        )
    }
}
