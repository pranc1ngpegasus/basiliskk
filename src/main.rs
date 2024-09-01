use shadow_rs::shadow;

shadow!(build);

fn main() {
    println!("{}", build::SHORT_COMMIT);
}
