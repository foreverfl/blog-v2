use crate::types::{DietProfile, DietProfileStats};

/// Burning one kilogram of body fat, the figure every diet chart quotes.
const KCAL_PER_KG_FAT: f64 = 7700.0;

/// Walking burns ~2.75 kcal per kg of body weight per hour (diet.md's 80 kg ×
/// 220 kcal/h, divided back out) and ~0.5 kcal per kg per km — which is the same
/// thing said twice, at 5.5 km/h.
const WALK_KCAL_PER_KG_HOUR: f64 = 2.75;
const WALK_KCAL_PER_KG_KM: f64 = 0.5;

/// Running is roughly 1 kcal per kg per km, and 9 kcal per kg per hour — again
/// the same pace stated two ways, at 9 km/h.
const RUN_KCAL_PER_KG_HOUR: f64 = 9.0;
const RUN_KCAL_PER_KG_KM: f64 = 1.0;

fn round1(value: f64) -> f64 {
    (value * 10.0).round() / 10.0
}

/// Derive every number the status card shows from the profile and one weight.
/// Nothing here is stored — the inputs are, and this runs on every read.
/// Everything below `bmi` needs a goal weight, so those stay None without one.
pub fn derive(profile: &DietProfile, weight_kg: f64) -> DietProfileStats {
    let height_m = profile.height_cm / 100.0;
    let bmi = weight_kg / (height_m * height_m);

    let remaining_kg = profile
        .target_weight_kg
        .map(|target| (weight_kg - target).max(0.0));
    let remaining_kcal = remaining_kg.map(|kg| kg * KCAL_PER_KG_FAT);

    DietProfileStats {
        weight_kg,
        bmi: round1(bmi),
        target_weight_kg: profile.target_weight_kg,
        remaining_kg: remaining_kg.map(round1),
        remaining_kcal: remaining_kcal.map(|kcal| kcal.round()),
        walk_hours: remaining_kcal.map(|kcal| round1(kcal / (WALK_KCAL_PER_KG_HOUR * weight_kg))),
        walk_km: remaining_kcal.map(|kcal| round1(kcal / (WALK_KCAL_PER_KG_KM * weight_kg))),
        run_hours: remaining_kcal.map(|kcal| round1(kcal / (RUN_KCAL_PER_KG_HOUR * weight_kg))),
        run_km: remaining_kcal.map(|kcal| round1(kcal / (RUN_KCAL_PER_KG_KM * weight_kg))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn profile(target: Option<f64>) -> DietProfile {
        DietProfile {
            height_cm: 176.0,
            target_weight_kg: target,
            bmr_kcal: Some(1700),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn derives_the_numbers_on_the_status_card() {
        let stats = derive(&profile(Some(65.0)), 80.0);

        assert_eq!(stats.bmi, 25.8);
        assert_eq!(stats.remaining_kg, Some(15.0));
        assert_eq!(stats.remaining_kcal, Some(115_500.0));
        assert_eq!(stats.walk_hours, Some(525.0));
        assert_eq!(stats.run_km, Some(1443.8));
    }

    #[test]
    fn goal_reached_leaves_nothing_to_burn() {
        let stats = derive(&profile(Some(65.0)), 64.0);

        assert_eq!(stats.remaining_kg, Some(0.0));
        assert_eq!(stats.remaining_kcal, Some(0.0));
        assert_eq!(stats.walk_hours, Some(0.0));
    }

    #[test]
    fn without_a_goal_only_bmi_is_known() {
        let stats = derive(&profile(None), 80.0);

        assert_eq!(stats.bmi, 25.8);
        assert_eq!(stats.remaining_kg, None);
        assert_eq!(stats.walk_hours, None);
    }
}
