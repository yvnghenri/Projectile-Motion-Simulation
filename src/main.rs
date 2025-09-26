use std::io::{self, Write};

const VER:&str = "1.0.3";
const NAME: &str = "Projectile Motion Simulator";

fn main() {
    println!("{} b({})", NAME, VER);
    let x_0:f64 = 0_f64; // starting position (m)
    let y_0:f64 = 0_f64; // starting position (m)
    let g:f64 = 9.81; // gravitational acceleration, downwards pull (m/s^2)

    // Initial velocity (m/s)
    let v_0:String = input_helper("\nInitial velocity in m/s: ");
    let v_0:f64 = v_0.trim().parse().expect("Invalid input");

    // Shooting angle of projectile (°)
    let angle_degrees:f64 = input_helper("Angle in degrees: ").trim().parse().expect("Invalid input");
    let angle_radians:f64 = angle_degrees.to_radians();

    println!("\nCalculating roots...");
    let landing_time:f64 = get_landing_time(-0.5 * g, v_y_helper(v_0, angle_radians), y_0);
    println!("Projectile will land after {:.2} seconds air time!", landing_time);

    let step:f64 = 0.1; // (s)
    let mut current_time:f64 = 0_f64;

    println!("\nStarting simulation...");

    while current_time <= landing_time {
        println!("\nx pos at {:.2}s: {:.2}m", current_time, x(current_time, x_0, v_0, angle_radians));
        println!("y pos at {:.2}s: {:.2}m", current_time, y(current_time, y_0, v_0, g, angle_radians));
        current_time += step;
    }

    println!("\n\nExtremum:");

    let max_pos_t:f64 = maximum_pos(-0.5 * g, v_y_helper(v_0, angle_radians));
    println!("Maximum at {:.2}s", max_pos_t);
    println!("x pos: {:.2}m, y pos: {:.2}m", x(max_pos_t, x_0, v_0, angle_radians), y(max_pos_t, y_0, v_0, g, angle_radians));
}

/// x(t) = x_0 + v_0 * cos(α) * t, horizontal
fn x(t:f64, x_0:f64, v_0:f64, angle:f64) -> f64{
    let v_x:f64 = v_0 * angle.cos();
    x_0 + (v_x * t)
}

/// y(t) = y_0 + v_0 * sin(α) * t - 1/2 * g * t^2, vertical
fn y(t:f64, y_0:f64, v_0:f64, g:f64, angle:f64) -> f64{
    y_0 + (v_y_helper(v_0, angle) * t) - ((1_f64 / 2_f64) * g * t * t)
}

/// velocity in y direction without gravitational acceleration
fn v_y_helper(v_0:f64, angle:f64) -> f64 {
    v_0 * angle.sin()
}

/// calculate the roots of y(t) for the landing time -> y(t) = 0
fn get_landing_time(a:f64, b:f64, c:f64) -> f64 {
    let discriminant:f64 = b * b - (4f64 * a * c);

    //let t1:f64 = (b * -1f64 + discriminant.sqrt()) / (2f64 * a);
    let t2:f64 = (b * -1f64 - discriminant.sqrt()) / (2f64 * a);

    t2
}

/// maximum of the trajectory (vertex), t = -b / 2 * a
fn maximum_pos(a:f64, b:f64) -> f64 {
    let t = -b / (2_f64 * a);
    t
}

fn input_helper(prompt:&str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}
