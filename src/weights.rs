use crate::armory::{PrimStats,SecStats,Race};
use crate::utils::{Args};


pub struct StatShift {
    pub text: String,
    pub prim_stats: PrimStats,
    pub sec_stats: SecStats
}

impl StatShift {

    pub fn new(args: &Args) -> Vec<StatShift> {
        let mut vec = Vec::new();
        vec.push(StatShift::get_zero_object());
        if args.weights {
            vec.push(StatShift::get_agility_object_from_sign(
                    Sign::Negative, args));
            vec.push(StatShift::get_agility_object_from_sign(
                    Sign::Positive, args));
            vec.push(StatShift::get_strength_object_from_sign(
                    Sign::Negative, args));
            vec.push(StatShift::get_strength_object_from_sign(
                    Sign::Positive, args));
            vec.push(StatShift::get_crit_object_from_sign(
                    Sign::Negative, args));
            vec.push(StatShift::get_crit_object_from_sign(
                    Sign::Positive, args));
            vec.push(StatShift::get_hit_object_from_sign(
                    Sign::Negative, args));
            vec.push(StatShift::get_hit_object_from_sign(
                    Sign::Positive, args));
            vec.push(StatShift::get_dagger_skill_object_from_sign(
                    Sign::Negative, args));
            vec.push(StatShift::get_dagger_skill_object_from_sign(
                    Sign::Positive, args));
        }
        return vec;
    }

    fn get_zero_object() -> StatShift {
        StatShift {
            text: "Base dps:\t".to_string(),
            prim_stats: PrimStats::new_from_race(Race::None),
            sec_stats: SecStats::new_from_race(Race::None)
        }
    }

    fn get_agility_object_from_sign(sign: Sign, args: &Args) -> StatShift {

        let mut obj = StatShift::get_zero_object();

        let mut value: i32 = 10 * args.weight_mult;
        if sign == Sign::Positive { 
            obj.text = format!("{:+} agility:\t", value);
        } 
        else { 
            value *= -1; 
            obj.text = format!("\n{:+} agility:\t", value);
        }

        obj.prim_stats.agility = value;

        return obj;
    }

    fn get_strength_object_from_sign(sign: Sign, args: &Args) -> StatShift {

        let mut obj = StatShift::get_zero_object();

        let mut value: i32 = 10 * args.weight_mult;
        if sign == Sign::Positive { 
            obj.text = format!("{:+} strength:\t", value);
        } 
        else { 
            value *= -1; 
            obj.text = format!("\n{:+} strength:\t", value);
        }

        obj.prim_stats.strength = value;

        return obj;
    }

    fn get_hit_object_from_sign(sign: Sign, args: &Args) -> StatShift {

        let mut obj = StatShift::get_zero_object();

        let mut value: f32 = 0.01 * args.weight_mult as f32;
        if sign == Sign::Positive { 
            obj.text = format!("{:+} hit:\t", value);
        } 
        else { 
            value *= -1.0; 
            obj.text = format!("\n{:+} hit:\t", value);
        }

        obj.sec_stats.hit = value;

        return obj;
    }

    fn get_crit_object_from_sign(sign: Sign, args: &Args) -> StatShift {

        let mut obj = StatShift::get_zero_object();

        let mut value: f32 = 0.01 * args.weight_mult as f32;
        if sign == Sign::Positive { 
            obj.text = format!("{:+} crit:\t", value);
        } 
        else { 
            value *= -1.0; 
            obj.text = format!("\n{:+} crit:\t", value);
        }

        obj.sec_stats.crit = value;

        return obj;
    }

    fn get_dagger_skill_object_from_sign(sign: Sign, args: &Args) -> StatShift {

        let mut obj = StatShift::get_zero_object();

        let mut value: i32 = 1 * args.weight_mult;
        if sign == Sign::Positive { 
            obj.text = format!("{:+} dagger skill:\t", value);
        } 
        else { 
            value *= -1; 
            obj.text = format!("\n{:+} dagger skill:\t", value);
        }

        obj.prim_stats.dagger_skill = value;

        return obj;
    }
}

#[derive(PartialEq)]
enum Sign {
    Positive,
    Negative
}
