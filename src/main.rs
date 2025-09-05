use std::io::{self, Write};

const VER:&str = "1.0.3";
const NAME: &str = "Projectile Motion Simulator";

fn main() {
    println!("{} b({})", NAME, VER);
    let x_0:f64 = 0_f64; // starting position x_0
    let y_0:f64 = 0_f64; // starting position y_0
    let v_0:f64 = 27_f64; // initial velocity in meters per second v_0
    let g:f64 = 9.81; // gravitational acceleration, downwards pull in meters per second to the power of two

    let angle_degrees:f64 = input_helper("\nAngle in degrees: ").trim().parse().expect("Invalid input");
    let angle_radians:f64 = angle_degrees.to_radians();

    println!("\nCalculating roots...");
    let landing_time:f64 = get_landing_time(-0.5 * g, v_y_helper(v_0, angle_radians), y_0); // find the roots of the equation for the y position to determine when the projectile will hit the ground
    println!("Projectile will land after {:.2} seconds air time!", landing_time);

    let step:f64 = 0.1;
    let mut current_time:f64 = 0_f64;

    while current_time <= landing_time {
        if y(current_time, y_0, v_0, g, angle_radians) < 0_f64 {
            return;
        }
        println!("\nx pos at {:.2}s: {:.2}m", current_time, x(current_time, x_0, v_0, angle_radians));
        println!("y pos at {:.2}s: {:.2}m", current_time, y(current_time, y_0, v_0, g, angle_radians));
        current_time += step;
    }
}

fn x(t:f64, x_0:f64, v_0:f64, angle:f64) -> f64{
    let v_x:f64 = v_0 * angle.cos(); //velocity in x direction with the angle (sidewards)
    x_0 + (v_x * t) // x position at any given time
}

fn y(t:f64, y_0:f64, v_0:f64, g:f64, angle:f64) -> f64{
    y_0 + (v_y_helper(v_0, angle) * t) - ((1_f64 / 2_f64) * g * t * t) // y position at any given time
}

fn v_y_helper(v_0:f64, angle:f64) -> f64 {
    v_0 * angle.sin() // starting velocity times the sinus of the angle in which the projectile is shot equals the velocity in y direction
}

fn get_landing_time(a:f64, b:f64, c:f64) -> f64 {
    let discriminant:f64 = b * b - (4f64 * a * c);

    //let x1:f64 = (b * -1f64 + discriminant.sqrt()) / (2f64 * a);
    let x2:f64 = (b * -1f64 - discriminant.sqrt()) / (2f64 * a);

    x2
}

fn input_helper(prompt:&str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}
